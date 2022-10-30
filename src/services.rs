use crate::{
    errors::MyError,
    utils::{generate_hashval, Pool},
};
use actix_multipart::Multipart;
use actix_web::{
    delete, get, post,
    web::{Bytes, Data, Path},
    HttpRequest, HttpResponse,
};
use futures_util::TryStreamExt;
use log::debug;
use redis::Commands;
use tokio::io::AsyncWriteExt;

#[get("/{hash}")]
pub async fn get_message(
    _: HttpRequest,
    pool: Data<Pool>,
    path: Path<String>,
) -> Result<HttpResponse, MyError> {
    let mut connection = pool.get().map_err(|err| MyError::R2d2Error(err))?;
    let hash = path.into_inner();
    debug!("get message invoked with hash {}", hash);
    if let Some(message) = connection.get::<&str, Option<String>>(hash.as_str())? {
        debug!("get message succeeded with message {}", message);
        Ok(HttpResponse::Ok().body(message))
    } else {
        debug!("get message failed to find message");
        Ok(HttpResponse::BadRequest().body("message not found"))
    }
}

#[delete("/{hash}")]
pub async fn delete_message(
    _: HttpRequest,
    pool: Data<Pool>,
    path: Path<String>,
) -> Result<HttpResponse, MyError> {
    let mut connection = pool.get()?;
    let hash = path.into_inner();
    debug!("delete message with hash {}", hash);
    connection.del(hash.as_str())?;
    debug!("delete message success with hash {}", hash);
    Ok(HttpResponse::Ok().body("Deleted."))
}

#[post("/")]
pub async fn post_message(request_raw: Bytes, pool: Data<Pool>) -> Result<HttpResponse, MyError> {
    let mut connection = pool.get()?;
    let message = String::from_utf8(request_raw.to_vec())?;
    debug!("post message invoked with message {}", message);
    let hash = loop {
        let hashval = generate_hashval();
        if let None = connection.get::<&str, Option<String>>(hashval.as_str())? {
            connection.set(hashval.as_str(), message.as_str())?;
            break hashval;
        }
    };
    debug!("post message succeed with hash {}", hash);
    Ok(HttpResponse::Ok().body(hash))
}

pub async fn upload_file(
    mut payload: Multipart,
    dir: Data<String>,
) -> actix_web::Result<HttpResponse> {
    while let Some(mut field) = payload.try_next().await? {
        let content_decomposition = field.content_disposition();
        let filename = content_decomposition
            .get_filename()
            .map_or_else(|| generate_hashval(), sanitize_filename::sanitize);
        let path = std::path::PathBuf::from(dir.as_str()).join(filename.as_str());
        let mut f = tokio::fs::File::create(path).await?;
        while let Some(chunk) = field.try_next().await? {
            f.write_all(&chunk).await?;
        }
    }
    Ok(HttpResponse::Ok().body(format!("File uploaded.")))
}

#[delete("/{hashval}")]
pub async fn delete_file(_: HttpRequest, dir: Data<String>, path: Path<String>) -> HttpResponse {
    let mut base_path = std::path::PathBuf::from(dir.get_ref());
    let path = path.into_inner();
    let suffix = std::path::Path::new(path.as_str());
    base_path.push(suffix);

    tokio::fs::remove_file(base_path)
        .await
        .map_or(HttpResponse::BadRequest().body("File not exist."), |_| {
            HttpResponse::Ok().body("Deleted.")
        })
}
