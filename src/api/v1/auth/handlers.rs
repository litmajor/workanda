use actix_web::{web, HttpResponse, Responder};
use crate::database::DbPool;
use crate::models::user::{User, NewUser};
use crate::error::ApiError;

pub async fn register_handler(
    pool: web::Data<DbPool>,
    data: web::Json<RegisterRequest>,
) -> impl Responder {
    let new_user = data.into_inner();

    match crate::database::queries::create_user(pool.as_ref(), new_user).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_) => HttpResponse::BadRequest().body("Failed to register user"),
    }
}

pub async fn login_handler(
    pool: web::Data<DbPool>,
    data: web::Json<LoginRequest>,
) -> impl Responder {
    let user_data = data.into_inner();

    match crate::database::queries::get_user_by_email(pool.as_ref(), &user_data.email).await {
        Ok(Some(user)) => {
            if bcrypt::verify(&user_data.password, &user.password_hash).unwrap_or(false) {
                let token = crate::auth::jwt::create_jwt(&user);
                HttpResponse::Ok().json(json!({ "token": token }))
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        _ => HttpResponse::NotFound().body("User not found"),
    }
}

pub async fn login(
    pool: web::Data<DbPool>,
    data: web::Json<LoginRequest>,
) -> Result<HttpResponse, ApiError> {
    let email = &data.email;
    let password = &data.password;

    // Fetch the user from the database
    let user = match crate::database::queries::get_user_by_email(pool.as_ref(), email).await? {
        Some(user) => user,
        None => return Err(ApiError::NotFound("User not found".to_string())),
    };

    // Verify the password hash
    if !verify_password(password, &user.hashed_password) {
        return Err(ApiError::Unauthorized("Invalid credentials".to_string()));
    }

    // Generate a JWT token for the user
    let token = create_jwt(&user)?;

    Ok(HttpResponse::Ok().json(json!({ "token": token })))
}

pub async fn register(
    pool: web::Data<DbPool>,
    data: web::Json<RegisterRequest>,
) -> Result<HttpResponse, ApiError> {
    let new_user = NewUser {
        username: data.username.clone(),
        email: data.email.clone(),
        hashed_password: hash_password(&data.password)?,
        profile_picture: data.profile_picture.clone(),
    };

    // Insert the new user into the database
    let created_user = match crate::database::queries::create_user(pool.as_ref(), new_user).await {
        Ok(user) => user,
        Err(_) => return Err(ApiError::Conflict("Email already registered".to_string())),
    };

    info!("User registered successfully: {:?}", created_user);
    Ok(HttpResponse::Created().json("Registered"))
}