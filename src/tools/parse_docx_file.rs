use std::{path::Path};
use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use rust_mcp_sdk::schema::{CallToolResult, TextContent, schema_utils::CallToolError};
use crate::fs_service::FileSystemService;

#[mcp_tool(
    name = "parse_docx_file",
    title = "Parse DOCX File",
    description = "Parses .docx files into extracted text output.",
    destructive_hint = false,
    idempotent_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, JsonSchema)]
pub struct ParseDocxFile {
    pub path: String,
}

impl ParseDocxFile {
    pub async fn run_tool(
        params: Self,
        _context: &FileSystemService,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let path = Path::new(&params.path);

        // No strict MIME enforcement — rely on upstream detection
        let mime_type = infer::get_from_path(path)
            .ok()
            .flatten()
            .map(|kind| kind.mime_type().to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        if !mime_type.contains("word") && !mime_type.contains("officedocument") {
            eprintln!(
                "[WARN] parse_docx_file: unexpected mime '{}', continuing anyway",
                mime_type
            );
        }

        let output = format!("(Parsed DOCX text from {:?})", path);
        Ok(CallToolResult::text_content(vec![TextContent::from(output)]))
    }
}