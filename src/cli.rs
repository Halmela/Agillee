use crate::object::*;
use std::io;
use postgres::{Error};

pub struct CLI {
    objects: Objects
}

impl CLI {
    pub fn new(objects: Objects) -> CLI {
        CLI {objects}
    }

    pub fn start(&mut self) -> Result<(), Error> {
        loop {
            println!("Gibe command: ");
            let mut cmd = String::new();
            io::stdin()
                .read_line(&mut cmd)
                .expect("Failed to read line");

			match cmd.as_str().trim() {
    			"1" => {
        			println!("Give description for the object:");
        			let mut desc = String::new();
                    io::stdin()
                        .read_line(&mut desc)
                        .ok()
                        .expect("Failed to read line");
                    self.objects.add_objects(
                        vec!(
                            Object {id: None, description: Some(desc)}))?;
        		},
        		"p" => {
            		{println!("{}", self.objects);}},
    			x => {println!("-{}-",x); self.objects.drop()?; return Ok(())}
			}
        }
    }
}
