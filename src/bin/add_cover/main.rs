use anyhow::Result;
use minio::s3::{
    args::{BucketExistsArgs, MakeBucketArgs, StatObjectArgs, UploadObjectArgs},
    client::Client,
    creds::StaticProvider,
    http::BaseUrl,
};

#[tokio::main]
async fn main() -> Result<()> {
    let admin = std::env::var("MINIO_USER").unwrap_or("minioadmin".to_string());
    let password = std::env::var("MINIO_PASSWD").unwrap_or("minioadmin".to_string());
    let base_url = std::env::var("MINIO_ADDRESS")
        .unwrap_or("http://localhost:9000".to_string())
        .parse::<BaseUrl>()?;
    let bucket_name = std::env::var("MINIO_BUCKET").unwrap_or("covers".to_string());
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://127.0.0.1:5433/wuxia2kindle".to_string());

    let args = std::env::args().collect::<Vec<String>>();
    let static_provider = StaticProvider::new(&admin, &password, None);
    let client = Client::new(base_url, Some(Box::new(static_provider)), None, None)?;

    let (name, ext) = &args[1]
        .split('/')
        .last()
        .expect("Invalid file path")
        .split_once('.')
        .expect("Invalid file path");

    let id = name.parse::<i32>()?;
    let filename = format!("{}.{}", id, ext);

    let bucket_exists = client
        .bucket_exists(&BucketExistsArgs::new(&bucket_name).unwrap())
        .await?;
    if !bucket_exists {
        client
            .make_bucket(&MakeBucketArgs::new(&bucket_name).unwrap())
            .await?;
    }

    let file_stat = client
        .stat_object(&StatObjectArgs::new(&bucket_name, &filename).unwrap())
        .await;

    match file_stat {
        Ok(file) => {
            println!("File already exists");
            println!("File: {:?}", file.etag);
            save_in_db(db_url, format!("{}/{}", bucket_name, filename), id).await?;
            Ok(())
        }
        Err(err) => {
            if err.to_string().contains("NoSuchKey") {
                println!("Uploading file");
            } else {
                return Err(err.into());
            }

            let mut upload_args = UploadObjectArgs::new(&bucket_name, &filename, &args[1]).unwrap();
            let response = client.upload_object(&mut upload_args).await?;

            println!("File uploaded: {:?}", response.etag);
            save_in_db(db_url, format!("{}/{}", bucket_name, filename), id).await?;
            Ok(())
        }
    }
}

async fn save_in_db(database_url: String, path: String, id: i32) -> Result<()> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    sqlx::query!("UPDATE books SET cover = $1 WHERE id = $2", path, id)
        .execute(&pool)
        .await?;
    pool.close().await;
    Ok(())
}
