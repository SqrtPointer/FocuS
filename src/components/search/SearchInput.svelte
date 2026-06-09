<script lang="ts">
  import { searchQuery } from "../../lib/stores";

  let value = $state("");

  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    value = target.value;
    $searchQuery = target.value;
  }

  // Auto-focus on mount
  let inputEl: HTMLInputElement | undefined = $state();
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
    oninput={handleInput}
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
