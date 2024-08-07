
#[macro_use] extern crate rocket;
mod routes;
use crate::routes::auth::route::auth;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
#[launch]
fn rocket() -> _ {
    #[allow(unused_variables)]
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);
    rocket::build()
    .mount("/api/v1", 
    routes![
        auth
        ]
)
}