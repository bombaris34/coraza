use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: ChronoDateTimeWithTimeZone,
    pub last_login: Option<ChronoDateTimeWithTimeZone>,
    pub is_active: bool,
    pub ip_address: Option<String>,
    pub banned: bool,
    pub ban_reason: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct UserUpdate {
    pub username: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
    pub banned: Option<bool>,
    pub ban_reason: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UserRegistration {
    pub username: String,
    pub email: String,
    pub password: String,
}
