import { NgModule } from '@angular/core'
import { RouterModule, type Routes } from '@angular/router'
import { HomeComponent } from './pages/home/home.component'
import { NewConfigComponent } from './pages/new-config/new-config.component'
import { RoutingDebugService } from './services/routing-debug.service'

const routes: Routes = [
  {
    path: '',
    component: HomeComponent
  },
  {
    path: 'new-config',
    component: NewConfigComponent
  }
]

@NgModule({
  imports: [
    RouterModule.forRoot(routes, {
      initialNavigation: 'enabledBlocking'
    })
  ],
  exports: [RouterModule],
  providers: [RoutingDebugService]
})
export class AppRoutingModule {
  public constructor (private readonly routingDebug: RoutingDebugService) {}
}
