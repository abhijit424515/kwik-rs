# kwik-rs

A fast, simple CLI todo manager written in Rust.

## Features

- Simple command-line interface
- Color-coded todos based on status and deadlines
- Persistent storage using JSON
- Automatic sorting by deadline
- Support for deadlines with date and time
- Cross-platform support

## Usage

### Adding a todo

```bash
a (date time) task name
```

Example: `a (25 Dec 13:45) Buy Christmas presents`

### Toggle completion

```bash
t index
```

Example: `t 0` toggles completion of first todo

### Delete a todo

```bash
d index
```

Example: `d 2` deletes the third todo

### Edit a todo

```bash
e index new task name
```

Example: `e 1 Updated task description`

## Display Format

Todos are displayed in the following format:

- `[ ]` uncompleted todo
- `[✓]` completed todo
- Green text indicates completed todos
- Red text indicates overdue todos
- Todos are automatically sorted by deadline

## Storage

Todos are stored in a JSON file located at:

- Linux/macOS: `~/.todos`
- Windows: `C:\Users\<username>\.todos`

## Building from Source

```bash
git clone https://github.com/abhijit424515/kwik-rs
cd kwik-rs
cargo build --release
```

## Dependencies

- chrono - Date and time functionality
- colored - Terminal colors
- dirs - Cross-platform directories
- lazy_static - Static regex compilation
- regex - Command parsing
- serde - JSON serialization
- serde_json - JSON handling

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
