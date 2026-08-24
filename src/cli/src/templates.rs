//! 模板渲染机制 + 话术内容加载。
//!
//! 话术模板存储在 `templates/` 目录下的文本文件中，格式为：
//! - 第一行：邮件主题
//! - 其余行：邮件正文
//!
//! 内容严格照业务实体手册 `quanttide-handbook-of-business-entity`
//! `qtrecurit/connect/content.md`（工作流沟通内容，最新版）。

use std::collections::HashMap;
use std::fs;

use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct MailTemplate {
    pub name: &'static str,
    pub description: &'static str,
    pub subject: String,
    pub body: String,
}

/// 模板变量替换：{{key}} → value
pub fn render_template(template: &MailTemplate, vars: &[(String, String)]) -> String {
    let mut body = template.body.clone();
    for (k, v) in vars {
        body = body.replace(&format!("{{{{{}}}}}", k), v);
    }
    body
}

/// 解析 --vars "key=value,key2=value2" 为键值对列表
pub fn parse_vars(raw: Option<&str>) -> Vec<(String, String)> {
    let mut out = Vec::new();
    if let Some(raw) = raw {
        for pair in raw.split(',') {
            if let Some((k, v)) = pair.split_once('=') {
                out.push((k.trim().to_string(), v.trim().to_string()));
            }
        }
    }
    out
}

// ── 模板加载 ────────────────────────────────────────────────────────

/// 模板描述信息（编译时已知，运行时加载内容）
const TEMPLATE_DESCRIPTIONS: &[(&str, &str)] = &[
    ("survey", "准入问卷发放：候选人投递后，进入筛选流程前"),
    ("invite", "邀请进群：准入问卷通过后，正式受邀加入量潮实训基地"),
    ("exam", "笔试：发送笔试邀请，候选人以实际成果参与考核"),
    ("interview", "面试通知：筛选/考核通过后，安排面试"),
];

/// 从 templates/ 目录加载所有模板
fn load_templates_from_files() -> HashMap<String, MailTemplate> {
    let mut templates = HashMap::new();
    
    // 确定模板目录路径（相对于当前工作目录或可执行文件）
    let template_dir = find_template_dir();
    
    for (name, description) in TEMPLATE_DESCRIPTIONS {
        let file_path = template_dir.join(format!("{}.txt", name));
        if let Ok(content) = fs::read_to_string(&file_path) {
            let mut lines = content.lines();
            let subject = lines.next().unwrap_or("").to_string();
            let body: String = lines.collect::<Vec<&str>>().join("\n");
            
            templates.insert(
                name.to_string(),
                MailTemplate {
                    name: name,
                    description: description,
                    subject: subject,
                    body: body,
                },
            );
        }
    }
    
    templates
}

/// 查找模板目录
fn find_template_dir() -> std::path::PathBuf {
    // 优先使用环境变量
    if let Ok(env_dir) = std::env::var("QTRECURIT_TEMPLATES_DIR") {
        return std::path::PathBuf::from(env_dir);
    }
    
    // 开发环境：使用 CARGO_MANIFEST_DIR 定位源码目录
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let src_path = std::path::PathBuf::from(manifest_dir).join("src/templates");
        if src_path.exists() {
            return src_path;
        }
    }
    
    // 生产环境：相对于可执行文件的路径
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let prod_path = exe_dir.join("templates");
            if prod_path.exists() {
                return prod_path;
            }
        }
    }
    
    // 默认返回当前目录下的 templates
    std::path::PathBuf::from("templates")
}

/// 全局模板缓存
static TEMPLATES: Lazy<HashMap<String, MailTemplate>> = Lazy::new(load_templates_from_files);

pub fn find_template(name: &str) -> Option<&'static MailTemplate> {
    TEMPLATES.get(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_template_all_four() {
        assert!(find_template("survey").is_some());
        assert!(find_template("invite").is_some());
        assert!(find_template("exam").is_some());
        assert!(find_template("interview").is_some());
        assert!(find_template("unknown").is_none());
    }

    #[test]
    fn test_template_vars_rendered() {
        for name in ["survey", "invite", "interview"] {
            let tpl = find_template(name).unwrap();
            let vars: Vec<(String, String)> = vec![
                ("name".into(), "张三".into()),
                ("link".into(), "https://example.com/survey".into()),
                ("position".into(), "数据工程师".into()),
                ("time".into(), "6月20日 10:00".into()),
            ];
            let rendered = render_template(tpl, &vars);
            assert!(
                !rendered.contains("{{"),
                "模板 {} 渲染后仍有未解析占位符: {:?}",
                name,
                rendered
            );
        }
    }

    #[test]
    fn test_exam_template_no_vars() {
        let tpl = find_template("exam").unwrap();
        assert!(
            !tpl.body.contains("{{"),
            "assess 模板有占位符: {:?}",
            tpl.body
        );
    }

    #[test]
    fn test_render_template_vars() {
        let tpl = MailTemplate {
            name: "test",
            description: "",
            subject: "s".to_string(),
            body: "{{name}}你好，欢迎 {{company}}".to_string(),
        };
        let rendered = render_template(&tpl, &[("name".to_string(), "张三".to_string())]);
        assert!(rendered.contains("张三你好"));
        assert!(rendered.contains("欢迎 {{company}}"));
        assert!(!rendered.contains("{{name}}"));
    }

    #[test]
    fn test_parse_vars() {
        let vars = parse_vars(Some("name=张三,company=示例企业"));
        assert_eq!(vars.len(), 2);
        assert_eq!(vars[0], ("name".to_string(), "张三".to_string()));
        assert_eq!(vars[1], ("company".to_string(), "示例企业".to_string()));
    }

    #[test]
    fn test_parse_vars_empty() {
        assert!(parse_vars(None).is_empty());
        assert!(parse_vars(Some("noequalsign")).is_empty());
    }
}
