use linkedin_api_rs::{Linkedin, LinkedinError};
use std::env;

#[tokio::main]
async fn main() -> Result<(), LinkedinError> {
    let username = env::var("LINKEDIN_USERNAME").unwrap_or_else(|_| panic!("Missing env var"));
    let password = env::var("LINKEDIN_PASSWORD").unwrap_or_else(|_| panic!("Missing env var"));

    let api = Linkedin::new(&username, &password, false).await?;

    let profile = api.get_profile("ACoAABQ11fIBQLGQbB1V1XPBZJsRwfK5r1U2Rzw").await?;
    let contact_info = api.get_profile_contact_info("ACoAABQ11fIBQLGQbB1V1XPBZJsRwfK5r1U2Rzw").await?;
    let connections = api.get_profile_connections(&profile.profile_id).await?;

    println!("Profile: {:?}", profile);
    println!("Contact info: {:?}", contact_info);
    println!("Connections: {:?}", connections);
    Ok(())
}
