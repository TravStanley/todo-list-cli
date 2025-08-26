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

- **Storage: TOML for Phase 1 → SQLite for Phase 2**  
  Tasks are persisted in a human-readable TOML file in Phase 1. This keeps the project simple and requires no external database setup.  
  _Phase 2 upgrade:_ move to **SQLite** using `sqlx` to support more scalable storage and enable multi-user/team features. This will retain task persistence but allow for faster queries and more robust data handling.

- **Enums for Priority and Status**  
  Instead of free-form strings, enums provide compile-time safety and prevent invalid values. They also allow for easier future extension (e.g., adding new statuses without breaking existing code).

- **CLI-First Approach**  
  Starting with a command-line interface keeps the project lightweight, scriptable, and developer-friendly. It also provides a solid foundation for future UI layers (TUI or web client).

- **Phased Roadmap**  
  Development is intentionally split into phases:
  1. **Phase 1:** Personal To-Do List (local TOML storage).
  2. **Phase 2:** Upgrade to a **SQLite backend** for more scalable and robust storage.
  3. **Phase 3:** Migrate to a real-time server with interactions through server functions, enabling collaborative multi-user features.

---

## Phase 2: SQLite Backend Usage Notes

- **Dependencies:**

  - [`sqlx`](https://crates.io/crates/sqlx) for async database access.
  - [`tokio`](https://crates.io/crates/tokio) for async runtime.

- **Database Initialization:**

  - The app will create a SQLite database file if it doesn’t exist.
  - Tables will be automatically created using `sqlx::migrate` or custom setup scripts.

- **Task Persistence:**

  - Tasks are stored in the SQLite database instead of TOML files.
  - All operations (add, edit, delete, complete) now interact with the database.

- **CLI Commands:**

  - Commands remain the same as Phase 1, but under the hood, they query/update the database instead of reading/writing TOML.

- **Future Extensions:**
  - Multi-user support: add user accounts and task ownership.
  - Easier migration to a server backend for collaborative usage.

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
