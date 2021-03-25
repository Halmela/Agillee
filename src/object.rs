use postgres::{Error};
use std::fmt;
use std::fmt::Write;
use crate::database::Database;
use std::collections::{HashMap, HashSet};


pub struct Object {
    pub id: Option<i32>,
    pub description: Option<String>
}


pub struct Objects {
	pub objects:   HashMap<i32, Object>,
    pub relations: HashSet< (i32, i32) >,
    database:	   Database
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
    		relations: HashSet::new(),
    		database:  db
		}
    }

/*
	fn insert_relations(&mut self, start: &i32, end: &i32, rel: Relation) -> Result<(), Error> {
    	self.database.client.
    	Ok(())
	}
*/

	pub fn insert_objects(&mut self, objects: Vec<Object>) -> Result<(), Error> {
    	let mut transaction = self.database.client.transaction()?;
    	let statement = transaction.prepare("INSERT INTO objects (description) VALUES ($1)")?;

    	for o in objects {
        	match o.id {
            	Some(_) => {0},
            	None     => {transaction.execute(&statement, &[&o.description])?}
        	};
    	}

    	
    	Ok(transaction.commit()?)
	}

	pub fn insert_relations(&mut self, relations: Vec<(i32, i32)>) -> Result<(), Error> {
    	let mut transaction = self.database.client.transaction()?;
    	let statement = transaction.prepare("INSERT INTO relations (start, sink) VALUES ($1, $2)")?;

    	for (start, sink) in relations {
            transaction.execute(&statement, &[&start, &sink])?;
    	}

        Ok(transaction.commit()?)
	}

	pub fn add_relations(&mut self, id: &i32, rel: Relation) -> Result<(), Error> {
		let res = self.query_relations(id, rel)?;
		let mut to_query = vec!();

		for r in res {
    		if !self.objects.contains_key(&r.0) {
        		to_query.push(*&r.0);
    		}
    		
    		if !self.objects.contains_key(&r.1) {
        		to_query.push(*&r.1);
    		}

    		self.relations.insert(r);
		}

		self.query_objects(to_query)?;

		Ok(())
	}

	fn query_objects(&mut self, ids: Vec<i32>) -> Result<(), Error> {
    	let mut transaction = self.database.client.transaction()?;
    	let statement = transaction.prepare("SELECT id, description FROM objects WHERE id = $1")?;
    	//let mut objs: Vec<(i32, String)> = vec!();

		for id in ids {
    		let obj = transaction.query_one(&statement, &[&id])?;
    		self.objects.insert(id, Object {
    			id: obj.get("id"),
    			description: obj.get("description") });
		}
		
    	Ok(transaction.commit()?)
	}
    
    fn query_relations(&mut self, id: &i32, rel: Relation) -> Result<Vec<(i32, i32)>, Error> {
        match rel {
            Relation::Any => Ok(self.database.client.query(
                    "SELECT start, sink
                     FROM relations
                     WHERE (start = $1) OR (sink = $1);", &[id])?.iter()
                        .map(|r| (r.get("start"), r.get("sink")))
                        .collect()),
            Relation::In => Ok(self.database.client.query(
                    "SELECT start, sink
                     FROM relations
                     WHERE (start = $1) AND sink NOT IN (
                         SELECT start FROM relations WHERE (sink = $1));", &[id])?.iter()
                        .map(|r| (r.get("start"), r.get("sink")))
                        .collect()),
            Relation::Out => Ok(self.database.client.query(
                    "SELECT start, sink
                     FROM relations
                     WHERE (sink = $1) AND start NOT IN (
                         SELECT sink FROM relations WHERE (start = $1));", &[id])?.iter()
                        .map(|r| (r.get("start"), r.get("sink")))
                        .collect()),
            _  => Ok(vec!())
        }
    }

	pub fn drop(&mut self) -> Result<(), Error> {
    	Ok(self.database.drop_tables()?)
	}
}


pub enum Relation {
	In,
	Out,
	Both,
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
        for (_, o) in &self.objects {
        	write!(&mut res, "{}\n", o.to_string()).unwrap();
        }

		write!(&mut res, "start\tsink\n")?;
        for (start, sink) in &self.relations {
            write!(&mut res, "{:<7} {}\n", start, sink)?;
        }
        write!(f, "{}", res)
	}
}
