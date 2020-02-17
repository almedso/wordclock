import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Routes, RouterModule } from '@angular/router';
import { ClockComponent } from './clock.component';
import { ClockRouterComponent } from './clock-router.component';

const routes: Routes = [
  {
    path: ':dialect',
    component: ClockRouterComponent,
  },
];

@NgModule({
  declarations: [
    ClockComponent,
    ClockRouterComponent,
  ],
  imports: [
    CommonModule,
    RouterModule.forChild(routes),
  ],
  exports: [
    RouterModule,
    ClockComponent,
    ClockRouterComponent,
  ]
})
export class ClockModule { }
