<script lang="ts">
  import type { AppConfig } from "../../lib/types";
  import { setAcrylicOpacity } from "../../lib/commands";

  export let config: AppConfig;

  function onSearchOpacityChange(e: Event) {
    const val = parseFloat((e.target as HTMLInputElement).value);
    config.appearance.search_bar.acrylic_opacity = val;
    setAcrylicOpacity("search", val);
  }

  function onWheelOpacityChange(e: Event) {
    const val = parseFloat((e.target as HTMLInputElement).value);
    config.appearance.wheel.acrylic_opacity = val;
    setAcrylicOpacity("wheel", val);
  }
</script>

<div class="appearance-settings">
  <section>
    <h3>Search Bar</h3>
    <label>
      <span>Acrylic Opacity</span>
      <input
        type="range"
        min="0.3"
        max="1.0"
        step="0.05"
        value={config.appearance.search_bar.acrylic_opacity}
        on:input={onSearchOpacityChange}
      />
      <span class="value">{Math.round(config.appearance.search_bar.acrylic_opacity * 100)}%</span>
    </label>
  </section>

  <section>
    <h3>Wheel</h3>
    <label>
      <span>Acrylic Opacity</span>
      <input
        type="range"
        min="0.3"
        max="1.0"
        step="0.05"
        value={config.appearance.wheel.acrylic_opacity}
        on:input={onWheelOpacityChange}
      />
      <span class="value">{Math.round(config.appearance.wheel.acrylic_opacity * 100)}%</span>
    </label>

    <label>
      <span>Border Color</span>
      <input
        type="color"
        bind:value={config.appearance.wheel.border_color}
      />
    </label>

    <label>
      <span>Border Width</span>
      <input
        type="range"
        min="1"
        max="6"
        step="0.5"
        bind:value={config.appearance.wheel.border_width}
      />
      <span class="value">{config.appearance.wheel.border_width}px</span>
    </label>
  </section>
</div>

<style>
  .appearance-settings section {
    margin-bottom: 28px;
  }

  .appearance-settings h3 {
    margin: 0 0 14px;
    font-size: 14px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.7);
  }

  label {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 12px;
    font-size: 13px;
    color: rgba(255, 255, 255, 0.8);
  }

  label span:first-child {
    width: 120px;
  }

  input[type="range"] {
    flex: 1;
    accent-color: #60CDFF;
    height: 4px;
  }

  input[type="color"] {
    width: 36px;
    height: 28px;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 4px;
    background: transparent;
    cursor: pointer;
    padding: 2px;
  }

  .value {
    width: 40px;
    text-align: right;
    font-variant-numeric: tabular-nums;
  }
</style>
