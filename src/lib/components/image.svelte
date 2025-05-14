<script>
  import { appState } from "$lib/appState.svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";

  let { style, data } = $props();
</script>

<!-- svelte-ignore a11y_missing_attribute -->
<img {style} src={convertFileSrc(data.join(""))} alt={data.join("")} />
