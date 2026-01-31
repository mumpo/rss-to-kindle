# RSS to Kindle App - Development Plan

## Project Overview
A Rust CLI application that fetches RSS feeds, extracts article content from RSS descriptions, generates text-only EPUB files, and delivers them to Kindle via email.

## Architecture
- **CLI-first design** with simple, scriptable interface
- **Environment variable configuration** for easy deployment
- **Parallel RSS fetching** for performance
- **Text-only EPUB generation** for smaller files and faster delivery
- **SMTP email delivery** to Kindle addresses

## Directory Structure
```
src/
├── main.rs                 # CLI entry point with argument parsing
├── lib.rs                  # Public API and module exports
├── config/
│   ├── mod.rs             # Configuration module exports
│   └── env.rs             # Environment variable handling and validation
├── feed/
│   ├── mod.rs             # Feed management exports
│   ├── fetcher.rs         # RSS fetching with parallel processing
│   └── processor.rs       # Content extraction from RSS descriptions
├── kindle/
│   ├── mod.rs             # Kindle delivery exports
│   ├── epub_builder.rs    # Text-only EPUB generation
│   └── email_sender.rs    # SMTP email delivery to Kindle
├── models/
│   ├── mod.rs             # Data model exports
│   └── article.rs         # Article and feed data structures
└── error.rs               # Custom error types with proper handling

Cargo.toml                 # Dependencies
.env.example              # Environment variables template
README.md                 # Usage instructions
PLAN.md                   # This file
```

## Configuration (Environment Variables)
```env
# Required
KINDLE_EMAIL=your.kindle@kindle.com
RSS_FEEDS=https://feed1.com/rss,https://feed2.com/rss
SMTP_HOST=smtp.gmail.com
SMTP_USER=your.email@gmail.com
SMTP_PASS=your-app-password

# Optional
SMTP_PORT=587
MAX_ARTICLES=20
EPUB_TITLE=RSS Digest
SMTP_TLS=true
```

## Core Components

### 1. Feed Processing
- **Parallel fetching**: Use `rayon` for concurrent RSS downloads
- **Simple content extraction**: Use RSS `<description>` or `<content:encoded>` fields
- **No deduplication**: Process all articles as received
- **Filtering**: Basic content cleaning (remove HTML tags, normalize whitespace)

### 2. EPUB Generation (Text-only)
- **Structure**: Title page, table of contents, articles
- **Metadata**: Feed source, publication dates, title
- **Formatting**: Clean typography, proper paragraphs
- **Size optimization**: Text-only for faster transfers

### 3. Email Delivery
- **SMTP Integration**: Use `lettre` crate for reliable email sending
- **Attachments**: EPUB file as attachment
- **Error handling**: Retry logic for failed deliveries
- **Logging**: Clear success/failure reporting

### 4. CLI Interface
```bash
# Basic usage
rss-to-kindle

# With custom feed list
rss-to-kindle --feeds "https://feed1.com,https://feed2.com"

# Test mode (no email sending)
rss-to-kindle --dry-run

# Verbose output
rss-to-kindle --verbose
```

## Dependencies to Add
```toml
lettre = "0.11"           # Email sending
clap = { version = "4.0", features = ["derive"] }  # CLI parsing
serde = { version = "1.0", features = ["derive"] } # Serialization
rayon = "1.8"             # Parallel processing
regex = "1.10"            # Content cleaning
anyhow = "1.0"            # Error handling
env_logger = "0.11"       # Logging
```

## Implementation Steps

### Phase 1: Foundation
1. **Setup**: Update Cargo.toml, create directory structure
2. **Configuration**: Environment variable handling with validation
3. **Data Models**: Article, Feed, and Config structures
4. **Error Handling**: Comprehensive error types and logging

### Phase 2: Core Features
5. **Feed Fetching**: Parallel RSS parsing with the existing `rss` crate
6. **Content Processing**: Text extraction and cleaning
7. **EPUB Building**: Structure generation with `epub-builder`

### Phase 3: Delivery & Interface
8. **Email Delivery**: SMTP integration with `lettre`
9. **CLI Interface**: Command-line argument parsing with `clap`
10. **Testing**: Unit tests and integration tests

## Key Design Decisions

### Content Strategy
- **Simple extraction**: Use RSS description/content fields only
- **Text-only EPUBs**: Smaller files, faster email delivery, no image complexity
- **No deduplication**: Simpler logic, user gets all available content

### Configuration Philosophy
- **Environment variables**: Easy deployment in containers/cron jobs
- **Minimal defaults**: Require explicit configuration for security
- **Validation**: Fail fast with clear error messages for missing config

### Performance Considerations
- **Parallel fetching**: Process multiple feeds simultaneously
- **Memory efficiency**: Stream processing for large feeds
- **Network timeouts**: Prevent hanging on slow feeds

## Cron Integration Example
```bash
# Daily delivery at 7 AM
0 7 * * * /usr/local/bin/rss-to-kindle >> /var/log/rss-to-kindle.log 2>&1

# Every 4 hours during reading hours
0 7,11,15,19 * * * /usr/local/bin/rss-to-kindle --verbose
```

## Success Criteria
- ✅ Successfully fetches RSS feeds from multiple sources
- ✅ Generates valid EPUB files readable on Kindle
- ✅ Delivers EPUBs via email to Kindle addresses
- ✅ Handles configuration errors gracefully
- ✅ Works reliably in automated cron jobs
- ✅ Provides clear logging for troubleshooting

## Future Enhancements (Post-MVP)
- Full article content extraction with readability parsing
- Image support with optimization
- Feed deduplication and filtering
- Web UI for feed management
- Database-backed state tracking
- Kindle Personal Document sync support