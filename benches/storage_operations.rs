//! Benchmarks for storage operations
//!
//! This module benchmarks the performance of storage-related operations
//! including file I/O, serialization, and deserialization.

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use std::fs;
use tempfile::TempDir;
use vibe_ticket::core::{ProjectState, Ticket};
use vibe_ticket::storage::FileStorage;

/// Create a large ticket with lots of data
fn create_large_ticket(id: usize) -> Ticket {
    let mut ticket = Ticket::new(
        &format!("202507230000-large-ticket-{}", id),
        &format!(
            "Large Test Ticket {} with Very Long Title for Benchmarking Purposes",
            id
        ),
    );

    // Add a large description
    ticket.description = (0..100)
        .map(|i| format!("This is line {} of a very long description. It contains detailed information about the ticket, including requirements, specifications, and implementation details. ", i))
        .collect::<Vec<_>>()
        .join("\n");

    // Add many tags
    ticket.tags = (0..50).map(|i| format!("tag-{}", i)).collect();

    // Add many tasks
    for i in 0..20 {
        ticket.add_task(&format!(
            "Task {} - Implement feature X with detailed requirements",
            i
        ));
    }

    // Add metadata
    for i in 0..10 {
        ticket.metadata.insert(
            format!("custom_field_{}", i),
            serde_json::json!({
                "value": format!("Custom value {}", i),
                "timestamp": chrono::Utc::now(),
                "metadata": {
                    "nested": true,
                    "level": i,
                }
            }),
        );
    }

    ticket
}

/// Benchmark YAML serialization
fn bench_yaml_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("yaml_serialization");

    group.bench_function("serialize_small_ticket", |b| {
        let ticket = Ticket::new("test-1", "Small Ticket");
        b.iter(|| {
            black_box(serde_yaml::to_string(&ticket).unwrap());
        });
    });

    group.bench_function("serialize_large_ticket", |b| {
        let ticket = create_large_ticket(1);
        b.iter(|| {
            black_box(serde_yaml::to_string(&ticket).unwrap());
        });
    });

    group.finish();
}

/// Benchmark YAML deserialization
fn bench_yaml_deserialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("yaml_deserialization");

    group.bench_function("deserialize_small_ticket", |b| {
        let ticket = Ticket::new("test-1", "Small Ticket");
        let yaml = serde_yaml::to_string(&ticket).unwrap();
        b.iter(|| {
            black_box(serde_yaml::from_str::<Ticket>(&yaml).unwrap());
        });
    });

    group.bench_function("deserialize_large_ticket", |b| {
        let ticket = create_large_ticket(1);
        let yaml = serde_yaml::to_string(&ticket).unwrap();
        b.iter(|| {
            black_box(serde_yaml::from_str::<Ticket>(&yaml).unwrap());
        });
    });

    group.finish();
}

/// Benchmark file I/O operations
fn bench_file_io(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_io");

    group.bench_function("write_ticket_file", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let ticket = create_large_ticket(1);
                let yaml = serde_yaml::to_string(&ticket).unwrap();
                let path = temp_dir.path().join("ticket.yaml");
                (yaml, path, temp_dir)
            },
            |(yaml, path, _temp_dir)| {
                fs::write(&path, &yaml).unwrap();
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("read_ticket_file", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let ticket = create_large_ticket(1);
                let yaml = serde_yaml::to_string(&ticket).unwrap();
                let path = temp_dir.path().join("ticket.yaml");
                fs::write(&path, &yaml).unwrap();
                (path, temp_dir)
            },
            |(path, _temp_dir)| {
                black_box(fs::read_to_string(&path).unwrap());
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Benchmark directory operations
fn bench_directory_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("directory_operations");

    group.bench_function("list_ticket_files", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let tickets_dir = temp_dir.path().join("tickets");
                fs::create_dir_all(&tickets_dir).unwrap();

                // Create 100 ticket files
                for i in 0..100 {
                    let path = tickets_dir.join(format!("ticket-{}.yaml", i));
                    fs::write(&path, format!("ticket: {}", i)).unwrap();
                }

                (tickets_dir, temp_dir)
            },
            |(tickets_dir, _temp_dir)| {
                let entries: Vec<_> = fs::read_dir(&tickets_dir)
                    .unwrap()
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.path()
                            .extension()
                            .and_then(|s| s.to_str())
                            .map(|s| s == "yaml")
                            .unwrap_or(false)
                    })
                    .collect();
                black_box(entries);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Benchmark project state operations
fn bench_project_state(c: &mut Criterion) {
    let mut group = c.benchmark_group("project_state");

    group.bench_function("save_project_state", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let storage = FileStorage::new(temp_dir.path());
                storage.ensure_directories().unwrap();
                let state = ProjectState {
                    name: "benchmark-project".to_string(),
                    description: Some("A project for benchmarking".to_string()),
                    created_at: chrono::Utc::now(),
                };
                (storage, state, temp_dir)
            },
            |(storage, state, _temp_dir)| {
                storage.save_state(&state).unwrap();
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("load_project_state", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let storage = FileStorage::new(temp_dir.path());
                storage.ensure_directories().unwrap();
                let state = ProjectState {
                    name: "benchmark-project".to_string(),
                    description: Some("A project for benchmarking".to_string()),
                    created_at: chrono::Utc::now(),
                };
                storage.save_state(&state).unwrap();
                (storage, temp_dir)
            },
            |(storage, _temp_dir)| {
                black_box(storage.load_state().unwrap());
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Benchmark concurrent operations
fn bench_concurrent_operations(c: &mut Criterion) {
    use std::sync::Arc;
    use std::thread;

    let mut group = c.benchmark_group("concurrent_operations");

    group.bench_function("concurrent_reads", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let storage = Arc::new(FileStorage::new(temp_dir.path()));
                storage.ensure_directories().unwrap();

                // Create tickets
                let mut ticket_ids = Vec::new();
                for i in 0..10 {
                    let ticket = Ticket::new(&format!("ticket-{}", i), &format!("Ticket {}", i));
                    storage.save(&ticket).unwrap();
                    ticket_ids.push(ticket.id);
                }

                (storage, ticket_ids, temp_dir)
            },
            |(storage, ticket_ids, _temp_dir)| {
                let handles: Vec<_> = (0..4)
                    .map(|thread_id| {
                        let storage = Arc::clone(&storage);
                        let ids = ticket_ids.clone();
                        thread::spawn(move || {
                            for (i, id) in ids.iter().enumerate() {
                                if i % 4 == thread_id {
                                    black_box(storage.load(id).unwrap());
                                }
                            }
                        })
                    })
                    .collect();

                for handle in handles {
                    handle.join().unwrap();
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_yaml_serialization,
    bench_yaml_deserialization,
    bench_file_io,
    bench_directory_operations,
    bench_project_state,
    bench_concurrent_operations
);
criterion_main!(benches);
