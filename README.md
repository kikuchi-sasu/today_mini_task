<p align="center">
  <h1 align="center">⚡ QuickTask</h1>
  <p align="center">
    超軽量タスク管理アプリ — 脳の短期メモリを解放する<br>
    <em>Ultra-lightweight task manager — Free your brain's short-term memory</em>
  </p>
</p>

---

## 🎯 コンセプト / Concept

**1秒で起動、3秒で入力、時間になったら通知。**

カテゴリー、優先度、同期、プロジェクト管理といった余計な機能は一切排除。  
「思いついたことをすぐメモして、時間になったら教えてくれる」——それだけに特化したツールです。

**Launch in 1 second, input in 3 seconds, get notified when it's time.**

No categories, no priorities, no sync, no project management.  
Just a tool that lets you quickly jot things down and reminds you when the time comes.

---

## ✨ 特徴 / Features

| 機能 | 説明 |
|---|---|
| ⚡ **即時起動** | Tauri + Rust ベースで超高速起動 |
| 🧠 **インライン時間解析** | `13:00 会議` と入力するだけで、時間を自動抽出し通知をセット |
| 🔔 **OS ネイティブ通知** | 指定時刻にWindowsの通知でお知らせ |
| 🖥️ **システムトレイ常駐** | ✕ボタンで閉じてもバックグラウンドで待機 |
| 🌙 **モノトーン UI** | ノイズを排除した、ボーダーレスなダークモードデザイン |
| 💾 **ローカル保存** | データはすべてローカルJSON。外部送信なし |

---

## 📸 スクリーンショット / Screenshot

```
┌─────────────────────────────────┐
│  QUICKTASK              — ✕    │
│                                 │
│  + [ Add a quick task...      ] │
│  ───────────────────────────── │
│  • 13:00 資料の印刷     🕐13:00 │
│  • 15:30 チームMTG      🕐15:30 │
│  • 帰りに牛乳を買う             │
│                                 │
│  Enter: 追加  Ctrl+Enter: 全消去 │
└─────────────────────────────────┘
```

---

## ⌨️ ショートカットキー / Keyboard Shortcuts

| キー | 動作 |
|---|---|
| `Enter` | タスクを追加 |
| `Ctrl + Enter` | 全タスクを消去 |
| `Esc` | ウィンドウを最小化 |

---

## 🔧 環境構築 / Prerequisites

以下のツールが必要です。

| ツール | インストール方法 |
|---|---|
| **Rust** | `winget install --id Rustlang.Rustup` |
| **Node.js (LTS)** | `winget install --id OpenJS.NodeJS.LTS` |
| **MSVC Build Tools** | [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) の「Desktop development with C++」 |

> ⚠️ インストール後、ターミナル（場合によってはPC）の再起動が必要です。

---

## 🚀 ビルド・実行方法 / Getting Started

### 開発モード / Development

```bash
# 依存関係のインストール
npm install

# 開発サーバー起動（ホットリロード対応）
npm run tauri dev
```

### プロダクションビルド / Production Build

```bash
npm run tauri build
```

ビルド成果物は以下に出力されます:

```
src-tauri/target/release/bundle/
├── nsis/QuickTask_x.x.x_x64-setup.exe   # インストーラー
└── msi/QuickTask_x.x.x_x64_en-US.msi    # MSI パッケージ
```

---

## 📝 入力パターン / Input Patterns

時間付きタスクは以下のパターンで自動認識されます:

```
13:00 会議            →  テキスト: "会議"     通知: 13:00
資料作成 15:30        →  テキスト: "資料作成"  通知: 15:30
帰りに牛乳を買う      →  テキスト: そのまま    通知: なし
```

---

## 🏗️ 技術スタック / Tech Stack

| レイヤー | 技術 |
|---|---|
| Framework | **Tauri v2** (Rust) |
| Frontend | **SvelteKit 5** + TypeScript |
| Backend | **Rust** (tokio 非同期ランタイム) |
| Storage | Local JSON (`AppData` 配下) |
| Notification | OS Native Notification |

---

## 📁 プロジェクト構造 / Project Structure

```
ap_task管理/
├── src/                      # フロントエンド (SvelteKit)
│   ├── app.html              # HTML テンプレート
│   └── routes/
│       ├── +layout.ts        # SPA モード設定
│       └── +page.svelte      # メイン UI
├── src-tauri/                # バックエンド (Rust)
│   ├── src/
│   │   ├── lib.rs            # Tauri コマンド & アプリ設定
│   │   ├── main.rs           # エントリーポイント
│   │   ├── task.rs           # タスクモデル & 時間パーサー
│   │   └── scheduler.rs     # 通知スケジューラ
│   ├── Cargo.toml            # Rust 依存関係
│   ├── tauri.conf.json       # Tauri 設定
│   └── capabilities/
│       └── default.json      # 権限設定
├── package.json
└── vite.config.js
```

---

## 📄 ライセンス / License

MIT License
