# Contributing

## Scope

This project focuses on a small, dependable CLI for extracting ordered YouTube
playlist video URLs. Contributions should preserve that scope unless a broader
change is discussed first.

## Development Flow

- Do ongoing development work on `dev`
- Keep `main` clean and release-oriented
- Prefer small, reviewable pull requests
- Update documentation when user-facing behavior changes

## Local Verification

From `src-tauri/`:

```bash
cargo check
```

If you change CLI behavior, also verify:

- `README.md`
- `tauri.conf.json`
- command examples

## Pull Request Notes

- Describe the problem and the behavioral change
- Mention any CLI flag, output, or dependency impact
- Keep commits coherent and avoid unrelated formatting noise

## Code Style

- Prefer straightforward Rust over clever abstractions
- Keep dependencies minimal
- Preserve CLI-first behavior
