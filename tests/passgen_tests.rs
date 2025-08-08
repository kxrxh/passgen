use passgen::{generate_password, PasswordOptions};

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



