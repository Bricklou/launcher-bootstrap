use serde::Serialize;
use tauri::Manager;
use url::Url;

#[derive(Debug, Serialize, Clone)]
enum LinkEvent {
    NewConfig(String),
    OpenConfig(String),
}

#[derive(Debug, Serialize)]
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

    println!("path: {:?}", path);

    match path {
        "new-config" => {
            if let Some(config_name) = query_map.get("url") {
                println!("config_name: {:?}", config_name);

                let payload = LinkEventPayload {
                    event_type: LinkEvent::NewConfig(config_name.to_string()),
                    data: config_name.clone(),
                };
                handle.emit_all("link-event", serde_json::to_string(&payload)?)?;
            }
        }
        "open-config" => {
            if let Some(config_name) = query_map.get("url") {
                println!("config_name: {:?}", config_name);

                let payload = LinkEventPayload {
                    event_type: LinkEvent::OpenConfig(config_name.to_string()),
                    data: config_name.clone(),
                };
                handle.emit_all("link-event", serde_json::to_string(&payload)?)?;
            }
        }
        _ => {}
    }

    Ok(())
}
