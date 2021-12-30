use crate::object::*;
use crate::edge::*;
use std::collections::{HashSet};
use std::fmt;
use std::fmt::Write;
use itertools::Itertools;


pub struct Objects {
    objects: HashSet<Object>
}

impl Objects {
    pub fn empty() -> Objects {
        Objects { objects: HashSet::new() }
    }

    pub fn from_vec(v: Vec<Object>) -> Objects {
        let mut hs = HashSet::new();
        for o in v {
            hs.insert(o);
        }

        Objects {objects: hs}
    }

    pub fn merge(self, other: &Objects) -> Objects {
		Objects::from_vec(
        		self.objects.union(&other.objects)
            	.map(|o| o.clone())
            	.collect())
    }

    pub fn get_objects(&self) -> Vec<Object> {
        self.objects.iter().map(|o| o.clone()).collect()
    }
}


impl fmt::Display for Objects {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	let mut res = String::new();

    	if self.objects.is_empty() {
        	write!(&mut res, "No objects\n")?;
    	} else {
        	for o in self.objects.iter().sorted() {
            	write!(&mut res, "{}\n", o.to_string())?;
        	}

    	}
        write!(f, "{}", res)
	}
}

