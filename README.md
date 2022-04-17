# okari

> a stupidly simple static site generator

# **~ this is a work in progress ~**

## Testing

If you so insist:

1. `mkdir <folder>`
2. Add Markdown files inside `<folder>` (e.g. `site`), e.g.
    ```sh
    site
    ├── hello_world.md
    ├── index.md
    └── nested
        └── nested
            └── nested
                └── a.md
    ```
3. Build with `cargo build`
4. Generate with `./target/debug/okari site`

This will convert the files and slot them into `build`.

Okari uses [`comrak`](https://docs.rs/comrak/latest/comrak/), a CommonMark and GitHub Flavoured Markdown compatible Markdown parser. Thus, any input **must** be compatible with one of the specs. Example file:

```md
title: Hello World!
+++

# Hello World

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec odio metus, facilisis sed ipsum nec, consectetur laoreet augue. Ut vitae consequat ligula.

1. first
2. second
3. third


[carlos-menezes.com](https://carlos-menezes.com)
```

## Future
This is an ever expandable list of features:
- [x] Handle subdirectories
- [ ] Generate common metatags
  - [ ] `image`, `description` and `date` must be present in the frontmatter