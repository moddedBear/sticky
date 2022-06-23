# Sticky
A command line note manager that tries to balance simplicity and usability. Sort of like putting sticky notes up around your workspace but without the clutter.

## Installation
```
cargo install sticky
```

## Usage
```
USAGE:
    sticky [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add       add a new sticky note
    clear     remove sticky notes marked as done, or all notes if the `-a` flag is given
    done      toggle the completion status of a note
    help      Print this message or the help of the given subcommand(s)
    remove    remove a sticky note
```
Running `sticky` with no subcommands will display a list of all your current notes.

Notes are stored in a plain text file for simplicity's sake which gives the added benefit of easy programmatic or batch editing. The format is one note per line, with lines beginning with an asterisk (\*) marking a note as completed. The notes file will be located in one of the following directories depending on your OS.

### Linux
`~/.local/share/sticky or $XDG_DATA_HOME/sticky`

### Mac
`~/Library/Application Support/xyz.moddedBear.sticky`

### Windows
`C:\Users\{username}\AppData\Roaming\moddedBear\sticky\data`

## Usage Ideas
- Add sticky to your shell config to see your notes every time you launch a new terminal
- Modify your shell's "clear screen" keybinding (most likely Alt+L) to run sticky after you clear your screen.
