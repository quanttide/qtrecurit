use anyhow::Result;

use crate::connect::EmailFetcher;
use crate::human;

pub struct RecruitmentOrchestrator;

impl RecruitmentOrchestrator {
    pub fn run(fetcher: &dyn EmailFetcher) -> Result<String> {
        let cfg = human::config::load_config();
        let msgs = fetcher.fetch_all()?;

        let items: Vec<human::report::MailItem> = msgs
            .into_iter()
            .map(|m| human::report::MailItem {
                subject: m.subject,
                date: m.date,
            })
            .collect();

        let title = human::report::build_title(None, None, None);
        let items_ref: Vec<&human::report::MailItem> = items.iter().collect();
        let report = human::report::format_report(&items_ref, &cfg.rules, &title);

        Ok(report)
    }

    pub fn run_with_range(
        fetcher: &dyn EmailFetcher,
        start: Option<chrono::NaiveDate>,
        end: Option<chrono::NaiveDate>,
        days: Option<u32>,
    ) -> Result<String> {
        let cfg = human::config::load_config();
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
}
