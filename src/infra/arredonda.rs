/// Função para arredondar um número de acordo com as normas da ABNT.
/// `valor` é o número a ser arredondado.
/// `casas_decimais` é a quantidade de casas decimais desejada.
#[allow(dead_code)]
fn arredondar_abnt(valor: f32, casas_decimais: u32) -> f32 {
    let multiplicador = 10_f32.powi(casas_decimais as i32);
    let valor_arredondado = (valor * multiplicador).round();

    let resto = valor_arredondado % 10.0;
    // let divisor = 10_f32.powi(casas_decimais as i32 + 1);

    let arredondado = if resto == 5.0 {
        let sem_o_ultimo_digito = valor_arredondado / 10.0;

        if sem_o_ultimo_digito % 2.0 == 0.0 {
            // Caso par
            valor_arredondado
        } else {
            // Caso ímpar
            valor_arredondado + 1.0
        }
    } else {
        valor_arredondado
    };

    arredondado / multiplicador
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arredondar_abnt() {
        // Exemplo 1: Algoritmo inferior a 5, o algarismo permanece.
        assert_eq!(arredondar_abnt(4.303, 2), 4.30);

        // Exemplo 2: Algoritmo maior que 5, soma-se uma unidade.
        assert_eq!(arredondar_abnt(15.4875, 2), 15.49);

        // Exemplo 3: Algarismo ímpar seguido de 5 e zeros, soma-se uma unidade.
        assert_eq!(arredondar_abnt(25.7750, 2), 25.78);

        // Exemplo 4: Algarismo par seguido de 5 e zeros, permanece sem alteração.
        assert_eq!(arredondar_abnt(31.7250, 2), 31.72);
    }
}

