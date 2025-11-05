# Rust To-Do CLI

A simple and elegant command-line to-do list application built with Rust.

## Features

- ✅ Add tasks to your to-do list
- 📋 List all tasks (with filters for completed/pending)
- ✓ Mark tasks as complete
- 🗑️ Delete tasks
- 🧹 Clear all tasks
- 💾 Persistent storage (saved to `~/.rust-todo.json`)

## Installation

### Prerequisites

- Rust and Cargo installed on your system ([Install Rust](https://www.rust-lang.org/tools/install))

### Build

```bash
cargo build --release
```

The executable will be located at `target/release/rust-todo`.

### Run without building

```bash
cargo run -- [command]
```

## Usage

### Add a task

```bash
cargo run -- add "Buy groceries"
# or after building:
./target/release/rust-todo add "Buy groceries"
```

### List all tasks

```bash
cargo run -- list
```

### List only completed tasks

```bash
cargo run -- list --completed
# or
cargo run -- list -c
```

### List only pending tasks

```bash
cargo run -- list --pending
# or
cargo run -- list -p
```

### Mark a task as complete

```bash
cargo run -- complete 1
```

### Delete a task

```bash
cargo run -- delete 1
```

### Clear all tasks

```bash
cargo run -- clear --yes
# or
cargo run -- clear -y
```

## Examples

```bash
# Add some tasks
cargo run -- add "Learn Rust"
cargo run -- add "Build a CLI app"
cargo run -- add "Write documentation"

# List all tasks
cargo run -- list

# Mark task 1 as complete
cargo run -- complete 1

# List only pending tasks
cargo run -- list --pending

# Delete task 2
cargo run -- delete 2

# Clear all tasks (requires confirmation)
cargo run -- clear --yes
```

## Data Storage

Tasks are stored in JSON format at `~/.rust-todo.json`. The file is automatically created when you add your first task.

## License

MIT


