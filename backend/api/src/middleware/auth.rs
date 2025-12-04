use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use common::utils::verify_token;
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        Box::pin(async move {
            let auth_header = req.headers().get("Authorization");

            if let Some(auth_header) = auth_header {
                if let Ok(auth_str) = auth_header.to_str() {
                    // Support both "Bearer <token>" and direct "<token>" formats
                    let token = if auth_str.starts_with("Bearer ") {
                        &auth_str[7..]
                    } else {
                        auth_str
                    };

                    if let Ok(claims) = verify_token(token) {
                        req.extensions_mut().insert(claims);
                        return srv.call(req).await;
                    }
                }
            }

            // For now, if auth fails, we can either return 401 or let it pass but without claims.
            // The handlers can check for claims if they need protected access.
            // Or we can enforce it here.
            // Let's enforce it for simplicity, but we need to exclude public routes like login.
            // Since this middleware is applied to specific scopes or services, we can control it there.

            // If we apply this globally or to a scope, we must ensure public routes are not blocked.
            // However, Actix middleware runs for all routes in the scope.
            // A better approach for this simple implementation is to return 401 if token is invalid,
            // assuming this middleware is ONLY applied to protected routes.

            Err(actix_web::error::ErrorUnauthorized(
                "Invalid or missing token",
            ))
        })
    }
}
