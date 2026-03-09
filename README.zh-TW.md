# ytlist

[English](./README.md) | 繁體中文

ytlist 是一個使用 Tauri 與 Rust 製作的輕量命令列工具，可依照原始播放清單
順序擷取 YouTube 播放清單中的所有影片網址，適合腳本、自動化、封存與批次處理
流程。

## 功能

- 依播放清單順序擷取影片網址
- 預設輸出到 stdout
- 可用 `-o` 輸出到文字檔
- 使用 `yt-dlp` 提供穩定的播放清單解析
- 維持小型且 CLI 優先的設計

## 下載

預先建置的執行檔會發佈在 [GitHub Releases](../../releases)。

- Windows: 可直接執行的 `.exe`、安裝用 `.msi`、以及 `.zip`
- 原始碼: 每個 tag release 也會附上 zip / tarball

## 技術棧

- 桌面殼層: Tauri 2
- 後端: Rust、Tokio、Serde
- 外部依賴: `yt-dlp`

## 本機開發

需求：

- Rust stable
- `yt-dlp` 已安裝且可從 `PATH` 取得
- 目前主要支援 Windows 建置

建置 release 版本：

```bash
cd src-tauri
cargo build --release
```

常用指令：

```bash
cargo check --manifest-path src-tauri/Cargo.toml
src-tauri/target/release/ytlist.exe -p "https://www.youtube.com/playlist?list=PL123456"
```

## 用法

輸出到 stdout：

```bash
ytlist -p "https://www.youtube.com/playlist?list=PL123456"
```

輸出到檔案：

```bash
ytlist -p "https://www.youtube.com/playlist?list=PL123456" -o playlist.txt
```

## 專案結構

- `src-tauri/`: Rust 程式碼、Tauri 設定、產生的 schema 與圖示
- `README.md`: 英文說明
- `README.zh-TW.md`: 繁體中文說明
- `.github/workflows/`: GitHub Actions release workflow

## 備註

- `yt-dlp` 需要另外安裝，且必須存在於 `PATH`
- 無效、私有或空的播放清單會以非零狀態碼結束
- 正常 CLI 使用時不會顯示 GUI 視窗

## 貢獻

歡迎貢獻。請先閱讀 [CONTRIBUTING.md](./CONTRIBUTING.md) 了解工作流程與驗證方式。

## 授權

本專案採用 [MIT License](./LICENSE)。
