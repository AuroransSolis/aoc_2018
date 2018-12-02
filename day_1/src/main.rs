use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let input = input.to_string();
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &String) {
    let mut total = 0;
    for line in input.lines() {
        let parsed = line.parse::<i64>().unwrap();
        total += parsed;
    }
    println!("{}", total);
}

fn part_2(input: &String) {
    let mut total = 0;
    let mut starting_freqs = Vec::new();
    for line in input.lines() {
        let parsed = line.parse::<i64>().unwrap();
        total += parsed;
        starting_freqs.push(total);
    }
    let mut min_cycle = i64::max_value();
    let mut val_plus_ind = Vec::new();
    'a: for a in 0..starting_freqs.len() - 1 {
        for b in a + 1..starting_freqs.len() {
            let ba_diff = (starting_freqs[b] - starting_freqs[a]).abs();
            let cycle = ba_diff / total;
            if ba_diff % total == 0 && cycle <= min_cycle {
                min_cycle = cycle;
                val_plus_ind.push((starting_freqs[a], b, cycle));
            }
        }
    }
    let mut res_val = 0;
    let mut min_b = usize::max_value();
    for (val, b_ind, cycle) in val_plus_ind {
        if cycle != min_cycle {
            continue;
        } else {
            if b_ind < min_b {
                min_b = b_ind;
                res_val = val;
            }
        }
    }
    println!("{}", res_val);
}