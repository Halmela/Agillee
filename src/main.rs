use rand::Rng;
use postgres::{Error};
use agillee::object::*;
//use agillee::table::*;
use agillee::database::*;


fn main() -> Result<(), Error> {
    let db = initialize_db()?;
    let mut objs = Objects::new(db);
    let mut rng = rand::thread_rng();

	let n = 1000;
	let os = (0..n).map(|_| Object::new(None, None)).collect();
	let rels = (0..n).map(|_| (
        	 rng.gen_range(1,n),
             rng.gen_range(1,n),
             if rng.gen() {Some(rng.gen())} else {None},
             if rng.gen() {Some(rng.gen())} else {None},
	)).collect();

    println!("gen ready");
    objs.insert_objects(os)?;
    println!("objs inserted");
    objs.insert_relations(rels)?;
    println!("rels inserted");

	//println!("{:?}", objs.query_relations(&1, Relation::Any));
    
    println!("ins added");
    for i in 0..n {
        objs.add_relations(&i, Relation::Any)?;
    }
    println!("{}", &objs);

    Ok(())
    //Ok(objs.drop()?)
}

/*
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

    db.client.execute("DROP TABLE objects;", &[])?;

    Ok(())
}


*/
