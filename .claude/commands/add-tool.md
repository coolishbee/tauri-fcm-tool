# Add New Tool to Sidebar

새로운 도구를 사이드바 메뉴에 추가합니다.

## Arguments

- $ARGUMENTS: 도구 설명 (예: "encoding 카테고리에 URL Encoder 도구 추가")

## Steps

1. **사용자 입력 파싱**: $ARGUMENTS에서 카테고리와 도구 이름을 파악합니다. 명확하지 않으면 사용자에게 질문합니다.

2. **기존 카테고리 확인**: `src/lib/config/tools.ts`의 `toolCategories` 배열에서 해당 카테고리가 있는지 확인합니다.
   - 기존 카테고리에 추가할 경우: 해당 카테고리의 `tools` 배열에 항목 추가
   - 새 카테고리가 필요한 경우: `toolCategories` 배열에 새 카테고리 객체 추가

3. **tools.ts 수정**: `src/lib/config/tools.ts`에 새 도구 항목을 추가합니다.
   - `id`: kebab-case (예: `url-encoder`)
   - `name`: 표시용 이름 (예: `URL Encoder`)
   - `description`: 도구에 대한 한줄 설명 (예: `URL 인코딩/디코딩 도구`)
   - `path`: `/tools/{category-id}/{tool-id}`

4. **페이지 파일 생성**: `src/routes/tools/{category-id}/{tool-id}/+page.svelte` 파일을 생성합니다.
   - Svelte 5 runes 사용 (`$state`, `$derived`)
   - 세미콜론 없음, 더블 쿼트, 2스페이스 인덴트
   - 기본 레이아웃 구조:
     ```svelte
     <script lang="ts">
       // state and logic
     </script>

     <div class="p-6 max-w-4xl mx-auto">
       <h1 class="text-2xl font-bold mb-4">도구 이름</h1>
       <!-- tool UI -->
     </div>
     ```

5. **CLAUDE.md 업데이트**: 도구가 비자명한 패턴을 사용하거나 설명이 필요한 경우 CLAUDE.md의 Tools 섹션에 추가합니다.

6. **확인**: `bun run check`로 타입 체크를 수행합니다.
