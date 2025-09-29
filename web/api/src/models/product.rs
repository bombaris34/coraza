use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "products")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub identifier: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub size: String,
    pub color: String,
    pub image_url: Option<String>,
    pub category: Option<String>,
    pub in_stock: bool,
    pub is_discounted: bool,
    pub discounted_price: Option<f64>,
    pub frozen: bool,
    pub created_at: ChronoDateTimeWithTimeZone,
    pub updated_at: ChronoDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::activation_key::Entity")]
    ActivationKey,
}

impl Related<super::activation_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ActivationKey.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
