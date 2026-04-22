# Tauri FCM Tool

*Lire dans d'autres langues : [English](../README.md) | [한국어](README.ko.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Español](README.es.md)*

Un établi extensible pour les outils de développement, alimenté par le développement assisté par LLM.

## Aperçu

Tauri FCM Tool est une application de bureau qui vous aide à créer et gérer votre propre collection d'utilitaires de développement avec une interface de barre latérale intuitive. Ajoutez de nouveaux outils à la volée en utilisant l'assistance LLM, faisant de celle-ci votre établi de productivité personnalisé.

## Fonctionnalités principales

- Navigation par barre latérale pour un accès organisé aux outils
- Création et personnalisation d'outils assistées par LLM
- Performance native avec backend Tauri + Rust
- Organisation des outils par catégories

## Outils intégrés

### Encodage
- **Base64** - Encoder/décoder du texte en Base64

### Outils LLM
- **Motion Descriptor** - Convertir les gestes de la souris en descriptions en langage naturel

### Développement de jeux
- **Sprite Sheet Describer** - Annoter les grilles de feuilles de sprites avec des descriptions, exporter en JSON
- **Tween Visualizer** - Visualiser et comparer plus de 30 fonctions d'interpolation

## Démarrage

### Prérequis
- [Bun](https://bun.sh)
- [Rust](https://rustup.rs)

### Développement

```bash
bun install
bun run tauri dev
```

### Build de production

```bash
bun run tauri build
```

## Stack technique

- Frontend : SvelteKit 2.x + Svelte 5, Skeleton UI
- Backend : Rust + Tauri 2.0
- Sécurité des types : tauri-specta
- Styles : Tailwind CSS 4

## Philosophie du projet

Tauri FCM Tool n'est pas seulement un ensemble d'outils fixe, mais un framework pour construire vos propres outils. Tirez parti des LLM pour créer rapidement des utilitaires et créer un établi qui correspond à votre flux de travail.

## Ajouter de nouveaux outils

1. Définir l'outil dans `src/lib/config/tools.ts`
2. Créer la route : `src/routes/tools/{category}/{tool-id}/+page.svelte`
3. (Optionnel) Ajouter des commandes backend dans `src-tauri/src/tools/`

Consultez [CLAUDE.md](../Claude.md) pour des directives de développement détaillées.

## Structure du projet

```
tauri-fcm-tool/
├── src/                    # Frontend (SvelteKit)
│   ├── routes/tools/      # Implémentations d'outils
│   ├── lib/components/    # Composants UI
│   └── lib/config/        # Métadonnées d'outils
├── src-tauri/             # Backend (Rust)
│   ├── src/tools/         # Commandes backend
│   └── src/modules/       # Utilitaires
└── .github/workflows/     # CI/CD
```

## Publication

Poussez un tag de version pour déclencher des builds automatiques via GitHub Actions :

```bash
git tag v1.0.0
git push origin v1.0.0
```

Artefacts de build :
- Windows : installateur `.exe`
- macOS : `.dmg` + `.app` Universal (Intel + Apple Silicon)
- Linux : `.AppImage` + `.deb`

## Contribuer

Les contributions sont les bienvenues ! Nous apprécions les nouveaux outils et les améliorations.

1. Forkez le dépôt
2. Créez votre branche de fonctionnalité
3. Ajoutez votre outil en suivant la structure du projet
4. Soumettez une pull request

## Licence

MIT

## Remerciements

- [Tauri](https://tauri.app)
- [Skeleton UI](https://skeleton.dev)
