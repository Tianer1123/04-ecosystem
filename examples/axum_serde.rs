use anyhow::Result;
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex};
use tracing::{info, instrument, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer as _,
};

#[derive(Debug, Clone, PartialEq, Serialize)]
struct User {
    name: String,
    age: u32,
    skills: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct UserUpdate {
    age: Option<u32>,
    skills: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::DEBUG);

    tracing_subscriber::registry().with(console).init();

    let user = User {
        name: "Alice".to_string(),
        age: 28,
        skills: vec!["rust".to_string(), "Python".to_string()],
    };
    let user = Arc::new(Mutex::new(user));

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on http://{}", addr);

    let app = Router::new()
        .route("/", get(user_handler))
        .route("/", post(update_handler))
        .with_state(user);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[instrument]
async fn user_handler(State(user): State<Arc<Mutex<User>>>) -> Json<User> {
    user.lock().await.clone().into()
}

#[instrument]
async fn update_handler(
    State(user): State<Arc<Mutex<User>>>,
    Json(update_user): Json<UserUpdate>,
) -> Json<User> {
    let mut user = user.lock().await;
    if let Some(age) = update_user.age {
        user.age = age;
    }
    if let Some(skills) = update_user.skills {
        user.skills = skills;
    }
    user.clone().into()
}
