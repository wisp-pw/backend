use crate::{prelude::*, repositories::file::FileType};
use axum::extract::Multipart;

// POST /upload
pub async fn post(
    Extension(file_save_service): Extension<SharedFileSaveService>,
    mut mp: Multipart,
) -> Response {
    let mut img_name: Option<String> = None;
    let mut img_data: Option<Vec<u8>> = None;

    while let Some(field) = mp.next_field().await.unwrap() {
        let name = field.name().unwrap();
        if name == "img" {
            let data = field.bytes().await.unwrap();
            img_data = Some(data.to_vec());
        } else if name == "original_file_name" {
            let name = field.text().await.unwrap();
            img_name = Some(name);
        }
    }

    if img_name.is_none() {
        return GenericResponse::bad_req_error_msg("No image name found.");
    }

    if img_data.is_none() {
        return GenericResponse::bad_req_error_msg("No image data found.");
    }

    let img_name = img_name.unwrap();

    file_save_service
        .write()
        .unwrap()
        .save(FileType::Image, img_name.clone(), img_data.unwrap().as_slice())
        .unwrap();

    GenericResponse::ok_msg(&("https://wisp.pw/".to_string() + &img_name))
}
