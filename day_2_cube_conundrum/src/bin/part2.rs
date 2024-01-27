use std::collections::HashMap;

fn ball_map() -> HashMap<&'static str, usize> {
    HashMap::from(
        [
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]
    )
}

fn reset_ball_map(map: &mut HashMap<&'static str, usize>) {
    map.values_mut().for_each(|mut v| *v = 0);
}

fn main() {
    let input = {
        let args = std::env::args();
        let args: Vec<String> = args.collect();
        String::from(args.get(1).expect("no path to input given"))
    };
    let input = std::fs::read_to_string(&input).expect("Unable to read file to string").leak();
    let mut max_balls = ball_map();
    let sum = input.lines()
        .map(|line| {
            reset_ball_map(&mut max_balls);
            let draw_info = line.split(':')
                .filter(|s| !s.is_empty())
                .collect::<Box<[&str]>>()[1];
            draw_info.split(';').for_each(|d| {
                d.split(',').for_each(|b_set| {
                    let parts = b_set.split(' ')
                        .filter(|s| !s.is_empty())
                        .collect::<Box<[&str]>>();
                    let (p1, p2) = (parts[0], parts[1]);
                    if let Ok(n) = p1.parse::<usize>() {
                        if n > *max_balls.get(p2).unwrap() {
                            max_balls.entry(p2).and_modify(|v| *v = n);
                        }
                    } else if let Ok(n) = p2.parse::<usize>() {
                        if n > *max_balls.get(p1).unwrap() {
                            max_balls.entry(p1).and_modify(|v| *v = n);
                        }
                    }
                })
            });
            let accu = max_balls.values().map(|v| v.to_owned()).reduce(|acc, value|
                acc * value
            ).unwrap();
            accu
        })
        .sum::<usize>();
    println!("what a sum: {}", sum)
}