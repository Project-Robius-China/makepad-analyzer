# Makepad DSL Language Server

## Vision

we want to create a language server that can be used to provide code completion, hover, and definition capabilities for the Makepad DSL.

of course, we not only want to provide these basic language capabilities, but we also want to provide code style, formatting and linting capabilities in the future.

Maybe even a full-blown language server that can be used to compile and run Makepad DSL code, generate a base live view, just like web devtools, where you can tick and see the relevant properties and structures.

The experience might be better if it's integrated with `makepad-studio`, but for now we'll focus more on vscode in preparation for `makepad-studio`.

> [!NOTE]
> ⚠️ makepad-lsp-server is a work-in-progress that doesn't yet support all features.

## Capabilities

- [x] Syntax Highlighting: use `rust-analyzer`, not custom implemented yet
- [ ] Code Completion
- [ ] Hover
- [ ] Definition
