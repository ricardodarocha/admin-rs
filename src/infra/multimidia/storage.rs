// src/infrastructure/file_storage/salvar_imagem.rs

use actix_web::web;
use actix_multipart::Field;
use futures_util::StreamExt;
use nanoid::nanoid;
use serde::Serialize;
use tokio::fs;
use tokio::io::AsyncWriteExt as _;
use crate::infra::error::Error;
use crate::infra::result::Result;
use log::{debug, error, info};

use std::path::Path;
use image::{ DynamicImage, imageops::FilterType };

const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024; // 10 MB

#[derive(Clone, Serialize, Debug)]
pub struct FileStorage{
    pub path: String,
    pub avatar: String,
    pub extension: String,
    pub resources: Vec<String>,
}

impl FileStorage {
    /// Salva uma imagem no disco e retorna o caminho salvo
    pub async fn salvar_imagem(mut field: Field) -> Result<Self> {
        debug!("{:?}", field);
        let mut arquivos: Vec<String> = vec!();
        let content_type = field.content_type().expect("Erro ao identificar o formato da mídia");
        if content_type != &mime::IMAGE_JPEG && content_type != &mime::IMAGE_PNG {
            return Err(Error::Str("Apenas imagens JPEG e PNG são suportadas."));
        }

          let extension = match content_type.as_ref() {
            "image/jpeg" => "jpg",
            "image/png" => "png",
            _ => "x",
        };
        info!("processando {}", content_type.as_ref());

        let file_name = format!("{}", nanoid!(12));
        let now = time::OffsetDateTime::now_utc();
        let year = now.year();
        let month = now.month() as u8;

        let dir_path = format!("{base}/{y}/{m:02}", base = "storage/images", y = year, m = month);

        if !Path::new(&dir_path).exists() {
            fs::create_dir_all(&dir_path).await.unwrap();
            info!("Diretório criado {}", &dir_path)
        }

        let destination: String = format!(
                "{}/{}.{}",
                dir_path,
                file_name,
                extension
            );
            info!("  lendo {}", &destination.clone());

        let mut file: fs::File = fs::File::create(&destination.clone()).await.unwrap();
        let mut total_size = 0;
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            total_size += data.len();
            if total_size > MAX_IMAGE_SIZE {
                error!("❌ A imagem excede 10MB [{total_size}] ");
                return Err(Error::Str("A imagem excede o tamanho máximo de 10 MB."));
            }
            let _ = file.write_all(&data).await.unwrap();
        }

    let uploaded_img: DynamicImage = image::open(&destination).unwrap().clone();
    let _ = fs::remove_file(&Path::new(&destination)).await.unwrap();
    let tamanhos = [320_u32, 480, 768, 1024];
    for tamanho in tamanhos.into_iter() {
        let arquivo = format!("{}/{}_{}w.{}", dir_path.clone(), file_name, tamanho, extension);
        arquivos.push(arquivo.clone());
        
        let resized_img = uploaded_img.resize(tamanho, u32::MAX, FilterType::Gaussian);

        // redimensiona a Imagem
        web::block(move  || async move {
            let _ = resized_img.save(&arquivo);
            // Redimensiona para os tamanhos especificados
            
        }).await.unwrap().await;

    }
    Ok(FileStorage{
        path: dir_path,
        avatar: file_name,
        extension: extension.to_string(),
        resources: arquivos,
    })
    }
}
