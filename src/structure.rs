use crate::object::*;
use crate::objects::*;
use crate::edge::*;
use crate::edges::*;
use std::fmt;
use std::fmt::Write;
use std::convert::From;

#[derive(Default)]
pub struct Structure {
    objects: Objects,
    edges:   Edges
}

impl From<Object> for Structure {
    fn from(item: Object) -> Self {
        Structure {
            objects: Objects::from(item),
            edges: Edges::empty()
        }
    }
}

impl From<i32> for Structure {
    fn from(item: i32) -> Self {
        Structure {
            objects: Objects::from(Object::from(item)),
            edges: Edges::from(Edge::from(item))
        }
    }
}

impl fmt::Display for Structure {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	let mut res = String::new();
    	write!(&mut res, "{}\n", self.objects)?;
    	write!(&mut res, "{}\n", self.edges)?;

    	write!(f, "{}", res)
	}
}

impl Structure {
    pub fn new(os: Option<Vec<Object>>, es: Option<Vec<Edge>>) -> Structure {
        Structure {
            objects: Objects::from_vec(os.unwrap_or_default()),
			edges:   Edges::from_vec(es.unwrap_or_default())
        }
    }

    pub fn blank() -> Structure {
        Structure {
            objects: Objects::blank(),
            edges:   Edges::blank()
        }
    }

    pub fn from_structs(objects: Objects, edges: Edges) -> Structure {
        Structure {
            objects,
            edges
        }
    }

    pub fn empty() -> Structure {
        Structure {
            objects: Objects::empty(),
            edges:   Edges::empty()
        }
    }

    pub fn merge(self, other: Structure) -> Structure {
        Structure {
            objects: self.objects.merge(&other.objects),
            edges: self.edges.merge(&other.edges)
        }
    }

    pub fn get_objects(&self) -> Vec<Object> {
        self.objects.get_objects()
    }

    pub fn get_edges(&self) -> Vec<Edge> {
        self.edges.get_edges()
    }
}
