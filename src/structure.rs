use crate::object::*;
use crate::edge::*;
use std::collections::{HashSet};
use std::fmt;
use std::fmt::Write;
use itertools::Itertools;


struct Objects_n {
    objects: HashSet<Object>
}

impl Objects_n {
    pub fn from_vec(v: Vec<Object>) -> Objects_n {
        let mut hs = HashSet::new();
        for o in v {
            hs.insert(o);
        }

        Objects_n {objects: hs}
    }

    pub fn merge(self, other: Objects_n) -> Objects_n {
		Objects_n::from_vec(
        		self.objects.union(&other.objects)
            	.map(|o| o.clone())
            	.collect())
    }
}


impl fmt::Display for Objects_n {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	let mut res = String::new();

    	if self.objects.is_empty() {
        	write!(&mut res, "No edges\n")?;
    	} else {
        	for o in self.objects.iter().sorted() {
            	write!(&mut res, "{}\n", o.to_string())?;
        	}

    	}
        write!(f, "{}", res)
	}
}



struct Edges {
    edges: HashSet<Edge>
}

impl Edges {
    pub fn from_vec(v: Vec<Edge>) -> Edges {
        let mut hs = HashSet::new();
        for e in v {
            hs.insert(e);
        }

        Edges { edges: hs }
    }

    pub fn merge(self, other: Edges) -> Edges {
		Edges::from_vec(
        		self.edges.union(&other.edges)
            	.map(|e| e.clone())
            	.collect())
    }
}

impl fmt::Display for Edges {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	let mut res = String::new();

    	if self.edges.is_empty() {
        	write!(&mut res, "No objects\n")?;
    	} else {
        	for o in self.edges.iter().sorted() {
            	write!(&mut res, "{}\n", o.to_string())?;
        	}

    	}
        write!(f, "{}", res)
	}
}


pub struct Structure {
    objects: Objects_n,
    edges:   Edges
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
            objects: Objects_n::from_vec(os.unwrap_or_default()),
			edges:   Edges::from_vec(es.unwrap_or_default())
        }
    }

    pub fn merge(self, other: Structure) -> Structure {
        Structure {
            objects: self.objects.merge(other.objects),
            edges: self.edges.merge(other.edges)
        }
    }

}

