<script lang="ts">
  import type { WheelItem } from "../../lib/types";
  import { wheelItems } from "../../lib/stores";
  import { updateWheelLayout, getApps } from "../../lib/commands";
  import { onMount } from "svelte";

  let items: WheelItem[] = [];
  let apps: { title: string; path: string }[] = [];

  onMount(async () => {
    // Load current wheel layout
    items = $wheelItems.length > 0 ? [...$wheelItems] : getDefaultItems();

    // Load apps for dropdown
    const searchItems = await getApps();
    apps = searchItems
      .filter(s => s.action.type === "LaunchApp")
      .map(s => ({ title: s.title, path: s.action.path }));
  });

  function getDefaultItems(): WheelItem[] {
    return [
      { sector: 6, title: "VS Code", icon: "💻", action: { type: "LaunchApp", path: "code" } },
      { sector: 7, title: "Browser", icon: "🌐", action: { type: "LaunchApp", path: "msedge" } },
      { sector: 0, title: "Terminal", icon: "⬛", action: { type: "LaunchApp", path: "wt" } },
      { sector: 1, title: "Explorer", icon: "📁", action: { type: "LaunchApp", path: "explorer" } },
      { sector: 2, title: "Calc", icon: "🧮", action: { type: "LaunchApp", path: "calc" } },
      { sector: 3, title: "Snipping", icon: "✂️", action: { type: "LaunchApp", path: "snippingtool" } },
      { sector: 4, title: "Notepad", icon: "📝", action: { type: "LaunchApp", path: "notepad" } },
      { sector: 5, title: "Settings", icon: "⚙️", action: { type: "LaunchApp", path: "ms-settings:" } },
    ];
  }

  async function save() {
    $wheelItems = items;
    await updateWheelLayout(items);
  }

  const directionLabels = ["→ Right", "↘ BR", "↓ Down", "↙ BL", "← Left", "↖ TL", "↑ Up", "↗ TR"];
</script>

<div class="wheel-editor">
  <h3>Wheel Sector Configuration</h3>
  <p class="hint">Assign apps to each of the 8 wheel sectors.</p>

  {#each items as item, i}
    <div class="sector-row">
      <span class="sector-dir">{directionLabels[item.sector]}</span>
      <input type="text" bind:value={item.icon} class="icon-input" maxlength="2" title="Icon (emoji)" />
      <input type="text" bind:value={item.title} placeholder="App name" class="title-input" />
      <select bind:value={item.action.path}>
        <option value={item.action.path}>{item.action.path}</option>
        {#each apps as app}
          <option value={app.path}>{app.title}</option>
        {/each}
      </select>
    </div>
  {/each}

  <button class="save-btn" on:click={save}>Save Layout</button>
</div>

<style>
  .wheel-editor h3 {
    margin: 0 0 4px;
  }

  .hint {
    margin: 0 0 16px;
    font-size: 12px;
    color: rgba(255, 255, 255, 0.4);
  }

  .sector-row {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 10px;
  }

  .sector-dir {
    width: 60px;
    font-size: 12px;
    color: rgba(255, 255, 255, 0.5);
  }

  .icon-input {
    width: 40px;
    text-align: center;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    color: #fff;
    padding: 6px;
    font-size: 16px;
  }

  .title-input {
    flex: 1;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    color: #fff;
    padding: 6px 10px;
    font-size: 13px;
  }

  select {
    width: 160px;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    color: #fff;
    padding: 6px 8px;
    font-size: 12px;
  }

  select option {
    background: #2a2a2a;
    color: #fff;
  }

  .save-btn {
    margin-top: 16px;
    padding: 8px 24px;
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
</style>
