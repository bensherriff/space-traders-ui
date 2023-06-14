# Space Traders UI


## Development
Prerequisites
- [Rust](https://www.rust-lang.org/)
  - Cargo
  - Rustup
- [Tauri](https://tauri.app)
- [npm](https://github.com/nvm-sh/nvm)

The application can be started for development using the following:
```curl
npm install
npm run tauri dev
```

### Database Setup
Requires [diesel_cli](https://crates.io/crates/diesel_cli)

`cargo install diesel_cli --no-default-features --features "sqlite-bundled"` or `cargo install diesel_cli --no-default-features --features "sqlite"`

- Setup .env with the DATABASE_URL
```curl
echo DATABASE_URL=sqlite://data/stu.db > .env
source .env
```

- Setup and run migrations (from `src-tauri` directory)
```curl
diesel setup
diesel migration generate <migrations>
diesel migration run
```

Handy: `diesel migration redo` and `diesel database reset`

### Building
`npm run tauri build` or `npm run tauri build -- --debug`