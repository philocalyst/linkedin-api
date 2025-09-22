use linkedin_api_rs::{Identity, Linkedin, LinkedinError};
use std::env;

#[tokio::main]
async fn main() -> Result<(), LinkedinError> {
    let username = env::var("LINKEDIN_USERNAME").unwrap_or_else(|_| panic!("Missing env var"));
    let password = env::var("LINKEDIN_PASSWORD").unwrap_or_else(|_| panic!("Missing env var"));

    let input = Identity  { username, password, authentication_token: String::from("AQEDAUEPdx8FJo2CAAABmXMEsS0AAAGZlxE1LU4Aclty_bQmV4p4VWnlBVAerIOntfpKC8rMVg107RrypH6OLlgHUK0PqJ5Nssev_4lzITN-GptrMsPInTcSfuKKQwQAqJNEjhM9sWywSaYzvoobkkoc"), session_cookie: String::from("ajax:8702309092900260000") };

    let api = Linkedin::new(&input, false).await?;

    let profile = api.get_profile("ACoAABQ11fIBQLGQbB1V1XPBZJsRwfK5r1U2Rzw").await?;
    let contact_info = api.get_profile_contact_info("ACoAABQ11fIBQLGQbB1V1XPBZJsRwfK5r1U2Rzw").await?;
    let connections = api.get_profile_connections(&profile.profile_id).await?;

    println!("Profile: {:?}", profile);
    println!("Contact info: {:?}", contact_info);
    println!("Connections: {:?}", connections);
    Ok(())
}
