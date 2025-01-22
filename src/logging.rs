pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
}

pub struct Logger {
    pub level: LogLevel,
}

impl Logger {
    pub fn error(self, message: &str) {
      self.log(LogLevel::Error, message)
    }

    pub fn warn(self, message: &str) {
      self.log(LogLevel::Warn, message)
    }

    pub fn info(self, message: &str) {
      self.log(LogLevel::Info, message)
    }

    pub fn debug(self, message: &str) {
      self.log(LogLevel::Debug, message)
    }

    fn log(self, level: LogLevel, message: &str) {
      let prefix: &str = match level {
        LogLevel::Error => "[ERROR]",
        LogLevel::Warn => "[WARNING]",
        LogLevel::Debug => "[DEBUG]",
        LogLevel::Info => "[INFO]",
      };

      if self.level as u8 >= level as u8 {
          println!("{} {}", prefix, message)
      }
    }
}
