//! Collections Quick Tour
//!
//! - Portable analytics over Vec and Huge via Collections<T>
//! - Paging over Huge via HugeCursorSupport

use gds::collections::traits::Collections;

// Vec backends
use gds::collections::backends::vec::{VecDouble, VecLong};

// Huge backends
use gds::collections::backends::huge::{HugeDoubleArray, HugeLongArray};
use gds::concurrency::Concurrency;

// Huge paging
use gds::collections::cursor::{init_cursor, HugeCursor};

fn portable_vec_demo() {
    let v = VecLong::from(vec![1, 2, 3, 4, 5]);
    println!(
        "[VecLong] len={} sum={:?} mean={:?} median={:?}",
        v.len(),
        v.sum(),
        v.mean(),
        v.median()
    );

    let d = VecDouble::from(vec![1.5, 2.5, 3.5, 4.5]);
    // Percentile requires `T: Ord` on Collections; f64 does not implement Ord.
    // Print mean instead for floating-point collections.
    println!(
        "[VecDouble] len={} sum={:?} mean={:?}",
        d.len(),
        d.sum(),
        d.mean()
    );
}

fn portable_huge_demo() {
    // Small size for demo; same API scales to billions
    let h = HugeLongArray::with_generator(1_000, Concurrency::of(4), |i| (i as i64) % 10);
    println!(
        "[HugeLong] len={} sum={:?} mean={:?} median={:?}",
        h.len(),
        h.sum(),
        h.mean(),
        h.median()
    );

    let hd = HugeDoubleArray::with_generator(1_000, Concurrency::of(2), |i| (i as f64) * 0.5);
    println!("[HugeDouble] len={} mean={:?}", hd.len(), hd.mean());
}

fn huge_paging_demo() {
    let h = HugeLongArray::with_generator(128, Concurrency::of(1), |i| i as i64);
    // Portable access still available
    println!("[HugeLong paging] mean={:?}", h.mean());

    // Page-wise scan
    let mut cursor = h.new_cursor();
    init_cursor(&h, &mut cursor);
    let mut acc: i64 = 0;
    while cursor.next() {
        let page = cursor.array().unwrap();
        for i in cursor.offset()..cursor.limit() {
            acc += page[i];
        }
    }
    println!("[HugeLong paging] sum={}", acc);
}

fn main() {
    println!("== Collections Quick Tour ==");
    portable_vec_demo();
    portable_huge_demo();
    huge_paging_demo();
}
