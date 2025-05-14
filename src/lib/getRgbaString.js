/**
 * Converts a CSS color string (e.g., 'red', '#f00', 'rgb(255, 0, 0)') and an opacity value to an RGBA string.
 *
 * @param {string} color - The CSS color string.
 * @param {number} opacity - The opacity value (0 to 1).
 * @returns {string} The RGBA string (e.g., 'rgba(255, 0, 0, 0.5)').  Returns an empty string on error.
 */
export function getRgbaString(color, opacity) {
  if (typeof opacity !== "number" || opacity < 0 || opacity > 1) {
    return ""; // Invalid opacity
  }

  let r, g, b;

  // Handle named colors
  const namedColors = {
    black: "#000000",
    silver: "#c0c0c0",
    gray: "#808080",
    white: "#ffffff",
    maroon: "#800000",
    red: "#ff0000",
    purple: "#800080",
    fuchsia: "#ff00ff",
    green: "#008000",
    lime: "#00ff00",
    olive: "#808000",
    yellow: "#ffff00",
    navy: "#000080",
    blue: "#0000ff",
    teal: "#008080",
    aqua: "#00ffff",
    orange: "#ffa500", // Added orange
    pink: "#ffc0cb", // Added pink
    brown: "#a52a2a", // Added brown
    cyan: "#00ffff",
  };

  color = color.toLowerCase(); //simplify checks

  if (namedColors[color]) {
    color = namedColors[color];
  }

  // Handle hex colors
  if (color.startsWith("#")) {
    if (color.length === 4) {
      // #rgb
      r = parseInt(color[1] + color[1], 16);
      g = parseInt(color[2] + color[2], 16);
      b = parseInt(color[3] + color[3], 16);
    } else if (color.length === 7) {
      // #rrggbb
      r = parseInt(color.substring(1, 3), 16);
      g = parseInt(color.substring(3, 5), 16);
      b = parseInt(color.substring(5, 7), 16);
    } else {
      return ""; // Invalid hex format
    }
  }
  // Handle rgb/rgba colors
  else if (color.startsWith("rgb")) {
    const values = color
      .substring(color.indexOf("(") + 1, color.indexOf(")"))
      .split(",");
    if (values.length < 3 || values.length > 4) return "";

    r = parseInt(values[0].trim());
    g = parseInt(values[1].trim());
    b = parseInt(values[2].trim());
    if (values.length === 4) {
      opacity = parseFloat(values[3].trim());
      if (isNaN(opacity) || opacity < 0 || opacity > 1) return "";
    }
  } else {
    return ""; // Invalid color format
  }

  // Check if RGB values are valid
  if (
    isNaN(r) ||
    r < 0 ||
    r > 255 ||
    isNaN(g) ||
    g < 0 ||
    g > 255 ||
    isNaN(b) ||
    b < 0 ||
    b > 255
  ) {
    return "";
  }
  return `rgba(${r}, ${g}, ${b}, ${opacity})`;
}
