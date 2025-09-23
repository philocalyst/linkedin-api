use linkedin_api::types::{Identity, SearchPeopleParams};
use linkedin_api::{Linkedin, LinkedinError};
use std::env;

fn get_test_credentials() -> (Identity, String, String) {
    let li_at = env::var("LINKEDIN_LI_AT").expect("LINKEDIN_LI_AT not set");
    let jsession_id = env::var("LINKEDIN_JSESSIONID").expect("LINKEDIN_JSESSIONID not set");
    let profile_id = env::var("TEST_PROFILE_ID").expect("TEST_PROFILE_ID not set");
    let conversation_id = env::var("TEST_CONVERSATION_ID").expect("TEST_CONVERSATION_ID not set");

    let id = Identity {
        authentication_token: li_at,
        session_cookie: jsession_id,
    };

    (id, profile_id, conversation_id)
}

#[tokio::test]
async fn test_get_profile() -> Result<(), LinkedinError> {
    let (identity, profile_id, _) = get_test_credentials();
    let api = Linkedin::new(&identity, true).await?;
    let profile = api.get_profile(&profile_id).await?;

    assert!(!profile.profile_id.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_get_profile_contact_info() -> Result<(), LinkedinError> {
    let (identity, profile_id, _) = get_test_credentials();
    let api = Linkedin::new(&identity, false).await?;
    let contact_info = api.get_profile_contact_info(&profile_id).await?;

    println!("Contact info: {:?}", contact_info);
    Ok(())
}

#[tokio::test]
async fn test_get_profile_connections() -> Result<(), LinkedinError> {
    let (identity, profile_id, _) = get_test_credentials();
    let api = Linkedin::new(&identity, false).await?;
    let connections = api.get_profile_connections(&profile_id).await?;

    println!("Found {} connections", connections.len());
    Ok(())
}

#[tokio::test]
async fn test_get_conversations() -> Result<(), LinkedinError> {
    let (identity, _, _) = get_test_credentials();
    let api = Linkedin::new(&identity, false).await?;
    let conversations = api.get_conversations().await?;

    println!("Found {} conversations", conversations.len());
    Ok(())
}

#[tokio::test]
async fn test_get_company() -> Result<(), LinkedinError> {
    let (identity, _, _) = get_test_credentials();
    let api = Linkedin::new(&identity, false).await?;
    let company = api.get_company("linkedin").await?;

    assert_eq!(company.name, "LinkedIn");
    Ok(())
}

#[tokio::test]
async fn test_get_school() -> Result<(), LinkedinError> {
    let (identity, _, _) = get_test_credentials();
    let api = Linkedin::new(&identity, false).await?;
    let school = api.get_school("university-of-queensland").await?;

    assert_eq!(school.name, "The University of Queensland");
    Ok(())
}

#[tokio::test]
async fn test_search_people() -> Result<(), LinkedinError> {
    let (identity, _, _) = get_test_credentials();
    let api = Linkedin::new(&identity, false).await?;

    let params = SearchPeopleParams {
        keywords: Some("software".to_string()),
        limit: Some(5),
        ..Default::default()
    };

    let results = api.search_people(params).await?;

    println!("Found {} people", results.len());
    assert!(!results.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_get_invitations() -> Result<(), LinkedinError> {
    let (identity, _, _) = get_test_credentials();
    let api = Linkedin::new(&identity, false).await?;
    let invitations = api.get_invitations(0, 10).await?;

    println!("Found {} invitations", invitations.len());
    Ok(())
}

#[tokio::test]
async fn test_send_message_to_conversation() -> Result<(), LinkedinError> {
    let (identity, _, conversation_id) = get_test_credentials();
    let api = Linkedin::new(&identity, false).await?;

    let err = api
        .send_message(Some(&conversation_id), None, "test message from rust")
        .await?;

    println!("Send message error: {}", err);
    Ok(())
}

#[tokio::test]
async fn test_get_profile_skills() -> Result<(), LinkedinError> {
    let (identity, profile_id, _) = get_test_credentials();
    let api = Linkedin::new(&identity, false).await?;
    let skills = api.get_profile_skills(&profile_id).await?;

    println!("Found {} skills", skills.len());
    Ok(())
}
