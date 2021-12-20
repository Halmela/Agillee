/*
 * Every table/struct should also be enum
 */

pub enum Table {
    Object,
    Relation,
    Edge
}


/*
 * Return SQL-statement describing the table
 * or an empty table as fallback
 */

pub fn table_to_scheme(table: &Table) -> &'static str {
	match table {
    	// Objects contain at least root, tangible and intangible objects
    	Table::Object =>
            "CREATE TABLE Objects (
            	id 	        SERIAL PRIMARY KEY,
            	description TEXT
        	);
        	INSERT INTO Objects (description)
            	VALUES ('Root'), ('Tangible'), ('Intangible'), ('Void')",

        Table::Relation  =>
        	"CREATE TABLE Relations (
            	a    INTEGER REFERENCES Objects(id),
            	b    INTEGER REFERENCES Objects(id),
            	a2b  BOOLEAN,
            	b2a  BOOLEAN,
            	UNIQUE (a, b),
            	CHECK (a <= b)
        	);
        	INSERT INTO Relations (a, b, a2b, b2a)
            VALUES (1, 2, TRUE, TRUE), (1, 3, TRUE, TRUE), (2,3,FALSE,FALSE)",


        Table::Edge  =>
        	"CREATE TABLE Edges (
            	a    INTEGER REFERENCES Objects(id),
            	b    INTEGER REFERENCES Objects(id),
            	a2b  INTEGER REFERENCES Objects(id),
            	b2a  INTEGER REFERENCES Objects(id),
            	UNIQUE (a, b),
            	CHECK ((a <= b) AND NOT (a <= 4 AND b2a > 4))
        	);
        	INSERT INTO Edges (a, b, a2b, b2a)
            VALUES (1, 2, 2, 4),
                   (1, 3, 3, 4),
                   (2, 3, 4, 4),
                   (1, 4, 1, 4)",
		//_ 	   => "CREATE TABLE empty();"
	}
}


