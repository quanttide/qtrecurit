use crate::connect::EmailFetcher;
use crate::human;

/// 报表编排
pub fn generate_report(fetcher: &dyn EmailFetcher) -> anyhow::Result<String> {
    generate_report_with_range(fetcher, None, None, None)
}

/// 带日期范围的报表编排
pub fn generate_report_with_range(
    fetcher: &dyn EmailFetcher,
    start: Option<chrono::NaiveDate>,
    end: Option<chrono::NaiveDate>,
    days: Option<u32>,
) -> anyhow::Result<String> {
    let cfg = crate::meta::load_config();
    let msgs = fetcher.fetch_all()?;

    let items: Vec<human::report::MailItem> = msgs
        .into_iter()
        .map(|m| human::report::MailItem {
            subject: m.subject,
            date: m.date,
        })
        .collect();

    let filtered = human::report::filter_by_date(&items, start, end);
    let title = human::report::build_title(start, end, days);
    let report = human::report::format_report(&filtered, &cfg.rules, &title);

    Ok(report)
}
