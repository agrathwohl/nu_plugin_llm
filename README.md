# nu_plugin_llm

A [Nushell](https://www.nushell.sh/) plugin that provides LLM-optimized structured data output. Every command returns consistent JSON with rich metadata, designed for AI consumption with zero visual noise.

## Commands

| Command | Description |
|---------|-------------|
| `llm-test` | Plugin health check and capability listing |
| `llm-ls` | File listing with metadata, permissions, and optional SHA256 checksums |
| `llm-ps` | Process listing with CPU/memory stats (sensitive env vars redacted) |
| `llm-sys` | System info — OS, CPU, memory, load average, disks, network interfaces |
| `llm-env` | Environment variables with optional pattern filtering |
| `llm-git` | Git repository status via libgit2 |
| `llm-net` | TCP connections from `/proc/net/tcp` with decoded addresses and state filtering |
| `llm-find` | Regex file search with type filtering and depth limits |
| `llm-analyze` | File analysis — line/word/char counts, file type detection |
| `llm-wc` | Word count with frequency analysis and top-word ranking |
| `llm-du` | Disk usage with directory-level breakdown |
| `llm-mode` | Display output configuration options |

## Install

```sh
cargo build --release
plugin add target/release/nu_plugin_llm
```

Then restart Nushell.

## Build

### With Nix

```sh
nix develop
cargo build --release
```

### Without Nix

Requires Rust toolchain, `pkg-config`, `openssl-dev`, and optionally `libgit2-dev`.

```sh
cargo build --release
```

To build without git support:

```sh
cargo build --release --no-default-features
```

## Usage

```nu
# List files with checksums
llm-ls --recursive --checksums

# System info with disk and network details
llm-sys --detailed

# Find Rust files
llm-find "\.rs$" --type file

# Processes sorted by CPU
llm-ps

# Git status
llm-git

# TCP connections in LISTEN state
llm-net --listening

# Disk usage, 2 levels deep
llm-du --depth 2

# Environment variables matching "PATH"
llm-env PATH
```

Every command returns a JSON string with a `metadata` object containing timestamp, execution time, hostname, and request ID.

## Features

- **`git`** (default) — enables `llm-git` via libgit2. Disable with `--no-default-features`.

## License

MIT
