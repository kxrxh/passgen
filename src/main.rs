use clap::Parser;
use passgen::{generate_password, generate_multiple_passwords, generate_password_with_strength, generate_smart_password, SmartPasswordMode, PasswordOptions, PasswordSafe};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Password};

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

    /// Interactive password building wizard
    #[arg(short = 'i', long)]
    interactive: bool,

    /// Generate phonetic password (easier to remember)
    #[arg(long)]
    phonetic: bool,

    /// Generate password using pattern (U=uppercase, L=lowercase, D=digit, S=symbol)
    #[arg(short = 'p', long, value_name = "PATTERN")]
    pattern: Option<String>,



    /// Create a new password safe
    #[arg(long, value_name = "FILE")]
    create_safe: Option<String>,

    /// Open an existing password safe
    #[arg(long, value_name = "FILE")]
    open_safe: Option<String>,

    /// Add current generated password to safe
    #[arg(long)]
    store: bool,

    /// List passwords in safe
    #[arg(long)]
    list_safe: bool,

    /// Service name for storage operations
    #[arg(long, value_name = "SERVICE")]
    service: Option<String>,

    /// Username for storage operations
    #[arg(long, value_name = "USERNAME")]
    username: Option<String>,
}

fn main() {
    let mut args = Args::parse();

    // If detailed is requested, enable strength analysis
    if args.detailed {
        args.strength = true;
    }

    // Handle interactive mode
    if args.interactive {
        let opts = run_interactive_wizard();
        let pwd_strength = generate_password_with_strength(&opts);
        print_detailed_strength(&pwd_strength);
        return;
    }

    // Handle smart password generation modes
    if args.phonetic {
        let opts = PasswordOptions {
            length: args.length,
            uppercase: args.uppercase,
            lowercase: args.lowercase,
            numbers: args.numbers,
            special: args.special,
            avoid_ambiguous: args.avoid_ambiguous,
        };
        let pwd_strength = generate_smart_password(SmartPasswordMode::Phonetic, &opts);
        print_detailed_strength(&pwd_strength);
        return;
    }

    if let Some(pattern) = args.pattern {
        let opts = PasswordOptions {
            length: args.length,
            uppercase: args.uppercase,
            lowercase: args.lowercase,
            numbers: args.numbers,
            special: args.special,
            avoid_ambiguous: args.avoid_ambiguous,
        };
        let pwd_strength = generate_smart_password(SmartPasswordMode::Pattern(pattern), &opts);
        print_detailed_strength(&pwd_strength);
        return;
    }



    // Handle password safe operations
    if let Some(file_path) = args.create_safe {
        let master_password = Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter master password for new safe")
            .interact()
            .unwrap();

        let confirm_password = Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Confirm master password")
            .interact()
            .unwrap();

        if master_password != confirm_password {
            eprintln!("❌ Passwords don't match!");
            return;
        }

        match PasswordSafe::create(&file_path, &master_password) {
            Ok(_) => println!("✅ Password safe created successfully at: {}", file_path),
            Err(e) => eprintln!("❌ Failed to create safe: {:?}", e),
        }
        return;
    }

    if let Some(file_path) = args.open_safe {
        let master_password = Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter master password")
            .interact()
            .unwrap();

        match PasswordSafe::open(&file_path, &master_password) {
            Ok(safe) => {
                if args.list_safe {
                    println!("🔐 Passwords in safe:");
                    for (id, password) in safe.list_passwords() {
                        println!("  ID: {} | Service: {} | Username: {} | Tags: {}",
                            id,
                            password.service,
                            password.username.as_ref().unwrap_or(&"N/A".to_string()),
                            password.tags.join(", ")
                        );
                    }
                } else {
                    println!("✅ Safe opened successfully!");
                    println!("   Use --list-safe to view stored passwords");
                }
            }
            Err(e) => eprintln!("❌ Failed to open safe: {:?}", e),
        }
        return;
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

        // Handle password storage if requested
        if args.store {
            if let (Some(service), Some(username)) = (args.service.clone(), args.username.clone()) {
                println!("💾 Storing password in safe...");
                println!("   Service: {}", service);
                println!("   Username: {}", username);

                let safe_file = "passwords.safe";
                let master_password = Password::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter master password for safe")
                    .interact()
                    .unwrap();

                match PasswordSafe::open(safe_file, &master_password) {
                    Ok(mut safe) => {
                        match safe.add_password(service, Some(username), password, None, vec![]) {
                            Ok(id) => println!("✅ Password stored with ID: {}", id),
                            Err(e) => eprintln!("❌ Failed to store password: {:?}", e),
                        }
                    }
                    Err(_) => {
                        // Try to create new safe
                        let create_new = Confirm::with_theme(&ColorfulTheme::default())
                            .with_prompt("Safe doesn't exist. Create new safe?")
                            .default(true)
                            .interact()
                            .unwrap();

                        if create_new {
                            match PasswordSafe::create(safe_file, &master_password) {
                                Ok(mut safe) => {
                                    match safe.add_password(
                                        args.service.unwrap(),
                                        args.username,
                                        password,
                                        None,
                                        vec![]
                                    ) {
                                        Ok(id) => println!("✅ Safe created and password stored with ID: {}", id),
                                        Err(e) => eprintln!("❌ Failed to store password: {:?}", e),
                                    }
                                }
                                Err(e) => eprintln!("❌ Failed to create safe: {:?}", e),
                            }
                        }
                    }
                }
            } else {
                eprintln!("❌ To store password, you must provide --service and --username");
            }
        }
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

fn run_interactive_wizard() -> PasswordOptions {
    println!("🔐 Welcome to PassGen Interactive Mode!");
    println!("Let's build your perfect password together.\n");

    // Get password length
    let length: usize = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Password length")
        .default(16)
        .interact_text()
        .unwrap();

    // Select character sets
    let character_sets = &[
        "Uppercase letters (A-Z)",
        "Lowercase letters (a-z)",
        "Numbers (0-9)",
        "Special characters (!@#$%^&*)",
    ];

    let defaults = &[true, true, true, true];
    let selected_sets: Vec<usize> = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select character sets to include")
        .items(character_sets)
        .defaults(defaults)
        .interact()
        .unwrap();

    // Parse selections
    let uppercase = selected_sets.contains(&0);
    let lowercase = selected_sets.contains(&1);
    let numbers = selected_sets.contains(&2);
    let special = selected_sets.contains(&3);

    // Ask about ambiguous characters
    let avoid_ambiguous = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Exclude ambiguous characters? (1, l, I, 0, O)")
        .default(true)
        .interact()
        .unwrap();

    println!("\n📋 Configuration Summary:");
    println!("   Length: {}", length);
    println!("   Character sets: {}",
        [
            if uppercase { Some("Uppercase") } else { None },
            if lowercase { Some("Lowercase") } else { None },
            if numbers { Some("Numbers") } else { None },
            if special { Some("Special") } else { None },
        ]
        .iter()
        .filter_map(|&x| x)
        .collect::<Vec<_>>()
        .join(", ")
    );
    println!("   Avoid ambiguous: {}", if avoid_ambiguous { "Yes" } else { "No" });

    PasswordOptions {
        length,
        uppercase,
        lowercase,
        numbers,
        special,
        avoid_ambiguous,
    }
}

