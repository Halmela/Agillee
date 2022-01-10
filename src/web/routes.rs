#[macro_use]
use crate::models::objects::*;
use crate::models::structure::*;
use crate::models::object::*;
use crate::models::edge::*;
use crate::commander::*;
use crate::web::templates::page;

use rocket_sync_db_pools::{ postgres, database };
use rocket::{routes, get, Rocket, Build };
use rocket::fairing::AdHoc;
use rocket::response::content;


#[get("/")]
async fn index(client: Db) -> content::Html<String> {
    let s = client.run(
        move |c| {
            Commander::read(
                c.transaction()?,
                Structure::blank())
        }
    ).await.unwrap();

    let res = page(Some(s));
    
    content::Html(res)
}



        
#[get("/o/<id>")]
async fn object(client: Db, id: i32) -> content::Html<String> {
    let o = client.run(
        move |c| {
            Commander::execute(
                c.transaction()?,
                Command::Read(Structure::from(id))
            )
        }
    ).await.unwrap();

    let res = "".to_string();

/*
    let res = format!("{}", html! {
        : doctype::HTML;
        html {
            head {
                title : "Agilleen kantapää";
            }
            body {
                : render_structure(&o)
            }
        }
    });
    */
    content::Html(res)
}


async fn init_db(rocket: Rocket<Build>) -> Rocket<Build> {
    Db::get_one(&rocket).await
        .expect("database mounte failed")
        .run(|c| Commander::init(c.transaction()?))
        .await
        .expect("init failed");
    rocket
}

#[database("postgres")]
struct Db(postgres::Client);

pub fn stage_db() -> AdHoc {
    AdHoc::on_ignite("Postgres Stage", |rocket| async {
        rocket.attach(Db::fairing())
            .attach(AdHoc::on_ignite("Postgres Init", init_db))
            .mount("/", routes![index, object])
    })
}

