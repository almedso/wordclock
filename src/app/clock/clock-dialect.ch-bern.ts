import { IClockDialect, ClockValue } from "./clock-dialect.type";

export class ClockDialectBern implements IClockDialect {

    // the grid is 11 x 11
    public GRID = [
        'E', 'S', 'K', 'I', 'S', 'C', 'H', 'A', 'F', 'Ü', 'F',
        'V', 'I', 'E', 'R', 'T', 'U', 'B', 'F', 'Z', 'Ä', 'Ä',
        'Z', 'W', 'Ä', 'N', 'Z', 'G', 'S', 'I', 'V', 'O', 'R',
        'A', 'B', 'O', 'H', 'A', 'U', 'B', 'I', 'E', 'P', 'M',
        'E', 'I', 'S', 'Z', 'W', 'O', 'I', 'S', 'D', 'R', 'Ü',
        'V', 'I', 'E', 'R', 'F', 'Ü', 'N', 'F', 'I', 'Q', 'T',
        'S', 'E', 'C', 'H', 'S', 'I', 'S', 'I', 'B', 'N', 'I',
        'A', 'C', 'H', 'T', 'I', 'N', 'Ü', 'N', 'I', 'E', 'L',
        'Z', 'Ä', 'N', 'I', 'E', 'R', 'B', 'E', 'U', 'F', 'I',
        'Z', 'W', 'Ö', 'U', 'F', 'I', 'A', 'M', 'U', 'H', 'R',
        ' ', ' ', ' ', '*', '*', '*', '*', ' ', ' ', ' ', ' ',
      ];

    private setStateFunction = ( hour: number, minute: number): void  => {};

    mapTimeToGrid(
        hour: number,
        minute: number,
        setStateFunction: ( hour: number, minute: number) => void,
    ): void {
        this.setStateFunction = setStateFunction;
        let clockValues: ClockValue[] = this.mapClock(hour, minute);
        clockValues.forEach( value => this.setFields(value));
    }

    private setFields(clockValue: ClockValue) {

        const FIELD_MAP: Record<number, [number,number]> = {
          [ClockValue.zero]: [0, 0],
          [ClockValue.one]: [4 * 11 + 0, 3],
          [ClockValue.two]: [4 * 11 + 3, 4],
          [ClockValue.three]: [4 * 11 + 8, 3],
          [ClockValue.four]: [5 * 11 + 0, 4],
          [ClockValue.five]: [5 * 11 + 4, 5],
          [ClockValue.six]: [6 * 11 + 0, 6],
          [ClockValue.seven]: [6 * 11 + 6, 5],
          [ClockValue.eight]: [7 * 11 + 0, 5],
          [ClockValue.nine]: [7 * 11 + 5, 4],
          [ClockValue.ten]: [8 * 11 + 0, 4],
          [ClockValue.eleven]: [8 * 11 + 7, 4],
          [ClockValue.twelve]: [9 * 11 + 0, 6],
          [ClockValue.full_clock]: [9 * 11 + 8, 3],
          [ClockValue.half]: [3 * 11 + 3, 5],
          [ClockValue.five_m]: [0 * 11 + 8, 3],
          [ClockValue.ten_m]: [1 * 11 + 8, 3],
          [ClockValue.quarter]: [1 * 11 + 0, 6],
          [ClockValue.twenty]: [2 * 11 + 0, 6],
          [ClockValue.to]: [2 * 11 + 8, 3],
          [ClockValue.past]: [3 * 11 + 0, 2],
          [ClockValue.it]: [0 * 11 + 0, 2],
          [ClockValue.is]: [0 * 11 + 3, 4],
          [ClockValue.one_m]: [10 * 11 + 3, 1],
          [ClockValue.two_m]: [10 * 11 + 3, 2],
          [ClockValue.three_m]: [10 * 11 + 3, 3],
          [ClockValue.four_m]: [10 * 11 + 3, 4],
        };
        this.setStateFunction(FIELD_MAP[clockValue][0],FIELD_MAP[clockValue][1]);
    }

    private mapClock(hour: number, minute: number): ClockValue[] {
        let retArray : ClockValue[] = [];

        const modus_5 = minute % 5;
        minute = minute - modus_5;

        // handle the minutes below 5
        switch (modus_5) {
          case 1:
            retArray.push(ClockValue.one_m);
            break;
          case 2:
            retArray.push(ClockValue.two_m);
            break;
          case 3:
            retArray.push(ClockValue.three_m);
            break;
          case 4:
            retArray.push(ClockValue.four_m);
            break;
        }

        // handle the minutes
        switch (minute) {
          case 0:
            retArray.push(ClockValue.it);
            retArray.push(ClockValue.is);
            retArray.push(ClockValue.full_clock);
            break;
          case 5:
            retArray.push(ClockValue.five_m);
            retArray.push(ClockValue.past);
            break;
          case 10:
            retArray.push(ClockValue.ten_m);
            retArray.push(ClockValue.past);
            break;
          case 15:
            retArray.push(ClockValue.quarter);
            retArray.push(ClockValue.past);
            break;
          case 20:
            retArray.push(ClockValue.twenty);
            retArray.push(ClockValue.past);
            break;
          case 25:
            retArray.push(ClockValue.five_m);
            retArray.push(ClockValue.to);
            retArray.push(ClockValue.half);
            hour += 1;
            break;
          case 30:
            retArray.push(ClockValue.it);
            retArray.push(ClockValue.is);
            retArray.push(ClockValue.half);
            hour += 1;
            break;
          case 35:
            retArray.push(ClockValue.five_m);
            retArray.push(ClockValue.past);
            retArray.push(ClockValue.half);
            hour += 1;
            break;
          case 40:
            retArray.push(ClockValue.twenty);
            retArray.push(ClockValue.to);
            hour += 1;
            break;
          case 45:
            retArray.push(ClockValue.quarter);
            retArray.push(ClockValue.to);
            hour += 1;
            break;
          case 50:
            retArray.push(ClockValue.ten_m);
            retArray.push(ClockValue.to);
            hour += 1;
            break;
          case 55:
            retArray.push(ClockValue.five_m);
            retArray.push(ClockValue.to);
            hour += 1;
            break;
          }

        // handle the hour
        hour = hour % 12;
        if ( hour === 0 ) { hour = 12; }
        const value: ClockValue = hour as ClockValue;
        retArray.push(value);

        return retArray;
      }

}
