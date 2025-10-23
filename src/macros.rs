/// Generates a `match` expression for dispatching `FileSystemTools` variants to their respective `run_tool` methods.
///
/// This macro reduces boilerplate in matching `FileSystemTools` enum variants by generating a `match` arm
/// for each specified tool. Each arm calls the tool's `run_tool` method with the provided parameters and
/// filesystem service, handling the async dispatch uniformly.
///
/// # Parameters
/// - `$params:expr`: A `FileSystemTools` value to dispatch on.
/// - `$fs_service:expr`: The filesystem service reference passed to each tool’s `run_tool`.
/// - `$($tool:ident),*`: Tool identifiers corresponding to `FileSystemTools` variants and their types.
///
/// # Expected signature (per tool)
/// ```rust,ignore
/// async fn run_tool(params: ParamsType, fs_service: &FsService) -> ServiceResult<()>
/// ```
///
/// # Example
/// ```rust,ignore
/// invoke_tools!(
///     tool_params,
///     &fs_service,
///     ReadMediaFile,
///     WriteFile
/// );
/// ```
///
/// This expands roughly to:
/// ```rust,ignore
/// match tool_params {
///     FileSystemTools::ReadMediaFile(params) => ReadMediaFile::run_tool(params, &fs_service).await,
///     FileSystemTools::WriteFile(params) => WriteFile::run_tool(params, &fs_service).await,
/// }
/// ```
///
/// # Notes
/// - Ensure each `$tool` matches a `FileSystemTools` variant name and a type with `run_tool`.
/// - Examples are marked `ignore` to avoid doctest compilation since they’re schematic.
#[macro_export]
macro_rules! invoke_tools {
    ($params:expr, $fs_service:expr, $($tool:ident),* $(,)?) => {
        match $params {
            $(
                FileSystemTools::$tool(params) => $tool::run_tool(params, $fs_service).await,
            )*
        }
    };
}
