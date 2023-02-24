import { invoke } from "@tauri-apps/api";

export interface RemoteConfig {
  server_name: string;
  folder_name: string;
  files: string[];
  last_modified: Date;
}

export async function downloadConfig(url: string): Promise<RemoteConfig> {
  let config = await invoke<RemoteConfig>("fetch_config", { url });
  config.last_modified = new Date(config.last_modified);
  return config;
}
