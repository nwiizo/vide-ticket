# Fuzzy Search in vibe-ticket

The fuzzy search feature allows you to find tickets even when your search query contains typos, uses partial words, or doesn't exactly match the ticket content. This makes discovering relevant tickets much easier and more forgiving.

## How to Use Fuzzy Search

### Command Line Usage

To use fuzzy search from the command line, add the `--fuzzy` flag to your search command:

```bash
# Regular search (exact matching)
vibe-ticket search "authentication"

# Fuzzy search (typo-tolerant)
vibe-ticket search "autentication" --fuzzy

# Fuzzy search with partial words
vibe-ticket search "auth" --fuzzy

# Combine with other search options
vibe-ticket search "databse" --fuzzy --title  # Search only in titles
vibe-ticket search "perfrmance" --fuzzy --tags  # Search only in tags
```

### How It Works

The fuzzy search implementation uses the Skim algorithm (similar to fzf) which provides:

1. **Typo Tolerance**: Finds matches even with spelling mistakes
   - "autentication" matches "authentication"
   - "databse" matches "database"

2. **Partial Matching**: Finds tickets with partial word matches
   - "auth" matches "authentication", "authorize", etc.
   - "impl" matches "implement", "implementation", etc.

3. **Smart Scoring**: Results are ranked by relevance
   - Exact matches score higher
   - Matches at word boundaries score higher
   - Shorter distances between matched characters score higher

4. **Field Weighting**: Different fields have different importance
   - Title matches: 2.0x weight
   - Description matches: 1.0x weight
   - Tag matches: 1.5x weight

## Examples

### Finding Tickets with Typos

```bash
# You typed "refactr" instead of "refactor"
vibe-ticket search "refactr" --fuzzy

# Output:
Found 2 tickets matching 'refactr'

🔄 refactor-authentication - Refactor authentication module
   Priority: medium | Status: doing | Matched in: title (score: 85)

📋 refactor-database - Refactor database connection handling
   Priority: low | Status: todo | Matched in: title (score: 85)
```

### Partial Word Search

```bash
# Search for all authentication-related tickets
vibe-ticket search "auth" --fuzzy

# Output:
Found 3 tickets matching 'auth'

🚫 fix-authentication-bug - Fix authentication issues in login flow
   Priority: high | Status: blocked | Matched in: title (score: 90), tags (score: 90)

🔄 improve-auth-performance - Improve authentication performance
   Priority: medium | Status: doing | Matched in: title (score: 95)

📋 oauth-integration - Implement OAuth2 integration
   Priority: medium | Status: todo | Matched in: title (score: 80), description (score: 75)
```

### Field-Specific Fuzzy Search

```bash
# Search only in descriptions with fuzzy matching
vibe-ticket search "optimizaton" --fuzzy --description

# Search only in tags
vibe-ticket search "perfrmance" --fuzzy --tags
```

## Configuration

The fuzzy search behavior can be customized through the API:

```rust
use vibe_ticket::search::{FuzzySearcher, FuzzySearchConfig};

// Custom configuration
let mut config = FuzzySearchConfig {
    min_score: 60,              // Minimum score threshold (0-100)
    search_title: true,         // Search in titles
    search_description: false,  // Skip descriptions
    search_tags: true,          // Search in tags
    case_sensitive: false,      // Case-insensitive search
    max_results: 50,            // Maximum results to return
    title_weight: 3.0,          // Increase title importance
    tag_weight: 2.0,            // Increase tag importance
    description_weight: 1.0,    // Description weight
};

let searcher = FuzzySearcher::with_config(config);
```

## Performance Considerations

- Fuzzy search is more computationally intensive than exact matching
- For large ticket databases (>10,000 tickets), consider using `--limit` to restrict results
- The search is optimized to score and sort only the top matches

## Tips for Effective Fuzzy Search

1. **Start with partial words**: Instead of typing the full word, use the beginning
   - Use "impl" for "implement", "implementation", etc.
   - Use "auth" for "authentication", "authorization", etc.

2. **Don't worry about typos**: The fuzzy matcher will find what you meant
   - "teh" will match "the"
   - "recieve" will match "receive"

3. **Use field filters for precision**: Combine fuzzy search with field filters
   - `--fuzzy --title` for title-only fuzzy search
   - `--fuzzy --tags` for tag-only fuzzy search

4. **Adjust your query if needed**: If you get too many or too few results
   - Make your query more specific for fewer results
   - Make your query more general for more results