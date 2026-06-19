pub mod email;
pub mod notice;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Message {
    pub subject: String,
    pub date: String,
}

pub trait EmailFetcher {
    fn fetch_all(&self) -> Result<Vec<Message>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockFetcher {
        messages: Vec<Message>,
    }

    impl EmailFetcher for MockFetcher {
        fn fetch_all(&self) -> Result<Vec<Message>> {
            Ok(self.messages.clone())
        }
    }

    #[test]
    fn test_mock_fetcher_returns_messages() {
        let fetcher = MockFetcher {
            messages: vec![
                Message { subject: "应聘全栈工程师".into(), date: "2026-06-15".into() },
            ],
        };
        let result = fetcher.fetch_all().unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].subject, "应聘全栈工程师");
    }

    #[test]
    fn test_mock_fetcher_empty() {
        let fetcher = MockFetcher { messages: vec![] };
        let result = fetcher.fetch_all().unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_message_debug_and_clone() {
        let msg = Message { subject: "test".into(), date: "2026-06-01".into() };
        let cloned = msg.clone();
        assert_eq!(format!("{:?}", cloned), format!("{:?}", msg));
    }
}
