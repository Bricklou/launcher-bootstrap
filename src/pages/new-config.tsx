import { useEffect, useState } from "react";
import { useLocation } from "react-router-dom";
import { downloadConfig, createConfig, RemoteConfig } from "../utils/config";
import style from "../styles/pages/new-config.module.css";
import { Button } from "../components/button/Button";
import { Plus } from "lucide-react";

export function NewConfigPage(): JSX.Element {
  const location = useLocation();
  const url = location.state.config;
  const [config, setConfig] = useState<RemoteConfig | null>(null);
  const [error, setError] = useState<string | null>(null);

  const fetchConfig = async () => {
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

  const saveConfig = async () => {
    console.trace("Save config: %O", config);
    if (config === null) return;

    try {
      await createConfig(url, config);
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
    void fetchConfig();
  }, []);

  return (
    <div className={style.newConfig}>
      <h1 className={style.title}>New config</h1>
      {config !== null && (
        <>
          <section>
            <h2>
              Install the{" "}
              <span className="text-secondary font-semibold">
                {config.server_name}
              </span>{" "}
              launcher?
            </h2>

            <p>
              After clicking the <span className="kbd kbd-sm">Create</span>{" "}
              button, the bootstrap will install the launcher.
            </p>
            <p>If this action doesn't come from you, close the window.</p>
          </section>
          <footer>
            <Button icon={<Plus />} onClick={saveConfig}>
              Create
            </Button>
          </footer>
        </>
      )}
    </div>
  );
}
