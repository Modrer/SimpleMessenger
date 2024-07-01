use std::ffi::OsStr;
use std::path::Path;

use rocket::fs::TempFile;
use uuid::Uuid;

pub struct FileInfo {
    pub name: String,
    pub content_type: String,
    //pub saving: dyn Future<Output=io::Result<()>>
}

pub async fn save_file(file: &mut TempFile<'_>) -> anyhow::Result<FileInfo> {
    let id = Uuid::new_v4().to_string();
    let path = Path::new("./static/");

    let file_name = file.raw_name().unwrap().dangerous_unsafe_unsanitized_raw().as_str();

    // Get extension of file name
    let extension = Path::new(file_name).extension().and_then(OsStr::to_str).unwrap();


    let file_name = format!("{}{}{}",&id,".",extension);

    // Build path to save file
    let path = path.join(&file_name);

    println!("{} {} {}", &file_name,&id,extension );

    let _saved = file.persist_to(path.clone()).await?;

    Ok(FileInfo {
        name: file_name,
        content_type: file
            .content_type()
            .ok_or(anyhow::Error::msg("no content"))?
            .to_string(),
    })
}
