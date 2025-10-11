use rust_gds::collections::{
    HugeSparseDoubleArrayList, HugeSparseDoubleList, HugeSparseLongArrayArrayList,
    HugeSparseLongArrayList, HugeSparseLongList,
};

#[test]
fn test_long_long_consumer() {
    let list = HugeSparseLongList::of(0);
    list.set(10, 42);
    list.set(100, 99);
    list.set(1000, 123);

    let mut collected = Vec::new();

    // Using closure directly
    list.for_all(|index, value| {
        collected.push((index, value));
    });

    collected.sort();
    assert_eq!(collected, vec![(10, 42), (100, 99), (1000, 123)]);
}

#[test]
fn test_long_double_consumer() {
    let list = HugeSparseDoubleList::of(0.0);
    list.set(10, 3.15); // Not PI, just a test value
    list.set(100, 2.70);

    let mut sum = 0.0;

    list.for_all(|_index, value| {
        sum += value;
    });

    assert!((sum - 5.85_f64).abs() < 1e-10);
}

#[test]
fn test_long_long_array_consumer() {
    let list = HugeSparseLongArrayList::of(vec![]);
    list.set(10, vec![1, 2, 3]);
    list.set(100, vec![4, 5]);

    let mut total_elements = 0;

    list.for_all(|_index, array| {
        total_elements += array.len();
    });

    assert_eq!(total_elements, 5);
}

#[test]
fn test_long_double_array_consumer() {
    let list = HugeSparseDoubleArrayList::of(vec![]);
    list.set(10, vec![3.15, 2.70]); // Not PI/E, just test values
    list.set(100, vec![1.41]);

    let mut sum = 0.0;

    list.for_all(|_index, array| {
        for &value in array {
            sum += value;
        }
    });

    assert!((sum - 7.26_f64).abs() < 1e-10);
}

#[test]
fn test_long_long_array_array_consumer() {
    let list = HugeSparseLongArrayArrayList::of(vec![]);
    list.set(10, vec![vec![1, 2], vec![3, 4, 5]]);
    list.set(100, vec![vec![6]]);

    let mut total_elements = 0;

    list.for_all(|_index, matrix| {
        for row in matrix {
            total_elements += row.len();
        }
    });

    assert_eq!(total_elements, 6);
}

#[test]
fn test_consumer_with_named_function() {
    fn print_consumer(index: usize, value: i64) {
        // This would print in a real scenario
        // For test, we just verify it compiles and can be called
        let _ = (index, value);
    }

    let list = HugeSparseLongList::of(0);
    list.set(10, 42);

    // Can use named function matching the LongLongConsumer signature
    list.for_all(print_consumer);

    assert_eq!(list.get(10), 42);
}

#[test]
fn test_consumer_type_inference() {
    let list = HugeSparseLongList::of(0);
    list.set(10, 42);
    list.set(100, 99);

    // Rust's type inference works with the consumer closure
    let mut max_value = i64::MIN;

    list.for_all(|_idx, val| {
        if val > max_value {
            max_value = val;
        }
    });

    assert_eq!(max_value, 99);
}

#[test]
fn test_consumer_captures_mutable_state() {
    let list = HugeSparseLongArrayList::of(vec![]);
    list.set(0, vec![1, 2]);
    list.set(1, vec![3, 4, 5]);
    list.set(2, vec![6]);

    let mut histogram: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();

    // Consumer captures and mutates external state
    list.for_all(|_index, array| {
        let len = array.len();
        *histogram.entry(len).or_insert(0) += 1;
    });

    assert_eq!(histogram.get(&1), Some(&1)); // one array of length 1
    assert_eq!(histogram.get(&2), Some(&1)); // one array of length 2
    assert_eq!(histogram.get(&3), Some(&1)); // one array of length 3
}
