# Sniff

A tool written in rust to read for file changes and accordingly run build commands.

# Config file example:

- A global config can be defined in `~/.config/sniff/sniff.json`.
- A local config overrides the global one. Local configs should be placed in the root of the project under the name `sniff.json`

```json
{
  ".*.rs": ["cargo build --release"],
  ".*.zig": ["zig test .", "zig build"]
}
```
