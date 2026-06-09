<script lang="ts">
  import ResultItem from "./ResultItem.svelte";
  import type { SearchItem } from "../../lib/types";

  let { results = [], selectedIndex = 0, onselect }: {
    results: SearchItem[];
    selectedIndex: number;
    onselect?: (item: SearchItem) => void;
  } = $props();
</script>

<div class="result-list">
  {#if results.length === 0}
    <div class="empty-state">Start typing to search...</div>
  {:else}
    {#each results as item, i}
      <button
        class="result-row"
        class:active={i === selectedIndex}
        onclick={() => onselect?.(item)}
      >
        <ResultItem {item} active={i === selectedIndex} />
      </button>
    {/each}
  {/if}
</div>

<style>
  .result-list {
    max-height: 360px;
    overflow-y: auto;
    padding: 8px;
  }

  .empty-state {
    padding: 24px;
    text-align: center;
    color: rgba(255, 255, 255, 0.3);
    font-size: 14px;
  }

  .result-row {
    display: block;
    width: 100%;
    background: none;
    border: none;
    text-align: left;
    cursor: pointer;
    padding: 0;
  }

  .result-list::-webkit-scrollbar {
    width: 4px;
  }

  .result-list::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 2px;
  }
</style>
