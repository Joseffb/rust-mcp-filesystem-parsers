# Rust MCP Filesystem — Extended Parsers Fork

This repository is a **fork** of the original `rust-mcp-filesystem`, extending it with document-parsing capabilities to make file contents (PDFs, spreadsheets, and similar documents) directly accessible to MCP clients.  
It transforms a basic filesystem server into a more powerful content-aware MCP backend — ideal for AI assistants, RAG pipelines, and automated document workflows.

## 🔧 What’s New in This Fork

- **📄 PDF Parsing** — Use `filesystem.read_pdf` (or similar) to extract text from PDF documents, not just raw bytes.  
- **📊 Spreadsheet / Excel Parsing** — Use `filesystem.read_excel` to load structured spreadsheet data from `.xlsx` or other spreadsheet files.  
- **🗃️ Modular Multi-Format Parser Framework** — A clean, extensible parser architecture allowing future support for CSV, Markdown, DOCX, and other document formats.  
- **🔌 Extensible Parser Infrastructure** — Easy-to-add parser modules: drop a new format module, register it, and expose the new parsing tool via MCP without touching core filesystem code.  
- **💡 Semantic Content Access (not just raw file access)** — Clients can ingest and manipulate content semantically — enabling text analysis, data extraction, indexing, RAG, or automation workflows directly through MCP tools.  

## 🚀 Why This Matters

By combining filesystem operations with content-aware document parsing, this fork enables AI agents to do more than just read files — they can **understand documents**. Great for:

- document analysis workflows (PDF reports, manuals, logs)  
- data extraction from spreadsheets / tables  
- building RAG-based assistants with “read my docs and know everything” capabilities  
- scripts and pipelines that need structured data from user uploads  

## 🧰 Built With

- [rust-mcp-sdk](https://github.com/rust-mcp-stack/rust-mcp-sdk) — core toolkit for MCP server implementation :contentReference[oaicite:2]{index=2}  
- [rust-mcp-schema](https://github.com/rust-mcp-stack/rust-mcp-schema) — official MCP protocol schema support for Rust :contentReference[oaicite:3]{index=3}  
- The existing `rust-mcp-filesystem` core — for filesystem and glob support :contentReference[oaicite:4]{index=4}  

## ⚙️ Example Tool Calls

```jsonc
// Read text from a PDF file
{
  "method": "filesystem.read_pdf",
  "params": {
    "path": "documents/report.pdf"
  }
}

// Read a sheet from an Excel workbook
{
  "method": "filesystem.read_excel",
  "params": {
    "path": "data/financials.xlsx",
    "sheet": "Q4"
  }
}
````

## 🎯 Use Cases

* AI assistants that need to ingest user-supplied documents (PDF, spreadsheet) and answer questions
* RAG / vector-store pipelines that automatically index mixed-format documents
* Internal tools for data extraction from Excel reports or tabular logs
* Automated document analysis & summarization workflows
* Anything where “filesystem + document semantics” is more powerful than “filesystem alone”

## 📦 Installation & Configuration

Refer to the original installation instructions from `rust-mcp-filesystem`. This fork installs and runs in the same way; the additional parser tools are simply available to clients once the server is up.

### Quick install (example with shell script)

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/<your-username>/rust-mcp-filesystem-parsers/releases/download/v0.x.y/installer.sh | sh
```

### Docker / Binary / Homebrew / etc.

Same options as upstream — this fork preserves the lightweight, dependency-free binary distribution model.

## 🔗 Repository

Read the source and browse the code: [https://github.com/Joseffb/rust-mcp-filesystem-parsers](https://github.com/Joseffb/rust-mcp-filesystem-parsers)

---

## 🧬 License

MIT License — same as the upstream project.

---

## 📦 Acknowledgments

* This project builds on the original `rust-mcp-filesystem` by rust-mcp-stack. ([GitHub][1])
* Thanks to the authors and maintainers of `rust-mcp-sdk` and `rust-mcp-schema` for providing the core Rust MCP tooling. ([GitHub][2])
