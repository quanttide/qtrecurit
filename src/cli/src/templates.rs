//! 模板渲染机制（关键机制，保留）。
//!
//! 只提供机制（MailTemplate / render_template / parse_vars），不写具体模板内容——
//! 话术内容随招聘业务命令（动词命名）各自维护。

#[derive(Debug, Clone)]
pub struct MailTemplate {
    pub name: &'static str,
    pub description: &'static str,
    pub subject: &'static str,
    pub body: &'static str,
}

/// 模板变量替换：{{key}} → value
pub fn render_template(template: &MailTemplate, vars: &[(String, String)]) -> String {
    let mut body = template.body.to_string();
    for (k, v) in vars {
        body = body.replace(&format!("{{{{{}}}}}", k), v);
    }
    body
}

/// 解析 --vars "name=张三,company=示例企业" 为键值对列表
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

// ── 话术内容（招聘域业务资产）────────────────────────────────────────────


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_template_vars() {
        let tpl = MailTemplate {
            name: "test",
            description: "",
            subject: "s",
            body: "{{name}}你好，欢迎 {{company}}",
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
