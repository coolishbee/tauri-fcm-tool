<script lang="ts">
  interface Props {
    activeTab: "general" | "settings"
    onTabChange: (tab: "general" | "settings") => void
  }

  let { activeTab, onTabChange }: Props = $props()
</script>

<div class="h-screen border-r border-surface-200-800 flex flex-col w-48">
  <div class="p-4 border-b border-surface-200-800">
    <h1 class="text-lg font-bold">FCM Push Tool</h1>
  </div>

  <nav class="flex-1 p-2">
    <ul class="space-y-1">
      <li>
        <button
          class="w-full text-left px-3 py-2 rounded-lg transition-colors {activeTab === 'general' 
            ? 'bg-primary-500 text-white' 
            : 'hover:bg-surface-200-800'}"
          onclick={() => onTabChange("general")}
        >
          <span class="font-medium">일반</span>
          <p class="text-xs opacity-70 mt-0.5">푸시 발송 및 관리</p>
        </button>
      </li>
      <li>
        <button
          class="w-full text-left px-3 py-2 rounded-lg transition-colors {activeTab === 'settings' 
            ? 'bg-primary-500 text-white' 
            : 'hover:bg-surface-200-800'}"
          onclick={() => onTabChange("settings")}
        >
          <span class="font-medium">설정</span>
          <p class="text-xs opacity-70 mt-0.5">인증 및 앱 설정</p>
        </button>
      </li>
    </ul>
  </nav>

  <div class="p-2 border-t border-surface-200-800 text-xs opacity-50 text-center">
    v0.1.0
  </div>
</div>
