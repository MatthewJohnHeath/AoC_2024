use std::fs::read_to_string;

use std::collections::BTreeMap;
fn main() {
    let mut first_list  = Vec::<u32>::new();
    let mut second_list = Vec::<u32>::new();

    for line in read_to_string("input.txt").unwrap().lines() {
        let numbers = line.to_string()
            .split(" ")
            .filter_map(|word|word.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        first_list.push(numbers[0]);
        second_list.push(numbers[1]);
    }

    first_list.sort();
    second_list.sort();

    let answer = first_list.iter().zip(second_list.iter())
        .map(|(&first, &second)| first.abs_diff(second))
        .sum::<u32>();
    println!("{}", answer);

    let mut counts = BTreeMap::new();
    for &key in &second_list{
        match counts.get(&key){
            Some(&count) => counts.insert(key, count + 1u32),
            None => counts.insert(key, 1)
        };
    }


    let second_answer = first_list.iter()
        .filter_map(|entry|counts.get_key_value(entry))
        .map(|(&entry, &count)| entry * count)
        .sum::<u32>();

    print!("second answer {}", second_answer);
}
