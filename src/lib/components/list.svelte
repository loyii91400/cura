<script>
  // @ts-nocheck
  import Component from "$lib/components/component.svelte";
  import ComponentWrapper from "$lib/components/component.svelte";

  import { appState } from "$lib/appState.svelte";

  let listRef;
  /**
   * @type {HTMLElement[]}
   */
  let componentRefs = $state([]);

  let { layout, style, data } = $props();

  // @ts-ignore
  function styleToString(style) {
    if (!style) return "";
    return Object.entries(style)
      .map(([key, value]) => {
        // Convert camelCase to kebab-case
        const kebabKey = key
          .replace(/([a-z0-9])([A-Z])/g, "$1-$2")
          .toLowerCase();
        return `${kebabKey}: ${value};`;
      })
      .join(" ");
  }

  $effect(() => {
    if (!listRef) return;

    if (
      appState.selected >= 0 &&
      componentRefs[appState.selected] != undefined
    ) {
      const containerRect = listRef.getBoundingClientRect();
      const itemRect = componentRefs[appState.selected].getBoundingClientRect();

      const isFullyVisible =
        itemRect.top >= containerRect.top &&
        itemRect.bottom <= containerRect.bottom;

      if (!isFullyVisible) {
        const itemTop =
          componentRefs[appState.selected].offsetTop - listRef.offsetTop;

        listRef.scrollTo({
          top: itemTop,
          behavior: "smooth",
        });
      }
    }
  });
</script>

<div bind:this={listRef} {style}>
  {#each data as item, i}
    <span bind:this={componentRefs[i]}>
      {#if layout.children && layout.children.length > 0}
        {#each layout.children as child}
          <Component
            selected={appState.selected === i}
            layout={child}
            data={[data[i]]}
          />
        {/each}
      {/if}
    </span>
  {/each}
</div>
