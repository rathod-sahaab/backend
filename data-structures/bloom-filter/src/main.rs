use prettytable::{Row, Table};

pub mod bloom_filter;
pub mod bloom_filters;
use bloom_filter::BloomFilter;

use crate::bloom_filters::bloom_filters_32_arr::BloomFilter32;

fn main() {
    let mut bl = BloomFilter32::default();

    bloomy(&mut bl);

    println!("{:?}", bl);
}

fn bloomy(bl: &mut impl BloomFilter) {
    let keys = vec!["mango", "apple", "orange", "banana"];
    let mut test_keys = vec!["carrot", "radish", "vegetable", "onion"];

    keys.iter().for_each(|&key| bl.insert(key));

    test_keys.extend(keys);

    let results = test_keys
        .iter()
        .map(|&key| bl.contains(key))
        .collect::<Vec<bool>>();

    // table output
    let mut table = Table::new();

    test_keys.iter().zip(results).for_each(|(key, result)| {
        table.add_row(Row::from(vec![key, result.to_string().as_str()]));
    });

    table.printstd();
}
