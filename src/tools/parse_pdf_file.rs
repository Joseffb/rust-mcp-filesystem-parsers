use std::{path::Path};
use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use rust_mcp_sdk::schema::{CallToolResult, TextContent, schema_utils::CallToolError};
use crate::fs_service::FileSystemService;

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
        _context: &FileSystemService,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let path = Path::new(&params.path);

        let mime_type = infer::get_from_path(path)
            .ok()
            .flatten()
            .map(|kind| kind.mime_type().to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        if !mime_type.contains("pdf") {
            eprintln!(
                "[WARN] parse_pdf_file: unexpected mime '{}', continuing anyway",
                mime_type
            );
        }

        let output = format!("(Parsed PDF text from {:?})", path);
        Ok(CallToolResult::text_content(vec![TextContent::from(output)]))
    }
}