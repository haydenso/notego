# Notego Changelog

## v0.2.0 - Major Update

### New Features

- **User frontmatter override**: Add custom YAML frontmatter in your notes to override auto-generated values
  - Specify custom `title`, `slug`, `date`, or `desc` in your notes
  - Any unspecified fields will be auto-generated as usual
- **Creation date by default**: Now uses note creation date instead of modification date
  - Use `--date modified` to switch back to modification date
- **Always overwrite**: Files are now always overwritten (no `--force` flag needed)
  - Simplifies workflow - just run and export

### Improvements

- Better link handling with alt text preservation
- Cleaner markdown output for all link types
- Improved frontmatter parsing and stripping

### Breaking Changes

- Default date changed from `modified` to `created`
- Removed `--force` flag (always overwrites now)

## v0.1.0 - Initial Release

### Features

- Export Apple Notes to Markdown (.mdx) files via AppleScript/JXA
- YAML frontmatter generation with:
  - `title`: Note title
  - `slug`: URL-friendly slug (also used as filename)
  - `date`: Formatted date (Month DD YYYY)
  - `desc`: Auto-generated description from first N lines
- Smart content processing:
  - Strips existing frontmatter from note content
  - Removes duplicate title headings (both `# Title` and underline-style)
  - Converts HTML to clean Markdown
  - Preserves formatting (bold, italic, links, lists)
- Image attachment handling:
  - Extracts base64-encoded images
  - Saves to `attachments/[slug]/` folder
  - Updates markdown with relative paths
- CLI options:
  - `--folder`: Specify Notes folder to export (required)
  - `--out`: Output directory (default: ./out)
  - `--ext`: File extension (default: mdx)
  - `--date`: Use creation or modification date (default: modified)
  - `--desc-lines`: Number of lines for description (default: 3)
  - `--attachments`: Enable/disable image extraction (default: true)
  - `--dry-run`: Preview without writing files
  - `--force`: Overwrite existing files

### Technical Details

- Lightweight Rust implementation (4.1MB binary)
- Uses AppleScript/JXA for robust Notes access
- Dependencies:
  - `clap`: CLI argument parsing
  - `serde`/`serde_json`: JSON handling
  - `html2md`: HTML to Markdown conversion
  - `slug`: URL-friendly slug generation
  - `chrono`: Date parsing and formatting
  - `regex`: Pattern matching for images
  - `base64`: Image data decoding
  - `anyhow`: Error handling

### Known Limitations

- Requires macOS with Notes.app
- Needs Terminal automation permission for Notes
- Only supports embedded base64 images (file:// attachments not yet supported)
- Date format is fixed to "Month DD YYYY"
