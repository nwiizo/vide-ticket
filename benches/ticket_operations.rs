//! Benchmarks for ticket operations
//!
//! This module contains performance benchmarks for critical ticket operations
//! to ensure the system meets the 10x performance goal.

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use tempfile::TempDir;
use vibe_ticket::core::{Priority, Status, Ticket};
use vibe_ticket::storage::{FileStorage, TicketRepository};

/// Create a test ticket with standard data
fn create_test_ticket(id: usize) -> Ticket {
    let mut ticket = Ticket::new(
        &format!("202507230000-test-ticket-{}", id),
        &format!("Test Ticket {}", id),
    );
    ticket.description = format!("This is a test ticket number {} for benchmarking", id);
    ticket.priority = match id % 4 {
        0 => Priority::Low,
        1 => Priority::Medium,
        2 => Priority::High,
        _ => Priority::Critical,
    };
    ticket.status = match id % 5 {
        0 => Status::Todo,
        1 => Status::Doing,
        2 => Status::Review,
        3 => Status::Blocked,
        _ => Status::Done,
    };
    ticket.tags = vec![
        format!("tag{}", id % 10),
        format!("category{}", id % 5),
        "benchmark".to_string(),
    ];
    ticket
}

/// Benchmark ticket save operations
fn bench_ticket_save(c: &mut Criterion) {
    c.bench_function("ticket_save_single", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let storage = FileStorage::new(temp_dir.path());
                storage.ensure_directories().unwrap();
                let ticket = create_test_ticket(1);
                (storage, ticket, temp_dir)
            },
            |(storage, ticket, _temp_dir)| {
                storage.save(&ticket).unwrap();
            },
            BatchSize::SmallInput,
        );
    });
}

/// Benchmark ticket load operations
fn bench_ticket_load(c: &mut Criterion) {
    c.bench_function("ticket_load_single", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let storage = FileStorage::new(temp_dir.path());
                storage.ensure_directories().unwrap();
                let ticket = create_test_ticket(1);
                storage.save(&ticket).unwrap();
                (storage, ticket.id.clone(), temp_dir)
            },
            |(storage, id, _temp_dir)| {
                black_box(storage.load(&id).unwrap());
            },
            BatchSize::SmallInput,
        );
    });
}

/// Benchmark loading all tickets
fn bench_load_all_tickets(c: &mut Criterion) {
    let mut group = c.benchmark_group("load_all_tickets");

    for size in [10, 100, 1000].iter() {
        group.bench_function(format!("{}_tickets", size), |b| {
            b.iter_batched(
                || {
                    let temp_dir = TempDir::new().unwrap();
                    let storage = FileStorage::new(temp_dir.path());
                    storage.ensure_directories().unwrap();

                    // Create tickets
                    for i in 0..*size {
                        let ticket = create_test_ticket(i);
                        storage.save(&ticket).unwrap();
                    }

                    (storage, temp_dir)
                },
                |(storage, _temp_dir)| {
                    black_box(storage.load_all().unwrap());
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

/// Benchmark ticket search operations
fn bench_ticket_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("ticket_search");

    group.bench_function("search_100_tickets", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let storage = FileStorage::new(temp_dir.path());
                storage.ensure_directories().unwrap();

                // Create 100 tickets
                let mut tickets = Vec::new();
                for i in 0..100 {
                    let ticket = create_test_ticket(i);
                    storage.save(&ticket).unwrap();
                    tickets.push(ticket);
                }

                (tickets, temp_dir)
            },
            |(tickets, _temp_dir)| {
                // Simulate search by filtering tickets
                let query = "test";
                let results: Vec<_> = tickets
                    .iter()
                    .filter(|t| {
                        t.title.to_lowercase().contains(query)
                            || t.description.to_lowercase().contains(query)
                            || t.tags.iter().any(|tag| tag.to_lowercase().contains(query))
                    })
                    .collect();
                black_box(results);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Benchmark ticket filtering operations
fn bench_ticket_filter(c: &mut Criterion) {
    let mut group = c.benchmark_group("ticket_filter");

    group.bench_function("filter_by_status", |b| {
        b.iter_batched(
            || {
                let mut tickets = Vec::new();
                for i in 0..1000 {
                    tickets.push(create_test_ticket(i));
                }
                tickets
            },
            |tickets| {
                let results: Vec<_> = tickets
                    .into_iter()
                    .filter(|t| t.status == Status::Doing)
                    .collect();
                black_box(results);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("filter_by_priority", |b| {
        b.iter_batched(
            || {
                let mut tickets = Vec::new();
                for i in 0..1000 {
                    tickets.push(create_test_ticket(i));
                }
                tickets
            },
            |tickets| {
                let results: Vec<_> = tickets
                    .into_iter()
                    .filter(|t| t.priority == Priority::High)
                    .collect();
                black_box(results);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("filter_complex", |b| {
        b.iter_batched(
            || {
                let mut tickets = Vec::new();
                for i in 0..1000 {
                    tickets.push(create_test_ticket(i));
                }
                tickets
            },
            |tickets| {
                let results: Vec<_> = tickets
                    .into_iter()
                    .filter(|t| {
                        t.status == Status::Doing
                            && (t.priority == Priority::High || t.priority == Priority::Critical)
                            && t.tags.contains(&"benchmark".to_string())
                    })
                    .collect();
                black_box(results);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Benchmark ticket sorting operations
fn bench_ticket_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("ticket_sort");

    group.bench_function("sort_by_created", |b| {
        b.iter_batched(
            || {
                let mut tickets = Vec::new();
                for i in 0..1000 {
                    tickets.push(create_test_ticket(i));
                }
                tickets
            },
            |mut tickets| {
                tickets.sort_by_key(|t| t.created_at);
                black_box(tickets);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sort_by_priority", |b| {
        b.iter_batched(
            || {
                let mut tickets = Vec::new();
                for i in 0..1000 {
                    tickets.push(create_test_ticket(i));
                }
                tickets
            },
            |mut tickets| {
                tickets.sort_by_key(|t| t.priority);
                black_box(tickets);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_ticket_save,
    bench_ticket_load,
    bench_load_all_tickets,
    bench_ticket_search,
    bench_ticket_filter,
    bench_ticket_sort
);
criterion_main!(benches);
