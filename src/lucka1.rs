use std::fs::read_to_string;
use lazy_static::lazy_static;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn extract_last(input: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
    }
    let mut res: String = "".to_owned();
    for c in input.chars() {
        if c.is_numeric() { res.push(c) }
    }
    let mut out: String = "".to_owned();
    out.push(res.chars().next().unwrap());
    out.push(res.chars().nth(res.len() - 1).unwrap());
    out
}

fn do_it(filename: &str) -> i32 {
    read_lines(filename)
        .to_owned()
        .iter()
        .map(|it| extract_last(it))
        .map(|it| it.parse::<i32>().unwrap())
        .sum()
}

#[test]
fn test_mine() {
    assert_eq!(extract_last("abc2"), "22");
    assert_eq!(extract_last("a1bc2"), "12");
    assert_eq!(extract_last("a1bc2re5gt"), "15");
}

#[test]
fn test_file() {
    assert_eq!(read_lines("dataLucka1.txt").len(), 1000);
    assert_eq!(do_it("dataLucka1.txt"), 54338);
}

