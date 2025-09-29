use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "activation_keys")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub key: String,
    pub product_id: Uuid,
    pub duration_days: i32,
    pub is_redeemed: bool,
    pub created_at: ChronoDateTimeWithTimeZone,
    pub generated_by: Option<Uuid>,
    pub redeemed_by: Option<Uuid>,
    pub is_free: bool,
    pub price_paid: f64,
    pub order_id: Option<Uuid>,
    pub replaced_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::product::Entity",
        from = "Column::ProductId",
        to = "super::product::Column::Id"
    )]
    Product,
}

impl Related<super::product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
