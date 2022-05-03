# Sniff

A **simple** tool written in rust to read for file changes and accordingly run build commands.

Note this tool is for linux and linux only. If it happens to work on bsd ( which it probably will ) then awesome!

I will provide absolutely no support for windows.

# Config file:

- A global config can be defined in `~/.config/sniff/sniff.json`.
- A local config overrides the global one. Local configs should be placed in the root of the project under the name `sniff.json`

```json
{
  ".*.rs": ["cargo build --release"],
  ".*.zig": ["zig test .", "zig build"],
  "sniff_ignore_dir": ["target"],
  "sniff_ignore_file": ["test.rs"]
}
```

# Sniff ignore:

- Make sniff ignore directories with the `sniff_ignore_dir` key.
- Make sniff ignore files with the `sniff_ignore_file` key.

# Installation:

## Dependencies:

1. git
1. make

## Steps:

1. Clone this repo: `git clone https://github.com/shinyzenith/sniff &cd sniff`
1. `make`
1. `sudo make install`
