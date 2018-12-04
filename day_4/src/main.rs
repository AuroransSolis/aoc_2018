fn main() {
    let input = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    part_1(&input);
}

#[derive(Copy, Clone, Debug)]
enum Action {
    WakesUp,
    FallsAsleep,
    BeginsShift
}

#[derive(Copy, Clone, Debug)]
struct LogItem {
    date: (usize, usize, usize),
    time: (usize, usize),
    guard: Option<usize>,
    action: Action
}

impl LogItem {
    fn new(date: (usize, usize, usize), time: (usize, usize), guard: Option<usize>, action: Action)
        -> Self {
        LogItem {
            date,
            time,
            guard,
            action
        }
    }
}

use std::cmp::Ordering::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn part_1(input: &Vec<&str>) {
    let mut log = Vec::new();
    for line in input {
        let tmp = line.split_at(1).1;
        let (year_str, rest) = tmp.split_at(4);
        let (_, rest) = rest.split_at(1);
        let (month_str, rest) = rest.split_at(2);
        let (_, rest) = rest.split_at(1);
        let (day_str, rest) = rest.split_at(2);
        let (_, rest) = rest.split_at(1);
        let (hour_str, rest) = rest.split_at(2);
        let (_, rest) = rest.split_at(1);
        let (minute_str, rest) = rest.split_at(2);
        let (_, log_item) = rest.split_at(2);
        let date = (year_str.parse::<usize>().unwrap(), month_str.parse::<usize>().unwrap(),
            day_str.parse::<usize>().unwrap());
        let time = (hour_str.parse::<usize>().unwrap(), minute_str.parse::<usize>().unwrap());
        let guard = if log_item.contains("Guard") {
            let (_, id_rest) = log_item.split_at(7);
            let id = id_rest.split_whitespace().next().unwrap();
            Some(id.parse::<usize>().unwrap())
        } else {
            None
        };
        let action = if log_item.contains("Guard") {
            Action::BeginsShift
        } else if log_item.contains("falls asleep") {
            Action::FallsAsleep
        } else {
            Action::WakesUp
        };
        log.push(LogItem::new(date, time, guard, action));
    }
    log.sort_unstable_by(|a, b| {
        match (a.date.0.cmp(&b.date.0), a.date.1.cmp(&b.date.1), a.date.2.cmp(&b.date.2),
            a.time.0.cmp(&b.time.0), a.time.1.cmp(&b.time.1)) {
            r @ (Less, _, _, _, _) | r @ (Greater, _, _, _, _) => r.0,
            r @ (_, Less, _, _, _) | r @ (_, Greater, _, _, _) => r.1,
            r @ (_, _, Less, _, _) | r @ (_, _, Greater, _, _) => r.2,
            r @ (_, _, _, Less, _) | r @ (_, _, _, Greater, _) => r.3,
            r @ (_, _, _, _, Less) | r @ (_, _, _, _, Greater) => r.4,
            _ => Equal
        }
    });
    let mut sleep_amts = HashMap::new();
    let mut id_tracker = HashSet::new();
    let mut log_iter = log.into_iter();
    let mut log_item = log_iter.next().unwrap();
    sleep_amts.insert(first.guard.unwrap(), 0);
    'outer: while let Some(item) = log_iter.next() {
        let mut minutes = [0u8; 60];
        let mut holder = log_iter.next().unwrap();
        'inner: while let Some(next) = log_iter.next() {
            match next {
                n @ LogItem{action: Action::BeginsShift, ..} => {
                    log_item = n;
                    if id_tracker.insert(item.guard.unwrap()) {
                        sleep_amts.insert(item.guard.unwrap(), minutes);
                    } else {
                        let tmp = sleep_amts.get_mut(&item.guard.unwrap()).unwrap();
                        for i in 0..60 {
                            tmp[i] = minutes[i];
                        }
                    }
                    continue 'outer;
                },
                n @ LogItem{action: Action::WakesUp, ..} => {
                    for i in holder.time.1..n.time.1 {
                        minutes[i] += 1;
                    }
                }
                n @ LogItem{..} => {
                    holder = n;
                    continue 'inner;
                }
            }
        }
    }
    let mut max = 0;
    let mut winner_id = 0;
    for (id, t) in sleep_amts.iter() {
        println!("Guard {}: {}", id, t);
        let tmp = t.iter().max().unwrap();
        if tmp > max {
            max = tmp;
            winner_id = id;
        }
    }
    println!("{} * {} => {}", max, winner_id, max * winner_id);
}