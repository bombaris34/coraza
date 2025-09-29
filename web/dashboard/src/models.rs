use chrono::{DateTime, Utc};
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use uuid::Uuid;

fn deserialize_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum F64OrString {
        F64(f64),
        String(String),
        Int(i64),
    }

    match F64OrString::deserialize(deserializer)? {
        F64OrString::F64(f) => Ok(f),
        F64OrString::Int(i) => Ok(i as f64),
        F64OrString::String(s) => s.parse::<f64>().map_err(de::Error::custom),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Admin,
    User,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserRole::Admin => write!(f, "Admin"),
            UserRole::User => write!(f, "User"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: bool,
    #[serde(default)]
    pub hardware_id_hash: Option<String>,
    #[serde(default)]
    pub hardware_info: Option<Value>,
    #[serde(default)]
    pub last_hardware_hash: Option<String>,
    #[serde(default)]
    pub last_hardware_info: Option<Value>,
    #[serde(default)]
    pub ip_address: Option<String>,
    #[serde(default)]
    pub ban_reason: Option<String>,
    #[serde(rename = "banned")]
    pub is_banned: bool,
}

impl User {
    #[allow(dead_code)]
    pub fn new(username: String, email: String, role: UserRole) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            role,
            created_at: Utc::now(),
            last_login: None,
            is_active: true,
            hardware_id_hash: None,
            hardware_info: None,
            last_hardware_hash: None,
            last_hardware_info: None,
            ip_address: None,
            ban_reason: None,
            is_banned: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegistration {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<UserRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DowntimeLog {
    pub id: Uuid,
    pub server_name: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub image_url: Option<String>,
    pub category: Option<String>,
    pub in_stock: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegistrationStats {
    pub date: String,
    pub count: i64,
}


pub mod product;

#[derive(Deserialize)]
pub struct TokenResponse {
    pub token: String,
}
