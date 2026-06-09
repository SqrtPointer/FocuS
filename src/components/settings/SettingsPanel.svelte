<script lang="ts">
  import AppearanceSettings from "./AppearanceSettings.svelte";
  import WheelEditor from "./WheelEditor.svelte";
  import { appConfig } from "../../lib/stores";
  import { getConfig, updateConfig } from "../../lib/commands";
  import { onMount } from "svelte";

  let activeTab: "appearance" | "wheel" = "appearance";
  let config: import("../../lib/types").AppConfig | null = null;

  onMount(async () => {
    config = await getConfig();
    $appConfig = config;
  });

  async function save() {
    if (config) {
      await updateConfig(config);
    }
  }
</script>

<div class="settings-container">
  <header class="settings-header">
    <h1>FocuS Settings</h1>
    <button class="save-btn" on:click={save}>Save</button>
  </header>

  <nav class="settings-tabs">
    <button class:active={activeTab === "appearance"} on:click={() => activeTab = "appearance"}>
      Appearance
    </button>
    <button class:active={activeTab === "wheel"} on:click={() => activeTab = "wheel"}>
      Wheel Layout
    </button>
  </nav>

  <main class="settings-body">
    {#if activeTab === "appearance" && config}
      <AppearanceSettings bind:config />
    {:else if activeTab === "wheel" && config}
      <WheelEditor />
    {/if}
  </main>
</div>

<style>
  .settings-container {
    height: 100vh;
    display: flex;
    flex-direction: column;
    background: #1e1e1e;
    color: #fff;
    font-family: 'Segoe UI', system-ui, sans-serif;
  }

  .settings-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 24px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
  }

  .settings-header h1 {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
  }

  .save-btn {
    padding: 6px 16px;
    background: #60CDFF;
    color: #000;
    border: none;
    border-radius: 6px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
  }

  .save-btn:hover {
    background: #80D8FF;
  }

  .settings-tabs {
    display: flex;
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    padding: 0 24px;
  }

  .settings-tabs button {
    padding: 10px 20px;
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    color: rgba(255, 255, 255, 0.5);
    cursor: pointer;
    font-size: 13px;
  }

  .settings-tabs button.active {
    color: #fff;
    border-bottom-color: #60CDFF;
  }

  .settings-body {
    flex: 1;
    overflow-y: auto;
    padding: 24px;
  }
</style>
