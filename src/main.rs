#[macro_use] extern crate rocket;
mod cmd;
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![cmd::api::hello])
}