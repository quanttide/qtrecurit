//! 模板渲染机制（关键机制）+ 招聘沟通话术内容。
//!
//! 内容严格照业务实体手册 `quanttide-handbook-of-business-entity`
//! `qtrecurit/connect/content.md`（工作流沟通内容，最新版）。

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

// ── 话术内容（源自业务实体手册 qtrecurit/connect/content.md）────────────

pub const TEMPLATES: &[MailTemplate] = &[
    MailTemplate {
        name: "survey",
        description: "准入问卷发放：候选人投递后，进入筛选流程前",
        subject: "量潮科技准入问卷",
        body: r#"{{name}}你好，感谢你对量潮科技的关注与投递。在继续招聘流程之前，请先完成以下准入问卷：{{link}}

问卷大约需要15-20分钟，请基于真实想法认真作答。这是进入筛选流程的必要条件。未在3个工作日内提交的，申请将被视为未完成。仅提醒一次。

量潮科技HR"#,
    },
    MailTemplate {
        name: "invite",
        description: "邀请进群：准入问卷通过后，正式受邀加入量潮实训基地",
        subject: "量潮实训基地邀请",
        body: r#"{{name}}你好，感谢你完成量潮科技的准入问卷。经评估，你已通过初筛，正式受邀加入量潮实训基地。

实训基地是量潮科技对外招聘考核的组成部分。你将在这里通过完成真实的工作任务接受考核，以实际产出代替答卷。

请扫码加入实训基地群（见附件二维码），进群后修改昵称为「{{name}}-岗位意向」。

具体考核规则将在群内发布，请关注群公告和资料。

期待在基地见到你。

量潮科技 招聘团队"#,
    },
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
    MailTemplate {
        name: "interview",
        description: "面试通知：筛选/考核通过后，安排面试",
        subject: "量潮面试通知",
        body: r#"{{name}}你好，我是量潮科技的HR，感谢你应聘我司的{{position}}。

面试时间：{{time}}
面试形式：飞书线上面试

面试主要围绕你此前提交的材料与实际成果展开，期待与你深入交流。

如有任何问题，请随时与我联系。期待你的表现！

量潮科技 HR"#,
    },
];

pub fn find_template(name: &str) -> Option<&'static MailTemplate> {
    TEMPLATES.iter().find(|t| t.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_template_all_four() {
        assert!(find_template("survey").is_some());
        assert!(find_template("invite").is_some());
        assert!(find_template("assess").is_some());
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
    fn test_assess_template_no_vars() {
        let tpl = find_template("assess").unwrap();
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
