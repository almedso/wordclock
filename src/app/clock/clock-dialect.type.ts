export interface IClockDialect {
    GRID: string[];
    mapTimeToGrid(
        hour: number,
        minute: number,
        setStateFunction: (hour: number, minute: number) => void
    );
}

export enum ClockValue {
    // hours,
    zero = 0,  // not really needed
    one = 1,
    two = 2,
    three = 3,
    four = 4,
    five = 5,
    six = 6,
    seven = 7,
    eight = 8,
    nine = 9,
    ten = 10,
    eleven = 11,
    twelve = 12,
    // more about numbers
    full_clock, // full hour
    half,
    quarter,
    one_m,
    two_m,
    three_m,
    four_m,
    five_m,
    ten_m,
    twenty,
    to,  // directions
    past,
    it, // filler for full hour
    is, // same here
  };

