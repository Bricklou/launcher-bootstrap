import { useEffect } from "react";
import { useEventListener } from "../events";

interface GlobalEventHandlerProps {
  children: JSX.Element;
}

enum GlobalEvent {
  LinkEvent = "link-event",
}

interface ILinkEvent {
  event_type: GlobalEvent.LinkEvent;
  data: string;
}

export function GlobalEventHandler(
  props: GlobalEventHandlerProps
): JSX.Element {
  const { listen, unlisten } = useEventListener<ILinkEvent>(
    GlobalEvent.LinkEvent
  );

  useEffect(() => {
    listen((event) => {
      console.log(event);
    });

    return () => {
      unlisten();
    };
  }, []);

  return props.children;
}
