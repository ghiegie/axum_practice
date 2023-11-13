// dependencies from packages

// dependencies from other files
use crate::routes_setup_lib::create_routes;

// submodule declaration

pub async fn run() {
    let app = create_routes();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
