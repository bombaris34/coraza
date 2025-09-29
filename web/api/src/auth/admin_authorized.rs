use actix_web::{dev::Payload, FromRequest, HttpRequest, HttpMessage};
use futures::future::{err, ok, Ready};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminAuthorized {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
}

impl FromRequest for AdminAuthorized {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let extensions = req.extensions();
        if let Some(user) = extensions.get::<crate::auth::authorized::Authorized>() {
            if user.role == "Admin" {
                ok(AdminAuthorized {
                    id: user.id,
                    username: user.username.clone(),
                    email: user.email.clone(),
                    role: user.role.clone(),
                })
            } else {
                err(actix_web::error::ErrorForbidden("Requires admin role"))
            }
        } else {
            err(actix_web::error::ErrorUnauthorized("Unauthorized"))
        }
    }
}
