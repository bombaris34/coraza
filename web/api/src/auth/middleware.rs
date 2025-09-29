use crate::{
    auth::{authorized::Authorized, models::Claims},
    db::establish_connection,
    models::user::Entity as User,
};
use actix_web::{
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sea_orm::EntityTrait;
use std::rc::Rc;
use std::task::{Context, Poll};
use uuid::Uuid;

pub struct Authentication;

impl<S> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let auth_header = req.headers().get("Authorization");

        if let Some(auth_header) = auth_header {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
                    if let Ok(token_data) = decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(secret_key.as_ref()),
                        &Validation::default(),
                    ) {
                        let user_id = token_data.claims.sub.clone();

                        return Box::pin(async move {
                            let db = establish_connection().await;
                            let user = User::find_by_id(Uuid::parse_str(&user_id).unwrap())
                                .one(&db)
                                .await
                                .unwrap();

                            if let Some(user) = user {
                                // Insert the authorized user data into request extensions
                                req.extensions_mut().insert(Authorized {
                                    id: user.id,
                                    username: user.username,
                                    email: user.email,
                                    role: user.role,
                                });

                                // Now call the service with the modified request
                                service.call(req).await
                            } else {
                                let res = req.into_response(HttpResponse::Unauthorized().finish());
                                Ok(res)
                            }
                        });
                    }
                }
            }
        }

        Box::pin(async move {
            let res = req.into_response(HttpResponse::Unauthorized().finish());
            Ok(res)
        })
    }
}
