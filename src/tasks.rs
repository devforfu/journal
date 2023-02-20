use std::{fs, io::Write, path::PathBuf};
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: impl AsRef<str>) -> Self {
        let created_at: DateTime<Utc> = now();
        Self {
            text: text.as_ref().to_string(),
            created_at,
        }
    }
}

type IOResult = std::io::Result<()>;

pub fn add_task(journal_path: PathBuf, task: Task) -> IOResult {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    write_task(file, task)
}

fn write_task(writer: impl Write, task: Task) -> IOResult {
    Ok(())
}

fn now() -> DateTime<Utc> {
    if cfg!(test) {
        use chrono::TimeZone;
        Utc.timestamp(1_500_000_000, 0)
    } else {
        Utc::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use serde_json;

    #[test]
    fn creating_a_new_task() {
        let title = "create a new repo";
        let task = Task::new(title);
        assert_eq!(&task.text, title);
    }

    #[test]
    fn serialize_task() {
        let task = Task::new("serialized");
        let result = serde_json::to_string(&task).unwrap();
        assert_eq!(result, r#"{"text":"serialized","created_at":1500000000}"#);
    }

    #[test]
    fn deserialize_task() {
        let task_string = r#"{"text":"deserialized","created_at":1234567890}"#;
        let result: Task = serde_json::from_str(task_string).unwrap();
        assert_eq!(result.text, "deserialized");
        assert_eq!(result.created_at, Utc.timestamp(1234567890, 0));
    }

    #[test]
    fn adding_task_to_a_journal() {
        
    }
}
