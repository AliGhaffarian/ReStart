pub mod utils{

    pub fn insert_sorted<T: Ord>(vec: &mut Vec<T>, item: T) {
        let pos = match vec.binary_search(&item) {
            Ok(pos) => pos -1,
            Err(pos) => pos,
        };

        vec.insert(pos, item);
    }

}
