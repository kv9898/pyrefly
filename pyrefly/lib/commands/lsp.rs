/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::sync::Arc;

use clap::Parser;
use clap::ValueEnum;
use lsp_server::Connection;
use lsp_server::ProtocolError;
use lsp_types::InitializeParams;

use crate::commands::util::CommandExitStatus;
use crate::lsp::non_wasm::server::capabilities;
use crate::lsp::non_wasm::server::lsp_loop;

/// Pyrefly's indexing strategy for open projects when performing go-to-definition
/// requests.
#[deny(clippy::missing_docs_in_private_items)]
#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, Default)]
pub(crate) enum IndexingMode {
    /// Do not index anything. Features that depend on indexing (e.g. find-refs) will be disabled.
    None,
    /// Start indexing when opening a file that belongs to a config in the background.
    /// Indexing will happen in another thread, so that normal IDE services are not blocked.
    #[default]
    LazyNonBlockingBackground,
    /// Start indexing when opening a file that belongs to a config in the background.
    /// Indexing will happen in the main thread, so that IDE services will be blocked.
    /// However, this is useful for deterministic testing.
    LazyBlocking,
}

/// Configuration for which language services should be disabled
#[deny(clippy::missing_docs_in_private_items)]
#[derive(Debug, Clone, Default)]
pub struct DisabledLanguageServices {
    /// Disable go-to-definition
    pub(crate) definition: bool,
    /// Disable go-to-type-definition
    pub(crate) type_definition: bool,
    /// Disable code actions (quick fixes)
    pub(crate) code_action: bool,
    /// Disable completion (autocomplete)
    pub(crate) completion: bool,
    /// Disable document highlight
    pub(crate) document_highlight: bool,
    /// Disable find references
    pub(crate) references: bool,
    /// Disable rename
    pub(crate) rename: bool,
    /// Disable signature help (parameter hints)
    pub(crate) signature_help: bool,
    /// Disable hover (tooltips)
    pub(crate) hover: bool,
    /// Disable inlay hints
    pub(crate) inlay_hint: bool,
    /// Disable document symbols (outline)
    pub(crate) document_symbol: bool,
    /// Disable workspace symbols
    pub(crate) workspace_symbol: bool,
    /// Disable semantic tokens
    pub(crate) semantic_tokens: bool,
}

/// Arguments for LSP server
#[deny(clippy::missing_docs_in_private_items)]
#[derive(Debug, Parser, Clone)]
pub struct LspArgs {
    /// Find the struct that contains this field and add the indexing mode used by the language server
    #[arg(long, value_enum, default_value_t)]
    pub(crate) indexing_mode: IndexingMode,
    /// Sets the maximum number of user files for Pyrefly to index in the workspace.
    /// Note that indexing files is a performance-intensive task.
    #[arg(long, default_value_t = if cfg!(fbcode_build) {0} else {2000})]
    pub(crate) workspace_indexing_limit: usize,
    /// Disable go-to-definition
    #[arg(long)]
    pub(crate) disable_definition: bool,
    /// Disable go-to-type-definition
    #[arg(long)]
    pub(crate) disable_type_definition: bool,
    /// Disable code actions (quick fixes)
    #[arg(long)]
    pub(crate) disable_code_action: bool,
    /// Disable completion (autocomplete)
    #[arg(long)]
    pub(crate) disable_completion: bool,
    /// Disable document highlight
    #[arg(long)]
    pub(crate) disable_document_highlight: bool,
    /// Disable find references
    #[arg(long)]
    pub(crate) disable_references: bool,
    /// Disable rename
    #[arg(long)]
    pub(crate) disable_rename: bool,
    /// Disable signature help (parameter hints)
    #[arg(long)]
    pub(crate) disable_signature_help: bool,
    /// Disable hover (tooltips)
    #[arg(long)]
    pub(crate) disable_hover: bool,
    /// Disable inlay hints
    #[arg(long)]
    pub(crate) disable_inlay_hint: bool,
    /// Disable document symbols (outline)
    #[arg(long)]
    pub(crate) disable_document_symbol: bool,
    /// Disable workspace symbols
    #[arg(long)]
    pub(crate) disable_workspace_symbol: bool,
    /// Disable semantic tokens
    #[arg(long)]
    pub(crate) disable_semantic_tokens: bool,
}

pub fn run_lsp(
    connection: Arc<Connection>,
    args: LspArgs,
    version_string: &str,
) -> anyhow::Result<()> {
    let initialization_params = match initialize_connection(&connection, &args, version_string) {
        Ok(it) => it,
        Err(e) => {
            // Use this in later versions of LSP server
            // if e.channel_is_disconnected() {
            // io_threads.join()?;
            // }
            return Err(e.into());
        }
    };
    lsp_loop(
        connection,
        initialization_params,
        args.indexing_mode,
        args.workspace_indexing_limit,
        args.disabled_services(),
    )?;
    Ok(())
}

fn initialize_connection(
    connection: &Connection,
    args: &LspArgs,
    version_string: &str,
) -> Result<InitializeParams, ProtocolError> {
    let (request_id, initialization_params) = connection.initialize_start()?;
    let initialization_params: InitializeParams =
        serde_json::from_value(initialization_params).unwrap();
    let server_capabilities = serde_json::to_value(capabilities(
        args.indexing_mode,
        &initialization_params,
        &args.disabled_services(),
    ))
    .unwrap();
    let initialize_data = serde_json::json!({
        "capabilities": server_capabilities,
        "serverInfo": {
            "name": "pyrefly-lsp",
            "version": version_string,
        }
    });

    connection.initialize_finish(request_id, initialize_data)?;
    Ok(initialization_params)
}

impl LspArgs {
    /// Extract disabled language services configuration from command-line arguments
    pub(crate) fn disabled_services(&self) -> DisabledLanguageServices {
        DisabledLanguageServices {
            definition: self.disable_definition,
            type_definition: self.disable_type_definition,
            code_action: self.disable_code_action,
            completion: self.disable_completion,
            document_highlight: self.disable_document_highlight,
            references: self.disable_references,
            rename: self.disable_rename,
            signature_help: self.disable_signature_help,
            hover: self.disable_hover,
            inlay_hint: self.disable_inlay_hint,
            document_symbol: self.disable_document_symbol,
            workspace_symbol: self.disable_workspace_symbol,
            semantic_tokens: self.disable_semantic_tokens,
        }
    }

    pub fn run(self, version_string: &str) -> anyhow::Result<CommandExitStatus> {
        // Note that  we must have our logging only write out to stderr.
        eprintln!("starting generic LSP server");

        // Create the transport. Includes the stdio (stdin and stdout) versions but this could
        // also be implemented to use sockets or HTTP.
        let (connection, io_threads) = Connection::stdio();

        run_lsp(Arc::new(connection), self, version_string)?;
        io_threads.join()?;
        // We have shut down gracefully.
        eprintln!("shutting down server");
        Ok(CommandExitStatus::Success)
    }
}
