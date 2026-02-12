# User Frontmatter Override Example

## How to Use Custom Frontmatter

You can add custom YAML frontmatter to any note in Apple Notes to override the auto-generated values.

### Example Note in Apple Notes

```
---
title: My Custom Article
slug: my-article-2025
date: January 15 2025
desc: "This is my custom description that I control"
---

# Content starts here

This is the actual content of my note. The frontmatter above will be parsed
and used instead of auto-generated values.

- The title will be "My Custom Article" instead of the note's title
- The slug will be "my-article-2025" (so file saved as my-article-2025.mdx)
- The date will be "January 15 2025" instead of creation/modification date
- The description will be my custom text

You can omit any field you don't want to customize, and it will be auto-generated.
```

### Exported Result

When you run `notego --folder "MyFolder"`, the above note becomes:

**Filename**: `my-article-2025.mdx`

```markdown
---
title: My Custom Article
slug: my-article-2025
date: January 15 2025
desc: "This is my custom description that I control"
---

# Content starts here

This is the actual content of my note. The frontmatter above will be parsed
and used instead of auto-generated values.

- The title will be "My Custom Article" instead of the note's title
- The slug will be "my-article-2025" (so file saved as my-article-2025.mdx)
- The date will be "January 15 2025" instead of creation/modification date
- The description will be my custom text

You can omit any field you don't want to customize, and it will be auto-generated.
```

## Partial Override Example

You can override just the slug and let everything else auto-generate:

### Note in Apple Notes

```
---
slug: my-custom-url
---

This note will use:
- Auto-generated title (from note name)
- Auto-generated date (creation date)
- Auto-generated description (first 3 lines)
- Custom slug: "my-custom-url"
```

### Result

**Filename**: `my-custom-url.mdx`

```markdown
---
title: [Auto-generated from note name]
slug: my-custom-url
date: [Auto-generated creation date]
desc: "[Auto-generated from first 3 lines]"
---

This note will use:
- Auto-generated title (from note name)
- Auto-generated date (creation date)
- Auto-generated description (first 3 lines)
- Custom slug: "my-custom-url"
```

## Benefits

1. **Control permalinks**: Set custom slugs for consistent URLs
2. **Override titles**: Use different title for export vs. Notes app
3. **Custom dates**: Set publication dates different from creation
4. **Custom descriptions**: Write SEO-friendly descriptions

## Notes

- Frontmatter must be at the very start of the note
- Use standard YAML format with `---` delimiters
- Fields are optional - omit any you don't want to override
- Invalid YAML will be ignored and defaults will be used
