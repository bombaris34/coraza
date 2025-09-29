use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::LocalBoxFuture;
use log::info;
use std::env;

pub struct InternalAuthentication;

impl<S, B> Transform<S, ServiceRequest> for InternalAuthentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = InternalAuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(InternalAuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct InternalAuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for InternalAuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

        let auth_header = req.headers().get("X-Internal-Secret").cloned();

        let is_authorized = match auth_header {
            Some(key) => {
                let key_str = key.to_str().unwrap_or("");
                key_str == secret_key.trim()
            }
            None => false,
        };

        if is_authorized {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            info!("Authorization failed.");
            Box::pin(async move { Err(actix_web::error::ErrorUnauthorized("Unauthorized")) })
        }
    }
}
