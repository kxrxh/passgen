use clap::Parser;
use passgen::{generate_password, generate_multiple_passwords, generate_password_with_strength, PasswordOptions};

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

    /// Number of passwords to generate
    #[arg(short, long, default_value_t = 1)]
    count: usize,

    /// Show password strength analysis
    #[arg(short = 't', long)]
    strength: bool,

    /// Output in JSON format
    #[arg(short = 'j', long)]
    json: bool,

    /// Show detailed analysis (implies --strength)
    #[arg(short = 'd', long)]
    detailed: bool,
}

fn main() {
    let mut args = Args::parse();

    // If detailed is requested, enable strength analysis
    if args.detailed {
        args.strength = true;
    }

    // Set default character sets if none specified
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

    // Handle JSON output
    if args.json {
        let analysis = generate_multiple_passwords(&opts, args.count);
        let json_output = serde_json::to_string_pretty(&analysis).unwrap();
        println!("{}", json_output);
        return;
    }

    // Handle multiple passwords without JSON
    if args.count > 1 {
        let analysis = generate_multiple_passwords(&opts, args.count);

        for (i, pwd_strength) in analysis.passwords.iter().enumerate() {
            if args.count > 1 {
                println!("Password {}: {}", i + 1, pwd_strength.password);
            } else {
                println!("{}", pwd_strength.password);
            }

            if args.strength {
                print_strength_info(pwd_strength, args.detailed);
                println!();
            }
        }

        if args.count > 1 {
            println!("Generated {} passwords", analysis.count);
            println!("Average entropy: {:.1} bits", analysis.average_entropy);
            println!("Average strength score: {:.1}/4", analysis.average_strength_score);
        }
        return;
    }

    // Handle single password
    if args.strength {
        let pwd_strength = generate_password_with_strength(&opts);
        if args.detailed {
            print_detailed_strength(&pwd_strength);
        } else {
            println!("{}", pwd_strength.password);
            print_strength_info(&pwd_strength, false);
        }
    } else {
        let password = generate_password(&opts);
        println!("{}", password);
    }
}

fn print_strength_info(pwd_strength: &passgen::PasswordStrength, detailed: bool) {
    println!("  Entropy: {:.1} bits", pwd_strength.entropy_bits);
    println!("  Strength: {} ({})", pwd_strength.strength_label, pwd_strength.strength_score);
    println!("  Crack time: {}", pwd_strength.crack_time_display);

    if detailed {
        println!("  Character sets: {}", pwd_strength.character_sets.join(", "));
    }
}

fn print_detailed_strength(pwd_strength: &passgen::PasswordStrength) {
    println!("Password: {}", pwd_strength.password);
    println!("Length: {} characters", pwd_strength.password.len());
    println!("Entropy: {:.1} bits", pwd_strength.entropy_bits);
    println!("Strength: {} ({}/4)", pwd_strength.strength_label, pwd_strength.strength_score);
    println!("Estimated crack time: {}", pwd_strength.crack_time_display);
    println!("Character sets used: {}", pwd_strength.character_sets.join(", "));

    // Add some guidance based on strength
    match pwd_strength.strength_score {
        0..=1 => println!("⚠️  Warning: This password is weak and should not be used for sensitive accounts"),
        2 => println!("ℹ️  This password has moderate strength"),
        3 => println!("✅ This password has good strength"),
        4 => println!("🔒 This password has excellent strength"),
        _ => {}
    }
}

