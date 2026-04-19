# Barebones — Spin Starter Template

A minimal Spin project template with commonly used files, boilerplate code, and a built-in Swagger-style API Explorer for testing endpoints during development.

## What's Included

| File / Directory                | Purpose                                                             |
| ------------------------------- | ------------------------------------------------------------------- |
| `spin.toml`                     | Spin application manifest — defines components, routes, and config  |
| `Cargo.toml`                    | Rust workspace root — lists all component crates                    |
| `Makefile`                      | Common build, test, lint, and run commands                          |
| `README.md`                     | **Your project README** — fill in with your data        |
| `CHANGELOG.md`                  | Keep a Changelog format — document releases here                    |
| `LICENSE`                       | License file placeholder — replace with your chosen license         |
| `.gitignore`                    | Ignores build artifacts, Spin state, and Cargo cache                |
| `.editorconfig`                 | Consistent editor settings (indentation, line endings, etc.)        |
| `assets/index.html`             | API Explorer UI served at `/api-explorer`                           |
| `components/hello/`             | Starter HTTP component mounted at `/hello/...`                      |
| `components/api-explorer/`      | Component that serves the API Explorer HTML page                    |

## Getting Started

### 1. Prerequisites

- **Rust** — Install via [rustup](https://www.rust-lang.org/tools/install), then add the Wasm target:
  ```bash
  rustup target add wasm32-wasip1
  ```
- **Spin CLI v3+** — Install from [spinframework.dev](https://spinframework.dev/v3/install)

### 2. Build and Run

```bash
make build    # Format, lint, and compile all components
make run      # Start the Spin dev server at http://127.0.0.1:3000
```

### 3. Explore Endpoints

Visit `http://127.0.0.1:3000/api-explorer` to test your endpoints using the built-in Swagger-style UI.

## Adding Components

```bash
# Scaffold a new Rust HTTP component
spin add -t http-rust -o components/<component-name>
```

After scaffolding:

1. Add `"components/<component-name>"` to the `members` array in the root `Cargo.toml`.
2. The `spin add` command will automatically register the component and trigger in `spin.toml`.
3. Update the `endpoints` array in `assets/index.html` so the API Explorer knows about the new route.

## Project Layout Convention

All components live under `components/` to keep the project organized:

```
components/
├── hello/               # Each component is its own Cargo crate
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── api-explorer/
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

## Makefile Reference

| Target        | Description                                    |
| ------------- | ---------------------------------------------- |
| `make help`   | List all available targets                     |
| `make build`  | Format + lint + cargo build + spin build       |
| `make run`    | Run the app with `spin up`                     |
| `make test`   | Run tests with `spin test`                     |
| `make check`  | Cargo check + format check                    |
| `make audit`  | Run `cargo audit` for known vulnerabilities    |
| `make ci`     | Checks + tests (suitable for CI pipelines)     |
| `make all`    | Check + test + build                           |

## Updating the API Explorer

The API Explorer reads its endpoint list from a JavaScript array in `assets/index.html`. To add or remove endpoints from the UI, edit the `endpoints` array:

```javascript
const endpoints = [
  {
    method: "GET",
    path: "/hello",
    summary: "Returns the hello component response",
  },
  // Add more endpoints here in the index.html
];
```

## Useful Spin Commands

```bash
spin build              # Build all components defined in spin.toml
spin up                 # Run the application locally
spin test               # Run component tests
spin add -t http-rust   # Scaffold a new Rust HTTP component
spin doctor             # Check your Spin environment
```

## Resources

- [Spin Documentation](https://spinframework.dev/v3/)
- [Spin Manifest Reference](https://spinframework.dev/v3/manifest-reference)
- [Spin Rust SDK](https://crates.io/crates/spin-sdk)
- [Fermyon Developer Hub](https://developer.fermyon.com/)
