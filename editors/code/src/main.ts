import * as vscode from 'vscode';

import {
  Executable,
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
  const command = process.env.SERVER_PATH;
  console.log("Starting Makepad Analyzer with command", command);
  const run: Executable = {
    command,
    options: {
      env: {
          ...process.env,
          RUST_LOG: "debug",
      },
    },
  };
  const serverOptions: ServerOptions = {
    run,
    debug: run,
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "rust" }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher("**/.clientrc"),
    },
  }

  client = new LanguageClient("makepad-analyzer", "Makepad Analyzer", serverOptions, clientOptions);

  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
      return undefined;
  }

  console.log("Stopping Makepad Analyzer");

  return client.stop();
}
