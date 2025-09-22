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

use serde_json::Value;
use std::collections::HashMap;

pub use crate::error::LinkedinError;
use crate::{linkedin::LinkedinInner, types::{Company, Connection, ContactInfo, Conversation, ConversationDetails, Invitation, MemberBadges, NetworkInfo, PersonSearchResult, Profile, School, SearchPeopleParams, Skill, UniformResourceName}};

pub mod client;
pub mod error;
pub mod linkedin;
pub mod utils;
pub mod types;

/// Main struct for interacting with the LinkedIn API asynchronously.
#[derive(Clone)]
pub struct Linkedin {
    inner: LinkedinInner,
}

impl UniformResourceName {
    pub fn parse(urn: &str) -> Result<Self, LinkedinError> {
        let parts: Vec<&str> = urn.split(':').collect();
        if parts.len() < 4 {
            return Err(LinkedinError::InvalidInput(
                format!("Not enough components in URN: {}", urn),
            ));
        }

        // Skip the first part (just qualifier that this is indeed an urn)

        let namespace = parts[2].to_string();
        let id = parts[3].to_string();

        Ok(Self { namespace, id })
    }

    /// Return the "id" part only, e.g. the ACoAAA… string.
    pub fn id_str(&self) -> &str {
        &self.id
    }

    /// Return the full URN as a string ("urn:li:ns:id").
    pub fn as_str(&self) -> String {
        format!("urn:li:{}:{}", self.namespace, self.id)
    }
}

impl AsRef<str> for UniformResourceName {
    fn as_ref(&self) -> &str {
        &self.id
    }
}

pub struct Identity {
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
    pub async fn get_profile_by_urn(&self, urn: &UniformResourceName) -> Result<Profile, LinkedinError> {
        self.inner.get_profile(None, Some(urn)).await
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
    pub async fn get_profile_skills_by_urn(&self, urn: UniformResourceName) -> Result<Vec<Skill>, LinkedinError> {
        self.inner.get_profile_skills(None, Some(&urn)).await
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


