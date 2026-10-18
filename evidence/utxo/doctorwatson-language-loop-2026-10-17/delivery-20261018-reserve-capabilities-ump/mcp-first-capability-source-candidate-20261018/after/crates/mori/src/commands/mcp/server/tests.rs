use super::{MoriServer, TOOL_NAME};
use clap::Parser;
use rmcp::{ServiceExt, model::CallToolRequestParams};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
struct Case {
    name: String,
    source: String,
    file: String,
    expected_mcp: Value,
}

#[test]
fn public_mcp_subcommand_selects_the_server() {
    let cli = crate::cli::Cli::try_parse_from(["mori", "mcp"]).unwrap();
    assert!(matches!(cli.command, crate::cli::Command::Mcp(_)));
}

#[test]
fn normal_sdk_session_lists_and_checks_all_public_profile_fixtures() {
    let cases: Vec<Case> = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../fixtures/mcp/check-cases.json"
    )))
    .unwrap();
    let expected_listing: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../fixtures/mcp/tools-list.response.json"
    )))
    .unwrap();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            tokio::time::timeout(std::time::Duration::from_secs(5), async {
                let (server_io, client_io) = tokio::io::duplex(16_384);
                let starting_server = tokio::spawn(async {
                    MoriServer.serve(server_io).await.unwrap()
                });
                let client = ().serve(client_io).await.unwrap();
                let server = starting_server.await.unwrap();
                let listing = client.list_tools(None).await.unwrap();
                assert_eq!(listing.tools.len(), 1);
                assert_eq!(
                    serde_json::to_value(&listing.tools[0]).unwrap(),
                    expected_listing["result"]["tools"][0]
                );
                for case in &cases {
                    let args = json!({"source": case.source, "file": case.file})
                        .as_object()
                        .unwrap()
                        .clone();
                    let result = client
                        .call_tool(CallToolRequestParams::new(TOOL_NAME).with_arguments(args))
                        .await
                        .unwrap();
                    assert_eq!(result.is_error, Some(false), "{} tool result", case.name);
                    assert_eq!(
                        result.structured_content.as_ref().unwrap(),
                        &case.expected_mcp,
                        "{} complete diagnostics",
                        case.name
                    );
                    assert_eq!(result.content.len(), 1);
                    let text = result.content[0].as_text().unwrap();
                    assert_eq!(
                        serde_json::from_str::<Value>(&text.text).unwrap(),
                        case.expected_mcp,
                        "{} text and structured agreement",
                        case.name
                    );
                }
                // A normal later request still works after reported source errors.
                let mut expected = cases[0].expected_mcp.clone();
                expected["file"] = json!("mcp-input.mori");
                let args = json!({"source": cases[0].source})
                    .as_object()
                    .unwrap()
                    .clone();
                let later = client
                    .call_tool(CallToolRequestParams::new(TOOL_NAME).with_arguments(args))
                    .await
                    .unwrap();
                assert_eq!(later.structured_content, Some(expected));
                client.cancel().await.unwrap();
                server.waiting().await.unwrap();
            })
            .await
            .expect("normal in-memory SDK session completes");
        });
}
