import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";

import { AppComponent } from "./app.component";
import {HomeComponent} from "./pages/home/home.component";
import {NewConfigComponent} from "./pages/new-config/new-config.component";
import {BrowserAnimationsModule} from "@angular/platform-browser/animations";
import {AppRoutingModule} from "./app-routing.module";

@NgModule({
  declarations: [
      AppComponent,
      HomeComponent,
      NewConfigComponent
  ],
  imports: [
      BrowserModule,
      BrowserAnimationsModule,

      AppRoutingModule
  ],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
