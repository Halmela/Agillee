use crate::models::structure::*;
use crate::models::object::*;
use crate::commander::*;
use crate::web::templates;

use rocket_sync_db_pools::{ postgres, database };
use rocket::{routes, get, post, Rocket, Build };
use rocket::form::Form;
use rocket::fairing::AdHoc;
use rocket::response::content::{RawHtml, RawCss};
use rocket::http::ContentType;


#[get("/")]
async fn index(client: Db) -> RawHtml<String> {
    let s = client.run(
        move |c| {
            Commander::read(
                c.transaction()?,
                Structure::blank())
        }
    ).await.unwrap();

    let res = templates::index(s);
    
    RawHtml(res)
}

#[post("/o/new", data = "<object>")]
async fn new_object(client: Db, object: Form<Object>) -> RawHtml<String> {
    let o = object.into_inner();
    let s = client.run (
        move |c| {
            Commander::create(
                c.transaction()?,
                Structure::from(o)
            )
        }
    ).await.unwrap();

    RawHtml(templates::response(s))
}

#[get("/style.css")]
async fn css() -> RawCss<&'static str> {
    RawCss(include_str!("style.css"))
}
        
#[get("/o/<id>")]
async fn object(client: Db, id: i32) -> RawHtml<String> {
    let s = client.run(
        move |c| {
            Commander::read(
                c.transaction()?,
                Structure::from(id)
            )
        }
    ).await.unwrap();

    let res = templates::response(s);

    RawHtml(res)
}

#[get("/o")]
async fn objects(client: Db) -> RawHtml<String> {
    let s = client.run(
        move |c| {
            Commander::read(
                c.transaction()?,
                Structure::blank()
            )
        }
    ).await.unwrap();

    let res = templates::page(s.objects);

    RawHtml(res)
}


#[get("/o/new")]
async fn object_form() -> RawHtml<String> {
    RawHtml(templates::object_form())
}

#[get("/e")]
async fn edges(client: Db) -> RawHtml<String> {
    let s = client.run(
        move |c| {
            Commander::read(
                c.transaction()?,
                Structure::blank()
            )
        }
    ).await.unwrap();

    let res = templates::page(s.edges);

    RawHtml(res)
}

#[get("/favicon.ico")]
async fn favicon() -> (ContentType, &'static [u8]) {
    (ContentType::Icon, include_bytes!("favicon.ico"))
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
            .mount("/", routes![
                index,
                favicon,
                object,
                objects,
                edges,
                css,
                new_object,
                object_form
                ])
    })
}

