import { Component, OnInit, OnDestroy, Input } from '@angular/core';
import { Location, LocationStrategy, PathLocationStrategy } from '@angular/common';

import { HostListener } from '@angular/core';

import { IClockDialect } from './clock-dialect.type';
import { ClockDialectBern } from './clock-dialect.ch-bern';

interface GridElement {
  value: string;
  index: number;
};

@Component({
  selector: 'app-clock',
  providers: [Location, {provide: LocationStrategy, useClass: PathLocationStrategy}],
  templateUrl: './clock.component.html',
  styleUrls: ['./clock.component.scss']
})
export class ClockComponent implements OnInit, OnDestroy {
    @Input() dialectName: string;

  // dynamic styling at
  // https://ultimatecourses.com/blog/using-ngstyle-in-angular-for-dynamic-styling

  private cellState: Boolean[] = [
    false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false,
  ];

  public grid: GridElement[];  // values to manipulate the presentation
  private interval;  // the interval used by the timer.
  private dialect: IClockDialect;  // which dialect shall be used

  constructor(
    private location: Location,
  ) {}

  initGrid() {
    this.grid = [];  // reset to an empty array
    for (const index in this.dialect.GRID) {
      const element: GridElement = {
        value:  this.dialect.GRID[index], index: Number(index)
      };
      this.grid.push(element);
    }
  }

  public getColor(element: number): String {
    if (this.cellState[element]) {
      return 'green';
    } else {
      return '#303030';
    }
  }

  ngOnInit() {
    console.log("onInit");
    this.dialect = new ClockDialectBern();
    this.initGrid();
    this.updateTime();
    this.interval = setInterval( () => this.updateTime(), 60000);
  }

  updateTime(hour?: number, minute?: number) {
    if (hour === undefined ) {
      const date = new Date();
      minute = date.getMinutes();
      hour = date.getHours() % 12;
    }
    console.log('New time is ' + hour.toString() + ':' + minute.toString());
    this.resetAllStates();
    this.dialect.mapTimeToGrid(hour, minute, this.setState);
  }

  ngOnDestroy() {
    if (this.interval) {this.interval.clearInterval();}
  }

  private setState = (start: number, len: number, state: boolean = true) =>  {
    const end = start + len;
    for (let i = start; i < end; i++ ) {
      this.cellState[i] = state;
    }

  }

  private resetAllStates() {
    this.setState(0, this.cellState.length, false);
  }

  @HostListener('document:keyup', ['$event'])
  @HostListener('document:click', ['$event'])
  @HostListener('document:wheel', ['$event'])
  onIdleFinish() {
    this.location.back();  // go back to previous page
  }

}
