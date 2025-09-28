use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs::write;
use url::Url;
use urlencoding::encode;

use crate::client::Client;
use crate::error::LinkedinError;
use crate::types::ProfileView;
use crate::{
    types::Education, types::Experience, Company, Connection, ContactInfo, Conversation,
    ConversationDetails, Identity, Invitation, MemberBadges, NetworkInfo, PersonSearchResult,
    Profile, School, SearchPeopleParams, Skill, UniformResourceName,
};

const MAX_UPDATE_COUNT: usize = 100;
const MAX_SEARCH_COUNT: usize = 49;
const MAX_REPEATED_REQUESTS: usize = 200;

#[derive(Clone)]
pub struct LinkedinInner {
    client: Client,
}

impl LinkedinInner {
    pub async fn new(identity: &Identity, refresh_cookies: bool) -> Result<Self, LinkedinError> {
        let client = Client::new()?;
        client.authenticate(identity, refresh_cookies).await?;
        Ok(Self { client })
    }

    pub async fn get_profile(
        &self,
        public_id: Option<&str>,
        urn: Option<&UniformResourceName>,
    ) -> Result<ProfileView, LinkedinError> {
        let id = if let Some(pid) = public_id {
            pid.to_string()
        } else if let Some(urn) = urn {
            urn.id.clone()
        } else {
            return Err(LinkedinError::InvalidInput(
                "public_id or uniform_resource_name required".into(),
            ));
        };

        let res = self
            .client
            .get(&format!("/identity/profiles/{id}/profileView"))
            .await?;
        if res.status() != 200 {
            return Err(LinkedinError::RequestFailed(format!(
                "status {}",
                res.status()
            )));
        }

        let data: serde_json::Value = res.json().await?;

        let mut profile_view: ProfileView = serde_json::from_value(data)?;

        // Derive helper fields not serialized directly
        if let Some(mini) = &profile_view.profile.mini_profile {
            if let Some(urn_str) = mini.entity_urn.as_deref() {
                if let Ok(urn) = UniformResourceName::parse(urn_str) {
                    profile_view.profile.profile_id = urn.id;
                }
            }
        }

        // Fill in profile_id
        if let Some(mini) = &profile_view.profile.mini_profile {
            if let Some(urn_str) = mini.entity_urn.as_deref() {
                if let Ok(urn) = UniformResourceName::parse(urn_str) {
                    profile_view.profile.profile_id = urn.id;
                }
            }
        }

        // Fill in skills (separate endpoint)
        profile_view.skills = self.get_profile_skills(public_id, urn).await?;

        // Fill in contact info (separate endpoint)
        profile_view.profile.contact = self.get_profile_contact_info(public_id, urn).await?;

        Ok(profile_view)
    }

    pub async fn get_profile_contact_info(
        &self,
        public_id: Option<&str>,
        uniform_resource_name: Option<&UniformResourceName>,
    ) -> Result<ContactInfo, LinkedinError> {
        let id = if let Some(pid) = public_id {
            pid.to_string() // use raw string
        } else if let Some(urn) = uniform_resource_name {
            urn.id.clone() // use strong type's .id
        } else {
            return Err(LinkedinError::InvalidInput(
                "Either public_id or uniform_resource_name must be provided".into(),
            ));
        };

        let res = self
            .client
            .get(&format!("/identity/profiles/{id}/profileContactInfo"))
            .await?;

        let data: Value = res.json().await?;

        let mut contact_info = ContactInfo {
            email_address: data
                .get("emailAddress")
                .and_then(|e| e.as_str())
                .map(|s| s.parse().unwrap()),

            websites: vec![],
            twitter: vec![],
            phone_numbers: vec![],
            birthdate: data
                .get("birthDateOn")
                .and_then(|b| b.as_str())
                .map(|s| s.parse().unwrap()),

            ims: data.get("ims").map(|i| vec![i.clone()]),
        };

        // Parse websites
        if let Some(websites) = data.get("websites").and_then(|w| w.as_array()) {
            for website in websites {
                let mut site = crate::types::Website {
                    url: Some(
                        website
                            .get("url")
                            .and_then(|u| u.as_str())
                            .unwrap_or_default()
                            .parse()
                            .unwrap(),
                    ),
                    label: None,
                };

                if let Some(website_type) = website.get("type") {
                    if let Some(standard) =
                        website_type.get("com.linkedin.voyager.identity.profile.StandardWebsite")
                    {
                        site.label = standard
                            .get("category")
                            .and_then(|c| c.as_str())
                            .map(|s| s.to_string());
                    } else if let Some(custom) =
                        website_type.get("com.linkedin.voyager.identity.profile.CustomWebsite")
                    {
                        site.label = custom
                            .get("label")
                            .and_then(|l| l.as_str())
                            .map(|s| s.to_string());
                    }
                }

                contact_info.websites.push(site);
            }
        }

        // Parse Twitter handles
        if let Some(twitter_handles) = data.get("twitterHandles").and_then(|t| t.as_array()) {
            for handle in twitter_handles {
                if let Some(name) = handle.get("name").and_then(|n| n.as_str()) {
                    contact_info.twitter.push(name.to_string());
                }
            }
        }

        // Parse phone numbers
        if let Some(phone_numbers) = data.get("phoneNumbers").and_then(|p| p.as_array()) {
            for phone in phone_numbers {
                if let Some(number) = phone.get("number").and_then(|n| n.as_str()) {
                    contact_info.phone_numbers.push(number.parse().unwrap());
                }
            }
        }

        Ok(contact_info)
    }

    pub async fn get_profile_skills(
        &self,
        public_id: Option<&str>,
        uniform_resource_name: Option<&UniformResourceName>,
    ) -> Result<Vec<Skill>, LinkedinError> {
        let id = if let Some(pid) = public_id {
            pid.to_string() // use raw string
        } else if let Some(urn) = uniform_resource_name {
            urn.id.clone() // use strong type's .id
        } else {
            return Err(LinkedinError::InvalidInput(
                "Either public_id or uniform_resource_name must be provided".into(),
            ));
        };

        let res = self
            .client
            .get(&format!("/identity/profiles/{id}/skills?count=100&start=0"))
            .await?;

        let data: Value = res.json().await?;

        let mut skills = vec![];

        if let Some(elements) = data.get("elements").and_then(|e| e.as_array()) {
            for element in elements {
                if let Some(name) = element.get("name").and_then(|n| n.as_str()) {
                    skills.push(Skill {
                        entity_urn: "".to_string(),
                        name: name.to_string(),
                    });
                }
            }
        }

        Ok(skills)
    }

    pub async fn get_profile_connections(
        &self,
        uniform_resource_name: &str,
    ) -> Result<Vec<Connection>, LinkedinError> {
        let params = SearchPeopleParams {
            connection_of: Some(uniform_resource_name.to_string()),
            network_depth: Some("F".to_string()),
            ..Default::default()
        };

        let results = self.search_people(params).await?;

        Ok(results
            .into_iter()
            .map(|r| Connection {
                urn_id: r.urn_id,
                public_id: r.public_id,
                distance: r.distance,
            })
            .collect())
    }

    pub async fn search(
        &self,
        mut params: HashMap<String, String>,
        limit: Option<usize>,
    ) -> Result<Vec<Value>, LinkedinError> {
        let count = limit.unwrap_or(MAX_SEARCH_COUNT).min(MAX_SEARCH_COUNT);

        let default_params = vec![
            ("count".to_string(), count.to_string()),
            ("filters".to_string(), "List()".to_string()),
            ("origin".to_string(), "GLOBAL_SEARCH_HEADER".to_string()),
            ("q".to_string(), "all".to_string()),
            ("start".to_string(), "0".to_string()),
            ("queryContext".to_string(), "List(spellCorrectionEnabled->true,relatedSearchesEnabled->true,kcardTypes->PROFILE|COMPANY)".to_string()),
        ];

        for (key, value) in default_params {
            params.entry(key).or_insert(value);
        }

        let mut results = vec![];
        let mut start = 0;
        let target_limit = limit.unwrap_or(usize::MAX);

        loop {
            params.insert("start".to_string(), start.to_string());

            let query_string: String = params
                .iter()
                .map(|(k, v)| format!("{}={}", encode(k), encode(v)))
                .collect::<Vec<_>>()
                .join("&");

            let res = self
                .client
                .get(&format!("/search/blended?{query_string}"))
                .await?;
            let data: Value = res.json().await?;

            let mut new_elements = vec![];

            if let Some(elements) = data
                .get("data")
                .and_then(|d| d.get("elements"))
                .and_then(|e| e.as_array())
            {
                for element in elements {
                    if let Some(inner_elements) = element.get("elements").and_then(|e| e.as_array())
                    {
                        new_elements.extend(inner_elements.iter().cloned());
                    }
                }
            }

            if new_elements.is_empty() {
                break;
            }

            results.extend(
                new_elements
                    .iter()
                    .take(target_limit.saturating_sub(results.len()))
                    .cloned(),
            );

            if results.len() >= target_limit || results.len() / count >= MAX_REPEATED_REQUESTS {
                break;
            }

            start += count;
        }

        Ok(results.into_iter().take(target_limit).collect())
    }

    pub async fn search_people(
        &self,
        params: SearchPeopleParams,
    ) -> Result<Vec<PersonSearchResult>, LinkedinError> {
        let mut filters = vec!["resultType->PEOPLE".to_string()];

        if let Some(connection_of) = &params.connection_of {
            filters.push(format!("connectionOf->{connection_of}"));
        }
        if let Some(network_depth) = &params.network_depth {
            filters.push(format!("network->{network_depth}"));
        }
        if let Some(regions) = &params.regions {
            filters.push(format!("geoRegion->{}", regions.join("|")));
        }
        if let Some(industries) = &params.industries {
            filters.push(format!("industry->{}", industries.join("|")));
        }
        if let Some(current_company) = &params.current_company {
            filters.push(format!("currentCompany->{}", current_company.join("|")));
        }
        if let Some(past_companies) = &params.past_companies {
            filters.push(format!("pastCompany->{}", past_companies.join("|")));
        }
        if let Some(profile_languages) = &params.profile_languages {
            filters.push(format!("profileLanguage->{}", profile_languages.join("|")));
        }
        if let Some(nonprofit_interests) = &params.nonprofit_interests {
            filters.push(format!(
                "nonprofitInterest->{}",
                nonprofit_interests.join("|")
            ));
        }
        if let Some(schools) = &params.schools {
            filters.push(format!("schools->{}", schools.join("|")));
        }

        let mut search_params = HashMap::new();
        search_params.insert(
            "filters".to_string(),
            format!("List({})", filters.join(",")),
        );

        if let Some(keywords) = &params.keywords {
            search_params.insert("keywords".to_string(), keywords.clone());
        }

        let data = self.search(search_params, params.limit).await?;

        let mut results = vec![];
        for item in data {
            if let Some(public_id) = item.get("publicIdentifier").and_then(|p| p.as_str()) {
                let urn_id = item
                    .get("targetUrn")
                    .and_then(|u| u.as_str())
                    .and_then(|s| UniformResourceName::parse(s).ok())
                    .map(|urn| urn.id)
                    .unwrap_or_default();
                let distance = item
                    .get("memberDistance")
                    .and_then(|d| d.get("value"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                results.push(PersonSearchResult {
                    urn_id: urn_id.to_string(),
                    public_id: public_id.to_string(),
                    distance: distance.to_string(),
                });
            }
        }

        Ok(results)
    }

    pub async fn get_company_updates(
        &self,
        public_id: Option<&str>,
        uniform_resource_name: Option<&str>,
        max_results: Option<usize>,
    ) -> Result<Vec<Value>, LinkedinError> {
        let id = public_id.or(uniform_resource_name).ok_or_else(|| {
            LinkedinError::InvalidInput(
                "Either public_id or uniform_resource_name must be provided".to_string(),
            )
        })?;

        let mut results = vec![];
        let mut start = 0;
        let max_results = max_results.unwrap_or(usize::MAX);

        loop {
            let params = format!("?companyUniversalName={id}&q=companyFeedByUniversalName&moduleKey=member-share&count={MAX_UPDATE_COUNT}&start={start}");

            let res = self.client.get(&format!("/feed/updates{params}")).await?;
            let data: Value = res.json().await?;

            if let Some(elements) = data.get("elements").and_then(|e| e.as_array()) {
                if elements.is_empty()
                    || results.len() >= max_results
                    || results.len() / MAX_UPDATE_COUNT >= MAX_REPEATED_REQUESTS
                {
                    break;
                }

                results.extend(
                    elements
                        .iter()
                        .take(max_results.saturating_sub(results.len()))
                        .cloned(),
                );
                start += MAX_UPDATE_COUNT;
            } else {
                break;
            }
        }

        Ok(results)
    }

    pub async fn get_profile_updates(
        &self,
        public_id: Option<&str>,
        uniform_resource_name: Option<&str>,
        max_results: Option<usize>,
    ) -> Result<Vec<Value>, LinkedinError> {
        let id = public_id.or(uniform_resource_name).ok_or_else(|| {
            LinkedinError::InvalidInput(
                "Either public_id or uniform_resource_name must be provided".to_string(),
            )
        })?;

        let mut results = vec![];
        let mut start = 0;
        let max_results = max_results.unwrap_or(usize::MAX);

        loop {
            let params = format!(
                "?profileId={id}&q=memberShareFeed&moduleKey=member-share&count={MAX_UPDATE_COUNT}&start={start}"
            );

            let res = self.client.get(&format!("/feed/updates{params}")).await?;
            let data: Value = res.json().await?;

            if let Some(elements) = data.get("elements").and_then(|e| e.as_array()) {
                if elements.is_empty()
                    || results.len() >= max_results
                    || results.len() / MAX_UPDATE_COUNT >= MAX_REPEATED_REQUESTS
                {
                    break;
                }

                results.extend(
                    elements
                        .iter()
                        .take(max_results.saturating_sub(results.len()))
                        .cloned(),
                );
                start += MAX_UPDATE_COUNT;
            } else {
                break;
            }
        }

        Ok(results)
    }

    pub async fn get_current_profile_views(&self) -> Result<u64, LinkedinError> {
        let res = self.client.get("/identity/wvmpCards").await?;
        let data: Value = res.json().await?;

        let views = data
            .get("elements")
            .and_then(|e| e.get(0))
            .and_then(|e| e.get("value"))
            .and_then(|v| v.get("com.linkedin.voyager.identity.me.wvmpOverview.WvmpViewersCard"))
            .and_then(|c| c.get("insightCards"))
            .and_then(|i| i.get(0))
            .and_then(|i| i.get("value"))
            .and_then(|v| {
                v.get("com.linkedin.voyager.identity.me.wvmpOverview.WvmpSummaryInsightCard")
            })
            .and_then(|s| s.get("numViews"))
            .and_then(|n| n.as_u64())
            .unwrap_or(0);

        Ok(views)
    }

    pub async fn get_school(&self, public_id: &str) -> Result<School, LinkedinError> {
        let params = format!("?decorationId=com.linkedin.voyager.deco.organization.web.WebFullCompanyMain-12&q=universalName&universalName={public_id}");

        let res = self
            .client
            .get(&format!("/organization/companies{params}"))
            .await?;
        let data: Value = res.json().await?;

        if let Some(status) = data.get("status") {
            if status != 200 {
                return Err(LinkedinError::RequestFailed(
                    "School request failed".to_string(),
                ));
            }
        }

        let school_data = data
            .get("elements")
            .and_then(|e| e.get(0))
            .ok_or_else(|| LinkedinError::RequestFailed("No school data found".to_string()))?;

        let name = school_data
            .get("name")
            .and_then(|n| n.as_str())
            .ok_or_else(|| LinkedinError::RequestFailed("No school name found".to_string()))?;

        Ok(School {
            name: name.to_string(),
        })
    }

    pub async fn get_company(&self, public_id: &str) -> Result<Company, LinkedinError> {
        let params = format!("?decorationId=com.linkedin.voyager.deco.organization.web.WebFullCompanyMain-12&q=universalName&universalName={public_id}");

        let res = self
            .client
            .get(&format!("/organization/companies{params}"))
            .await?;
        let data: Value = res.json().await?;

        if let Some(status) = data.get("status") {
            if status != 200 {
                return Err(LinkedinError::RequestFailed(
                    data.get("message")
                        .unwrap_or(&Value::String("Unknown error".to_string()))
                        .as_str()
                        .unwrap()
                        .to_string(),
                ));
            }
        }

        let company_data = data
            .get("elements")
            .and_then(|e| e.get(0))
            .ok_or_else(|| LinkedinError::RequestFailed("No company data found".to_string()))?;

        let name = company_data
            .get("name")
            .and_then(|n| n.as_str())
            .ok_or_else(|| LinkedinError::RequestFailed("No company name found".to_string()))?;

        Ok(Company {
            name: name.to_string(),
        })
    }

    pub async fn get_conversation_details(
        &self,
        profile_uniform_resource_name: &str,
    ) -> Result<ConversationDetails, LinkedinError> {
        let res = self.client.get(&format!("/messaging/conversations?keyVersion=LEGACY_INBOX&q=participants&recipients=List({profile_uniform_resource_name})")).await?;
        let data: Value = res.json().await?;

        let item = data
            .get("elements")
            .and_then(|e| e.get(0))
            .ok_or_else(|| LinkedinError::RequestFailed("No conversation found".to_string()))?;

        let entity_urn = item
            .get("entityUrn")
            .and_then(|u| u.as_str())
            .ok_or(LinkedinError::RequestFailed("No entityUrn".into()))?;
        let urn = UniformResourceName::parse(entity_urn)?;
        let id = urn.id;

        Ok(ConversationDetails { id: id.to_string() })
    }

    pub async fn get_conversations(&self) -> Result<Vec<Conversation>, LinkedinError> {
        let res = self
            .client
            .get("/messaging/conversations?keyVersion=LEGACY_INBOX")
            .await?;
        let data: Value = res.json().await?;

        let mut conversations = vec![];

        if let Some(elements) = data.get("elements").and_then(|e| e.as_array()) {
            for element in elements {
                if let Some(entity_urn) = element.get("entityUrn").and_then(|u| u.as_str()) {
                    let id = UniformResourceName::parse(entity_urn).unwrap().id;
                    conversations.push(Conversation { id });
                }
            }
        }

        Ok(conversations)
    }

    pub async fn get_conversation(
        &self,
        conversation_uniform_resource_name: &str,
    ) -> Result<Conversation, LinkedinError> {
        let res = self
            .client
            .get(&format!(
                "/messaging/conversations/{conversation_uniform_resource_name}/events"
            ))
            .await?;
        let _data: Value = res.json().await?;

        Ok(Conversation {
            id: conversation_uniform_resource_name.to_string(),
        })
    }

    pub async fn send_message(
        &self,
        conversation_uniform_resource_name: Option<&str>,
        recipients: Option<Vec<String>>,
        message_body: &str,
    ) -> Result<bool, LinkedinError> {
        if conversation_uniform_resource_name.is_none() && recipients.is_none() {
            return Ok(true); // Error case
        }

        if message_body.is_empty() {
            return Ok(true); // Error case
        }

        let message_event = json!({
            "eventCreate": {
                "value": {
                    "com.linkedin.voyager.messaging.create.MessageCreate": {
                        "body": message_body,
                        "attachments": [],
                        "attributedBody": {
                            "text": message_body,
                            "attributes": []
                        },
                        "mediaAttachments": []
                    }
                }
            }
        });

        let res = if let Some(conv_id) = conversation_uniform_resource_name {
            self.client
                .post(
                    &format!("/messaging/conversations/{conv_id}/events?action=create"),
                    &message_event,
                )
                .await?
        } else if let Some(recips) = recipients {
            let mut payload = message_event;
            payload["recipients"] = json!(recips);
            payload["subtype"] = json!("MEMBER_TO_MEMBER");

            let full_payload = json!({
                "keyVersion": "LEGACY_INBOX",
                "conversationCreate": payload
            });

            self.client
                .post("/messaging/conversations?action=create", &full_payload)
                .await?
        } else {
            return Ok(true); // Error case
        };

        Ok(res.status() != 201)
    }

    pub async fn mark_conversation_as_seen(
        &self,
        conversation_uniform_resource_name: &str,
    ) -> Result<bool, LinkedinError> {
        let payload = json!({
            "patch": {
                "$set": {
                    "read": true
                }
            }
        });

        let res = self
            .client
            .post(
                &format!("/messaging/conversations/{conversation_uniform_resource_name}"),
                &payload,
            )
            .await?;
        Ok(res.status() != 200)
    }

    pub async fn get_user_profile(&self) -> Result<Value, LinkedinError> {
        crate::utils::evade().await;
        let res = self.client.get("/me").await?;
        res.json().await.map_err(Into::into)
    }

    pub async fn get_invitations(
        &self,
        start: usize,
        limit: usize,
    ) -> Result<Vec<Invitation>, LinkedinError> {
        let params =
            format!("?start={start}&count={limit}&includeInsights=true&q=receivedInvitation");

        let res = self
            .client
            .get(&format!("/relationships/invitationViews{params}"))
            .await?;

        if res.status() != 200 {
            return Ok(vec![]);
        }

        let data: Value = res.json().await?;

        let mut invitations = vec![];

        if let Some(elements) = data.get("elements").and_then(|e| e.as_array()) {
            for element in elements {
                if let Some(invitation) = element.get("invitation") {
                    if let (Some(entity_urn), Some(shared_secret)) = (
                        invitation.get("entityUrn").and_then(|u| u.as_str()),
                        invitation.get("sharedSecret").and_then(|s| s.as_str()),
                    ) {
                        invitations.push(Invitation {
                            entity_urn: entity_urn.to_string(),
                            shared_secret: shared_secret.to_string(),
                        });
                    }
                }
            }
        }

        Ok(invitations)
    }

    pub async fn reply_invitation(
        &self,
        invitation_entity_urn: &str,
        invitation_shared_secret: &str,
        action: &str,
    ) -> Result<bool, LinkedinError> {
        let urn = UniformResourceName::parse(invitation_entity_urn)?;

        let payload = json!({
            "invitationId": urn.id,
            "invitationSharedSecret": invitation_shared_secret,
            "isGenericInvitation": false
        });

        let invitation_id = urn.id;
        let res = self
            .client
            .post(
                &format!("/relationships/invitations/{invitation_id}?action={action}"),
                &payload,
            )
            .await?;

        Ok(res.status() == 200)
    }

    pub async fn remove_connection(&self, public_profile_id: &str) -> Result<bool, LinkedinError> {
        let res = self
            .client
            .post(
                &format!("/identity/profiles/{public_profile_id}/profileActions?action=disconnect"),
                &json!({}),
            )
            .await?;

        Ok(res.status() != 200)
    }

    pub async fn get_profile_privacy_settings(
        &self,
        public_profile_id: &str,
    ) -> Result<HashMap<String, Value>, LinkedinError> {
        let res = self
            .client
            .get(&format!(
                "/identity/profiles/{public_profile_id}/privacySettings"
            ))
            .await?;

        if res.status() != 200 {
            return Ok(HashMap::new());
        }

        let data: Value = res.json().await?;

        if let Some(data_obj) = data.get("data").and_then(|d| d.as_object()) {
            Ok(data_obj
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect())
        } else {
            Ok(HashMap::new())
        }
    }

    pub async fn get_profile_member_badges(
        &self,
        public_profile_id: &str,
    ) -> Result<MemberBadges, LinkedinError> {
        let res = self
            .client
            .get(&format!(
                "/identity/profiles/{public_profile_id}/memberBadges"
            ))
            .await?;

        if res.status() != 200 {
            return Ok(MemberBadges {
                premium: false,
                open_link: false,
                influencer: false,
                job_seeker: false,
            });
        }

        let data: Value = res.json().await?;

        let empty_map = Value::Object(serde_json::Map::new());
        let badges_data = data.get("data").unwrap_or(&empty_map);

        Ok(MemberBadges {
            premium: badges_data
                .get("premium")
                .and_then(|p| p.as_bool())
                .unwrap_or(false),
            open_link: badges_data
                .get("openLink")
                .and_then(|o| o.as_bool())
                .unwrap_or(false),
            influencer: badges_data
                .get("influencer")
                .and_then(|i| i.as_bool())
                .unwrap_or(false),
            job_seeker: badges_data
                .get("jobSeeker")
                .and_then(|j| j.as_bool())
                .unwrap_or(false),
        })
    }

    pub async fn get_profile_network_info(
        &self,
        public_profile_id: &str,
    ) -> Result<NetworkInfo, LinkedinError> {
        let res = self
            .client
            .get(&format!(
                "/identity/profiles/{public_profile_id}/networkinfo"
            ))
            .await?;

        if res.status() != 200 {
            return Ok(NetworkInfo { followers_count: 0 });
        }

        let data: Value = res.json().await?;

        let followers_count = data
            .get("data")
            .and_then(|d| d.get("followersCount"))
            .and_then(|f| f.as_u64())
            .unwrap_or(0);

        Ok(NetworkInfo { followers_count })
    }

    pub async fn stub_people_search(
        &self,
        query: &str,
        count: usize,
        start: usize,
    ) -> Result<Value, LinkedinError> {
        let encoded_query = encode(query);

        let mut url = format!("/search/hits?count={count}&guides=List%28v-%253EPEOPLE%29&keywords={encoded_query}&origin=SWITCH_SEARCH_VERTICAL&q=guided");

        if start > 0 {
            url.push_str(&format!("&start={start}"));
        }

        let res = self.client.get(&url).await?;

        if res.status() != 200 {
            return Ok(json!({}));
        }

        res.json().await.map_err(Into::into)
    }
}
