use crate::object::*;
use crate::edge::*;
use crate::commander::*;
use crate::structure::*;
use std::io;
use postgres::{Error};
use itertools::Itertools;


pub struct CLI {
    commander: Commander
}

impl CLI {
    pub fn new(commander: Commander) -> CLI {
        CLI {commander}
    }

    pub fn start(&mut self) -> Result<(), Error> {

        /*
        let o = self.objects.add_object(
            Object::new(None,Some("test".to_string()),Some(Form::Tangible),Some(1))
        )?;
        
        //println!("{}", o.unwrap().0);
        let t_o = Object::new(None,None,None,None);
        self.objects.temp_q(t_o)?;
        */

        //help();
        //self.objects.get_all_objects()?;
        //self.objects.get_all_relations()?;

        
        match self.main_loop() {
			Err(e) => {
    			println!("operation failed with error:\n{}", e);
    			self.main_loop()?
			},
			_ => println!("goodbye")
        }

        
        
        //self.objects.drop()?;
        Ok(())
    }

    fn main_loop(&mut self) -> Result<(), Error> {
        loop {
            println!("Gibe command: ");
            let mut cmd = String::new();
            io::stdin()
                .read_line(&mut cmd)
                .expect("Failed to read line");

			match cmd.trim() {
    			"ao" => self.add_object()?,
    			"q" => self.query()?,
    			//"ar" => self.add_relation()?,
        		//"p" => println!("{}", self.objects),
        		"h" => help(),
        		"x" => { return Ok(()) },
    			x => { println!("{} is not a valid command",x); }
			}
        }
    }

    fn query(&mut self) -> Result<(), Error> {
        let st = self.commander.execute(
            Command::Read(
                Structure::new(
                    Some(vec!(Object::new(None, None, None, None))),
                	Some(vec!(Edge::new(None, None, None, None))))
            )
        )?;
        println!("{}", st);
        Ok(())
    }

    fn query_object(&mut self) -> Result<(), Error> {
        let st = self.commander.execute(
            Command::ReadObject(
                Object::new(None, None, None, None)))?;
        println!("{}", st);
        Ok(())
    }

    fn add_object(&mut self) -> Result<(), Error> {
        if let (Some(desc), Some(form), Some(root)) = (ask_description(), ask_form(), ask_root()) {
            let st =self.commander
                .execute(Command::CreateObject(
                    Object {
                        id: None,
                        description: Some(desc),
                        form: Some(form),
                        root: Some(root)
                }))?;
            println!("{}",st);
        }

        
        Ok(())
    }
}

fn ask_description() -> Option<String> {
	println!("Give description for the object: (empty for abort)");
	let mut desc = String::new();
    io::stdin()
        .read_line(&mut desc)
        .ok()
        .expect("Failed to read line");

    match desc.trim() {
        "" => None,
        s => Some(s.to_string()),
    }
}

fn ask_root() -> Option<i32> {
	println!("default (r)oot or custom id? (empty for abort)");
	let mut root = String::new();
    io::stdin()
        .read_line(&mut root)
        .ok()
        .expect("Failed to read line");

    match root.trim() {
        "" => None,
        "r" => Some(1),
        s => match s.parse() {
            Ok(x) => Some(x),
            Err(_) =>{ println!("give valid answer"); ask_root() }
        }
    }
}

fn ask_form() -> Option<Form> {
	println!("Based on what? (t)angible, (i)ntangible or (v)oid? empty for abort");
	let mut q = String::new();
    io::stdin()
        .read_line(&mut q)
        .ok()
        .expect("Failed to read line");

	match q.trim() {
		"i" => Some(Form::Intangible),
		"t" => Some(Form::Tangible),
		"v" => Some(Form::Void),
		""  => None,
		_   => { println!("give valid answer"); ask_form() }
	}
}


/*
fn parse_relations(s: &str) -> Option<Vec<(i32, i32, Relation)>> {
    let v: Vec<&str> = s.split(&[' ','-'][..]).collect();
    if let Some((a,r,b)) = v.iter().collect_tuple() {
        let a: i32 = a.parse().ok()?;
        let b: i32 = b.parse().ok()?;
        let mut res = vec!();

        match *r {
            "X" => { res.push((a,b,Relation::Both))   },
            "|" => { res.push((a,b,Relation::Closed)) },
            "~" => { res.push((a,b,Relation::Empty))  },
            ">" => { res.push((a,b,Relation::Start(Some(true))))  },
            "<" => { res.push((a,b,Relation::Sink(Some(true))))  },
             x  => { if let Some((b2a, a2b)) = x.chars().collect_tuple() {
                    match b2a {
                        '<' => res.push((a,b,Relation::Sink(Some(true)))),
                        '|' => res.push((a,b,Relation::Sink(Some(false)))),
                        '~' => res.push((a,b,Relation::Sink(None))),
                         _  => {}
                    }

                    match a2b {
                        '>' => res.push((a,b,Relation::Start(Some(true)))),
                        '|' => res.push((a,b,Relation::Start(Some(false)))),
                        '~' => res.push((a,b,Relation::Start(None))),
                         _  => {}
                    } 
                } else { return None }
            }
        }

        Some(res)

    } else {
        None
    }
}
*/

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

fn relation_help() {
    let help: &'static str =
        "a [s] b\twhere\n\
        	\ta, b are ids of objects\n\
        	\ts is one or two of following symbols:\n\
            	\t\t>\t from a to b\n\
                \t\t<\t from b to a\n\
            \tif there are two symbols, first one corresponds to the relation from b to a,\n\
            \tand second one to the relation from a to b\n\
            \t so a |> b is allowed, but a |< b is not\n\
            \tfor convenience you can use \n\
                \t\t>\t same as ~>\n\
                \t\t<\t same as <~\n\
        		\t\tX\t same as <>\n\
        		\t\t|\t same as ||\n\
        		\t\t~\t same as ~~\n\
            this will overwrite any previous relation";
    println!("{}", help);
}

/*
    	\t\t \n\
*/

