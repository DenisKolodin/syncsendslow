
extern crate mongodb;
use mongodb::coll::Collection;

pub trait Worker {
    fn next(&mut self);
}

impl<F> Worker for F where F: FnMut(), F: Sync + Send {
    fn next(&mut self) { self() }
}

pub struct GetRecord {
	collection: Collection,
}

impl Worker for GetRecord {
	fn next(&mut self) {
		let mut cursor = self.collection.find(None, None).unwrap();
		let _ = cursor.next();
	}
}

fn main() {
}
