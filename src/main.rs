use std::fmt;
use rand::Rng;
use std::iter;

fn main() {
    let mut rng = rand::thread_rng();

	let n: u32 = 100;

	iter::once(
    		Object {id: 0, parent: None})
    	.chain(
    	(0..n)
			.map(|i| match rng.gen_range(0,i+1) {
    			0 => Object { id: i, parent: None },
    			_ => Object {
    			id: i,
    			parent: Some(rng.gen_range(0, i))},
			}))
		.for_each(|o| println!("{}", o));

}


#[derive(Debug)]
struct Object {
    id: u32,
    parent: Option<u32>
}
/*
impl Object {
    fn ancestors() -> Vec<Object> {

    }
}
*/

impl fmt::Display for Object{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	let parent = match self.parent {
        	Some(id) => id.to_string(),
        	None	 => String::from("None")
    	};
    	
    	write!(f, "id: {}\t parent: {}", self.id, parent)
	}
}
