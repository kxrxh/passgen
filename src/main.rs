use clap::Parser;
use rand::prelude::IteratorRandom;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Length of the password
    #[arg(short, long, default_value_t = 16)]
    length: usize,

    /// Include uppercase letters
    #[arg(short = 'u', long)]
    uppercase: bool,

    /// Include lowercase letters
    #[arg(short = 'w', long)]
    lowercase: bool,

    /// Include numbers
    #[arg(short = 'n', long)]
    numbers: bool,

    /// Include special characters
    #[arg(short = 's', long)]
    special: bool,

    /// Exclude ambiguous characters like 1, l, I, 0, O
    #[arg(short = 'a', long)]
    avoid_ambiguous: bool,
}

fn main() {
    let mut args = Args::parse();

    if !args.uppercase && !args.lowercase && !args.numbers && !args.special {
        args.uppercase = true;
        args.lowercase = true;
        args.numbers = true;
        args.special = true;
    }

    if args.length < 4 && (args.uppercase && args.lowercase && args.numbers && args.special) {
        eprintln!("Warning: Password length is too short to include all character types.");
    }

    let password = generate_password(&args);
    println!("{}", password);
}

fn generate_password(args: &Args) -> String {
    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const NUMBERS: &str = "0123456789";
    const SPECIAL: &str = "!@#$%^&*()-=_+[]{}|;:,.<>?";
    const AMBIGUOUS_UPPER: &[char] = &['I', 'O'];
    const AMBIGUOUS_LOWER: &[char] = &['l'];
    const AMBIGUOUS_NUMBERS: &[char] = &['0', '1'];

    let mut rng = thread_rng();
    let mut required_chars = Vec::new();
    let mut all_chars_pool = String::new();

    let mut add_set = |set: &str, should_add: bool, ambiguous_to_remove: &[char]| {
        if should_add {
            let filtered_set: String = if args.avoid_ambiguous {
                set.chars()
                    .filter(|c| !ambiguous_to_remove.contains(c))
                    .collect()
            } else {
                set.to_string()
            };

            if !filtered_set.is_empty() {
                if let Some(ch) = filtered_set.chars().choose(&mut rng) {
                    required_chars.push(ch);
                    all_chars_pool.push_str(&filtered_set);
                }
            }
        }
    };

    add_set(UPPERCASE, args.uppercase, AMBIGUOUS_UPPER);
    add_set(LOWERCASE, args.lowercase, AMBIGUOUS_LOWER);
    add_set(NUMBERS, args.numbers, AMBIGUOUS_NUMBERS);
    add_set(SPECIAL, args.special, &[]); // No ambiguous special chars

    if all_chars_pool.is_empty() {
        eprintln!(
            "Warning: No character sets selected or available after filtering. Returning a default password."
        );
        return String::from("DefaultPwd1!");
    }

    let mut password_chars = required_chars.clone();

    // Clamp length if required chars already exceed it
    if password_chars.len() >= args.length {
        password_chars.shuffle(&mut rng);
        password_chars.truncate(args.length);
        return password_chars.into_iter().collect();
    }

    let remaining_len = args.length - password_chars.len();

    // Fill the rest of the password length
    if remaining_len > 0 {
        let pool_vec: Vec<char> = all_chars_pool.chars().collect();
        if !pool_vec.is_empty() {
            password_chars.extend(pool_vec.choose_multiple(&mut rng, remaining_len).cloned());
        } else if password_chars.len() < args.length {
            let fill_len = args.length - password_chars.len();
            if !required_chars.is_empty() {
                // Now accessing the original required_chars
                password_chars.extend(required_chars.choose_multiple(&mut rng, fill_len).cloned());
            }
        }
    }

    // Shuffle all characters
    password_chars.shuffle(&mut rng);

    password_chars.truncate(args.length);

    password_chars.into_iter().collect()
}
