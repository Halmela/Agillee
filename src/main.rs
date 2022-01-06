use ::postgres::{NoTls, Error,Transaction};
use agillee::objects::*;
use agillee::structure::*;
use agillee::object::*;
use agillee::database::*;
//use agillee::cli::*;
use agillee::commander::*;
use rocket_sync_db_pools::{ postgres, database };
use rocket::{ Rocket, Build };
use rocket::fairing::AdHoc;

#[macro_use] extern crate rocket;

#[get("/")]
async fn index(client: Db) -> String {
    let res = client.run(
        move |c| {
            Commander::execute(
                c.transaction()?,
                Command::Read(Structure::blank()))
        //let cmd = Commander::new(c.transaction()?);
        }
    ).await;

    res.unwrap().to_string()
}

#[get("/o/<id>")]
async fn object(client: Db, id: i32) -> String {
    let res = client.run(
        move |c| {
            Commander::execute(
                c.transaction()?,
                Command::Read(Structure::from(id))
            )
        }
    ).await;

    res.unwrap().to_string()
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(stage())
}

async fn init_db(rocket: Rocket<Build>) -> Rocket<Build> {
    Db::get_one(&rocket).await
        .expect("database mounted")
        .run(|c| Commander::execute(c.transaction()?, Command::Init))
        .await
        .expect("inited");
    rocket
}

#[database("postgres")]
struct Db(postgres::Client);

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Postgres Stage", |rocket| async {
        rocket.attach(Db::fairing())
            .attach(AdHoc::on_ignite("Postgres Init", init_db))
            .mount("/", routes![index, object])
    })
}
