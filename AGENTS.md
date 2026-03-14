# AGENTS.md

## Project

- Canonical project name: `ytlist`
- Stack: Rust CLI packaged with Tauri v2
- Primary executable name: `ytlist`

## Working rules

- Keep the app CLI-first; no GUI behavior should be introduced unless explicitly requested.
- Prefer minimal dependencies and straightforward Rust code.
- When updating docs or examples, use `ytlist` consistently.
- Do not commit build outputs from `src-tauri/target/`.
- For public GitHub presentation, align README and release formatting with the chosen reference repo.
- For this repository, the current formatting reference is `ResumePlayer`.
- Keep public `main` clean and publication-ready; keep real development history on local/private `dev`.
- Exclude agent-only files such as `AGENTS.md` from the public `main` snapshot unless explicitly requested.

## Verification

- Preferred validation command: `cargo check` from `src-tauri/`
