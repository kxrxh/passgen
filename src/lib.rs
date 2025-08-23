use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use zxcvbn::zxcvbn;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
pub struct PasswordOptions {
    pub length: usize,
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub special: bool,
    pub avoid_ambiguous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordStrength {
    pub password: String,
    pub entropy_bits: f64,
    pub strength_score: u8, // 0-4 (0=very weak, 4=very strong)
    pub strength_label: String,
    pub crack_time_seconds: f64,
    pub crack_time_display: String,
    pub character_sets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordAnalysis {
    pub passwords: Vec<PasswordStrength>,
    pub count: usize,
    pub average_entropy: f64,
    pub average_strength_score: f64,
}

/// Generate a password using the provided options.
/// When no sets are effectively selected (after filtering), returns a default password.
pub fn generate_password(opts: &PasswordOptions) -> String {
    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const NUMBERS: &str = "0123456789";
    const SPECIAL: &str = "!@#$%^&*()-=_+[]{}|;:,.<>?";
    const AMBIGUOUS_UPPER: &[char] = &['I', 'O'];
    const AMBIGUOUS_LOWER: &[char] = &['l'];
    const AMBIGUOUS_NUMBERS: &[char] = &['0', '1'];

    let mut rng: StdRng = StdRng::from_entropy();

    // Build filtered sets based on options
    let build_filtered = |set: &str, ambiguous: &[char]| -> Vec<char> {
        if opts.avoid_ambiguous {
            set.chars().filter(|c| !ambiguous.contains(c)).collect()
        } else {
            set.chars().collect()
        }
    };

    let mut selected_sets: Vec<Vec<char>> = Vec::with_capacity(4);
    if opts.uppercase {
        let v = build_filtered(UPPERCASE, AMBIGUOUS_UPPER);
        if !v.is_empty() {
            selected_sets.push(v);
        }
    }
    if opts.lowercase {
        let v = build_filtered(LOWERCASE, AMBIGUOUS_LOWER);
        if !v.is_empty() {
            selected_sets.push(v);
        }
    }
    if opts.numbers {
        let v = build_filtered(NUMBERS, AMBIGUOUS_NUMBERS);
        if !v.is_empty() {
            selected_sets.push(v);
        }
    }
    if opts.special {
        let v = build_filtered(SPECIAL, &[]);
        if !v.is_empty() {
            selected_sets.push(v);
        }
    }

    // Generalized warning: length is less than the number of effective sets
    let effective_sets_count = selected_sets.len();
    if opts.length < effective_sets_count {
        eprintln!(
            "Warning: Password length is shorter than the number of selected character types; some types may be omitted."
        );
    }

    // Build the full pool once
    let mut all_chars_pool: Vec<char> = Vec::new();
    for set in &selected_sets {
        all_chars_pool.extend_from_slice(set);
    }

    if all_chars_pool.is_empty() {
        eprintln!(
            "Warning: No character sets selected or available after filtering. Returning a default password."
        );
        return String::from("DefaultPwd1!");
    }

    // Ensure at least one character from each selected set
    let mut password_chars: Vec<char> = Vec::with_capacity(opts.length.max(1));
    for set in &selected_sets {
        let idx = rng.gen_range(0..set.len());
        password_chars.push(set[idx]);
    }

    // If we already have enough chars, shuffle, truncate and return
    if password_chars.len() >= opts.length {
        password_chars.shuffle(&mut rng);
        password_chars.truncate(opts.length);
        return password_chars.into_iter().collect();
    }

    // Fill the remaining length with random picks from the pool
    let remaining_len = opts.length - password_chars.len();
    for _ in 0..remaining_len {
        let idx = rng.gen_range(0..all_chars_pool.len());
        password_chars.push(all_chars_pool[idx]);
    }

    // Final shuffle for randomness
    password_chars.shuffle(&mut rng);
    password_chars.truncate(opts.length);

    password_chars.into_iter().collect()
}

/// Calculate the entropy in bits for a given password length and character set size
pub fn calculate_entropy(length: usize, charset_size: usize) -> f64 {
    if charset_size <= 1 {
        return 0.0;
    }
    length as f64 * (charset_size as f64).log2()
}

/// Get the character sets used in the password options
pub fn get_character_sets(opts: &PasswordOptions) -> Vec<String> {
    let mut sets = Vec::new();
    if opts.uppercase {
        sets.push("uppercase".to_string());
    }
    if opts.lowercase {
        sets.push("lowercase".to_string());
    }
    if opts.numbers {
        sets.push("numbers".to_string());
    }
    if opts.special {
        sets.push("special".to_string());
    }
    sets
}

/// Calculate the total character set size based on options
pub fn get_charset_size(opts: &PasswordOptions) -> usize {
    let mut size = 0;
    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const NUMBERS: &str = "0123456789";
    const SPECIAL: &str = "!@#$%^&*()-=_+[]{}|;:,.<>?";

    let build_filtered = |set: &str, ambiguous: &[char]| -> usize {
        if opts.avoid_ambiguous {
            set.chars().filter(|c| !ambiguous.contains(c)).count()
        } else {
            set.len()
        }
    };

    const AMBIGUOUS_UPPER: &[char] = &['I', 'O'];
    const AMBIGUOUS_LOWER: &[char] = &['l'];
    const AMBIGUOUS_NUMBERS: &[char] = &['0', '1'];

    if opts.uppercase {
        size += build_filtered(UPPERCASE, AMBIGUOUS_UPPER);
    }
    if opts.lowercase {
        size += build_filtered(LOWERCASE, AMBIGUOUS_LOWER);
    }
    if opts.numbers {
        size += build_filtered(NUMBERS, AMBIGUOUS_NUMBERS);
    }
    if opts.special {
        size += build_filtered(SPECIAL, &[]);
    }

    size
}

/// Analyze password strength using zxcvbn
pub fn analyze_password_strength(password: &str, opts: &PasswordOptions) -> PasswordStrength {
    let entropy = calculate_entropy(password.len(), get_charset_size(opts));
    let character_sets = get_character_sets(opts);

    let estimate = zxcvbn(password, &[]).unwrap();
    let strength_score = estimate.score();
    let strength_label = match strength_score {
        0 => "Very Weak".to_string(),
        1 => "Weak".to_string(),
        2 => "Good".to_string(),
        3 => "Strong".to_string(),
        4 => "Very Strong".to_string(),
        _ => "Unknown".to_string(),
    };

    // For now, use a simplified crack time estimate based on entropy
    let crack_time_seconds = estimate_crack_time_from_entropy(entropy);
    let crack_time_display = format_crack_time(crack_time_seconds);

    PasswordStrength {
        password: password.to_string(),
        entropy_bits: entropy,
        strength_score,
        strength_label,
        crack_time_seconds,
        crack_time_display,
        character_sets,
    }
}

/// Estimate crack time based on entropy (simplified calculation)
fn estimate_crack_time_from_entropy(entropy: f64) -> f64 {
    // Assume 1e10 guesses per second (typical for offline attacks)
    let guesses_per_second = 1e10;
    let total_guesses = 2.0_f64.powi(entropy as i32);
    total_guesses / guesses_per_second
}

/// Format crack time in human-readable format
fn format_crack_time(seconds: f64) -> String {
    if seconds < 1.0 {
        "less than a second".to_string()
    } else if seconds < 60.0 {
        format!("{:.0} seconds", seconds)
    } else if seconds < 3600.0 {
        format!("{:.0} minutes", seconds / 60.0)
    } else if seconds < 86400.0 {
        format!("{:.0} hours", seconds / 3600.0)
    } else if seconds < 31536000.0 {
        format!("{:.0} days", seconds / 86400.0)
    } else {
        format!("{:.0} years", seconds / 31536000.0)
    }
}

/// Generate a single password with strength analysis
pub fn generate_password_with_strength(opts: &PasswordOptions) -> PasswordStrength {
    let password = generate_password(opts);
    analyze_password_strength(&password, opts)
}

/// Generate multiple passwords with strength analysis
pub fn generate_multiple_passwords(opts: &PasswordOptions, count: usize) -> PasswordAnalysis {
    let mut passwords = Vec::with_capacity(count);
    let mut total_entropy = 0.0;
    let mut total_strength = 0.0;

    for _ in 0..count {
        let pwd_strength = generate_password_with_strength(opts);
        total_entropy += pwd_strength.entropy_bits;
        total_strength += pwd_strength.strength_score as f64;
        passwords.push(pwd_strength);
    }

    let average_entropy = total_entropy / count as f64;
    let average_strength_score = total_strength / count as f64;

    PasswordAnalysis {
        passwords,
        count,
        average_entropy,
        average_strength_score,
    }
}


