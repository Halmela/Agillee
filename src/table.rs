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
            "CREATE TABLE objects (
            	id 	        SERIAL PRIMARY KEY,
            	description TEXT
        	);",
        Table::Relation =>
        	"CREATE TABLE relations (
            	start INTEGER REFERENCES objects(id),
            	sink  INTEGER REFERENCES objects(id)
        	);",
		_ 	   => "CREATE TABLE empty();"
	}
}

