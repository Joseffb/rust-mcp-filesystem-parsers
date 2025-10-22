use std::{path::Path, io};
use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use rust_mcp_sdk::schema::{
    CallToolResult,
    ContentBlock, // important for correct matching
    TextContent,
    schema_utils::CallToolError
};
use crate::{
    fs_service::FileSystemService,
    tools::{ParsePdfFile, ParseDocxFile},
};
use tree_magic_mini;

#[mcp_tool(
    name = "parse_file",
    title = "Parse File",
    description = "Parses text-based files (.txt, .pdf, .docx) into text output automatically.",
    destructive_hint = false,
    idempotent_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, JsonSchema)]
pub struct ParseFile {
    pub path: String,
}

fn is_plaintext_type(mime: &str) -> bool {
    matches!(mime, "text/plain" | "text/markdown" | "text/csv")
}

impl ParseFile {
    pub async fn run_tool(
        params: Self,
        context: &FileSystemService,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let path = Path::new(&params.path);
        let mime_type = tree_magic_mini::from_filepath(path).unwrap_or("application/octet-stream");

        // Unify into a single Result<String, _>
        let output_result: Result<String, _> = if is_plaintext_type(mime_type) {
            context.read_text_file(path).await
        } else {
            Ok(match mime_type {
                "application/pdf" => {
                    let inner = ParsePdfFile { path: params.path.clone() };
                    let res = ParsePdfFile::run_tool(inner, context).await?;
                    extract_text_content(res)
                }
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
                    let inner = ParseDocxFile { path: params.path.clone() };
                    let res = ParseDocxFile::run_tool(inner, context).await?;
                    extract_text_content(res)
                }
                _ => {
                    return Err(CallToolError::new(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Unsupported file MIME type: {}", mime_type),
                    )))
                }
            })
        };

        let output = output_result.map_err(CallToolError::new)?;
        Ok(CallToolResult::text_content(vec![TextContent::from(output)]))
    }
}

// ✅ Helper to pull out string content from CallToolResult
fn extract_text_content(result: CallToolResult) -> String {
    result
        .content
        .iter()
        .filter_map(|block| match block {
            ContentBlock::TextContent(t) => Some(t.text.clone()),            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}