use postgres::{Error};
use std::fmt;
use std::fmt::Write;
use crate::database::Database;
use std::collections::{HashMap};
use std::hash::{Hash, Hasher};


pub struct Object {
    pub id: Option<i32>,
    pub description: Option<String>
}


pub struct Objects {
	pub objects:   HashMap<i32, Object>,
    pub relations: HashMap<(i32, i32), (Option<bool>, Option<bool>)>,
    database:	   Database
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Object {
    pub fn new(id: Option<&i32>, desc: Option<String>) -> Object {
		match id {
    		Some(id) =>
        		Object {
            		id: Some(*id),
            		description: desc
        		},
        	None     =>
        		Object {
            		id: None,
            		description: desc
        		}
		}
    }
}

impl Objects {
    pub fn new(db: Database) -> Objects {
		Objects {
    		objects:   HashMap::new(),
    		relations: HashMap::new(),
    		database:  db
		}
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
    		match o.id {
        		Some(id) => {self.objects.insert(id, o);},
        		_        => {}
    		}
		}

		Ok(())
	}


	/*
     * Add objects to self and insert them to database.
     */
	pub fn add_objects(&mut self, objs: Vec<Object>) -> Result<(), Error> {
    	/*
		for o in objs {
    		match o.id {
        		Some(id) => {self.objects.insert(id, o);},
        		_        => {}
    		}
		}
		*/
    	self.database.insert_objects(objs)?;
    	Ok(())
	}

	
	pub fn drop(&mut self) -> Result<(), Error> {
    	Ok(self.database.drop_tables()?)
	}
}


pub enum Relation {
	Start,
	Sink,
	Both,
	OneWay,
	Closed,
	Any
}


/*
impl From<Row> for Object {
    fn from(row: Row) -> Self {
		Object {
    		id: row.get(0),
    		relations: row.get(1)
		}
    }
}
*/


impl fmt::Display for Object {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	let id = self.id.unwrap_or_default();
    	
    	write!(f, "id: {}", id)
	}
}


impl fmt::Display for Objects {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	let mut res = String::new();

    	if self.objects.is_empty() {
        	write!(&mut res, "No objects\n").unwrap();
    	} else {
            for (_, o) in &self.objects {
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
        		_ => '-'
    		};

        	write!(&mut res, "a       b\n")?;

            for ((a, b), (a2b, b2a)) in &self.relations {
                write!(&mut res, "{:<4} {}-{}  {}\n",a, c(*b2a,true), c(*a2b,false),b)?;
            }
		}
        write!(f, "{}", res)
	}
}
