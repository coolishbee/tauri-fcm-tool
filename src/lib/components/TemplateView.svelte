<script lang="ts">
  import { onMount } from "svelte"
  import { commands, type Template } from "$lib/bindings"

  let templates = $state<Template[]>([])
  let selectedId = $state<string | null>(null)
  let name = $state("")
  let title = $state("")
  let body = $state("")
  let isLoading = $state(false)
  let message = $state("")
  let messageType = $state<"success" | "error" | "">("")

  onMount(async () => {
    await loadTemplates()
  })

  async function loadTemplates() {
    try {
      const result = await commands.getTemplates()
      if (result.status === "ok") {
        templates = result.data.templates || []
      } else {
        showMessage(`템플릿 로드 실패: ${result.error}`, "error")
      }
    } catch (e) {
      showMessage(`템플릿 로드 실패: ${e}`, "error")
    }
  }

  async function handleSave() {
    if (!name.trim()) {
      showMessage("템플릿 이름을 입력해주세요", "error")
      return
    }
    if (!title.trim() || !body.trim()) {
      showMessage("제목과 내용을 입력해주세요", "error")
      return
    }

    isLoading = true
    try {
      const now = new Date().toISOString()
      const template: Template = {
        id: selectedId || crypto.randomUUID(),
        name: name.trim(),
        title: title.trim(),
        body: body.trim(),
        createdAt: selectedId ? (templates.find(t => t.id === selectedId)?.createdAt || now) : now,
        updatedAt: now
      }

      const result = await commands.saveTemplate(template)
      if (result.status === "ok") {
        await loadTemplates()
        showMessage("템플릿이 저장되었습니다", "success")
      } else {
        showMessage(`저장 실패: ${result.error}`, "error")
      }
    } catch (e) {
      showMessage(`저장 실패: ${e}`, "error")
    } finally {
      isLoading = false
    }
  }

  async function handleDelete() {
    if (!selectedId) {
      showMessage("삭제할 템플릿을 선택해주세요", "error")
      return
    }

    if (!confirm("이 템플릿을 삭제하시겠습니까?")) return

    isLoading = true
    try {
      const result = await commands.deleteTemplate(selectedId)
      if (result.status === "ok") {
        await loadTemplates()
        handleNew()
        showMessage("템플릿이 삭제되었습니다", "success")
      } else {
        showMessage(`삭제 실패: ${result.error}`, "error")
      }
    } catch (e) {
      showMessage(`삭제 실패: ${e}`, "error")
    } finally {
      isLoading = false
    }
  }

  function selectTemplate(template: Template) {
    selectedId = template.id
    name = template.name
    title = template.title
    body = template.body
  }

  function handleNew() {
    selectedId = null
    name = ""
    title = ""
    body = ""
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

<div class="p-6 max-w-2xl mx-auto">
  {#if message}
    <div class="mb-4 p-3 rounded-lg {messageType === 'success' ? 'bg-success-500/20 text-success-500' : 'bg-error-500/20 text-error-500'}">
      {message}
    </div>
  {/if}

  <!-- 템플릿 목록 -->
  <div class="mb-6">
    <label class="block text-sm font-medium mb-2">저장된 템플릿</label>
    <select
      class="select w-full"
      value={selectedId || ""}
      onchange={(e) => {
        const id = (e.target as HTMLSelectElement).value
        if (id) {
          const template = templates.find(t => t.id === id)
          if (template) selectTemplate(template)
        }
      }}
    >
      <option value="">-- 템플릿 선택 --</option>
      {#each templates as template}
        <option value={template.id}>{template.name}</option>
      {/each}
    </select>
  </div>

  <hr class="my-6 border-surface-200-800" />

  <!-- 편집 폼 -->
  <div class="space-y-4 mb-6">
    <div>
      <label class="block text-sm font-medium mb-1" for="name">템플릿 이름</label>
      <input
        id="name"
        type="text"
        class="input w-full"
        placeholder="템플릿 이름"
        bind:value={name}
      />
    </div>
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
  <div class="flex gap-2">
    <button
      class="btn preset-filled-primary-500 flex-1"
      onclick={handleSave}
      disabled={isLoading}
    >
      {isLoading ? "저장 중..." : "저장"}
    </button>
    <button
      class="btn preset-filled-error-500"
      onclick={handleDelete}
      disabled={isLoading || !selectedId}
    >
      삭제
    </button>
    <button
      class="btn preset-tonal"
      onclick={handleNew}
      disabled={isLoading}
    >
      새 템플릿
    </button>
  </div>
</div>
