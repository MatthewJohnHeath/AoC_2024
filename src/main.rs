use std::fs::read_to_string;

use std::collections::BTreeMap;

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

    fn raising_slowly_mostly( levels: &[i32])->bool{
        if levels.len() < 3 {return true;}
  
        let mut removed = None;
        for i in 1..levels.len(){

            let current = levels[i];
            let previous = if removed == Some(i-1){levels[i-2]} else {levels[i-1]};
            let diff = current - previous;
            
            if diff > 0 && diff < 4 {
                continue;
            }

            if removed.is_some() {return false;}
            if i == levels.len() - 1 {return true};
            let next = levels[i+1];

            if diff < 1  {
                if next > previous {removed = Some(i);}
                else if i == 1 || current > levels[i-2] {removed = Some(i-1);}
                else {return false;}
            }
            else{
                if next < current || next - previous < 4 {removed = Some(i);}
                else if i == 1 { removed = Some(i - 1);}
                else {return false;}
            }
        }
        true
    }

    fn safe(levels:&[i32])->bool{
       let safe_up = raising_slowly_mostly(&levels);
       let safe_down  = raising_slowly_mostly(&levels.iter().rev().map(|&x|x).collect::<Vec<i32>>()); 
       safe_up || safe_down 
    }

    let second_part = reports
    .iter()
    .filter(|vec|safe(&vec))
    .count();

    println!("day 2, part 2: {}", second_part);
}