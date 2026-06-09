<script lang="ts">
  import type { SearchItem } from "../../lib/types";

  export let item: SearchItem;
  export let active: boolean = false;

  $: typeLabel = item.item_type === "Application" ? "App"
    : item.item_type === "File" ? "File"
    : item.item_type === "Folder" ? "Folder"
    : "";
</script>

<div class="result-item" class:active>
  <span class="item-icon">{item.icon}</span>
  <div class="item-info">
    <span class="item-title">{item.title}</span>
    <span class="item-subtitle">{item.subtitle}</span>
  </div>
  <span class="item-type">{typeLabel}</span>
</div>

<style>
  .result-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 12px;
    border-radius: 8px;
    transition: background 80ms ease;
  }

  .result-item.active,
  .result-item:hover {
    background: rgba(255, 255, 255, 0.08);
  }

  .item-icon {
    font-size: 20px;
    flex-shrink: 0;
  }

  .item-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .item-title {
    font-size: 14px;
    color: #fff;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .item-subtitle {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.4);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .item-type {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.25);
    flex-shrink: 0;
  }
</style>
