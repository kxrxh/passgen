use clap::Parser;
use passgen::{generate_password, PasswordOptions};

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

    let opts = PasswordOptions {
        length: args.length,
        uppercase: args.uppercase,
        lowercase: args.lowercase,
        numbers: args.numbers,
        special: args.special,
        avoid_ambiguous: args.avoid_ambiguous,
    };

    let password = generate_password(&opts);
    println!("{}", password);
}

