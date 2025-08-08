use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

#[derive(Clone, Debug, Default)]
pub struct PasswordOptions {
    pub length: usize,
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub special: bool,
    pub avoid_ambiguous: bool,
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


