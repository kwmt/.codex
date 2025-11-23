# Codex ワークスペース

このリポジトリは Codex CLI の設定と、エージェントのターン完了時にデスクトップ通知を出す小さな Rust 製ヘルパー `notify-rs` を含みます。

## 必要環境
- Rust ツールチェーン（1.80 以降推奨）: https://www.rust-lang.org/tools/install
- macOS の通知コマンド `terminal-notifier`: `brew install terminal-notifier`

## notifier のビルド
```bash
cd notify-rs
cargo build --release
```
ビルド後のバイナリは `notify-rs/target/release/notify-rs` に生成されます。

## Codex へのパス設定
このディレクトリの `config.toml` でバイナリへのパスを指定します。
```toml
notify = ["./notify-rs/target/release/notify-rs"]
```
相対パスにすることでマシン固有のパス依存を避けられます。

## 動作確認
ビルド後、以下で通知を送って確認できます。
```bash
./notify-rs/target/release/notify-rs \
  '{"type":"agent-turn-complete","last-assistant-message":"Hello","input-messages":["ping"],"cwd":"/tmp"}'
```
`Codex: Hello` というタイトルの macOS 通知が表示されれば成功です。

## Slack への投稿（オプション）
Slack の Incoming Webhook URL を `SLACK_WEBHOOK_URL` 環境変数で指定すると、同じ内容を Slack にもポストします。

```bash
export SLACK_WEBHOOK_URL="https://hooks.slack.com/services/..."
./notify-rs/target/release/notify-rs '{"type":"agent-turn-complete","last-assistant-message":"Hello"}'
```
失敗した場合は標準エラーに理由を出力しますが、デスクトップ通知は従来どおり送られます。

## 補足
- `type` が `agent-turn-complete` 以外のイベントは無視されます。
- 通知が出ない場合は `terminal-notifier` がインストール済みか、システム設定 → 通知で許可されているかを確認してください。
