use tokio::process::Command;
use std::process::Stdio;
use std::error::Error;
use csv::ReaderBuilder;
use std::io::Cursor;
use std::sync::Arc;
use crate::global_state::{GlobalState, Record};

pub async fn parse_inbox(csv_data: &str, state: Arc<GlobalState>, update_state: bool) -> Result<String, Box<dyn Error>> {
    let reader = Cursor::new(csv_data);
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(reader);

    let mut new_records = Vec::new();

    for result in rdr.records() {
        let record = result?;
        if record.get(3) == Some("#Inbox") {
            new_records.push(Record {
                id: record.get(0).unwrap().to_string(),
                priority: record.get(1).unwrap().to_string(),
                name: record.get(5).unwrap().to_string(),
            });
        }
    }

    if update_state {
        let mut records = state.todoist_inbox.lock().await;
        *records = new_records.clone();
    }

    let json = serde_json::to_string(&new_records)?;
    Ok(json)
}

pub async fn runcmd_async(command: &str, args: &[&str]) -> Result<(), Box<dyn Error>> {
    Command::new(command)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}

pub async fn runcmd(command: &str, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if output.status.success() {
        Ok(stdout)
    }
    else {
        Err(format!("Command failed with error: {}", stderr).into())
    }
}
