use crate::service::RedirectSchemeService;
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, BaseHttpResponse};
use futures::future::{ok, Ready, LocalBoxFuture};
use actix_web::body::AnyBody;

#[derive(Default, Clone)]
pub struct RedirectScheme {
    // Disabled redirections
    pub disable: bool,
    // Redirect to HTTP (true: HTTP -> HTTPS, false: HTTPS -> HTTP)
    pub https_to_http: bool,
    // Temporary redirect (true: 307 Temporary Redirect, false: 301 Moved Permanently)
    pub temporary: bool,
    // List of string replacements
    pub replacements: Vec<(String, String)>,
}

impl RedirectScheme {
    pub fn simple(https_to_http: bool) -> Self {
        RedirectScheme {
            https_to_http,
            ..Self::default()
        }
    }
    
    pub fn with_replacements<S: ToString>(https_to_http: bool, replacements: &[(S, S)]) -> Self {
        let replacements = replacements
            .iter()
            .map(|(a, b)| ((*a).to_string(), (*b).to_string()))
            .collect();
        RedirectScheme {
            https_to_http,
            replacements,
            ..Self::default()
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RedirectScheme
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error, Future = LocalBoxFuture<'static, Result<ServiceResponse, Error>>>,
    AnyBody: Into<BaseHttpResponse<B>>
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RedirectSchemeService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RedirectSchemeService {
            service,
            disable: self.disable,
            https_to_http: self.https_to_http,
            temporary: self.temporary,
            replacements: self.replacements.clone(),
        })
    }
}
