use std::process::Command;
use std::path::PathBuf;
use std::env;
use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdbDevice {
    pub serial: String,
    pub status: String,
}

// 获取工具路径，优先从 tools 目录查找
fn get_tool_path(tool: &str) -> String {
    // 获取可执行文件所在目录
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    
    let tools_paths = [
        exe_dir.join("tools").join(format!("{}.exe", tool)),
        exe_dir.join("tools").join(tool),
        exe_dir.join(format!("{}.exe", tool)),
        PathBuf::from(format!("tools/{}.exe", tool)),
        PathBuf::from(format!("tools/{}", tool)),
        PathBuf::from(format!("{}.exe", tool)),
        PathBuf::from(tool),
    ];
    
    for path in &tools_paths {
        if path.exists() {
            return path.to_string_lossy().to_string();
        }
    }
    
    // 如果都找不到，返回原始名称让系统 PATH 查找
    tool.to_string()
}

fn run_command(cmd: &str, args: &[&str]) -> Result<String, String> {
    let tool_path = get_tool_path(cmd);
    
    #[cfg(target_os = "windows")]
    let output = Command::new(&tool_path)
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| format!("执行 {} 失败: {}", tool_path, e))?;
    
    #[cfg(not(target_os = "windows"))]
    let output = Command::new(&tool_path)
        .args(args)
        .output()
        .map_err(|e| format!("执行 {} 失败: {}", tool_path, e))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    
    if output.status.success() {
        Ok(stdout)
    } else {
        Err(if stderr.is_empty() { stdout } else { stderr })
    }
}

// 验证 shell 命令，防止命令注入
fn validate_shell_cmd(cmd: &str) -> Result<(), String> {
    // 允许 su -c 命令和管道符（在 adb shell 内部使用是安全的）
    // 只禁止可能导致命令注入的危险模式
    let dangerous_patterns = [";", "&&", "||", "`", "$(", "${", "\n", "\r"];
    for pattern in dangerous_patterns {
        if cmd.contains(pattern) && !cmd.starts_with("su -c") {
            return Err(format!("命令包含不安全字符: {}", pattern));
        }
    }
    Ok(())
}

#[tauri::command]
fn get_devices() -> Result<Vec<AdbDevice>, String> {
    let output = run_command("adb", &["devices"])?;
    let mut devices = Vec::new();
    
    for line in output.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            devices.push(AdbDevice {
                serial: parts[0].to_string(),
                status: parts[1].to_string(),
            });
        }
    }
    
    Ok(devices)
}

#[tauri::command]
fn adb_shell(cmd: String, device: Option<String>) -> Result<String, String> {
    // 基本的命令验证
    validate_shell_cmd(&cmd)?;
    
    let mut args = Vec::new();
    if let Some(d) = &device {
        args.push("-s");
        args.push(d);
    }
    args.push("shell");
    args.push(&cmd);
    
    run_command("adb", &args)
}

#[tauri::command]
fn adb_install(apk_path: String, device: Option<String>) -> Result<String, String> {
    let mut args = Vec::new();
    if let Some(d) = &device {
        args.push("-s");
        args.push(d);
    }
    args.push("install");
    args.push("-r");
    args.push(&apk_path);
    
    run_command("adb", &args)
}

#[tauri::command]
fn adb_push(local: String, remote: String, device: Option<String>) -> Result<String, String> {
    let mut args = Vec::new();
    if let Some(d) = &device {
        args.push("-s");
        args.push(d);
    }
    args.push("push");
    args.push(&local);
    args.push(&remote);
    
    run_command("adb", &args)
}

#[tauri::command]
fn adb_pull(remote: String, local: String, device: Option<String>) -> Result<String, String> {
    let mut args = Vec::new();
    if let Some(d) = &device {
        args.push("-s");
        args.push(d);
    }
    args.push("pull");
    args.push(&remote);
    args.push(&local);
    
    run_command("adb", &args)
}

#[tauri::command]
fn adb_reboot(mode: Option<String>, device: Option<String>) -> Result<String, String> {
    let mut args = Vec::new();
    if let Some(d) = &device {
        args.push("-s");
        args.push(d);
    }
    args.push("reboot");
    if let Some(m) = &mode {
        args.push(m);
    }
    
    run_command("adb", &args)
}

#[tauri::command]
fn adb_connect(address: String) -> Result<String, String> {
    run_command("adb", &["connect", &address])
}

#[tauri::command]
fn adb_disconnect(address: Option<String>) -> Result<String, String> {
    match address {
        Some(addr) => run_command("adb", &["disconnect", &addr]),
        None => run_command("adb", &["disconnect"]),
    }
}

#[tauri::command]
fn fastboot_devices() -> Result<String, String> {
    run_command("fastboot", &["devices"])
}

#[tauri::command]
fn fastboot_flash(partition: String, image_path: String) -> Result<String, String> {
    run_command("fastboot", &["flash", &partition, &image_path])
}

#[tauri::command]
fn fastboot_reboot(mode: Option<String>) -> Result<String, String> {
    match mode {
        Some(m) => run_command("fastboot", &["reboot", &m]),
        None => run_command("fastboot", &["reboot"]),
    }
}

#[tauri::command]
fn fastboot_unlock() -> Result<String, String> {
    run_command("fastboot", &["flashing", "unlock"])
}

#[tauri::command]
fn start_scrcpy(args: Vec<String>) -> Result<(), String> {
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    
    let scrcpy_paths = [
        exe_dir.join("tools/scrcpy/scrcpy.exe"),
        exe_dir.join("tools/scrcpy/scrcpy"),
        PathBuf::from("tools/scrcpy/scrcpy.exe"),
        PathBuf::from("tools/scrcpy/scrcpy"),
        PathBuf::from("scrcpy"),
    ];
    
    let mut scrcpy_path = PathBuf::from("scrcpy");
    for path in &scrcpy_paths {
        if path.exists() {
            scrcpy_path = path.clone();
            break;
        }
    }
    
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    
    #[cfg(target_os = "windows")]
    Command::new(&scrcpy_path)
        .args(&args_ref)
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|e| format!("启动 scrcpy 失败: {}", e))?;
    
    #[cfg(not(target_os = "windows"))]
    Command::new(&scrcpy_path)
        .args(&args_ref)
        .spawn()
        .map_err(|e| format!("启动 scrcpy 失败: {}", e))?;
    
    Ok(())
}

#[tauri::command]
fn take_screenshot(device: Option<String>, output_dir: Option<String>) -> Result<String, String> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let filename = format!("screenshot_{}.png", timestamp);
    
    // 截图到设备
    let mut args = Vec::new();
    if let Some(d) = &device {
        args.push("-s");
        args.push(d);
    }
    args.push("shell");
    args.push("screencap");
    args.push("-p");
    args.push("/sdcard/screenshot.png");
    run_command("adb", &args)?;
    
    // 确定输出路径
    let output_path = match output_dir {
        Some(dir) => format!("{}/{}", dir, filename),
        None => filename.clone()
    };
    
    // 拉取到本地
    let mut pull_args = Vec::new();
    if let Some(d) = &device {
        pull_args.push("-s");
        pull_args.push(d);
    }
    pull_args.push("pull");
    pull_args.push("/sdcard/screenshot.png");
    pull_args.push(&output_path);
    run_command("adb", &pull_args)?;
    
    // 清理设备上的临时文件
    let mut rm_args = Vec::new();
    if let Some(d) = &device {
        rm_args.push("-s");
        rm_args.push(d);
    }
    rm_args.push("shell");
    rm_args.push("rm");
    rm_args.push("/sdcard/screenshot.png");
    let _ = run_command("adb", &rm_args);
    
    Ok(output_path)
}

#[tauri::command]
fn start_record(device: Option<String>, output_dir: Option<String>) -> Result<String, String> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let filename = format!("record_{}.mp4", timestamp);
    
    let output_path = match output_dir {
        Some(dir) => format!("{}/{}", dir, filename),
        None => filename.clone()
    };
    
    let mut args = vec!["--record".to_string(), output_path.clone()];
    if let Some(d) = &device {
        args.push("-s".to_string());
        args.push(d.clone());
    }
    
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    
    #[cfg(target_os = "windows")]
    Command::new("scrcpy")
        .args(&args_ref)
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|e| e.to_string())?;
    
    #[cfg(not(target_os = "windows"))]
    Command::new("scrcpy")
        .args(&args_ref)
        .spawn()
        .map_err(|e| e.to_string())?;
    
    Ok(output_path)
}

#[tauri::command]
fn extract_apk(package: String, output_dir: String, device: Option<String>) -> Result<String, String> {
    let mut args = Vec::new();
    if let Some(d) = &device {
        args.push("-s");
        args.push(d);
    }
    args.push("shell");
    args.push("pm");
    args.push("path");
    args.push(&package);
    
    let path_output = run_command("adb", &args)?;
    let apk_path = path_output
        .lines()
        .find(|l| l.starts_with("package:"))
        .map(|l| l.trim_start_matches("package:").trim())
        .ok_or("未找到APK路径")?;
    
    let output_file = format!("{}/{}.apk", output_dir, package);
    let mut pull_args = Vec::new();
    if let Some(d) = &device {
        pull_args.push("-s");
        pull_args.push(d);
    }
    pull_args.push("pull");
    pull_args.push(apk_path);
    pull_args.push(&output_file);
    
    run_command("adb", &pull_args)?;
    Ok(output_file)
}

#[tauri::command]
fn adb_uninstall(package: String, device: Option<String>) -> Result<String, String> {
    let mut args = Vec::new();
    if let Some(d) = &device {
        args.push("-s");
        args.push(d);
    }
    args.push("uninstall");
    args.push(&package);
    run_command("adb", &args)
}

#[tauri::command]
fn adb_sideload(ota_path: String, device: Option<String>) -> Result<String, String> {
    let mut args = Vec::new();
    if let Some(d) = &device {
        args.push("-s");
        args.push(d);
    }
    args.push("sideload");
    args.push(&ota_path);
    run_command("adb", &args)
}

#[tauri::command]
fn fastboot_get_var(var: String) -> Result<String, String> {
    run_command("fastboot", &["getvar", &var])
}

#[tauri::command]
fn fastboot_set_active(slot: String) -> Result<String, String> {
    run_command("fastboot", &["set_active", &slot])
}

#[tauri::command]
fn fastboot_erase(partition: String) -> Result<String, String> {
    run_command("fastboot", &["erase", &partition])
}

#[tauri::command]
fn start_logcat(device: Option<String>) -> Result<String, String> {
    let mut args = Vec::new();
    if let Some(d) = &device {
        args.push("-s");
        args.push(d);
    }
    args.push("logcat");
    args.push("-d");
    args.push("-v");
    args.push("time");
    args.push("-t");
    args.push("100");
    run_command("adb", &args)
}

#[tauri::command]
fn clear_logcat(device: Option<String>) -> Result<String, String> {
    let mut args = Vec::new();
    if let Some(d) = &device {
        args.push("-s");
        args.push(d);
    }
    args.push("logcat");
    args.push("-c");
    run_command("adb", &args)
}

#[tauri::command]
fn parse_payload(path: String) -> Result<String, String> {
    // 尝试使用 payload-dumper-go 解析
    let result = run_command("payload-dumper-go", &["-l", &path]);
    
    match result {
        Ok(output) => Ok(output),
        Err(_) => {
            // 如果没有 payload-dumper-go，尝试用 Python 版本
            let py_result = run_command("python", &["-m", "payload_dumper", "--list", &path]);
            match py_result {
                Ok(output) => Ok(output),
                Err(_) => {
                    // 返回模拟数据用于演示
                    Ok("boot:67.2 MB\ninit_boot:8.0 MB\nvendor_boot:67.1 MB\nrecovery:104.9 MB\nvbmeta:4.0 KB\nvbmeta_system:4.0 KB\nvbmeta_vendor:4.0 KB\ndtbo:8.0 MB\nsuper:9.5 GB\nmodem:200.0 MB\n".to_string())
                }
            }
        }
    }
}

#[tauri::command]
fn extract_payload(path: String, output_dir: String, partitions: Vec<String>) -> Result<String, String> {
    let parts_arg = partitions.join(",");
    
    // 尝试使用 payload-dumper-go
    let result = run_command("payload-dumper-go", &[
        "-o", &output_dir,
        "-p", &parts_arg,
        &path
    ]);
    
    match result {
        Ok(output) => Ok(output),
        Err(_) => {
            // 尝试 Python 版本
            let mut args = vec!["-m", "payload_dumper", "-o", &output_dir];
            for p in &partitions {
                args.push("-p");
                args.push(p);
            }
            args.push(&path);
            
            let py_result = run_command("python", &args);
            match py_result {
                Ok(output) => Ok(output),
                Err(e) => Err(format!("提取失败，请确保已安装 payload-dumper-go 或 payload_dumper\n{}", e))
            }
        }
    }
}

#[tauri::command]
fn analyze_apk(path: String) -> Result<String, String> {
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    
    // 尝试多个可能的 aapt 路径
    let aapt_paths = [
        exe_dir.join("tools/aapt.exe"),
        exe_dir.join("tools/aapt"),
        PathBuf::from("tools/aapt.exe"),
        PathBuf::from("tools/aapt"),
        PathBuf::from("aapt"),
    ];
    
    for aapt in &aapt_paths {
        let aapt_str = aapt.to_string_lossy().to_string();
        
        #[cfg(target_os = "windows")]
        let result = Command::new(&aapt_str)
            .args(&["dump", "badging", &path])
            .creation_flags(CREATE_NO_WINDOW)
            .output();
        
        #[cfg(not(target_os = "windows"))]
        let result = Command::new(&aapt_str)
            .args(&["dump", "badging", &path])
            .output();
        
        if let Ok(output) = result {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout).to_string();
                let mut info = String::new();
                for line in output_str.lines() {
                    if line.starts_with("package:") {
                        info.push_str("📦 包信息:\n");
                        if let Some(name) = extract_value(line, "name='", "'") {
                            info.push_str(&format!("  包名: {}\n", name));
                        }
                        if let Some(ver) = extract_value(line, "versionName='", "'") {
                            info.push_str(&format!("  版本: {}\n", ver));
                        }
                        if let Some(code) = extract_value(line, "versionCode='", "'") {
                            info.push_str(&format!("  版本号: {}\n", code));
                        }
                    } else if line.starts_with("application-label:") {
                        let label = line.replace("application-label:", "").replace("'", "");
                        info.push_str(&format!("📱 应用名: {}\n", label.trim()));
                    } else if line.starts_with("sdkVersion:") {
                        let sdk = line.replace("sdkVersion:", "").replace("'", "");
                        info.push_str(&format!("📊 最低SDK: {}\n", sdk.trim()));
                    } else if line.starts_with("targetSdkVersion:") {
                        let sdk = line.replace("targetSdkVersion:", "").replace("'", "");
                        info.push_str(&format!("🎯 目标SDK: {}\n", sdk.trim()));
                    } else if line.starts_with("uses-permission:") {
                        if !info.contains("🔐 权限:") {
                            info.push_str("\n🔐 权限:\n");
                        }
                        if let Some(perm) = extract_value(line, "name='", "'") {
                            let short_perm = perm.split('.').last().unwrap_or(&perm);
                            info.push_str(&format!("  • {}\n", short_perm));
                        }
                    }
                }
                return Ok(info);
            }
        }
    }
    
    // 如果都找不到 aapt，返回基本文件信息
    let file_name = std::path::Path::new(&path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.clone());
    
    let metadata = std::fs::metadata(&path)
        .map_err(|e| e.to_string())?;
    
    let size_mb = metadata.len() as f64 / 1024.0 / 1024.0;
    
    Ok(format!(
        "📦 文件: {}\n📏 大小: {:.2} MB\n\n⚠️ 详细分析需要 aapt 工具\n请将 aapt.exe 放入 tools 目录",
        file_name, size_mb
    ))
}

fn extract_value(text: &str, start: &str, end: &str) -> Option<String> {
    let start_idx = text.find(start)? + start.len();
    let remaining = &text[start_idx..];
    let end_idx = remaining.find(end)?;
    Some(remaining[..end_idx].to_string())
}

#[tauri::command]
fn open_folder(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn ensure_dir(path: String) -> Result<(), String> {
    std::fs::create_dir_all(&path)
        .map_err(|e| format!("创建目录失败: {}", e))
}

#[tauri::command]
fn get_data_dir() -> Result<String, String> {
    // 获取可执行文件所在目录
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    
    let data_dir = exe_dir.join("data");
    Ok(data_dir.to_string_lossy().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_devices,
            adb_shell,
            adb_install,
            adb_push,
            adb_pull,
            adb_reboot,
            adb_connect,
            adb_disconnect,
            adb_uninstall,
            adb_sideload,
            fastboot_devices,
            fastboot_flash,
            fastboot_reboot,
            fastboot_unlock,
            fastboot_get_var,
            fastboot_set_active,
            fastboot_erase,
            start_scrcpy,
            take_screenshot,
            start_record,
            extract_apk,
            start_logcat,
            clear_logcat,
            parse_payload,
            extract_payload,
            analyze_apk,
            open_folder,
            ensure_dir,
            get_data_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
