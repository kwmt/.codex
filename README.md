# Codex Workspace

This repo holds your Codex CLI config plus a small Rust helper (`notify-rs`) that surfaces desktop notifications when an agent turn completes.

## Prerequisites
- Rust toolchain (1.80+ recommended): https://www.rust-lang.org/tools/install
- macOS `terminal-notifier` (used by `notify-rs`): `brew install terminal-notifier`

## Build the notifier
```bash
cd notify-rs
cargo build --release
```
The binary will be at `notify-rs/target/release/notify-rs`.

## Point Codex at the binary
Edit `config.toml` in this directory if needed:
```toml
notify = ["./notify-rs/target/release/notify-rs"]
```
Using the relative path keeps the config portable across machines.

## Quick test
After building, you can sanity-check the notifier:
```bash
./notify-rs/target/release/notify-rs \
  '{"type":"agent-turn-complete","last-assistant-message":"Hello","input-messages":["ping"],"cwd":"/tmp"}'
```
You should see a macOS notification titled `Codex: Hello`.

## Notes
- The notifier ignores events whose `type` is not `agent-turn-complete`.
- If notifications do not appear, confirm `terminal-notifier` is installed and has permission in System Settings → Notifications.
