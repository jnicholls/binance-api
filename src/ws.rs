use std::collections::BTreeMap;
use std::marker::{PhantomData, Unpin};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Duration;

use async_tungstenite::{
    tokio::{connect_async, ConnectStream},
    tungstenite::Message,
    WebSocketStream,
};
use futures::{
    future::{self, Either, FutureExt},
    sink::SinkExt,
    stream::{SplitSink, SplitStream, Stream, StreamExt},
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::{
    sync::{mpsc, oneshot},
    time,
};

use crate::{error::*, extensions::*, models::*};

const WS_REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const WSSAPI_HOST: &str = "wss://stream.binance.com:9443/ws/";
const WSFAPI_HOST: &str = "wss://fstream.binance.com/ws/";

pub type WSFClient = WSClient<WSFApi>;
pub type WSSClient = WSClient<WSSApi>;

#[derive(Debug)]
struct ClientState {
    is_closed: bool,
    next_id: u64,
    requests: BTreeMap<u64, oneshot::Sender<Result<WSResponse, WSApiCode>>>,
}

struct EventDispatcher<OrderType> {
    close_rx: oneshot::Receiver<()>,
    event_tx: mpsc::Sender<Result<WSEvent<OrderType>, WSApiCode>>,
    request_tx: mpsc::Sender<WSMessage<OrderType>>,
    state: Arc<Mutex<ClientState>>,
    stream: SplitStream<WebSocketStream<ConnectStream>>,
}

impl<OrderType> EventDispatcher<OrderType>
where
    OrderType: DeserializeOwned,
{
    // TODO: Clean this up, bit of staircase hell going on here.
    async fn dispatch_events(mut self) {
        loop {
            let either = future::select(self.stream.next(), self.close_rx).await;
            match either {
                Either::Left((next, close_rx)) => {
                    self.close_rx = close_rx;
                    if let Some(msg) = next {
                        match msg {
                            Ok(msg) => match msg {
                                Message::Text(t) => match serde_json::from_str(&t) {
                                    Ok(msg) => match msg {
                                        WSMessage::Event(event) => {
                                            let _ = self.event_tx.send(Ok(event)).await;
                                        }
                                        WSMessage::Response(resp) => {
                                            let tx = {
                                                let mut state = self.state.lock().unwrap();
                                                state.requests.remove(&resp.id)
                                            };
                                            if let Some(tx) = tx {
                                                let _ = tx.send(Ok(resp));
                                            }
                                        }
                                        _ => (),
                                    },
                                    Err(e) => {
                                        let _ = self.event_tx.send(Err(e.into())).await;
                                        break;
                                    }
                                },
                                Message::Ping(p) => {
                                    let _ = self.request_tx.send(WSMessage::Pong(p)).await;
                                }
                                Message::Close(_) => break,
                                _ => (),
                            },
                            Err(e) => {
                                let _ = self.event_tx.send(Err(e.into())).await;
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
                Either::Right(_) => break,
            }
        }

        let mut state = self.state.lock().unwrap();
        state.is_closed = true;
    }
}

struct RequestDispatcher<OrderType> {
    state: Arc<Mutex<ClientState>>,
    request_rx: mpsc::Receiver<WSMessage<OrderType>>,
    sink: SplitSink<WebSocketStream<ConnectStream>, Message>,
}

impl<OrderType> RequestDispatcher<OrderType>
where
    OrderType: DeserializeOwned,
{
    async fn dispatch_requests(mut self) {
        while let Some(msg) = self.request_rx.recv().await {
            match msg {
                WSMessage::Pong(p) => {
                    let _ = self.sink.send(Message::Pong(p)).await;
                }
                WSMessage::Request(req) => match self.dispatch_request(&req).await {
                    Err(e) => self.return_error(e, req.id.as_ref().unwrap()).await,
                    _ => (),
                },
                _ => (),
            }
        }
    }

    async fn return_error(&self, e: Error<WSApiCode>, id: &u64) {
        let tx = {
            let mut state = self.state.lock().unwrap();
            state.requests.remove(id)
        };

        if let Some(tx) = tx {
            let _ = tx.send(Err(e));
        }
    }

    async fn dispatch_request(&mut self, req: &WSRequest) -> Result<(), WSApiCode> {
        let msg = Message::Text(serde_json::to_string(req)?);
        self.sink.send(msg).await?;
        Ok(())
    }
}

pub struct WSClient<A: WSApi> {
    close_tx: oneshot::Sender<()>,
    request_tx: mpsc::Sender<WSMessage<A::OrderType>>,
    state: Arc<Mutex<ClientState>>,
    _marker: PhantomData<A>,
}

impl<A> WSClient<A>
where
    A: WSApi,
{
    async fn connect<S>(stream: Option<WSStream<S>>) -> Result<(Self, WSClientStream<A>), WSApiCode>
    where
        S: AsRef<str>,
    {
        let path = match stream.as_ref() {
            Some(s) => format!("{}{}", A::host(), s),
            None => A::host().to_string(),
        };

        let (ws_stream, _) = connect_async(path).await?;
        let (sink, stream) = ws_stream.split();
        let (event_tx, event_rx) = mpsc::channel(100);
        let (request_tx, request_rx) = mpsc::channel(1);
        let (close_tx, close_rx) = oneshot::channel();
        let state = Arc::new(Mutex::new(ClientState {
            is_closed: false,
            next_id: 1,
            requests: BTreeMap::new(),
        }));

        {
            let state = state.clone();
            let request_dispatcher = RequestDispatcher::<A::OrderType> {
                state,
                request_rx,
                sink,
            };
            tokio::spawn(request_dispatcher.dispatch_requests());
        }
        {
            let request_tx = request_tx.clone();
            let state = state.clone();
            let event_dispatcher = EventDispatcher::<A::OrderType> {
                close_rx,
                event_tx,
                request_tx,
                state,
                stream,
            };
            tokio::spawn(event_dispatcher.dispatch_events());
        }

        Ok((
            Self {
                close_tx,
                request_tx,
                state,
                _marker: PhantomData,
            },
            WSClientStream(event_rx),
        ))
    }

    async fn send_request(&self, mut req: WSRequest) -> Result<WSResponse, WSApiCode> {
        let timeout = req.timeout;
        let (tx, rx) = oneshot::channel();
        {
            let mut state = self.state.lock().unwrap();
            if state.is_closed {
                return Err(Error::WebsocketClosed);
            }

            let id = state.next_id;
            req.id = Some(id);
            state.requests.insert(id, tx);
            state.next_id += 1;
        }

        let _ = self.request_tx.send(WSMessage::Request(req)).await;

        let wait_for_result =
            rx.map(|r| r.map_err(|_| Error::WebsocketRequestCancelled).x_flatten());

        let wait_for_timeout = match timeout {
            Some(timeout) => Either::Left(time::sleep(timeout)),
            None => Either::Right(future::pending::<()>()),
        }
        .map(|_| Err(Error::WebsocketRequestTimeout));
        futures::pin_mut!(wait_for_timeout);

        match future::select(wait_for_result, wait_for_timeout).await {
            Either::Left((result, _)) => result,
            Either::Right((timeout, _)) => timeout,
        }
    }

    pub async fn market() -> Result<(Self, WSClientStream<A>), WSApiCode> {
        let stream: Option<WSStream<&str>> = None;
        Self::connect(stream).await
    }

    pub async fn user_data<S>(listen_key: S) -> Result<(Self, WSClientStream<A>), WSApiCode>
    where
        S: AsRef<str>,
    {
        Self::connect(Some(WSStream::UserData(listen_key))).await
    }

    pub fn close(self) {
        let _ = self.close_tx.send(());
    }

    pub fn is_closed(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.is_closed
    }

    pub async fn subscribe<S>(&self, stream: WSStream<S>) -> Result<WSResponse, WSApiCode>
    where
        S: AsRef<str>,
    {
        self.send_request(
            WSRequest::new(WSRequestMethod::Subscribe)
                .stream(stream)
                .timeout(WS_REQUEST_TIMEOUT),
        )
        .await
    }

    pub async fn unsubscribe<S>(&self, stream: WSStream<S>) -> Result<WSResponse, WSApiCode>
    where
        S: AsRef<str>,
    {
        self.send_request(
            WSRequest::new(WSRequestMethod::Unsubscribe)
                .stream(stream)
                .timeout(WS_REQUEST_TIMEOUT),
        )
        .await
    }

    pub async fn list_subscriptions(&self) -> Result<WSResponse, WSApiCode> {
        self.send_request(
            WSRequest::new(WSRequestMethod::ListSubscriptions).timeout(WS_REQUEST_TIMEOUT),
        )
        .await
    }

    pub async fn get_property<S>(&self, property: S) -> Result<WSResponse, WSApiCode>
    where
        S: AsRef<str>,
    {
        self.send_request(
            WSRequest::new(WSRequestMethod::GetProperty)
                .get_property(property)
                .timeout(WS_REQUEST_TIMEOUT),
        )
        .await
    }

    pub async fn set_property<S, T>(&self, property: S, value: T) -> Result<WSResponse, WSApiCode>
    where
        S: AsRef<str>,
        T: Serialize,
    {
        self.send_request(
            WSRequest::new(WSRequestMethod::SetProperty)
                .set_property(property, value)
                .timeout(WS_REQUEST_TIMEOUT),
        )
        .await
    }
}

pub struct WSClientStream<A: WSApi>(mpsc::Receiver<Result<WSEvent<A::OrderType>, WSApiCode>>);

impl<A> Stream for WSClientStream<A>
where
    A: WSApi,
{
    type Item = Result<WSEvent<A::OrderType>, WSApiCode>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.poll_recv(cx)
    }
}

pub trait WSApi: Send + Sync + Unpin + 'static {
    type OrderType: DeserializeOwned + Send;

    fn host() -> &'static str;
}

pub struct WSFApi;
impl WSApi for WSFApi {
    type OrderType = FOrderType;

    fn host() -> &'static str {
        WSFAPI_HOST
    }
}

pub struct WSSApi;
impl WSApi for WSSApi {
    type OrderType = SOrderType;

    fn host() -> &'static str {
        WSSAPI_HOST
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_wsclient() {
//         let (client, stream) = WSFClient::market().await.unwrap();
//         client
//             .send_request(
//                 WSRequest::new(WSRequestMethod::Subscribe).stream(WSStream::AggTrade("BTCUSDT")),
//             )
//             .await
//             .unwrap();
//         {
//             tokio::spawn(async move {
//                 time::sleep(time::Duration::from_secs(10)).await;
//                 client.close();
//             });
//         }
//         stream
//             .for_each(|e| {
//                 if let Ok(e) = e {
//                     eprintln!("{:?}", e);
//                 }
//                 future::ready(())
//             })
//             .await;
//     }

//     #[tokio::test]
//     async fn test_partial_depth() {
//         let (client, stream) = WSFClient::market().await.unwrap();
//         client
//             .send_request(
//                 WSRequest::new(WSRequestMethod::Subscribe)
//                     .stream(WSStream::PartialBookDepth500ms("BTCUSDT", 20)),
//             )
//             .await
//             .unwrap();
//         {
//             tokio::spawn(async move {
//                 time::sleep(time::Duration::from_secs(10)).await;
//                 client.close();
//             });
//         }
//         stream
//             .for_each(|e| {
//                 if let Ok(e) = e {
//                     eprintln!("{:?}", e);
//                 }
//                 future::ready(())
//             })
//             .await;
//     }
// }
