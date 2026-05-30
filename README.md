# Endtime
<sup>a time system for the end of the world.</sup>

## How does it work?

Endtime is a dozenal<sub>(base 12)</sub> date and time system epoched at 2012-12-21 00:00:00 UTC. 

The date and time in Endtime consists of a single rational (base 12) number where 1.0 is equal to one day, so that the dozenal point `.` distinguishes between the date and time components.

This table shows how the day is subdivided:

| Endtime Duration | Standard Duration |
|------------------|-------------------|
| 0.1              | 2 hours           |
| 0.01             | 10 minutes        |
| 0.001            | 50 seconds        |
| 0.0001           | 4&frac16; seconds |
| 0.00001          | 347.2222... ms    |

For the eqivalent of an HH:MM clock, 3 digits are used giving resolution down to 50s. For the equivalent of HH:MM:SS clocks, either 4 or 5 digits are used giving a resolution of ~4s or ~&frac13;s respectivly.

## Why?

I really like the design elegance of the dozenal system for keeping time, since it allows the time to be expressed in one uniform number.

The date aspect was added mostly as a meme since as I was working out the subdivisions I found myself treating 1.0 days as my base unit. The choice of epoch was 100% because I found it funny rather than because it's a good or practical choice.

The single biggest functionality flaw of this system is (imo) that 365.25 (base 10) days is 265.3 in endtime, which doesn't map nicely to any digit place, so it lacks an at-a-glance measure of the current solar year (or lunar year for that matter).
