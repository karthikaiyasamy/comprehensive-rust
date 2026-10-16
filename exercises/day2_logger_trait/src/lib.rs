pub trait Logger {
    fn log(&self, verbosity: u8, message: &str);
}

pub struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("[v{verbosity}] {message}");
    }
}

pub struct MemoryLogger {
    pub logs: std::sync::Mutex<Vec<(u8, String)>>,
}

impl MemoryLogger {
    pub fn new() -> Self {
        MemoryLogger {
            logs: std::sync::Mutex::new(Vec::new()),
        }
    }
}

impl Logger for MemoryLogger {
    fn log(&self, verbosity: u8, message: &str) {
        self.logs.lock().unwrap().push((verbosity, message.to_string()));
    }
}

pub struct VerbosityFilter<L: Logger> {
    pub max_verbosity: u8,
    pub inner: L,
}

impl<L: Logger> Logger for VerbosityFilter<L> {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_filter() {
        let mem = MemoryLogger::new();
        let filter = VerbosityFilter {
            max_verbosity: 2,
            inner: mem,
        };

        filter.log(1, "important");
        filter.log(2, "normal");
        filter.log(3, "verbose debug");

        let logs = filter.inner.logs.lock().unwrap();
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0].1, "important");
        assert_eq!(logs[1].1, "normal");
    }
}
