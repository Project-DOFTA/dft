use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::{self, Claims},
    error::DoftaError,
    models::Member,
};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub farm_name: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub member: Member,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Register a new member
pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, DoftaError> {
    // Register member
    let member = auth::register_member(
        &pool,
        &payload.email,
        &payload.password,
        &payload.name,
        payload.farm_name.as_deref(),
        payload.location.as_deref(),
    )
    .await?;

    // Generate JWT token
    let token = auth::generate_token(&member.id)?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse { member, token }),
    ))
}

/// Login existing member
pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, DoftaError> {
    // Authenticate member
    let member = auth::authenticate_member(&pool, &payload.email, &payload.password).await?;

    // Generate JWT token
    let token = auth::generate_token(&member.id)?;

    Ok(Json(AuthResponse { member, token }))
}

/// Get current member profile
pub async fn get_profile(
    State(pool): State<PgPool>,
    claims: Claims,
) -> Result<impl IntoResponse, DoftaError> {
    let member = sqlx::query_as::<_, Member>("SELECT * FROM members WHERE id = $1")
        .bind(claims.sub)
        .fetch_one(&pool)
        .await
        .map_err(|_| DoftaError::Unauthorized("Member not found".to_string()))?;

    Ok(Json(member))
}
