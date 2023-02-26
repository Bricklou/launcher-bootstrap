import { useEffect, useState } from "react";
import { useLocation } from "react-router-dom";
import {
  downloadConfig,
  createConfig,
  RemoteConfig,
  checkIfConfigExists,
} from "../utils/config";
import style from "../styles/pages/new-config.module.css";
import { Button } from "../components/button/Button";
import { Modal } from "../components/modal/Modal";
import { Plus } from "lucide-react";

export function NewConfigPage(): JSX.Element {
  const location = useLocation();
  const url = location.state.config;
  const [config, setConfig] = useState<RemoteConfig | null>(null);
  const [exists, setExists] = useState<boolean>(false);
  const [overrideConfirm, setOverrideConfirm] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const fetchConfig = async () => {
    try {
      setExists(await checkIfConfigExists(url));

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

            {exists && (
              <div className={style.exists}>
                <div>
                  <p>
                    <span className="text-secondary font-semibold">
                      {config.server_name}
                    </span>{" "}
                    launcher already exists. Clicking the{" "}
                    <span className="kbd kbd-sm">Override</span> button will
                    override the existing launcher.
                  </p>
                </div>
              </div>
            )}
          </section>
          <footer>
            <Button
              icon={<Plus />}
              onClick={exists ? () => setOverrideConfirm(true) : saveConfig}
            >
              {exists ? "Override" : "Create"}
            </Button>

            <Modal
              title="Are you sure ?"
              show={overrideConfirm}
              onClose={() => setOverrideConfirm(false)}
              actions={[
                {
                  label: "Cancel",
                  onClick: () => {},
                  btnTypeClass: "btn-outline",
                },
                {
                  label: "Override",
                  onClick: saveConfig,
                  btnTypeClass: "btn-outline btn-error",
                },
              ]}
            >
              <p>
                By doing this, you will override the current configuration for
                this server, are you sure to continue ?
              </p>
            </Modal>
          </footer>
        </>
      )}
    </div>
  );
}
