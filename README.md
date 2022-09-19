# Sniff

A **simple** tool written in rust to read for file changes and accordingly run build commands.

Note this tool is for linux and linux only. If it happens to work on bsd ( which it probably will ) then awesome!

I will provide absolutely no support for windows.

# Config file:

Sniff is used to develop sniff. Look at `sniff.json` in the project root to understand how sniff is configured.

A detailed explanation is present in `man 5 sniff`.

# Installation:

## Dependencies:

1. git
1. make
1. scdoc (Optional. if present then man pages are transpiled and installed.)

## Steps:

1. Clone this repo: `git clone https://github.com/shinyzenith/sniff &cd sniff`
1. `make`
1. `sudo make install`
