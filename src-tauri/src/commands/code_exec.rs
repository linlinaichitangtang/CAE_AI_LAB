use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use tauri::AppHandle;

#[derive(serde::Serialize, Clone)]
pub struct CodeOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

#[tauri::command]
pub async fn execute_code(
    _app: AppHandle,
    language: String,
    code: String,
    working_dir: Option<String>,
) -> Result<CodeOutput, String> {
    // 根据 language 选择解释器
    let (cmd, args) = match language.as_str() {
        "python" | "python3" => ("python3", vec!["-c", &code]),
        "javascript" | "js" | "node" => ("node", vec!["-e", &code]),
        "typescript" | "ts" => ("npx", vec!["ts-node", "-e", &code]),
        "rust" => {
            // Rust 代码需要写入临时文件再编译运行
            let tmp_dir = std::env::temp_dir();
            let tmp_file = tmp_dir.join("caelab_temp.rs");
            std::fs::write(&tmp_file, &code).map_err(|e| e.to_string())?;
            return run_command("rustc", vec![tmp_file.to_str().unwrap(), "-o", &format!("{}/caelab_temp", tmp_dir.display())], working_dir.as_deref())
                .and_then(|_| run_command(&format!("{}/caelab_temp", tmp_dir.display()), vec![], working_dir.as_deref()));
        }
        _ => return Err(format!("Unsupported language: {}", language)),
    };

    run_command(cmd, args, working_dir.as_deref())
}

fn run_command(cmd: &str, args: Vec<&str>, working_dir: Option<&str>) -> Result<CodeOutput, String> {
    let mut command = Command::new(cmd);
    command.args(&args);
    if let Some(dir) = working_dir {
        command.current_dir(dir);
    }
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn().map_err(|e| format!("Failed to start {}: {}", cmd, e))?;

    // 读取 stdout 和 stderr
    let stdout = BufReader::new(child.stdout.take().unwrap())
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<_>>()
        .join("\n");

    let stderr = BufReader::new(child.stderr.take().unwrap())
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<_>>()
        .join("\n");

    let status = child.wait().map_err(|e| e.to_string())?;

    Ok(CodeOutput {
        stdout,
        stderr,
        exit_code: status.code().unwrap_or(-1),
    })
}
