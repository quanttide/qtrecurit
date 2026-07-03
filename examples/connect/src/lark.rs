/// 调用 lark-cli 命令，返回解析后的 JSON。
///
/// 会自动过滤掉 lark-cli 输出的 `tip:` 前缀行（来自配置提示），
/// 只保留纯 JSON 部分进行解析。
pub fn run_lark(args: &[&str]) -> Result<serde_json::Value, String> {
    let output = std::process::Command::new("lark-cli")
        .args(args)
        .output()
        .map_err(|e| format!("failed to run lark-cli: {}", e))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    // 过滤掉 lark-cli 的 tip 行（非 JSON 的提示信息）
    let filtered: String = stdout
        .lines()
        .filter(|l| !l.starts_with("tip:"))
        .collect::<Vec<_>>()
        .join("\n");
    serde_json::from_str(&filtered).map_err(|e| format!("JSON parse error: {}", e))
}
