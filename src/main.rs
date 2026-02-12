use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use chrono::DateTime;
use clap::Parser;
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(name = "notego")]
#[command(about = "Export Apple Notes to Markdown files", long_about = None)]
struct Args {
    /// Notes folder name to export (required)
    #[arg(short, long)]
    folder: String,

    /// Output directory path
    #[arg(short, long, default_value = "./out")]
    out: PathBuf,

    /// File extension for exported files
    #[arg(short, long, default_value = "md")]
    ext: String,

    /// Date field to use in frontmatter (created or modified)
    #[arg(short, long, default_value = "created")]
    date: String,

    /// Number of lines to extract for description
    #[arg(long, default_value = "3")]
    desc_lines: usize,

    /// Include attachments (images)
    #[arg(long, default_value = "true")]
    attachments: bool,

    /// Dry run - don't write files
    #[arg(long, default_value = "false")]
    dry_run: bool,
}

#[derive(Deserialize, Debug)]
struct Note {
    #[allow(dead_code)]
    id: String,
    title: String,
    #[serde(rename = "creationDate")]
    creation_date: String,
    #[serde(rename = "modificationDate")]
    modification_date: String,
    #[serde(rename = "bodyHTML")]
    body_html: String,
}

#[derive(Debug, Default)]
struct UserFrontmatter {
    title: Option<String>,
    slug: Option<String>,
    date: Option<String>,
    desc: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate date argument
    if args.date != "created" && args.date != "modified" {
        anyhow::bail!("--date must be either 'created' or 'modified'");
    }

    println!("🗒️  Notego - Exporting notes from folder '{}'", args.folder);

    // Generate and run JXA script
    let jxa_script = generate_jxa_script(&args.folder);
    let output = Command::new("osascript")
        .args(&["-l", "JavaScript", "-e", &jxa_script])
        .output()
        .context("Failed to execute osascript. Make sure you're on macOS.")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("AppleScript failed: {}", stderr);
    }

    // JXA console.log writes to stderr, not stdout
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Try stderr first (JXA console.log), fallback to stdout
    let json_output = if !stderr.trim().is_empty() && stderr.trim().starts_with('[') {
        stderr.as_ref()
    } else if !stdout.trim().is_empty() {
        stdout.as_ref()
    } else {
        anyhow::bail!("AppleScript returned no data. Make sure the folder '{}' exists and you have granted Terminal permission to control Notes.", args.folder);
    };

    let notes: Vec<Note> = serde_json::from_str(json_output).context(format!(
        "Failed to parse notes JSON from AppleScript. Raw output:\n{}",
        json_output
    ))?;

    if notes.is_empty() {
        println!("⚠️  No notes found in folder '{}'", args.folder);
        println!("   Make sure the folder name is correct and contains notes.");
        return Ok(());
    }

    println!("📝 Found {} notes", notes.len());

    // Create output directory
    if !args.dry_run {
        fs::create_dir_all(&args.out)
            .context(format!("Failed to create output directory {:?}", args.out))?;
    }

    let mut exported = 0;
    let mut skipped = 0;

    for note in notes {
        match process_note(&note, &args) {
            Ok(path) => {
                if args.dry_run {
                    println!("  [DRY RUN] Would write: {}", path.display());
                } else {
                    println!("  ✓ {}", path.display());
                }
                exported += 1;
            }
            Err(e) => {
                println!("  ✗ Failed to export '{}': {}", note.title, e);
                skipped += 1;
            }
        }
    }

    println!("\n✨ Done!");
    println!("   Exported: {}", exported);
    if skipped > 0 {
        println!("   Skipped:  {}", skipped);
    }

    Ok(())
}

fn generate_jxa_script(folder_name: &str) -> String {
    format!(
        r#"
ObjC.import('Foundation')
var Notes = Application('Notes')
var folderName = "{}"
var folders = Notes.folders.whose({{name: folderName}})
if (folders.length === 0) {{
  console.log(JSON.stringify([]))
}} else {{
  var notes = folders[0].notes()
  var out = []
  for (var i = 0; i < notes.length; i++) {{
    var n = notes[i]
    out.push({{
      id: n.id ? String(n.id()) : "",
      title: n.name() ? String(n.name()) : "",
      creationDate: n.creationDate() ? String(n.creationDate()) : "",
      modificationDate: n.modificationDate() ? String(n.modificationDate()) : "",
      bodyHTML: n.body() ? String(n.body()) : ""
    }})
  }}
  console.log(JSON.stringify(out))
}}
"#,
        folder_name.replace('"', r#"\""#)
    )
}

fn process_note(note: &Note, args: &Args) -> Result<PathBuf> {
    // Convert HTML to Markdown first
    let mut markdown = html2md::parse_html(&note.body_html);

    // Parse user-specified frontmatter from the note content
    let (user_frontmatter, content_without_frontmatter) = extract_user_frontmatter(&markdown);
    markdown = content_without_frontmatter;

    // Remove the first H1 if it matches the title (Notes adds this automatically)
    markdown = strip_title_heading(&markdown, &note.title);

    // Choose date based on args (or user override)
    let date_str = if args.date == "created" {
        &note.creation_date
    } else {
        &note.modification_date
    };

    // Use user-specified values or generate defaults
    let title = user_frontmatter
        .title
        .as_ref()
        .unwrap_or(&note.title)
        .clone();
    let formatted_date = user_frontmatter.date.clone().unwrap_or_else(|| {
        parse_and_format_date(date_str).unwrap_or_else(|_| date_str.to_string())
    });
    let slug = user_frontmatter
        .slug
        .clone()
        .unwrap_or_else(|| create_slug(&title));

    // Extract and process images if attachments enabled
    let _attachments_dir = if args.attachments {
        Some(extract_images(
            &mut markdown,
            &args.out,
            &slug,
            args.dry_run,
        )?)
    } else {
        None
    };

    // Generate description from first N lines (or use user-specified)
    let description = user_frontmatter
        .desc
        .clone()
        .unwrap_or_else(|| extract_description(&markdown, args.desc_lines));

    // Generate frontmatter
    let frontmatter = format!(
        "---\ntitle: {}\nslug: {}\ndate: {}\ndesc: \"{}\"\n---\n\n",
        title, slug, formatted_date, description
    );

    // Combine frontmatter and content
    let full_content = format!("{}{}", frontmatter, markdown);

    // Generate filename using slug
    let filename = format!("{}.{}", slug, args.ext);
    let filepath = args.out.join(&filename);

    // Write file (always overwrite - force is default)
    if !args.dry_run {
        fs::write(&filepath, full_content)
            .context(format!("Failed to write file {:?}", filepath))?;
    }

    Ok(filepath)
}

fn extract_user_frontmatter(content: &str) -> (UserFrontmatter, String) {
    let lines: Vec<&str> = content.lines().collect();
    let mut frontmatter = UserFrontmatter::default();
    let mut in_frontmatter = false;
    let mut frontmatter_end_idx = 0;
    let mut frontmatter_delimiters = 0;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Detect YAML frontmatter delimiters (--- or \---)
        if trimmed == "---" || trimmed == "\\---" {
            frontmatter_delimiters += 1;
            if frontmatter_delimiters == 1 {
                in_frontmatter = true;
                continue;
            } else if frontmatter_delimiters == 2 {
                frontmatter_end_idx = i + 1;
                break;
            }
        }

        // Parse frontmatter fields
        if in_frontmatter {
            if let Some(colon_idx) = trimmed.find(':') {
                let key = trimmed[..colon_idx].trim();
                let value = trimmed[colon_idx + 1..]
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string();

                match key {
                    "title" => frontmatter.title = Some(value),
                    "slug" => frontmatter.slug = Some(value),
                    "date" => frontmatter.date = Some(value),
                    "desc" => frontmatter.desc = Some(value),
                    _ => {}
                }
            }
        }
    }

    // Return content without the frontmatter section
    let content_lines = if frontmatter_end_idx > 0 {
        &lines[frontmatter_end_idx..]
    } else {
        &lines[..]
    };

    let cleaned_content = content_lines.join("\n").trim().to_string();
    (frontmatter, cleaned_content)
}

fn strip_title_heading(content: &str, title: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut skip_next = false;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // If we're told to skip this line
        if skip_next {
            skip_next = false;
            continue;
        }

        // Skip the first H1 markdown heading if it matches the title
        if trimmed == format!("# {}", title) || trimmed == format!("#{}", title) {
            continue;
        }

        // Skip underline-style H1: if current line matches title and next line is ===
        if i + 1 < lines.len() && trimmed == title {
            let next_line = lines[i + 1].trim();
            if next_line.chars().all(|c| c == '=') && next_line.len() >= 3 {
                skip_next = true; // Skip the underline on next iteration
                continue; // Skip the title line
            }
        }

        // Skip standalone underlines (=== or ---)
        if trimmed.chars().all(|c| c == '=' || c == '-') && trimmed.len() >= 3 {
            // Only skip if previous line was the title or empty
            if i > 0 && (lines[i - 1].trim() == title || lines[i - 1].trim().is_empty()) {
                continue;
            }
        }

        result.push(*line);
    }

    result.join("\n").trim().to_string()
}

fn parse_and_format_date(date_str: &str) -> Result<String> {
    // macOS JXA date format: "Wed Feb 11 2026 00:03:39 GMT+0800 (Hong Kong Standard Time)"
    // We'll parse this manually since it's not RFC 2822 or RFC 3339

    // Try parsing the JXA format: "Day Mon DD YYYY HH:MM:SS GMT+/-HHMM (Timezone Name)"
    let parts: Vec<&str> = date_str.split_whitespace().collect();

    if parts.len() >= 4 {
        // parts[1] = month (Feb), parts[2] = day (11), parts[3] = year (2026)
        let month = parts[1];
        let day = parts[2];
        let year = parts[3];

        // Format as "February 11 2026"
        let month_full = match month {
            "Jan" => "January",
            "Feb" => "February",
            "Mar" => "March",
            "Apr" => "April",
            "May" => "May",
            "Jun" => "June",
            "Jul" => "July",
            "Aug" => "August",
            "Sep" => "September",
            "Oct" => "October",
            "Nov" => "November",
            "Dec" => "December",
            _ => month,
        };

        return Ok(format!("{} {} {}", month_full, day, year));
    }

    // Try RFC 2822 format as fallback
    if let Ok(dt) = DateTime::parse_from_rfc2822(date_str) {
        return Ok(dt.format("%B %d %Y").to_string());
    }

    // Try RFC 3339 format as fallback
    if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
        return Ok(dt.format("%B %d %Y").to_string());
    }

    // If all else fails, return something reasonable
    Ok(date_str.to_string())
}

fn extract_description(markdown: &str, num_lines: usize) -> String {
    let lines: Vec<&str> = markdown
        .lines()
        .map(|l| l.trim())
        .filter(|l| {
            !l.is_empty() 
            && !l.starts_with('#')  // Skip headings
            && !l.starts_with("---") // Skip hr/frontmatter
            && !l.starts_with("===") // Skip underline headings
            && !l.starts_with("\\---") // Skip escaped frontmatter
            && !l.chars().all(|c| c == '=' || c == '-') // Skip horizontal rules
        })
        .take(num_lines)
        .collect();

    lines.join(" ").chars().take(200).collect()
}

fn create_slug(title: &str) -> String {
    if title.is_empty() {
        return "untitled".to_string();
    }

    slug::slugify(title)
}

fn extract_images(
    markdown: &mut String,
    output_dir: &Path,
    note_slug: &str,
    dry_run: bool,
) -> Result<PathBuf> {
    let attachments_dir = output_dir.join("attachments").join(note_slug);

    // Regex to find data URI images
    let re = Regex::new(r#"!\[([^\]]*)\]\(data:image/([^;]+);base64,([^)]+)\)"#)?;

    let mut counter = 1;
    let mut replacements = Vec::new();

    for cap in re.captures_iter(markdown) {
        let alt_text = cap.get(1).map_or("", |m| m.as_str());
        let img_format = cap.get(2).map_or("png", |m| m.as_str());
        let base64_data = cap.get(3).map_or("", |m| m.as_str());

        // Decode base64
        if let Ok(img_data) = general_purpose::STANDARD.decode(base64_data) {
            let img_filename = format!("img-{}.{}", counter, img_format);
            let img_path = attachments_dir.join(&img_filename);

            // Create attachments directory
            if !dry_run {
                fs::create_dir_all(&attachments_dir)?;
                fs::write(&img_path, img_data)?;
            }

            // Relative path from note to image
            let relative_path = format!("attachments/{}/{}", note_slug, img_filename);

            replacements.push((
                cap.get(0).unwrap().as_str().to_string(),
                format!("![{}]({})", alt_text, relative_path),
            ));

            counter += 1;
        }
    }

    // Apply replacements
    for (old, new) in replacements {
        *markdown = markdown.replace(&old, &new);
    }

    Ok(attachments_dir)
}
