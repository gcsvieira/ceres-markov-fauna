use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Attach {
    Before, // Suffixes (e.g., word.)
    After,  // Prefixes (e.g., $word)
    Both,   // Infixes/Glue (e.g., word_word)
}

pub(crate) fn get_punctuation_map() -> HashMap<char, Attach> {
    let mut puncts = HashMap::new();

    // --- ATTACH BEFORE (Trailing) ---
    let before = [
        '.', ',', ';', ':', '!', '?',
        ')', ']', '}', '>',
        '°', '%', '…', '”', '’'
    ];
    for c in before { puncts.insert(c, Attach::Before); }

    // --- ATTACH AFTER (Leading) ---
    let after = [
        '(', '[', '{', '<',
        '$', '€', '£', '¥', '₹',
        '@', '#', '¿', '¡', '“', '‘'
    ];
    for c in after { puncts.insert(c, Attach::After); }

    // --- ATTACH BOTH (Glue / Infix) ---
    // These typically connect two alphanumeric parts
    let both = [
        '_', '-', '–', '—', '/', '\\',
        '&', '~', '*', '≈', '+', '=',
        '|', '·', '^'
    ];
    for c in both { puncts.insert(c, Attach::Both); }

    puncts
}