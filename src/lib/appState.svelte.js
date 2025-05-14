/**
 * @typedef {Object} Config
 * @property {string} version
 * @property {boolean} exit_on_esc
 * @property {number} width
 * @property {number} height
 * @property {number} scroll_delay
 * @property {number} scroll_speed
 */

/**
 * @typedef {Object} Layout
 * @property {string} component
 * @property {Object} [style]
 * @property {Layout[]} [children]
 * @property {string} [id]
 * @property {string} [data_index]
 * @property {string} [placeholder]
 * @property {Object} [selected_style]
 * @property {number} [data_index]
 */

/**
 * @typedef {Object} AppState
 * @property {Layout|null} layout
 * @property {Config|null} conf
 * @property {string[]|null} stdIn
 * @property {string[]|null} filteredData
 * @property {string} delimiter
 * @property {number} selected
 */

/**
 * @type {AppState}
 */
export const appState = $state({
  layout: null,
  conf: null,
  stdIn: null,
  filteredData: null,
  delimiter: "",
  selected: -1,
});

// Export the derived data
export function getData() {
  if (appState.filteredData != null) {
    return appState.filteredData ?? [];
  }
  return appState.stdIn ?? [];
}
