use Event::*;
use std::mem::take;

impl Solution {
    pub fn count_mentions(n_users: i32, events: Vec<Vec<String>>) -> Vec<i32>  {
        let mut mentions = vec![0; n_users as usize];
        let mut offline  = vec![0; n_users as usize];

        let mut events = events.into_iter().map(Event::new).collect::<Vec<_>>();

        events.sort_unstable_by_key(Event::sort_key);

        for event in events {
            match event {
                Message(timestamp, mentions_str) => {
                    match mentions_str.as_str() {
                        "ALL" => {
                            for m in &mut mentions {
                                *m += 1;
                            }
                        },
                        "HERE" => {
                            let iter = mentions.iter_mut().zip(&offline);
                            for (m, &off) in iter {
                                if off <= timestamp {
                                    *m += 1;
                                }
                            }
                        },
                        ids_str => {
                            for ids in ids_str.split_whitespace() {
                                let id = ids[2..].parse::<usize>().unwrap();
                                mentions[id] += 1;
                            }
                        }
                    }
                },
                Offline(timestamp, id) => {
                    offline[id] = timestamp + 60;
                },
            }
        }
        mentions
    }
}

enum Event {
    Message(i32, String),
    Offline(i32, usize),
}

impl Event {
    fn new(mut e: Vec<String>) -> Self {
        let timestamp = e[1].parse().unwrap();
        match e[0].as_str() {
            "MESSAGE" => Message(timestamp, take(&mut e[2])),
            "OFFLINE" => Offline(timestamp, e[2].parse().unwrap()),
            _ => panic!("Bad event type."),
        }
    }
    fn sort_key(&self) -> (i32, i32) {
        match self {
            Message(ts, _) => (*ts, 1),
            Offline(ts, _) => (*ts, 0),
        }
    }
}