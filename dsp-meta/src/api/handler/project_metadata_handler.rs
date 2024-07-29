use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use tracing::{instrument, trace};

use crate::api::model::project_metadata_dto::{ProjectMetadataDto, ProjectMetadataWithInfoDto};
use crate::app_state::AppState;
use crate::domain::model::draft_model::Shortcode;
use crate::domain::service::project_metadata_api_contract::ProjectMetadataApiContract;
use crate::domain::service::repository_contract::{Filter, Pagination};
use crate::error::DspMetaError;

/// GET /project_metadata/:shortcode
/// Get project metadata by shortcode
///
/// TODO: Add error handling with correct status codes
#[instrument(skip(state))]
pub async fn get_project_metadata_by_shortcode(
    Path(shortcode): Path<Shortcode>,
    State(state): State<Arc<AppState>>,
) -> Result<Response, DspMetaError> {
    trace!("entered get_project_metadata_by_shortcode()");
    state
        .project_metadata_service
        .find_by_id(&shortcode)
        .map(|option| match option {
            Some(metadata) => (StatusCode::OK, ProjectMetadataDto(metadata)).into_response(),
            None => (
                StatusCode::NOT_FOUND,
                format!("No project {} available", shortcode.as_string()),
            )
                .into_response(),
        })
}

#[instrument(skip(state))]
pub async fn get_all_project_metadata(
    State(state): State<Arc<AppState>>,
    pagination: Option<Query<Pagination>>,
    filter: Option<Query<Filter>>,
) -> Result<Response, DspMetaError> {
    trace!("entered get_all_project_metadata()");
    let Query(pagination) = pagination.unwrap_or_default();
    let Query(filter) = filter.unwrap_or_default();
    let page = state.project_metadata_service.find(&filter, &pagination)?;
    let mut response = Json(
        page.data
            .into_iter()
            .map(ProjectMetadataWithInfoDto::from)
            .collect::<Vec<ProjectMetadataWithInfoDto>>(),
    )
    .into_response();
    response
        .headers_mut()
        .insert("X-Total-Count", page.total.to_string().parse().unwrap());
    Ok(response)
}
