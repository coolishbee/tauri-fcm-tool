# Tauri FCM Tool

*他の言語で読む：[English](../README.md) | [한국어](README.ko.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md)*

LLM 支援開発により駆動される拡張可能な開発者ツールワークベンチ。

## 概要

Tauri FCM Tool は、直感的なサイドバーインターフェースで独自の開発ユーティリティコレクションを構築・管理できるデスクトップアプリケーションです。LLM 支援を使用してその場で新しいツールを追加し、パーソナライズされた生産性ワークベンチを作成できます。

## 主な機能

- 整理されたツールアクセスのためのサイドバーナビゲーション
- LLM 支援によるツールの作成とカスタマイズ
- Tauri + Rust バックエンドによるネイティブパフォーマンス
- カテゴリベースのツール整理

## 組み込みツール

### エンコーディング
- **Base64** - テキストを Base64 にエンコード/デコード

### LLM ツール
- **Motion Descriptor** - マウスジェスチャーを自然言語の説明に変換

### ゲーム開発
- **Sprite Sheet Describer** - スプライトシートグリッドに注釈を追加、JSON としてエクスポート
- **Tween Visualizer** - 30 種類以上のイージング関数を視覚化・比較

## はじめに

### 前提条件
- [Bun](https://bun.sh)
- [Rust](https://rustup.rs)

### 開発

```bash
bun install
bun run tauri dev
```

### プロダクションビルド

```bash
bun run tauri build
```

## 技術スタック

- フロントエンド：SvelteKit 2.x + Svelte 5, Skeleton UI
- バックエンド：Rust + Tauri 2.0
- 型安全性：tauri-specta
- スタイリング：Tailwind CSS 4

## プロジェクトの理念

Tauri FCM Tool は単なる固定されたツールセットではなく、独自のツールを構築するためのフレームワークです。LLM を活用してユーティリティを素早くスキャフォールディングし、ワークフローに合ったワークベンチを作成しましょう。

## 新しいツールの追加

1. `src/lib/config/tools.ts` でツールを定義
2. ルートを作成：`src/routes/tools/{category}/{tool-id}/+page.svelte`
3. （オプション）`src-tauri/src/tools/` にバックエンドコマンドを追加

詳細な開発ガイドラインは [CLAUDE.md](../Claude.md) をご覧ください。

## プロジェクト構造

```
tauri-fcm-tool/
├── src/                    # フロントエンド (SvelteKit)
│   ├── routes/tools/      # ツール実装
│   ├── lib/components/    # UI コンポーネント
│   └── lib/config/        # ツールメタデータ
├── src-tauri/             # バックエンド (Rust)
│   ├── src/tools/         # バックエンドコマンド
│   └── src/modules/       # ユーティリティ
└── .github/workflows/     # CI/CD
```

## リリース

バージョンタグをプッシュして GitHub Actions による自動ビルドをトリガーします：

```bash
git tag v1.0.0
git push origin v1.0.0
```

ビルド成果物：
- Windows：`.exe` インストーラー
- macOS：Universal `.dmg` + `.app`（Intel + Apple Silicon）
- Linux：`.AppImage` + `.deb`

## コントリビューション

コントリビューションを歓迎します！新しいツールや改善をお待ちしています。

1. リポジトリをフォーク
2. フィーチャーブランチを作成
3. プロジェクト構造に従ってツールを追加
4. プルリクエストを提出

## ライセンス

MIT

## 謝辞

- [Tauri](https://tauri.app)
- [Skeleton UI](https://skeleton.dev)
