# Rust‑Powered Shell (`rspwsh`)

A lightning‑fast, object‑aware command‑line shell written from scratch in **Rust**.
This README corresponds to the repository **[`KartikDua1504/Powershell`](https://github.com/KartikDua1504/Powershell)**.

---

## 1 · Quick Install

```bash
# prerequisite: Rust ≥ 1.77 (install with rustup if missing)

# clone *this* repo
 git clone https://github.com/KartikDua1504/Powershell.git
 cd Powershell

# build an optimised release binary
 cargo build --release        # ≈ 20 s on a recent laptop

# (optional) move it onto $PATH so you can call it from anywhere
 sudo mv target/release/rspwsh /usr/local/bin

# launch an interactive session
 rspwsh
```

Run a one‑shot command and exit:

```bash
rspwsh -c "echo 'hello world'"
```

---

## 2 · Everyday Usage

| Task                   | Example                    |                     |
| ---------------------- | -------------------------- | ------------------- |
| Change directory       | `cd ~/projects`            |                     |
| Install a package      | `sudo dnf install ripgrep` |                     |
| List only large files  | \`ls                       | where size > 10MB\` |
| Search command history | `history -p make`          |                     |
| Exit the shell         | `exit` or **Ctrl+D**       |                     |

**Shortcuts** – Tab = autocomplete · Ctrl+R = reverse search · Ctrl+C = cancel · Ctrl+L = clear.

---

## 3 · How It Works (Bird’s‑Eye View)

```
+-----------+           +-------------------+
| main.rs   |──► input │  command::parser  │  ➜ AST
+-----------+           +-------------------+
                                │
                                ▼
                        +-------------------+
                        │  shell::exec      │  built‑in? spawn?
                        +-------------------+
                                │
                                ▼
                        +-------------------+
                        │ trie::autocomplete│  Tab suggestions
                        +-------------------+
```

* **Zero unsafe code** – memory safety throughout.
* **termion** – raw‑mode terminal handling and key events.
* **serde** – stream JSON/CSV as rich objects between commands.
* **anyhow / thiserror** – ergonomic error propagation.

---

## 4 · Extending `rspwsh`

Adding a built‑in command is a two‑step affair:

```rust
struct Greet;
impl BuiltIn for Greet {
    fn name(&self) -> &'static str { "greet" }
    fn execute(&self, args: &[String]) -> Result<i32, ShellError> {
        println!("Hello, {}!", args.get(0).unwrap_or(&"world".into()));
        Ok(0)
    }
}
```

Register it in `shell/mod.rs`:

```rust
builtins.insert("greet", Box::new(Greet));
```

Re‑compile (`cargo build --release`) and the command is live—complete with autocomplete and `help` support.

---

## 5 · Troubleshooting

| Symptom                       | Remedy                                                          |
| ----------------------------- | --------------------------------------------------------------- |
| Binary won’t execute          | Build directly on the target machine or use correct `--target`. |
| Prompt lacks colour           | Ensure `$TERM` supports ANSI (e.g. `xterm-256color`).           |
| Slow autocomplete (first run) | Trie builds once; subsequent sessions are instant.              |

---
