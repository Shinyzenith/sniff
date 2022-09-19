sniff(5) "github.com/shinyzenith/sniff" "File Formats Manual"

# NAME

sniff - Executes build commands in response to file modifications

# CONFIG FILE

Sniff uses the JSON file format. Configuration files should always be named *sniff.json*.

- A global config can be defined in *~/.config/sniff/sniff.json*. Sniff attempts to look in your *$XDG_CONFIG_HOME*, failing which it defaults to *~/.config*.
- A local config overrides the global one. Local configs should be placed in the root of the project.

# SYNTAX


All JSON keys apart from the following are treated as regular expressions to
check for file changes:
	- _sniff_ignore_dir_ (Make sniff ignore directories.)
	- _sniff_ignore_file_ (Make sniff ignore files.)
	- _sniff_cooldown_ (State the cooldown between consecutive build commands.)

# EXAMPLE

```
{
  ".*.rs": ["cargo build --release"],
  ".*.zig": ["zig test .", "zig build"],
  "sniff_ignore_dir": ["target"],
  "sniff_ignore_file": ["test.rs"],
  "sniff_cooldown": 650
}
```

# AUTHORS

Maintained by Shinyzenith <aakashsensharma@gmail.com>.
For more information about development, see <https://github.com/shinyzenith/sniff>.

# SEE ALSO

*sniff*(1)
