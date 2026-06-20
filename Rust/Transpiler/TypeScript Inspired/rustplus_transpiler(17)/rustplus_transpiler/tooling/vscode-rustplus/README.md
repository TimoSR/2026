# Rust Plus Syntax for VS Code

Syntax highlighting and snippets for Rust Plus `.rp` files.

This is a grammar-only extension. It does not run the transpiler, type-check `.rp` files, or replace rust-analyzer. The transpiler still generates sibling `.rs` files, and rust-analyzer works against those generated Rust files.

## Supported highlighting

- `.rp` file association
- `class`
- `interface`
- `abstract class`
- `pub` / `public`; fields are private by default
- `pub`
- `this`
- normal Rust syntax via VS Code's Rust TextMate grammar

## Install into the project workspace

From the repository root, run the included VS Code task:

```text
Terminal -> Run Task -> Rust Plus: Install VS Code Extension
```

The repository root has `.vscode/settings.json` configured with:

```json
{
    "files.associations": {
        "*.rp": "rustplus"
    }
}
```

After installing, reload VS Code and open a `.rp` file. The lower-right language mode should show `Rust Plus`.

## Install manually as VSIX

Install Node.js first if needed, then run from this extension folder:

```bash
npx --yes @vscode/vsce package --allow-missing-repository
code --install-extension rustplus-syntax-0.1.2.vsix --force
```

After installing, reload VS Code and open a `.rp` file.

## Run in extension development mode

From this extension folder:

```bash
code .
```

Press `F5` to launch an Extension Development Host.

Open a `.rp` file in the Extension Development Host and select the `Rust Plus` language mode if needed.

## Snippets

Available snippet prefixes:

```text
interface
class
classimpl
abstract
```


## File icons

This extension now bundles a Rust Plus file icon theme based on the R+ logo.

After installing the extension:

1. Open **Preferences: File Icon Theme**.
2. Select **Rust Plus File Icons**.
3. Open any `.rp` file to see the custom icon in the Explorer.

The extension icon and the `.rp` file icon both use the packaged R+ logo.
