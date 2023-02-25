import { useEffect, useState } from "react";
import { useLocation, useNavigation } from "react-router-dom";
import { downloadConfig, RemoteConfig } from "../utils/config";
import style from "../styles/pages/new-config.module.css";
import { Button } from "../components/button/Button";
import { Plus } from "lucide-react";

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
            <Button icon={<Plus />}>Create</Button>
          </footer>
        </>
      )}
    </div>
  );
}
