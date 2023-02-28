import { invoke } from "@tauri-apps/api";

export interface RemoteConfig {
  server_name: string;
  folder_name: string;
  files_url: string;
  theme_url: string;
  url: string;
}

export interface LocalConfigFile {
  configs: Record<string, Omit<RemoteConfig, "url">>;
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

export async function getConfigs(): Promise<LocalConfigFile> {
  const config = await invoke<LocalConfigFile>("get_configs");
  return config;
}

export async function checkIfConfigExists(url: string): Promise<boolean> {
  const configs = await getConfigs();
  return configs.configs[url] !== undefined;
}

export async function getConfig(url: string): Promise<RemoteConfig | null> {
  const configs = await getConfigs();
  if (!configs.configs[url]) {
    return null;
  }

  return {
    ...configs.configs[url],
    url,
  };
}
