use minijinja::Error;

pub fn multimidia(base_path: &str, alt: &str, classe: &str) -> Result<String, Error> {
    // Definindo tamanhos e substituindo "<SIZE>" pelo valor
    let sizes = vec![320, 480, 768, 1024];
    let mut srcset = Vec::new();
    
    // Substitui "<SIZE>" em `base_path` por cada tamanho e monta o `srcset`
    for size in &sizes {
        let image_path = base_path.replace("<SIZE>", &size.to_string());
        srcset.push(format!("{} {}w", image_path, size));
    }

    // Construindo o `sizes` com base na largura da tela
    let sizes_attr = "(max-width: 320px) 280px, \
                      (max-width: 480px) 440px, \
                      (max-width: 768px) 720px, \
                      1024px";

    // Caminho principal (maior resolução) para `src`
    let src = base_path.replace("<SIZE>", "1024");

    // Gerando a tag completa `<img>`
    let img_tag = format!(
        r#"<img src="{}" class="{}"
             srcset="{}" 
             sizes="{}" 
             alt="{}" />"#,
        src,
        classe,
        srcset.join(", "),
        sizes_attr,
        alt
    );

    Ok(img_tag)
}
