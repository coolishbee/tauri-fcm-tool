# Tauri FCM Tool

*다른 언어로 읽기: [English](../README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Español](README.es.md) | [Français](README.fr.md)*

LLM 기반 개발로 구동되는 확장 가능한 개발자 도구 워크벤치입니다.

## 개요

Tauri FCM Tool는 직관적인 사이드바 인터페이스로 자신만의 개발 유틸리티 컬렉션을 구축하고 관리하는 데 도움을 주는 데스크톱 애플리케이션입니다. LLM 지원을 사용하여 즉시 새로운 도구를 추가하여 개인화된 생산성 워크벤치를 만들 수 있습니다.

## 주요 기능

- 체계적인 도구 접근을 위한 사이드바 내비게이션
- LLM 지원 도구 생성 및 커스터마이징
- Tauri + Rust 백엔드로 네이티브 성능 제공
- 카테고리 기반 도구 구성

## 내장 도구

### 인코딩
- **Base64** - 텍스트를 Base64로 인코딩/디코딩

### LLM 도구
- **Motion Descriptor** - 마우스 제스처를 자연어 설명으로 변환

### 게임 개발
- **Sprite Sheet Describer** - 스프라이트 시트 그리드에 주석 추가, JSON으로 내보내기
- **Tween Visualizer** - 30개 이상의 이징 함수 시각화 및 비교

## 시작하기

### 필수 요구사항
- [Bun](https://bun.sh)
- [Rust](https://rustup.rs)

### 개발

```bash
bun install
bun run tauri dev
```

### 프로덕션 빌드

```bash
bun run tauri build
```

## 기술 스택

- 프론트엔드: SvelteKit 2.x + Svelte 5, Skeleton UI
- 백엔드: Rust + Tauri 2.0
- 타입 안전성: tauri-specta
- 스타일링: Tailwind CSS 4

## 프로젝트 철학

Tauri FCM Tool는 고정된 도구 세트가 아니라 자신만의 도구를 구축하기 위한 프레임워크입니다. LLM을 활용하여 유틸리티를 빠르게 스캐폴딩하고 워크플로우에 맞는 워크벤치를 만드세요.

## 새 도구 추가하기

1. `src/lib/config/tools.ts`에 도구 정의
2. 라우트 생성: `src/routes/tools/{category}/{tool-id}/+page.svelte`
3. (선택 사항) `src-tauri/src/tools/`에 백엔드 명령 추가

자세한 개발 가이드라인은 [CLAUDE.md](../Claude.md)를 참조하세요.

## 프로젝트 구조

```
tauri-fcm-tool/
├── src/                    # 프론트엔드 (SvelteKit)
│   ├── routes/tools/      # 도구 구현
│   ├── lib/components/    # UI 컴포넌트
│   └── lib/config/        # 도구 메타데이터
├── src-tauri/             # 백엔드 (Rust)
│   ├── src/tools/         # 백엔드 명령
│   └── src/modules/       # 유틸리티
└── .github/workflows/     # CI/CD
```

## 릴리스

버전 태그를 푸시하여 GitHub Actions를 통한 자동 빌드를 트리거합니다:

```bash
git tag v1.0.0
git push origin v1.0.0
```

빌드 결과물:
- Windows: `.exe` 설치 파일
- macOS: Universal `.dmg` + `.app` (Intel + Apple Silicon)
- Linux: `.AppImage` + `.deb`

## 기여하기

기여를 환영합니다! 새로운 도구와 개선 사항을 기대합니다.

1. 저장소 포크
2. 기능 브랜치 생성
3. 프로젝트 구조에 따라 도구 추가
4. 풀 리퀘스트 제출

## 라이선스

MIT

## 감사의 말

- [Tauri](https://tauri.app)
- [Skeleton UI](https://skeleton.dev)
