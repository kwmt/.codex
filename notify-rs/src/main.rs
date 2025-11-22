use serde::Deserialize;
use std::env;
use std::process::Command;

#[derive(Deserialize)]
struct Notification {
    #[serde(rename = "type")]
    ntype: String,
    #[serde(rename = "last-assistant-message")]
    last_assistant_message: Option<String>,
    #[serde(rename = "input-messages")]
    input_messages: Option<Vec<String>>,
    #[serde(rename = "thread-id")]
    thread_id: Option<String>,
    cwd: Option<String>,
}

fn build_text(n: &Notification) -> (String, String) {
    let title_msg = n
        .last_assistant_message
        .as_deref()
        .unwrap_or("Turn complete");
    let title = format!("Codex: {}", title_msg);

    let mut body = n
        .input_messages
        .as_ref()
        .map(|v| v.join(" "))
        .unwrap_or_default();
    if body.len() > 180 {
        body.truncate(180);
    }
    if let Some(cwd) = &n.cwd {
        if body.is_empty() {
            body = cwd.clone();
        } else {
            body.push_str("  (");
            body.push_str(cwd);
            body.push(')');
        }
    }
    if body.is_empty() {
        body = "Finished".to_string();
    }
    (title, body)
}

fn main() {
    // Expect a single JSON arg from Codex
    let mut args = env::args().skip(1);
    let Some(json_arg) = args.next() else {
        eprintln!("Usage: notify-rs '<NOTIFICATION_JSON>'");
        std::process::exit(1);
    };

    let n: Notification = match serde_json::from_str(&json_arg) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("Invalid JSON: {err}");
            std::process::exit(1);
        }
    };

    if n.ntype != "agent-turn-complete" {
        // Ignore unknown types to stay silent.
        return;
    }

    let (title, message) = build_text(&n);
    let thread = n
        .thread_id
        .as_deref()
        .map(|s| format!("codex-{s}"))
        .unwrap_or_else(|| "codex".to_string());

    if let Err(err) = Command::new("terminal-notifier")
        .args([
            "-title",
            &title,
            "-message",
            &message,
            "-group",
            &thread,
            "-ignoreDnD",
        ])
        .status()
    {
        eprintln!("failed to run terminal-notifier: {err}");
        std::process::exit(1);
    }
}
