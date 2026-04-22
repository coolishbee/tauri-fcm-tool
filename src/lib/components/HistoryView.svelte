<script lang="ts">
  import { onMount } from "svelte"
  import { commands, type HistoryEntry } from "$lib/bindings"

  let entries = $state<HistoryEntry[]>([])
  let selectedEntry = $state<HistoryEntry | null>(null)
  let isLoading = $state(false)
  let message = $state("")
  let messageType = $state<"success" | "error" | "">("")

  onMount(async () => {
    await loadHistory()
  })

  async function loadHistory() {
    try {
      const result = await commands.getHistory()
      if (result.status === "ok") {
        entries = result.data.entries || []
      } else {
        showMessage(`히스토리 로드 실패: ${result.error}`, "error")
      }
    } catch (e) {
      showMessage(`히스토리 로드 실패: ${e}`, "error")
    }
  }

  async function handleClear() {
    if (!confirm("모든 히스토리를 삭제하시겠습니까?")) return

    isLoading = true
    try {
      const result = await commands.clearHistory()
      if (result.status === "ok") {
        entries = []
        selectedEntry = null
        showMessage("히스토리가 삭제되었습니다", "success")
      } else {
        showMessage(`삭제 실패: ${result.error}`, "error")
      }
    } catch (e) {
      showMessage(`삭제 실패: ${e}`, "error")
    } finally {
      isLoading = false
    }
  }

  function formatTime(timestamp: string): string {
    const date = new Date(timestamp)
    return date.toLocaleTimeString("ko-KR", { hour: "2-digit", minute: "2-digit", second: "2-digit" })
  }

  function formatDate(timestamp: string): string {
    const date = new Date(timestamp)
    return date.toLocaleDateString("ko-KR")
  }

  function getTypeLabel(type: string): string {
    switch (type) {
      case "single": return "단일"
      case "topic": return "토픽"
      default: return type
    }
  }

  function showMessage(msg: string, type: "success" | "error") {
    message = msg
    messageType = type
    setTimeout(() => {
      message = ""
      messageType = ""
    }, 3000)
  }
</script>

<div class="p-6">
  {#if message}
    <div class="mb-4 p-3 rounded-lg {messageType === 'success' ? 'bg-success-500/20 text-success-500' : 'bg-error-500/20 text-error-500'}">
      {message}
    </div>
  {/if}

  <div class="flex items-center justify-between mb-4">
    <h2 class="text-lg font-semibold">발송 기록 (최근 100개)</h2>
    <div class="flex gap-2">
      <button class="btn btn-sm preset-tonal" onclick={loadHistory} disabled={isLoading}>
        새로고침
      </button>
      <button
        class="btn btn-sm preset-filled-error-500"
        onclick={handleClear}
        disabled={isLoading || entries.length === 0}
      >
        히스토리 지우기
      </button>
    </div>
  </div>

  {#if entries.length === 0}
    <div class="text-center py-12 opacity-50">
      <p>발송 기록이 없습니다</p>
    </div>
  {:else}
    <!-- 히스토리 테이블 -->
    <div class="overflow-x-auto">
      <table class="table w-full">
        <thead>
          <tr class="border-b border-surface-200-800">
            <th class="text-left py-2 px-3">시간</th>
            <th class="text-left py-2 px-3">방식</th>
            <th class="text-left py-2 px-3">결과</th>
            <th class="text-left py-2 px-3">제목</th>
          </tr>
        </thead>
        <tbody>
          {#each entries as entry}
            <tr
              class="border-b border-surface-200-800 hover:bg-surface-200-800/50 cursor-pointer transition-colors"
              onclick={() => selectedEntry = entry}
              onkeydown={(e) => e.key === 'Enter' && (selectedEntry = entry)}
              tabindex="0"
              role="button"
            >
              <td class="py-2 px-3 text-sm">{formatTime(entry.timestamp)}</td>
              <td class="py-2 px-3 text-sm">{getTypeLabel(entry.messageType)}</td>
              <td class="py-2 px-3">
                <span class="text-sm {entry.success ? 'text-success-500' : 'text-error-500'}">
                  {entry.success ? "성공" : "실패"}
                </span>
              </td>
              <td class="py-2 px-3 text-sm truncate max-w-[200px]">{entry.title}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>

<!-- 상세 모달 -->
{#if selectedEntry}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onclick={() => selectedEntry = null}>
    <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
    <div class="card preset-filled-surface-100-900 p-6 w-[500px] max-h-[80vh] overflow-y-auto" onclick={(e) => e.stopPropagation()}>
      <h2 class="text-lg font-semibold mb-4">발송 기록 상세</h2>
      
      <div class="space-y-3 text-sm">
        <div class="flex justify-between">
          <span class="opacity-50">발송 시간</span>
          <span>{formatDate(selectedEntry.timestamp)} {formatTime(selectedEntry.timestamp)}</span>
        </div>
        <div class="flex justify-between">
          <span class="opacity-50">발송 방식</span>
          <span>{getTypeLabel(selectedEntry.messageType)}</span>
        </div>
        <div class="flex justify-between">
          <span class="opacity-50">결과</span>
          <span class="{selectedEntry.success ? 'text-success-500' : 'text-error-500'}">
            {selectedEntry.success ? "성공" : "실패"}
          </span>
        </div>
        
        <hr class="border-surface-200-800" />
        
        <div>
          <span class="opacity-50 block mb-1">제목</span>
          <span>{selectedEntry.title}</span>
        </div>
        <div>
          <span class="opacity-50 block mb-1">내용</span>
          <span class="whitespace-pre-wrap">{selectedEntry.body}</span>
        </div>
        <div>
          <span class="opacity-50 block mb-1">상세</span>
          <span class="whitespace-pre-wrap">{selectedEntry.details}</span>
        </div>
      </div>
      
      <div class="mt-6 flex justify-end">
        <button class="btn preset-tonal" onclick={() => selectedEntry = null}>
          닫기
        </button>
      </div>
    </div>
  </div>
{/if}
