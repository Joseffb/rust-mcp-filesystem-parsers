use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use anyhow::Result;

#[derive(Deserialize)]
pub struct ParseFileRequest {
    pub path: String,
}

#[derive(Serialize)]
pub struct ParseFileResponse {
    pub content: String,
    pub metadata: serde_json::Value,
}

pub async fn parse_file(req: ParseFileRequest) -> Result<ParseFileResponse> {
    let path = &req.path;
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(ParseFileResponse {
        content: buf,
        metadata: serde_json::json!({"parser":"plain"}),
    })
}