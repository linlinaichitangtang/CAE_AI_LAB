use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AIConfig {
    pub source: String,
    pub api_url: String,
    pub api_key: String,
    pub model_name: String,
    pub max_tokens: usize,
    pub temperature: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIResponse {
    pub content: String,
    pub error: Option<String>,
}

/// 调用AI进行对话（非流式）
#[tauri::command]
pub async fn ai_chat(
    prompt: String,
    config: AIConfig,
) -> Result<AIResponse, String> {
    if config.api_url.is_empty() || config.model_name.is_empty() {
        return Ok(AIResponse {
            content: String::new(),
            error: Some("请先在设置中配置AI API".to_string()),
        });
    }

    // 构建请求体
    let request_body = serde_json::json!({
        "model": config.model_name,
        "messages": [
            {"role": "system", "content": "你是CAELab的AI助手，专门帮助用户进行CAE仿真、代码编写、笔记整理等任务。请用中文回答。"},
            {"role": "user", "content": prompt}
        ],
        "max_tokens": config.max_tokens,
        "temperature": config.temperature
    });

    // 发送HTTP请求
    let client = reqwest::Client::new();
    let response = client
        .post(&config.api_url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?;

    let response_json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    // 提取内容
    let content = response_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();

    Ok(AIResponse {
        content,
        error: None,
    })
}

/// 调用AI进行对话（流式返回最终结果）
#[tauri::command]
pub async fn ai_chat_stream(
    prompt: String,
    config: AIConfig,
) -> Result<String, String> {
    if config.api_url.is_empty() || config.model_name.is_empty() {
        return Err("请先在设置中配置AI API".to_string());
    }

    // 构建请求体（流式）
    let request_body = serde_json::json!({
        "model": config.model_name,
        "messages": [
            {"role": "system", "content": "你是CAELab的AI助手，专门帮助用户进行CAE仿真、代码编写、笔记整理等任务。请用中文回答。"},
            {"role": "user", "content": prompt}
        ],
        "max_tokens": config.max_tokens,
        "temperature": config.temperature,
        "stream": true
    });

    let client = reqwest::Client::new();
    let response = client
        .post(&config.api_url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?;

    // 收集所有流式内容
    let mut full_content = String::new();
    let mut stream = response.bytes_stream();

    use futures::StreamExt;
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(bytes) => {
                // 解析SSE格式的chunk
                if let Ok(s) = String::from_utf8(bytes.to_vec()) {
                    for line in s.lines() {
                        if line.starts_with("data: ") {
                            let data = &line[6..];
                            if data == "[DONE]" {
                                continue;
                            }
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                                if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                                    full_content.push_str(content);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                tracing::warn!("Stream error: {}", e);
                break;
            }
        }
    }

    Ok(full_content)
}

/// 测试AI连接
#[tauri::command]
pub async fn test_ai_connection(config: AIConfig) -> Result<bool, String> {
    if config.api_url.is_empty() || config.model_name.is_empty() {
        return Err("请先配置API地址和模型名称".to_string());
    }

    let request_body = serde_json::json!({
        "model": config.model_name,
        "messages": [{"role": "user", "content": "Hi"}],
        "max_tokens": 5
    });

    let client = reqwest::Client::new();
    let response = client
        .post(&config.api_url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("连接失败: {}", e))?;

    Ok(response.status().is_success())
}