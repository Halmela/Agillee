use std::fmt;
use rand::Rng;
use std::iter;
use postgres::{Client, NoTls, Error, Row};

fn main() -> Result<(), Error> {
    let mut client = Client::connect("host=localhost port=5432 dbname=agillee user=postgres", NoTls)?;

	client.execute("
    	CREATE TABLE objects (
        	id	    SERIAL PRIMARY KEY,
        	parent  INTEGER REFERENCES objects(id)
    	)
    ;", &[])?;

  
    let mut rng = rand::thread_rng();

	let n: i32 = 100;

	let objs: Vec<Object> = 
    	(1..n)
			.map(|i| match rng.gen_range(1,i+1) {
    			1 => Object { id: i, parent: None },
    			_ => Object {
    			id: i,
    			parent: Some(rng.gen_range(1, i))},
			})
		.collect();


	let mut transaction = client.transaction()?;

	
	for o in objs {
    	println!("{}", o);
    	match o.parent {
        	Some(p) => transaction.execute(
                    	"INSERT INTO objects
                    		(parent) VALUES ($1)
                    	", &[&(p)])?,
        	None    => transaction.execute(
                    	"INSERT INTO objects
                    		(parent) VALUES (NULL)
                    	", &[])?
    	};

	}

	transaction.commit()?;
	
	for row in client.query("SELECT * FROM objects", &[])? {
		println!("{}",Object::from(row));
	}

    client.execute("DROP TABLE objects;", &[])?;

    Ok(())
}


struct Object {
    id: i32,
    parent: Option<i32>
}

impl From<Row> for Object {
    fn from(row: Row) -> Self {
		Object {
    		id: row.get(0),
    		parent: row.get(1)
		}
    }
}


impl fmt::Display for Object{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	let parent = match self.parent {
        	Some(id) => id.to_string(),
        	None	 => String::from("None")
    	};
    	
    	write!(f, "id: {}\t parent: {}", self.id, parent)
	}
}
