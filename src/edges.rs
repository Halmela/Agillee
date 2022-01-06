use crate::edge::*;
use std::collections::{HashSet};
use std::fmt;
use std::fmt::Write;
use itertools::Itertools;
use std::convert::From;


#[derive(Default)]
pub struct Edges {
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

    pub fn empty() -> Edges {
        Edges {
            edges: HashSet::new()
        }
    }

    pub fn blank() -> Edges {
        Edges {
            edges: [Default::default()].iter().cloned().collect()
        }
    }

    pub fn merge(self, other: &Edges) -> Edges {
		Edges::from_vec(
        		self.edges.union(&other.edges)
            	.map(|e| e.clone())
            	.collect())
    }

    pub fn get_edges(&self) -> Vec<Edge> {
        self.edges.iter().map(|o| o.clone()).collect()
    }
}

impl From<Edge> for Edges {
    fn from(item: Edge) -> Self {
        Edges { edges: [item].iter().cloned().collect() }
    }

}

impl fmt::Display for Edges {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	let mut res = String::new();

    	if self.edges.is_empty() {
        	write!(&mut res, "No edges\n")?;
    	} else {
        	for o in self.edges.iter().sorted() {
            	write!(&mut res, "{}\n", o.to_string())?;
        	}

    	}
        write!(f, "{}", res)
	}
}


