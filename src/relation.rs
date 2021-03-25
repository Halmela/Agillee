use agillee::object::*;

enum Relation {
    One(Object, Object),
    Both(Object, Object),
	Nothing
}
