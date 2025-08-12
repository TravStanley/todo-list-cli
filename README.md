# Rust To-Do List CLI → Team Job Planner

A command-line application for managing tasks, built in Rust.  
Starts as a simple personal to-do list and is designed to grow into a collaborative team job planner.

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

## Planned Features

### Phase 2 – Multi-User / Team Features

- Assign tasks to team members.
- Filter tasks by user, project, or status.
- Import/export task lists for sharing.
- Optional migration from TOML storage to SQLite.

### Phase 3 – Networked Team Planner

- Backend HTTP server for centralized task storage.
- CLI client syncs tasks with server in real time.
- User authentication & access control.
- Possible web or TUI (terminal UI) interface.

---

## Installation

You’ll need [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
# Clone the repository
git clone https://github.com/yourusername/todo-cli.git
cd todo-cli

# Build the project
cargo build --release
```
