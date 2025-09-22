//! Rust Wrapper for the LinkedIn API
//!
//! This crate provides an asynchronous interface to the LinkedIn Voyager API.
//!
//! # Example
//!
//! ```no_run
//! use linkedin_api_rs::Linkedin;
//! use std::env;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), linkedin_api_rs::LinkedinError> {
//!     let username = env::var("LINKEDIN_USERNAME").unwrap();
//!     let password = env::var("LINKEDIN_PASSWORD").unwrap();
//!
//!     let api = Linkedin::new(&username, &password, false).await?;
//!
//!     let profile = api.get_profile("billy-g").await?;
//!
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub use crate::error::LinkedinError;
use crate::linkedin::LinkedinInner;

pub mod client;
pub mod error;
pub mod linkedin;
pub mod utils;

/// Main struct for interacting with the LinkedIn API asynchronously.
#[derive(Clone)]
pub struct Linkedin {
    inner: LinkedinInner,
}

pub struct Identity {
    pub username: String,
    pub password: String,
    pub authentication_token: String,
    pub session_cookie: String,
}

impl Linkedin {
    /// Create a new LinkedIn client and authenticate.
    pub async fn new(identity: &Identity, refresh_cookies: bool) -> Result<Self, LinkedinError> {
        let inner = LinkedinInner::new(identity, refresh_cookies).await?;
        Ok(Self { inner })
    }

    /// Returns a LinkedIn profile.
    pub async fn get_profile(&self, public_id: &str) -> Result<Profile, LinkedinError> {
        self.inner.get_profile(Some(public_id), None).await
    }

    /// Returns a LinkedIn profile by URN ID.
    pub async fn get_profile_by_urn(&self, urn_id: &str) -> Result<Profile, LinkedinError> {
        self.inner.get_profile(None, Some(urn_id)).await
    }

    /// Returns a LinkedIn profile's first degree connections.
    pub async fn get_profile_connections(&self, urn_id: &str) -> Result<Vec<Connection>, LinkedinError> {
        self.inner.get_profile_connections(urn_id).await
    }

    /// Returns a LinkedIn profile's contact information.
    pub async fn get_profile_contact_info(&self, public_id: &str) -> Result<ContactInfo, LinkedinError> {
        self.inner.get_profile_contact_info(Some(public_id), None).await
    }

    /// Returns a LinkedIn profile's contact information by URN ID.
    pub async fn get_profile_contact_info_by_urn(&self, urn_id: &str) -> Result<ContactInfo, LinkedinError> {
        self.inner.get_profile_contact_info(None, Some(urn_id)).await
    }

    /// Returns a LinkedIn profile's skills.
    pub async fn get_profile_skills(&self, public_id: &str) -> Result<Vec<Skill>, LinkedinError> {
        self.inner.get_profile_skills(Some(public_id), None).await
    }

    /// Returns a LinkedIn profile's skills by URN ID.
    pub async fn get_profile_skills_by_urn(&self, urn_id: &str) -> Result<Vec<Skill>, LinkedinError> {
        self.inner.get_profile_skills(None, Some(urn_id)).await
    }

    /// Returns a LinkedIn profile's privacy settings.
    pub async fn get_profile_privacy_settings(&self, public_id: &str) -> Result<HashMap<String, Value>, LinkedinError> {
        self.inner.get_profile_privacy_settings(public_id).await
    }

    /// Returns a LinkedIn profile's member badges.
    pub async fn get_profile_member_badges(&self, public_id: &str) -> Result<MemberBadges, LinkedinError> {
        self.inner.get_profile_member_badges(public_id).await
    }

    /// Returns high-level network info for a profile.
    pub async fn get_profile_network_info(&self, public_id: &str) -> Result<NetworkInfo, LinkedinError> {
        self.inner.get_profile_network_info(public_id).await
    }

    /// Removes a connection.
    pub async fn remove_connection(&self, public_id: &str) -> Result<bool, LinkedinError> {
        self.inner.remove_connection(public_id).await
    }

    /// Return list of metadata of the user's conversations.
    pub async fn get_conversations(&self) -> Result<Vec<Conversation>, LinkedinError> {
        self.inner.get_conversations().await
    }

    /// Return conversation details for a profile URN ID.
    pub async fn get_conversation_details(&self, profile_urn_id: &str) -> Result<ConversationDetails, LinkedinError> {
        self.inner.get_conversation_details(profile_urn_id).await
    }

    /// Return a conversation.
    pub async fn get_conversation(&self, conversation_urn_id: &str) -> Result<Conversation, LinkedinError> {
        self.inner.get_conversation(conversation_urn_id).await
    }

    /// Sends a message to a conversation or recipients.
    pub async fn send_message(&self, conversation_urn_id: Option<&str>, recipients: Option<Vec<String>>, message_body: &str) -> Result<bool, LinkedinError> {
        self.inner.send_message(conversation_urn_id, recipients, message_body).await
    }

    /// Mark a conversation as seen.
    pub async fn mark_conversation_as_seen(&self, conversation_urn_id: &str) -> Result<bool, LinkedinError> {
        self.inner.mark_conversation_as_seen(conversation_urn_id).await
    }

    /// Get view statistics for the current profile.
    pub async fn get_current_profile_views(&self) -> Result<u64, LinkedinError> {
        self.inner.get_current_profile_views().await
    }

    /// Returns a school's LinkedIn profile.
    pub async fn get_school(&self, public_id: &str) -> Result<School, LinkedinError> {
        self.inner.get_school(public_id).await
    }

    /// Returns a company's LinkedIn profile.
    pub async fn get_company(&self, public_id: &str) -> Result<Company, LinkedinError> {
        self.inner.get_company(public_id).await
    }

    /// Perform a LinkedIn search.
    pub async fn search(&self, params: HashMap<String, String>, limit: Option<usize>) -> Result<Vec<Value>, LinkedinError> {
        self.inner.search(params, limit).await
    }

    /// Perform a people search.
    pub async fn search_people(&self, params: SearchPeopleParams) -> Result<Vec<PersonSearchResult>, LinkedinError> {
        self.inner.search_people(params).await
    }

    /// Get company updates.
    pub async fn get_company_updates(&self, public_id: Option<&str>, urn_id: Option<&str>, max_results: Option<usize>) -> Result<Vec<Value>, LinkedinError> {
        self.inner.get_company_updates(public_id, urn_id, max_results).await
    }

    /// Get profile updates.
    pub async fn get_profile_updates(&self, public_id: Option<&str>, urn_id: Option<&str>, max_results: Option<usize>) -> Result<Vec<Value>, LinkedinError> {
        self.inner.get_profile_updates(public_id, urn_id, max_results).await
    }

    /// Get all invitations for the current profile.
    pub async fn get_invitations(&self, start: usize, limit: usize) -> Result<Vec<Invitation>, LinkedinError> {
        self.inner.get_invitations(start, limit).await
    }

    /// Reply to an invitation.
    pub async fn reply_invitation(&self, invitation_entity_urn: &str, invitation_shared_secret: &str, action: &str) -> Result<bool, LinkedinError> {
        self.inner.reply_invitation(invitation_entity_urn, invitation_shared_secret, action).await
    }

    /// Get current user profile.
    pub async fn get_user_profile(&self) -> Result<Value, LinkedinError> {
        self.inner.get_user_profile().await
    }

    /// Stub people search with query.
    pub async fn stub_people_search(&self, query: &str, count: usize, start: usize) -> Result<Value, LinkedinError> {
        self.inner.stub_people_search(query, count, start).await
    }
}

/// Parameters for people search.
#[derive(Debug, Clone, Default)]
pub struct SearchPeopleParams {
    pub keywords: Option<String>,
    pub connection_of: Option<String>,
    pub network_depth: Option<String>,
    pub current_company: Option<Vec<String>>,
    pub past_companies: Option<Vec<String>>,
    pub nonprofit_interests: Option<Vec<String>>,
    pub profile_languages: Option<Vec<String>>,
    pub regions: Option<Vec<String>>,
    pub industries: Option<Vec<String>>,
    pub schools: Option<Vec<String>>,
    pub include_private_profiles: bool,
    pub limit: Option<usize>,
}

/// Strongly-typed struct for profiles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub profile_id: String,
    #[serde(rename = "displayPictureUrl")]
    pub display_picture_url: Option<String>,
    pub experience: Vec<Experience>,
    pub education: Vec<Education>,
    pub skills: Vec<Skill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub title: Option<String>,
    #[serde(rename = "companyName")]
    pub company_name: Option<String>,
    #[serde(rename = "companyLogoUrl")]
    pub company_logo_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    #[serde(rename = "schoolName")]
    pub school_name: Option<String>,
    pub degree: Option<String>,
    pub field_of_study: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    #[serde(rename = "emailAddress")]
    pub email_address: Option<String>,
    pub websites: Vec<Website>,
    pub twitter: Vec<String>,
    #[serde(rename = "phoneNumbers")]
    pub phone_numbers: Vec<String>,
    pub birthdate: Option<String>,
    pub ims: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Website {
    pub url: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub urn_id: String,
    pub public_id: String,
    pub distance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationDetails {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberBadges {
    pub premium: bool,
    pub open_link: bool,
    pub influencer: bool,
    pub job_seeker: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub followers_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct School {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonSearchResult {
    pub urn_id: String,
    pub public_id: String,
    pub distance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invitation {
    #[serde(rename = "entityUrn")]
    pub entity_urn: String,
    #[serde(rename = "sharedSecret")]
    pub shared_secret: String,
}
