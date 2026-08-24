//! 模板渲染机制 + 话术内容加载。
//!
//! 话术模板在编译时通过 `include_str!` 嵌入二进制，格式为：
//! - 第一行：邮件主题
//! - 其余行：邮件正文
//!
//! 内容严格照业务实体手册 `quanttide-handbook-of-business-entity`
//! `qtrecurit/connect/content.md`（工作流沟通内容，最新版）。

use std::collections::HashMap;

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

// ── 模板加载（编译时嵌入） ─────────────────────────────────────────

/// 解析模板内容：第一行是主题，其余是正文
fn parse_template(content: &'static str) -> (String, String) {
    let mut lines = content.lines();
    let subject = lines.next().unwrap_or("").to_string();
    let body: String = lines.collect::<Vec<&str>>().join("\n");
    (subject, body)
}

macro_rules! include_template {
    ($name:expr, $desc:expr, $file:expr) => {
        {
            let (subject, body) = parse_template(include_str!($file));
            MailTemplate {
                name: $name,
                description: $desc,
                subject,
                body,
            }
        }
    };
}

/// 全局模板缓存（编译时嵌入，零运行时 I/O）
static TEMPLATES: Lazy<HashMap<&'static str, MailTemplate>> = Lazy::new(|| {
    let mut m = HashMap::new();
    
    let survey = include_template!("survey", "准入问卷发放：候选人投递后，进入筛选流程前", "../templates/survey.txt");
    m.insert("survey", survey);
    
    let invite = include_template!("invite", "邀请进群：准入问卷通过后，正式受邀加入量潮实训基地", "../templates/invite.txt");
    m.insert("invite", invite);
    
    let exam = include_template!("exam", "笔试：发送笔试邀请，候选人以实际成果参与考核", "../templates/exam.txt");
    m.insert("exam", exam);
    
    let interview = include_template!("interview", "面试通知：筛选/考核通过后，安排面试", "../templates/interview.txt");
    m.insert("interview", interview);
    
    m
});

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
