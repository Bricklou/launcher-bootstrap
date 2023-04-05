import {NgModule} from '@angular/core'
import {RouterModule, Routes} from '@angular/router'
import {HomeComponent} from "./pages/home/home.component";
import {NewConfigComponent} from "./pages/new-config/new-config.component";

const routes: Routes = [
    {
        path: '',
        component: HomeComponent,
    },
    {
        path: 'new-config',
        component: NewConfigComponent,
    }
]

@NgModule({
    imports: [
        RouterModule.forRoot(routes, {
            initialNavigation: 'enabledBlocking',
        })
    ],
    exports: [RouterModule],
    providers: []
})
export class AppRoutingModule {
}