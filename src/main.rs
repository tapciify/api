pub mod routes;

use routes::create_routes;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let app = create_routes();

    Ok(app.into())
}
