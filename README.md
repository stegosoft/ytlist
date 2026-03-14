# ytlist

English | [繁體中文](./README.zh-TW.md)

ytlist is a lightweight command-line tool built with Tauri and Rust. It extracts
all video URLs from a YouTube playlist in the original playlist order and is
designed for scripting, archiving, and batch workflows.

## Features

- Extract video URLs in playlist order
- Print results to stdout by default
- Save output to a text file with `-o`
- Use `yt-dlp` for robust playlist parsing
- Keep the project small and CLI-first

## Download

Prebuilt binaries are published on [GitHub Releases](../../releases).

- Windows: standalone `.exe`, installer `.msi`, and `.zip`
- Source code: zip / tarball on each tagged release

## Tech Stack

- Desktop shell: Tauri 2
- Backend: Rust, Tokio, Serde
- External dependency: `yt-dlp`

## Local Development

Requirements:

- Rust stable
- `yt-dlp` available on `PATH`
- Windows is the primary supported build target today

Build a release binary:

```bash
cd src-tauri
cargo build --release
```

Useful commands:

```bash
cargo check --manifest-path src-tauri/Cargo.toml
src-tauri/target/release/ytlist.exe -p "https://www.youtube.com/playlist?list=PL123456"
```

## Usage

Print URLs to stdout:

```bash
ytlist -p "https://www.youtube.com/playlist?list=PL123456"
```

Write URLs to a file:

```bash
ytlist -p "https://www.youtube.com/playlist?list=PL123456" -o playlist.txt
```

## Project Structure

- `src-tauri/`: Rust source, Tauri config, generated schemas, and icons
- `README.md`: English documentation
- `README.zh-TW.md`: Traditional Chinese documentation
- `.github/workflows/`: GitHub Actions release workflow

## Notes

- `yt-dlp` must be installed separately and available on `PATH`
- Invalid, private, or empty playlists return a non-zero exit code
- No GUI window is shown during normal CLI usage

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](./CONTRIBUTING.md) for the
expected workflow and verification steps.

## License

This project is licensed under the [MIT License](./LICENSE).
