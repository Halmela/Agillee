use ::postgres::{NoTls, Error,Transaction};
use agillee::objects::*;
use agillee::structure::*;
use agillee::object::*;
use agillee::database::*;
//use agillee::cli::*;
use agillee::commander::*;
use rocket_sync_db_pools::{ postgres, database };

#[macro_use] extern crate rocket;

#[get("/")]
async fn index(client: Db) -> String {
    let res = client.run(
        |c| {
            Commander::execute(
                c.transaction()?,
                Command::Read(Structure::blank()))
        //let cmd = Commander::new(c.transaction()?);
        }
    ).await;

    res.unwrap().to_string()
}


#[launch]
fn rocket() -> _ {
    //rocket::build().mount("/", routes![index])
    rocket::build().attach(Db::fairing()).mount("/", routes![index])
}



#[database("postgres")]
struct Db(postgres::Client);

