// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::json;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_video_info(id: &str) -> String {
    // https://tyrrrz.me/blog/reverse-engineering-youtube-revisited
    // https://github.com/Tyrrrz/YoutubeExplode

    let client = reqwest::blocking::Client::new();

    client.post("https://www.youtube.com/youtubei/v1/player")
        .header("User-Agent", "com.google.android.youtube/17.36.4 (Linux; U; Android 12; GB) gzip")
        .json(&json!({
            "videoId": id,
            "context": {
                "client": {
                    "clientName": "ANDROID_TESTSUITE",
                    "clientVersion": "1.9",
                    "androidSdkVersion": 30,
                    "hl": "en",
                    "gl": "US",
                    "utcOffsetMinutes": 0
                }
            }
        }))
        .send().unwrap()
        .text().unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_video_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
