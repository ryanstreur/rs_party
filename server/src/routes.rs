/*!
# Axum route handlers

Some functions
*/

use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Json, Path, State};
use axum::http::{HeaderMap, StatusCode};
use regex::Regex;

use sqlx::pool::PoolConnection;
use sqlx::{PgPool, Postgres};
use uuid::Uuid;

use crate::db::get_user_from_session_key;
use crate::model::{self, ApiError, NewUserParams, RoleType, SessionUser};
use crate::{db, model::LoginParams};

/// State Struct To Get Passed to Routes
pub struct AppState {
    pub db: PgPool,
}

/// Get database connection from Application State
pub async fn conn_from_state(state: &Arc<AppState>) -> Result<PoolConnection<Postgres>, ApiError> {
    match state.db.acquire().await {
        Ok(conn) => Ok(conn),
        Err(e) => Err(ApiError::from(e)),
    }
}

/// Get bearer token from header string
pub fn extract_bearer_token(header_str: &str) -> Result<Uuid, ApiError> {
    // https://docs.rs/regex/latest/regex/

    let Ok(re) = Regex::new(r"Bearer: (?<token>.+)") else {
        return Err(ApiError::from((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create regex",
        )));
    };

    let Some(caps) = re.captures(header_str) else {
        return Err(ApiError::from((StatusCode::BAD_REQUEST, "No bearer token")));
    };

    let token = caps["token"].to_string();
    let session_key_result = Uuid::from_str(&token);

    let session_key = match session_key_result {
        Ok(key) => key,
        Err(_e) => {
            return Err(ApiError::from((
                StatusCode::BAD_REQUEST,
                "failed to parse uuid",
            )))
        }
    };

    Ok(session_key)
}

/// Handle health check requests
pub async fn get_hc_handler() -> StatusCode {
    StatusCode::OK
}

/// Handle registration requests
pub async fn registration_handler(
    State(state): State<Arc<AppState>>,
    Json(new_user_params): Json<NewUserParams>,
) -> Result<String, ApiError> {
    let mut conn = conn_from_state(&state).await?;
    let user_result = db::insert_user(&mut conn, &new_user_params).await;

    let user = match user_result {
        Ok(user) => user,
        Err(_) => {
            return Err(ApiError::from((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert user",
            )))
        }
    };

    let session = db::create_session(&mut conn, &user).await?;
    Ok(session.session_key.to_string())
}

/// Handle login requests
pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(login_params): Json<LoginParams>,
) -> Result<String, ApiError> {
    let conn_result = state.db.acquire().await;

    let conn = match conn_result {
        Ok(c) => c,
        Err(e) => {
            return Err(ApiError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(e.to_string()),
            })
        }
    };

    let user_result = db::login(conn, &login_params).await;
    match user_result {
        Ok(session_id) => Ok(session_id),
        Err(err_str) => Err(err_str),
    }
}

/// Get session and user data by extracting session key from header and
/// getting data from database.
pub async fn authenticate(
    state: Arc<AppState>,
    headers: HeaderMap,
) -> Result<SessionUser, ApiError> {
    let mut conn = conn_from_state(&state).await?;
    let header = match headers.get("Authorization") {
        Some(h) => h,
        None => {
            return Err(ApiError {
                status_code: StatusCode::UNAUTHORIZED,
                message: Some("Auth header missing".to_string()),
            })
        }
    };

    let header_str = match header.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            return Err(ApiError {
                status_code: StatusCode::BAD_REQUEST,
                message: Some("Could not extract authorization header".to_string()),
            })
        }
    };

    let token = extract_bearer_token(&header_str)?;
    let su_result = get_user_from_session_key(&mut conn, &token).await;
    let su = match su_result {
        Ok(s) => s,
        Err(_) => {
            return Err(ApiError::from((
                StatusCode::UNAUTHORIZED,
                "Session not retrieved",
            )))
        }
    };
    let now = chrono::Utc::now();

    let diff = now - su.created;

    // If the most recent session is older than 5 hours, send 401
    if diff.num_hours() >= 5 {
        return Err(ApiError::from((
            StatusCode::UNAUTHORIZED,
            "Session expired",
        )));
    }

    Ok(su)
}

/// Get data for the currently authenticated user based on the session key in the header
pub async fn get_user_self_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    _: String,
) -> Result<Json<model::User>, ApiError> {
    let su = authenticate(state, headers).await?;
    Ok(Json(model::User::from(su)))
}

/// Handle post requests for creating new events
pub async fn post_event_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(new_event): Json<model::Event>,
) -> Result<Json<model::Event>, ApiError> {
    let mut conn = conn_from_state(&state).await?;
    let su = authenticate(state, headers).await?;
    let event = db::insert_event(&mut conn, &new_event).await?;

    let event_id = match event.id {
        Some(id) => id,
        None => {
            return Err(ApiError::from((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to save event",
            )))
        }
    };

    let new_role = model::Role {
        role_type: model::RoleType::Owner,
        user_id: su.user_id,
        event_id,
        ..Default::default()
    };

    let role_insert_result = db::insert_role(&mut conn, &new_role).await;

    match role_insert_result {
        Ok(_) => Ok(Json(event)),
        Err(e) => Err(ApiError::from(e)),
    }
}

/// Handle patch requests to update events
pub async fn patch_event_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(event_to_update): Json<model::Event>,
) -> Result<Json<model::Event>, ApiError> {
    let mut conn = conn_from_state(&state).await?;
    let su = authenticate(state, headers).await?;

    let event_id = match event_to_update.id {
        Some(id) => id,
        None => return Err(ApiError::from((StatusCode::BAD_REQUEST, "Event has no ID"))),
    };

    let user_role = db::get_event_user_role(&mut conn, &su.user_id, &event_id).await?;

    if user_role.role_type == RoleType::Guest {
        return Err(ApiError::from((
            StatusCode::UNAUTHORIZED,
            "Not authorized to modify event",
        )));
    }

    let update_result = db::update_event(&mut conn, &event_to_update).await;

    match update_result {
        Ok(event) => Ok(Json(event)),
        Err(e) => Err(ApiError::from(e)),
    }
}

/// Handle delete requests to remove events
pub async fn delete_event_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(event_id): Path<i64>,
) -> Result<(), ApiError> {
    let mut conn = conn_from_state(&state).await?;
    let su = authenticate(state, headers).await?;

    let user_role = db::get_event_user_role(&mut conn, &su.user_id, &event_id).await?;

    if user_role.role_type == RoleType::Guest {
        return Err(ApiError::from((
            StatusCode::UNAUTHORIZED,
            "Not authorized to delete event",
        )));
    }

    let role_id = match user_role.id {
        Some(id) => id,
        None => {
            return Err(ApiError::from((
                StatusCode::INTERNAL_SERVER_ERROR,
                "No role ID",
            )))
        }
    };

    let role_deletion_result = db::delete_role(&mut conn, &role_id).await;

    match role_deletion_result {
        Ok(_) => (),
        Err(e) => return Err(ApiError::from(e)),
    }

    let deletion_result = db::delete_event(&mut conn, &event_id).await;

    match deletion_result {
        Ok(_) => Ok(()),
        Err(e) => Err(ApiError::from(e)),
    }
}

/// Get a list of events for which the currently-authenticated user is the owner
pub async fn get_owned_events_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<model::Event>>, ApiError> {
    let mut conn = conn_from_state(&state).await?;
    let su = authenticate(state, headers).await?;

    let owned_events_result = db::owned_events(&mut conn, &su.user_id).await;

    match owned_events_result {
        Ok(events) => Ok(Json(events)),
        Err(e) => Err(ApiError::from(e)),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_extract_bearer_token() {
        let token_in = Uuid::new_v4().to_string();
        let auth_header = format!("Bearer: {}", token_in);

        // let token_out = extract_bearer_token(&auth_header).expect("Failed to extract bearer token");

        let Ok(re) = Regex::new(r"Bearer: (?<token>.+)") else {
            panic!("Regex construction failed");
        };

        let Some(caps) = re.captures(&auth_header) else {
            panic!("No captures");
        };

        let token_out = caps["token"].to_string();
        assert_eq!(token_out, token_in.to_string());

        let token_out_uuid =
            extract_bearer_token(&auth_header).expect("Capturing bearer token failed");
        assert_eq!(token_out_uuid.to_string(), token_in);
    }
}
