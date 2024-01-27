use std::collections::HashMap;

fn main() {
    let input = {
        let args = std::env::args();
        let args: Vec<String> = args.collect();
        String::from(args.get(1).expect("no path to input given"))
    };
    let input = std::fs::read_to_string(&input).expect("Unable to read file to string");
    let bag = {
        let mut bag = HashMap::new();
        bag.insert(String::from("red"), 12);
        bag.insert(String::from("green"), 13);
        bag.insert(String::from("blue"), 14);
        bag
    };
    let mut ids: Vec<usize> = Vec::with_capacity(100);
    for line in input.lines() {
        let (id, draw_str) = {
            let parts = line.split(':')
                .filter(|s| !s.is_empty())
                .map(str::trim)
                .collect::<Box<[&str]>>();
            assert_eq!(parts.len(), 2);
            let id: usize = parts.get(0).unwrap()
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse().unwrap();
            let draw_info = *parts.get(1).unwrap();
            (id, draw_info)
        };
        if !draw_str.split(';')
            .map(|s| s.split(',').map(str::trim))
            .flatten()
            .any(|ball_info| {
                let parts = ball_info.split(' ').filter(|s| !s.is_empty()).collect::<Box<[&str]>>();
                assert_eq!(parts.len(), 2);
                let p1 = parts[0];
                let p2 = parts[1];
                if let Ok(n) = p1.parse::<usize>() {
                    n > *bag.get(p2).unwrap()
                } else if let Ok(n) = p2.parse::<usize>() {
                    n > *bag.get(p1).unwrap()
                } else {
                    false
                }
            }) {
            ids.push(id);
        }
    }
    println!("Sum of passed ids: {}", ids.iter().sum::<usize>());
}