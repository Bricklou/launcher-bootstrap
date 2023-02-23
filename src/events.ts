import { useMemo, useState } from "react";
import { emit, EventCallback, listen, UnlistenFn } from "@tauri-apps/api/event";

export function useEventEmitter<P>(name: string) {
  return {
    emit: async (data: P) => {
      await emit(name, data);
    },
  };
}

export function useEventListener<R>(name: string) {
  const [unlistenFn, setUnlistenFn] = useState<UnlistenFn | null>(null);

  const _listen = async (callback: EventCallback<R>) => {
    const unlisten = await listen(name, callback);
    setUnlistenFn(unlisten);
  };

  const _unlisten = () => {
    if (unlistenFn) {
      unlistenFn();
      setUnlistenFn(null);
    }
  };

  return useMemo(
    () => ({
      listen: _listen,
      unlisten: _unlisten,
    }),
    [unlistenFn]
  );
}
