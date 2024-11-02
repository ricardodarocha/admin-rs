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

pub fn anonimizar(texto: &str) -> String {
    let comprimento = texto.len();

    //percentual de anonimizacao
    const CFATOR: f64 = 0.35;

    let visivel = (comprimento as f64 * (1.0 - CFATOR)).round() as usize;
    let inicio_visivel = visivel / 2;
    let fim_visivel = visivel - inicio_visivel;

    // Se o texto for muito curto, anonimiza-o totalmente
    if comprimento <= inicio_visivel + fim_visivel {
        return "*".repeat(comprimento);
    }

    // Pega o início e o fim visíveis, e anonimiza o meio
    let inicio = &texto[..inicio_visivel];
    let fim = &texto[comprimento - fim_visivel..];
    let anonimizado = "*".repeat(comprimento - inicio_visivel - fim_visivel);

    format!("{}{}{}", inicio, anonimizado, fim)
}