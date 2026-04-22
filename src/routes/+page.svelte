<script lang="ts">
  import { onMount } from "svelte"
  import SendView from "$lib/components/SendView.svelte"
  import TemplateView from "$lib/components/TemplateView.svelte"
  import HistoryView from "$lib/components/HistoryView.svelte"
  import { commands } from "$lib/bindings"

  let activeTab = $state<"send" | "template" | "history">("send")
  let isAuthenticated = $state(false)

  onMount(async () => {
    await checkAuth()
  })

  async function checkAuth() {
    try {
      const result = await commands.isAuthenticated()
      if (result.status === "ok") {
        isAuthenticated = result.data
      }
    } catch (e) {
      console.error("인증 상태 확인 실패:", e)
    }
  }
</script>

<div class="h-full flex flex-col">
  <!-- 탭 헤더 -->
  <div class="border-b border-surface-200-800 px-4">
    <div class="flex gap-1">
      <button
        class="px-4 py-2 text-sm font-medium border-b-2 transition-colors {activeTab === 'send' ? 'border-primary-500 text-primary-500' : 'border-transparent hover:border-surface-400'}"
        onclick={() => activeTab = "send"}
      >
        발송
      </button>
      <button
        class="px-4 py-2 text-sm font-medium border-b-2 transition-colors {activeTab === 'template' ? 'border-primary-500 text-primary-500' : 'border-transparent hover:border-surface-400'}"
        onclick={() => activeTab = "template"}
      >
        템플릿
      </button>
      <button
        class="px-4 py-2 text-sm font-medium border-b-2 transition-colors {activeTab === 'history' ? 'border-primary-500 text-primary-500' : 'border-transparent hover:border-surface-400'}"
        onclick={() => activeTab = "history"}
      >
        히스토리
      </button>
    </div>
  </div>

  <!-- 인증 필요 경고 -->
  {#if !isAuthenticated}
    <div class="bg-warning-500/20 text-warning-500 px-4 py-3 text-sm">
      FCM 발송을 위해 먼저 <strong>설정</strong> 탭에서 로그인해주세요.
    </div>
  {/if}

  <!-- 탭 컨텐츠 -->
  <div class="flex-1 overflow-y-auto">
    {#if activeTab === "send"}
      <SendView {isAuthenticated} />
    {:else if activeTab === "template"}
      <TemplateView />
    {:else}
      <HistoryView />
    {/if}
  </div>
</div>
