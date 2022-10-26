use maud::{ html, DOCTYPE, Markup, Render };

use crate::models::structure::*;
use crate::models::object::*;
use crate::models::objects::*;
use crate::models::edge::*;
use crate::models::edges::*;

use std::fmt::Display;

pub fn index<T: Into<Structure>>(s: T) -> String {
    let s: Structure = s.into();
    page(html! {
        p { "etu sivu" }
        @if s.is_empty() {
            p { "tyhjä tieto kanta" }
        } @else {
            (s)
        }
    })
}

pub fn response<T: Into<Structure>>(s: T) -> String {
    let s: Structure = s.into();
    page(html! {
        .top-left-quarter-circle {}
        @if s.is_empty() {
            p { "ope raatio epä onnistui" }
        } @else {
            (s)
        }
    })
}
       
pub fn page(content: impl Render) -> String {
    let title: &'static str = "Agilleen tietokanta";
    html! {
        link rel="stylesheet" type="text/css" href="/style.css";
        (header(title))
        a href="/" {
        	h1 { (title) }
        }
        (content)
    }.into_string()
}

fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
    }
}

pub fn object_form() -> String {
    page(html! { .object_form {
        form action="/o/new" method="post" {
            .input type="hidden" name="_method" value="put";
            .object_grid {
                    .grid-item { label for="description" { "Description: " } }
                    .grid-item { input type="text" name="description"; }

                    .grid-item { label for="root" { "Root: " } }
                    .grid-item { input type="number" name="root"; }

                label for="formation" { "Form: " }
                .grid-container {
                    .grid-item { label for="tangible" { "Tangible" } }
                    .grid-item { input type="radio" id="tangible" name="form" value="tangible"; }

                    .grid-item { label for="intangible" { "Intangible" } }
                    .grid-item { input type="radio" id="intangible" name="form" value="intangible"; }

                    .grid-item { label for="void" { "Void" } }
                    .grid-item { input type="radio" id="void" name="form" value="void"; }
                }


                .submit-box  {input type="submit" value="Submit";}
            }
        }
    } })
}

fn parse<T: Display>(t: Option<T>) -> String {
    t.map(|x| x.to_string()).unwrap_or_else(|| String::from("-"))
}


impl Render for Object {
    fn render(&self) -> Markup {
        let (id, desc, form, root) =(parse(self.get_id()),   parse(self.get_description()),
                                     parse(self.get_form()), parse(self.get_root()));
        html! { .object {
                    p { ( id ) }
                    p { (desc) }
                    p { (form) }
                    p { (root) }
        }}
    }
}

impl Render for Edge {
    fn render(&self) -> Markup {
        let (a,b,a2b,b2a) =(parse(self.get_a()),   parse(self.get_b()),
                            parse(self.get_a2b()), parse(self.get_b2a()));
    	html! { .edge {
        	/*
                    th { "a" }
                    th { "b" }
                    th { "a2b" }
                    th { "b2a" }
                        */
                    .edge-box { }
                    p { (a2b) }
                    .edge-box { }
                    p { (a) }
                    .edge-box { }
                    p { (b) }
                    .edge-box { }
                    p { (b2a) }
                    .edge-box { }
        }}
    }
}


impl Render for Structure {
    fn render(&self) -> Markup {
        html! { .structure {
            (self.objects)
            (self.edges)
        }}
    }
}



impl Render for Objects {
    fn render(&self) -> Markup {
        if self.get_objects().len() == 1 {
            self.get_objects()[0].render()
        } else {
            html! { .objects {
                h3 { "Objects" }
                .grid-container {
                    .object {
                        b { "id" }
                        b { "description" }
                        b { "form" }
                        b { "root" }
                    }
                    @for o in self.get_objects() {
                        (o)
                    }
                }
            }}
        }
    }
}

impl Render for Edges {
    fn render(&self) -> Markup {
        html! { .edges {
            h3 { "Edges" }
            table {
                @for e in self.get_edges() {
                    (e)
                }
            }
        }}
    }
}
