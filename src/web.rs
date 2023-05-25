use crate::database::*;
use askama::Template;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::post;
use axum::{routing::get, Router};
use axum::{Form, Server};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Entry {
    timestamp: String,
    bmi: String,
}

#[derive(Template, Default)]
#[template(path = "index.html")]
struct Indextemplate {
    entries: Vec<Entry>,
}

//Start seite
async fn index(State(database): State<Arc<RwLock<Database>>>) -> impl IntoResponse {
    println!("index");
    let entry = database
        .read()
        .await
        .entry_iter()
        .map(|entry| Entry {
            timestamp: entry.timestamp().to_string(),
            bmi: entry.bmi().value().to_string(),
        })
        .collect();
    let template = Indextemplate { entries: entry };
    render_template(template)
}

#[derive(Template, Default)]
#[template(path = "entry_form.html")]
struct Entryform {
    weight: f64,
    height: f64,
}

async fn new_entry() -> impl IntoResponse {
    render_template(Entryform::default())
}

#[derive(Deserialize)]
struct FormFields {
    height: f64,
    weight: f64,
}

#[derive(Template)]
#[template(path = "submit.html")]
struct NewEntry {
    bmi: f64,
}

#[derive(Template)]
#[template(path = "submit_error.html")]
struct BmiError {
    error: String,
}

#[derive(Template)]
#[template(path = "submit_error.html")]
struct DBError {
    error: String,
}

//submit new entrie
async fn submit_new_entry(
    State(database): State<Arc<RwLock<Database>>>,
    Form(fields): Form<FormFields>,
) -> impl IntoResponse {
    match crate::calculate_bmi(
        crate::height::Height::new(fields.height),
        crate::weight::Weight::new(fields.weight),
    ) {
        Ok(bmi) => match DatabaseEntry::new(bmi.clone()) {
            Ok(entry) => {
                let mut write_handle = database.write().await;
                write_handle.add_entry(entry);
                render_template(NewEntry { bmi: bmi.value() })
            }
            Err(dberror) => render_template(DBError {
                error: format!("{dberror:?}"),
            }),
        },
        Err(error) => render_template(BmiError {
            error: format!("{error:?}"),
        }),
    }
}

pub async fn router(State(database): State<Arc<RwLock<Database>>>) {
    let router = Router::new()
        .route("/", get(index))
        .route("/new", get(new_entry))
        .route("/submit", post(submit_new_entry))
        .with_state(database);

    let socket_addr = "127.0.0.1:8080".parse().unwrap();

    Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await
        .unwrap()
}

pub fn render_template(template: impl Template) -> (StatusCode, Html<String>) {
    match template.render() {
        Ok(rendered) => (StatusCode::OK, Html(rendered)),
        Err(e) => {
            eprintln!("Failed to render template: {e:?}");
            (StatusCode::INTERNAL_SERVER_ERROR, Html(String::new()))
        }
    }
}
