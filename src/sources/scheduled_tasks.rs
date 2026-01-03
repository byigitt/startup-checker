use std::process::Command;

use crate::error::{Error, Result};
use crate::models::{ItemStatus, SourceType, StartupItem};

use super::StartupSource;

pub struct TaskSchedulerScanner;

impl TaskSchedulerScanner {
    pub fn new() -> Self {
        Self
    }

    fn parse_schtasks_output(&self, output: &str) -> Vec<StartupItem> {
        let mut items = Vec::new();

        // Parse CSV output from schtasks
        let lines: Vec<&str> = output.lines().collect();
        if lines.len() <= 1 {
            return items;
        }

        // Skip header line
        for line in lines.iter().skip(1) {
            let fields: Vec<&str> = line.split(',').map(|s| s.trim_matches('"')).collect();

            if fields.len() < 4 {
                continue;
            }

            let task_name = fields[0].to_string();
            let status_str = fields[2];
            let _next_run = fields[1];

            // Skip system tasks and tasks that don't run at logon/boot
            if task_name.starts_with("\\Microsoft\\") {
                continue;
            }

            let status = if status_str.eq_ignore_ascii_case("Ready")
                || status_str.eq_ignore_ascii_case("Running")
            {
                ItemStatus::Enabled
            } else if status_str.eq_ignore_ascii_case("Disabled") {
                ItemStatus::Disabled
            } else {
                continue;
            };

            // Extract just the task name (remove path)
            let display_name = task_name
                .rsplit('\\')
                .next()
                .unwrap_or(&task_name)
                .to_string();

            let item = StartupItem::new(
                display_name,
                SourceType::ScheduledTask,
                task_name.clone(),
                task_name,
            )
            .with_status(status);

            items.push(item);
        }

        items
    }
}

impl StartupSource for TaskSchedulerScanner {
    fn scan(&self) -> Result<Vec<StartupItem>> {
        // Use schtasks command to list tasks
        let output = Command::new("schtasks")
            .args(["/query", "/fo", "CSV", "/v"])
            .output()
            .map_err(|e| Error::TaskScheduler(format!("Failed to run schtasks: {}", e)))?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);

        // Parse and filter for logon/boot tasks
        let mut items = Vec::new();

        // Parse the verbose CSV output
        let lines: Vec<&str> = output_str.lines().collect();
        if lines.len() <= 1 {
            return Ok(items);
        }

        // Find column indices
        let header = lines[0];
        let headers: Vec<&str> = header.split(',').map(|s| s.trim_matches('"')).collect();

        let task_name_idx = headers.iter().position(|h| h.contains("TaskName"));
        let status_idx = headers.iter().position(|h| h.contains("Status"));
        let trigger_idx = headers.iter().position(|h| h.contains("Trigger") || h.contains("Start"));

        let task_name_idx = match task_name_idx {
            Some(i) => i,
            None => return Ok(items),
        };

        // Process each line
        for line in lines.iter().skip(1) {
            // Parse CSV carefully (fields may contain commas)
            let fields = parse_csv_line(line);

            if fields.len() <= task_name_idx {
                continue;
            }

            let task_name = fields[task_name_idx].clone();

            // Skip Microsoft system tasks
            if task_name.contains("\\Microsoft\\") || task_name.starts_with("Microsoft") {
                continue;
            }

            // Check trigger for logon/boot
            let trigger = trigger_idx
                .and_then(|i| fields.get(i))
                .map(|s| s.as_str())
                .unwrap_or("");

            let is_startup_task = trigger.to_lowercase().contains("logon")
                || trigger.to_lowercase().contains("boot")
                || trigger.to_lowercase().contains("startup");

            if !is_startup_task {
                continue;
            }

            // Get status
            let status_str = status_idx
                .and_then(|i| fields.get(i))
                .map(|s| s.as_str())
                .unwrap_or("");

            let status = if status_str.eq_ignore_ascii_case("Ready")
                || status_str.eq_ignore_ascii_case("Running")
            {
                ItemStatus::Enabled
            } else if status_str.eq_ignore_ascii_case("Disabled") {
                ItemStatus::Disabled
            } else {
                ItemStatus::Enabled
            };

            // Extract display name
            let display_name = task_name
                .rsplit('\\')
                .next()
                .unwrap_or(&task_name)
                .to_string();

            let item = StartupItem::new(
                display_name,
                SourceType::ScheduledTask,
                task_name.clone(),
                format!("Scheduled Task: {}", task_name),
            )
            .with_status(status);

            items.push(item);
        }

        // Deduplicate by task name
        items.sort_by(|a, b| a.name.cmp(&b.name));
        items.dedup_by(|a, b| a.name == b.name);

        Ok(items)
    }

    fn enable(&self, item: &StartupItem) -> Result<()> {
        let output = Command::new("schtasks")
            .args(["/change", "/tn", &item.source_location, "/enable"])
            .output()
            .map_err(|e| Error::TaskScheduler(format!("Failed to run schtasks: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::TaskScheduler(format!(
                "Failed to enable task: {}",
                stderr
            )));
        }

        Ok(())
    }

    fn disable(&self, item: &StartupItem) -> Result<()> {
        let output = Command::new("schtasks")
            .args(["/change", "/tn", &item.source_location, "/disable"])
            .output()
            .map_err(|e| Error::TaskScheduler(format!("Failed to run schtasks: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::TaskScheduler(format!(
                "Failed to disable task: {}",
                stderr
            )));
        }

        Ok(())
    }

    fn source_types(&self) -> Vec<SourceType> {
        vec![SourceType::ScheduledTask]
    }
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in line.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                fields.push(current.trim().to_string());
                current = String::new();
            }
            _ => current.push(ch),
        }
    }

    fields.push(current.trim().to_string());
    fields
}

impl Default for TaskSchedulerScanner {
    fn default() -> Self {
        Self::new()
    }
}
