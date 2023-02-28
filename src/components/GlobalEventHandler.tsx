import { useEffect } from "react";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { useNavigate } from "react-router-dom";

interface GlobalEventHandlerProps {
  children: JSX.Element;
}

enum GlobalEvent {
  LinkEvent = "link-event",
}

enum LinkEventType {
  OpenConfig = "open-config",
  NewConfig = "new-config",
}

interface ILinkEvent {
  event_type: LinkEventType;
  data: string;
}

export function GlobalEventHandler(
  props: GlobalEventHandlerProps
): JSX.Element {
  const navigate = useNavigate();

  let unlistenEvent: UnlistenFn | null = null;

  useEffect(() => {
    (async () => {
      if (!unlistenEvent) {
        unlistenEvent = await listen<ILinkEvent>(
          GlobalEvent.LinkEvent,
          (event) => {
            const payload = event.payload;
            switch (payload.event_type) {
              case LinkEventType.OpenConfig:
                {
                  navigate("/open-config", {
                    state: { config: payload.data },
                    replace: true,
                  });
                }
                break;
              case LinkEventType.NewConfig:
                {
                  console.trace("new config: %s", payload.data);
                  navigate("/new-config", {
                    state: { config: payload.data },
                    replace: true,
                  });
                }
                break;
            }
          }
        );
      }
    })();

    return () => {
      if (unlistenEvent) {
        void unlistenEvent();
      }
    };
  }, []);

  return props.children;
}
