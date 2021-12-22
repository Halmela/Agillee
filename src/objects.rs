use postgres::{Error};
use std::fmt;
use std::fmt::Write;
use std::collections::{HashMap};
use itertools::Itertools;
use crate::database::Database;
use crate::object::Object;
use crate::edge::*;


pub struct Objects {
	pub objects:   HashMap<i32, Object>,
    pub relations: HashMap<(i32, i32), (Option<bool>, Option<bool>)>,
    database:	   Database
}

impl Objects {
    pub fn new(db: Database) -> Result<Objects, Error> {
		Ok(Objects {
    		objects:   HashMap::new(),
    		relations: HashMap::new(),
    		database:  db
		})
    }

/*
	fn insert_relations(&mut self, a: &i32, end: &i32, rel: Relation) -> Result<(), Error> {
    	self.database.client.
    	Ok(())
	}
*/

	pub fn add_relations(&mut self, rels: Vec<((i32, i32), (Option<bool>, Option<bool>))>) -> Result<(), Error> {
    	Ok(self.database.insert_relations(rels)?)
	}

	pub fn add_relation(&mut self, a: i32, b: i32, rel: Relation) -> Result<(), Error> {
    	self.database.insert_relation(a.clone(), b.clone(), rel)?;
    	self.get_relations(&a, Relation::Any)?;
    	self.get_relations(&b, Relation::Any)?;
    	Ok(())
	}


	/*
     * Fetch relations matching to a given 
     */
	pub fn get_relations(&mut self, id: &i32, rel: Relation) -> Result<(), Error> {
		let res = self.database.query_relations(id, rel)?;
		let mut to_query = vec!();

		for (s,r) in res {
    		if !self.objects.contains_key(&s.0) {
        		to_query.push(*&s.0);
    		}
    		
    		if !self.objects.contains_key(&s.1) {
        		to_query.push(*&s.1);
    		}

    		self.relations.insert(s, r);
		}

		for o in self.database.query_objects(to_query)? {
    		if let Some(id) = o.id {
        		self.objects.insert(id, o);
    		}
		}

		Ok(())
	}

	pub fn get_all_objects(&mut self) -> Result<(), Error> {
    	for o in self.database.query_all_objects()? {
    		match o.id {
        		Some(id) => {self.objects.insert(id, o);},
        		_        => {}
    		}
    	}

    	Ok(())
	}

	pub fn get_all_relations(&mut self) -> Result<(), Error> {
    	for (o,r) in self.database.query_all_relations()? {
        	self.relations.insert(o,r);
    	}

    	Ok(())
	}


	/*
     * Add objects to self and insert them to database.
     */
	pub fn add_objects(&mut self, objs: Vec<Object>) -> Result<(Vec<i32>), Error> {
		for o in &objs {
    		match o.id {
        		Some(id) => {self.objects.insert(id, o.clone());},
        		_        => {}
    		}
		}

		let mut ids = vec!();
    	for o in self.database.insert_objects(objs)? {
        	match o.id {
            	Some(id) => {ids.push(id); self.objects.insert(id, o);},
            	_        => {}
        	}
    	}
    	Ok(ids)
	}

	pub fn add_object(&mut self, obj: Object) -> Result<Option<(Object, Edge)>, Error> {
    	self.database.create_object(obj)
	}

	pub fn temp_q(&mut self, obj: Object) -> Result<(), Error> {
    	for o in self.database.query_with_object(obj)? {
        	println!("{}", o);
    	}
    	Ok(())
	}

	
	pub fn drop(&mut self) -> Result<(), Error> {
    	Ok(self.database.drop_tables()?)
	}
}

impl fmt::Display for Objects {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	let mut res = String::new();


    	if self.objects.is_empty() {
        	write!(&mut res, "No objects\n").unwrap();
    	} else {
        	for (_,o) in self.objects.iter().sorted() {
            	write!(&mut res, "{}\n", o.to_string()).unwrap();
        	}

    	}

		if self.relations.is_empty() {
        	write!(&mut res, "No relations\n").unwrap();
		} else {
    		let c = |o,b| match (o,b) {
        		(Some(true),false) => '>',
        		(Some(true),true) => '<',
        		(Some(false),_) => '|',
        		_ => '~'
    		};

        	write!(&mut res, "a         b\n")?;

            for ((a, b), (a2b, b2a)) in self.relations.iter().sorted() {
                write!(&mut res, "{:<4} {}-{}  {}\n",a, c(*b2a,true), c(*a2b,false),b)?;
            }
		}
        write!(f, "{}", res)
	}
}


#[derive(Debug)]
pub enum Relation {
	Start(Option<bool>),
	Sink(Option<bool>),
	Both,
	OneWay,
	Closed,
	Any,
	Empty
}

