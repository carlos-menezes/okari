# okari

> a stupidly simple static site generator

# **~ this is a work in progress ~**

## Testing

If you so insist:

1. `mkdir <folder>`
2. Add Markdown files inside `<folder>`, e.g.
    ```sh
    folder/
    ├── bye.md
    └── hello_world.md
    ```
3. Build with `cargo build`
4. Generate with `./target/debug/okari <folder>`

This will convert the files and slot them into `build`.

Example file:

```md
title: Byeeee World!
+++

# Bye World

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec odio metus, facilisis sed ipsum nec, consectetur laoreet augue. Ut vitae consequat ligula.

1. first
2. second
3. third


[carlos-menezes.com](https://carlos-menezes.com)
```

## Future
This is an ever expandable list of features:
- [ ] Handle subdirectories (at the moment, only flat dirs are supported)
- [ ] Generate common metatags
  - [ ] `image`, `description` and `date` must be present in the frontmatter