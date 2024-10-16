use unicode_normalization::UnicodeNormalization;
use unicode_categories::UnicodeCategories;

pub fn tira_acento(input: &str) -> String {
    // 1. Remove acentos
    let without_accent: String = input.nfd()
        .filter(|c| !c.is_mark_nonspacing()) // Remove diacríticos
        .collect();

    // 2. Converte tudo para maiúsculo
    let uppercased = without_accent.to_uppercase();

    // 3. Remove espaços duplicados
    let normalized_spaces: String = uppercased
        .split_whitespace()  // Divide as palavras, ignorando espaços extras
        .collect::<Vec<_>>() // Coleta as palavras em um vetor
        .join(" ");          // Junta as palavras com um único espaço entre elas

    normalized_spaces
}