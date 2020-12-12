//! Demonstrate how to store strings
//! Usecase is storing 10M tag values (aka 10M strings) (us-east and us-midwest)

const NUM_TAGS: usize = 20_000_000;
//const NUM_TAGS: usize = 10_000;

use arrow::array::{Array, StringBuilder};
use arrow::compute::kernels::comparison::neq_utf8_scalar;


use std::time::Instant;

struct RAAITimer {
    start: std::time::Instant,
}

impl Default for RAAITimer {
    fn default() -> Self {
        Self { start: Instant::now() }
    }
}

impl RAAITimer {
    fn done(self) -> String{
        format!("{:?}", self.start.elapsed())
    }
}


pub fn example_with_vec() {
    println!("example_with_vec");
    let t = RAAITimer::default();

    let string_vec: Vec<String> = (0..NUM_TAGS).enumerate()
        .map(|(i, _)| {
            match i % 3 {
                0 => "us-east",
                1 => "us-midwest",
                2 => "us-west",
                _ => unreachable!(),
            }.into()
        })
        .collect();

     println!("created array with {} elements in {}", string_vec.len(), t.done());

    // Boolean array of which values are in / not in us-west
    for _ in 0..10 {
        let t = RAAITimer::default();
        let not_west_bitset: Vec<bool> = string_vec
            .iter()
            .map(|s| s != "us-west")
            .collect();

        println!("Completed finding bitset: {} elements in {}", not_west_bitset.len(), t.done());
    }
}


pub fn example_with_arrow() {
    println!("example_with_arrow");
    let t = RAAITimer::default();
    let mut builder = StringBuilder::new(NUM_TAGS);

    (0..NUM_TAGS)
        .enumerate()
        .for_each(|(i, _)| {
            let location = match i % 3 {
                0 => "us-east",
                1 => "us-midwest",
                2 => "us-west",
                _ => unreachable!(),
            };

            builder.append_value(location).unwrap()
        });

    let array = builder.finish();

    println!("created array with {} elements in {}", array.len(), t.done());

    // Boolean array of which values are in / not in us-west
    for _ in 0..10 {
        let t = RAAITimer::default();
        let not_west_bitset = neq_utf8_scalar(&array, "us-west").unwrap();
        println!("Found {} not in west in {}", not_west_bitset.len(), t.done());
    }
}
