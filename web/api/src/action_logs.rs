use crate::db::establish_connection;
use crate::models::action_log::{ActiveModel as ActionLogActiveModel};
use sea_orm::{ActiveModelTrait, Set};
use serde_json::Value;
use uuid::Uuid;

pub async fn log_action(user_id: Option<Uuid>, action: Value) {
    let db = establish_connection().await;
    let log = ActionLogActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        action_data: Set(action),
        created_at: Set(chrono::Utc::now().into()),
    };
    let _ = log.insert(&db).await;
}
