import { Injectable, OnDestroy } from '@angular/core'
import { NavigationEnd, Router } from '@angular/router'
import { Subscription } from 'rxjs'

@Injectable({
  providedIn: 'root'
})
export class RoutingDebugService implements OnDestroy {
  private routerSubscription: Subscription

  public constructor (private router: Router) {
    this.routerSubscription = this.router.events.subscribe((event) => {
      if (event instanceof NavigationEnd) {
        console.info('Route changed to: %s', event.urlAfterRedirects)
      }
    })
  }

  public ngOnDestroy (): void {
    this.routerSubscription.unsubscribe()
  }
}
