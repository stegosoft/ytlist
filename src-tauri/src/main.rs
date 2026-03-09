// ytlist - extract all video URLs from a YouTube playlist in order.
//
// Usage:
//   ytlist -p <PLAYLIST_URL>
//   ytlist -p <PLAYLIST_URL> -o output.txt

use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use tauri_plugin_cli::CliExt;

// ── JSON structures returned by yt-dlp -J ────────────────────────────────────

#[derive(Deserialize)]
struct PlaylistEntry {
    id: String,
}

#[derive(Deserialize)]
struct PlaylistInfo {
    entries: Option<Vec<PlaylistEntry>>,
}

// ── Core logic ────────────────────────────────────────────────────────────────

/// Call yt-dlp, parse the flat-playlist JSON, and return watch URLs in order.
async fn extract_playlist_urls(playlist_url: &str) -> Result<Vec<String>> {
    let output = tokio::process::Command::new("yt-dlp")
        .args(["--flat-playlist", "-J", playlist_url])
        .output()
        .await
        .context(
            "Failed to execute yt-dlp. \
             Make sure yt-dlp is installed and available in your PATH.",
        )?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("yt-dlp exited with an error:\n{}", stderr.trim());
    }

    let json_str =
        String::from_utf8(output.stdout).context("yt-dlp produced non-UTF-8 output")?;

    let playlist: PlaylistInfo =
        serde_json::from_str(&json_str).context("Failed to parse yt-dlp JSON output")?;

    let entries = playlist
        .entries
        .context("No 'entries' field found – the URL may not be a playlist")?;

    if entries.is_empty() {
        anyhow::bail!("The playlist is empty");
    }

    let urls = entries
        .into_iter()
        .map(|e| format!("https://www.youtube.com/watch?v={}", e.id))
        .collect();

    Ok(urls)
}

/// Write URLs to a file, one per line.
fn write_to_file(path: &str, urls: &[String]) -> Result<()> {
    let content = urls.join("\n");
    fs::write(path, content).with_context(|| format!("Failed to write to '{}'", path))?;
    Ok(())
}

// ── Tauri entry point ─────────────────────────────────────────────────────────

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            // Parse CLI arguments defined in tauri.conf.json → plugins.cli
            let matches = app.cli().matches().context("Failed to parse CLI arguments")?;

            let playlist_url = matches
                .args
                .get("playlist")
                .and_then(|a| a.value.as_str())
                .map(str::to_owned);

            let out_path = matches
                .args
                .get("out")
                .and_then(|a| a.value.as_str())
                .map(str::to_owned);

            // --playlist is required
            let playlist_url = match playlist_url {
                Some(u) if !u.is_empty() => u,
                _ => {
                    eprintln!("Error: --playlist (-p) is required.\n");
                    eprintln!("Usage: ytlist -p <PLAYLIST_URL> [-o <OUTPUT_FILE>]");
                    std::process::exit(1);
                }
            };

            // Run the async extraction on a dedicated Tokio runtime
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .context("Failed to create Tokio runtime")?;

            let urls = match rt.block_on(extract_playlist_urls(&playlist_url)) {
                Ok(u) => u,
                Err(e) => {
                    eprintln!("Error: {:#}", e);
                    std::process::exit(1);
                }
            };

            // Output
            match out_path {
                Some(ref path) => {
                    if let Err(e) = write_to_file(path, &urls) {
                        eprintln!("Error: {:#}", e);
                        std::process::exit(1);
                    }
                    println!(
                        "Wrote {} URL{} to '{}'",
                        urls.len(),
                        if urls.len() == 1 { "" } else { "s" },
                        path
                    );
                }
                None => {
                    for url in &urls {
                        println!("{}", url);
                    }
                }
            }

            std::process::exit(0);
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
