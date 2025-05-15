<script>
  import { appState } from "$lib/appState.svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import imageType from "image-type";

  let { style, data } = $props();

  const filePath = $derived(data.join(""));
  let fileData = $state("");

  let fileType = $state(null);

  /**
   * @param {string} filePath
   */
  function getFileType(filePath) {
    const extension = filePath.split(".").pop().toLowerCase();

    switch (extension) {
      case "jpg":
      case "jpeg":
      case "png":
      case "gif":
      case "bmp":
      case "webp":
        return "image";
      case "txt":
        return "text";
      // Add more cases for other file types as needed
      // case 'pdf':
      //   return 'pdf';
      default:
        return "none";
    }
  }

  onMount(async () => {
    // @ts-ignore
    fileType = getFileType(filePath);
    if (fileType == "text") {
      let res = await fetch(convertFileSrc(filePath));
      fileData = await res.text();
    }
    console.log(fileData);
  });
</script>

<!-- svelte-ignore a11y_missing_attribute -->
{#if fileType === "image"}
  <img {style} src={convertFileSrc(filePath)} alt={data.join("")} />
{:else if fileType === "text"}
  <p {style}>{fileData}</p>
{:else if fileType === "none"}
  <p {style}>{filePath}</p>
{/if}
