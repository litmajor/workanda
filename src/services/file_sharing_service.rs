
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use futures_util::StreamExt;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;
use chrono::Utc;

pub struct FileMetadata {
    pub id: String,
    pub filename: String,
    pub file_path: String,
    pub file_size: u64,
    pub mime_type: String,
    pub uploaded_by: i32,
    pub uploaded_at: i64,
}

pub struct FileSharingService {
    upload_dir: PathBuf,
}

impl FileSharingService {
    pub fn new(upload_dir: &str) -> Result<Self, std::io::Error> {
        let path = PathBuf::from(upload_dir);
        fs::create_dir_all(&path)?;
        Ok(Self { upload_dir: path })
    }

    pub async fn upload_file(
        &self,
        mut payload: Multipart,
        user_id: i32,
    ) -> Result<FileMetadata, Error> {
        let mut filename = String::new();
        let mut file_data = Vec::new();
        let mut mime_type = String::from("application/octet-stream");

        while let Some(item) = payload.next().await {
            let mut field = item?;
            
            let content_disposition = field.content_disposition();
            filename = content_disposition
                .get_filename()
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("file_{}", Uuid::new_v4()));

            if let Some(content_type) = field.content_type() {
                mime_type = content_type.to_string();
            }

            while let Some(chunk) = field.next().await {
                let data = chunk?;
                file_data.extend_from_slice(&data);
            }
        }

        let file_id = Uuid::new_v4().to_string();
        let sanitized_filename = sanitize_filename(&filename);
        let file_path = self.upload_dir.join(format!("{}_{}", file_id, sanitized_filename));

        let mut file = fs::File::create(&file_path)?;
        file.write_all(&file_data)?;

        let file_size = file_data.len() as u64;

        Ok(FileMetadata {
            id: file_id,
            filename: sanitized_filename,
            file_path: file_path.to_string_lossy().to_string(),
            file_size,
            mime_type,
            uploaded_by: user_id,
            uploaded_at: Utc::now().timestamp(),
        })
    }

    pub async fn download_file(&self, file_id: &str) -> Result<(Vec<u8>, String), Error> {
        let files = fs::read_dir(&self.upload_dir)?;
        
        for entry in files {
            let entry = entry?;
            let path = entry.path();
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str.starts_with(file_id) {
                    let data = fs::read(&path)?;
                    let filename = name_str.split('_').skip(1).collect::<Vec<_>>().join("_");
                    return Ok((data, filename));
                }
            }
        }

        Err(actix_web::error::ErrorNotFound("File not found"))
    }

    pub async fn delete_file(&self, file_id: &str) -> Result<(), Error> {
        let files = fs::read_dir(&self.upload_dir)?;
        
        for entry in files {
            let entry = entry?;
            let path = entry.path();
            if let Some(name) = path.file_name() {
                if name.to_string_lossy().starts_with(file_id) {
                    fs::remove_file(&path)?;
                    return Ok(());
                }
            }
        }

        Err(actix_web::error::ErrorNotFound("File not found"))
    }

    pub fn get_upload_dir(&self) -> &PathBuf {
        &self.upload_dir
    }
}

fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '-' | '_' => c,
            _ => '_',
        })
        .collect()
}
