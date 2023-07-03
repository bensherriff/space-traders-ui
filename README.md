# Space Traders UI
A GUI and automation tool for [Space Traders](https://spacetraders.io/)
  - Built on the v2 API
  - [Rust](https://www.rust-lang.org/)
  - [Tauri](https://tauri.app)

## Contributing
For development and debugging, the application can be started using the following:
```curl
npm install
npm run tauri dev
```

### Database Interaction
The database is a SQLITE file located in the local app data directory. Tables can be manipulated using [diesel_cli](https://crates.io/crates/diesel_cli), PSQL, or a tool like [SQLite Browser](https://sqlitebrowser.org/)

`cargo install diesel_cli --no-default-features --features "sqlite-bundled"` or `cargo install diesel_cli --no-default-features --features "sqlite"`

- Setup and run migrations (from `src-tauri` directory)
```curl
diesel setup
diesel migration generate <migrations>
diesel migration run
```

Handy: `diesel migration redo` and `diesel database reset`

## Building
`npm run tauri build`
The installation executable is located under `src-tauri/target/release/bundle/nsis/`

A debug build can be built with the following:
`npm run tauri build -- --debug`
