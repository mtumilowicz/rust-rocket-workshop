use rocket::{launch};
use rust_rocket_workshop::app::server;

#[launch]
fn rocket() -> _ {
    server()
}
