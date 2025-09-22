use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct Identity {
    pub authentication_token: String,
    pub session_cookie: String,
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    #[serde(rename = "entityUrn")]
    pub entity_urn: Option<String>,

    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub headline: Option<String>,
    pub summary: Option<String>,
    pub industry_name: Option<String>,
    #[serde(rename = "industryUrn")]
    pub industry_urn: Option<String>,

    pub geo_country_name: Option<String>,
    #[serde(rename = "geoCountryUrn")]
    pub geo_country_urn: Option<String>,
    #[serde(rename = "geoLocationName")]
    pub geo_location_name: Option<String>,
    #[serde(rename = "geoLocationBackfilled")]
    pub geo_location_backfilled: Option<bool>,

    pub address: Option<String>,
    pub birth_date: Option<BirthDate>,

    pub default_locale: Option<Locale>,
    pub supported_locales: Option<Vec<Locale>>,

    pub location: Option<Location>,
    #[serde(rename = "locationName")]
    pub location_name: Option<String>,

    #[serde(rename = "miniProfile")]
    pub mini_profile: Option<MiniProfile>,

    pub profile_picture: Option<ProfilePicture>,
    #[serde(rename = "profilePictureOriginalImage")]
    pub profile_picture_original_image: Option<VectorImageContainer>,

    #[serde(rename = "showEducationOnProfileTopCard")]
    pub show_education_on_profile_top_card: Option<bool>,
    pub student: Option<bool>,

    #[serde(rename = "versionTag")]
    pub version_tag: Option<String>,

    #[serde(skip)]
    pub profile_id: String, // extracted from entityUrn with get_id_from_urn

    #[serde(skip)]
    pub experience: Vec<Experience>,

    #[serde(skip)]
    pub education: Vec<Education>,

    #[serde(skip)]
    pub skills: Vec<Skill>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BirthDate {
    pub day: Option<u32>,
    pub month: Option<u32>,
    pub year: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Locale {
    pub country: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub basic_location: Option<BasicLocation>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicLocation {
    pub country_code: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniProfile {
    #[serde(rename = "dashEntityUrn")]
    pub dash_entity_urn: Option<String>,
    #[serde(rename = "entityUrn")]
    pub entity_urn: Option<String>,
    #[serde(rename = "objectUrn")]
    pub object_urn: Option<String>,
    pub public_identifier: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub occupation: Option<String>,
    pub tracking_id: Option<String>,
    pub picture: Option<VectorImageContainer>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilePicture {
    pub display_image: Option<String>,
    pub original_image: Option<String>,
    pub photo_filter_edit_info: Option<PhotoFilterEditInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotoFilterEditInfo {
    pub top_left: Option<Point>,
    pub top_right: Option<Point>,
    pub bottom_left: Option<Point>,
    pub bottom_right: Option<Point>,
    pub brightness: Option<f32>,
    pub contrast: Option<f32>,
    pub saturation: Option<f32>,
    pub vignette: Option<f32>,
    pub photo_filter_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VectorImageContainer {
    #[serde(rename = "com.linkedin.common.VectorImage")]
    pub vector_image: Option<VectorImage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VectorImage {
    pub root_url: Option<String>,
    pub artifacts: Vec<ImageArtifact>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageArtifact {
    pub height: u32,
    pub width: u32,
    pub expires_at: Option<u64>,
    pub file_identifying_url_path_segment: Option<String>,
}

/// An experience entry
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Experience {
    pub title: Option<String>,
    #[serde(rename = "companyName")]
    pub company_name: Option<String>,
    #[serde(rename = "companyLogoUrl")]
    pub company_logo_url: Option<String>,
    pub description: Option<String>,
}

/// An education entry
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Education {
    pub school_name: Option<String>,
    pub degree: Option<String>,
    pub field_of_study: Option<String>,
}

/// A skill entry
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

pub(crate) struct UniformResourceName {
    pub(crate) namespace: String, // the context of the id
    pub(crate) id: String,
}
