use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub text: String,
    pub done: bool,
    pub notify_at: Option<String>, // "HH:mm" format
    pub notified: bool,
    pub created_at: String, // ISO 8601
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStore {
    pub tasks: Vec<Task>,
}

impl TaskStore {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }
}

/// Parse time from input text.
/// Supports patterns:
///   - "HH:mm テキスト" (e.g., "13:00 印刷")
///   - "テキスト HH:mm" (e.g., "資料作成 15:30")
/// Returns (clean_text, Option<time_string>)
pub fn parse_task_input(input: &str) -> (String, Option<String>) {
    let input = input.trim();
    if input.is_empty() {
        return (String::new(), None);
    }

    // Regex pattern for HH:mm (0-23 hours, 0-59 minutes)
    let time_pattern = regex::Regex::new(r"(\d{1,2}):(\d{2})").unwrap();

    if let Some(captures) = time_pattern.find(input) {
        let time_str = captures.as_str();

        // Validate the time
        let parts: Vec<&str> = time_str.split(':').collect();
        if parts.len() == 2 {
            if let (Ok(h), Ok(m)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                if h < 24 && m < 60 {
                    let formatted_time = format!("{:02}:{:02}", h, m);
                    // Remove the time from the text
                    let clean_text = input.replace(captures.as_str(), "").trim().to_string();
                    let clean_text = if clean_text.is_empty() {
                        format!("Task at {}", formatted_time)
                    } else {
                        clean_text
                    };
                    return (clean_text, Some(formatted_time));
                }
            }
        }
    }

    (input.to_string(), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_time_prefix() {
        let (text, time) = parse_task_input("13:00 印刷");
        assert_eq!(text, "印刷");
        assert_eq!(time, Some("13:00".to_string()));
    }

    #[test]
    fn test_parse_time_suffix() {
        let (text, time) = parse_task_input("資料作成 15:30");
        assert_eq!(text, "資料作成");
        assert_eq!(time, Some("15:30".to_string()));
    }

    #[test]
    fn test_parse_no_time() {
        let (text, time) = parse_task_input("牛乳を買う");
        assert_eq!(text, "牛乳を買う");
        assert_eq!(time, None);
    }

    #[test]
    fn test_parse_single_digit_hour() {
        let (text, time) = parse_task_input("9:00 朝会");
        assert_eq!(text, "朝会");
        assert_eq!(time, Some("09:00".to_string()));
    }
}
