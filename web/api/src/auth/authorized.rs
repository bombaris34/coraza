use actix_web::{dev::Payload, FromRequest, HttpRequest, HttpMessage};
use futures::future::{err, ok, Ready};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Authorized {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
}

impl FromRequest for Authorized {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let extensions = req.extensions();
        if let Some(user) = extensions.get::<Authorized>() {
            ok(user.clone())
        } else {
            err(actix_web::error::ErrorUnauthorized("Unauthorized"))
        }
    }
}
