use actix_files::NamedFile;
use actix_multipart::form::MultipartForm;
use actix_web::{
    get, post,
    web::{self, ReqData},
    HttpResponse, Responder, Result,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    app_data::AppData,
    models::user_file::{get_all_user_files, get_user_file_by_file_id, save_user_file, UploadFile},
    utility::jwt_token::Claims,
};

pub fn user_file_config(config: &mut web::ServiceConfig) {
    let scope = web::scope("/file")
        .service(get_all_files)
        .service(save_file)
        .service(get_file_by_id);

    config.service(scope);
}

#[get("/")]
pub async fn get_all_files(
    data: web::Data<AppData>,
    req_user: Option<ReqData<Claims>>,
) -> impl Responder {
    let user_id = req_user.unwrap().id;

    let files = get_all_user_files(&data.pg_conn, &user_id).await;

    HttpResponse::Ok().json(json!(files))
}

#[post("/save")]
pub async fn save_file(
    data: web::Data<AppData>,
    req_user: Option<ReqData<Claims>>,
    form: MultipartForm<UploadFile>,
) -> impl Responder {
    /*
      part of the solution was referenced
      from a post on stackoverflow

      post : https://stackoverflow.com/a/75849261/13026811

      refere for more information
    */

    // 10 MB
    const MAX_FILE_SIZE: u64 = 1024 * 1024 * 10;
    // const MAX_FILE_COUNT: i32 = 1;

    // reject malformed requests
    match form.file.size {
        0 => return HttpResponse::BadRequest().finish(),
        length if length > MAX_FILE_SIZE.try_into().unwrap() => {
            return HttpResponse::BadRequest().body(format!(
                "The uploaded file is too large. Maximum size is {} bytes.",
                MAX_FILE_SIZE
            ));
        }
        _ => {}
    };

    let user_id = req_user.unwrap().id;

    let saved_file = save_user_file(&data.pg_conn, &data.data_path, &user_id, form.0).await;

    match saved_file {
        Some(saved_file) => HttpResponse::Ok().json(json!(saved_file)),
        None => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{file_id}")]
pub async fn get_file_by_id(
    file_id: web::Path<Uuid>,
    data: web::Data<AppData>,
    req_user: Option<ReqData<Claims>>,
) -> Result<NamedFile> {
    let user_id = req_user.unwrap().id;

    let file_path = get_user_file_by_file_id(&data.pg_conn, &data.data_path, &user_id, &file_id)
        .await
        .unwrap();

    Ok(NamedFile::open(file_path)?)
}
