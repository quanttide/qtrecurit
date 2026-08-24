//! 模板渲染机制（关键机制，保留）+ 考核域话术内容。
//!
//! 机制（MailTemplate / render_template / parse_vars）与考核（access）话术
//! 内容一并维护；内容源自业务实体手册 qtrecurit/connect/content.md。

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

// ── 考核（access）话术内容 ────────────────────────────────────────────────

pub const TEMPLATES: &[MailTemplate] = &[
    MailTemplate {
        name: "assess",
        description: "招聘考核邀请：邀请材料与流程表现突出的候选人直接参与招聘考核",
        subject: "量潮招聘考核邀请",
        body: r#"你好，

我们认真看了你此前提交的材料及招聘流程中的整体表现，认为你目前展现出的能力和潜力符合量潮进一步招聘考核的要求，因此想邀请你直接参与招聘考核，也想先听听你的想法和意愿。

量潮目前的人才选拔以实际成果为核心，考核标准是：在相对开放的环境中，自主发现并提出有价值的问题，通过自己的方式创造实际成果。我们的考核不会以固定题目为主，而是希望你真正创造一个东西，以过程和产出作为判断依据。

如果你暂时不适合这种方式，也可以选择实训等其他培养路径，通过阶段化任务逐步积累能力。

需要提前说明的是，通过招聘考核代表你达到了进入量潮团队的人才选拔标准，但最终是否进入团队，还要看届时公司的岗位和项目情况。如果暂时没有合适岗位，我们也会优先考虑让你进入长期实训，或保留后续合作的可能。

如果你愿意参与招聘考核，可以直接回复我们，确认意愿后，我们会与你沟通具体考核方式和下一步安排。如果你希望先通过实训参与量潮，或者暂时不打算继续任何后续安排，也可以直接告诉我们。

期待你的回复。"#,
    },
];

pub fn find_template(name: &str) -> Option<&'static MailTemplate> {
    TEMPLATES.iter().find(|t| t.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_template_assess() {
        assert!(find_template("assess").is_some());
        assert!(find_template("unknown").is_none());
    }

    #[test]
    fn test_assess_template_no_unresolved_vars() {
        let tpl = find_template("assess").unwrap();
        assert!(
            !tpl.body.contains("{{"),
            "assess 模板有未解析占位符: {:?}",
            tpl.body
        );
    }

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
