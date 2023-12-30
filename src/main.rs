use rocket::{launch};
use rust_rocket_workshop::app::server;

#[launch]
async fn rocket() -> _ {
    server()
}
