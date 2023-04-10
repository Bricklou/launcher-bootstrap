import { Injectable, type OnDestroy } from '@angular/core'
import { Router } from '@angular/router'
import { Event, listen, type UnlistenFn } from '@tauri-apps/api/event'

export enum GlobalEvent {
  LinkEvent = 'link-event'
}

export enum LinkEventType {
  OpenConfig = 'open-config',
  NewConfig = 'new-config'
}

interface ILinkEvent {
  event_type: LinkEventType
  data: string
}

@Injectable({
  providedIn: 'root'
})
export class TauriApiService implements OnDestroy {
  private unlistenEvent?: UnlistenFn

  public constructor (private readonly router: Router) {
    this.init()
  }

  private async init () {
    this.unlistenEvent = await listen<ILinkEvent>(
      GlobalEvent.LinkEvent,
      (event) => this.linkEventHandler(event)
    )
  }

  public ngOnDestroy (): void {
    this.unlistenEvent?.()
    this.unlistenEvent = undefined
  }

  private async linkEventHandler (event: Event<ILinkEvent>) {
    const { payload } = event

    switch (payload.event_type) {
      case LinkEventType.OpenConfig: {
        await this.router.navigate(['open-config'], {
          queryParams: {
            config: payload.data
          }
        })
        break
      }

      case LinkEventType.NewConfig: {
        console.log('New config')
        await this.router.navigate(['new-config'], {
          queryParams: {
            config: payload.data
          }
        })
        break
      }

      default:
        break
    }
  }
}
