use std::fmt;
use rand::Rng;
use postgres::{Client, NoTls, Error, Row, Transaction};


fn build_client() -> Result<Client, Error> {
	let client = Client::connect("host=localhost port=5432 dbname=agillee user=postgres", NoTls)?;
	Ok(initialize_db(client)?)
}


fn initialize_db(mut client: Client) -> Result<Client, Error> {
    let db = Database {
        tables: vec!(Table::Object) };

	Ok(db.add_tables(client)?)
}

struct Database {
	tables: Vec<Table>
}

impl Database {
	fn add_tables(&self, mut client: Client) -> Result<Client, Error> {
    	
		for table in &self.tables {
    		let mut transaction = client.transaction()?;
    		Database::add_scheme(table_to_scheme(table), transaction)?;
		}

		Ok(client)
	}

	fn add_scheme(scheme: &str, mut transaction: Transaction) -> Result<(), Error> {
        let res =transaction.execute(scheme, &[]);
		transaction.commit()?;
    	match res {
            Err(e) => match e.code().unwrap().code() {
                "42P07" => Ok(()),
                _	    => Err(e)},
            Ok(_) 		=> {
                Ok(())
            }
        }
	}
}

fn table_to_scheme(table: &Table) -> &'static str {
	match table {
    	Object => 
            "CREATE TABLE objects (
            	id	    SERIAL PRIMARY KEY,
            	parent  INTEGER REFERENCES objects(id)
        	);",
		_ 	   => "CREATE TABLE empty();"
	}
}


fn main() -> Result<(), Error> {
    let mut client = build_client()?;
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

//    client.execute("DROP TABLE objects;", &[])?;

    Ok(())
}


struct Object {
    id: i32,
    parent: Option<i32>
}


enum Table {
    Object
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
