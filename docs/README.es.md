# Tauri FCM Tool

*Leer en otros idiomas: [English](../README.md) | [한국어](README.ko.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Français](README.fr.md)*

Un banco de trabajo extensible para herramientas de desarrollo, impulsado por desarrollo asistido por LLM.

## Descripción general

Tauri FCM Tool es una aplicación de escritorio que te ayuda a construir y gestionar tu propia colección de utilidades de desarrollo con una interfaz de barra lateral intuitiva. Añade nuevas herramientas sobre la marcha usando asistencia LLM, haciendo de ella tu banco de trabajo de productividad personalizado.

## Características principales

- Navegación por barra lateral para acceso organizado a las herramientas
- Creación y personalización de herramientas asistidas por LLM
- Rendimiento nativo con backend Tauri + Rust
- Organización de herramientas basada en categorías

## Herramientas integradas

### Codificación
- **Base64** - Codificar/decodificar texto a Base64

### Herramientas LLM
- **Motion Descriptor** - Convertir gestos del ratón a descripciones en lenguaje natural

### Desarrollo de juegos
- **Sprite Sheet Describer** - Anotar cuadrículas de hojas de sprites con descripciones, exportar como JSON
- **Tween Visualizer** - Visualizar y comparar más de 30 funciones de suavizado

## Primeros pasos

### Requisitos previos
- [Bun](https://bun.sh)
- [Rust](https://rustup.rs)

### Desarrollo

```bash
bun install
bun run tauri dev
```

### Compilación de producción

```bash
bun run tauri build
```

## Stack tecnológico

- Frontend: SvelteKit 2.x + Svelte 5, Skeleton UI
- Backend: Rust + Tauri 2.0
- Seguridad de tipos: tauri-specta
- Estilos: Tailwind CSS 4

## Filosofía del proyecto

Tauri FCM Tool no es solo un conjunto de herramientas fijo, sino un marco para construir tus propias herramientas. Aprovecha los LLM para crear rápidamente utilidades y crear un banco de trabajo que se adapte a tu flujo de trabajo.

## Agregar nuevas herramientas

1. Define la herramienta en `src/lib/config/tools.ts`
2. Crea la ruta: `src/routes/tools/{category}/{tool-id}/+page.svelte`
3. (Opcional) Añade comandos backend en `src-tauri/src/tools/`

Consulta [CLAUDE.md](../Claude.md) para pautas de desarrollo detalladas.

## Estructura del proyecto

```
tauri-fcm-tool/
├── src/                    # Frontend (SvelteKit)
│   ├── routes/tools/      # Implementaciones de herramientas
│   ├── lib/components/    # Componentes UI
│   └── lib/config/        # Metadatos de herramientas
├── src-tauri/             # Backend (Rust)
│   ├── src/tools/         # Comandos backend
│   └── src/modules/       # Utilidades
└── .github/workflows/     # CI/CD
```

## Lanzamiento

Empuja una etiqueta de versión para activar compilaciones automáticas mediante GitHub Actions:

```bash
git tag v1.0.0
git push origin v1.0.0
```

Artefactos de compilación:
- Windows: instalador `.exe`
- macOS: `.dmg` + `.app` Universal (Intel + Apple Silicon)
- Linux: `.AppImage` + `.deb`

## Contribuir

¡Las contribuciones son bienvenidas! Apreciamos nuevas herramientas y mejoras.

1. Haz fork del repositorio
2. Crea tu rama de características
3. Añade tu herramienta siguiendo la estructura del proyecto
4. Envía una pull request

## Licencia

MIT

## Reconocimientos

- [Tauri](https://tauri.app)
- [Skeleton UI](https://skeleton.dev)
