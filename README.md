# Rust To-Do List CLI → Team Job Planner

A command-line application for managing tasks, built in Rust.  
Starts as a simple personal to-do list and can be built upon to grow into a collaborative team job planner.

---

## Features (Phase 1: Personal To-Do List)

- Add, edit, delete, and complete tasks.
- Each task includes:
  - **ID** (auto-generated)
  - **Title**
  - **Description** (optional)
  - **Priority** (`Low`, `Medium`, `High`) - will use an enum
  - **Status** (`Pending`, `In Progress`, `Done`) - will use an enum
  - **Due date** (optional)
- Store tasks locally in a **TOML file** for simplicity.
- Easy-to-use CLI interface with clear commands and options.

---

## Design Choices

- **Storage: TOML for Phase 1**  
  Tasks are persisted in a human-readable TOML file. This keeps the project simple, easy to debug, and requires no external database setup. TOML also integrates well with Rust via the `serde` ecosystem.  
  _Planned upgrade:_ move to SQLite or another database when multi-user/team features require more scalability.

- **Enums for Priority and Status**  
  Instead of free-form strings, enums provide compile-time safety and prevent invalid values. They also allow for easier future extension (e.g., adding new statuses without breaking existing code).

- **CLI-First Approach**  
  Starting with a command-line interface keeps the project lightweight, scriptable, and developer-friendly. It also provides a solid foundation for future UI layers (TUI or web client).

- **Phased Roadmap**  
  Development is intentionally split into phases:
  1. Personal To-Do List (local, simple storage).
  2. Upgrade to a sqLite backend.
  3. Migrate to a real time server with interactions through server functions
     This ensures incremental progress without overengineering in early stages.

## Installation

You’ll need [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
# Clone the repository
git clone https://github.com/yourusername/todo-cli.git
cd todo-cli

# Build the project
cargo build --release
```
