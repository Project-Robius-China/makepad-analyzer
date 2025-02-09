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
    initializationOptions: {
      client: "vscode",
      logging: {
        level: "debug",
      }
    }
  }

  // const completionProvider = vscode.languages.registerCompletionItemProvider(
  //   { scheme: "file", language: "rust" },
  //   {
  //     provideCompletionItems: async (document, position, token, context) => {
  //       const result = await client.sendRequest("textDocument/completion", {
  //         textDocument: { uri: document.uri.toString() },
  //         position: client.code2ProtocolConverter.asPosition(position),
  //         context
  //       });

  //       if (Array.isArray(result)) {
  //         console.log("Got completion items", result);
  //         return result.map(item => {
  //           const vscodeCompletionItem = new vscode.CompletionItem(
  //             item.label,
  //             item.kind
  //           );

  //           if (item.detail) {
  //             vscodeCompletionItem.detail = item.detail;
  //           }
  //           if (item.documentation) {
  //             vscodeCompletionItem.documentation = item.documentation;
  //           }

  //           console.log("Returning completion item", vscodeCompletionItem);

  //           return vscodeCompletionItem;
  //         });
  //       };
  //       return [];
  //     },
  //   },
  //   ':',
  //   '/',
  // );

  vscode.workspace.onDidOpenTextDocument(document => {
    console.log("Opened document", document.uri.toString());
    client.sendNotification("textDocument/didOpen", {
      textDocument: {
        uri: document.uri,
        languageId: document.languageId,
        version: document.version,
        text: document.getText()
      }
    })
  })

  // context.subscriptions.push(completionProvider);

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
