<script>
  import Self from "$lib/components/component.svelte";
  import Input from "$lib/components/input.svelte";
  import List from "$lib/components/list.svelte";
  import Container from "$lib/components/container.svelte";
  import Text from "$lib/components/text.svelte";
  import Image from "$lib/components/image.svelte";
  import { appState } from "$lib/appState.svelte";

  /**
   * @type{Object.<string, import("svelte").Component>}
   */
  const ComponentMap = {
    container: Container,
    list: List,
    input: Input,
    text: Text,
    image: Image,
  };

  let { selected = false, scrollOnSelected = false, layout, data } = $props();

  // @ts-ignore
  function styleToString(style) {
    if (!style) return "";
    return Object.entries(style)
      .map(([key, value]) => {
        return `${key}: ${value};`;
      })
      .join(" ");
  }

  let splittedData = $derived.by(() => {
    if (appState.delimiter != "" && data.length == 1) {
      let split = data[0].split(appState.delimiter);
      if (
        layout.data_index != undefined &&
        layout.data_index >= 0 &&
        layout.data_index < split.length
      ) {
        return [split[layout.data_index]];
      }
    }

    return data;
  });
</script>

{#if layout}
  {@const Component = ComponentMap[layout.component]}
  {#if Component}
    <Component
      {layout}
      style={styleToString({
        ...layout.style,
        ...(selected ? layout.selected_style : {}),
      })}
      data={splittedData}
      placeholder={layout.placeholder}
      {selected}
    >
      {#if layout.children && layout.children.length > 0}
        {#each layout.children as child}
          <Self layout={child} {data} {selected} />
        {/each}
      {/if}
    </Component>
  {:else}
    <div style="background-color: red; width: 100%; height: 100%;">
      Not a valid component {layout.component}
    </div>
  {/if}
{/if}
