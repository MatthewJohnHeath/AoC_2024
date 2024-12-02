use std::fs::read_to_string;

use std::collections::BTreeMap;
use std::collections::HashSet;

fn main() {
    day_one("input1.txt");
    day_two("input2.txt");
}

fn day_one(input:&str){
    let mut first_list  = Vec::<u32>::new();
    let mut second_list = Vec::<u32>::new();

    for line in read_to_string(input).unwrap().lines() {
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

    println!("second answer {}", second_answer);
}

fn day_two(input: &str){
    let reports= read_to_string(input).unwrap()
    .lines()
    .map(|line|
        line.split(" ")
        .filter_map(|word| word.parse::<i32>().ok()).collect::<Vec<i32>>()
    ).collect::<Vec<Vec<i32>>>();

    let diff_lines= reports.iter().map(|line| 
        line.windows(2)
        .map(|win|win[0]-win[1]).collect::<Vec<i32>>()
    ).collect::<Vec<Vec<i32>>>();

    let first_part= &diff_lines.iter().filter(|&diffs| 
         diffs.iter().all(|diff| diff.abs() < 4 )
          &&
          (diffs.iter().all(|&diff| diff > 0) || diffs.iter().all(|&diff| diff < 0))
    )
    .count();
    println!("day 2, part 1: {}", first_part);

    fn safe(diffs:&[i32])->bool{
        let non_decreasing = diffs.iter()
            .zip(0..)
            .filter(|(&diff,_)|diff >= 0)
            .map(|(_,i)|i).collect::<Vec<usize>>();

        let non_increasing = diffs.iter()
            .zip(0..)
            .filter(|(&diff, _)|diff <= 0)
            .map(|(_,i)|i).collect::<Vec<usize>>();

        let big_jumps = diffs.iter()
            .zip(0..)
            .filter(|(&diff,_)|diff.abs() > 3)
            .map(|(_,i)|i).collect::<Vec<usize>>();
        
        if big_jumps.len() > 1 {return false;}
        if non_decreasing.len() > 1 && non_increasing.len() > 1 {return false;}
        
        let no_jump = big_jumps.is_empty();
        let start_jump = big_jumps.len() == 1 && big_jumps[0] == 0 ;
        let end_jump = big_jumps.len() == 1 && big_jumps[0] == diffs.len() - 1 ;
        
        if non_decreasing.len() == 0 && non_increasing.len() == 0 {
            return no_jump || start_jump || end_jump ;
        }
 
        let decreasing = non_decreasing.len() == 1 ;
        let index = if decreasing {non_decreasing[0]} else {non_increasing[0]};
        
        if index == 0{
            return no_jump || start_jump;
        }
        if index == diffs.len() -1{
            return no_jump || end_jump
        }
        let new_diff = diffs[index] + diffs[index + 1];
        let new_diff_good = if decreasing {new_diff <0 && new_diff > -4} else {new_diff>0 && new_diff < 4};
        
        if !new_diff_good {return false;}
        if no_jump {return true;}
        
        let jump_index = big_jumps[0]; 
        
    }
    let second_part = diff_lines
    .iter()
    .filter(|vec|safe(&vec))
    .count();

    println!("day 2, part 2: {}", second_part);
}