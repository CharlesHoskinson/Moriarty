//! One source-check tool, served through the SDK's standard MCP transport.

use std::sync::Arc;

use rmcp::{
    ErrorData, RoleServer, ServerHandler,
    model::{
        CacheScope, CallToolRequestMethod, CallToolRequestParams, CallToolResponse,
        CallToolResult, Implementation, JsonObject, ListToolsResult, PaginatedRequestParams,
        ServerCapabilities, ServerConfig, Tool, ToolAnnotations,
    },
    service::RequestContext,
};
use serde::Deserialize;
use serde_json::{Value, json};

use super::super::check_source::{CheckAnalysis, analyze_source};

const TOOL_NAME: &str = "mori_check_source";
const SOURCE_INPUT_BYTES: usize = 1_048_576;
const INPUT_SCHEMA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../fixtures/mcp/input-schema.json"
));
const OUTPUT_SCHEMA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../fixtures/mcp/output-schema.json"
));

pub(super) struct MoriServer;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckArgs {
    source: String,
    #[serde(default = "default_file")]
    file: String,
}

fn default_file() -> String {
    "mcp-input.mori".into()
}

fn schema(text: &str) -> Arc<JsonObject> {
    // Both arguments are embedded, versioned JSON objects, never caller input.
    Arc::new(serde_json::from_str(text).expect("embedded MCP schema is a JSON object"))
}

fn tool_definition() -> Tool {
    Tool::new(
        TOOL_NAME,
        "Return source-check diagnostics through the same profile consumer as mori check. Accepted means local source checking only.",
        schema(INPUT_SCHEMA),
    )
    .with_title("Check Moriarty source")
    .with_raw_output_schema(schema(OUTPUT_SCHEMA))
    .with_annotations(ToolAnnotations::new().read_only(true).open_world(false))
}

fn diagnostic_payload(args: &CheckArgs) -> Value {
    let analysis = analyze_source(&args.file, &args.source);
    let (profile, diagnostics, qualification) = match &analysis {
        CheckAnalysis::Beta { report, .. } => {
            let diagnostics: Vec<Value> = report
                .diagnostics
                .iter()
                .map(|diagnostic| {
                    json!({
                        "kind": "beta",
                        "code": diagnostic.code,
                        "severity": diagnostic.severity,
                        "message": diagnostic.message,
                        "help": diagnostic.help,
                        "labels": diagnostic.labels
                    })
                })
                .collect();
            ("moriarty-beta/1", diagnostics, Value::Null)
        }
        CheckAnalysis::Core { result, staged } => {
            let diagnostics = match result {
                Ok(_) => Vec::new(),
                Err(error) => vec![json!({
                    "kind": "core", "code": error.code, "message": error.message,
                    "offset": error.offset, "step": error.step
                })],
            };
            let qualification = if *staged {
                Value::Object(
                    mori_core::stage::STAGE_LABELS
                        .into_iter()
                        .map(|(name, value)| (name.to_owned(), Value::String(value.to_owned())))
                        .collect(),
                )
            } else {
                Value::Null
            };
            (
                if *staged { "moriarty/2" } else { "moriarty/1" },
                diagnostics,
                qualification,
            )
        }
    };
    json!({
        "format": "mori-mcp-check/1",
        "kind": "check",
        "scope": "source-check-only",
        "file": args.file,
        "source_hash": mori_core::intent::hash_bytes(args.source.as_bytes()),
        "checked_profile": profile,
        "status": if analysis.is_accepted() { "accepted" } else { "rejected" },
        "diagnostics": diagnostics,
        "stage_qualification": qualification
    })
}

impl ServerHandler for MoriServer {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("mori", env!("CARGO_PKG_VERSION")))
            .with_instructions(
                "One source-check tool. A local accepted report is not compilation, proof or settlement.",
            )
    }

    async fn list_tools(
        &self,
        request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        if request.as_ref().is_some_and(|params| params.cursor.is_some()) {
            return Err(ErrorData::invalid_params("No tool-list continuation exists.", None));
        }
        Ok(ListToolsResult::with_all_items(vec![tool_definition()])
            .with_ttl_ms(0)
            .with_cache_scope(CacheScope::Private))
    }

    fn get_tool(&self, name: &str) -> Option<Tool> {
        (name == TOOL_NAME).then(tool_definition)
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        if request.name != TOOL_NAME {
            return Err(ErrorData::method_not_found::<CallToolRequestMethod>());
        }
        let args: CheckArgs = serde_json::from_value(Value::Object(
            request.arguments.unwrap_or_default(),
        ))
        .map_err(|_| {
            ErrorData::invalid_params(
                "mori_check_source requires source:string and optional file:string; unknown fields are rejected.",
                None,
            )
        })?;
        if args.source.len() > SOURCE_INPUT_BYTES {
            return Ok(CallToolResult::structured_error(json!({
                "format": "mori-mcp-check/1",
                "kind": "tool_error",
                "code": "source_too_large",
                "message": "source exceeds the existing public CLI 1 MiB input bound",
                "limit_bytes": SOURCE_INPUT_BYTES
            }))
            .into());
        }
        Ok(CallToolResult::structured(diagnostic_payload(&args)).into())
    }
}

#[cfg(test)]
mod tests;
