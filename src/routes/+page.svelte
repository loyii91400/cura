<script>
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import Component from "$lib/components/component.svelte";
  import { registerKeymaps } from "$lib/registerKeymaps";
  import { appState, getData } from "$lib/appState.svelte";
  import {
    checkAccessibilityPermission,
    checkFullDiskAccessPermission,
    requestAccessibilityPermission,
    requestFullDiskAccessPermission,
  } from "tauri-plugin-macos-permissions-api";
  import { window } from "@tauri-apps/api";

  let data = $derived.by(() => {
    return getData();
  });

  onMount(async () => {
    // ensure window is ceneted
    const currentWindow = getCurrentWindow();
    currentWindow.center();

    // check file permissions
    const authorizedAccessibility = await checkAccessibilityPermission();
    if (!authorizedAccessibility) {
      await requestAccessibilityPermission();
    }
    const authorized = await checkFullDiskAccessPermission();
    if (!authorized) {
      await requestFullDiskAccessPermission();
    }

    appState.delimiter = await invoke("read_delimiter");
    appState.stdIn = await invoke("read_stdin");
    await window.getCurrentWindow().onFocusChanged(({ payload: focused }) => {
      if (!focused) {
        invoke("exit_app");
      }
    });

    try {
      appState.conf = await invoke("read_config_state");
      document.body.style.backgroundColor = "transparent";
    } catch (error) {
      console.log(error);
    }

    try {
      appState.layout = await invoke("read_layout_state");
      console.log(appState.layout);
    } catch (error) {
      console.log(error);
    }

    await registerKeymaps(appState.conf);
  });
</script>

<Component layout={appState.layout} {data}></Component>

<style>
</style>
