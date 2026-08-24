//! 招聘沟通话术模板（referral / training / exam）与模板渲染机制。
//!
//! 模板机制（渲染/变量解析）是招聘域自持的关键机制；话术内容为本域业务资产。
//! 自 qtcloud-connect CLI 随业务迁入（issue #1）。

// ── 模板机制 ─────────────────────────────────────────────────────────────

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

pub const TEMPLATES: &[MailTemplate] = &[
    MailTemplate {
        name: "referral",
        description: "企业内推沟通话术：向候选人确认是否接受推荐",
        subject: "量潮人才推荐沟通",
        body: r#"你好，

我们认真查看了你的简历，认为你的经验和能力都很突出。考虑到目前量潮的规模还比较小，入职后可能无法完全匹配你的职业期待和发展空间，因此我们正在考虑将一些能力优秀的候选人推荐到更大的平台，帮助你们在更适合的岗位上发挥所长。今天想先听听你对此的想法和意愿。

我们此前与西安交通大学樊老师有合作基础，樊老师的一位学生也曾在量潮实习。目前该学生在另一家公司实习，并负责协助所在公司招聘实习生，使用其个人内推码。基于樊老师为该学生提供的担保，我们与樊老师及该学生之间形成了信任关系，因此正在共同建立以内部推荐为主的招聘渠道，用于向该公司推荐合适的实习生人选。

关于你后续的安排，我们目前是这样考虑的：如果你接受推荐，为了便于统一管理和对外沟通，我们会以"量潮课堂的学生"这一身份为你进行推荐；如果你想继续留在量潮，就保持现在的安排不变；如果你暂时不想接受推荐，也不打算留在量潮，也请直接告诉我们，我们会尊重你的个人决定，不会勉强。这样主要是为了让推荐流程更规范，也避免信息混乱，同时也尊重你自己的选择。"#,
    },
    MailTemplate {
        name: "training",
        description: "实训邀请沟通话术：邀请通过初筛的候选人加入实训基地",
        subject: "量潮实训基地邀请",
        body: r#"{{name}}你好，

感谢你完成量潮科技的准入问卷。经评估，你已通过初筛，正式受邀加入量潮实训基地。

实训基地是量潮科技对外招聘考核的组成部分。你将在这里通过完成真实的工作任务接受考核，以实际产出代替答卷。

请扫码加入实训基地群（见附件二维码），进群后修改昵称为「{{name}}-岗位意向」。

具体考核规则将在群内发布，请关注群公告和资料。

期待在基地见到你。

量潮科技 招聘团队"#,
    },
    MailTemplate {
        name: "exam",
        description: "招聘考核说明话术：邀请候选人直接参与招聘考核",
        subject: "量潮招聘考核邀请",
        body: r#"你好，

我们认真看了你此前提交的材料及招聘流程中的整体表现，认为你目前展现出的能力和潜力符合量潮进一步招聘考核的要求，因此想邀请你直接参与招聘考核，也想先听听你的想法和意愿。

量潮目前的人才选拔以实际成果为核心，考核标准是：在相对开放的环境中，自主发现并提出有价值的问题，通过自己的方式创造实际成果。我们的考核不会以固定题目为主，而是希望你真正创造一个东西，以过程和产出作为判断依据。如果你暂时不适合这种方式，也可以选择实训等其他培养路径，通过阶段化任务逐步积累能力。需要提前说明的是，通过招聘考核代表你达到了进入量潮团队的人才选拔标准，但最终是否进入团队，还要看届时公司的岗位和项目情况。如果暂时没有合适岗位，我们也会优先考虑让你进入长期实训，或保留后续合作的可能。

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
    use super::render_template;

    #[test]
    fn test_find_template_all_three() {
        assert!(find_template("referral").is_some());
        assert!(find_template("training").is_some());
        assert!(find_template("exam").is_some());
        assert!(find_template("unknown").is_none());
    }

    #[test]
    fn test_render_no_unresolved_vars_in_default_templates() {
        // training 模板含 {{name}} 占位符，渲染后必须被替换
        let tpl = find_template("training").unwrap();
        let rendered = render_template(tpl, &[("name".to_string(), "测试".to_string())]);
        assert!(
            !rendered.contains("{{"),
            "渲染后仍有未解析占位符: {:?}",
            rendered
        );
        // referral 和 exam 无占位符
        for t in [
            find_template("referral").unwrap(),
            find_template("exam").unwrap(),
        ] {
            assert!(
                !t.body.contains("{{"),
                "模板 {} 有占位符: {:?}",
                t.name,
                t.body
            );
        }
    }
}
