use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return help(&args);
    }

    match args[1].as_str() {
        "--help" | "-h" | "help" => help(&args),
        "--encode" | "-e" | "encode" => println!("{}", encode(&args[2], &args[3])),
        "--decode" | "-d" | "decode" => println!("{}", decode(&args[2], &args[3])),
        "--table" | "-t" | "table" => print_table(&create_table().0),
        _ => help(&args),
    }
}

fn encode(text: &str, key: &str) -> String {
    let (table, _) = create_table();
    text.chars()
        .enumerate()
        .map(|(pos, t)| {
            let key_pos = pos % key.len();
            let pair = &format!("{}-{}", t, &key[key_pos..key_pos + 1]);

            table[pair]
        })
        .collect::<String>()
}

fn decode(text: &str, key: &str) -> String {
    let (_, table) = create_table();
    text.chars()
        .enumerate()
        .map(|(pos, t)| {
            let key_pos = pos % key.len();
            let pair = &format!("{}-{}", t, &key[key_pos..key_pos + 1]);

            table[pair]
        })
        .collect::<String>()
}

fn create_table() -> (HashMap<String, char>, HashMap<String, char>) {
    // txt: a-zA-Z0-9
    // key: ascii from 32 to 126
    //
    //     0 1 2 3 4 ...
    //
    // _   0 1 2 3 4
    // !   1 2 3 4 5
    // "   2 3 4 5 6
    //
    let mut table = HashMap::new();
    let mut table_rev = HashMap::new();
    let txt = Txt::new().collect::<Vec<_>>();

    for (pos, t) in txt.iter().enumerate() {
        let mut idx = pos;
        for k in 32..127 {
            let pair = format!("{}-{}", t, k as u8 as char);
            let pair_rev = format!("{}-{}", txt[idx], k as u8 as char);
            table.insert(pair, txt[idx]);
            table_rev.insert(pair_rev, t.to_owned());
            idx = (idx + 1) % txt.len();
        }
    }

    (table, table_rev)
}

fn print_table(table: &HashMap<String, char>) {
    let txt = Txt::new().collect::<Vec<_>>();

    print!(" ");
    for t in txt.iter() {
        print!(" {}", t);
    }
    println!();

    for k in 32..127 {
        let k = k as u8 as char;
        print!("{} ", k);
        for t in txt.iter() {
            let pair = &format!("{}-{}", t, k as u8 as char);
            print!("{} ", table[pair]);
        }
        println!();
    }
}

struct Txt(u8);

impl Txt {
    fn new() -> Txt {
        Txt(48)
    }
}

impl Iterator for Txt {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        //           ascii
        // 0 - 9    48 - 57
        // A - Z    65 - 90
        // a - z    97 - 122
        //
        let c = self.0 as char;

        if self.0 == 57 {
            self.0 = 64;
        } else if self.0 == 90 {
            self.0 = 96;
        } else if self.0 == 123 {
            return None;
        }

        self.0 += 1;
        Some(c)
    }
}

fn help(args: &Vec<String>) {
    println!(
        r#"Usage: 
    {} --table
    {} --encode <text> <key>
    {} --decode <text> <key>"#,
        args[0], args[0], args[0]
    );
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};
    use regex::Regex;

    quickcheck::quickcheck! {
        fn valid(text: String, key: String) -> bool {
            let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();
            let text = re.replace_all(&text, "");

            let re = Regex::new(r"[^ -~]").unwrap();
            let key = re.replace_all(&key, "");

            if text == "" || key == "" {
                return true;
            }

            text == decode(&encode(&text, &key), &key)
        }
    }
}
