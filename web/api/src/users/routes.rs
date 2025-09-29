use crate::{
    auth::{Authorized, AdminAuthorized},
    db::establish_connection,
    models::user::{ActiveModel as UserActiveModel, Entity as User, Model as UserModel, UserUpdate},
    action_logs,
};
use actix_web::{get, post, delete, put, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use bcrypt;

#[get("/me")]
pub async fn get_current_user(user: Authorized) -> impl Responder {
    let db = establish_connection().await;
    let current_user = User::find_by_id(user.id).one(&db).await.unwrap();
    HttpResponse::Ok().json(current_user)
}

#[get("/")]
async fn list_users(_admin: AdminAuthorized) -> impl Responder {
    let db = establish_connection().await;
    let users = User::find().all(&db).await.unwrap();
    HttpResponse::Ok().json(users)
}

#[post("/")]
async fn create_user(_admin: AdminAuthorized, user: web::Json<UserModel>) -> impl Responder {
    let db = establish_connection().await;
    let mut new_user: UserActiveModel = user.into_inner().into_active_model();
    if new_user.password.is_set() {
        if let Some(pass) = new_user.password.take() {
            new_user.password = Set(bcrypt::hash(pass, bcrypt::DEFAULT_COST).unwrap());
        }
    } else {
        new_user.password = Set(bcrypt::hash("password", bcrypt::DEFAULT_COST).unwrap());
    }
    let user = new_user.insert(&db).await.unwrap();
    HttpResponse::Ok().json(user)
}

#[delete("/{id}")]
async fn delete_user(_admin: AdminAuthorized, id: web::Path<uuid::Uuid>) -> impl Responder {
    let db = establish_connection().await;
    match User::delete_by_id(id.into_inner()).exec(&db).await {
        Ok(res) if res.rows_affected == 1 => HttpResponse::Ok().finish(),
        _ => HttpResponse::NotFound().finish(),
    }
}

#[put("/{id}")]
pub async fn update_user(
    _admin: AdminAuthorized,
    id: web::Path<uuid::Uuid>,
    user_data: web::Json<UserUpdate>,
) -> impl Responder {
    let db_conn = establish_connection().await;
    let user = User::find_by_id(id.into_inner())
        .one(&db_conn)
        .await
        .unwrap();
    if let Some(user) = user {
        let mut user = user.into_active_model();
        if let Some(username) = user_data.username.clone() {
            user.username = Set(username);
        }
        if let Some(email) = user_data.email.clone() {
            user.email = Set(email);
        }
        if let Some(role) = user_data.role.clone() {
            user.role = Set(role);
        }
        if let Some(is_active) = user_data.is_active {
            user.is_active = Set(is_active);
        }
        if let Some(banned) = user_data.banned {
            user.banned = Set(banned);
        }
        if let Some(reason) = user_data.ban_reason.clone() {
            user.ban_reason = Set(Some(reason));
        }
        if let Some(pass) = user_data.password.clone() {
            let hashed = bcrypt::hash(pass, bcrypt::DEFAULT_COST).unwrap();
            user.password = Set(hashed);
        }
        let user = user.update(&db_conn).await.unwrap();
        action_logs::log_action(Some(_admin.id), serde_json::json!({"action":"update_user","id": user.id})).await;
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().finish()
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_current_user)
        .service(list_users)
        .service(create_user)
        .service(update_user)
        .service(delete_user);
}
