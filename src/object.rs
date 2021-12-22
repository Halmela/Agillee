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

    pub fn get_id(&self) -> Option<i32> {
        self.id
    }

    pub fn get_form(&self) -> Option<Form> {
        self.form.clone()
    }

    pub fn get_form_id(&self) -> Option<i32> {
        self.form.clone().map(|x| x.to_id())
    }

    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
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
    		Some(Form::Void) => "void",
    		None => "formless"
		};
    	
    	write!(f, "id: {}\tdescription: {}\t{}", id, desc, form)
	}
}


#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Form {
    Tangible,
    Intangible,
    Void
}

impl Form {
    pub fn to_id(&self) -> i32 {
        match self {
            Form::Tangible   => 2,
            Form::Intangible => 3,
            Form::Void       => 4,
        }
    }

    pub fn from_id(id: Option<i32>) -> Option<Form> {
        match id {
            Some(2) => Some(Form::Tangible),
            Some(3) => Some(Form::Intangible),
            Some(4) => Some(Form::Void),
            _ => None
        }
    }
}
