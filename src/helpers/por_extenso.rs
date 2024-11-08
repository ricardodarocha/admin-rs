use std::collections::HashMap;

pub fn numero_por_extenso(valor: f64) -> String {
    let parte_inteira = valor.floor() as u64;
    let centavos = ((valor.fract() * 100.0).round()) as u64;
    
    let inteiros_extenso = formatar_grande_numero(parte_inteira);
    let centavos_extenso = formatar_centavos(centavos);
    
    format!(
        "{} {}",
        inteiros_extenso,
        centavos_extenso
    ).trim().to_string()
}

fn formatar_grande_numero(numero: u64) -> String {
    if numero == 0 {
        return "zero".to_string();
    }
    
    let milhoes = numero / 1_000_000;
    let milhares = (numero % 1_000_000) / 1_000;
    let centenas = numero % 1_000;
    
    let mut partes = vec![];
    
    if milhoes > 0 {
        partes.push(formatar_milhares(milhoes, "milhão", "milhões"));
    }
    if milhares > 0 {
        partes.push(formatar_milhares(milhares, "mil", "mil"));
    }
    if centenas > 0 || partes.is_empty() {
        partes.push(formatar_cem(centenas));
    }
    
    partes.join(", ")
}

fn formatar_centavos(centavos: u64) -> String {
    if centavos == 0 {
        "".to_string()
    } else {
        formatar_cem(centavos) + " "
    }
}

fn formatar_milhares(numero: u64, singular: &str, plural: &str) -> String {
    if numero == 1 {
        format!("{} ", singular)
    } else  {
    
        let _milhar = (numero / 1000) * 100;
        let resto = numero % 1000;
        
        if resto < 100 && resto > 0 {
            format!("{} {}", formatar_cem(numero), plural)
        } else if resto >= 100 {
            format!("{} {}", formatar_cem(numero), plural)
        } else  {
            format!("{} {}", formatar_cem(numero), plural)
        }
    }
}

fn formatar_cem(numero: u64) -> String {
    let mut numeros = HashMap::new();
    numeros.insert(1, "um");
    numeros.insert(2, "dois");
    numeros.insert(3, "três");
    numeros.insert(4, "quatro");
    numeros.insert(5, "cinco");
    numeros.insert(6, "seis");
    numeros.insert(7, "sete");
    numeros.insert(8, "oito");
    numeros.insert(9, "nove");
    numeros.insert(10, "dez");
    numeros.insert(11, "onze");
    numeros.insert(12, "doze");
    numeros.insert(13, "treze");
    numeros.insert(14, "quatorze");
    numeros.insert(15, "quinze");
    numeros.insert(16, "dezesseis");
    numeros.insert(17, "dezessete");
    numeros.insert(18, "dezoito");
    numeros.insert(19, "dezenove");
    numeros.insert(20, "vinte");
    numeros.insert(30, "trinta");
    numeros.insert(40, "quarenta");
    numeros.insert(50, "cinquenta");
    numeros.insert(60, "sessenta");
    numeros.insert(70, "setenta");
    numeros.insert(80, "oitenta");
    numeros.insert(90, "noventa");
    numeros.insert(100, "cem");
    numeros.insert(200, "duzentos");
    numeros.insert(300, "trezentos");
    numeros.insert(400, "quatrocentos");
    numeros.insert(500, "quinhentos");
    numeros.insert(600, "seiscentos");
    numeros.insert(700, "setecentos");
    numeros.insert(800, "oitocentos");
    numeros.insert(900, "novecentos");
    
    if let Some(&palavra) = numeros.get(&numero) {
        palavra.to_string()
    } else if numero > 100 && numero < 200 {
        let resto = numero % 100;
        format!("cento e {}", formatar_cem(resto))
    } else if numero > 100 {
        let centenas = (numero / 100) * 100;
        let resto = numero % 100;
        format!("{} e {}", numeros[&centenas], formatar_cem(resto))
    } else if numero > 20 {
        let dezenas = (numero / 10) * 10;
        let resto = numero % 10;
        format!("{} e {}", numeros[&dezenas], numeros[&resto])
    } else {
        "zero".to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(numero_por_extenso(0.), "zero");
    }

    #[test]
    fn test_unidades() {
        assert_eq!(numero_por_extenso(1.0), "um");
        assert_eq!(numero_por_extenso(9.0), "nove");
    }

    #[test]
    fn test_dezenas() {
        assert_eq!(numero_por_extenso(10.0), "dez");
        assert_eq!(numero_por_extenso(12.0), "doze");
        assert_eq!(numero_por_extenso(14.0), "quatorze");
        assert_eq!(numero_por_extenso(21.0), "vinte e um");
        assert_eq!(numero_por_extenso(94.0), "noventa e quatro");
    }

    #[test]
    fn test_dezenas_especiais() {
        assert_eq!(numero_por_extenso(11.0), "onze");
        assert_eq!(numero_por_extenso(13.0), "treze");
        assert_eq!(numero_por_extenso(19.0), "dezenove");
    }

    #[test]
    fn test_centenas() {
        assert_eq!(numero_por_extenso(100.0), "cem");
        assert_eq!(numero_por_extenso(200.0), "duzentos");
        assert_eq!(numero_por_extenso(999.0), "novecentos e noventa e nove");
    }

    #[test]
    fn test_milhares() {
        assert_eq!(numero_por_extenso(1_000.0), "mil");
        assert_eq!(numero_por_extenso(2_000.0), "dois mil");
        assert_eq!(numero_por_extenso(10_000.0), "dez mil");
        assert_eq!(numero_por_extenso(50_000.0), "cinquenta mil");
    }

    #[test]
    fn test_centenas_milhar() {
        assert_eq!(numero_por_extenso(100_000.0), "cem mil");
        assert_eq!(numero_por_extenso(200_000.0), "duzentos mil");
        assert_eq!(numero_por_extenso(999_000.0), "novecentos e noventa e nove mil");
    }

    #[test]
    fn test_milhares_e_centenas() {
        assert_eq!(numero_por_extenso(123_456.0), "cento e vinte e três mil, quatrocentos e cinquenta e seis");
        assert_eq!(numero_por_extenso(987_654.0), "novecentos e oitenta e sete mil, seiscentos e cinquenta e quatro");
        assert_eq!(numero_por_extenso(100_001.0), "cem mil, um");
    }

    #[test]
    fn test_numeros_complexos() {
        assert_eq!(numero_por_extenso(305.0), "trezentos e cinco");
        assert_eq!(numero_por_extenso(7_521.0), "sete mil, quinhentos e vinte e um");
        assert_eq!(numero_por_extenso(450_780.0), "quatrocentos e cinquenta mil, setecentos e oitenta");
    }
}