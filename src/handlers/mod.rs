use crate::{
    app::State,
    templates::{self, Html, RenderRucte},
};
use chrono::{Datelike, Timelike, Utc};
use lazy_static::lazy_static;
use std::{convert::Infallible, fmt, sync::Arc};
use tracing::instrument;
use warp::{
    http::{Response, StatusCode},
    Rejection, Reply,
};

lazy_static! {
    pub static ref LAST_MODIFIED: String = {
        let now = Utc::now();
        format!(
            "{dayname}, {day} {month} {year} {hour}:{minute}:{second} GMT",
            dayname = now.weekday(),
            day = now.day(),
            month = now.month(),
            year = now.year(),
            hour = now.hour(),
            minute = now.minute(),
            second = now.second()
        )
    };
}

#[instrument]
pub async fn index() -> Result<impl Reply, Rejection> {
    Response::builder()
        .header("Last-Modified", &*LAST_MODIFIED)
        .html(|o| templates::index_html(o))
}

#[instrument]
pub async fn contact() -> Result<impl Reply, Rejection> {
    Response::builder()
        .header("Last-Modified", &*LAST_MODIFIED)
        .html(|o| templates::contact_html(o))
}

#[instrument]
pub async fn feeds() -> Result<impl Reply, Rejection> {
    Response::builder()
        .header("Last-Modified", &*LAST_MODIFIED)
        .html(|o| templates::feeds_html(o))
}

#[instrument(skip(state))]
pub async fn resume(state: Arc<State>) -> Result<impl Reply, Rejection> {
    let state = state.clone();
    Response::builder()
        .header("Last-Modified", &*LAST_MODIFIED)
        .html(|o| templates::resume_html(o, Html(state.resume.clone())))
}

#[instrument(skip(state))]
pub async fn patrons(state: Arc<State>) -> Result<impl Reply, Rejection> {
    let state = state.clone();
    match &state.patrons {
        None => Response::builder().status(500).html(|o| {
            templates::error_html(
                o,
                "Could not load patrons, let me know the API token expired again".to_string(),
            )
        }),
        Some(patrons) => Response::builder()
            .header("Last-Modified", &*LAST_MODIFIED)
            .html(|o| templates::patrons_html(o, patrons.clone())),
    }
}

#[instrument]
pub async fn not_found() -> Result<impl Reply, Rejection> {
    Response::builder()
        .header("Last-Modified", &*LAST_MODIFIED)
        .html(|o| templates::notfound_html(o, "some path".into()))
}

pub mod blog;
pub mod feeds;
pub mod gallery;

#[derive(Debug, thiserror::Error)]
struct PostNotFound(String, String);

impl fmt::Display for PostNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "not found: {}/{}", self.0, self.1)
    }
}

impl warp::reject::Reject for PostNotFound {}

#[derive(Debug, thiserror::Error)]
struct SeriesNotFound(String);

impl fmt::Display for SeriesNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl warp::reject::Reject for SeriesNotFound {}

#[instrument]
pub async fn rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let path: String;
    let code;

    if err.is_not_found() {
        path = "".into();
        code = StatusCode::NOT_FOUND;
    } else if let Some(SeriesNotFound(series)) = err.find() {
        log::error!("invalid series {}", series);
        path = format!("/blog/series/{}", series);
        code = StatusCode::NOT_FOUND;
    } else if let Some(PostNotFound(kind, name)) = err.find() {
        log::error!("unknown post {}/{}", kind, name);
        path = format!("/{}/{}", kind, name);
        code = StatusCode::NOT_FOUND;
    } else {
        log::error!("unhandled rejection: {:?}", err);
        path = format!("weird rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
    }

    Ok(warp::reply::with_status(
        Response::builder()
            .html(|o| templates::notfound_html(o, path))
            .unwrap(),
        code,
    ))
}
