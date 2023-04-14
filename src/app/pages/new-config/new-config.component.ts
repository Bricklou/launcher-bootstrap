import { Component, OnDestroy } from '@angular/core'
import { ActivatedRoute } from '@angular/router'
import { Subscription, forkJoin, map, switchMap, toArray } from 'rxjs'
import { TauriApiService } from 'src/app/services/tauri-api.service'
import { RemoteConfig } from 'src/app/types/config'

@Component({
  selector: 'app-new-config',
  templateUrl: './new-config.component.html',
  styleUrls: ['./new-config.component.css']
})
export class NewConfigComponent implements OnDestroy {
  protected config?: RemoteConfig
  protected alreadyExists = false

  private configSub: Subscription

  public constructor (private tauriService: TauriApiService, private activeRoute: ActivatedRoute) {
    this.configSub = activeRoute.queryParamMap
      .pipe(
        map(params => params.get('config')),
        switchMap(configName => {
          if (!configName) {
            throw new Error('Config url not provided')
          }
          return configName
        }),
        map(configName => forkJoin([
          // tauriService.getConfig(configName),
          // tauriService.fetchConfig(configName)
          tauriService.getOrFetchConfig(configName)
        ]).pipe(map(([config, remoteConfig]) => ({
          localConfig: config,
          remoteConfig
        })))),
        map(values => values)
      )
      .subscribe({
        next: config => {
          console.log('Config', config)
          this.alreadyExists = config != null
        },

        error: err => {
          console.error(err)
        }
      })
  }

  public ngOnDestroy (): void {
    this.configSub.unsubscribe()
  }
}
