use std::fmt;
use crate::object::*;

#[derive(Clone)]
pub struct Edge {
    pub a:   Option<i32>,
    pub b:   Option<i32>,
    pub a2b: Option<i32>,
    pub b2a: Option<i32>,
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

	pub fn root(object: &Option<Object>) -> Option<Edge> {
    	if let Some((Some(r), Some(o))) = object.as_ref().map(|o| (o.root, o.id)) {
        	Some(Edge::new(
            	Some(r),
                Some(o),
                Some(r),
                Some(Form::Void.to_id())))
    	} else { None }
	}
}


impl fmt::Display for Edge {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	write!(f, "{:?} {:?} {:?} {:?}", self.a, self.b, self.a2b, self.b2a)
	}
}

