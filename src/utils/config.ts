import { invoke } from "@tauri-apps/api";

export interface RemoteConfig {
  server_name: string;
  folder_name: string;
  files_url: string;
  theme_url: string;
}

export async function downloadConfig(url: string): Promise<RemoteConfig> {
  let config = await invoke<RemoteConfig>("fetch_config", { url });
  return config;
}
