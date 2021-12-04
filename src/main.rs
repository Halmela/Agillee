use rand::Rng;
use postgres::{Error};
use agillee::object::*;
//use agillee::table::*;
use agillee::database::*;


fn main() -> Result<(), Error> {
    let db = initialize_db()?;
    let mut objs = Objects::new(db);
    let mut rng = rand::thread_rng();

	let n = 100;
	let os = (0..n).map(|_| Object::new(None, None)).collect();
	let rels = (0..n).map(|_| (
        	 (rng.gen_range(1,n),
             rng.gen_range(1,n)),
             (if rng.gen() {Some(rng.gen())} else {None},
             if rng.gen() {Some(rng.gen())} else {None})
	)).collect();

    println!("gen ready");
    objs.add_objects(os)?;
    println!("objs inserted");
    objs.add_relations(rels)?;
    println!("rels inserted");
   
    for i in 0..n {
        objs.get_relations(&i, Relation::Closed)?;
    }
    println!("{}", &objs);

    Ok(())
}


