use std::collections::BTreeMap;

use chrono::NaiveDate;
use chrono::Datelike;

use crate::connect::config;
use crate::connect::email;

pub fn build_title(start: Option<NaiveDate>, end: Option<NaiveDate>, days: Option<u32>) -> String {
    match (start, end, days) {
        (Some(s), Some(e), None) => {
            format!(
                "量潮招聘数据统计 ({}/{} 至 {}/{})",
                s.month(),
                s.day(),
                e.month(),
                e.day()
            )
        }
        (Some(s), None, None) => format!("量潮招聘数据统计 ({} 起)", s),
        (_, _, Some(d)) => format!("量潮招聘数据统计 (最近 {} 天)", d),
        _ => "量潮招聘数据统计".to_string(),
    }
}

pub fn format_report(
    items: &[&email::MailItem],
    rules: &[config::PositionRule],
    title: &str,
) -> String {
    let total = items.len();
    let mut positions: BTreeMap<&str, usize> = BTreeMap::new();
    let mut unnamed_subjects: Vec<&str> = Vec::new();
    let mut daily: BTreeMap<String, usize> = BTreeMap::new();

    for m in items {
        let subj = m.subject.trim();
        let cat = if subj.is_empty() {
            None
        } else {
            config::classify(subj, rules)
        };

        match cat {
            Some(pos) => {
                *positions.entry(pos).or_insert(0) += 1;
            }
            None => {
                unnamed_subjects.push(subj);
            }
        }

        if let Some(d) = email::extract_date(&m.date) {
            *daily.entry(d.to_string()).or_insert(0) += 1;
        }
    }

    let mut out = String::new();

    let identified = total - unnamed_subjects.len();
    let identified_pct = if total > 0 {
        identified * 100 / total
    } else {
        0
    };

    out.push_str(&format!("# {}\n\n", title));
    out.push_str(&format!("{} 封投递。\n", total));
    if total > 0 {
        out.push_str(&format!(
            "其中可识别岗位 {} 封（{}%），其余为自动回复、空主题等。\n",
            identified, identified_pct
        ));
    }
    out.push('\n');

    out.push_str("## 岗位分布\n\n");
    out.push_str("| 岗位 | 人数 |\n");
    out.push_str("|------|------|\n");
    let mut sorted: Vec<_> = positions.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    for (pos, count) in &sorted {
        out.push_str(&format!("| {} | {} |\n", pos, count));
    }
    out.push('\n');

    if !daily.is_empty() {
        let avg = daily.values().sum::<usize>() as f64 / daily.len() as f64;
        let max_day = daily.iter().max_by_key(|&(_, c)| c).unwrap();
        out.push_str("## 投递趋势\n\n");
        out.push_str(&format!(
            "> 日均投递：{:.1} 封，最高峰：{}（{} 封）\n\n",
            avg, max_day.0, max_day.1
        ));

        out.push_str("| 日期 | 投递数 | 趋势 |\n");
        out.push_str("|------|--------|------|\n");
        let mut prev_count: Option<usize> = None;
        for (d, count) in &daily {
            let arrow = match prev_count {
                Some(prev) if *count > prev => "↑",
                Some(prev) if *count < prev => "↓",
                _ => "-",
            };
            out.push_str(&format!("| {} | {} | {} |\n", d, count, arrow));
            prev_count = Some(*count);
        }
        out.push('\n');
    }

    if !unnamed_subjects.is_empty() {
        out.push_str(&format!(
            "## 未识别邮件样本（前{}条）\n\n",
            unnamed_subjects.len().min(10)
        ));
        out.push_str("建议根据以下样本调整分类规则：\n\n");
        for subj in unnamed_subjects.iter().take(10) {
            let display = if subj.is_empty() { "【空主题】" } else { subj };
            out.push_str(&format!("- {}\n", display));
        }
        out.push('\n');
    }

    out
}

/// 从已获取的邮件条目生成报表
pub fn generate_report_from_items(
    items: &[email::MailItem],
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    days: Option<u32>,
) -> String {
    let cfg = config::load_config();
    let filtered = email::filter_by_date(items, start, end);
    let title = build_title(start, end, days);
    format_report(&filtered, &cfg.rules, &title)
}

/// 报表编排
pub fn generate_report(fetcher: &dyn crate::connect::EmailFetcher) -> anyhow::Result<String> {
    generate_report_with_range(fetcher, None, None, None)
}

/// 带日期范围的报表编排
pub fn generate_report_with_range(
    fetcher: &dyn crate::connect::EmailFetcher,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    days: Option<u32>,
) -> anyhow::Result<String> {
    let msgs = fetcher.fetch_all()?;
    let items: Vec<email::MailItem> = msgs
        .into_iter()
        .map(|m| email::MailItem {
            subject: m.subject,
            date: m.date,
        })
        .collect();

    Ok(generate_report_from_items(&items, start, end, days))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_rules() -> Vec<config::PositionRule> {
        config::load_config().rules
    }

    #[test]
    fn test_format_report_contains_title() {
        let rules = test_rules();
        let report = format_report(&[], &rules, "测试报告");
        assert!(report.contains("# 测试报告"));
        assert!(report.contains("0 封投递。"));
    }

    #[test]
    fn test_format_report_with_data() {
        let rules = config::load_config().rules;
        let items = vec![
            email::MailItem { subject: "应聘全栈工程师 - 张三".into(), date: "2026-06-15".into() },
            email::MailItem { subject: "【后端开发】李四".into(), date: "2026-06-16".into() },
        ];
        let refs: Vec<&email::MailItem> = items.iter().collect();
        let report = format_report(&refs, &rules, "测试报告");
        assert!(report.contains("2 封投递。"));
        assert!(report.contains("岗位分布"));
        assert!(report.contains("全栈工程师"));
        assert!(report.contains("投递趋势"));
    }

    #[test]
    fn test_format_report_unnamed_samples() {
        let rules = test_rules();
        let items = vec![
            email::MailItem { subject: "自动回复：感谢投递".into(), date: "2026-06-15".into() },
        ];
        let refs: Vec<&email::MailItem> = items.iter().collect();
        let report = format_report(&refs, &rules, "测试");
        assert!(report.contains("未识别邮件样本"));
        assert!(report.contains("自动回复：感谢投递"));
    }

    #[test]
    fn test_format_report_empty_subject() {
        let rules = test_rules();
        let items = vec![
            email::MailItem { subject: "".into(), date: "2026-06-15".into() },
        ];
        let refs: Vec<&email::MailItem> = items.iter().collect();
        let report = format_report(&refs, &rules, "测试");
        assert!(report.contains("【空主题】"));
    }
}
