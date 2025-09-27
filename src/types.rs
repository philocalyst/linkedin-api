use email_address::EmailAddress;
use isolang;
use my_country::Country;
use phonenumber::PhoneNumber;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use time::Month;
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Locale {
    pub country: Country,
    pub language: Language,
}

pub struct Identity {
    pub authentication_token: String,
    pub session_cookie: String,
}

/// The complete LinkedIn profile view structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedInProfileView {
    pub certification_view: CertificationView,
    pub course_view: CourseView,
    pub education_view: EducationView,
    pub entity_urn: String,
    pub honor_view: HonorView,
    pub language_view: LanguageView,
    pub organization_view: OrganizationView,
    pub patent_view: PatentView,
    pub position_group_view: PositionGroupView,
    pub position_view: PositionView,
    pub primary_locale: Locale,
    pub profile: Profile,
    pub project_view: ProjectView,
    pub publication_view: PublicationView,
    pub skill_view: SkillView,
    pub summary_treasury_media_count: u32,
    pub summary_treasury_medias: Vec<Value>,
    pub test_score_view: TestScoreView,
    pub volunteer_cause_view: VolunteerCauseView,
    pub volunteer_experience_view: VolunteerExperienceView,
}

/// Strongly-typed name structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonName {
    pub first: String,
    pub last: String,
}

impl PersonName {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first, self.last)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Address {
    pub raw: String,
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<Country>,
    pub postal_code: Option<String>,
}

impl Address {
    pub fn parse(raw_address: &str) -> Self {
        let parts: Vec<&str> = raw_address.split(',').map(|s| s.trim()).collect();
        Self {
            raw: raw_address.to_string(),
            street: parts.get(0).map(|s| s.to_string()),
            city: parts.get(1).map(|s| s.to_string()),
            state: parts.get(2).map(|s| s.to_string()),
            country: None,     // Cannot be derived from a simple string parse
            postal_code: None, // Cannot be derived from a simple string parse
        }
    }
}

// Custom deserializer for Address
impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Ok(Address::parse(&raw))
    }
}

/// Enhanced Profile with all fields from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub entity_urn: Option<String>,

    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub headline: Option<String>,
    pub summary: Option<String>,

    pub industry_name: Option<String>,
    pub industry_urn: Option<String>,

    pub geo_country_name: Option<String>,
    pub geo_country_urn: Option<String>,
    pub geo_location_name: Option<String>,
    pub geo_location_backfilled: Option<bool>,
    pub geo_location: Option<GeoLocation>,

    pub address: Option<Address>,
    pub birth_date: Option<BirthDate>,

    pub default_locale: Option<Locale>,
    pub supported_locales: Option<Vec<Locale>>,

    pub location: Option<Location>,
    pub location_name: Option<String>,

    pub mini_profile: Option<MiniProfile>,
    pub profile_picture: Option<ProfilePicture>,
    pub profile_picture_original_image: Option<VectorImageContainer>,

    pub show_education_on_profile_top_card: Option<bool>,
    pub student: Option<bool>,
    pub elt: Option<bool>,

    pub version_tag: Option<String>,

    #[serde(skip)]
    pub profile_id: String,

    #[serde(skip)]
    pub experience: Vec<Experience>,

    #[serde(skip)]
    pub education: Vec<Education>,

    #[serde(skip)]
    pub skills: Vec<Skill>,

    #[serde(skip)]
    pub contact: ContactInfo,
}

impl Profile {
    /// Helper method to get full name
    pub fn get_full_name(&self) -> Option<String> {
        match (&self.first_name, &self.last_name) {
            (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
            (Some(first), None) => Some(first.clone()),
            (None, Some(last)) => Some(last.clone()),
            (None, None) => None,
        }
    }

    /// Helper method to get profile image URL
    pub fn get_profile_image_url(&self) -> Option<Url> {
        self.profile_picture_original_image
            .as_ref()
            .and_then(|container| container.vector_image.as_ref())
            .and_then(|vector_image| {
                if let (Some(root_url), Some(artifact)) =
                    (&vector_image.root_url, vector_image.artifacts.first())
                {
                    let full_url = format!(
                        "{}{}",
                        root_url,
                        artifact
                            .file_identifying_url_path_segment
                            .as_deref()
                            .unwrap_or("")
                    );
                    Url::parse(&full_url).ok()
                } else {
                    None
                }
            })
    }

    /// Get profile ID from entity URN
    pub fn get_profile_id(&self) -> Option<String> {
        self.entity_urn
            .as_ref()
            .and_then(|urn| urn.split(':').last().map(|id| id.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoLocation {
    pub geo_urn: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BirthDate {
    pub day: Option<u8>,
    pub month: Option<Month>,
    pub year: Option<u16>,
}

impl BirthDate {
    /// Get as a proper date if all fields are present
    pub fn as_date(&self) -> Option<time::Date> {
        if let (Some(year), Some(month), Some(day)) = (self.year, self.month, self.day) {
            let month = Month::try_from(month).ok()?;
            time::Date::from_calendar_date(year as i32, month, day).ok()
        } else {
            None
        }
    }
}

/// Generic paging structure used throughout LinkedIn API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Paging {
    pub count: u32,
    pub links: Vec<Value>,
    pub start: u32,
    pub total: u32,
}

/// Certification view
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificationView {
    pub elements: Vec<Certification>,
    pub entity_urn: String,
    pub paging: Paging,
    pub profile_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Certification {
    pub entity_urn: String,
    pub name: String,
    pub authority: Option<String>,
    pub license_number: Option<String>,
    pub time_period: Option<TimePeriod>,
    pub url: Option<Url>,
}

/// Course view
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseView {
    pub elements: Vec<Course>,
    pub entity_urn: String,
    pub paging: Paging,
    pub profile_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Course {
    pub entity_urn: String,
    pub name: String,
    pub number: Option<String>,
}

/// Honor/Awards view
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HonorView {
    pub elements: Vec<Honor>,
    pub entity_urn: String,
    pub paging: Paging,
    pub profile_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Honor {
    pub entity_urn: String,
    pub title: String,
    pub issuer: Option<String>,
    pub issue_date: Option<YearMonth>,
    pub description: Option<String>,
    pub occupation: Option<String>,
}

/// Language view
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageView {
    pub elements: Vec<Language>,
    pub entity_urn: String,
    pub paging: Paging,
    pub profile_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Language {
    pub entity_urn: String,
    pub name: String,
    pub proficiency: LanguageProficiency,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LanguageProficiency {
    #[serde(rename = "NATIVE_OR_BILINGUAL")]
    NativeOrBilingual,
    #[serde(rename = "FULL_PROFESSIONAL")]
    FullProfessional,
    #[serde(rename = "PROFESSIONAL_WORKING")]
    ProfessionalWorking,
    #[serde(rename = "LIMITED_WORKING")]
    LimitedWorking,
    #[serde(rename = "ELEMENTARY")]
    Elementary,
}

/// Enhanced Experience with all fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Experience {
    pub entity_urn: Option<String>,
    pub title: Option<String>,
    pub company_name: Option<String>,
    pub company_urn: Option<String>,
    pub company: Option<CompanyInfo>,
    pub description: Option<String>,
    pub time_period: Option<TimePeriod>,
    pub geo_location_name: Option<String>,
    pub geo_urn: Option<String>,
    pub location_name: Option<String>,
    pub region: Option<String>,
}

impl Experience {
    /// Get company logo URL
    pub fn get_company_logo_url(&self) -> Option<Url> {
        self.company
            .as_ref()
            .and_then(|company| company.mini_company.as_ref())
            .and_then(|mini| mini.logo.as_ref())
            .and_then(|container| container.vector_image.as_ref())
            .and_then(|vector_image| {
                if let (Some(root_url), Some(artifact)) =
                    (&vector_image.root_url, vector_image.artifacts.first())
                {
                    let full_url = format!(
                        "{}{}",
                        root_url,
                        artifact
                            .file_identifying_url_path_segment
                            .as_deref()
                            .unwrap_or("")
                    );
                    Url::parse(&full_url).ok()
                } else {
                    None
                }
            })
    }

    /// Check if this is current position (no end date)
    pub fn is_current(&self) -> bool {
        self.time_period
            .as_ref()
            .map(|tp| tp.end_date.is_none())
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyInfo {
    pub employee_count_range: Option<EmployeeCountRange>,
    pub industries: Vec<String>,
    pub mini_company: Option<MiniCompany>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmployeeCountRange {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniCompany {
    pub active: bool,
    pub dash_company_urn: Option<String>,
    pub entity_urn: String,
    pub logo: Option<VectorImageContainer>,
    pub name: String,
    pub object_urn: String,
    pub showcase: bool,
    pub tracking_id: String,
    pub universal_name: Option<String>,
}

/// Enhanced Education with all fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Education {
    pub entity_urn: String,
    pub school_name: Option<String>,
    pub school_urn: Option<String>,
    pub school: Option<SchoolInfo>,
    pub degree_name: Option<String>,
    pub degree_urn: Option<String>,
    pub field_of_study: Option<String>,
    pub field_of_study_urn: Option<String>,
    pub activities: Option<String>,
    pub description: Option<String>,
    pub grade: Option<String>,
    pub time_period: Option<TimePeriod>,
    pub honors: Vec<String>,
    pub test_scores: Vec<String>,
}

impl Education {
    /// Get school logo URL
    pub fn get_school_logo_url(&self) -> Option<Url> {
        self.school
            .as_ref()
            .and_then(|school| school.logo.as_ref())
            .and_then(|container| container.vector_image.as_ref())
            .and_then(|vector_image| {
                if let (Some(root_url), Some(artifact)) =
                    (&vector_image.root_url, vector_image.artifacts.first())
                {
                    let full_url = format!(
                        "{}{}",
                        root_url,
                        artifact
                            .file_identifying_url_path_segment
                            .as_deref()
                            .unwrap_or("")
                    );
                    Url::parse(&full_url).ok()
                } else {
                    None
                }
            })
    }

    /// Parse activities into a list
    pub fn get_activities_list(&self) -> Vec<String> {
        self.activities
            .as_ref()
            .map(|activities| {
                activities
                    .split('\n')
                    .filter_map(|activity| {
                        let trimmed = activity.trim().trim_start_matches("- ");
                        if trimmed.is_empty() {
                            None
                        } else {
                            Some(trimmed.to_string())
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolInfo {
    pub active: bool,
    pub entity_urn: String,
    pub logo: Option<VectorImageContainer>,
    pub object_urn: String,
    pub school_name: String,
    pub tracking_id: String,
}

/// Test Score view
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScoreView {
    pub elements: Vec<TestScore>,
    pub entity_urn: String,
    pub paging: Paging,
    pub profile_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScore {
    pub entity_urn: String,
    pub name: String,
    pub score: String,
    pub date: Option<YearMonth>,
    pub description: Option<String>,
    pub occupation: Option<String>,
}

/// Enhanced Contact Info with strong typing
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContactInfo {
    pub email_address: Option<EmailAddress>,
    pub websites: Vec<Website>,
    pub twitter: Vec<String>,
    pub phone_numbers: Vec<PhoneNumber>,
    pub birthdate: Option<BirthDate>,
    pub ims: Option<Vec<Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Website {
    pub url: Option<Url>,
    pub label: Option<String>,
}

/// Position Group View
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionGroupView {
    pub elements: Vec<PositionGroup>,
    pub entity_urn: String,
    pub paging: Paging,
    pub profile_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionGroup {
    pub entity_urn: String,
    pub name: String,
    pub mini_company: Option<MiniCompany>,
    pub paging: Paging,
    pub positions: Vec<Experience>,
    pub time_period: Option<TimePeriod>,
    pub region: Option<String>,
}

/// Position View (individual positions)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionView {
    pub elements: Vec<Experience>,
    pub entity_urn: String,
    pub paging: Paging,
    pub profile_id: String,
}

/// Enhanced Education View
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EducationView {
    pub elements: Vec<Education>,
    pub entity_urn: String,
    pub paging: Paging,
    pub profile_id: String,
}

/// Skill View
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillView {
    pub elements: Vec<Skill>,
    pub entity_urn: String,
    pub paging: Paging,
    pub profile_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Skill {
    pub entity_urn: String,
    pub name: String,
}

/// Volunteer Experience View
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolunteerExperienceView {
    pub elements: Vec<VolunteerExperience>,
    pub entity_urn: String,
    pub paging: Paging,
    pub profile_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolunteerExperience {
    pub entity_urn: String,
    pub role: String,
    pub company_name: Option<String>,
    pub company_urn: Option<String>,
    pub company: Option<CompanyInfo>,
    pub cause: Option<String>,
    pub description: Option<String>,
    pub time_period: Option<TimePeriod>,
    pub region: Option<String>,
}

/// Volunteer Cause View
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolunteerCauseView {
    pub elements: Vec<VolunteerCause>,
    pub entity_urn: String,
    pub paging: Paging,
    pub profile_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolunteerCause {
    pub cause_name: String,
    pub cause_type: String,
}

/// Generic view for empty sections
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationView {
    pub elements: Vec<Value>,
    pub entity_urn: String,
    pub paging: Paging,
    pub profile_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatentView {
    pub elements: Vec<Value>,
    pub entity_urn: String,
    pub paging: Paging,
    pub profile_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectView {
    pub elements: Vec<Value>,
    pub entity_urn: String,
    pub paging: Paging,
    pub profile_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicationView {
    pub elements: Vec<Value>,
    pub entity_urn: String,
    pub paging: Paging,
    pub profile_id: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub basic_location: Option<BasicLocation>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicLocation {
    pub country_code: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniProfile {
    pub dash_entity_urn: Option<String>,
    pub entity_urn: Option<String>,
    pub object_urn: Option<String>,
    pub public_identifier: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub occupation: Option<String>,
    pub tracking_id: Option<String>,
    pub picture: Option<VectorImageContainer>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilePicture {
    pub display_image: Option<String>,
    pub original_image: Option<String>,
    pub photo_filter_edit_info: Option<PhotoFilterEditInfo>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VectorImageContainer {
    pub vector_image: Option<VectorImage>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VectorImage {
    pub root_url: Option<String>,
    pub artifacts: Vec<ImageArtifact>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageArtifact {
    pub height: u32,
    pub width: u32,
    pub expires_at: Option<u64>,
    pub file_identifying_url_path_segment: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Clone, Deserialize)]
pub struct TimePeriod {
    pub start_date: YearMonth,
    pub end_date: Option<YearMonth>,
}

impl TimePeriod {
    /// Calculate duration in months (approximate)
    pub fn duration_months(&self) -> Option<u32> {
        let start = &self.start_date;
        let now = &YearMonth {
            year: time::OffsetDateTime::now_utc().year(),
            month: time::OffsetDateTime::now_utc().month(),
        };
        let end = self.end_date.as_ref().unwrap_or(now);

        let months =
            (end.year - start.year) * 12 + (end.month as u8 as i32 - start.month as u8 as i32);
        Some(months.max(1) as u32)
    }

    /// Format as human-readable duration
    pub fn duration_string(&self) -> String {
        match self.duration_months() {
            Some(months) if months < 12 => {
                if months == 1 {
                    "1 month".to_string()
                } else {
                    format!("{} months", months)
                }
            }
            Some(months) => {
                let years = months / 12;
                let remaining_months = months % 12;
                if remaining_months == 0 {
                    if years == 1 {
                        "1 year".to_string()
                    } else {
                        format!("{} years", years)
                    }
                } else {
                    if years == 1 {
                        format!("1 year {} months", remaining_months)
                    } else {
                        format!("{} years {} months", years, remaining_months)
                    }
                }
            }
            None => "Unknown duration".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct YearMonth {
    pub year: i32,
    pub month: Month,
}

impl Serialize for YearMonth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut s = serializer.serialize_struct("YearMonth", 2)?;
        s.serialize_field("year", &self.year)?;
        s.serialize_field("month", &(self.month as u8))?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for YearMonth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            year: i32,
            month: u8,
        }

        let raw = Raw::deserialize(deserializer)?;
        let month = Month::try_from(raw.month)
            .map_err(|_| serde::de::Error::custom("invalid month value"))?;

        Ok(YearMonth {
            year: raw.year,
            month,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub urn_id: String,
    pub public_id: String,
    pub distance: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberBadges {
    pub premium: bool,
    pub open_link: bool,
    pub influencer: bool,
    pub job_seeker: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonSearchResult {
    pub urn_id: String,
    pub public_id: String,
    pub distance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invitation {
    pub entity_urn: String,
    pub shared_secret: String,
}

pub struct UniformResourceName {
    pub namespace: String, // the context of the id
    pub id: String,
}
