use std::future::{ready, Ready};

use actix_identity::IdentityExt;
use actix_web::{body::EitherBody, dev::{self, Service, ServiceRequest, ServiceResponse, Transform}, Error, http, HttpResponse};
use futures_util::future::LocalBoxFuture;
use futures_util::FutureExt;

static LOGIN: &str = "/login";
static REGISTER: &str = "/register";

pub struct CheckLogin;

impl<S, B> Transform<S, ServiceRequest> for CheckLogin
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = CheckLoginMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckLoginMiddleware { service }))
    }
}

pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckLoginMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();
        let is_logged_in = is_logged_in(&req);
        if !is_logged_in {
            return match path {
                path if is_secured(&path) => {
                    Box::pin(async move {
                        let response = HttpResponse::Unauthorized()
                            .finish()
                            .map_into_right_body();
                        let (http_request, _) = req.into_parts();
                        return Ok(ServiceResponse::new(http_request, response));
                    })
                }
                path if is_unsecured(&path) => {
                    let res = self.service.call(req);
                    Box::pin(async move {
                        res.await.map(ServiceResponse::map_into_left_body)
                    })
                }
                _ => {
                    Box::pin(async move {
                        let response = HttpResponse::Found()
                            .insert_header((http::header::LOCATION, LOGIN))
                            .finish()
                            .map_into_right_body();
                        let (http_request, _) = req.into_parts();
                        return Ok(ServiceResponse::new(http_request, response));
                    })
                }
            }
        }
        let res = self.service.call(req);
        Box::pin(async move {
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}

fn is_logged_in(req: &ServiceRequest) -> bool {
    if let Ok(_identity_result) = req.get_identity() {
        true
    } else {
        false
    }
}

fn is_secured(path: &str) -> bool {
    path.contains("api")
}

fn is_unsecured(path: &str) -> bool {
    path == LOGIN || path == REGISTER || path.starts_with("/assets")
}