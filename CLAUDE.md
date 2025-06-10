# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Running Solutions
- Run a specific day: `cargo run -- YYYY-DD` (e.g., `cargo run -- 2015-01`)
- Build project: `cargo build`
- Check project: `cargo check`

### Development Workflow
- Watch for changes: `just watch` (uses entr to auto-recompile and run 2015-20)
- The justfile contains the watch command that monitors src files

## Architecture

This is an Advent of Code solutions repository written in Rust, organized by year and day.

### Project Structure
- `src/main.rs`: Central dispatcher that matches command-line arguments to specific day implementations
- `src/day.rs`: Defines the `Day` trait that all solutions must implement
- `src/year_YYYY/`: Contains individual day solutions for each year
- `input/YYYY/`: Contains puzzle input files organized by year
- `src/util/`: Shared utilities (A*, cardinal directions, priority queues, etc.)
- `src/data/`: Common data structures (Point2D, StringIdMap)

### Implementation Pattern
Each day solution:
1. Implements the `Day` trait with a `main() -> Result<()>` function
2. Reads input from `input/YYYY/dayDD.txt`
3. Solves both parts of the puzzle and prints results
4. Is imported in main.rs with year-specific aliases (e.g., `y2015_d01`)

### Adding New Solutions
1. Create `src/year_YYYY/dayDD.rs` implementing the `Day` trait
2. Add the module to `src/year_YYYY/mod.rs`
3. Import and add match case in `src/main.rs`
4. Place input file in `input/YYYY/dayDD.txt`

### Input Loading
The project supports loading puzzle inputs from both local files and the Advent of Code website:
- Local files: `input/YYYY/dayDD.txt` (if present, will be used first)
- Web fetch: Uses `get_input(year, day)` function from `src/input.rs`
- Requires `cookies.txt` file in project root with AOC session cookie for web fetching
- Cookie format: `session=your_session_token_here`

### Key Dependencies
- `anyhow`: Error handling
- `itertools`: Iterator extensions
- `reqwest`: HTTP client for fetching inputs from AOC website
- `serde`/`serde_json`: JSON parsing for some puzzles
- `md5`: Cryptographic hashing
- `lazy-regex`: Regex utilities