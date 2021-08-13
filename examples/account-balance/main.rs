use std::env;
use std::error::Error;

use binance_api::{
    account::Account,
    client::{Credentials, FClient},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get credentials.
    let api_key = env::var("API_KEY")?;
    let mut api_key_parts = api_key.split(":");
    let credentials = match (api_key_parts.next(), api_key_parts.next()) {
        (Some(key), Some(secret)) => Credentials::new(key.to_string(), secret.to_string()),
        _ => return Err("Invalid API_KEY value, must be '<API_KEY>:<SECRET_KEY>'.".into()),
    };

    // Create a Futures API client.
    let client = FClient::with_credentials(credentials);

    // Create a new Account API provider.
    let account = Account::new(client);

    // Get the balance of the Futures wallet.
    let balances = account.balance().await?;

    // Print the balances of the Futures wallet.
    println!("{:?}", balances);

    Ok(())
}
