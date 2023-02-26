import { invoke } from "@tauri-apps/api";

export interface RemoteConfig {
  server_name: string;
  folder_name: string;
  files_url: string;
  theme_url: string;
  url: string;
}

export async function downloadConfig(url: string): Promise<RemoteConfig> {
  const config = await invoke<Omit<RemoteConfig, "url">>("fetch_config", {
    url,
  });
  return {
    ...config,
    url,
  };
}

export async function createConfig(url: string, config: RemoteConfig) {
  await invoke<void>("create_config", { metadata: config, url });
}
