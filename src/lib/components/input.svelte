<script>
  import { appState } from "$lib/appState.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let { style, data, placeholder } = $props();
  let search = $state("");

  /**
   * @type {HTMLInputElement}
   */
  let input;

  onMount(() => {
    const handleFocusSearch = () => {
      input?.focus();
    };

    document.addEventListener("focus-search-input", handleFocusSearch);

    return () => {
      document.removeEventListener("focus-search-input", handleFocusSearch);
    };
  });
</script>

<input
  type="text"
  {style}
  placeholder={placeholder ?? "Search"}
  bind:this={input}
  bind:value={search}
  autocomplete="off"
  spellcheck="false"
  oninput={async () => {
    appState.selected = -1;
    if (search === "") {
      appState.filteredData = null;
      return;
    }
    const res = await invoke("autodetect_search", { query: search });
    appState.filteredData = res.matches;
  }}
/>
