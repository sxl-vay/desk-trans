use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::fs;
use std::path::PathBuf;
use std::env;
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationResult {
    pub original: String,
    pub translated: String,
    pub from: String,
    pub to: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationRequest {
    pub text: String,
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiConfig {
    pub api_type: String,
    pub api_key: String,
}

pub struct AppState {
    pub shortcut: Mutex<String>,
}

// 获取剪贴板内容
#[tauri::command]
async fn get_clipboard_text() -> Result<String, String> {
    // 在Tauri 2.0中，剪贴板功能需要通过插件实现
    // 暂时返回一个占位符
    Ok("剪贴板功能需要额外配置".to_string())
}

// 设置剪贴板内容
#[tauri::command]
async fn set_clipboard_text(text: String) -> Result<(), String> {
    // 在Tauri 2.0中，剪贴板功能需要通过插件实现
    // 暂时返回成功
    Ok(())
}

// 注册全局快捷键
#[tauri::command]
async fn register_shortcut(app_handle: AppHandle, shortcut: String) -> Result<(), String> {
    // 在Tauri 2.0中，全局快捷键功能需要通过插件实现
    // 暂时返回成功
    Ok(())
}

// 翻译文本
async fn translate_text(text: &str, from: Option<String>, to: Option<String>) -> Result<TranslationResult, String> {
    // 这里使用免费的翻译API，您可以替换为其他API
    let url = "https://api.mymemory.translated.net/get";
    let from_lang = from.unwrap_or_else(|| "auto".to_string());
    let to_lang = to.unwrap_or_else(|| "zh".to_string());
    
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .query(&[
            ("q", text),
            ("langpair", &format!("{}|{}", from_lang, to_lang)),
        ])
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    
    let translated_text = json["responseData"]["translatedText"]
        .as_str()
        .ok_or("No translation found")?;
    
    Ok(TranslationResult {
        original: text.to_string(),
        translated: translated_text.to_string(),
        from: from_lang,
        to: to_lang,
    })
}

// 手动翻译文本
#[tauri::command]
async fn translate_text_command(request: TranslationRequest) -> Result<TranslationResult, String> {
    translate_text(&request.text, request.from, request.to).await
}

// 保存API配置
#[tauri::command]
async fn save_api_config(config: ApiConfig) -> Result<(), String> {
    // 使用当前工作目录作为配置存储位置
    let config_dir = PathBuf::from("config");
    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create config dir: {}", e))?;
    
    let config_path = config_dir.join("api-config.json");
    let config_json = serde_json::to_string(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    
    fs::write(&config_path, config_json)
        .map_err(|e| format!("Failed to write config file: {}", e))?;
    
    Ok(())
}

// 读取API配置
#[tauri::command]
async fn load_api_config() -> Result<ApiConfig, String> {
    let config_path = PathBuf::from("config").join("api-config.json");
    
    if !config_path.exists() {
        return Ok(ApiConfig {
            api_type: "mymemory".to_string(),
            api_key: "".to_string(),
        });
    }
    
    let config_content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;
    
    let config: ApiConfig = serde_json::from_str(&config_content)
        .map_err(|e| format!("Failed to parse config: {}", e))?;
    
    Ok(config)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            shortcut: Mutex::new("".to_string()),
        })
        .invoke_handler(tauri::generate_handler![
            get_clipboard_text,
            set_clipboard_text,
            register_shortcut,
            translate_text_command,
            save_api_config,
            load_api_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
