use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{web, HttpResponse};
use serde_json::json;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(MultipartForm)]
pub struct Upload {
    file: TempFile,
}

pub struct FileService;

impl FileService {
    pub async fn save_file(form: MultipartForm<Upload>) -> HttpResponse {
        const MAX_FILE_SIZE: usize = 1024 * 1024 * 100; // 100 MB
        match form.file.size {
            0 => return HttpResponse::BadRequest().finish(),
            length if length > MAX_FILE_SIZE => {
                return HttpResponse::BadRequest().body(format!(
                    "The uploaded file is too large. Maximum size is {} bytes.",
                    MAX_FILE_SIZE
                ));
            }
            _ => {}
        };
        let temp_file_path = form.file.file.path();
        let file_name: &str = form
            .file
            .file_name
            .as_ref()
            .map(|m| m.as_ref())
            .unwrap_or("null");

        let file_type = file_name.split(".").last().unwrap();
        let file_uuid = Uuid::new_v4();
        let new_filename = file_uuid.to_string() + file_type;

        let mut file_path = PathBuf::from("./upload/file");
        file_path.push(&sanitize_filename::sanitize(&new_filename));

        match std::fs::rename(temp_file_path, file_path) {
            Ok(_) => {
                let json = json!({"filename": new_filename});
                HttpResponse::Ok().json(web::Json(json))
            },
            Err(_) => {
                let msg = format!("Upload file {} error.", file_name);
                let json = json!({"message": msg});
                HttpResponse::InternalServerError().json(web::Json(json))
            },
        }
    }
}
