import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { ClockModule } from './clock/clock.module';
import { AppComponent } from './app.component';

@NgModule({
  declarations: [
    AppComponent,
  ],
  imports: [
    BrowserModule,
    ClockModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
