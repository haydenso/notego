# Notego - Quick Start

## What you have

A complete Rust CLI tool that exports Apple Notes to Markdown files with:
- YAML frontmatter (title, date, description)
- Clean Markdown conversion
- Image attachment extraction
- Flexible CLI options

## Files created

```
notego/
├── Cargo.toml          # Rust dependencies
├── src/
│   └── main.rs         # Complete implementation
├── README.md           # Full documentation
├── .gitignore          # Git ignore rules
└── example.sh          # Build helper script
```

## Quick test

1. Build the tool:
   ```bash
   cargo build --release
   ```

2. Test with your "logs" folder (dry run first):
   ```bash
   ./target/release/notego --folder "logs" --dry-run
   ```

3. Export for real:
   ```bash
   ./target/release/notego --folder "logs" --out ./exported-notes
   ```

## Key features

- Uses AppleScript/JXA to read Notes (no database hacking)
- Converts HTML → Markdown with preserved formatting
- Extracts embedded images to `attachments/` folder
- Generates frontmatter like:
  ```
  ---
  title: On China and MAIM
  date: December 01 2025
  desc: "First few lines..."
  ---
  ```

## Customization

All in `src/main.rs`:
- Line 149-164: Date parsing/formatting
- Line 166-170: Description extraction
- Line 172-178: Slug/filename generation
- Line 234-284: Image extraction logic

## Build & install

```bash
# Development build (fast, larger binary)
cargo build

# Release build (optimized, smaller binary)
cargo build --release

# Install to PATH
cargo install --path .
```

## Next steps

1. Test with your actual Notes folder
2. Adjust `--desc-lines` if needed
3. Choose `--date created` or `--date modified`
4. Optionally disable images with `--attachments=false`

Enjoy! 🎉
