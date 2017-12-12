use std::io::{BufReader, BufRead};
use std::fs::File;

#[test]
fn test_anagrams() {
    assert_eq!(anagrams(&vec!["abcde".into(), "xyz".into(), "ecdab".into()]), false);
    assert_eq!(anagrams(&vec!["iiii".into(), "oiii".into(), "ooii".into(), "oooi".into(), "oooo".into()]), true);
}

fn anagrams(w: &[String]) -> bool {
    let m = w.iter()
              .map(|x| {let mut q: Vec<u32> = x.chars().map(|c| c as u32).collect(); q.sort(); q})
              .collect::<Vec<Vec<u32>>>();
    for (i, x) in m.iter().enumerate() {
        for j in i+1 .. m.len() {
            if x.len() == m.get(j).unwrap().len() {
                if x.iter().zip(m.get(j).unwrap()).fold(0, |acc, (a,b)| acc + (a ^ b)) == 0 {
                    println!("anagram detected: {} -> {}", w[i], w[j]);
                    return false
                }
            }            
        }
    }
    true
}

fn main() {
    println!("Hello, world!");
    let f = File::open("input").unwrap();
    let b = BufReader::new(&f);

    let mut first:  u32 = 0;
    let mut second: u32 = 0;
    
    for line in b.lines().map(|line| line.unwrap()) {
        let mut words = line.split_whitespace().map(|w| w.to_string()).collect::<Vec<_>>();
        let w = words.len();
        words.sort();
        words.dedup();

        if w == words.len() {
            first += 1;
            if anagrams(&words[..]) {
                second += 1;
            } 
        }

    }
    println!("{}, {}", first, second);
}
