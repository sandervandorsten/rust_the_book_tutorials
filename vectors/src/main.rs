use std::collections::HashMap;

fn main() {
    let v1 = vec![8, 2, 8, 1, 3, 2, 2, 3];
    let stats = collection_exercise_1(&v1);
    println!("The Statistics for vector {v1:?} are {stats:?}");
}

fn vec_exercise() {
    let mut vec = vec![1, 2, 3, 4, 5];
    println!("This is my vector : {vec:?}");
    vec.push(6);

    for i in &mut vec {
        *i += 50;
    }

    let third: &i32 = &vec[2];
    println!("third value = {third}");

    let third2 = vec.get(3);

    let third2 = match third2 {
        Some(val) => val,
        None => &0,
    };
    println!("Third2 value = {third2:?}");

    println!("{vec:?} and {third}");
}

#[derive(Debug)]
struct Statistics {
    mode: i32,
    median: f64,
}

fn collection_exercise_1(items: &Vec<i32>) -> Statistics {
    // Given a list of integers, use a vector and return
    // - the median (when sorted, the value in the middle position)
    // - mode (the value that occurs most often; a hash map will be helpful here) of the list.

    Statistics {
        mode: get_mode(items),
        median: get_median(items),
    }
}

// Gets the median of a non-empty integer slice or vector
fn get_median(items: &[i32]) -> f64 {
    // Sort the items in the vector without changing the original vector
    let mut sorted_items: Vec<i32> = items.to_vec();
    sorted_items.sort_unstable();
    println!("sorted vector: {sorted_items:?}");

    let len = sorted_items.len();

    // if uneven, take the middle value.
    if len % 2 == 1 {
        sorted_items[len / 2] as f64
    }
    // if even, grab the two middle values and calc average
    else {
        let mid = len / 2;
        (sorted_items[mid - 1] + sorted_items[mid]) as f64 / 2.0
    }
}

// Calculates the mode from a Vector
// in case of 'no mode' or multiple modes, it returns a greedy answer
// i.e. the first value it finds with that frequency.
fn get_mode(items: &[i32]) -> i32 {
    // calculate the mode hashmap

    let mut counts = HashMap::new();
    for &item in items {
        *counts.entry(item).or_insert(0) += 1;
    }

    // Get the most occurring value from the Mode Hashmap
    // let mut max_count: i32 = 0;
    // let mut mode: Option<i32> = None;
    // for (&item, &count) in &counts {
    //     if count > max_count {
    //         mode = Some(item);
    //         max_count = count;
    //     }
    // }
    let (mode, _): (i32, i32) = counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap_or((0, 0));
    mode
}
