use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Object {
    pub id: Option<i32>,
    pub description: Option<String>,
    pub form: Option<Form>
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}


impl Object {
    pub fn new(id: Option<&i32>, desc: Option<String>, form: Option<Form>) -> Object {
		match id {
    		Some(id) =>
        		Object {
            		id: Some(*id),
            		description: desc,
            		form: form
        		},
        	None     =>
        		Object {
            		id: None,
            		description: desc,
            		form: form
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
		let form = match &self.form {
    		Some(Form::Tangible) => "tangible",
    		Some(Form::Intangible) => "intangible",
    		None => "formless"
		};
    	
    	write!(f, "id: {}\tdescription: {}\t{}", id, desc, form)
	}
}


#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Form {
    Tangible,
    Intangible
}
