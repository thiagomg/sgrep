# Simple Grep (sgrep)

A simple grep util for those lazy to remember many command line options

## Usage

```bash
# Displays lines containing "pub struct" string in all .rs files in the local directory
sgrep -p "pub struct" *.rs

# Displays lines containing "#ifdef" or "#ifndef" in all *.c* and *.h* files
sgrep -p "#ifdef" -p "#ifndef" -f .c -f .h
```
