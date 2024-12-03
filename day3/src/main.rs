use std::fs;
fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    println!("{}", mul_sum(&text));

    let second_answer = text.split("do()")
        .filter_map(|substring| substring.split("don't()").next())
        .map(mul_sum)
        .sum::<i32>();
     
    println!("{}", second_answer);
    
}
fn mul( first:&str, second :&str)->Option<i32>{
    let f = first.parse::<i32>().ok()?;
    let s = second.parse::<i32>().ok()?;
    Some(f*s)
}

fn mul_sum(text:&str)->i32{
    (" ".to_owned() + text).split("mul(")
    .filter_map(|substring| substring.split_once(')'))
    .filter_map(|(before_bracket, _)| before_bracket.split_once(','))
    .filter_map(|(f,s)|mul(f,s))
    .sum::<i32>()
}