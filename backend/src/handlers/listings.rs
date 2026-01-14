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
    auth::Claims,
    error::DoftaError,
    listings::{self, CreateListingData, ListingFilters, UpdateListingData},
    models::ProductListing,
};

#[derive(Debug, Deserialize)]
pub struct CreateListingRequest {
    pub name: String,
    pub description: String,
    pub category: String,
    pub unit_price: String,
    pub quantity_available: i32,
    pub unit_of_measure: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateListingRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub unit_price: Option<String>,
    pub quantity_available: Option<i32>,
    pub unit_of_measure: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub category: Option<String>,
    pub min_price: Option<String>,
    pub max_price: Option<String>,
    pub available_only: Option<bool>,
}

/// Create a new listing
pub async fn create_listing(
    State(pool): State<PgPool>,
    claims: Claims,
    Json(payload): Json<CreateListingRequest>,
) -> Result<impl IntoResponse, DoftaError> {
    let data = CreateListingData {
        member_id: claims.sub,
        name: payload.name,
        description: payload.description,
        category: payload.category,
        unit_price: payload.unit_price.parse().map_err(|_| {
            DoftaError::InvalidInput("Invalid unit price format".to_string())
        })?,
        quantity_available: payload.quantity_available,
        unit_of_measure: payload.unit_of_measure,
    };

    let listing = listings::create_listing(&pool, data).await?;

    Ok((StatusCode::CREATED, Json(listing)))
}

/// Get all listings (with optional filters)
pub async fn get_listings(
    State(pool): State<PgPool>,
    Query(query): Query<SearchQuery>,
) -> Result<impl IntoResponse, DoftaError> {
    let filters = ListingFilters {
        category: query.category,
        min_price: query.min_price.and_then(|p| p.parse().ok()),
        max_price: query.max_price.and_then(|p| p.parse().ok()),
        available_only: query.available_only.unwrap_or(true),
    };

    let listings = listings::search_listings(&pool, filters).await?;

    Ok(Json(listings))
}

/// Get a single listing by ID
pub async fn get_listing(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, DoftaError> {
    let listing = listings::get_listing(&pool, id).await?;

    Ok(Json(listing))
}

/// Update a listing
pub async fn update_listing(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateListingRequest>,
) -> Result<impl IntoResponse, DoftaError> {
    // Verify ownership
    let existing = listings::get_listing(&pool, id).await?;
    if existing.member_id != claims.sub {
        return Err(DoftaError::Forbidden(
            "You can only update your own listings".to_string(),
        ));
    }

    let data = UpdateListingData {
        name: payload.name,
        description: payload.description,
        category: payload.category,
        unit_price: payload.unit_price.and_then(|p| p.parse().ok()),
        quantity_available: payload.quantity_available,
        unit_of_measure: payload.unit_of_measure,
    };

    let listing = listings::update_listing(&pool, id, data).await?;

    Ok(Json(listing))
}

/// Delete a listing
pub async fn delete_listing(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, DoftaError> {
    // Verify ownership
    let existing = listings::get_listing(&pool, id).await?;
    if existing.member_id != claims.sub {
        return Err(DoftaError::Forbidden(
            "You can only delete your own listings".to_string(),
        ));
    }

    listings::delete_listing(&pool, id).await?;

    Ok(StatusCode::NO_CONTENT)
}
