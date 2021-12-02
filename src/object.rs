use postgres::{Error};
use std::fmt;
use std::fmt::Write;
use crate::database::Database;
use std::collections::{HashMap};


pub struct Object {
    pub id: Option<i32>,
    pub description: Option<String>
}


pub struct Objects {
	pub objects:   HashMap<i32, Object>,
    pub relations: HashMap<(i32, i32), (Option<bool>, Option<bool>)>,
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

	pub fn insert_objects(&mut self, objects: Vec<Object>) -> Result<(), Error> {
    	let mut transaction = self.database.client.transaction()?;
    	let statement = transaction.prepare("INSERT INTO Objects (description) VALUES ($1);")?;

    	for o in objects {
        	match o.id {
            	Some(_) => {0},
            	None     => {transaction.execute(&statement, &[&o.description])?}
        	};
    	}

    	
    	Ok(transaction.commit()?)
	}

	pub fn insert_relations(&mut self, relations: Vec<((i32, i32), (Option<bool>, Option<bool>))>) -> Result<(), Error> {
    	let mut transaction = self.database.client.transaction()?;
    	let stmnt = transaction.prepare(
        	"
            	INSERT INTO Relations AS R (a, b, a2b, b2a)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (a, b) DO UPDATE
                	SET a2b = COALESCE(EXCLUDED.a2b,FALSE) OR R.a2b,
                    	b2a = COALESCE(EXCLUDED.b2a,FALSE) OR R.b2a
            ;")?;

    	for ((a, b), (a2b, b2a)) in relations {
        	if a < b {
                transaction.execute(&stmnt, &[&a, &b, &a2b, &b2a])?;
        	} else {
                transaction.execute(&stmnt, &[&b, &a, &b2a, &a2b])?;
        	}
    	}

        Ok(transaction.commit()?)
	}

	pub fn add_relations(&mut self, id: &i32, rel: Relation) -> Result<(), Error> {
		let res = self.query_relations(id, rel)?;
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

		self.query_objects(to_query)?;

		Ok(())
	}

	fn query_objects(&mut self, ids: Vec<i32>) -> Result<(), Error> {
    	let mut transaction = self.database.client.transaction()?;
    	let statement = transaction.prepare("SELECT id, description FROM Objects WHERE id = $1;")?;
    	//let mut objs: Vec<(i32, String)> = vec!();

		for id in ids {
    		let obj = transaction.query_one(&statement, &[&id])?;
    		self.objects.insert(id, Object {
    			id: obj.get("id"),
    			description: obj.get("description") });
		}
		
    	Ok(transaction.commit()?)
	}
    
    fn query_relations(&mut self, id: &i32, rel: Relation) -> Result<Vec<((i32, i32), (Option<bool>, Option<bool>))>, Error> {
        let statement = self.database.client.prepare(
            match rel {
                Relation::Any => 
                        "SELECT a, b, a2b, b2a
                         FROM Relations
                         WHERE ((a = $1) OR (b = $1)) AND (a2b OR b2a);",
                Relation::Start => 
                        "SELECT a, b, a2b, b2a
                         FROM Relations
                         WHERE (a = $1 AND a2b) OR (b = $1 AND b2a);",
                Relation::Sink =>
                        "SELECT a, b, a2b, b2a
                         FROM Relations
                         WHERE (a = $1 AND b2a) OR (a = $1 AND a2b);",
                Relation::Both =>
                    	"SELECT a, b, a2b, b2a
                         FROM Relations
                         WHERE (a = $1 AND a2b AND b2a) OR (b = $1 AND a2b AND b2a);"
        })?;
		Ok(
    		self.database.client.query(&statement, &[id])?.iter()
                .map(|r| ((r.get("a"), r.get("b")), (r.get("a2b"), r.get("b2a"))))
                .collect()
		)

    }


	pub fn drop(&mut self) -> Result<(), Error> {
    	Ok(self.database.drop_tables()?)
	}
}


pub enum Relation {
	Start,
	Sink,
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
    		let c = |b| match b {
        		Some(true) => '>',
        		Some(false) => '<',
        		None => '-'
    		};

        	write!(&mut res, "a       b\n")?;

            for ((a, b), (a2b, b2a)) in &self.relations {
                write!(&mut res, "{:<5} {}-{} {}\n",a,b, c(*a2b), c(*b2a))?;
            }
		}
        write!(f, "{}", res)
	}
}
