import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-clock-route',
  templateUrl: './clock-router.component.html',
  styleUrls: ['./clock-router.component.scss']
})
export class ClockRouterComponent implements OnInit {

  dialect: string;

  constructor(
    private route: ActivatedRoute,
  ) {}

  ngOnInit() {
    let dialect = this.route.snapshot.paramMap.get('dialect');
    // do something more if more dialects are there
    // fall back to default
    dialect = 'ch-bern';
  }

}
