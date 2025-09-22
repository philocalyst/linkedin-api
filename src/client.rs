use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Arc;

use reqwest::cookie::{CookieStore, Jar};
use reqwest::{header, Client as ReqwestClient, Response, Url};
use serde_json::Value;

use crate::error::LinkedinError;
use crate::utils::evade;
use crate::Identity;

const API_BASE_URL: &str = "https://www.linkedin.com/voyager/api";
const AUTH_BASE_URL: &str = "https://www.linkedin.com";
const COOKIE_FILE_PATH: &str = ".cookies.json";

#[derive(Clone)]
pub struct Client {
    pub(crate) client: ReqwestClient,
    cookie_jar: Arc<Jar>,
}

impl Client {
    pub fn new() -> Result<Self, LinkedinError> {
        let jar = Arc::new(Jar::default());
        
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "user-agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/66.0.3359.181 Safari/537.36"
                .parse()?,
        );
        headers.insert("accept-language", "en-AU,en-GB;q=0.9,en-US;q=0.8,en;q=0.7".parse()?);
        headers.insert("x-li-lang", "en_US".parse()?);
        headers.insert("x-restli-protocol-version", "2.0.0".parse()?);
        
        let client = ReqwestClient::builder()
            .cookie_provider(jar.clone())
            .default_headers(headers)
            .build()?;
            
        Ok(Self { client, cookie_jar: jar })
    }

    pub async fn authenticate(&self, identity: &Identity, refresh: bool) -> Result<(), LinkedinError> {
        let url = Url::parse("https://www.linkedin.com")?;
        if !refresh {
            if self.load_cookies().is_ok() {
                return Ok(());
            }
        }

        // Request session cookies
        self.request_session_cookies().await?;

        self.cookie_jar.add_cookie_str(&format!("li_at={}; Domain=.linkedin.com; Path=/; Secure; HttpOnly", identity.authentication_token), &url);
        self.cookie_jar.add_cookie_str(&format!("JSESSIONID={}; Domain=.linkedin.com; Path=/; Secure; HttpOnly", identity.session_cookie), &url);

        let mut form = std::collections::HashMap::new();
        form.insert("session_key", &identity.username);
        form.insert("session_password", &identity.password);
        
        let jsession_id = self.get_jsession_id();
        form.insert("JSESSIONID", &jsession_id);

        let mut headers = header::HeaderMap::new();
        headers.insert("X-Li-User-Agent", "LIAuthLibrary:3.2.4 com.linkedin.LinkedIn:8.8.1 iPhone:8.3".parse()?);
        headers.insert("User-Agent", "LinkedIn/8.8.1 CFNetwork/711.3.18 Darwin/14.0.0".parse()?);
        headers.insert("X-User-Language", "en".parse()?);
        headers.insert("X-User-Locale", "en_US".parse()?);
        headers.insert("Accept-Language", "en-us".parse()?);

        let res = self.client.post(&format!("{}/uas/authenticate", AUTH_BASE_URL))
            .headers(headers)
            .form(&form)
            .send()
            .await?;

        dbg!(&res);
        if res.status() == 401 {
            return Err(LinkedinError::Unauthorized("Authentication failed".to_string()));
        }

        if res.status() != 200 {
            return Err(LinkedinError::RequestFailed(format!("Authentication request failed with status: {}", res.status())));
        }

        let data: Value = res.json().await?;

        if let Some(login_result) = data.get("login_result") {
            if login_result != "PASS" {
                return Err(LinkedinError::Challenge(login_result.as_str().unwrap_or("Unknown").to_string()));
            }
        }

        self.save_cookies()?;
        self.set_csrf_token();

        Ok(())
    }

    async fn request_session_cookies(&self) -> Result<(), LinkedinError> {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-Li-User-Agent", "LIAuthLibrary:3.2.4 com.linkedin.LinkedIn:8.8.1 iPhone:8.3".parse()?);
        headers.insert("User-Agent", "LinkedIn/8.8.1 CFNetwork/711.3.18 Darwin/14.0.0".parse()?);
        headers.insert("X-User-Language", "en".parse()?);
        headers.insert("X-User-Locale", "en_US".parse()?);
        headers.insert("Accept-Language", "en-us".parse()?);

        let _res = self.client.get(&format!("{}/uas/authenticate", AUTH_BASE_URL))
            .headers(headers)
            .send()
            .await?;

        Ok(())
    }

    fn get_jsession_id(&self) -> String {
        let url = Url::parse(AUTH_BASE_URL).unwrap();
        if let Some(cookies) = self.cookie_jar.cookies(&url) {
            for cookie in cookies.to_str().unwrap_or("").split(';') {
                let cookie = cookie.trim();
                if cookie.starts_with("JSESSIONID=") {
                    return cookie.replace("JSESSIONID=", "").trim_matches('"').to_string();
                }
            }
        }
        String::new()
    }

    fn set_csrf_token(&self) {
        // The csrf-token header is set to the JSESSIONID value
        // This would need to be handled per request in a real implementation
    }

    fn load_cookies(&self) -> Result<(), LinkedinError> {
        let path = Path::new(COOKIE_FILE_PATH);
        if !path.exists() {
            return Err(LinkedinError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Cookie file not found")));
        }
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let cookies: Vec<String> = serde_json::from_reader(reader)?;
        
        let url = Url::parse(AUTH_BASE_URL)?;
        for cookie in cookies {
            self.cookie_jar.add_cookie_str(&cookie, &url);
        }
        
        Ok(())
    }

    fn save_cookies(&self) -> Result<(), LinkedinError> {
        let url = Url::parse(AUTH_BASE_URL)?;
        let cookies: Vec<String> = if let Some(cookie_header) = self.cookie_jar.cookies(&url) {
            cookie_header.to_str()?.split(';').map(|s| s.trim().to_string()).collect()
        } else {
            vec![]
        };
        
        let file = OpenOptions::new().write(true).create(true).truncate(true).open(COOKIE_FILE_PATH)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &cookies)?;
        Ok(())
    }

    pub async fn get(&self, uri: &str) -> Result<Response, LinkedinError> {
        evade().await;
        let url = format!("{}{}", API_BASE_URL, uri);
        
        let mut headers = header::HeaderMap::new();
        headers.insert("csrf-token", self.get_jsession_id().parse()?);
        
        let res = self.client.get(&url).headers(headers).send().await?;
        Ok(res)
    }

    pub async fn post(&self, uri: &str, data: &Value) -> Result<Response, LinkedinError> {
        evade().await;
        let url = format!("{}{}", API_BASE_URL, uri);
        
        let mut headers = header::HeaderMap::new();
        headers.insert("csrf-token", self.get_jsession_id().parse()?);
        headers.insert("content-type", "application/json".parse()?);
        
        let res = self.client.post(&url).headers(headers).json(data).send().await?;
        Ok(res)
    }
}
