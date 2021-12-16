use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Object {
    pub id: Option<i32>,
    pub description: Option<String>
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

impl fmt::Display for Object {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	let id = self.id.unwrap_or_default();
    	let desc = match &self.description {
        	Some(s) => s,
        	None    => ""
	};
    	
    	write!(f, "id: {}\tdescription: {}", id, desc)
	}
}
