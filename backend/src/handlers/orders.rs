use axum::{
    extract::{Path, State},
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
    models::{Order, OrderStatus},
    orders::{self, CreateOrderData},
};

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub listing_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: String,
}

/// Create a new order
pub async fn create_order(
    State(pool): State<PgPool>,
    claims: Claims,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<impl IntoResponse, DoftaError> {
    let data = CreateOrderData {
        listing_id: payload.listing_id,
        quantity: payload.quantity,
    };

    let order = orders::create_order(&pool, claims.sub, data).await?;

    Ok((StatusCode::CREATED, Json(order)))
}

/// Get all orders for the current user (as buyer or seller)
pub async fn get_my_orders(
    State(pool): State<PgPool>,
    claims: Claims,
) -> Result<impl IntoResponse, DoftaError> {
    let mut buyer_orders = orders::get_orders_by_buyer(&pool, claims.sub).await?;
    let seller_orders = orders::get_orders_by_seller(&pool, claims.sub).await?;

    buyer_orders.extend(seller_orders);

    Ok(Json(buyer_orders))
}

/// Get a single order by ID
pub async fn get_order(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, DoftaError> {
    let order = orders::get_order(&pool, id).await?;

    // Verify user is buyer or seller
    if order.buyer_id != claims.sub && order.seller_id != claims.sub {
        return Err(DoftaError::Forbidden(
            "You can only view your own orders".to_string(),
        ));
    }

    Ok(Json(order))
}

/// Update order status
pub async fn update_order_status(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateOrderStatusRequest>,
) -> Result<impl IntoResponse, DoftaError> {
    let order = orders::get_order(&pool, id).await?;

    // Parse status
    let new_status: OrderStatus = payload
        .status
        .parse()
        .map_err(|_| DoftaError::InvalidInput("Invalid order status".to_string()))?;

    // Determine which action to take based on status and user role
    let updated_order = match new_status {
        OrderStatus::Accepted => {
            if order.seller_id != claims.sub {
                return Err(DoftaError::Forbidden(
                    "Only seller can accept order".to_string(),
                ));
            }
            orders::accept_order(&pool, id, claims.sub).await?
        }
        OrderStatus::Rejected => {
            if order.seller_id != claims.sub {
                return Err(DoftaError::Forbidden(
                    "Only seller can reject order".to_string(),
                ));
            }
            orders::reject_order(&pool, id, claims.sub).await?
        }
        OrderStatus::Completed => {
            orders::complete_order(&pool, id).await?
        }
        OrderStatus::Cancelled => {
            if order.buyer_id != claims.sub {
                return Err(DoftaError::Forbidden(
                    "Only buyer can cancel order".to_string(),
                ));
            }
            orders::cancel_order(&pool, id, claims.sub).await?
        }
        _ => {
            return Err(DoftaError::InvalidInput(
                "Invalid status transition".to_string(),
            ))
        }
    };

    Ok(Json(updated_order))
}
