# Libra
Libra is a work-in-progress quantitative finance library written in Rust, inspired by [QuantLib](https://github.com/lballabio/quantlib).

## Features

### Time
Calendars and dates are based on QuantLib, aiming for parity of holidays.
- [ ] **Calendars:**
    - [ ] Implement standard market calendars.
      - [x] UK
      - [ ] US
      - [ ] EU
    - [x] Implement holiday and business day logic.
    - [ ] Allow for custom calendar definitions.
- [ ] **Day Conventions:**
    - [x] Implement common day count conventions (e.g., Actual/360, 30/360, Actual/Actual).
    - [ ] Provide functionality for calculating day fractions between dates.
- [ ] **Dates:**
    - [x] Implement a robust Date struct with support for date arithmetic.
    - [ ] Provide parsing and formatting of dates.
- [ ] **Periods:**
    - [x] Implement a Period struct for representing time intervals (e.g., days, weeks, months, years).
    - [x] Support arithmetic operations between Periods and Dates.
- [ ] **Schedules:**
    - [ ] Implement functionality for generating payment schedules based on start/end dates, frequency, and business day conventions.

### Instruments
- [ ] **Bonds:**
    - [ ] Implement representation of various bond types.
      - [ ] Zero-Coupon
      - [ ] Fixed-Rate
      - [ ] Floating-Rate
    - [ ] Implement yield calculation.
    - [ ] Implement present value calculation.
- [ ] **Forward Rate Agreements (FRAs):**
    - [ ] Implement FRA contract representation.
    - [ ] Implement FRA pricing.
- [ ] **Swaps:**
    - [ ] Implement interest rate swap representation (e.g., fixed-for-floating).
    - [ ] Implement basic swap pricing.
- [ ] **Options:**
    - [ ] Implement basic option contracts.
      - [ ] European
      - [ ] American
    - [ ] Implement payoff functions.
- [ ] **Futures:**
    - [ ] Implement futures contract representation.
    - [ ] Implement basic futures pricing.

### Pricing
- [ ] **Discounting:**
    - [ ] Implement discount factor curves.
    - [ ] Implement zero-coupon bond pricing.
- [ ] **Yield Curve Construction:**
    - [ ] Implement basic yield curve bootstrapping methods.
- [ ] **Option Pricing:**
    - [ ] Implement the Black-Scholes for European options.
    - [ ] Explore implementation of other pricing models.
