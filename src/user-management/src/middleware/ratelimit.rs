// user-service/src/middleware/rate_limit.rs
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use redis::AsyncCommands;
use actix_web::{dev::ServiceRequest, Error};
use actix_web_lab::middleware::Next;
use redis::AsyncCommands;



pub struct RateLimiter {
    requests_per_minute: u32,
}

impl RateLimiter {
    pub fn new(requests_per_minute: u32) -> Self {
        Self { requests_per_minute }
    }
}

impl<S> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse>,
{
    type Response = ServiceResponse;
    type Error = S::Error;
    type Transform = RateLimiterMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware {
            service,
            requests_per_minute: self.requests_per_minute,
        }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
    requests_per_minute: u32,
}

impl<S> Service<ServiceRequest> for RateLimiterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse>,
{
    type Response = ServiceResponse;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let redis_client = req.app_data::<web::Data<RedisClient>>().unwrap().clone();
        let path = req.path().to_string();
        let ip = req.connection_info().peer_addr().unwrap_or("unknown").to_string();

        let limit = self.requests_per_minute;
        let fut = self.service.call(req);

        Box::pin(async move {
            let key = format!("rate_limit:{}:{}", ip, path);
            let mut conn = redis_client.get_async_connection().await?;
            let count: u32 = conn.incr(&key, 1).await?;
            
            if count == 1 {
                let _: () = conn.expire(&key, 60).await?;
            }

            if count > limit {
                return Ok(ServiceResponse::new(
                    req.into_parts().0,
                    HttpResponse::TooManyRequests().finish(),
                ));
            }

            fut.await
        })
    }
}

pub async fn rate_limiter(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody + 'static>,
) -> Result<impl actix_web::Responder, Error> {
    let redis = req.app_data::<redis::Client>().expect("Redis client");
    let path = req.path();
    let ip = req.connection_info().realip_remote_addr().unwrap_or("unknown");

    let mut conn = redis.get_async_connection().await?;
    let key = format!("rate_limit:{}:{}", ip, path);
    
    let count: i64 = conn.incr(&key, 1).await?;
    if count == 1 {
        let _: () = conn.expire(&key, 60).await?;
    }

    if count > match path {
        "/auth/login" => 5,    // 5 attempts/min for login
        "/auth/register" => 3, // 3 registrations/min
        _ => 100               // General API limit
    } {
        return Err(Error::from(AuthError::RateLimitExceeded));
    }

    next.call(req).await
}

pub async fn track_login_attempt(
    redis: &mut redis::aio::Connection,
    email: &str
) -> Result<(), redis::RedisError> {
    let key = format!("login_attempts:{}", email);
    redis.incr(&key).await?;
    redis.expire(&key, 3600).await?;
    Ok(())
}