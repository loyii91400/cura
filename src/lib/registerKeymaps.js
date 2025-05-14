// @ts-nocheck
import { invoke } from "@tauri-apps/api/core";
import { register } from "@tauri-apps/plugin-global-shortcut";
import { appState, getData } from "$lib/appState.svelte";

// Store for tracking hold state and intervals for all keys
const keyState = {
  keys: {}, // { [key]: { isPressed, timeout, interval } }
};

/**
 * @param {string} action
 */
function actionFactory(action) {
  switch (action) {
    case "exit":
      return (event) => {
        invoke("exit_app").catch((error) => {
          console.log(error);
        });
      };
    case "select":
      return (event) => {
        if (getData().length == 0) {
          return;
        }

        if (event.state == "Released") {
          let selected = appState.selected;
          if (selected < 0) {
            selected = 0;
          }

          let output = getData()[selected];
          invoke("print", {
            out: output,
          }).then(() => {
            invoke("exit_app").catch((error) => {
              console.log(error);
            });
          });
        }
      };
    case "search":
      return (event) => {
        document.dispatchEvent(new CustomEvent("focus-search-input"));
      };
    case "cycle_next":
      return (event) => {
        const dataLength = getData().length;
        if (dataLength === 0) return;

        const key = event.key || event.shortcut; // Use the key identifier
        if (!keyState.keys[key]) {
          keyState.keys[key] = {
            isPressed: false,
            timeout: null,
            interval: null,
          };
        }

        const state = keyState.keys[key];

        if (event.state === "Pressed") {
          // Immediate cycle on press
          appState.selected = Math.min(dataLength - 1, appState.selected + 1);
          state.isPressed = true;

          // Clear any existing timeout/interval
          if (state.timeout) clearTimeout(state.timeout);
          if (state.interval) clearInterval(state.interval);

          // Set timeout for continuous cycling after 1 second
          state.timeout = setTimeout(() => {
            state.interval = setInterval(() => {
              if (state.isPressed) {
                appState.selected = Math.min(
                  dataLength - 1,
                  appState.selected + 1,
                );
              }
            }, appState.conf.scroll_delay); // 100ms delay between cycles
          }, appState.conf.scroll_speed); // 1-second initial delay
        } else if (event.state === "Released") {
          state.isPressed = false;
          if (state.timeout) clearTimeout(state.timeout);
          if (state.interval) clearInterval(state.interval);
        }
      };
    case "cycle_previous":
      return (event) => {
        const dataLength = getData().length;
        if (dataLength === 0) return;

        const key = event.key || event.shortcut; // Use the key identifier
        if (!keyState.keys[key]) {
          keyState.keys[key] = {
            isPressed: false,
            timeout: null,
            interval: null,
          };
        }

        const state = keyState.keys[key];

        if (event.state === "Pressed") {
          // Immediate cycle on press
          appState.selected = Math.max(0, appState.selected - 1);
          state.isPressed = true;

          // Clear any existing timeout/interval
          if (state.timeout) clearTimeout(state.timeout);
          if (state.interval) clearInterval(state.interval);

          // Set timeout for continuous cycling after 1 second
          state.timeout = setTimeout(() => {
            state.interval = setInterval(() => {
              if (state.isPressed) {
                appState.selected = Math.max(0, appState.selected - 1);
              }
            }, appState.conf.scroll_delay); // 100ms delay between cycles
          }, appState.conf.scroll_speed); // 1-second initial delay
        } else if (event.state === "Released") {
          state.isPressed = false;
          if (state.timeout) clearTimeout(state.timeout);
          if (state.interval) clearInterval(state.interval);
        }
      };
    default:
      break;
  }
}

export async function registerKeymaps(conf) {
  const exitOnEsc = conf.exit_on_esc ?? true;
  const selectOnReturn = conf.select_on_return ?? true;

  if (exitOnEsc) {
    const action = actionFactory("exit");
    if (action) {
      await register("ESCAPE", action);
    }
  }

  if (selectOnReturn) {
    const action = actionFactory("select");
    if (action) {
      await register("ENTER", action);
    }
  }

  // Register default cycle keys
  const cycleNextAction = actionFactory("cycle_next");
  if (cycleNextAction) {
    await register("DOWN", cycleNextAction);
  }

  const cyclePreviousAction = actionFactory("cycle_previous");
  if (cyclePreviousAction) {
    await register("UP", cyclePreviousAction);
  }

  // Register additional keymaps
  for (const [key, value] of Object.entries(conf.keymaps || {})) {
    const action = actionFactory(value);
    if (action) {
      await register(key, action);
    }
  }
}
