use std::{env, process::exit};

pub fn collect_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <op> <text>", args[0]);
        exit(1);
    }
    let op = &args[1];
    let text = &args[2];

    let res = match op.as_str() {
        "reverse" => text.chars().rev().collect::<String>(),
        "invert" => text
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    c.to_lowercase().to_string()
                } else {
                    c.to_uppercase().to_string()
                }
            })
            .collect::<String>(),
        "uppercase" => text.to_uppercase(),
        "no-spaces" => text
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>(),
        "leet" => text
            .chars()
            .map(|c| match c {
                'a' | 'A' => '4',
                'e' | 'E' => '3',
                'i' | 'I' => '1',
                'o' | 'O' => '0',
                's' | 'S' => '5',
                't' | 'T' => '7',
                _ => c,
            })
            .collect::<String>(),
        "acronym" => text
            .split_whitespace()
            .map(|word| word.chars().next().unwrap())
            .collect::<String>()
            .to_uppercase(),
        _ => {
            eprintln!("Invalid operation: {}", op);
            exit(1);
        }
    };
    res
}
