use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use image::{DynamicImage, GenericImageView, ImageFormat};
use image::imageops::FilterType;
use base64::encode;
use std::fs::File;
use std::io::Read;

// Função para verificar se o arquivo é uma imagem válida
fn is_valid_image(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("png") | Some("jpg") | Some("jpeg") | Some("bmp") => true,
        _ => false,
    }
}

// Função para normalizar a imagem
fn normalize_image(image: DynamicImage, carteira: &str) -> io::Result<()> {
    let (width, height) = image.dimensions();
    let new_height = 500 * height / width;

    // Redimensionar a imagem para 500xA
    let resized_image = image.resize(500, new_height, FilterType::Lanczos3);

    // Verificar se a imagem é horizontal
    if width > height {
        println!("Aviso: A imagem da carteira {} pode estar invertida.", carteira);
    }

    // Verificar resolução mínima e máxima
    if width < 150 || height < 300 || width > 600 || height > 1200 {
        println!("Aviso: A imagem da carteira {} está fora das resoluções esperadas.", carteira);
    }

    // Verificar se a altura não excede duas vezes a largura
    if height > 2 * width {
        println!("Aviso: A altura da imagem da carteira {} excede duas vezes a largura.", carteira);
    }

    // Salvar a imagem como BMP sem compactação
    let output_path = format!("img/sinc_{}.bmp", carteira);
    resized_image.save_with_format(&output_path, ImageFormat::Bmp)?;

    // Verificar o tamanho do arquivo
    let metadata = fs::metadata(&output_path)?;
    if metadata.len() > 100 * 1024 {
        println!("Aviso: A imagem da carteira {} excede 100KB.", carteira);
    }

    // Converter a imagem para Base64 e salvar em um arquivo .txt
    let mut img_file = File::open(&output_path)?;
    let mut buffer = Vec::new();
    img_file.read_to_end(&mut buffer)?;
    let base64_str = encode(&buffer);
    let txt_path = format!("img/sinc_{}.txt", carteira);
    let mut txt_file = File::create(&txt_path)?;
    writeln!(txt_file, "{}", base64_str)?;

    println!("Base64 salvo em: {}", txt_path);

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Por favor, forneça pelo menos um número de carteira.");
        return Ok(());
    }

    // Processar em blocos de 10 carteiras
    for chunk in args.chunks(10) {
        for carteira in chunk {
            // Procurar por imagens na pasta img/ com o código da carteira
            let pattern = format!("img/{}*.*", carteira);
            for entry in glob::glob(&pattern).expect("Falha ao ler o padrão") {
                match entry {
                    Ok(path) if is_valid_image(&path) => {
                        let img = image::open(&path)?;
                        normalize_image(img, &carteira)?;
                    }
                    Ok(_) => {
                        println!("Arquivo não é uma imagem válida.");
                    }
                    Err(e) => {
                        println!("Erro: {}", e);
                    }
                }
            }
        }
    }

    Ok(())
}
