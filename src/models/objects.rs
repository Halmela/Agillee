use crate::models::object::*;
use std::collections::{HashSet};
use itertools::Itertools;
use std::convert::From;


#[derive(Default)]
pub struct Objects {
    objects: HashSet<Object>
}

impl Objects {
    pub fn empty() -> Objects {
        Objects { objects: HashSet::new() }
    }

    pub fn blank() -> Objects {
		Objects {
    		objects: [Default::default()].iter().cloned().collect()
		}
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
        self.objects.iter().sorted().map(|o| o.clone()).collect()
    }
}

impl From<Object> for Objects {
    fn from(item: Object) -> Self {
        Objects { objects: [item].iter().cloned().collect() }
    }
}


/*
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
*/

