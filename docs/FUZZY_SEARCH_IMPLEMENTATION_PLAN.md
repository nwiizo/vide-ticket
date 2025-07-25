# Fuzzy Search Implementation Plan for vibe-ticket

## Overview
This document outlines the implementation plan for adding fuzzy search capability to the vibe-ticket search functionality. This feature will improve user experience by allowing approximate matches and typo tolerance in search queries.

## Current State Analysis

### Existing Search Implementation
- **Location**: `/src/cli/handlers/search.rs`
- **Current Features**:
  - Exact substring matching (case-insensitive)
  - Regex pattern matching
  - Field-specific search (title, description, tags)
  - JSON and text output formats
  - Match context extraction

### Architecture Considerations
- The search functionality is well-structured with separate functions for different search types
- The `SearchParams` struct already contains all necessary parameters
- Output formatting is cleanly separated from search logic

## Implementation Strategy

### 1. Fuzzy Search Library Selection
**Options Evaluated**:
- **strsim** (v0.11): Lightweight, provides multiple string similarity algorithms
  - Pros: Small footprint, well-maintained, pure Rust
  - Cons: Basic algorithms only
- **fuzzy-matcher** (v0.3): More sophisticated fuzzy matching with scoring
  - Pros: Better matching quality, sublime-text style fuzzy matching
  - Cons: Slightly larger dependency

**Recommendation**: Use `fuzzy-matcher` for better user experience and more intuitive matching.

### 2. Architecture Design

#### New Components
1. **Fuzzy Search Module** (`src/search/fuzzy.rs`):
   ```rust
   pub struct FuzzyMatcher {
       matcher: SkimMatcherV2,
       threshold: f64,
   }
   
   pub struct FuzzyMatch {
       pub score: i64,
       pub matched_indices: Vec<usize>,
   }
   ```

2. **Extended SearchParams**:
   ```rust
   pub struct SearchParams<'a> {
       // ... existing fields ...
       pub fuzzy: bool,
       pub fuzzy_threshold: Option<f64>,
   }
   ```

#### Integration Points
1. Modify `search_tickets()` to handle fuzzy matching
2. Add fuzzy-specific sorting by match score
3. Enhance output to show match quality indicators

### 3. Implementation Steps

#### Phase 1: Core Fuzzy Search (Priority: High)
1. **Add dependency** to `Cargo.toml`:
   ```toml
   fuzzy-matcher = "0.3"
   ```

2. **Create fuzzy search module** with:
   - Fuzzy matcher initialization
   - Score-based matching functions
   - Match highlighting support

3. **Implement fuzzy matching logic**:
   - Match against title, description, and tags
   - Calculate combined scores for multi-field matches
   - Apply configurable score threshold

#### Phase 2: CLI Integration (Priority: Medium)
1. **Add CLI flag** in `src/cli/commands.rs`:
   ```rust
   #[arg(long, help = "Use fuzzy matching for search")]
   fuzzy: bool,
   
   #[arg(long, help = "Minimum fuzzy match score (0-100)", default_value = "60")]
   fuzzy_threshold: Option<f64>,
   ```

2. **Update search handler** to:
   - Switch between exact and fuzzy matching
   - Sort results by fuzzy score when applicable
   - Show match quality in output

#### Phase 3: Enhanced Features (Priority: Low)
1. **Match highlighting**:
   - Highlight matched characters in output
   - Use terminal colors to show match strength

2. **Smart scoring**:
   - Boost scores for matches at word boundaries
   - Prioritize title matches over description matches
   - Consider match density and position

### 4. Testing Strategy

#### Unit Tests
1. Test fuzzy matching algorithms with various inputs
2. Verify score calculations and thresholds
3. Test edge cases (empty strings, special characters)

#### Integration Tests
1. Test CLI with fuzzy flag
2. Verify output format with fuzzy matches
3. Test performance with large ticket sets

#### Example Test Cases
```rust
#[test]
fn test_fuzzy_match_typo_tolerance() {
    // "serch" should match "search"
    let matcher = FuzzyMatcher::new(0.7);
    let result = matcher.match_text("serch", "search functionality");
    assert!(result.is_some());
    assert!(result.unwrap().score > 70);
}

#[test]
fn test_fuzzy_match_abbreviation() {
    // "impl fz src" should match "implement fuzzy search"
    let matcher = FuzzyMatcher::new(0.6);
    let result = matcher.match_text("impl fz src", "implement fuzzy search");
    assert!(result.is_some());
}
```

### 5. Performance Considerations

#### Optimization Strategies
1. **Pre-compute lowercase versions** of searchable text
2. **Cache fuzzy matcher instances** to avoid re-initialization
3. **Implement early termination** for low-scoring matches
4. **Use parallel processing** for large ticket sets

#### Benchmarks to Add
1. Fuzzy search vs exact search performance
2. Impact of threshold values on performance
3. Scaling with number of tickets

### 6. User Experience Enhancements

#### Output Format Examples
```bash
# Current output
Found 3 tickets matching 'search'

# Enhanced fuzzy output
Found 3 tickets matching 'serch' (fuzzy)

📋 improve-ticket-search - Add fuzzy search capability for better ticket discovery
   Match Score: 95% | Priority: medium | Status: doing
   Matched in: title (fuzzy)
   
🔄 implement-search-filters - Add advanced search filters
   Match Score: 78% | Priority: low | Status: todo
   Matched in: title (fuzzy), description (fuzzy)
```

#### Configuration Options
```yaml
# .vibe-ticket/config.yaml
search:
  fuzzy:
    enabled: true
    default_threshold: 0.7
    highlight_matches: true
    boost_title_matches: 1.5
```

### 7. Documentation Updates

#### User Documentation
1. Update help text for search command
2. Add fuzzy search examples to README
3. Document threshold tuning guidelines

#### Developer Documentation
1. Document fuzzy search algorithm choice
2. Add architecture decision record (ADR)
3. Document performance characteristics

## Risk Assessment

### Technical Risks
1. **Performance degradation** with large ticket sets
   - Mitigation: Implement caching and parallel processing
2. **False positives** with low thresholds
   - Mitigation: Sensible defaults and user education

### User Experience Risks
1. **Confusion between exact and fuzzy modes**
   - Mitigation: Clear output indicators
2. **Unexpected matches**
   - Mitigation: Show match scores and reasons

## Success Metrics

1. **Functionality**:
   - Typo tolerance (1-2 character errors)
   - Abbreviation support
   - Partial word matching

2. **Performance**:
   - < 100ms search time for 1000 tickets
   - < 10% overhead vs exact search

3. **User Satisfaction**:
   - Reduced "no results found" scenarios
   - Intuitive match results

## Timeline Estimate

- **Week 1**: Core implementation and testing
- **Week 2**: CLI integration and optimization
- **Week 3**: Documentation and refinement

## Conclusion

This implementation plan provides a comprehensive approach to adding fuzzy search capability to vibe-ticket. The phased approach ensures that core functionality is delivered quickly while allowing for iterative improvements based on user feedback.