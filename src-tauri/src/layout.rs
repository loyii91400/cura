use std::{path::PathBuf, sync::Mutex};

use tauri::State;

use crate::AppState;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Layout {
    pub component: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<std::collections::HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Layout>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_style: Option<std::collections::HashMap<String, String>>,
}

impl Default for Layout {
    fn default() -> Self {
        Layout {
            component: "container".to_string(),
            style: Some(std::collections::HashMap::from([
                ("padding".to_string(), "8px".to_string()),
                ("background".to_string(), "#eaeaea".to_string()),
                ("overflow".to_string(), "hidden".to_string()),
                ("height".to_string(), "100vh".to_string()),
            ])),
            children: vec![
                Layout {
                    component: "input".to_string(),
                    id: Some("search".to_string()),
                    placeholder: Some("Search...".to_string()),
                    style: Some(std::collections::HashMap::from([
                        ("margin-bottom".to_string(), "8px".to_string()),
                        ("background".to_string(), "#f9f9f9".to_string()),
                        ("padding".to_string(), "8px".to_string()),
                        ("z-index".to_string(), "1".to_string()),
                    ])),
                    selected_style: None,
                    data_index: None,
                    children: Vec::new(),
                },
                Layout {
                    component: "list".to_string(),
                    style: Some(std::collections::HashMap::from([
                        ("height".to_string(), "100vh".to_string()),
                        ("overflow-y".to_string(), "auto".to_string()),
                        ("border".to_string(), "1px solid #ccc".to_string()),
                        ("border-radius".to_string(), "4px".to_string()),
                        ("background-color".to_string(), "#fff".to_string()),
                    ])),
                    children: vec![Layout {
                        component: "container".to_string(),
                        style: Some(std::collections::HashMap::from([
                            ("max-width".to_string(), "100%".to_string()),
                            ("word-warp".to_string(), "break-word".to_string()),
                            ("padding".to_string(), "8px".to_string()),
                            ("background".to_string(), "#ffffff".to_string()),
                            ("margin-bottom".to_string(), "8px".to_string()),
                            ("border-radius".to_string(), "4px".to_string()),
                            (
                                "box-shadow".to_string(),
                                "0 1px 3px rgba(0,0,0,0.12)".to_string(),
                            ),
                        ])),
                        selected_style: Some(std::collections::HashMap::from([(
                            "border".to_string(),
                            "thick double #32a1ce".to_string(),
                        )])),
                        children: vec![Layout {
                            component: "text".to_string(),
                            data_index: None,
                            style: Some(std::collections::HashMap::from([
                                ("margin-bottom".to_string(), "4px".to_string()),
                                ("font-weight".to_string(), "bold".to_string()),
                                ("word-warp".to_string(), "break-word".to_string()),
                            ])),
                            id: None,
                            placeholder: None,
                            selected_style: None,
                            children: Vec::new(),
                        }],
                        id: None,
                        placeholder: None,
                        data_index: None,
                    }],
                    id: None,
                    placeholder: None,
                    data_index: None,
                    selected_style: None,
                },
            ],
            id: None,
            placeholder: None,
            data_index: None,
            selected_style: None,
        }
    }
}

pub fn get_layout_path(path: &str) -> Result<PathBuf, String> {
    if !path.is_empty() {
        return Ok(PathBuf::from(path));
    }
    Err("No path".to_string())
}

pub fn read_layout(path: &str) -> Result<Layout, String> {
    let layout_path = get_layout_path(path)?;

    if !layout_path.exists() {
        return Ok(Layout::default());
    }

    let contents = std::fs::read_to_string(&layout_path)
        .map_err(|e| format!("Failed to read {}: {}", layout_path.display(), e))?;

    let config: Layout =
        serde_json::from_str(&contents).map_err(|e| format!("Failed to parse layout: {}", e))?;

    Ok(config)
}

#[tauri::command]
pub fn read_layout_state(state: State<Mutex<AppState>>) -> Layout {
    let state = state.lock().unwrap();
    state.layout.clone()
}
