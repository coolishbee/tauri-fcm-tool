<script lang="ts">
  import { onMount } from "svelte"
  import { commands, type MessageType } from "$lib/bindings"

  interface Props {
    isAuthenticated: boolean
  }

  let { isAuthenticated }: Props = $props()

  let messageType = $state<MessageType>("single")
  let token = $state("")
  let topic = $state("")
  let title = $state("")
  let body = $state("")
  let isLoading = $state(false)
  let result = $state("")
  let resultType = $state<"success" | "error" | "">("")
  let requestJson = $state("")

  let androidChannelId = $state("")

  // 템플릿 목록 (템플릿에서 불러오기용)
  let templates = $state<Array<{id: string, name: string, title: string, body: string}>>([])
  let showTemplateModal = $state(false)

  onMount(async () => {
    await loadTemplates()
  })

  async function loadTemplates() {
    try {
      const res = await commands.getTemplates()
      if (res.status === "ok") {
        templates = res.data.templates || []
      }
    } catch (e) {
      console.error("템플릿 로드 실패:", e)
    }
  }

  async function handleSend() {
    if (!isAuthenticated) {
      showResult("먼저 로그인해주세요", "error")
      return
    }

    if (!title.trim() || !body.trim()) {
      showResult("제목과 내용을 입력해주세요", "error")
      return
    }

    // 타입별 검증
    if (messageType === "single" && !token.trim()) {
      showResult("FCM 토큰을 입력해주세요", "error")
      return
    }
    if (messageType === "topic" && !topic.trim()) {
      showResult("토픽 이름을 입력해주세요", "error")
      return
    }

    isLoading = true
    result = ""
    requestJson = ""

    try {
      const request = {
        messageType: messageType,
        message: { title: title.trim(), body: body.trim() },
        token: messageType === "single" ? token.trim() : null,
        topic: messageType === "topic" ? topic.trim() : null,
        android: {
          priority: "high",
          channelId: androidChannelId.trim()
        },
        apns: {
          priority: "10",
          sound: "default"
        }
      }

      // FCM Request JSON 저장
      requestJson = JSON.stringify(request, null, 2)

      const res = await commands.sendFcmMessage(request)

      if (res.status === "ok") {
        if (res.data.success) {
          showResult(res.data.details, "success")
        } else {
          showResult(res.data.details, "error")
        }
      } else {
        showResult(`발송 실패: ${res.error}`, "error")
      }
    } catch (e) {
      showResult(`발송 실패: ${e}`, "error")
    } finally {
      isLoading = false
    }
  }

  function showResult(msg: string, type: "success" | "error") {
    result = msg
    resultType = type
  }

  function loadFromTemplate(template: {title: string, body: string}) {
    title = template.title
    body = template.body
    showTemplateModal = false
  }

  function reset() {
    token = ""
    topic = ""
    title = ""
    body = ""
    result = ""
    resultType = ""
    requestJson = ""
    androidChannelId = ""
  }

  function highlightJson(json: string): string {
    return json
      .replace(/"([^"]+)":/g, '<span class="text-green-400">"$1"</span>:')
      .replace(/: "([^"]*)"/g, ': <span class="text-yellow-400">"$1"</span>')
      .replace(/: (\d+)/g, ': <span class="text-purple-400">$1</span>')
      .replace(/: null/g, ': <span class="text-gray-500">null</span>')
      .replace(/: (true|false)/g, ': <span class="text-purple-400">$1</span>')
  }
</script>

<div class="p-6 max-w-2xl mx-auto">
  <!-- 발송 방식 선택 -->
  <div class="mb-6">
    <label class="block text-sm font-medium mb-2" for="messageType">발송 방식</label>
    <div class="flex gap-2" id="messageType">
      <button
        class="btn btn-sm {messageType === 'single' ? 'preset-filled-primary-500' : 'preset-tonal'}"
        onclick={() => messageType = "single"}
      >
        단일 디바이스
      </button>
      <button
        class="btn btn-sm {messageType === 'topic' ? 'preset-filled-primary-500' : 'preset-tonal'}"
        onclick={() => messageType = "topic"}
      >
        토픽
      </button>
    </div>
  </div>

  <!-- 대상 입력 -->
  <div class="mb-6">
    {#if messageType === "single"}
      <label class="block text-sm font-medium mb-1" for="token">FCM 디바이스 토큰</label>
      <input
        id="token"
        type="text"
        class="input w-full"
        placeholder="FCM 디바이스 토큰 입력"
        bind:value={token}
      />
    {:else}
      <label class="block text-sm font-medium mb-1" for="topic">토픽 이름</label>
      <input
        id="topic"
        type="text"
        class="input w-full"
        placeholder="토픽 이름 입력 (예: ko)"
        bind:value={topic}
      />
    {/if}
  </div>

  <hr class="my-6 border-surface-200-800" />

  <!-- Android Channel ID -->
  <div class="mb-6">
    <label class="block text-sm font-medium mb-1" for="androidChannelId">Android Channel ID</label>
    <input
      id="androidChannelId"
      type="text"
      class="input w-full"
        placeholder="Android Channel ID 입력"
        bind:value={androidChannelId}
      />
  </div>

  <hr class="my-6 border-surface-200-800" />

  <!-- 메시지 입력 -->
  <div class="space-y-4 mb-6">
    <div>
      <label class="block text-sm font-medium mb-1" for="title">메시지 제목</label>
      <input
        id="title"
        type="text"
        class="input w-full"
        placeholder="알림 제목"
        bind:value={title}
      />
    </div>
    <div>
      <label class="block text-sm font-medium mb-1" for="body">메시지 내용</label>
      <textarea
        id="body"
        class="textarea w-full min-h-[120px]"
        placeholder="알림 내용"
        bind:value={body}
      ></textarea>
    </div>
  </div>

  <!-- 버튼들 -->
  <div class="flex gap-2 mb-6">
    <button
      class="btn preset-tonal"
      onclick={() => { loadTemplates(); showTemplateModal = true }}
      disabled={isLoading}
    >
      템플릿에서 불러오기
    </button>
    <button
      class="btn preset-filled-primary-500 flex-1"
      onclick={handleSend}
      disabled={isLoading || !isAuthenticated}
    >
      {isLoading ? "발송 중..." : "발송"}
    </button>
    <button
      class="btn preset-tonal"
      onclick={reset}
      disabled={isLoading}
    >
      초기화
    </button>
  </div>

  <!-- 결과 표시 -->
  {#if result}
    <div class="card p-4 {resultType === 'success' ? 'bg-success-500/20' : 'bg-error-500/20'}">
      <h3 class="font-medium mb-1">발송 결과</h3>
      <p class="text-sm {resultType === 'success' ? 'text-success-500' : 'text-error-500'}">
        {result}
      </p>
    </div>
  {/if}

  <!-- FCM Request JSON -->
  {#if result && requestJson}
    <div class="card p-4 preset-filled-surface-200-800 mt-4">
      <h3 class="font-medium mb-2">FCM Request JSON</h3>
      <pre class="text-xs overflow-x-auto whitespace-pre-wrap bg-surface-900 p-3 rounded text-gray-300">{@html highlightJson(requestJson)}</pre>
    </div>
  {/if}
</div>

<!-- 템플릿 선택 모달 -->
{#if showTemplateModal}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onclick={() => showTemplateModal = false}>
    <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
    <div class="card preset-filled-surface-100-900 p-6 w-96 max-h-[80vh] overflow-y-auto" onclick={(e) => e.stopPropagation()}>
      <h2 class="text-lg font-semibold mb-4">템플릿 선택</h2>
      
      {#if templates.length === 0}
        <p class="text-sm opacity-50">저장된 템플릿이 없습니다</p>
      {:else}
        <div class="space-y-2">
          {#each templates as template}
            <button
              class="w-full text-left p-3 rounded-lg hover:bg-surface-200-800 transition-colors"
              onclick={() => loadFromTemplate(template)}
            >
              <div class="font-medium">{template.name}</div>
              <div class="text-xs opacity-50 truncate">{template.title}</div>
            </button>
          {/each}
        </div>
      {/if}
      
      <div class="mt-4 flex justify-end">
        <button class="btn preset-tonal" onclick={() => showTemplateModal = false}>
          닫기
        </button>
      </div>
    </div>
  </div>
{/if}
