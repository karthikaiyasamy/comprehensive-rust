pub struct LogEntry<'a> {
    pub level: &'a str,
    pub message: &'a str,
}

pub fn filter_logs<'a, P>(logs: &[LogEntry<'a>], predicate: P) -> Vec<&'a str>
where
    P: Fn(&str) -> bool,
{
    logs.iter()
        .filter(|e| predicate(e.message))
        .map(|e| e.message)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        let logs = [
            LogEntry { level: "INFO", message: "Starting server" },
            LogEntry { level: "ERROR", message: "Database connection failed" },
            LogEntry { level: "WARN", message: "High memory usage" },
        ];
        let matches = filter_logs(&logs, |m| m.contains("Database"));
        assert_eq!(matches, vec!["Database connection failed"]);
    }
}
