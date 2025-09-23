use linkedin_api::{types::Identity, Linkedin, LinkedinError};

#[tokio::main]
async fn main() -> Result<(), LinkedinError> {
    let input = Identity  { authentication_token: String::from("AQEDAUEPdx8FJo2CAAABmXMEsS0AAAGZlxE1LU4Aclty_bQmV4p4VWnlBVAerIOntfpKC8rMVg107RrypH6OLlgHUK0PqJ5Nssev_4lzITN-GptrMsPInTcSfuKKQwQAqJNEjhM9sWywSaYzvoobkkoc"), session_cookie: String::from("ajax:8702309092900260000") };

    let api = Linkedin::new(&input, false).await?;

    let profile = api.get_profile("miles-wirht-b3b675265").await?;

    println!("Profile: {:?}", profile);
    Ok(())
}
