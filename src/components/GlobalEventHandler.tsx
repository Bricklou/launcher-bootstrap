import { useEffect } from "react";
import { listen, UnlistenFn, Event } from "@tauri-apps/api/event";
import { useNavigate, useNavigation } from "react-router-dom";

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
        unlistenEvent = await listen<ILinkEvent>(GlobalEvent.LinkEvent, (event)=> {
          const payload = event.payload;
          switch (payload.event_type) {
            case LinkEventType.OpenConfig: {
              navigate("/open-config", { state: { config: payload.data } });
            }
            case LinkEventType.NewConfig: {
              navigate("/new-config", { state: { config: payload.data } });
            }
          }
        });
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
