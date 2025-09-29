use crate::auth::models::Claims;
use crate::{
    action_logs::log_action,
    auth::jwt,
    db::establish_connection,
    models::{
        activation_key, login_history,
        product::{Entity as Product, Model as ProductModel},
        user,
    },
};
use actix_web::{post, web, HttpResponse, Responder};
use bcrypt;
use jsonwebtoken::DecodingKey;
use log::info;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoaderLoginRequest {
    pub username: String,
    pub password: String,
    pub ip_address: String,
}

#[derive(Deserialize)]
pub struct LoaderRegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub ip_address: String,
}

#[derive(Serialize)]
pub struct LoaderLoginResponse {
    pub message: String,
    pub token: String,
    pub products: Vec<ProductModel>,
}

#[derive(Deserialize)]
pub struct ActivationRequest {
    pub username: String,
    pub key: String,
}

#[derive(Serialize)]
pub struct ActivationResponse {
    pub message: String,
    pub expiry: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize)]
pub struct BanUserRequest {
    pub token: Option<String>,
    pub username: Option<String>,
    pub ip_address: String,
    pub reason: String,
}

#[derive(Serialize)]
pub struct BanUserResponse {
    pub message: String,
    pub banned_user_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[post("/loader_login")]
pub async fn loader_login(req: web::Json<LoaderLoginRequest>) -> impl Responder {
    let db = establish_connection().await;

    let user = user::Entity::find()
        .filter(user::Column::Username.eq(req.username.clone()))
        .one(&db)
        .await
        .unwrap();

    if let Some(user) = user {
        let password_match = bcrypt::verify(&req.password, &user.password).unwrap_or(false);

        if !password_match {
            let _ = login_history::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(user.id),
                success: Set(false),
                ip_address: Set(Some(req.ip_address.clone())),
                login_time: Set(chrono::Utc::now().into()),
                ..Default::default()
            }
            .insert(&db)
            .await;
            log_action(
                None,
                serde_json::json!({"action":"internal_login","username": req.username, "ip": req.ip_address, "result": "wrong_password"}),
            ).await;
            return HttpResponse::Unauthorized().json(ErrorResponse {
                message: "wrong_password".to_string(),
            });
        }

        if user.banned {
            let _ = login_history::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(user.id),
                success: Set(false),
                ip_address: Set(Some(req.ip_address.clone())),
                login_time: Set(chrono::Utc::now().into()),
                ..Default::default()
            }
            .insert(&db)
            .await;
            log_action(
                None,
                serde_json::json!({"action":"internal_login","username": req.username, "ip": req.ip_address, "result": "banned"}),
            ).await;
            return HttpResponse::Forbidden().json(ErrorResponse {
                message: "banned".to_string(),
            });
        }

        let user_id = user.id;
        let mut user_model: user::ActiveModel = user.into();
        user_model.ip_address = Set(Some(req.ip_address.clone()));
        user_model.last_login = Set(Some(chrono::Utc::now().into()));

        let _ = user_model.update(&db).await;

        let _ = login_history::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            success: Set(true),
            ip_address: Set(Some(req.ip_address.clone())),
            login_time: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .insert(&db)
        .await;

        let products = Product::find().all(&db).await.unwrap();

        let token = jwt::create_jwt(user_id);
        let response = LoaderLoginResponse {
            message: "success".to_string(),
            token,
            products,
        };
        log_action(
            None,
            serde_json::json!({"action":"internal_login","username": req.username, "ip": req.ip_address, "result": "success"}),
        ).await;
        HttpResponse::Ok().json(response)
    } else {
        let _ = login_history::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(Uuid::nil()),
            success: Set(false),
            ip_address: Set(Some(req.ip_address.clone())),
            login_time: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .insert(&db)
        .await;
        info!(
            "loader_login username={} ip={} result=not_found",
            req.username, req.ip_address
        );
        HttpResponse::Unauthorized().json(ErrorResponse {
            message: "not_found".to_string(),
        })
    }
}

#[post("/loader_register")]
pub async fn loader_register(req: web::Json<LoaderRegisterRequest>) -> impl Responder {
    let db = establish_connection().await;

    let existing = user::Entity::find()
        .filter(
            Condition::any()
                .add(user::Column::Username.eq(req.username.clone()))
                .add(user::Column::Email.eq(req.email.clone())),
        )
        .one(&db)
        .await
        .unwrap();

    if existing.is_some() {
        log_action(
            None,
            serde_json::json!({"action":"internal_register","username": req.username, "ip": req.ip_address, "result": "already_registered"}),
        ).await;
        return HttpResponse::BadRequest().json(ErrorResponse {
            message: "already_registered".to_string(),
        });
    }

    if req.password.trim().is_empty() || req.password.len() < 5 {
        log_action(
            None,
            serde_json::json!({"action":"internal_register","username": req.username, "ip": req.ip_address, "result": "password_verification"}),
        ).await;
        return HttpResponse::BadRequest().json(ErrorResponse {
            message: "password_verification".to_string(),
        });
    }

    let hashed = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST).unwrap();

    let new_user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(req.username.clone()),
        email: Set(req.email.clone()),
        password: Set(hashed),
        role: Set("User".to_string()),
        created_at: Set(chrono::Utc::now().into()),
        is_active: Set(true),
        ip_address: Set(Some(req.ip_address.clone())),
        ..Default::default()
    };

    match new_user.insert(&db).await {
        Ok(_) => {
            log_action(
                None,
                serde_json::json!({"action":"internal_register","username": req.username, "ip": req.ip_address, "result": "success"}),
            ).await;
            HttpResponse::Ok().body("success")
        }
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            message: "internal_error".to_string(),
        }),
    }
}

#[post("/activate_key")]
pub async fn activate_key(req: web::Json<ActivationRequest>) -> impl Responder {
    let db = establish_connection().await;

    let user = match user::Entity::find()
        .filter(user::Column::Username.eq(req.username.clone()))
        .one(&db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            log_action(
                None,
                serde_json::json!({"action":"activate_key","username": req.username, "result":"user_not_found"}),
            ).await;
            return HttpResponse::NotFound().json(ActivationResponse {
                message: "user_not_found".to_string(),
                expiry: None,
            });
        }
        Err(_) => {
            log_action(
                None,
                serde_json::json!({"action":"activate_key","username": req.username, "result":"database_error"}),
            ).await;
            return HttpResponse::InternalServerError().json(ActivationResponse {
                message: "internal_error".to_string(),
                expiry: None,
            });
        }
    };

    let user_id = user.id;

    let activation_key = match activation_key::Entity::find()
        .filter(activation_key::Column::Key.eq(req.key.clone()))
        .filter(activation_key::Column::IsRedeemed.eq(false))
        .one(&db)
        .await
    {
        Ok(Some(key)) => key,
        Ok(None) => {
            log_action(
                None,
                serde_json::json!({"action":"activate_key","user_id": user_id, "key": req.key, "result":"key_not_found_or_redeemed"}),
            ).await;
            return HttpResponse::BadRequest().json(ActivationResponse {
                message: "invalid_key".to_string(),
                expiry: None,
            });
        }
        Err(_) => {
            log_action(
                None,
                serde_json::json!({"action":"activate_key","user_id": user_id, "result":"database_error"}),
            ).await;
            return HttpResponse::InternalServerError().json(ActivationResponse {
                message: "internal_error".to_string(),
                expiry: None,
            });
        }
    };

    let mut key_model: activation_key::ActiveModel = activation_key.clone().into();
    key_model.is_redeemed = Set(true);
    key_model.redeemed_by = Set(Some(user_id));

    if let Err(_) = key_model.update(&db).await {
        log_action(
            None,
            serde_json::json!({"action":"activate_key","user_id": user_id, "key": req.key, "result":"failed_to_update_key"}),
        ).await;
        return HttpResponse::InternalServerError().json(ActivationResponse {
            message: "internal_error".to_string(),
            expiry: None,
        });
    }

    log_action(
        None,
        serde_json::json!({"action":"activate_key","user_id": user_id, "key": req.key, "result":"success"}),
    ).await;
    HttpResponse::Ok().json(ActivationResponse {
        message: "success".to_string(),
        expiry: Some(chrono::Utc::now() + chrono::Duration::days(activation_key.duration_days as i64)),
    })
}

#[post("/ban_user")]
pub async fn ban_user(req: web::Json<BanUserRequest>) -> impl Responder {
    let db = establish_connection().await;

    let target_user_id = if let Some(token) = &req.token {
        let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let validation = jsonwebtoken::Validation::default();
        match jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret_key.as_ref()),
            &validation,
        ) {
            Ok(token_data) => match Uuid::parse_str(&token_data.claims.sub) {
                Ok(id) => Some(id),
                Err(_) => {
                    log_action(
                            None,
                            serde_json::json!({"action":"ban_user","result":"invalid_token_format","ip": req.ip_address}),
                        ).await;
                    return HttpResponse::BadRequest().json(BanUserResponse {
                        message: "invalid_token_format".to_string(),
                        banned_user_id: None,
                    });
                }
            },
            Err(_) => {
                log_action(
                    None,
                    serde_json::json!({"action":"ban_user","result":"invalid_token","ip": req.ip_address}),
                ).await;
                return HttpResponse::BadRequest().json(BanUserResponse {
                    message: "invalid_token".to_string(),
                    banned_user_id: None,
                });
            }
        }
    } else if let Some(username) = &req.username {
        match user::Entity::find()
            .filter(user::Column::Username.eq(username.clone()))
            .one(&db)
            .await
        {
            Ok(Some(user)) => Some(user.id),
            Ok(None) => {
                log_action(
                    None,
                    serde_json::json!({"action":"ban_user","username": username, "result":"user_not_found","ip": req.ip_address}),
                ).await;
                return HttpResponse::NotFound().json(BanUserResponse {
                    message: "user_not_found".to_string(),
                    banned_user_id: None,
                });
            }
            Err(_) => {
                log_action(
                    None,
                    serde_json::json!({"action":"ban_user","username": username, "result":"database_error","ip": req.ip_address}),
                ).await;
                return HttpResponse::InternalServerError().json(BanUserResponse {
                    message: "database_error".to_string(),
                    banned_user_id: None,
                });
            }
        }
    } else {
        log_action(
            None,
            serde_json::json!({"action":"ban_user","result":"missing_auth","ip": req.ip_address}),
        )
        .await;
        return HttpResponse::BadRequest().json(BanUserResponse {
            message: "missing_token_or_username".to_string(),
            banned_user_id: None,
        });
    };

    let user_id = match target_user_id {
        Some(id) => id,
        None => {
            return HttpResponse::InternalServerError().json(BanUserResponse {
                message: "internal_error".to_string(),
                banned_user_id: None,
            });
        }
    };

    let target_user = match user::Entity::find_by_id(user_id).one(&db).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            log_action(
                None,
                serde_json::json!({"action":"ban_user","user_id": user_id, "result":"user_not_found_by_id","ip": req.ip_address}),
            ).await;
            return HttpResponse::NotFound().json(BanUserResponse {
                message: "user_not_found".to_string(),
                banned_user_id: None,
            });
        }
        Err(_) => {
            log_action(
                None,
                serde_json::json!({"action":"ban_user","user_id": user_id, "result":"database_error","ip": req.ip_address}),
            ).await;
            return HttpResponse::InternalServerError().json(BanUserResponse {
                message: "database_error".to_string(),
                banned_user_id: None,
            });
        }
    };

    if target_user.banned {
        log_action(
            None,
            serde_json::json!({"action":"ban_user","user_id": user_id, "username": target_user.username, "result":"already_banned","ip": req.ip_address}),
        ).await;
        return HttpResponse::Conflict().json(BanUserResponse {
            message: "user_already_banned".to_string(),
            banned_user_id: Some(user_id),
        });
    }

    let mut user_model: user::ActiveModel = target_user.clone().into();
    user_model.banned = Set(true);
    user_model.ban_reason = Set(Some(req.reason.clone()));

    match user_model.update(&db).await {
        Ok(_) => {
            log_action(
                None,
                serde_json::json!({"action":"ban_user","user_id": user_id, "username": target_user.username, "reason": req.reason, "result":"success","ip": req.ip_address}),
            ).await;
            HttpResponse::Ok().json(BanUserResponse {
                message: "user_banned_successfully".to_string(),
                banned_user_id: Some(user_id),
            })
        }
        Err(_) => {
            log_action(
                None,
                serde_json::json!({"action":"ban_user","user_id": user_id, "username": target_user.username, "result":"failed_to_update","ip": req.ip_address}),
            ).await;
            HttpResponse::InternalServerError().json(BanUserResponse {
                message: "failed_to_ban_user".to_string(),
                banned_user_id: None,
            })
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(loader_login)
        .service(loader_register)
        .service(activate_key)
        .service(ban_user);
}
