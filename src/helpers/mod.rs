pub fn url(path: &str) -> String {
    let base_url = std::env::var("BASE_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    let path = if path.starts_with('/') { &path[1..] } else { path };
    format!("{base_url}/{path}")
}

/// Função usada para anonimizar informações sensíveis LGPD
/// Exemplo de retorno 056.3******-22
pub fn anonimizar(field: &str) -> String {
    crate::infra::strings::anonimizar(field)
}

pub fn mascara(field: &str, masc: &str) -> String {
    // Remove todos os caracteres de pontuação de `field`, mantendo letras e números
    let clean_field: String = field.chars().filter(|c| c.is_alphanumeric()).collect();
    let mut result = String::new();
    let mut field_chars = clean_field.chars();
    
    for masc_char in masc.chars() {
        if masc_char == '*' {
            // Substitui pelo próximo caractere de clean_field se houver, caso contrário, finaliza
            if let Some(_field_char) = field_chars.next() {
                result.push('*');
            }
        } else if masc_char.is_ascii_punctuation() || masc_char == ' ' {
            // Adiciona caracteres de pontuação diretamente da máscara
            result.push(masc_char);
        } else if let Some(field_char) = field_chars.next() {
            // Adiciona o próximo caractere de clean_field quando não há '*'
            result.push(field_char);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mascara() {
        let test_cases = vec![
            // CPF: 000.000.000-00 -> 0.00-00
            ("1.2.3-09", "0.00-00", "1.23-09"),
            ("987 21", "0.00-00", "9.87-21"),

            // CNPJ: 00.000.000/0000-00 -> 00.00-00
            ("123499", "00.00-00", "12.34-99"),
            ("1122-81", "0.000-00", "1.122-81"),

            // CEP: 00000-000 -> 00-0
            ("12378", "000-00", "123-78"),
            ("87621", "00.0-00", "87.6-21"),

            // Telefone: (00) 0000-0000 -> (00) 00-00
            ("21 9832", "(00) 00-00", "(21) 98-32"),
            ("117--890", "(00) 00-00", "(11) 78-90"),

            // Outros exemplos
            ("876", "0.0-0", "8.7-6"),  
            ("1234", "**-**", "**-**"),      
            ("44444", "0***0", "4***4"),      
        ];

        for (field, masc, expected) in test_cases {
            let masked = mascara(field, masc);
            assert_eq!(masked, expected, "O resultado foi {}, mas esperava-se {}", masked, expected);
        }
    }
}


pub mod filter {
    use std::fmt::Write;
    use minijinja::Value;
    use time::{format_description, OffsetDateTime};
    use crate::infra::result::Result;

    pub fn fmt(value: f32, symbol: Option<&str>) -> Value {
        // Obtém o valor f32
        
        // Usa o símbolo fornecido ou "R$" como padrão
        let currency_symbol = symbol.unwrap_or("R$");
        
        // Formata a moeda com duas casas decimais
        let mut formatted = String::new();
        write!(formatted, "{} {:.2}", currency_symbol, value).unwrap();
        formatted = formatted.replacen(".", ",", 1);
        
        // Retorna o valor formatado como string
        Value::from(formatted)
    }

    pub fn fmt3(value: f32, symbol: Option<&str>) -> Value {
        // Obtém o valor f32
        
        // Usa o símbolo fornecido ou "R$" como padrão
        let currency_symbol = symbol.unwrap_or("");
        
        // Formata a moeda com duas casas decimais
        let mut formatted = String::new();
        write!(formatted, "{} {:.3}", currency_symbol, value).unwrap();
        formatted = formatted.replacen(".", ",", 1);
        
        // Retorna o valor formatado como string
        Value::from(formatted)
    }
    use minijinja::{Error, ErrorKind};
    pub fn format_filter(format_string: Value, value: f32) -> Result<Value, minijinja::Error> {
            // Converte os valores para string e f64, tratando erros
            let format_str = format_string
                .as_str()
                .ok_or_else(|| Error::new(ErrorKind::InvalidOperation, "Format string should be a str"))?;


            // Extrai a precisão do formato, ex: "%.2f" -> 2
            let precision = if format_str.contains("%.2f") {
                2
            } else {
                // Caso padrão, ou você pode adicionar outras verificações
                0
            };

            // Formata o número de acordo com a precisão extraída
            let mut formatted = String::new();
            if precision > 0 {
                write!(formatted, "{:.1$}", value, precision).unwrap();
            } else {
                write!(formatted, "{}", value).unwrap();
            }
        
        // Converte o ponto decimal para vírgula (opcional para formatos brasileiros)
        formatted = formatted.replacen(".", ",", 1);

        Ok(Value::from(formatted))
    }

    pub fn fmtdate(value: Value) -> Value {
        // Obtém o OffsetDateTime
        let datetime = value
            .as_object()
            .and_then(|obj| obj.downcast_ref::<OffsetDateTime>()).unwrap();
        
        // Define o formato brasileiro "dd/mm/yyyy"
        let format = format_description::parse("[day]/[month]/[year]").unwrap();
        let formatted = datetime.format(&format);

        match formatted {
            Ok(value) => { Value::from(value) } ,
            Err(_) => { Value::from(value) },
        }
        
        // Retorna a data formatada como string
    }

    pub fn fmtdateopt(value: Value) -> Value {
        // Tenta converter o Value para uma string
        if let Some(date_str) = value.as_str() {
            // Define o formato do OffsetDateTime a ser parseado
            let format_in = format_description::parse("[offset_hour sign:mandatory][year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]Z").unwrap();

            // Faz o parse da string no formato específico para OffsetDateTime
            if let Ok(datetime) = OffsetDateTime::parse(date_str, &format_in) {
                // Define o formato de saída "dd/mm/yyyy"
                let format = format_description::parse("[day]/[month]/[year]").unwrap();
                let formatted = datetime.format(&format).unwrap();
                return Value::from(formatted);
            }
        }
        
        // Se a conversão falhar, retorna uma string vazia
        Value::from("")
    }

    pub fn fmttime(value: Value) -> Value {
        // Obtém o OffsetDateTime
        let datetime = value
            .as_object()
            .and_then(|obj| obj.downcast_ref::<OffsetDateTime>()).unwrap();
        
        // Define o formato brasileiro "dd/mm/yyyy"
        let format = format_description::parse("[hour]:[minute] [second]s").unwrap();
        let formatted = datetime.format(&format);

        match formatted {
            Ok(value) => { Value::from(value) } ,
            Err(_) => { Value::from(value) },
        }
        
        // Retorna a data formatada como string
    }

    pub fn fmttimeopt(value: Value) -> Value {
        // Tenta converter o Value para uma string
        if let Some(date_str) = value.as_str() {
            // Define o formato do OffsetDateTime a ser parseado
            let format_in = format_description::parse("[offset_hour sign:mandatory][year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]Z").unwrap();

            // Faz o parse da string no formato específico para OffsetDateTime
            if let Ok(datetime) = OffsetDateTime::parse(date_str, &format_in) {
                // Define o formato de saída "dd/mm/yyyy"
                let format = format_description::parse("[hour]:[minute] [second]s").unwrap();
                let formatted = datetime.format(&format).unwrap();
                return Value::from(formatted);
            }
        }
        
        // Se a conversão falhar, retorna uma string vazia
        Value::from("")
        }
}