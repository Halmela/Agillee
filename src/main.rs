#[macro_use]
extern crate horrorshow;
use horrorshow::helper::doctype;
use horrorshow::{ RenderBox };

use agillee::objects::*;
use agillee::structure::*;
use agillee::object::*;
use agillee::edge::*;
//use agillee::cli::*;
use agillee::commander::*;

use rocket_sync_db_pools::{ postgres, database };
use rocket::{ Rocket, Build };
use rocket::fairing::AdHoc;
use rocket::response::content;

#[macro_use] extern crate rocket;

#[get("/")]
async fn index(client: Db) -> content::Html<String> {
    let s = client.run(
        move |c| {
            Commander::execute(
                c.transaction()?,
                Command::Read(Structure::blank()))
        //let cmd = Commander::new(c.transaction()?);
        }
    ).await.unwrap();

    let res = format!("{}", html! {
        : doctype::HTML;
        html {
            head {
                title : "Agilleen kantapää";
            }
            body {
                h1(id="heading") {
                    : "otsikko"
                }
                : render_structure(&s);
            }
        }
    });
    
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

    content::Html(res)
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


fn render_structure(structure: &Structure) -> Box<dyn RenderBox + '_> {
    let (objects, edges) = (structure.get_objects(), structure.get_edges());
    box_html! {
        objects {
            header(class="structure-header") {
                h2 : "Structure";
                : render_objects(&objects);
                : render_edges(&edges);
            }
        }
    }
}


fn render_objects(objects: &Vec<Object>) -> Box<dyn RenderBox + '_> {
    box_html! {
        objects {
            header(class="objects-header") {
                h3 : "Objects";
                table {
                    tr {
                        th : "id";
                        th : "description";
                        th : "form";
                        th : "root";
                    }
                    @ for o in objects {
                        : render_object(&o)
                    }
                }
            }
        }
    }
}



fn render_object(object: &Object) -> Box<dyn RenderBox + '_> {
    //let Object { id, description, form, root } = object;
    box_html! {
        object {
            header(class="object-header") {
                    tr { 
                        td : object.get_id();
                        td : object.get_description();
                        td : object.get_form().map(|f| f.to_string());
                        td : object.get_root();
                    }
                
            }
        }
    }
}


fn render_edges(edges: &Vec<Edge>) -> Box<dyn RenderBox + '_> {
    box_html! {
        objects {
            header(class="edges-header") {
                h3 : "Edges";
                table {
                    tr {
                        th : "a";
                        th : "b";
                        th : "a2b";
                        th : "b2a";
                    }
                    @ for e in edges {
                        : render_edge(&e)
                    }
                }
            }
        }
    }
}

fn render_edge(edge: &Edge) -> Box<dyn RenderBox + '_> {
    box_html! {
        edge {
            header(class="edge-header") {
                tr {
                    td : edge.get_a();
                    td : edge.get_b();
                    td : edge.get_a2b();
                    td : edge.get_b2a();
                }
            }
        }
    }
}
