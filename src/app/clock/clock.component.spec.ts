import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { ClockComponent } from './clock.component';
import { ActivatedRoute } from '@angular/router';
import { Observable, of } from 'rxjs'
import { Location } from '@angular/common';


describe('ClockComponent', () => {
  let component: ClockComponent;
  let fixture: ComponentFixture<ClockComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ ClockComponent ],
      providers: [
        { provide: ActivatedRoute, useValue: {
          params: of( { dialect: 'ch-bern' } ),
          queryParams: of( {config: 'foo' } ),
          snapshot: {
            paramMap: {
              get: () => 1, // represents the bookId
            },
          },
          }
        },
        { provide: Location, useValue: {
            back: () => {},
          }
        },
      ],
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ClockComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
