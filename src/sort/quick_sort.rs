use std::time::Instant;
use rand::Rng;

pub fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return
    }

    let len = arr.len();
    let pivot_index = len - 1;

    let mut store = 0;

    for i in 0..pivot_index {
        if arr[i] < arr[pivot_index] {
            arr.swap(i, store);
            store += 1;
        }
    }

    // place pivot in final position
    arr.swap(store, pivot_index);

    let (left, right) = arr.split_at_mut(store);

    println!("{:?}", left.to_vec());
    println!("{:?}", right.to_vec());

    quick_sort(left);
    quick_sort(&mut right[1..]);
}

#[test]
fn test_quick_sort() {
    // generate a big random test vector
    let mut rng = rand::thread_rng();
    let mut data: Vec<i32> = (0..1_000_00)
        .map(|_| rng.gen_range(0..10_000_000))
        .collect();

    let mut copy = data.clone(); // for verification

    let start = Instant::now();
    quick_sort(&mut data);
    let dur = start.elapsed();

    println!("Quicksort finished in {:?}", dur);

    // verify correctness by comparing to std sort
    copy.sort();
    assert_eq!(data, copy);

    println!("Sort validated!");
}