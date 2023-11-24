use axum::{middleware, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;

use axum_practice::{fn_mod::*, router_mod::*, web_mod, error_mod::*, model_mod::ModelController};
use axum_practice::web_mod::routes_tickets::routes;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    // route_layer applies the middleware to this route only
    let routes_apis = routes(mc.clone()).route_layer(middleware::from_fn(web_mod::mw_auth::mw_require_auth));

    // merge "merges" router to another router
    // layer gets executed from bottom (furthermost) to top
    let routes_all = Router::new()
        .merge(routes_hello()) // merge a router to another route
        .nest("/api", routes_apis) // all routes in the arg is prefixed by /api
        .merge(web_mod::routes_login_mod::routes())
        // layer insert a middleware (middleware is a mod)
        // map_response creates a middleware from an async fn that modifies responses
        .layer(middleware::map_response(main_response_mapper))
        // at bottom to apply the cookiemanager to other parts of the router
        .layer(CookieManagerLayer::new());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("->> LISTENING on {}\n", addr);

    run_server(addr, routes_all).await;

    Ok(())
}
