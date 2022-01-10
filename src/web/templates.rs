//#[macro_use]
//use horrorshow::helper::doctype;
//use horrorshow::{ box_html, html, RenderBox };

use maud::{ html, DOCTYPE, Markup, Render };

use crate::models::structure::*;
use crate::models::object::*;
use crate::models::objects::*;
use crate::models::edge::*;
use crate::models::edges::*;


pub fn page(s: Option<Structure>) -> String {
    let title: &'static str = "Agilleen tietokanta";
    html! {
        (header(title))
        h1 { (title) }
        p {(s.unwrap())}
    }.into_string()


}

fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
    }
}


impl Render for Object {
    fn render(&self) -> Markup {
        //let f = |x| x.map(|y| y.to_string()).unwrap_or_else(|| "");
        let (id, desc, form, root) =( self.get_id().map(|y| y.to_string()).unwrap_or_else(|| String::from("-")),
                                      self.get_description().unwrap_or_else(|| String::from("-")),
                                      self.get_form().map(|f| f.to_string()).unwrap_or_else(|| String::from("-")),
                                      self.get_root().map(|y| y.to_string()).unwrap_or_else(|| String::from("-")));
        html! {
            table {
                tr {
                    th { "id" }
                    th { "description" }
                    th { "form" }
                    th { "root" }
                }
                tr {
                    td { ( id ) }
                    td { (desc) }
                    td { (form) }
                    td { (root) }
                }
            }
        }
    }
}

impl Render for Edge {
    fn render(&self) -> Markup {
        let (a,b,a2b,b2a) =(  self.get_a().map(|y| y.to_string()).unwrap_or_else(|| String::from("-")),
                              self.get_b().map(|y| y.to_string()).unwrap_or_else(|| String::from("-")),
                            self.get_a2b().map(|y| y.to_string()).unwrap_or_else(|| String::from("-")),
                            self.get_b2a().map(|y| y.to_string()).unwrap_or_else(|| String::from("-")));
    	html! {
            table {
                tr {
                    th { "a" }
                    th { "b" }
                    th { "a2b" }
                    th { "b2a" }
                }
                tr {
                    td { (a) }
                    td { (b) }
                    td { (a2b) }
                    td { (b2a) }
                }
            }
        }
    }
}


impl Render for Structure {
    fn render(&self) -> Markup {
        html! {
            (self.objects)
            (self.edges)
        }
    }
}



impl Render for Objects {
    fn render(&self) -> Markup {
        html! {
            table {
                @for o in self.get_objects() {
                    tr {
                        td { (o) }
                    }
                }
            }
        }
    }
}

impl Render for Edges {
    fn render(&self) -> Markup {
        html! {
            table {
                @for e in self.get_edges() {
                    tr {
                        td { (e) }
                    }
                }
            }
        }
    }
}

/*
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

*/
