pub(crate) mod model;
mod routes;
mod snapshot;

use crate::service::Service;
use anyhow::Result;
use axum::middleware;
use orbien_core::config::DashboardConfig;
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn run(svc: Arc<Service>, cfg: DashboardConfig) -> Result<()> {
    let addr = format!("{}:{}", cfg.addr, cfg.port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!(%addr, user = %cfg.user, "dashboard listening");

    let state = Arc::new(DashState { svc, cfg });
    let app = routes::router(state.clone())
        .layer(middleware::from_fn_with_state(state, routes::basic_auth))
        .into_make_service();

    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Clone)]
pub struct DashState {
    pub svc: Arc<Service>,
    pub cfg: DashboardConfig,
}
