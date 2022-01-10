use std::fmt;
use crate::object::*;
use std::hash::{Hash, Hasher};
use std::convert::From;
use std::default::Default;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct Edge {
    pub a:   Option<i32>,
    pub b:   Option<i32>,
    pub a2b: Option<i32>,
    pub b2a: Option<i32>,
}


impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.b.hash(state);
        self.a2b.hash(state);
        self.a2b.hash(state);
    }
}


impl Edge {
	pub fn new(a: Option<i32>,b: Option<i32>,a2b: Option<i32>,b2a: Option<i32>) -> Edge {
    	match (a, b)  {
        	(Some(x), Some(y)) if x < y => Edge {a,b,a2b,b2a},
        	(Some(_), Some(_))          => Edge {a:b,b:a,a2b:b2a,b2a:a2b},
            (None, Some(_))             => Edge {a:b,b:a,a2b:b2a,b2a:a2b},
        	_                           => Edge {a,b,a2b,b2a}
    	}
	}

	pub fn blank() -> Edge {
    	Edge {
			a:   None,
			b:   None,
			a2b: None,
			b2a: None }
	}

	pub fn root(object: &Option<Object>) -> Option<Edge> {
    	if let Some((Some(r), Some(o))) = object.as_ref().map(|o| (o.root, o.id)) {
        	Some(Edge::new(
            	Some(r),
                Some(o),
                Some(r),
                Some(Form::Void.to_id())))
    	} else { None }
	}

	pub fn get_a(&self) -> Option<i32> {
    	self.a
	}
   	
	pub fn get_b(&self) -> Option<i32> {
    	self.b
	}

	pub fn get_a2b(&self) -> Option<i32> {
    	self.a2b
	}

	pub fn get_b2a(&self) -> Option<i32> {
    	self.b2a
	}
}

impl From<i32> for Edge {
    fn from(item: i32) -> Self {
        Edge { a: Some(item), b: None, a2b: None, b2a: None }
    }
}


/*
impl fmt::Display for Edge {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	write!(f, "{:?} {:?} {:?} {:?}", self.a, self.b, self.a2b, self.b2a)
	}
}
*/

