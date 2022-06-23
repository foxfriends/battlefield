use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

pub struct Filter<M> {
    middleware: M,
    filter: fn(&ServiceRequest) -> bool,
}

impl<M> Filter<M> {
    pub fn new(middleware: M, filter: fn(&ServiceRequest) -> bool) -> Self {
        Self { middleware, filter }
    }
}

impl<S, B, M> Transform<S, ServiceRequest> for Filter<M>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    S: 'static,
    B: 'static,
    M: Transform<Rc<S>, ServiceRequest>,
    M::Future: 'static,
    M::Transform: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    <M::Transform as Service<ServiceRequest>>::Future: 'static,
    (): From<M::InitError>,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = FilterMiddleware<S, M::Transform>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Transform, Self::InitError>>>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let service = Rc::new(service);
        let middleware_future = self.middleware.new_transform(service.clone());
        let filter = self.filter;
        Box::pin(async move {
            Ok(FilterMiddleware {
                service,
                filter,
                middleware: middleware_future.await?,
            })
        })
    }
}

pub struct FilterMiddleware<S, M> {
    service: Rc<S>,
    filter: fn(&ServiceRequest) -> bool,
    middleware: M,
}

impl<S, B, M> Service<ServiceRequest> for FilterMiddleware<S, M>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
    M: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    M::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if (self.filter)(&req) {
            Box::pin(self.middleware.call(req))
        } else {
            Box::pin(self.service.call(req))
        }
    }
}
