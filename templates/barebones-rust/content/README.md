# Project Name

Brief description of what this project does.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (with the `wasm32-wasip1` target)
- [Spin CLI](https://spinframework.dev/v3/install) v3+

## Quick Start

```bash
# Build all components
make build

# Run the application
make run
```

The app will be available at `http://127.0.0.1:3000`.

## Project Structure

```
├── spin.toml                  # Spin application manifest
├── Cargo.toml                 # Rust workspace configuration
├── Makefile                   # Build, test, and run commands
├── assets/                    # Static files served at /static/...
│   └── index.html             # API Explorer UI
└── components/                # Spin components (one directory per component)
    ├── api-explorer/          # Swagger-style endpoint tester (/api-explorer)
    └── hello/                 # Starter HTTP component (/hello/...)
```

## Routes

| Route            | Component      | Description                              |
| ---------------- | -------------- | ---------------------------------------- |
| `/hello/...`     | hello          | Describe what this endpoint does.        |
| `/api-explorer`  | api-explorer   | Swagger-style UI for testing endpoints   |
| `/static/...`    | assets         | Static file server                       |

## Adding a New Component

```bash
# Scaffold a new HTTP component
spin add -t http-rust -o components/<component-name>
```

Then add `"components/<component-name>"` to the `members` list in `Cargo.toml`.

## Available Commands

| Command          | Description                                  |
| ---------------- | -------------------------------------------- |
| `make build`     | Format, lint, and build (Cargo + Spin)       |
| `make run`       | Start the Spin application                   |
| `make test`      | Run all tests                                |
| `make check`     | Run cargo check and format check             |
| `make audit`     | Run security audit on dependencies           |
| `make ci`        | Run checks and tests (CI pipeline)           |
| `make all`       | Check, test, and build                       |

## Configuration

Key configuration lives in [spin.toml](spin.toml):

- **`allowed_outbound_hosts`** — Whitelist external hosts a component can reach. Empty by default.
- **`route`** — The URL path pattern that triggers a component.
- **`files`** — Static files to bundle with a component.

## License

Choose a license — e.g., MIT, Apache-2.0, or remove this section.