use std::future;

use aws_sdk_s3::{primitives::ByteStream, types::ObjectCannedAcl};
use bytes::Bytes;
use color_eyre::{Result, eyre::eyre};
use envmnt::{ExpandOptions, ExpansionType};
use futures::{stream, StreamExt};
use serde_json::{Value, json};

// ATHENA_BUCKET=some-bucket cargo run

#[tokio::main]
async fn main() -> Result<()> {
    let jsons: Vec<Value> = (1..10).map(|i|{
        json!({
            "integer": i,
            "float": i as f32,
            "x2": i*i
        })
    }).collect();

    let s3 = MyS3::new().await?;

    for json in jsons {
        let json_str = serde_json::to_string(&json).unwrap();
        let key = format!("{}.json", json.pointer("/integer").unwrap());
        println!("Saving {}", &json_str);
        s3.save(json_str, &key).await?;
    }

    Ok(())
}

struct MyS3 {
    client: aws_sdk_s3::Client,
    bucket: String,
    prefix: String
}
impl MyS3 {
    pub async fn new() -> Result<Self> {
        let client = {
            let config = aws_config::from_env()
                .load()
                .await;
            aws_sdk_s3::Client::new(&config)
        };

        if !envmnt::exists("ATHENA_BUCKET") {
            return Err(eyre!(
                "You need to set the environment variable 'ATHENA_BUCKET'."
            ));
        }

        // Expand bucket environment variables as appropriate
        let mut options = ExpandOptions::new();
        options.expansion_type = Some(ExpansionType::Unix);
        let bucket = envmnt::expand("${ATHENA_BUCKET}", Some(options));
        
        Ok(Self { 
            client,
            bucket,
            prefix: "delete_me".into(),
        })
    }

    pub async fn save(&self, data: String, key: &str) -> Result<()> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&format!("{}/{}", &self.prefix, key))
            .body(ByteStream::from(Bytes::from(data)))
            .acl(ObjectCannedAcl::BucketOwnerFullControl)
            .send()
            .await?;

        Ok(())
    }
}

struct MyAthena{
    client: aws_sdk_athena::Client,
}
impl MyAthena {
    pub async fn new() -> Result<Self> {
        let client = {
            let config = aws_config::from_env()
                .load()
                .await;
            aws_sdk_athena::Client::new(&config)
        };

        Ok(Self {
            client
        })
    }
}