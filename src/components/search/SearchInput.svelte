<script lang="ts">
  import { searchQuery } from "../../lib/stores";

  let value = "";

  function handleInput(e: Event) {
    value = (e.target as HTMLInputElement).value;
    $searchQuery = value;
  }

  // Auto-focus on mount
  let inputEl: HTMLInputElement;
  $effect(() => {
    inputEl?.focus();
  });
</script>

<div class="search-input-wrapper">
  <span class="search-icon">🔍</span>
  <input
    bind:this={inputEl}
    type="text"
    placeholder="Search apps, files, commands..."
    {value}
    on:input={handleInput}
    spellcheck="false"
    autocomplete="off"
  />
</div>

<style>
  .search-input-wrapper {
    display: flex;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
  }

  .search-icon {
    font-size: 16px;
    margin-right: 12px;
    opacity: 0.5;
  }

  input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: #fff;
    font-size: 18px;
    font-family: 'Segoe UI', system-ui, sans-serif;
  }

  input::placeholder {
    color: rgba(255, 255, 255, 0.35);
  }
</style>
