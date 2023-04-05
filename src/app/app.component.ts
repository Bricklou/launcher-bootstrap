import {Component, OnDestroy, OnInit} from "@angular/core";
import {listen, UnlistenFn, Event} from "@tauri-apps/api/event";
import {Router} from "@angular/router";

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

@Component({
    selector: "app-root",
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.css"],
})
export class AppComponent implements OnInit, OnDestroy {
    private unlistenEvent: UnlistenFn | null = null;

    public constructor(private router: Router) {
    }

    ngOnDestroy(): void {
        this.unlistenEvent?.();
        this.unlistenEvent = null;
    }

    async ngOnInit(): Promise<void> {
        if (this.unlistenEvent) return;

        this.unlistenEvent = await listen<ILinkEvent>(GlobalEvent.LinkEvent, this.linkEventHandler);
    }

    private async linkEventHandler(event: Event<ILinkEvent>) {
        const payload = event.payload;

        switch (payload.event_type) {
            case LinkEventType.OpenConfig: {
                await this.router.navigate(["/open-config"], {
                    queryParams: {
                        config: payload.data,
                    }
                });
                break;
            }
            case LinkEventType.NewConfig: {
                console.log("New config");
                await this.router.navigate(["/new-config"], {
                    queryParams: {
                        config: payload.data,
                    }
                });
                break;
            }
        }
    }
}
