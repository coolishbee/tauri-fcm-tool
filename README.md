# Tauri FCM Tool

*Read this in other languages: [한국어](docs/README.ko.md) | [简体中文](docs/README.zh-CN.md) | [日本語](docs/README.ja.md) | [Español](docs/README.es.md) | [Français](docs/README.fr.md)*

An extensible workbench for developer tools, powered by LLM-assisted development.

## Overview

Tauri FCM Tool is a desktop application that helps you build and manage your own collection of developer utilities with an intuitive sidebar interface. Add new tools on the fly using LLM assistance, making it your personalized productivity workbench.

## Key Features

- Sidebar navigation for organized tool access
- LLM-assisted tool creation and customization
- Native performance with Tauri + Rust backend
- Category-based tool organization

## Built-in Tools

### Encoding
- **Base64** - Encode/decode text to Base64

### LLM Tools
- **Motion Descriptor** - Convert mouse gestures to natural language descriptions

### Game Development
- **Sprite Sheet Describer** - Annotate sprite sheet grids with descriptions, export as JSON
- **Tween Visualizer** - Visualize and compare 30+ easing functions

## Getting Started

### Prerequisites
- [Bun](https://bun.sh)
- [Rust](https://rustup.rs)

### Development

```bash
bun install
bun run tauri dev
```

### Production Build

```bash
bun run tauri build
```

## Tech Stack

- Frontend: SvelteKit 2.x + Svelte 5, Skeleton UI
- Backend: Rust + Tauri 2.0
- Type Safety: tauri-specta
- Styling: Tailwind CSS 4

## Project Philosophy

Tauri FCM Tool is not just a fixed toolset, but a framework for building your own tools. Leverage LLMs to quickly scaffold utilities and create a workbench that fits your workflow.

## Adding New Tools

1. Define tool in `src/lib/config/tools.ts`
2. Create route: `src/routes/tools/{category}/{tool-id}/+page.svelte`
3. (Optional) Add backend commands in `src-tauri/src/tools/`

See [CLAUDE.md](./Claude.md) for detailed development guidelines.

## Project Structure

```
tauri-fcm-tool/
├── src/                    # Frontend (SvelteKit)
│   ├── routes/tools/      # Tool implementations
│   ├── lib/components/    # UI components
│   └── lib/config/        # Tool metadata
├── src-tauri/             # Backend (Rust)
│   ├── src/tools/         # Backend commands
│   └── src/modules/       # Utilities
└── .github/workflows/     # CI/CD
```

## Release

Push a version tag to trigger automated builds via GitHub Actions:

```bash
git tag v0.0.1
git push origin v0.0.1
```

Build artifacts:
- Windows: `.exe` installer
- macOS: Universal `.dmg` + `.app` (Intel + Apple Silicon)
- Linux: `.AppImage` + `.deb`

## Contributing

Contributions are welcome! We appreciate new tools and improvements.

1. Fork the repository
2. Create your feature branch
3. Add your tool following the project structure
4. Submit a pull request

## License

MIT

## Acknowledgments

- [Tauri](https://tauri.app)
- [Skeleton UI](https://skeleton.dev)
