use linkedin_api::client::Client;
use linkedin_api::types::Identity;
use linkedin_api::LinkedinError;
use std::env;

#[tokio::test]
async fn test_client_authenticate() -> Result<(), LinkedinError> {
    let li_at = env::var("LINKEDIN_LI_AT").expect("LINKEDIN_LI_AT not set");
    let jsession_id = env::var("LINKEDIN_JSESSIONID").expect("LINKEDIN_JSESSIONID not set");

    let id = Identity { authentication_token: li_at, session_cookie: jsession_id };
        
    let client = Client::new()?;
    client.authenticate(&id, true).await?;

    // If we get here, authentication succeeded
    Ok(())
}

#[tokio::test]
async fn test_client_get_request() -> Result<(), LinkedinError> {
    let li_at = env::var("LINKEDIN_LI_AT").expect("LINKEDIN_LI_AT not set");
    let jsession_id = env::var("LINKEDIN_JSESSIONID").expect("LINKEDIN_JSESSIONID not set");

        let id = Identity { authentication_token: li_at, session_cookie: jsession_id };

    let client = Client::new()?;

        client.authenticate(&id, true).await?;

    let res = client.get("/me").await?;

    // Just verify we can make a request
    println!("Status: {}", res.status());
    Ok(())
}
