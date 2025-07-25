//! Benchmarks for CLI operations
//!
//! This module benchmarks the performance of command-line interface operations
//! including argument parsing, output formatting, and command execution.

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use vibe_ticket::cli::output::OutputFormatter;
use vibe_ticket::core::{Priority, Status, Ticket};

/// Create test tickets for benchmarking
fn create_test_tickets(count: usize) -> Vec<Ticket> {
    (0..count)
        .map(|i| {
            let mut ticket = Ticket::new(
                &format!("202507230000-bench-{}", i),
                &format!("Benchmark Ticket {}", i),
            );
            ticket.description = format!("Description for ticket {}", i);
            ticket.priority = match i % 4 {
                0 => Priority::Low,
                1 => Priority::Medium,
                2 => Priority::High,
                _ => Priority::Critical,
            };
            ticket.status = match i % 5 {
                0 => Status::Todo,
                1 => Status::Doing,
                2 => Status::Review,
                3 => Status::Blocked,
                _ => Status::Done,
            };
            ticket.tags = vec![format!("tag{}", i % 10), "benchmark".to_string()];
            ticket
        })
        .collect()
}

/// Benchmark output formatting for different formats
fn bench_output_formatting(c: &mut Criterion) {
    let mut group = c.benchmark_group("output_formatting");

    // Benchmark table output formatting
    group.bench_function("format_tickets_table", |b| {
        let tickets = create_test_tickets(100);
        let formatter = OutputFormatter::new(false, false);
        
        b.iter(|| {
            // Simulate table formatting by building the output string
            let mut output = String::new();
            output.push_str("ID       Status     Priority   Title                                    Tasks\n");
            output.push_str("──────────────────────────────────────────────────────────────────────────────────────────\n");
            
            for ticket in &tickets {
                let line = format!(
                    "{:<8} {:<10} {:<10} {:<40} {}/{}",
                    &ticket.id.to_string()[..8],
                    format!("{:?}", ticket.status),
                    format!("{:?}", ticket.priority),
                    if ticket.title.len() > 37 {
                        format!("{}...", &ticket.title[..37])
                    } else {
                        ticket.title.clone()
                    },
                    ticket.tasks.iter().filter(|t| t.completed).count(),
                    ticket.tasks.len()
                );
                output.push_str(&line);
                output.push('\n');
            }
            
            black_box(output);
        });
    });

    // Benchmark JSON output formatting
    group.bench_function("format_tickets_json", |b| {
        let tickets = create_test_tickets(100);
        
        b.iter(|| {
            let json = serde_json::json!({
                "tickets": tickets,
                "count": tickets.len(),
                "summary": {
                    "total": tickets.len(),
                    "by_status": {
                        "todo": tickets.iter().filter(|t| t.status == Status::Todo).count(),
                        "doing": tickets.iter().filter(|t| t.status == Status::Doing).count(),
                        "done": tickets.iter().filter(|t| t.status == Status::Done).count(),
                    },
                    "by_priority": {
                        "critical": tickets.iter().filter(|t| t.priority == Priority::Critical).count(),
                        "high": tickets.iter().filter(|t| t.priority == Priority::High).count(),
                        "medium": tickets.iter().filter(|t| t.priority == Priority::Medium).count(),
                        "low": tickets.iter().filter(|t| t.priority == Priority::Low).count(),
                    }
                }
            });
            black_box(serde_json::to_string(&json).unwrap());
        });
    });

    group.finish();
}

/// Benchmark slug validation
fn bench_slug_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("slug_validation");

    group.bench_function("validate_valid_slug", |b| {
        let slug = "fix-login-bug";
        b.iter(|| {
            // Simulate slug validation
            let is_valid = slug
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
            black_box(is_valid);
        });
    });

    group.bench_function("validate_invalid_slug", |b| {
        let slug = "Fix_Login Bug!";
        b.iter(|| {
            // Simulate slug validation
            let is_valid = slug
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
            black_box(is_valid);
        });
    });

    group.finish();
}

/// Benchmark tag parsing
fn bench_tag_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("tag_parsing");

    group.bench_function("parse_simple_tags", |b| {
        let tags_str = "bug,feature,urgent";
        b.iter(|| {
            let tags: Vec<String> = tags_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            black_box(tags);
        });
    });

    group.bench_function("parse_complex_tags", |b| {
        let tags_str = "bug, feature , urgent,  ,duplicate,performance-issue,ui/ux";
        b.iter(|| {
            let tags: Vec<String> = tags_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            black_box(tags);
        });
    });

    group.finish();
}

/// Benchmark date parsing
fn bench_date_parsing(c: &mut Criterion) {
    use chrono::{DateTime, Local, TimeZone, Utc};

    let mut group = c.benchmark_group("date_parsing");

    group.bench_function("parse_relative_date", |b| {
        b.iter(|| {
            let date_str = "3 days ago";
            let captures = regex::Regex::new(r"^(\d+)\s+days?\s+ago$")
                .unwrap()
                .captures(date_str);

            if let Some(captures) = captures {
                if let Some(days_str) = captures.get(1) {
                    if let Ok(days) = days_str.as_str().parse::<i64>() {
                        let date = Local::now() - chrono::Duration::days(days);
                        black_box(date);
                    }
                }
            }
        });
    });

    group.bench_function("parse_iso_date", |b| {
        let date_str = "2025-07-23";
        b.iter(|| {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                let datetime = date.and_hms_opt(0, 0, 0).unwrap().and_utc();
                black_box(datetime);
            }
        });
    });

    group.finish();
}

/// Benchmark priority/status parsing
fn bench_enum_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("enum_parsing");

    group.bench_function("parse_priority", |b| {
        let priorities = vec!["low", "medium", "high", "critical"];
        b.iter(|| {
            for priority_str in &priorities {
                let priority = Priority::try_from(*priority_str);
                black_box(priority);
            }
        });
    });

    group.bench_function("parse_status", |b| {
        let statuses = vec!["todo", "doing", "review", "blocked", "done"];
        b.iter(|| {
            for status_str in &statuses {
                let status = Status::try_from(*status_str);
                black_box(status);
            }
        });
    });

    group.finish();
}

/// Benchmark string operations common in CLI
fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");

    group.bench_function("slug_to_title", |b| {
        let slug = "fix-login-bug-with-oauth-integration";
        b.iter(|| {
            let title = slug
                .split('-')
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            black_box(title);
        });
    });

    group.bench_function("truncate_long_string", |b| {
        let long_string = "This is a very long string that needs to be truncated for display purposes in the terminal output";
        let max_length = 40;
        b.iter(|| {
            let truncated = if long_string.len() > max_length {
                format!("{}...", &long_string[..max_length - 3])
            } else {
                long_string.to_string()
            };
            black_box(truncated);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_output_formatting,
    bench_slug_validation,
    bench_tag_parsing,
    bench_date_parsing,
    bench_enum_parsing,
    bench_string_operations
);
criterion_main!(benches);
