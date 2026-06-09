<script lang="ts">
  import SearchInput from "./SearchInput.svelte";
  import ResultList from "./ResultList.svelte";
  import { searchQuery, searchResults } from "../../lib/stores";
  import { search, hideSearch, getRecent } from "../../lib/commands";

  let query = "";
  let results: Awaited<ReturnType<typeof search>> = [];
  let selectedIndex = 0;

  // Subscribe to query changes for search
  $: {
    query = $searchQuery;
    if (query.length > 0) {
      search(query).then(r => results = r);
    } else {
      getRecent().then(r => results = r);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      hideSearch();
      return;
    }
    if (e.key === "ArrowDown") {
      e.preventDefault();
      selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
    }
    if (e.key === "ArrowUp") {
      e.preventDefault();
      selectedIndex = Math.max(selectedIndex - 1, 0);
    }
    if (e.key === "Enter" && results[selectedIndex]) {
      executeItem(results[selectedIndex]);
    }
  }

  function executeItem(item: (typeof results)[0]) {
    if (item.action.type === "LaunchApp") {
      import("../../lib/commands").then(m => m.launchApp(item.action.path));
    }
    hideSearch();
  }

  function handleBlur() {
    hideSearch();
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="search-container" on:click|self={handleBlur}>
  <div class="search-panel" role="dialog" aria-label="Search">
    <SearchInput />
    <ResultList {results} {selectedIndex} on:select={(e) => executeItem(e.detail)} />
  </div>
</div>

<style>
  .search-container {
    width: 100vw;
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: flex-start;
    padding-top: 20vh;
    background: transparent;
  }

  .search-panel {
    width: 680px;
    max-height: 500px;
    background: rgba(26, 26, 26, var(--acrylic-opacity, 0.75));
    backdrop-filter: blur(30px);
    -webkit-backdrop-filter: blur(30px);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 12px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    overflow: hidden;
    animation: panelIn 200ms cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes panelIn {
    from {
      opacity: 0;
      transform: scale(0.96) translateY(-10px);
    }
    to {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
  }
</style>
