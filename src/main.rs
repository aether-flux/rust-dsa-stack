use std::io::{self, Write};

// // Stack
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    // // Additional Stack Methods
    // fn peek(&self) -> Option<&T> {
    //     self.items.last()
    // }
    //
    // fn is_empty(&self) -> bool {
    //     self.items.is_empty()
    // }
    //
    // fn size(&self) -> usize {
    //     self.items.len()
    // }
}

// // Text Buffer with UNDO/REDO support
struct TextBuf {
    text: String,
    undo_stack: Stack<String>,
    redo_stack: Stack<String>,
}

impl TextBuf {
    fn new() -> Self {
        TextBuf {
            text: String::new(),
            undo_stack: Stack::new(),
            redo_stack: Stack::new()
        }
    }

    fn insert(&mut self, new_text: &str) {
        self.undo_stack.push(self.text.clone());
        self.redo_stack = Stack::new();
        self.text.push_str(new_text);
    }

    fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.text.clone());
            self.text = prev;
        } else {
            println!("Nothing to undo!");
        }
    }

    fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.text.clone());
            self.text = next;
        } else {
            println!("Nothing to redo!");
        }
    }

    fn show(&self) {
        println!("Current buffer: \"{}\"", self.text);
    }
}

fn main() {
    let mut buf = TextBuf::new();

    println!("\nUndo-Redo CLI (Type 'help' for commands)");

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" {
            break;
        } else if input.starts_with("insert ") {
            let text = &input[7..];
            buf.insert(text);
        } else if input == "undo" {
            buf.undo();
        } else if input == "redo" {
            buf.redo();
        } else if input == "show" {
            buf.show();
        } else if input == "help" {
            println!("Commands:");
            println!("  insert <text> - Add text to the buffer");
            println!("  undo          - Undo last change");
            println!("  redo          - Redo last undone change");
            println!("  show          - Show current buffer");
            println!("  quit          - Exit program");
        } else {
            println!("Unknown command. Type 'help' for options.");
        }
    }
}
