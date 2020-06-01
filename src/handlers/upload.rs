use actix_multipart::Multipart;
use actix_web::{error::ParseError, Error, HttpResponse};
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};

lazy_static::lazy_static! {
    static ref UPLOAD: String = std::env::var("UPLOAD").unwrap_or_else(|_| "./tmp/".to_owned());
}

pub async fn file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| ParseError::Incomplete)?;
        let filepath = format!("{}/{}", UPLOAD.as_str(), sanitize_filename::sanitize(&filename));
        println!("{}", filepath);
        let mut f = async_std::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}

pub fn form() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/upload" method="POST" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}