use std::fmt;


/*
 * Every table/struct should also be enum
 */
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Table {
    Objects,
    Descriptions,
    Relations,
    Edges,
    Forms,
    Formations,
}

impl Table {
    pub fn all_schemes() -> Vec<&'static str> {
    [ Table::Objects, Table::Relations,
	  Table::Edges, Table::Forms,
      Table::Formations ].iter()
        .map(|t| t.to_scheme())
        .collect()
    }


    /*
     * Return SQL-statement describing the table
     * or an empty table as fallback
     */

    fn to_scheme(&self) -> &'static str {
    	match self {
        	// Objects contain at least root, tangible and intangible objects
        	Table::Objects =>
                "CREATE TABLE Objects (
                	id 	        SERIAL PRIMARY KEY,
                	description TEXT
            	);
            	INSERT INTO Objects (description)
                	VALUES ('Root'), ('Tangible'), ('Intangible'), ('Void')
            	",
            Table::Relations  =>
            	"CREATE TABLE Relations (
                	a    INTEGER REFERENCES Objects(id),
                	b    INTEGER REFERENCES Objects(id),
                	a2b  BOOLEAN,
                	b2a  BOOLEAN,
                	UNIQUE (a, b),
                	CHECK (a <= b)
            	);
            	INSERT INTO Relations (a, b, a2b, b2a)
                VALUES (1, 2, TRUE, TRUE), (1, 3, TRUE, TRUE), (2,3,FALSE,FALSE)
                ",
            Table::Edges  =>
            	"CREATE TABLE Edges (
                	id   SERIAL PRIMARY KEY,
                	a    INTEGER REFERENCES Objects(id),
                	b    INTEGER REFERENCES Objects(id),
                	a2b  INTEGER REFERENCES Objects(id),
                	b2a  INTEGER REFERENCES Objects(id)
            	);
            	INSERT INTO Edges (a,b,a2b,b2a) 
                VALUES (1, 1, 1, 4)
                ;",
            Table::Forms =>
                "CREATE TABLE Forms (
                    id   SERIAL PRIMARY KEY,
                    form TEXT
                );
                INSERT INTO Forms (form)
                VALUES ('Root'), ('Tangible'), ('Intangible'), ('Void')
                ;",
            Table::Formations =>
                "CREATE TABLE Formations (
                    object INTEGER REFERENCES Objects(id) ON DELETE CASCADE,
                    form   INTEGER REFERENCES Forms(id),
                    UNIQUE (object)
                );
                INSERT INTO Formations (object, form)
                VALUES (1,1), (2,2), (3,3), (4,4)
                ;",
    		_ 	   => "CREATE TABLE empty();"
    	}
    }

/*
    pub fn to_select(&self) -> &'static str {
        match self {
            Table::Objects => {""},
            Table::Formations => {""},
            Table::Descriptions => {""},
            _ => {""}
        };
        "SELECT"
    }

    pub fn to_from(&self) -> &'static str {
        match self {
            Table::Objects => {"Objects O"},
            Table::Formations => {""},
            Table::Descriptions => {""},
            _ => {""}
        };
        "FROM"
    }

    pub fn to_where(&self, i: i32) -> &'static str {
        match self {
            Table::Objects => {format!(" AND ${} ",i)},
            Table::Formations => {format!(" AND ${} ",i)},
            Table::Descriptions => {format!(" AND D.description LIKE ${} ",i)},
            _ => {format!("${}",i)}
        }
    }
    */

}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = match self {
            Table::Objects => "Objects",
            Table::Descriptions => "Descriptions",
            Table::Edges => "Edges",
            Table::Forms => "Forms",
            Table::Formations => "Formations",
            _ => ""
        };

        write!(f, "{}", t)
    }
}
