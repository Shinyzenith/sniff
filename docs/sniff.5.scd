sniff(5) "github.com/shinyzenith/sniff" "File Formats Manual"

# NAME

sniff - Executes build commands in response to file modifications

# CONFIG FILE

Sniff uses the JSON file format. Configuration files should always be named *sniff.json*.

- A global config can be defined in *~/.config/sniff/sniff.json*. Sniff attempts to look in your *$XDG_CONFIG_HOME*, failing which it defaults to *~/.config*.
- A local config overrides the global one. Local configs should be placed in the root of the project.

# SYNTAX


All JSON keys apart from the following are treated as patterns to check for file changes:
	- _sniff_ignore_dir_ (Make sniff ignore directories.)
	- _sniff_ignore_file_ (Make sniff ignore files.)
	- _sniff_cooldown_ (State the cooldown between consecutive build commands. Default is 650 ms.)
	- _sniff_clear_term_ (Call "clear" in the invoking terminal before running build commands.)

Each pattern check can either be an object or an array. 
	- If the pattern check if an array then the commands in it are executed.
	- If the pattern check is an object then the following keys are searched for:
		- "commands": The array of commands to execute. "%sniff_file_name%" is a reserved term which is substituted with the file name.
		- "relative_dir": The directory relative to which the commands should be executed.

# EXAMPLE

```
{
  "zig": [
    "make"
  ],
  "c": {
    "commands": [
      "make"
    ],
    "relative_dir": "./nextctl"
  },
  "rs": {
    "commands": [
      "cargo clippy",
      "cargo build --release"
    ],
    "relative_dir": "./nextctl-rs"
  },
  "sniff_ignore_dir": [
    "zig-out",
    "zig-cache",
    ".git",
    "target"
  ],
  "sniff_ignore_file": [],
  "sniff_cooldown": 650
}
```

# AUTHORS

Maintained by Shinyzenith <aakashsensharma@gmail.com>.
For more information about development, see <https://github.com/shinyzenith/sniff>.

# SEE ALSO

*sniff*(1)
