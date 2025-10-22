use std::{path::Path, io};
use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use rust_mcp_sdk::schema::{CallToolResult, TextContent, schema_utils::CallToolError};
use crate::fs_service::FileSystemService;
use tree_magic_mini;

#[mcp_tool(
    name = "parse_pdf_file",
    title = "Parse PDF File",
    description = "Parses .pdf files into extracted text output.",
    destructive_hint = false,
    idempotent_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, JsonSchema)]
pub struct ParsePdfFile {
    pub path: String,
}

impl ParsePdfFile {
    pub async fn run_tool(
        params: Self,
        _context: &FileSystemService, // unused but required for tool signature
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let path = Path::new(&params.path);
        let mime_type = tree_magic_mini::from_filepath(path).unwrap_or("application/octet-stream");

        if mime_type != "application/pdf" {
            return Err(CallToolError::new(io::Error::new(
                io::ErrorKind::Other,
                format!("Invalid MIME type: expected application/pdf, got {}", mime_type),
            )));
        }

        // Placeholder parse logic — integrate real parser later
        let output = format!("(Parsed PDF text from {:?})", path);

        Ok(CallToolResult::text_content(vec![TextContent::from(output)]))
    }
}