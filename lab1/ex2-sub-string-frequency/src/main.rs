use std::io::{self, stdin};
fn main() {
    println!("Input string:");
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let input_str = line.parse::<String>().unwrap();
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }
    let _input_str = line.parse::<String>().unwrap();

    println!("Searching character:");
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }
    let lower_case_search_char = line
        .parse::<String>()
        .unwrap()
        .to_lowercase()
        .chars()
        .next()
        .unwrap();

    println!("The input string is: {:#?}", input_str);
    println!("The search character is: {}", lower_case_search_char);

    let mut freq = 0;
    let mut excluded_str = String::new();
    for ch in input_str.to_lowercase().chars() {
        if lower_case_search_char != ch {
            excluded_str.push(ch)
        } else {
            freq += 1;
        }
    }
    println!("The frequency is: {}", freq);
    println!("The excluded string is: {:#?}", excluded_str);
}
