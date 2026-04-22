# AGENTS.md

## 현재 기준 소스 오브 트루스
- 루트 `README.md`는 현재 코드와 일부 불일치한다. 앱 구조와 작업 방식은 `src-tauri/src/lib.rs`, `src-tauri/src/command.rs`, `src/routes/+layout.svelte`, `src/routes/+page.svelte`를 기준으로 판단한다.
- 이 저장소는 다중 툴 워크벤치가 아니라 Tauri + SvelteKit 기반 FCM 발송 데스크톱 앱이다.

## 빠른 실행 / 검증
- 의존성 설치: `bun install`
- 전체 앱 개발 실행: `bun run tauri dev`
- 프런트엔드 타입체크: `bun run check`
- Rust 확인: `cargo check` (`/src-tauri`에서 실행)
- Rust 포맷/린트: `cargo fmt && cargo clippy` (`/src-tauri`에서 실행)
- 현재 저장소에는 별도 테스트 스위트가 없다. 보통 검증은 `bun run check` + `cargo check` 또는 `cargo clippy` 조합이다.

## 구조
- 프런트엔드는 라우트 기반 앱이 거의 아니다. 실제 UI 전환은 `src/routes/+layout.svelte`의 사이드바(`general`/`settings`)와 `src/routes/+page.svelte`의 탭(`send`/`template`/`history`) 상태로 처리한다.
- Tauri 명령 진입점은 `src-tauri/src/command.rs` 하나에 모여 있고, 등록은 `src-tauri/src/lib.rs`의 `collect_commands![]`에서 한다.
- 실제 도메인 로직은 `src-tauri/src/fcm/` 아래에 있다: `auth`, `client`, `config`, `message`, `template`, `history`.

## 생성 파일 / 코드젠
- `src/lib/bindings.ts`는 `tauri-specta` 생성 파일이다. 수동 편집하지 않는다.
- 새 Tauri command를 추가하거나 시그니처를 바꾸면:
  1. `src-tauri/src/command.rs`에 추가
  2. `src-tauri/src/lib.rs`의 `collect_commands![]`에 등록
  3. `bun run tauri dev`로 `src/lib/bindings.ts` 재생성
- 타입을 프런트엔드로 노출할 Rust struct는 `specta::Type`와 `#[serde(rename_all = "camelCase")]`를 맞춘다.
- `specta` 호환성 때문에 `usize` 대신 `u32`를 쓴다.

## 구현 시 주의점
- 설정/토큰/템플릿/히스토리는 Tauri Store에 저장하며 파일명은 각각 `config.json`, `token.json`, `templates.json`, `history.json`이다. 이 이름들은 `src-tauri/src/command.rs` 상수와 맞춰야 한다.
- OAuth 기본값은 Rust 쪽 `src-tauri/src/fcm/config.rs`에 있다. 설정 UI 기본값과 따로 놀지 않게 함께 확인한다.
- 히스토리는 `HistoryList`에서 최대 100개로 자른다. 히스토리 동작을 바꿀 때 `src-tauri/src/fcm/history.rs`를 본다.
- `firebaseProjectId`는 설정 화면에서 `dev-`/`qa-` 접두사를 붙여 저장한다. 단순 프로젝트명만 저장한다고 가정하면 안 된다.

## Tauri / SvelteKit 특이사항
- SvelteKit은 SSR 비활성화 SPA 모드다: `src/routes/+layout.ts`에서 `ssr = false`, `svelte.config.js`는 `adapter-static` + `fallback: "index.html"`.
- Tauri 개발 서버 포트는 고정이다: Vite `1420`, HMR `1421` (`vite.config.js`). 이미 사용 중이면 `tauri dev`가 실패한다.
- `src-tauri/src/lib.rs`의 타입스크립트 바인딩 export는 `#[cfg(debug_assertions)]` 안에 있으므로, 보통 개발 실행에서만 재생성된다.

## 커밋 전 기준
- 저장소 로컬 훅은 체크인되어 있지 않지만, `CLAUDE.md`가 커밋 전 `bun run check`와 `/src-tauri`의 `cargo fmt && cargo clippy` 실행을 요구한다.

## 릴리스
- 릴리스는 태그 푸시(`v*.*.*`)로만 시작된다. `.github/workflows/release.yml`이 macOS/Ubuntu/Windows 빌드를 수행한다.
