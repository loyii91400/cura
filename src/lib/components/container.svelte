<script>
  import { appState } from "$lib/appState.svelte";
  let { style, children } = $props();
</script>

<div {style}>
  {@render children?.()}
</div>
