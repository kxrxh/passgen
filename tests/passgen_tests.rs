use passgen::{generate_password, generate_password_with_strength, generate_multiple_passwords, PasswordOptions, calculate_entropy, get_charset_size};

fn has_uppercase(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_uppercase())
}
fn has_lowercase(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_lowercase())
}
fn has_digit(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_digit())
}
fn has_special(s: &str) -> bool {
    let specials = "!@#$%^&*()-=_+[]{}|;:,.<>?";
    s.chars().any(|c| specials.contains(c))
}

#[test]
fn generates_requested_length_uppercase_only() {
    let opts = PasswordOptions {
        length: 24,
        uppercase: true,
        lowercase: false,
        numbers: false,
        special: false,
        avoid_ambiguous: false,
    };
    let pwd = generate_password(&opts);
    assert_eq!(pwd.len(), 24);
    assert!(pwd.chars().all(|c| c.is_ascii_uppercase()));
}

#[test]
fn includes_each_selected_type_when_length_sufficient() {
    let opts = PasswordOptions {
        length: 32,
        uppercase: true,
        lowercase: true,
        numbers: true,
        special: true,
        avoid_ambiguous: false,
    };
    let pwd = generate_password(&opts);
    assert_eq!(pwd.len(), 32);
    assert!(has_uppercase(&pwd));
    assert!(has_lowercase(&pwd));
    assert!(has_digit(&pwd));
    assert!(has_special(&pwd));
}

#[test]
fn avoid_ambiguous_filters_characters() {
    let opts = PasswordOptions {
        length: 64,
        uppercase: true,
        lowercase: true,
        numbers: true,
        special: false,
        avoid_ambiguous: true,
    };
    let pwd = generate_password(&opts);
    // none of these should appear when avoid_ambiguous is set
    for bad in ['I', 'O', 'l', '0', '1'] {
        assert!(!pwd.contains(bad));
    }
}

#[test]
fn only_special_characters_when_selected() {
    let opts = PasswordOptions {
        length: 18,
        uppercase: false,
        lowercase: false,
        numbers: false,
        special: true,
        avoid_ambiguous: false,
    };
    let pwd = generate_password(&opts);
    let specials = "!@#$%^&*()-=_+[]{}|;:,.<>?";
    assert_eq!(pwd.len(), 18);
    assert!(pwd.chars().all(|c| specials.contains(c)));
}

#[test]
fn zero_length_is_allowed_and_empty() {
    let opts = PasswordOptions {
        length: 0,
        uppercase: true,
        lowercase: true,
        numbers: true,
        special: true,
        avoid_ambiguous: false,
    };
    let pwd = generate_password(&opts);
    assert_eq!(pwd.len(), 0);
}

#[test]
fn no_sets_selected_returns_default_password() {
    let opts = PasswordOptions {
        length: 16,
        uppercase: false,
        lowercase: false,
        numbers: false,
        special: false,
        avoid_ambiguous: false,
    };
    let pwd = generate_password(&opts);
    assert_eq!(pwd, "DefaultPwd1!");
}

#[test]
fn short_length_still_within_selected_union() {
    let opts = PasswordOptions {
        length: 2,
        uppercase: true,
        lowercase: true,
        numbers: true,
        special: true,
        avoid_ambiguous: false,
    };
    let pwd = generate_password(&opts);
    assert_eq!(pwd.len(), 2);
    // Cannot easily reconstruct the union here without duplicating impl.
    // Just ensure all chars are ascii and part of allowed broad categories.
    let allowed = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-=_+[]{}|;:,.<>?";
    assert!(pwd.chars().all(|c| allowed.contains(c)));
}

#[test]
fn calculate_entropy_correctness() {
    // Test entropy calculation: 16 chars from 26 letters = log2(26^16) bits
    let entropy = calculate_entropy(16, 26);
    let expected = 16.0 * (26.0_f64).log2();
    assert!((entropy - expected).abs() < 0.001);
}

#[test]
fn get_charset_size_with_all_options() {
    let opts = PasswordOptions {
        length: 16,
        uppercase: true,
        lowercase: true,
        numbers: true,
        special: true,
        avoid_ambiguous: false,
    };
    let size = get_charset_size(&opts);
    // 26 uppercase + 26 lowercase + 10 numbers + 26 special = 88
    assert_eq!(size, 88);
}

#[test]
fn get_charset_size_with_avoid_ambiguous() {
    let opts = PasswordOptions {
        length: 16,
        uppercase: true,
        lowercase: true,
        numbers: true,
        special: false,
        avoid_ambiguous: true,
    };
    let size = get_charset_size(&opts);
    // 24 uppercase (I,O removed) + 25 lowercase (l removed) + 8 numbers (0,1 removed) = 57
    assert_eq!(size, 57);
}

#[test]
fn generate_password_with_strength_returns_valid_data() {
    let opts = PasswordOptions {
        length: 20,
        uppercase: true,
        lowercase: true,
        numbers: true,
        special: true,
        avoid_ambiguous: false,
    };
    let pwd_strength = generate_password_with_strength(&opts);

    assert_eq!(pwd_strength.password.len(), 20);
    assert!(pwd_strength.entropy_bits > 0.0);
    assert!(pwd_strength.strength_score <= 4);
    assert!(!pwd_strength.strength_label.is_empty());
    assert!(!pwd_strength.character_sets.is_empty());
    assert!(pwd_strength.character_sets.contains(&"uppercase".to_string()));
    assert!(pwd_strength.character_sets.contains(&"lowercase".to_string()));
    assert!(pwd_strength.character_sets.contains(&"numbers".to_string()));
    assert!(pwd_strength.character_sets.contains(&"special".to_string()));
}

#[test]
fn generate_multiple_passwords_correct_count() {
    let opts = PasswordOptions {
        length: 12,
        uppercase: true,
        lowercase: true,
        numbers: false,
        special: false,
        avoid_ambiguous: false,
    };
    let analysis = generate_multiple_passwords(&opts, 5);

    assert_eq!(analysis.count, 5);
    assert_eq!(analysis.passwords.len(), 5);
    assert!(analysis.average_entropy > 0.0);
    assert!(analysis.average_strength_score >= 0.0 && analysis.average_strength_score <= 4.0);

    // Check that all passwords have the correct length and only use expected character sets
    for pwd_strength in &analysis.passwords {
        assert_eq!(pwd_strength.password.len(), 12);
        assert!(pwd_strength.character_sets.contains(&"uppercase".to_string()));
        assert!(pwd_strength.character_sets.contains(&"lowercase".to_string()));
        assert!(!pwd_strength.character_sets.contains(&"numbers".to_string()));
        assert!(!pwd_strength.character_sets.contains(&"special".to_string()));
    }
}

#[test]
fn strength_analysis_for_weak_password() {
    let opts = PasswordOptions {
        length: 4,
        uppercase: false,
        lowercase: true,
        numbers: false,
        special: false,
        avoid_ambiguous: false,
    };
    let pwd_strength = generate_password_with_strength(&opts);

    assert_eq!(pwd_strength.password.len(), 4);
    assert!(pwd_strength.entropy_bits < 30.0); // Should be quite low entropy
    assert!(pwd_strength.strength_score <= 2); // Should be weak or very weak
}

#[test]
fn strength_analysis_for_strong_password() {
    let opts = PasswordOptions {
        length: 32,
        uppercase: true,
        lowercase: true,
        numbers: true,
        special: true,
        avoid_ambiguous: true,
    };
    let pwd_strength = generate_password_with_strength(&opts);

    assert_eq!(pwd_strength.password.len(), 32);
    assert!(pwd_strength.entropy_bits > 100.0); // Should be high entropy
    assert!(pwd_strength.strength_score >= 3); // Should be strong or very strong
}

#[test]
fn json_serialization_works() {
    let opts = PasswordOptions {
        length: 16,
        uppercase: true,
        lowercase: true,
        numbers: true,
        special: true,
        avoid_ambiguous: false,
    };
    let analysis = generate_multiple_passwords(&opts, 2);

    // Test that serialization works (this would panic if there are serialization issues)
    let json = serde_json::to_string(&analysis).unwrap();
    assert!(json.contains("password"));
    assert!(json.contains("entropy_bits"));
    assert!(json.contains("strength_score"));
}



