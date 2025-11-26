use std::io::{self, Write};

fn read_line_trimmed() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("failed to read");
    s.trim().to_string()
}

fn main() {
    // Integer tests
    print!("Enter an integer: ");
    io::stdout().flush().unwrap();
    let input = read_line_trimmed();
    let n: i64 = match input.parse() {
        Ok(v) => v,
        Err(_) => {
            println!("Not a valid integer.");
            return;
        }
    };

    if n > 0 {
        println!("{} is positive.", n);
    } else if n < 0 {
        println!("{} is negative.", n);
    } else {
        println!("You entered zero.");
    }

    if n % 2 == 0 {
        println!("{} is even.", n);
    } else {
        println!("{} is odd.", n);
    }

    if n % 3 == 0 {
        println!("{} is divisible by 3.", n);
    }

    if (1..=10).contains(&n) {
        println!("{} is between 1 and 10 (inclusive).", n);
    }

    if n.abs() > 100 {
        println!("Absolute value of {} is greater than 100.", n);
    }

    // String tests
    print!("\nEnter a word or phrase: ");
    io::stdout().flush().unwrap();
    let text = read_line_trimmed();

    if text.is_empty() {
        println!("You entered an empty string.");
    } else {
        if text.len() > 10 {
            println!("That is a long input (more than 10 chars).");
        } else {
            println!("That is a short input (10 chars or fewer).");
        }

        let lower = text.to_lowercase();
        if lower.starts_with('a') {
            println!("Starts with 'a' or 'A'.");
        }

        if lower.contains("test") {
            println!("Contains the substring \"test\".");
        }
    }

    // Yes/No test
    print!("\nDo you like Rust? (y/n): ");
    io::stdout().flush().unwrap();
    let ans = read_line_trimmed().to_lowercase();

    if ans == "y" || ans == "yes" {
        println!("Glad to hear you like Rust!");
        if n > 0 {
            println!("Positive number and pro-Rust — good combo.");
        }
    } else if ans == "n" || ans == "no" {
        println!("Maybe it will grow on you.");
        if n < 0 {
            println!("Negative number and not a fan of Rust — quirky!");
        }
    } else {
        println!("I'll take that as a maybe.");
    }
}