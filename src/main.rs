use agillee::web::routes::{stage_db};

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(stage_db())
}
