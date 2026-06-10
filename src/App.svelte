<script lang="ts">
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import SearchBar from "./components/search/SearchBar.svelte";
  import RadialWheel from "./components/wheel/RadialWheel.svelte";
  import SettingsPanel from "./components/settings/SettingsPanel.svelte";

  let view = $state("search");

  onMount(async () => {
    try {
      const win = getCurrentWindow();
      view = win.label; // "search", "wheel", or "settings"
    } catch {
      view = "search";
    }
  });
</script>

{#if view === "search"}
  <SearchBar />
{:else if view === "wheel"}
  <RadialWheel />
{:else if view === "settings"}
  <SettingsPanel />
{/if}
