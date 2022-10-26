use std::hash::{Hash, Hasher};
use std::convert::From;
use std::collections::HashMap;
use crate::data::table::Table;
use crate::models::form::*;
use rocket::form::{FromForm};

pub struct Mabject<T> {
    pub id: Option<i32>,
    pub attributes: HashMap<Table, Option<Box<T>>>
}

impl<T> Mabject<T> {
    pub fn new(id: Option<i32>, attributes: HashMap<Table, Option<Box<T>>>) -> Mabject<T> {
        Mabject {
            id,
            attributes
        }
    }
}


impl<T> From<i32> for Mabject<T> {
    fn from(item: i32) -> Self {
        Mabject {
            id: Some(item),
            attributes: HashMap::new()
        }
    }
}



#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, FromForm, Debug)]
pub struct Object {
    pub id: Option<i32>,
    pub description: Option<String>,
    pub form: Option<Form>,
    pub root: Option<i32>
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}


impl Object {
    pub fn new(id: Option<&i32>, desc: Option<String>, form: Option<Form>, root: Option<i32>) -> Object {
        Object {
            id: id.map(|i| *i),
    		description: desc,
    		form: form,
    		root: root

        }
    }

	pub fn blank() -> Object {
    	Object {
			id:   None,
			description:   None,
			form: None,
			root: None
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

    pub fn get_root(&self) -> Option<i32> {
        self.root.clone()
    }
}

impl From<i32> for Object {
    fn from(item: i32) -> Self {
        Object {
            id: Some(item),
            description: None,
            form: None,
            root: None
        }
    }
}

/*
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
		let root = match &self.root {
    		Some(i) => format!("{}", i),
    		None    => "rootless".to_string()
		};
    	
    	write!(f, "id: {}\tdescription: {}\t{}\t{}", id, desc, form, root)
	}
}
*/

