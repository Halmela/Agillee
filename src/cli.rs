use crate::object::*;
use crate::objects::*;
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
        help();
        loop {
            println!("Gibe command: ");
            let mut cmd = String::new();
            io::stdin()
                .read_line(&mut cmd)
                .expect("Failed to read line");

			match cmd.as_str().trim() {
    			"ao" => self.add_object()?,
        		"p" => println!("{}", self.objects),
        		"h" => help(),
        		"x" => {self.objects.drop()?; return Ok(())},
    			x => {println!("{} is not a valid command",x); }
			}
        }
    }

    fn add_object(&mut self) -> Result<(), Error> {
		println!("Give description for the object:");
		let mut desc = String::new();
        io::stdin()
            .read_line(&mut desc)
            .ok()
            .expect("Failed to read line");
        self.objects.add_objects(
            vec!(
                Object {id: None, description: Some(desc)}))?;
        Ok(())
    }
}

fn help() {
    let help: &'static str = "Available commands:\n\
		\tao\t add object database\n\
		\tar\t add relation database\n\
		\tdo\t delete object\n\
		\tdr\t delete relation\n\
		\tp\t print objects\n\
    	\tuo\t update object\n\
    	\tro\t update relation\n\
    	\t\t \n\
    	\tx\t exit\n\
    	\th\t print this message";
	println!("{}", help);
}
/*
    	\t\t \n\
*/
