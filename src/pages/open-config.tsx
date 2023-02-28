import { useEffect, useState } from "react";
import { useLocation } from "react-router-dom";
import { getConfig, RemoteConfig } from "../utils/config";

export function OpenConfigPage(): JSX.Element {
  const location = useLocation();
  const url = location.state.url;
  const [config, setConfig] = useState<RemoteConfig | null>();

  const fetchConfig = async (): Promise<void> => {
    setConfig(await getConfig(url));
  };

  useEffect(() => {
    void fetchConfig();
  }, []);

  return (
    <div>
      <h1>Open config</h1>
    </div>
  );
}
