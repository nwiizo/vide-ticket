# vibe-ticket Performance Benchmarks

This directory contains comprehensive performance benchmarks for the vibe-ticket system to ensure it meets the 10x performance goal.

## Running Benchmarks

### Run all benchmarks
```bash
cargo bench
```

### Run specific benchmark suite
```bash
cargo bench ticket_operations
cargo bench storage_operations
cargo bench cli_operations
```

### Run specific benchmark
```bash
cargo bench bench_ticket_save
```

### Generate HTML reports
```bash
cargo bench -- --verbose
# Reports will be in target/criterion/
```

## Benchmark Suites

### 1. Ticket Operations (`ticket_operations.rs`)
Benchmarks core ticket functionality:
- **Single ticket operations**: Save, load
- **Bulk operations**: Loading 10/100/1000 tickets
- **Search operations**: Text search across tickets
- **Filter operations**: By status, priority, complex filters
- **Sort operations**: By creation date, priority

### 2. Storage Operations (`storage_operations.rs`)
Benchmarks storage layer performance:
- **Serialization**: YAML serialization of small/large tickets
- **Deserialization**: YAML parsing performance
- **File I/O**: Reading and writing ticket files
- **Directory operations**: Listing large numbers of tickets
- **Project state**: Save/load operations
- **Concurrent operations**: Multiple threads reading tickets

### 3. CLI Operations (`cli_operations.rs`)
Benchmarks command-line interface performance:
- **Output formatting**: Table and JSON formatting for ticket lists
- **Input validation**: Slug validation
- **Tag parsing**: Simple and complex tag strings
- **Date parsing**: Relative and ISO date formats
- **Enum parsing**: Priority and status parsing
- **String operations**: Slug-to-title conversion, truncation

## Performance Goals

Based on the 10x performance requirement, target benchmarks:
- Single ticket load: < 1ms
- Load 100 tickets: < 10ms
- Load 1000 tickets: < 100ms
- Search 1000 tickets: < 50ms
- Format 100 tickets (table): < 5ms

## Analyzing Results

After running benchmarks:

1. **Check regression**: Compare with previous runs
   ```bash
   cargo bench -- --baseline main
   ```

2. **View detailed reports**: Open `target/criterion/report/index.html`

3. **Profile hotspots**: Use with profiling tools
   ```bash
   cargo bench --profile profile ticket_operations
   ```

## Optimization Tips

Based on benchmark results:

1. **I/O Optimization**
   - Batch file operations
   - Use memory-mapped files for large datasets
   - Implement caching layer

2. **Serialization**
   - Consider binary formats for internal storage
   - Lazy deserialization for large tickets
   - Stream processing for bulk operations

3. **Concurrency**
   - Implement read-write locks for storage
   - Use thread pools for parallel operations
   - Consider async I/O for better scalability

## Continuous Performance Monitoring

To ensure performance doesn't regress:

1. Run benchmarks in CI on every PR
2. Set performance budgets for critical operations
3. Alert on performance regressions > 10%
4. Maintain benchmark history for trend analysis