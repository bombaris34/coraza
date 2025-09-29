use actix_web::{delete, get, post, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set, IntoActiveModel};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::AdminAuthorized,
    db::establish_connection,
    action_logs::log_action,
    models::activation_key::{
        ActiveModel as ActivationKeyActiveModel, Entity as ActivationKey,
    },
};

#[derive(Deserialize)]
pub struct ActivationKeyRequest {
    product_id: Uuid,
    #[serde(default)]
    is_free: bool,
    #[serde(default = "default_duration")] 
    duration_days: i32,
}

fn default_duration() -> i32 { 30 }

#[post("/activation_keys")]
async fn create_activation_key(
    admin: AdminAuthorized,
    req: web::Json<ActivationKeyRequest>,
) -> impl Responder {
    let db = establish_connection().await;
    let key = Uuid::new_v4().to_string();

    let new_key = ActivationKeyActiveModel {
        id: Set(Uuid::new_v4()),
        key: Set(key),
        product_id: Set(req.product_id),
        duration_days: Set(req.duration_days),
        is_redeemed: Set(false),
        created_at: Set(chrono::Utc::now().into()),
        generated_by: Set(Some(admin.id)),
        redeemed_by: Set(None),
        is_free: Set(req.is_free),
        price_paid: Set(0.0),
        order_id: Set(None),
        replaced_by: Set(None),
    };

    let key = new_key.insert(&db).await.unwrap();
    log_action(Some(admin.id), serde_json::json!({"action":"create_key","key_id": key.id})).await;
    HttpResponse::Ok().json(key)
}

#[get("/activation_keys")]
async fn get_activation_keys(_admin: AdminAuthorized) -> impl Responder {
    let db = establish_connection().await;
    let keys = ActivationKey::find().all(&db).await.unwrap();
    HttpResponse::Ok().json(keys)
}

#[delete("/activation_keys/{id}")]
async fn delete_activation_key(
    admin: AdminAuthorized,
    id: web::Path<Uuid>,
) -> impl Responder {
    let db = establish_connection().await;
    let key_id = id.into_inner();
    if let Some(key) = ActivationKey::find_by_id(key_id).one(&db).await.unwrap() {
        let _ = key.delete(&db).await;
        log_action(Some(admin.id), serde_json::json!({"action":"delete_key","id": key_id})).await;
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[post("/activation_keys/{id}/replace")]
async fn replace_activation_key(
    admin: AdminAuthorized,
    id: web::Path<Uuid>,
) -> impl Responder {
    let db = establish_connection().await;
    let key_id = id.into_inner();
    if let Some(existing) = ActivationKey::find_by_id(key_id)
        .one(&db)
        .await
        .unwrap()
    {
        let product_id = existing.product_id;
        let is_free = existing.is_free;
        let price_paid = existing.price_paid;
        let duration = existing.duration_days;
        let mut old_model = existing.clone().into_active_model();
        let new_key = ActivationKeyActiveModel {
            id: Set(Uuid::new_v4()),
            key: Set(Uuid::new_v4().to_string()),
            product_id: Set(product_id),
            duration_days: Set(duration),
            is_redeemed: Set(false),
            created_at: Set(chrono::Utc::now().into()),
            generated_by: Set(Some(admin.id)),
            redeemed_by: Set(None),
            is_free: Set(is_free),
            price_paid: Set(price_paid),
            order_id: Set(existing.order_id),
            replaced_by: Set(None),
        };
        let key = new_key.insert(&db).await.unwrap();
        old_model.replaced_by = Set(Some(key.id));
        old_model.is_redeemed = Set(true);
        let _ = old_model.update(&db).await;
        log_action(Some(admin.id), serde_json::json!({"action":"replace_key","old": key_id,"new": key.id})).await;
        HttpResponse::Ok().json(key)
    } else {
        HttpResponse::NotFound().finish()
    }
}
