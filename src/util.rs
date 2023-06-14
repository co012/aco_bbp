use std::collections::HashMap;

pub fn process_items(items: &[usize]) -> (usize, HashMap<usize, usize>, Vec<usize>) {
    let mut size_count = 1;
    let mut size_to_index: HashMap<usize, usize> = HashMap::new();
    let mut index_to_size: Vec<usize> = Vec::new();
    size_to_index.insert(items[0], 0);
    index_to_size.push(items[0]);
    for i in 0..(items.len() - 1) {
        if items[i] != items[i + 1] {
            size_to_index.insert(items[i + 1], size_count);
            index_to_size.push(items[i + 1]);
            size_count += 1;
        }
    }
    (size_count, size_to_index, index_to_size)
}