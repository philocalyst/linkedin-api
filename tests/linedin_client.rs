use linkedin_api_rs::client::Client;
use linkedin_api_rs::LinkedinError;
use std::env;

#[tokio::test]
async fn test_client_authenticate() -> Result<(), LinkedinError> {
    let username = env::var("LINKEDIN_USERNAME").expect("LINKEDIN_USERNAME not set");
    let password = env::var("LINKEDIN_PASSWORD").expect("LINKEDIN_PASSWORD not set");

    let client = Client::new()?;
    client.authenticate(&username, &password, true).await?;

    // If we get here, authentication succeeded
    Ok(())
}

#[tokio::test]
async fn test_client_get_request() -> Result<(), LinkedinError> {
    let username = env::var("LINKEDIN_USERNAME").expect("LINKEDIN_USERNAME not set");
    let password = env::var("LINKEDIN_PASSWORD").expect("LINKEDIN_PASSWORD not set");

    let client = Client::new()?;
    client.authenticate(&username, &password, false).await?;

    let res = client.get("/me").await?;

    // Just verify we can make a request
    println!("Status: {}", res.status());
    Ok(())
}
