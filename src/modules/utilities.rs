pub mod utils{

    //Ok if insert is made and Err if not if is_unique_collection is true then no insertion will be made if a match is found
    pub fn insert_sorted<T: Ord>(vec: &mut Vec<T>, item: T, is_unique_collection : bool)->Result<usize, usize> {
        return match vec.binary_search(&item) {
            Ok(pos) => {
                if is_unique_collection {
                    vec.insert(pos - 1, item);
                    return Ok(pos);
                }
                Err(pos)
            },
            Err(pos) => {
                vec.insert(pos , item);
                Ok(pos)
            },
        }
    }
    pub fn is_index_inbound<T>(x: &Vec<T>, index : usize)->bool {
        index < x.len()
    }

}
