/*
 * Every table/struct should also be enum
 */

pub enum Table {
    Object,
    Relation
}


/*
 * Return SQL-statement describing the table
 * or an empty table as fallback
 */

pub fn table_to_scheme(table: &Table) -> &'static str {
	match table {
    	Table::Object =>
            "CREATE TABLE Objects (
            	id 	        SERIAL PRIMARY KEY,
            	description TEXT
        	);",

        Table::Relation =>
        	"CREATE TABLE Relations (
            	id   SERIAL PRIMARY KEY,
            	a    INTEGER REFERENCES Objects(id),
            	b    INTEGER REFERENCES Objects(id),
            	a2b  BOOLEAN,
            	b2a  BOOLEAN,
            	CHECK (a <= b)
        	);",
		//_ 	   => "CREATE TABLE empty();"
	}
}

