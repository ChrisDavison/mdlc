# mdlc - Markdown Link Checker

[![Rust](https://github.com/ChrisDavison/mdlc/actions/workflows/rust.yml/badge.svg)](https://github.com/ChrisDavison/mdlc/actions/workflows/rust.yml)

The aim of this tool is to read in all parsed markdown files, extract all links (both local, for files, and remote), and then report which of these links are broken.

## TODO

- Get `HashMap<String, Vec<String>>`, representing `Filename: links` pairs
- Split links into `local` and `web`
- Validate each web link (`http` 200 response?)
- Validate each local link
    - If no valid local link is found, search within any given common parent (or use a passed directory as the parent), calculating string-similarity to determine a possible typo?
- Per-file, report:
    - Broken web links
    - Broken local links (optionally with most likely similar file, if similarity above a threshold)

[Readme](./README.md)
