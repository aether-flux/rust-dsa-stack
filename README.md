# Undo/Redo CLI
This is a program written in Rust that runs in the terminal. When run, it starts the program which starts a prompt to enter commands.

## Usage
To run this locally, first clone the repo:
```bash
git clone https://github.com/aether-flux/rust-dsa-stack
cd rust-dsa-stack
```

Then run it as you would any Rust project:
```bash
cargo run
```

## Example
```bash
Undo-Redo CLI (Type 'help' for commands)

> help
Commands:
  insert <text> - Add text to the buffer
  undo          - Undo last change
  redo          - Redo last undone change
  show          - Show current buffer
  quit          - Exit program

> insert Hello

> insert , World!

> show
Current buffer: "Hello, World!"

> undo

> show
Current buffer: "Hello"

> insert , Rust!

> show
Current buffer: "Hello, Rust!"

> undo

> show
Current buffer: "Hello"

> redo

> show
Current buffer: "Hello, Rust!"

> quit
```

## Tech Dive
**Language**: `Rust`
**Learnings**: Concept of `Stack` and (kind of) implementing it (although it's basically just a wrapper over `Vec<>`)
**Extra**: Created a TextBuf buffer struct which stores text buffer and handles undo/redo operations on text, and used some basic programming and `io` inputs to build a CLI.

