//! Public Transparency API — Proof-of-Reserves data feed.
//!
//! Routes:
//!   GET /v1/public/transparency              — latest signed snapshot
//!   GET /v1/public/transparency/history?days=30|90|365 — time-series data

use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::transparency::{TransparencyError, TransparencyService};

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

pub struct TransparencyState {
    pub service: Arc<TransparencyService>,
}

// ---------------------------------------------------------------------------
// Query params
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    /// Number of days of history to return. Allowed: 30, 90, 365. Default: 30.
    #[serde(default = "default_days")]
    pub days: u32,
}

fn default_days() -> u32 {
    30
}

// ---------------------------------------------------------------------------
// Error response
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

fn transparency_error_response(err: TransparencyError) -> Response {
    let (status, msg) = match &err {
        TransparencyError::NoSnapshot => (
            StatusCode::SERVICE_UNAVAILABLE,
            "No proof-of-reserves snapshot available yet".to_string(),
        ),
        _ => {
            tracing::error!(error = %err, "Transparency API error");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
        }
    };
    (status, Json(ErrorBody { error: msg })).into_response()
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// GET /v1/public/transparency
///
/// Returns the latest cryptographically-signed Proof-of-Reserves snapshot.
/// Suitable for direct consumption by CoinGecko, CoinMarketCap, DeFi Llama, etc.
pub async fn get_transparency(
    State(state): State<Arc<TransparencyState>>,
) -> Response {
    match state.service.get_latest().await {
        Ok(payload) => (
            StatusCode::OK,
            [
                (header::CACHE_CONTROL, "public, max-age=60, s-maxage=60"),
                (header::CONTENT_TYPE, "application/json"),
            ],
            Json(payload),
        )
            .into_response(),
        Err(e) => transparency_error_response(e),
    }
}

/// GET /v1/public/transparency/history?days=30
///
/// Returns time-series supply/reserve data for the requested period.
/// Allowed values for `days`: 30, 90, 365.
pub async fn get_transparency_history(
    State(state): State<Arc<TransparencyState>>,
    Query(params): Query<HistoryQuery>,
) -> Response {
    let days = match params.days {
        30 | 90 | 365 => params.days,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: "days must be 30, 90, or 365".to_string(),
                }),
            )
                .into_response();
        }
    };

    match state.service.get_history(days).await {
        Ok(history) => (
            StatusCode::OK,
            [
                (header::CACHE_CONTROL, "public, max-age=300, s-maxage=300"),
                (header::CONTENT_TYPE, "application/json"),
            ],
            Json(history),
        )
            .into_response(),
        Err(e) => transparency_error_response(e),
    }
}

// ---------------------------------------------------------------------------
// Router
// ---------------------------------------------------------------------------

pub fn transparency_routes(state: Arc<TransparencyState>) -> Router {
    Router::new()
        .route("/v1/public/transparency", get(get_transparency))
        .route("/v1/public/transparency/history", get(get_transparency_history))
        .with_state(state)
}
