<script lang="ts">
  import SearchBar from "./components/search/SearchBar.svelte";
  import RadialWheel from "./components/wheel/RadialWheel.svelte";
  import SettingsPanel from "./components/settings/SettingsPanel.svelte";
  import { currentView } from "./lib/stores";

  // Determine which view to show based on the Tauri window label
  function getWindowLabel(): string {
    // The window label is available via Tauri API at runtime
    // Default to search for now
    return "search";
  }

  $: view = $currentView || getWindowLabel();
</script>

{#if view === "search"}
  <SearchBar />
{:else if view === "wheel"}
  <RadialWheel />
{:else if view === "settings"}
  <SettingsPanel />
{/if}
