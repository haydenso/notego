# Notego

A lightweight Rust CLI tool to export Apple Notes to Markdown (.mdx) files.

## Features

- Export notes from a specific Apple Notes folder
- Converts rich text to clean Markdown (headings, lists, code blocks, formatting, links)
- Generates YAML frontmatter with title, slug, date, and description
- **User frontmatter override**: Specify custom frontmatter in your notes to override defaults
- Extracts embedded images as separate files with relative links
- Preserves note metadata (uses creation date by default)
- Single static binary - no dependencies
- Always overwrites existing files (no confirmation needed)

## Installation

### Clone repository

```bash
# Clone this repository (replace with the real repo URL)
git clone https://github.com/OWNER/notego.git
cd notego
```

### Build from source

```bash
# Clone or navigate to the notego directory
cd notego

# Build the release binary
cargo build --release

# The binary will be at target/release/notego
# Optionally, copy it to your PATH
cp target/release/notego /usr/local/bin/
```

## Usage

### Basic usage

Export all notes from a folder called "logs":

```bash
notego --folder "logs"
```

This will create `.mdx` files in the `./out` directory.

### Advanced usage

```bash
notego --folder "logs" \
  --out ./my-notes \
  --ext md \
  --date created \
  --desc-lines 5 \
  --force
```

### Command-line options

| Option | Description | Default |
|--------|-------------|---------|
| `--folder, -f` | Notes folder name to export (required) | - |
| `--out, -o` | Output directory path | `./out` |
| `--ext, -e` | File extension for exported files | `mdx` |
| `--date, -d` | Date field to use (`created` or `modified`) | `created` |
| `--desc-lines` | Number of lines to extract for description | `3` |
| `--attachments` | Include image attachments | `true` |
| `--dry-run` | Preview without writing files | `false` |

## Output format

Each note is exported as a file with YAML frontmatter:

```markdown
---
title: On Reinforcement Learning
slug: on-reinforcement-learning
date: December 01 2025
desc: "First few lines of the note content..."
---

## Your note content

- Bullet points preserved
- **Formatting** maintained
- Images extracted to `attachments/` folder

![figure](attachments/on-reinforcement-learning/rl-figure.png)
```

### User frontmatter override

You can specify custom frontmatter in your Notes to override the auto-generated values. Simply add YAML frontmatter at the start of your note:

```markdown
---
title: My Custom Title
slug: my-custom-slug
date: January 1 2025
desc: "My custom description"
---

Your note content here...
```

Any field you specify will override the default generated value. Fields you don't specify will be auto-generated as usual.

### File naming

Files are named using the slug: `[slug].mdx`

Example: `on-reinforcement-learning.mdx`

### Image attachments

When `--attachments` is enabled (default), embedded images are:
1. Decoded from base64 data URIs
2. Saved to `attachments/<note-slug>/img-N.png`
3. Referenced in markdown with relative paths

## macOS Permissions

The first time you run `notego`, macOS will prompt you to allow Terminal (or your shell) to control the Notes app. Click "OK" to grant permission.

If you see an error about permissions:
1. Open System Settings > Privacy & Security > Automation
2. Ensure your terminal app has permission to control Notes

## Examples

### Export journal entries

```bash
notego --folder "Journal" --out ~/Documents/journal-export
```

### Export with creation dates

```bash
notego --folder "Ideas" --desc-lines 10
```

Note: Creation dates are used by default. Use `--date modified` if you prefer modification dates.

### Dry run to preview

```bash
notego --folder "logs" --dry-run
```

### Export without images

```bash
notego --folder "Technical Notes" --attachments=false
```

## How it works

1. Uses AppleScript (JXA) to query the Notes app for notes in the specified folder
2. Retrieves note metadata (title, dates) and HTML body content
3. Converts HTML to Markdown using the `html2md` crate
4. Extracts base64-encoded images and saves them as separate files
5. Generates YAML frontmatter from note metadata
6. Writes `.mdx` files with relative image references

## Troubleshooting

### "No notes found in folder"

- Check the folder name spelling (case-sensitive)
- Make sure the folder exists in the Notes app
- Try viewing the folder in Notes to ensure it contains notes

### "AppleScript failed"

- Grant automation permissions in System Settings
- Make sure Notes.app is not disabled or restricted
- Try running Notes.app manually first to ensure it works

### Images not exporting

- Ensure `--attachments` is enabled (it's on by default)
- Some image formats may not be embedded as data URIs - these will be skipped with a warning
- Check that the output directory has write permissions

## Similar projects

- [notes2md](https://github.com/vacekj/notes2md) - Go-based Notes exporter

## License

MIT
