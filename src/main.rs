pub mod routes;

use routes::create_routes;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = create_routes();

    Ok(router.into())
}
