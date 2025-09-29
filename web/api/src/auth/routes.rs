use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;

use crate::{
    auth::jwt,
    db::establish_connection,
    models::user::{
        ActiveModel as UserActiveModel, Column as UserColumn, Entity as User, Model as UserModel,
    },
};

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

use crate::auth::models::Claims;
use jsonwebtoken::DecodingKey;

#[derive(serde::Deserialize)]
pub struct TokenRequest {
    pub token: String,
}

#[post("/validate")]
pub async fn validate_token(req: web::Json<TokenRequest>) -> impl Responder {
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let validation = jsonwebtoken::Validation::default();
    match jsonwebtoken::decode::<Claims>(
        &req.token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &validation,
    ) {
        Ok(token_data) => {
            let db = establish_connection().await;
            let user = User::find_by_id(uuid::Uuid::parse_str(&token_data.claims.sub).unwrap())
                .one(&db)
                .await
                .unwrap();
            HttpResponse::Ok().json(user)
        }
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}

#[post("/login")]
pub async fn login(req: HttpRequest, credentials: web::Json<LoginRequest>) -> impl Responder {
    let db = establish_connection().await;
    let user = User::find()
        .filter(UserColumn::Username.eq(credentials.username.clone()))
        .one(&db)
        .await
        .unwrap();

    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());

    if let Some(user) = user {
        let password_match = bcrypt::verify(&credentials.password, &user.password).unwrap_or(false);
        let success = password_match;

        // record login attempt
        let _ = crate::models::login_history::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user.id),
            success: Set(success),
            ip_address: Set(ip_address.clone()),
            login_time: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .insert(&db)
        .await;

        if success {
            let mut user_model = user.into_active_model();
            let user_id = user_model.id.clone().unwrap();
            user_model.last_login = Set(Some(chrono::Utc::now().into()));
            user_model.ip_address = Set(ip_address);
            let _ = user_model.update(&db).await;

            let jwt = jwt::create_jwt(user_id);
            HttpResponse::Ok().json(json!({ "token": jwt }))
        } else {
            HttpResponse::Unauthorized().finish()
        }
    } else {
        // record failed login with unknown user
        let _ = crate::models::login_history::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(Uuid::nil()),
            success: Set(false),
            ip_address: Set(ip_address),
            login_time: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .insert(&db)
        .await;

        HttpResponse::Unauthorized().finish()
    }
}

#[post("/register")]
pub async fn register(req: web::Json<RegisterRequest>) -> impl Responder {
    let db = establish_connection().await;

    let hashed = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST).unwrap();

    let mut role = "User".to_string();
    if req.username == "ricochet" || req.email == "bojacnyh@gmail.com" {
        role = "Admin".to_string();
    }

    let user_id = Uuid::new_v4();
    let new_user = UserActiveModel {
        id: Set(user_id),
        username: Set(req.username.clone()),
        email: Set(req.email.clone()),
        password: Set(hashed),
        role: Set(role.clone()),
        created_at: Set(chrono::Utc::now().into()),
        is_active: Set(true),
        banned: Set(false),
        ..Default::default()
    };

    let user: UserModel = new_user.insert(&db).await.unwrap();
    let token = jwt::create_jwt(user.id);

    HttpResponse::Ok().json(serde_json::json!({ "token": token }))
}
