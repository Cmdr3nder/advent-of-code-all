use std::collections::HashSet;

use anyhow::Result;

use crate::day::Day;
use crate::input::get_input;

pub struct Day07;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum Sequence {
    AreaBroadcastAccessor(char, char),
    ByteAllocationBlock(char, char),
}

impl Sequence {
    fn flip(self) -> Self {
        match self {
            Sequence::AreaBroadcastAccessor(a, b) => Sequence::ByteAllocationBlock(b, a),
            Sequence::ByteAllocationBlock(b, a) => Sequence::AreaBroadcastAccessor(a, b),
        }
    }
}

impl Day for Day07 {
    fn main() -> Result<()> {
        let input_str = get_input(2016, 7)?;
        let mut tls_count = 0;
        let mut ssl_count = 0;
        for line in input_str.lines() {
            let chars: Vec<char> = line.chars().collect();

            // TLS Checks
            let mut inside = false; // Are we inside a hypertext sequence?
            let mut abba_inside = false;
            let mut abba_outside = false;
            'tls_check: for win in chars.windows(4) {
                inside = match win[0] {
                    '[' => true,
                    ']' => false,
                    _ => inside,
                };
                for i in 0..4 {
                    if win[i] == '[' || win[i] == ']' {
                        continue 'tls_check;
                    }
                }
                if win[0] == win[3] && win[1] == win[2] && win[0] != win[1] {
                    abba_inside = abba_inside || inside;
                    abba_outside = abba_outside || !inside;
                }
            }
            if abba_outside && !abba_inside {
                tls_count += 1;
            }

            // SSL Checks
            let mut inside = false; // Are we inside a hypertext sequence?
            let mut seen: HashSet<Sequence> = HashSet::new();
            'ssl_check: for win in chars.windows(3) {
                inside = match win[0] {
                    '[' => true,
                    ']' => false,
                    _ => inside,
                };
                for i in 0..3 {
                    if win[i] == '[' || win[i] == ']' {
                        continue 'ssl_check;
                    }
                }
                if win[0] == win[2] {
                    let seq = if inside {
                        Sequence::ByteAllocationBlock(win[0], win[1])
                    } else {
                        Sequence::AreaBroadcastAccessor(win[0], win[1])
                    };
                    let rev = seq.flip();
                    seen.insert(seq);
                    if seen.contains(&rev) {
                        ssl_count += 1;
                        break 'ssl_check;
                    }
                }
            }
        }
        println!("{tls_count} IPs support TLS");
        println!("{ssl_count} IPs support SSL");
        Ok(())
    }
}
