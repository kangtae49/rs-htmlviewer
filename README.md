# Tauri + Vanilla

This template should help get you started developing with Tauri in vanilla HTML, CSS and Javascript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


```sh
$ cargo install create-tauri-app --locked
```

```sh
cargo create-tauri-app
✔ Project name · rs-htmlviewer
✔ Identifier · com.rs-htmlviewer.app
✔ Choose which language to use for your frontend · Rust - (cargo)
✔ Choose your UI template · Vanilla

Template created! To get started run:
  cd rs-htmlviewer
  cargo tauri android init

For Desktop development, run:
  cargo tauri dev

For Android development, run:
  cargo tauri android dev
```

```
https://github.com/new
```

```sh
cd rs-htmlviewer
git init
git config --global core.autocrlf true
git add .
git commit -m "first commit"
git branch -M main
git remote add origin https://github.com/kangtae49/rs-htmlviewer.git
git push -u origin main
```

```sh
cargo tauri dev
```