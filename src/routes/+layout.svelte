<script lang="ts">
  import { onMount } from "svelte"
  import "../app.css"
  import Sidebar from "$lib/components/Sidebar.svelte"
  import SettingsView from "$lib/components/SettingsView.svelte"
  import type { Snippet } from "svelte"

  interface Props {
    children: Snippet
  }

  let { children }: Props = $props()

  let activeTab = $state<"general" | "settings">("general")

  function handleTabChange(tab: "general" | "settings") {
    activeTab = tab
  }

  onMount(() => {
    // 저장된 테마 불러와서 적용
    const savedTheme = localStorage.getItem("theme") as "system" | "light" | "dark" | null
    const theme = savedTheme || "system"
    const html = document.documentElement
    if (theme === "system") {
      const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches
      html.classList.toggle("dark", prefersDark)
    } else {
      html.classList.toggle("dark", theme === "dark")
    }
  })
</script>

<div class="grid grid-cols-[auto_1fr] h-screen">
  <Sidebar {activeTab} onTabChange={handleTabChange} />
  <main class="overflow-y-auto">
    {#if activeTab === "general"}
      {@render children()}
    {:else}
      <SettingsView />
    {/if}
  </main>
</div>
