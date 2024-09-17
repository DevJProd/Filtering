
struct _FilterCondition<T>{
    condition: T
}

impl<T: PartialEq> _FilterCondition<T>{
    fn _is_match(&self, item: &T) -> bool{
        &self.condition == item
    }
}

fn _custom_filter<T: PartialEq + Clone>(_collection : &Vec<T>, _filter: &_FilterCondition<T>)-> Vec<T>{

    let collection_filtered = _collection
        .iter() // Convert Vec to an iterator
        .filter(|x| _filter._is_match(x)) // Filter elements that match the condition
        .cloned() // Clone the elements to return owned values
        .collect::<Vec<T>>(); // Collect into a Vec<T>

        collection_filtered

    }

fn main() {
    
    let items = vec![5, 10, 15, 20, 25, 10, 6, 10, 48, 10];

    let filter = _FilterCondition { condition: 10 };

    let filtered_items = _custom_filter(&items, &filter);

    println!("Filtered items: {:?}", filtered_items);


}
