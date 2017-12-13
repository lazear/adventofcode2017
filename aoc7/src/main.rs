use std::io::{BufRead,BufReader};
use std::fs::File;
use std::collections::{HashSet, HashMap};

fn tree(base: &str, roots: &HashMap<String, Vec<String>>, weights: &HashMap<String, u32>) -> u32 {
    let mut ret: u32 = 0;
    if let Some(root) = weights.get(base) {
        ret = *root;
        if let Some(branches) = roots.get(base) {
            let mut w: Vec<u32> = branches.iter().map(|b| tree(b, roots, weights)).collect();
            ret += w.iter().fold(0, |acc, x| acc + * x);
            let first = w[0];
            let mismatch = w.iter().zip(branches.iter()).filter(|&(weight, _)| *weight != first).collect::<Vec<_>>();
            if mismatch.len() > 0 {
                println!("diff @ {} of {} = {}", mismatch[0].1, mismatch[0].0 - first, weights.get(mismatch[0].1).unwrap() - (mismatch[0].0 -first));
            }
        }
    }
    ret
}

fn main() {
    let f = File::open("input").unwrap();
    let b = BufReader::new(&f);
    let mut tops: Vec<Vec<String>> = 
        b.lines().map(|s| s.unwrap().split("->").map(|s| s.trim().into())
         .collect::<Vec<String>>()).collect();
    let input: Vec<Vec<String>> = tops.iter().cloned().filter(|x| x.len()>1).collect();
    tops = tops.into_iter().filter(|x| x.len() == 1).collect();

    let mut lower: HashMap<String, u32> = HashMap::new();
    let mut upper: HashSet<String> = HashSet::new();
    let mut mapping: HashMap<String, Vec<String>> = HashMap::new();

    for x in input.into_iter() {
        let l: Vec<_> = x[0].split(|c| c == '(' || c == ')').collect();
        let u: Vec<_> = x[1].split(|c| c == ',' || c == ' ').filter(|s| *s != "").collect();
        let base: String = l[0].trim().into();
        lower.insert(base.clone(), l[1].parse::<u32>().unwrap());
        
        let v: Vec<String> = u.into_iter().map(|s| {
            let s: String = s.into();
            upper.insert(s.clone());
            s
        }).collect();
        mapping.insert(base, v);
    }
    let keys: HashSet<String> = lower.keys().cloned().collect();
    let base: Vec<String> = keys.difference(&upper).cloned().collect();

    println!("bottom most program = {:?}", &base);
    
    // Sloppy. Probably a better way than adding these back in later
    for x in tops.into_iter() {
        let l: Vec<_> = x[0].split(|c| c == '(' || c == ')').collect();
        let base: String = l[0].trim().into();
        lower.insert(base.clone(), l[1].parse::<u32>().unwrap());
    }
    println!("{:?}", tree(&base[0], &mapping, &lower));

}
