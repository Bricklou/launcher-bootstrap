import { useEffect, useState } from "react";
import { useLocation, useNavigation } from "react-router-dom";
import { downloadConfig, RemoteConfig } from "../utils/config";

export function NewConfigPage(): JSX.Element {
  const location = useLocation();
  const [config, setConfig] = useState<RemoteConfig | null>(null);
  const [error, setError] = useState<string | null>(null);

  const fetchConfig = async (url: string) => {
    try {
      const config = await downloadConfig(url);
      setConfig(config);
      console.log(config);
    } catch (e) {
      console.error(e);
      if (e instanceof Error) {
        setError(e.message);
      } else {
        setError("Unknown error");
      }
    }
  };

  useEffect(() => {
    console.log("New config page", location.state.config);
    void fetchConfig(location.state.config);
  }, []);

  return (
    <div>
      <h1>New config</h1>
    </div>
  );
}
