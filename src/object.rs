use postgres::Row;
use std::fmt;


pub struct Object {
    pub id: Option<i32>,
    pub parent: Option<i32>
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
    	let id = match self.id {
        	Some(id) => id.to_string(),
        	None	 => String::from("None")
    	};
    	let parent = match self.parent {
        	Some(id) => id.to_string(),
        	None	 => String::from("None")
    	};
    	
    	write!(f, "id: {}\t parent: {}", id, parent)
	}
}

