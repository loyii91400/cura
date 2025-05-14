<script>
  import { appState } from "$lib/appState.svelte";

  let { style, data } = $props();
</script>

<div {style}>{data.join("")}</div>
