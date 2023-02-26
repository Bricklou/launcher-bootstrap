use serde::Serialize;
use tauri::Manager;
use url::Url;

#[derive(Debug, Serialize, Clone)]
enum LinkEvent {
    #[serde(rename = "new-config")]
    NewConfig,
    #[serde(rename = "open-config")]
    OpenConfig,
}

#[derive(Debug, Clone, Serialize)]
struct LinkEventPayload {
    event_type: LinkEvent,
    data: String,
}

pub fn register_link_events(
    request: String,
    handle: &tauri::AppHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_url = Url::parse(&request)?;

    let path = input_url.domain().unwrap_or("");
    // Convert the query string pairs into a map
    let query_map: std::collections::HashMap<_, _> = input_url.query_pairs().into_owned().collect();

    let mut window = handle.get_window("main").unwrap();

    match path {
        "new-config" => {
            if let Some(config_name) = query_map.get("url") {
                let payload = LinkEventPayload {
                    event_type: LinkEvent::NewConfig,
                    data: config_name.clone(),
                };
                handle.emit_all("link-event", payload)?;
                window.request_user_attention(Some(tauri::UserAttentionType::Informational))?;
            }
        }
        "open-config" => {
            if let Some(config_name) = query_map.get("url") {
                let payload = LinkEventPayload {
                    event_type: LinkEvent::OpenConfig,
                    data: config_name.clone(),
                };
                handle.emit_all("link-event", payload)?;
                window.request_user_attention(Some(tauri::UserAttentionType::Informational))?;
            }
        }
        _ => {}
    }

    Ok(())
}
