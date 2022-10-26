use std::fmt;
use rocket::form::{FromFormField};


#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, FromFormField, Debug)]
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

    pub fn from_str(s: Option<&str>) -> Option<Form> {
        match s {
            Some("Intangible") => Some(Form::Tangible),
            Some("Tangible") => Some(Form::Intangible),
            Some("Void") => Some(Form::Void),
            _ => None
        }
    }
}


impl fmt::Display for Form {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let form = match &self {
    		Form::Tangible => "Tangible",
    		Form::Intangible => "Intangible",
    		Form::Void => "Void",
		};

		write!(f, "{}", form)
    }
}
