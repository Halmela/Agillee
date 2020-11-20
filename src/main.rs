use rand::Rng;
use postgres::{Client, NoTls, Error, Row, Transaction};
use agillee::object::*;
use agillee::table::*;
use agillee::database::*;


fn main() -> Result<(), Error> {
    let mut db = initialize_db()?;
    let mut rng = rand::thread_rng();
	let n: i32 = 100;

	let objs: Vec<Object> = 
    	(1..n)
			.map(|i| match rng.gen_range(1,i+1) {
    			1 => Object { id: Some(i), parent: None },
    			_ => Object {
            			id: Some(i),
            			parent: Some(rng.gen_range(1, i))},
            		})
		.collect();

	let mut transaction = db.client.transaction()?;

	
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
	
	for row in db.client.query("SELECT * FROM objects", &[])? {
		println!("{}",Object::from(row));
	}

//    db.client.execute("DROP TABLE objects;", &[])?;

    Ok(())
}



