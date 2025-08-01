//! #  SPICE Time Subsystem
//!
//!  Last revised on 2021 DEC 23 by B.V. Semenov
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  SPICE Time Subsystem provides routines for convertting times between
//!    several time systems and common time formats.
//!
//!  
//!
//!
//!  
//! ##  References
//!
//!  The formulation and the values used in this document are taken from the
//!    following sources:
//!
//!  
//!
//! ```text
//!       [1] Moyer, T.D., "Transformation from Proper Time on Earth to
//!           Coordinate Time in Solar System Barycentric Space-Time Frame
//!           of Reference, Parts 1 and 2," Celestial Mechanics 23 (1981),
//!           33-56 and 57-68.
//!  
//!       [2] Moyer, T.D., Effects of Conversion to the J2000  Astronomical
//!           Reference System on Algorithms for Computing Time Differences
//!           and Clock Rates, JPL IOM 314.5--942, 1 October 1985.
//!  
//!       [3] The Explanatory Supplement to the Astronomical Almanac (1992)
//!           Edited by P. Kenneth Seidelmann, University Science Books,
//!           Mill Valley, California 94941.
//!  
//!       [4] SCLK Required Reading (sclk.req)
//!  
//!       [5] Kernel Required Reading (kernel.req)
//!  
//!       [6] James Jespersen and Jane Fitz-Randolph ``From Sundials to
//!           Atomic Clocks---Understanding Time and Frequency''
//!           (Dover Publications, Inc. 1977) ISBN 0-486-24265-X.
//!  
//!       [7] Standish, E. M., Astron. Astrophys., "Time Scales in the JPL
//!           and CfA Ephemerides", 336, 381-384 (1998).
//!  
//!       [8] SPICE Time Subsystem Tutorial,
//!           (https://naif.jpl.nasa.gov/naif/tutorials.html).
//!  
//!       [9] Most Used SPICE APIs document (mostused.html).
//! ```
//!
//!  The variable names used are consistent with notations used in the
//!    Astronomical Almanac.
//!
//!  
//!
//!
//!  
//! ##  Introduction
//!
//!  This document describes the software available in the SPICE Toolkit for
//!    manipulating various representations of time. It is your main source for
//!    general information about calendar based and continuous time systems in
//!    SPICE. For specifics of a particular routine you should consult the
//!    header of that routine.
//!
//!  The Toolkit also supports conversion between spacecraft clock (SCLK) and
//!    Barycentric Dynamical Time (TDB). SPICE routines dealing with spacecraft
//!    clock are discussed in SCLK Required Reading ([sclk.req](crate::required_reading::sclk)).
//!
//!  
//!
//!
//!  
//! ###  Intended Audience
//!
//!  This document is intended for all SPICE users.
//!
//!  
//!
//!
//!  
//! ##  Overview
//!
//!  The SPICE system contains a versatile set of time conversion routines
//!    designed to simplify conversions between several time systems. In
//!    addition, most common time formats are supported including: calendar,
//!    day of year, and Julian Date.
//!
//!  
//!
//!
//!  
//! ##  SPICE Time Representations
//!
//!  The Toolkit directly supports three time representations. They are
//!
//!  
//!
//! *  1. Coordinated Universal Time (UTC) representation (not a time system, only a
//! representation)
//!
//!  *  2. Barycentric Dynamical Time (TDB) time system, also referred to as Ephemeris
//! Time (ET)
//!
//!  *  3. Spacecraft Clock Time (SCLK---pronounced "ess clock") time system
//!
//!  UTC is not a time system, rather a representation of a time. But it is
//!    often considered a time system.
//!
//!  The Toolkit supports these time systems
//!
//!  
//!
//! *  1. Barycentric Dynamical Time (TDB)
//!
//!  *  2. Terrestrial Time (TT or TDT)
//!
//!  *  3. International Atomic Time (TAI)
//!
//!  *  4. Global Position System Time (GPS)
//!
//!  Formerly, Terrestrial Time (TT) was referred to as Terrestrial Dynamical
//!    Time (TDT). In SPICE, both terms refer to the same time system.
//!
//!  
//!
//!
//!  
//! ##  Coordinated Universal Time (UTC)
//!
//!  
//!
//!
//!  
//! ###  International Atomic Time (TAI)
//!
//!  Before discussing Coordinated Universal Time we feel it is helpful to
//!    talk about International Atomic Time (TAI or atomic time). Atomic time
//!    is based upon the atomic second as defined by the "oscillation of the
//!    undisturbed cesium atom." Atomic time is simply a count of atomic
//!    seconds that have occurred since the astronomically determined instant
//!    of midnight January 1, 1958 00:00:00 at the Royal Observatory in
//!    Greenwich, England. Atomic time is kept by the International Earth
//!    Rotation Service (IERS, formally the Bureau International L'Heure---BIH)
//!    in Paris, France. The National Bureau of Standards and the U.S. Naval
//!    Observatory set their clocks by the clock maintained by the IERS.
//!
//!  
//!
//!
//!  
//! ###  Naming the seconds of TAI --- UTC
//!
//!  Coordinated Universal Time is a method of time keeping that gives a name
//!    to each instant of time of the TAI system. These names are formed from
//!    the calendar date and time of day that we use in our daily affairs. They
//!    consist of 6 components: year, month, day, hour, minutes and seconds.
//!    The year, month and day components are the normal calendar year month
//!    and day that appear on wall calendars. The hours component may assume
//!    any value from 0 through 23. The minutes component may assume any value
//!    from 0 to 59. The seconds will usually (but not always) range from 0 to
//!    59.999... . The hour-minute-second string
//!
//!  
//!
//! ```text
//!    00:00:00
//! ```
//!
//!  is midnight and is the first instant of the calendar day specified by
//!    the first three components of the UTC time.
//!
//!  In the SPICE system UTC times are represented by character strings.
//!    These strings contain: year, month, day, hour, minute and second
//!    separated by delimiters (spaces or punctuation marks). The various
//!    delimiters and substrings between the delimiters are called the tokens
//!    of the string. A typical time string looks like
//!
//!  
//!
//! ```text
//!    5 OCTOBER 1986 7:20:16.122 (UTC)
//! ```
//!
//!  The tokens of the string and the associated UTC time components are
//!
//!  
//!
//! ```text
//!    5       --- day
//!    OCTOBER --- month
//!    1986    --- year
//!    7       --- hours
//!    20      --- minutes
//!    16.122  --- seconds
//! ```
//!
//!  The link between any token and its corresponding UTC component is
//!    determined by examining the values of the tokens and comparing them to
//!    the other tokens. The precise rules used are spelled out in great detail
//!    in Appendix C. For now, simply be assured that the following five
//!    strings all mean the same thing and are interpreted in the same way by
//!    SPICE.
//!
//!  
//!
//! ```text
//!    5 OCTOBER 1986
//!    1986 OCTOBER 5
//!    1986 5 OCTOBER
//!    1986 10 5
//!    10 5 1986
//! ```
//!
//!     
//! ###  Tying UTC to the Earth's Rotation
//!
//!  The names given to TAI instants by the UTC system are governed by the
//!    earth's rotation. Ideally, UTC strings having hours, minutes and seconds
//!    components all zero should correspond to Greenwich midnight as
//!    determined by the observations of the transits of stars (the time system
//!    known as UT1). However, since the rotation of the earth is not uniform,
//!    this ideal cannot be realized. The difference between Greenwich midnight
//!    observed astronomically and UTC midnight is almost never zero. However,
//!    to keep the difference from becoming too large, UTC is occasionally
//!    adjusted so that the difference between the two midnights never exceeds
//!    .9 seconds. Thus from a knowledge of UTC one can always compute UT1 to
//!    better than 1 second accuracy.
//!
//!  
//!
//!
//!  
//! ###  Leapseconds
//!
//!  When Greenwich UT1 midnight lags behind UTC midnight by more than 0.9
//!    seconds the International Earth Rotation Service will announce that a
//!    leap second will be added to the collection of UTC names. This leap
//!    second has traditionally been added after the last "normal" UTC name
//!    of December 31 or June 30. Thus when a UTC second is added the
//!    hours-minutes-seconds portion of the UTC name progresses as shown here
//!
//!  
//!
//! ```text
//!    ... DECEMBER 31 23:59:57
//!    ... DECEMBER 31 23:59:58
//!    ... DECEMBER 31 23:59:59
//!    ... DECEMBER 31 23:59:60
//!    ... JANUARY   1 00:00:00
//! ```
//!
//!  instead of the usual progression
//!
//!  
//!
//! ```text
//!    ... DECEMBER 31 23:59:57
//!    ... DECEMBER 31 23:59:58
//!    ... DECEMBER 31 23:59:59
//!    ... JANUARY   1 00:00:00
//! ```
//!
//!  Should Greenwich UT1 midnight run ahead of UTC midnight by more than 0.9
//!    seconds the IERS will announce a negative leap second. In this case one
//!    of the usual UTC hours-minutes-seconds triples will be missing from the
//!    list of UTC names. In this case the progression will be:
//!
//!  
//!
//! ```text
//!    ... DECEMBER 31 23:59:57
//!    ... DECEMBER 31 23:59:58
//!    ... JANUARY   1 00:00:00
//! ```
//!
//!  Since 1972 when leap seconds and the UTC system were introduced, a
//!    negative leap second has not occurred.
//!
//!  Leapseconds occur at the same time in all time zones. In other words,
//!    the seconds component of a time string is the same for any time zone as
//!    is the seconds component of UTC. The following are all legitimate ways
//!    to represent an epoch of some event that occurred in the leapsecond
//!
//!  
//!
//! ```text
//!    1995 December 31  23:59:60.5  (UTC)
//!  
//!    1996 January   1, 05:29:60.5  (UTC+5:30 --- Calcutta Time)
//!    1995 December 31, 20:29:60.5  (UTC-3:30 --- Newfoundland)
//!    1995 December 31  18:59:60.5  (EST)
//!    1995 December 31  17:59:60.5  (CST)
//!    1995 December 31  16:59:60.5  (MST)
//!    1995 December 31  15:59:60.5  (PST)
//! ```
//!
//!     
//! ###  The Leapseconds Kernel (LSK)
//!
//!  The primary difficulty with UTC strings is that it is not possible to
//!    predict which atomic times will correspond to times during a UTC leap
//!    second. Thus algorithms for converting between UTC and time systems that
//!    simply use a continuous set of numeric markers require knowledge of the
//!    location of leap seconds in the list of names. This is the purpose of
//!    the Leapseconds Kernel (LSK). To convert between UTC times and any other
//!    system, you must first load the Leapseconds Kernel via a call to the
//!    routine [FURNSH](crate::raw::furnsh).
//!
//!  LSK files conform to a flexible format called "NAIF text kernel"
//!    format. The SPICE file identification word provided by itself on the
//!    first line of an LSK file is "KPL/LSK". Both the NAIF text kernel
//!    format and SPICE file identification word are described in detail in the
//!    Kernel Required Reading document, [kernel.req](crate::required_reading::kernel).
//!
//!  When the IERS announces a new leapsecond will be declared in the future,
//!    NAIF makes available an updated Leapseconds Lernel several months prior
//!    to the new leapsecond taking effect and announces its availability to
//!    the SPICE user community.
//!
//!  
//!
//!
//!  
//! ##  Barycentric Dynamic Time (TDB)
//!
//!  TDB is the uniform time scale represented by the independent variable in
//!    the differential equations that describe the motions of the planets, sun
//!    and moon. There are two forms of ephemeris time: Barycentric Dynamical
//!    Time (TDB) and Terrestrial Time (TT). These time systems are closely
//!    related, as described below.
//!
//!  
//!
//!
//!  
//! ###  The J2000 Epoch
//!
//!  The basic spatial reference system for SPICE is the J2000 system. This
//!    is an inertial reference frame in which the equations of motion for the
//!    solar system may be integrated. This reference frame is specified by the
//!    orientation of the earth's mean equator and equinox at a particular
//!    epoch --- the J2000 epoch. This epoch is Greenwich noon on January 1,
//!    2000 Barycentric Dynamical Time (TDB). Throughout the SPICE
//!    documentation you will see the expressions: "seconds past 2000";
//!    "seconds past J2000"; or "seconds past the J2000 epoch." In all
//!    cases, the reference epoch is noon January 1, 2000 on a particular time
//!    scale.
//!
//!  (As we've seen, "J2000" is used to name the fundamental inertial frame
//!    as well as a particular epoch. This can sometimes be confusing if you
//!    are not careful to distinguish the context in which the term "J2000"
//!    is used.)
//!
//!  
//!
//!
//!  
//! ###  Barycentric Dynamical Time (TDB)
//!
//!  Barycentric Dynamical Time is used when describing the motion of bodies
//!    with respect to the solar system barycenter.
//!
//!  
//!
//!
//!  
//! ###  Terrestrial Time (TT)
//!
//!  Terrestrial Time is used when describing motions of objects near the
//!    earth. As far as measurements have been able to detect, TT and TAI
//!    change at the same rate. Thus the difference between TT and TAI is a
//!    constant. It is defined to be 32.184 seconds. At the zero point of TAI,
//!    TT has a value of 32.184.
//!
//!  
//!
//!
//!  
//! ###  The Relationship between TT and TDB
//!
//!  TDB is believed to be in agreement with the time that would be kept by
//!    an atomic clock located at the solar system barycenter (SSB). A
//!    comparison of the times kept by a clock at the solar system barycenter
//!    with a TDB clock on earth would reveal that the two clocks are in close
//!    agreement but that they run at different rates at different times of the
//!    year. This is due to relativistic effects.
//!
//!  At some times in the year the TT clock appears to run fast when compared
//!    to the TDB clock, at other times of the year it appears to run slow. Let
//!    TDB0 be some fixed epoch on the TDB clock and TT0 be a fixed epoch on
//!    the TT clock (TDB0 and TT0 do not necessarily have to be the same
//!    epoch). Any epoch, EPOCH, can be represented in the following ways: as
//!    the number of seconds TDB(EPOCH), that have elapsed since TDB0 on the
//!    TDB clock; or as the number of seconds, TT(EPOCH), that have elapsed
//!    since TT0 on the TT clock. If we plot the differences TDB(EPOCH) -
//!    TT(EPOCH) against TDB(EPOCH) over all epochs, we will find that the
//!    graph is very close to a periodic function.
//!
//!  In SPICE the difference between TT and TDB is computed as follows:
//!
//!  
//!
//! ```text
//!          TDB - TT =  K * sin (E)                  (1)
//! ```
//!
//!  where K is a constant, and E is the eccentric anomaly of the
//!    heliocentric orbit of the Earth-Moon barycenter. This difference, which
//!    ignores small-period fluctuations, is accurate to about 0.000030
//!    seconds. To five decimal places the difference between TT and TDB is a
//!    periodic function with magnitude approximately 0.001658 seconds and
//!    period equal to one sidereal year.
//!
//!  The eccentric anomaly E is given by
//!
//!  
//!
//! ```text
//!         E = M + EB sin (M)                         (2)
//! ```
//!
//!  where EB and M are the eccentricity and mean anomaly of the heliocentric
//!    orbit of the Earth-Moon barycenter. The mean anomaly is in turn given by
//!
//!  
//!
//! ```text
//!         M = M0 + M1*t                              (3)
//! ```
//!
//!  where t is the epoch TDB expressed in barycentric dynamical seconds past
//!    the epoch of J2000.
//!
//!  The values K, EB, M0, and M1 are retrieved from the kernel pool. These
//!    are part of the Leapseconds Kernel. They correspond to the "kernel pool
//!    variables" [DELTET](crate::raw::deltet)/K, DELTET/EB, and DELTET/M. The nominal values are:
//!
//!  
//!
//! ```text
//!    DELTET/K               =    1.657D-3
//!    DELTET/EB              =    1.671D-2
//!    DELTET/M               = (  6.239996D0   1.99096871D-7 )
//! ```
//!
//!     
//! ###  In the Toolkit ET Means TDB
//!
//!  When ephemeris time is called for by SPICE routines, TDB is the implied
//!    time system. Software that converts between the various time systems
//!    described here use TDB whenever ephemeris time is called for.
//!
//!  Ephemeris time is given in terms of seconds past a reference epoch. The
//!    reference epoch used throughout the Toolkit is the epoch J2000 (roughly
//!    noon on January 1, 2000). Using the Toolkit software, you can find out
//!    how many seconds the J2000 epoch is from right now. SPICE uses a double
//!    precision value for TDB in all Toolkits.
//!
//!  
//!
//!
//!  
//! ##  Naming the Seconds of TDB
//!
//!  Although TDB is a formal time, within the limits of measurements it
//!    coincides with atomic time. As such we should be able to relate it to
//!    the expressions of time that we use everyday.
//!
//!  However, ephemeris time is described as a count of ephemeris seconds
//!    past the ephemeris reference epoch (J2000). For most of us the
//!    expression
//!
//!  
//!
//! ```text
//!    -312819349 seconds past the ephemeris epoch J2000
//! ```
//!
//!  bears little relationship to the time system we use to organize our
//!    lives. For this reason, it is common to give names to the various
//!    ephemeris seconds in a manner analogous to the UTC naming of the seconds
//!    of TAI---as a calendar date and time of day. The above string
//!    corresponds to
//!
//!  
//!
//! ```text
//!    1990 FEB 1 21:44:11 (TDB)
//! ```
//!
//!  There is an important distinction between the names given to ephemeris
//!    seconds and the names used by the UTC system. The names assigned to
//!    ephemeris times never have leap seconds. The 'seconds' component of the
//!    name is restricted to and includes all values from 0 to 59.999... . Thus
//!    the time string above does not represent the same moment in time as does
//!    "1990 FEB 1 21:44:11 (UTC)" There are two reasons. First, ephemeris
//!    time is ahead of atomic time by 32.184 seconds. Second, when a leap
//!    second occurs, UTC strings fit an extra name into the sequence of valid
//!    UTC names. Thus it appears that UTC names fall behind TDB names by a
//!    second after each leapsecond. For instance, as of 2020 DEC 01 UTC time
//!    strings appear to be 69.184 seconds behind TDB time strings. This
//!    difference is due to the fact that the two naming conventions are not
//!    the same; they simply have a lot of names in common.
//!
//!  It is both fortunate and unfortunate that there is a huge set of common
//!    names between calendar dates TDB and calendar dates UTC. Since there are
//!    relatively few leapseconds, a time given by an TDB name is always close
//!    to the time in the UTC system having the same name. Thus for planning
//!    whether or not you are likely to need a coat and how to arrange your
//!    daily activities around the observation. But for precise work you must
//!    pay attention to the difference between the two time systems. If in
//!    planning the observation of a stellar occultation by an asteroid the
//!    difference between the two naming systems is neglected, it is likely
//!    that the observation will be missed.
//!
//!  
//!
//!
//!  
//! ###  Leapseconds
//!
//!  There is no way of predicting when future leapseconds will occur. The
//!    IERS announces the addition of a new leapsecond several months in
//!    advance of its effective date. But beyond this, predictions of
//!    leapseconds are not reliable. As a result we cannot say with certainty
//!    when a particular future UTC epoch will occur. For example, suppose you
//!    have a timer that you can set to "beep" after some number of seconds
//!    have passed. If this timer counts seconds perfectly without loosing or
//!    gaining time over decades, you cannot set it today to beep at midnight
//!    (00:00:00) January 1 (UTC) ten years from now---the number of
//!    leapseconds that will have occurred in the next ten years is not known.
//!    On the other hand, it is possible to set the timer so that it will beep
//!    at midnight January 1 (TDB) since the TDB system does not have
//!    leapseconds. It is only necessary to know an algorithm for converting
//!    calendar epochs TDB to seconds past some reference epoch in order to
//!    determine how to set the timer to beep at the correct epoch.
//!
//!  Any given Leapseconds Kernel will eventually become obsolete. Sometime
//!    after the creation of any Leapseconds Kernel there will be a new
//!    leapsecond that must be accounted for.
//!
//!  When future leapseconds occur the old Leapseconds Kernel will no longer
//!    correctly describe the relationship between UTC, TT and TDB for epochs
//!    that follow the new leapsecond. However, for epochs prior to the new
//!    leapsecond, the old kernel will always correctly describe the
//!    relationship between UTC, TT and TDB.
//!
//!  NAIF announces the addition, or not, of a new leapsecond declared by the
//!    IERS several months in advance of it taking place. Simultaneously NAIF
//!    prepares and announces a new Leapseconds Kernel if one is needed.
//!
//!  
//!
//!
//!  
//! ##  Computing UTC from TDB
//!
//!  Below are a few epochs printed out in calendar format in both the TT and
//!    UTC time systems.
//!
//!  
//!
//! ```text
//!    1996, Oct 11, 12:01:02.1840  (TT)
//!    1996, Oct 11, 12:00:00.0000  (UTC)
//!  
//!    1996, Oct 12, 12:01:02.1840  (TT)
//!    1996, Oct 12, 12:00:00.0000  (UTC)
//!  
//!    1996, Oct 13, 12:01:02.1840  (TT)
//!    1996, Oct 13, 12:00:00.0000  (UTC)
//!  
//!    1996, Oct 14, 12:01:02.1840  (TT)
//!    1996, Oct 14, 12:00:00.0000  (UTC)
//!  
//!    1996, Oct 15, 12:01:02.1840  (TT)
//!    1996, Oct 15, 12:00:00.0000  (UTC)
//! ```
//!
//!  At least in October 1996, it's clear that if you have either TT or UTC
//!    you can construct the corresponding representation for the same epoch in
//!    the UTC or TT system by simply subtracting or adding 62.184 seconds.
//!
//!  If you don't worry about what happens during a leapsecond you can
//!    express the above idea as:
//!
//!  
//!
//! ```text
//!         DeltaTT =  TT - UTC                      (4)
//! ```
//!
//!  For all epochs except during UTC leapseconds the above expression makes
//!    sense. DeltaTT is simply a step function, increasing by one after each
//!    leapsecond. Thus DeltaTT can be viewed as a step function of either UTC
//!    or TT.
//!
//!  If you rearrange this expression, you can get
//!
//!  
//!
//! ```text
//!         UTC = TT - DeltaTT                       (5)
//! ```
//!
//!  Since, TT can be expressed as seconds past J2000 (TT), the above
//!    expression indicates that UTC can be expressed as some count of seconds.
//!    This representation is referred to by the dubious name of "UTC seconds
//!    past J2000." If you write down the UTC calendar time string
//!    corresponding to an epoch and count the number of seconds between that
//!    calendar expression and the UTC calendar expression "January 1, 2000
//!    12:00:00" and ignore leapseconds, you get the value of UTC in the
//!    expression above.
//!
//!  In practice this expression is broken down as follows:
//!
//!  
//!
//! ```text
//!         UTC  =  TT - DeltaTA - DeltaAT            (6)
//! ```
//!
//!  where
//!
//!  
//!
//! ```text
//!         DeltaTA =  (TT - TAI)
//! ```
//!
//!  and
//!
//!  
//!
//! ```text
//!         DeltaAT =  DeltaTT - DeltaTA
//! ```
//!
//!  The value DeltaTA is a constant, its value is nominally 32.184 seconds.
//!    DeltaTA is a step function. These two variables appear in the
//!    Leapseconds Kernel.
//!
//!  If we combine equation (6) above with equation (1) from the section
//!    "The Relationship between TT and TDB" we get the following expression
//!
//!  
//!
//! ```text
//!         TDB - UTC =  DeltaTA + DeltaAT + K*sin(E)  (7)
//! ```
//!
//!  This last value is called DeltaET and is computed by the SPICE routine [DELTET](crate::raw::deltet). The various values that are used in the computation of DeltaET
//!    are contained in the Leapseconds Kernel. Below we show the principal
//!    contents of a sample Leapseconds kernel.
//!
//!  
//!
//! ```text
//!    \begindata
//!  
//!    DELTET/DELTA_T_A       =   32.184
//!    DELTET/K               =    1.657D-3
//!    DELTET/EB              =    1.671D-2
//!    DELTET/M               = (  6.239996D0   1.99096871D-7 )
//!  
//!    DELTET/DELTA_AT        = ( 10,   @1972-JAN-1
//!                               11,   @1972-JUL-1
//!                               12,   @1973-JAN-1
//!                               13,   @1974-JAN-1
//!                               14,   @1975-JAN-1
//!                               15,   @1976-JAN-1
//!                               16,   @1977-JAN-1
//!                               17,   @1978-JAN-1
//!                               18,   @1979-JAN-1
//!                               19,   @1980-JAN-1
//!                               20,   @1981-JUL-1
//!                               21,   @1982-JUL-1
//!                               22,   @1983-JUL-1
//!                               23,   @1985-JUL-1
//!                               24,   @1988-JAN-1  )
//!  
//!    \begintext
//!    DELTET/DELTA_T_A  corresponds to DeltaTA in equation (7).
//!    DELTET/K          corresponds to K in equation (7).
//!    DELTET/EB         corresponds to EB in equation (2).
//!    DELTET/M          corresponds to M0 and M1 of equation (3).
//!    DELTET/DELTA_AT   corresponds to DeltaAT of equation (7).
//!                      Note that this expression gives the
//!                      points on the UTC scale at which
//!                      DeltaAT changes.
//! ```
//!
//!     
//! ###  Problems With the Formulation of DeltaET
//!
//!  As we pointed out above, the expression ( TT - UTC ) is meaningful as
//!    long as you stay away from leapseconds. If you write down the TT and UTC
//!    representations for an epoch that occurs during a leapsecond you will
//!    have something like this:
//!
//!  
//!
//! ```text
//!    1996 Jan 01, 00:01:01.6840  (TT)
//!    1996 Dec 31, 23:59:60.5000  (UTC)
//! ```
//!
//!  Given these two epochs, it is no longer clear what we should assign to
//!    the value TT - UTC. Thus, although equation (7) above provides a simple
//!    expression for computing the "difference between UTC and TDB", the
//!    expression fails to tell us how to convert between TDB (or TT) and UTC
//!    during leapseconds. For this reason the SPICE system does not use
//!    DeltaET when converting between TDB (or TT) and UTC. Instead, the table
//!    of offsets corresponding to DeltaAT in the Leapseconds Kernel is
//!    converted to an equivalent table as shown below.
//!
//!  
//!
//! ```text
//!    Day Number of 1971-DEC-31     TAI seconds past 2000 at
//!                                  beginning of 1971-DEC-31
//!  
//!    Day Number of 1972-JAN-01     TAI seconds past 2000 at
//!                                  beginning of 1972-JAN-01
//!  
//!    Day Number of 1972-JUN-30     TAI seconds past 2000 at
//!                                  beginning of 1972-JUN-30
//!  
//!    Day Number of 1972-JUL-01     TAI seconds past 2000 at
//!                                  beginning of 1972-JUL-01
//!  
//!    Day Number of 1972-DEC-31     TAI seconds past 2000 at
//!                                  beginning of 1972-DEC-31
//!  
//!    Day Number of 1973-JAN-01     TAI seconds past 2000 at
//!                                  beginning of 1973-JAN-01
//!  
//!    Day Number of 1973-DEC-31     TAI seconds past 2000 at
//!                                  beginning of 1973-DEC-31
//!               .                          .
//!               .                          .
//!               .                          .
//! ```
//!
//!  where the day number associated with a particular calendar date is the
//!    integer number of days that have passed since Jan 01, 0001 A.D. (on the
//!    extended Gregorian Calendar).
//!
//!  Given an epoch to be converted between UTC and some other time system
//!    (call this other system 'S'), we decompose the conversion problem into
//!    two parts:
//!
//!  
//!
//! *  1. converting between UTC and TAI
//!
//!  *  2. converting between TAI and system S.
//!
//!  To convert between TAI and UTC, we examine the above table to determine
//!    whether or not the epoch in question falls on a day containing a
//!    leapsecond or during a day that is 86400 seconds in length. Once the
//!    length of the day associated with the epoch has been determined, the
//!    conversion from UTC to TAI (or from TAI to UTC) is straight forward.
//!
//!  Having settled the problem of converting between TAI and UTC, the
//!    conversion between TAI and system S is carried out using the analytic
//!    expressions (equations (1), (2) and (3)) given above.
//!
//!  
//!
//!
//!  
//! ##  Spacecraft Clock (SCLK)
//!
//!  Most spacecraft have at least one onboard clock. This clock controls the
//!    times at which various actions are performed by the spacecraft and its
//!    science instruments. Observations are usually tagged with the spacecraft
//!    clock time when the observations are taken.
//!
//!  Each spacecraft clock can be constructed differently. For Galileo the
//!    SPICE spacecraft clock times looks like
//!
//!  
//!
//! ```text
//!    p/rrrrrrrr:mm:t:e
//!  
//!    p - partition number
//!    r - rim counts
//!    m - minor frame
//!    t - real time interrupt
//!    e - mod eight count
//! ```
//!
//!  When asking for the matrix which describes the pointing for some
//!    structure or instrument used to perform an observation, you will usually
//!    request this information by supplying the spacecraft clock string that
//!    was used to tag the observation. This string must usually be related to
//!    UTC or TDB. Consequently it is necessary to load a file of "spacecraft
//!    clock coefficients" that enables SPICE to transform the spacecraft
//!    clock string into one of the other time systems. This file of spacecraft
//!    clock coefficients is loaded with the routine [FURNSH](crate::raw::furnsh).
//!
//!  A more detailed discussion of Spacecraft Clock is contained in the
//!    Required Reading file [sclk.req](crate::required_reading::sclk) that is included with the SPICE Toolkit.
//!
//!  
//!
//!
//!  
//! ##  Julian Date
//!
//!  The Julian date system is a numerical time system that allows you to
//!    easily compute the number of days between two epochs. NAIF recognizes
//!    two types of Julian dates. Julian Ephemeris Date (JED) and Julian Date
//!    UTC (JDUTC). As with calendar dates used for ephemeris time and calendar
//!    dates UTC, the distinction between the two systems is important. The
//!    names of the two systems overlap, but they correspond to different
//!    moments of time.
//!
//!  Julian Ephemeris Date is computed directly from TDB via the formula
//!
//!  
//!
//! ```text
//!    JED = J2000 + TDB/SPD
//! ```
//!
//!  where J2000 is the Julian Ephemeris Date of the reference epoch for TDB,
//!    and [SPD](crate::raw::spd) is the number or seconds per day.
//!
//!  Julian Date UTC has an integer value (value, not integer type) whenever
//!    the corresponding UTC time is noon.
//!
//!  We recommend against using the JDUTC system as it provides no mechanism
//!    for talking about events that might occur during a leapsecond. All of
//!    the other time systems discussed can be used to refer to events
//!    occurring during a leap second.
//!
//!  
//!
//!
//!  
//! ###  The abbreviation JD
//!
//!  Julian date is often abbreviated as "JD." Unfortunately, the meaning
//!    of this string depends upon context. For example, the SPICE routine
//!    [STR2ET](crate::raw::str2et) treats the string "2451821.1928 JD" as Julian Date UTC. On the
//!    other hand, the SPICE routine [TPARSE](crate::raw::tparse) treats the same string as Julian
//!    Date TDB.
//!
//!  Consequently, for high accuracy work, you must be sure of the context
//!    when using strings labeled in this way. Unless context is clear, it's
//!    usually safer to label Julian Date strings with one of the unambiguous
//!    labels: JDUTC, JDTDB, or JDTDT.
//!
//!  SPICE does not accommodate use of Modified Julian Date (MJD), because
//!    this term has multiple definitions.
//!
//!  
//!
//!
//!  
//! ##  Time Subsystem Routines
//!
//!  
//!
//!
//!  
//! ###  Routine to load needed kernels
//!
//!  In almost all cases, before converting between different representations
//!    of time you must "load" a Leapseconds Kernel (LSK) into memory.
//!
//!  The Leapseconds Kernel is a text kernel loaded using the [FURNSH](crate::raw::furnsh) routine:
//!
//!  
//!
//! ```text
//!    FURNSH ( LSK )
//! ```
//!
//!  Load the Leapseconds Kernel only once per program run.
//!
//!  The precise contents of the Leapseconds Kernel are discussed in the
//!    section "Leapseconds." Text kernels and the [FURNSH](crate::raw::furnsh) routine are
//!    discussed in more detail in KERNEL Required Reading, [kernel.req](crate::required_reading::kernel).
//!
//!  
//!
//!
//!  
//! ###  Routine to convert a time string to TDB (ET)
//!
//!  If you start with a representation of time in the form of a string, such
//!    as "Mon Sep 30 09:59:10 PDT 1996", you will normally need to get this
//!    into a numeric representation before you can work with it. The primary
//!    routine for converting strings to numeric representation ("String to
//!    ET") is:
//!
//!  
//!
//! ```text
//!    STR2ET ( STRING, ET )
//! ```
//!
//!  This routine requires the LSK data.
//!
//!  The default interpretation of STRING is to regard the time of day to be
//!    a time on a 24-hour clock in the UTC time system. The date is a date on
//!    the Gregorian Calendar (this is the calendar used in nearly all Western
//!    societies).
//!
//!  The routine computes the ephemeris epoch corresponding to the input
//!    string. The ephemeris epoch is represented as seconds past the J2000
//!    epoch.
//!
//!  The variety of ways people have developed for representing times is
//!    enormous. It is unlikely that any single routine can accommodate all of
//!    the custom time formats that have arisen in various computing contexts.
//!    However, we believe that [STR2ET](crate::raw::str2et) correctly interprets most time formats
//!    used throughout the planetary science community. For example [STR2ET](crate::raw::str2et)
//!    supports ISO time formats, UNIX 'date' output formats, VMS time formats,
//!    MS-DOS formats, epochs in both the A.D. and B.C. eras, time zones, etc.
//!
//!  
//!
//!
//!  
//! ###  Routine to convert UTC to TDB (ET)
//!
//!  This older routine directly converts a UTC string to numeric ephemeris
//!    time (TDB):
//!
//!  
//!
//! ```text
//!    UTC2ET ( UTCSTR, ET )
//! ```
//!
//!  This routine requires the LSK data.
//!
//!  This routine converts strings in the UTC system to TDB seconds past the
//!    J2000 epoch. Unlike [STR2ET](crate::raw::str2et) it does not support other time systems or
//!    time zones. In addition, the routine does not recognize times on a
//!    12-hour clock. Strings such as
//!
//!  
//!
//! ```text
//!    1983 June 13, 9:00:00 A.M.
//! ```
//!
//!  are treated as erroneous.
//!
//!  
//!
//!
//!  
//! ###  Routine to convert a spacecraft clock time string to TDB (ET)
//!
//!  If you start with a representation of time in the form of a spacecraft
//!    clock (SCLK) string you also normally need to convert it to ephemeris
//!    time (TDB). The primary routine for converting SCLK strings to numeric
//!    TDB ("SCLK String to ET") is:
//!
//!  
//!
//! ```text
//!    SCS2E ( SC, SCLKCH, ET )
//! ```
//!
//!  This routine requires the appropriate SCLK and LSK data.
//!
//!  
//!
//!
//!  
//! ###  Routine to convert a TDB (ET) to a time string based on a format template
//!
//!  If you need to convert time represented as TDB (ET), SPICE provides a
//!    routine to convert TDB to a time string in almost any form of interest
//!    (including many that cannot be recognized by SPICE):
//!
//!  
//!
//! ```text
//!    TIMOUT ( ET, PICTUR, STRING )
//! ```
//!
//!  This routine requires the LSK data.
//!
//!  Consider the following example time string:
//!
//!  
//!
//! ```text
//!    04:29:29.292 Jan 13, 1996
//! ```
//!
//!  The value for PICTUR to create time strings similar in appearance to the
//!    example string is:
//!
//!  
//!
//! ```text
//!    PICTUR = 'HR:MN:SC.### Mon DD, YYYY ::RND'
//! ```
//!
//!  Note, PICTUR could describe a time string format the SPICE time
//!    subsystem parsing routines cannot recognize.
//!
//!  Most of the components in PICTUR are fairly obvious. The exception is
//!    the substring
//!
//!  
//!
//! ```text
//!    ::RND
//! ```
//!
//!  This substring tells the formatting logic to round the seconds portion
//!    of the output string instead of simply truncating. (Note that the case
//!    of the letters is significant in PICTUR.) [TIMOUT](crate::raw::timout) can produce strings
//!    representing epochs in the time systems (UTC, TDB, TT) or any time zone,
//!    and on the Julian Calendar, Gregorian Calendar or Mixed Calendar. You
//!    may round or truncate numeric components.
//!
//!  
//!
//!
//!  
//! ###  Routine to convert a TDB (ET) to UTC
//!
//!  This older routine directly converts an ephemeris time (TDB) to a UTC
//!    string:
//!
//!  
//!
//! ```text
//!    ET2UTC ( ET, FORMAT, PREC, UTCSTR )
//! ```
//!
//!  This routine requires the LSK data.
//!
//!  This routine is not as flexible as [TIMOUT](crate::raw::timout). All outputs are UTC outputs,
//!    and only a limited set of formats are supported.
//!
//!  
//!
//! ```text
//!    Format String   Name              Example String
//!    -------------   -----------       --------------------------
//!    C               Calendar          1979 JUL 04 14:19:57.184
//!    D               Day of Year       1979-114 // 14:19:57.184
//!    J               Julian Date       JD 2433282.529
//!    ISOC            ISO Calendar      1987-04-12T16:31:12.814
//!    ISOD            ISO Day of Year   1987-102T16:31:12.814
//! ```
//!
//!  You may specify the number of decimal places in the fractional part of
//!    the seconds token or the Julian Date (three are used in the examples
//!    above). Note that Julian Dates are prefaced with the character string
//!    'JD' (and are UTC Julian Dates).
//!
//!  [ET2UTC](crate::raw::et2utc) has one advantage over [TIMOUT](crate::raw::timout): it is can output years with more
//!    than four digits which [TIMOUT](crate::raw::timout) cannot do.
//!
//!  
//!
//!
//!  
//! ###  Routine to convert a TDB (ET) to Spacecraft Clock time string
//!
//!  This routine converts ephemeris time (TDB) to a SCLK string:
//!
//!  
//!
//! ```text
//!    SCE2S ( SC, ET, SCLKCH )
//! ```
//!
//!  This routine requires the appropriate SCLK and the LSK data.
//!
//!  
//!
//!
//!  
//! ###  Routine to convert a TDB (ET) to Calendar format TDB
//!
//!  This routine produces output in a single format with a fixed number of
//!    decimal places. Moreover, the calendar strings it produces are on a
//!    formal calendar. There are no leapseconds; each day has exactly 86400
//!    seconds. Since it does not make use of leapseconds, you don't need to
//!    load a Leapseconds Kernel prior to use. This makes it well suited for
//!    producing diagnostic messages. Indeed, it was created so that more user
//!    friendly diagnostic messages could be produced by those SPICE routines
//!    that require TDB as an input.
//!
//!  
//!
//! ```text
//!    ETCAL  ( ET, STRING )
//! ```
//!
//!  This routine requires no kernel data.
//!
//!  
//!
//!
//!  
//! ###  Routine to convert a TDB (ET) to Local True Solar Time
//!
//!  Local solar time is used to give people an idea of how high the sun is
//!    in the sky as seen from a particular site on surface of a planet or
//!    satellite. When the Sun is on the zenith meridian, the local solar time
//!    is 12:00:00 noon. For points on the equator of a body, the Sun rises
//!    around 6:00:00 A.M. local solar time; it sets around 6:00:00 P.M. local
//!    solar time.
//!
//!  Formally, the local solar time at a site on a body is the difference
//!    between the planetocentric longitude of the site and the planetocentric
//!    longitude of the Sun as seen from the center of the body. The angular
//!    difference in these two longitudes is measured in hours, minutes, and
//!    seconds in the same sense that hours, minutes and seconds are used to
//!    measure right ascension--- 24 hours in 360 degrees; 60 minutes in an
//!    hour; 60 seconds in a minute. When the sun is on the zenith meridian
//!    (directly overhead), the hour is defined to be 12. Finally, the hours
//!    increase from sunrise to sunset.
//!
//!  Because of these conventions, an hour of local solar time will not be of
//!    the same duration as a UTC hour. In the case of a site on Mars, a solar
//!    hour will be approximately 62 UTC minutes.
//!
//!  Local solar time for a specific site can be computed using the routine:
//!
//!  
//!
//! ```text
//!    ET2LST ( ET, BODY, LONG, TYPE, HR, MN, SC, TIME, AMPM )
//! ```
//!
//!  This routine requires appropriate SPK and PCK data.
//!
//!  This routine converts ephemeris time (TDB) to the local solar time for a
//!    point at a user specified longitude on the surface of a body. This
//!    computation is performed using the bodyfixed location of the sun.
//!    Consequently, you must first load SPK and PCK files that contain
//!    sufficient position and orientation data for the computation of the
//!    bodyfixed location of the sun.
//!
//!  Load SPK and PCKs (text and binary) using [FURNSH](crate::raw::furnsh).
//!
//!  As with the Leapseconds Kernel, SPKs and PCKs need to be loaded just
//!    once per program run---usually at program initialization.
//!
//!  Please refer to [kernel.req](crate::required_reading::kernel) for further information concerning kernels
//!    \[5].
//!
//!  
//!
//!
//!  
//! ###  Routine to convert a TDB (ET) to planetocentric longitude of the sun
//!
//!  This routine computes planetocentric longitude of the sun at a given
//!    ephemeris time for a given body:
//!
//!  
//!
//! ```text
//!    LS = LSPCN (body, et, abcorr )
//! ```
//!
//!  This routine requires the appropriate SPK and PCK data.
//!
//!  
//!
//!
//!  
//! ###  Routine to convert between uniform time scales
//!
//!  We use the term uniform time scale to refer to those representations of
//!    time that are numeric (each epoch is represented by a number) and
//!    additive, e.g. TT, Julian Date TDB, TAI seconds past J2000, etc.
//!
//!  A numeric time system is additive if given the representations E1 and E2
//!    of any pair of successive epochs, the time elapsed between the epochs is
//!    given by the difference E2 - E1.
//!
//!  Convert between uniform time scales using the appropriate routine:
//!
//!  
//!
//! ```text
//!    double = UNITIM ( EPOCH, INSYS, OUTSYS )
//! ```
//!
//!  This routine requires no kernel data.
//!
//!  The uniform time scales that are supported by this routine are:
//!
//!  
//!
//! ```text
//!    String ID   Time system
//!    ---------   --------------------------
//!    TAI         International Atomic Time
//!    TDB         Barycentric Dynamical Time
//!    TT          Terrestrial Time
//!    TDT         Terrestrial Dynamical Time (TT)
//!    ET          Ephemeris time, alias for TDB
//!    JDTDB       Julian Date relative to TDB
//!    JDTDT       Julian Date relative to TDT (TT)
//!    JED         Julian Ephemeris date (synonym to JDTDB)
//!    GPS         Global Positioning System Time
//! ```
//!
//!     
//! ###  Routine to compute the difference between TDB (ET) and UTC
//!
//!  The routine to compute the difference between TDB (ET) and UTC at a
//!    given epoch is:
//!
//!  
//!
//! ```text
//!    DELTET ( EPOCH, EPTYPE, DELTA )
//! ```
//!
//!  This routine requires the LSK data.
//!
//!  
//!
//!
//!  
//! ###  Routine to create a time string format picture
//!
//!  This routine programmatically constructs a format picture usable in
//!    [TIMOUT](crate::raw::timout) from an example time string:
//!
//!  
//!
//! ```text
//!    TPICTR ( EXAMPL, PICTUR, OK, ERRMSG )
//! ```
//!
//!  This routine requires no kernel data.
//!
//!  The arguments OK and ERRMSG exist because some EXAMPL strings are not
//!    recognized as time strings. [TPICTR](crate::raw::tpictr) recognizes the same set of time
//!    strings as the primary time string parsing routine [TPARSE](crate::raw::tparse). Please refer
//!    to the time string examples shown in the "Input String Conversion"
//!    section of this document.
//!
//!  If you want your output string to be in a system other than UTC you must
//!    supply the label for that system in your example string. [TPICTR](crate::raw::tpictr) can
//!    construct format pictures for strings that are not accepted by the
//!    string conversion routines. For example, if you would like to suppress
//!    the year in a calendar output format, you could use the following
//!    example string:
//!
//!  
//!
//! ```text
//!    EXAMPL = 'Jan 12, 02:28:29.### A.M. (PDT)'
//! ```
//!
//!  Even though this string is ambiguous as an epoch (there's no year
//!    specified), it is sufficient for determining a picture that describes
//!    its format. If you decide to use [TPICTR](crate::raw::tpictr) with inputs like this, be sure
//!    to check the output flag OK; even though you know what is intended, [TPICTR](crate::raw::tpictr) may have problems with some ambiguous time strings.
//!
//!  
//!
//!
//!  
//! ###  Routines returning time constants
//!
//!  The Julian ephemeris date (TDB) of the epoch of the Besselian date 1900:
//!
//!  
//!
//! ```text
//!    B1900()
//! ```
//!
//!  The Julian ephemeris date (TDB) of the epoch of the Besselian date 1950:
//!
//!  
//!
//! ```text
//!    B1950()
//! ```
//!
//!  The Julian Date of 1899 DEC 31 12:00:00 (TDB):
//!
//!  
//!
//! ```text
//!    J1900()
//! ```
//!
//!  The Julian ephemeris date of the epoch 1 Jan 1950 00:00:00 (TDB):
//!
//!  
//!
//! ```text
//!    J1950()
//! ```
//!
//!  The Julian ephemeris date of the epoch 1 Jan 2000 12:00:00 (TDB):
//!
//!  
//!
//! ```text
//!    J2000()
//! ```
//!
//!  The Julian ephemeris date of the epoch 1 Jan 2100 12:00:00:
//!
//!  
//!
//! ```text
//!    J2100()
//! ```
//!
//!  The number of seconds in a Julian year (365.25 Julian days):
//!
//!  
//!
//! ```text
//!    JYEAR()
//! ```
//!
//!  The number of TDB seconds in a Julian day TDB (86400 seconds):
//!
//!  
//!
//! ```text
//!    SPD()
//! ```
//!
//!  The number of seconds in a tropical year (approximately the number of
//!    seconds from one spring equinox to the next):
//!
//!  
//!
//! ```text
//!    TYEAR()
//! ```
//!
//!     
//! ##  Foundation Routines and Utilities
//!
//!  At the heart of the SPICE time subsystem are the "foundation" routines
//!    available: [TPARTV](crate::raw::tpartv) and [TTRANS](crate::raw::ttrans). TPARTV disassembles a time string and
//!    convert it to a vector of numeric components. [TTRANS](crate::raw::ttrans) serves the role of
//!    converting between the various numeric vector representations of time.
//!    If you need to build your own time conversion routines, these routines
//!    are a good place to begin.
//!
//!  These and other utility routines lack wrapper interfaces in CSPICE but
//!    can be used in their f2c'd form (see cspice.req included in the CSPICE
//!    toolkit for details).
//!
//!  
//!
//!
//!  
//! ###  Parse a time string to a time vector
//!
//!  
//!
//! ```text
//!    TPARTV ( STRING,
//!             TVEC,   NTVEC, TYPE,
//!             MODIFY, MODS,  YABBRV, SUCCES,
//!             PICTUR, ERROR )
//! ```
//!
//!     
//! ###  Convert between different parsed representations of time
//!
//!  
//!
//! ```text
//!    TTRANS ( FROM, TO, TVEC )
//! ```
//!
//!     
//! ###  Time utility routines
//!
//!  In addition to the foundation routines, you may find the following
//!    utility routines helpful.
//!
//!  Convert two-digit abbreviated years to full years. You set the lower
//!    bound of the 100 year mapping interval via the routine [TSETYR](crate::raw::tsetyr) discussed
//!    earlier in this document.
//!
//!  
//!
//! ```text
//!    TEXPYR ( YEAR )
//! ```
//!
//!  Take a numeric vector representing the components of a calendar time to
//!    check that all components are within the normal range used in
//!    conversation. Note that [TCHECK](crate::raw::tcheck) performs no action until you call [TPARCH](crate::raw::tparch) with an argument of "YES".
//!
//!  
//!
//! ```text
//!    TCHECK ( TVEC, TYPE, MODS, MODIFY, OK, ERROR )
//! ```
//!
//!  Determine if component checking has been enabled in [TCHECK](crate::raw::tcheck) via a call to [TPARCH](crate::raw::tparch).
//!
//!  
//!
//! ```text
//!    TCHCKD ( YESNO )
//! ```
//!
//!  Convert the year, month, and day of an epoch on the Julian Calendar to
//!    the corresponding year, month, day and day-of-year on the Gregorian
//!    calendar.
//!
//!  
//!
//! ```text
//!    JUL2GR (  YEAR, MONTH, DAY, DOY )
//! ```
//!
//!  Convert the year, month, and day of an epoch on the Gregorian Calendar
//!    to the corresponding year, month, day and day-of-year on the Julian
//!    calendar.
//!
//!  
//!
//! ```text
//!    GR2JUL (  YEAR, MONTH, DAY, DOY )
//! ```
//!
//!     
//! ##  Input String Conversion
//!
//!  We normally represent epochs as a combination of a date and time of day.
//!    The simplest means of specifying an epoch as a date and time is to
//!    create a string such as:
//!
//!  
//!
//! ```text
//!    STRING = 'Oct 1, 1996 09:12:32'
//! ```
//!
//!  However, arithmetic is most easily performed with numeric
//!    representations of time. In SPICE we represent epochs as some number of
//!    double precision seconds past the J2000 epoch.
//!
//!  The analyzing the input string and assigning meaning to its various
//!    components, a.k.a. parsing, is performed by lower level time system
//!    routines.
//!
//!  Below are a number of examples of strings and the interpretation
//!    assigned to the various components.
//!
//!  ISO (T) Formats.
//!
//!  
//!
//! ```text
//!    String                        Year Mon  DOY DOM  HR Min Sec
//!    ----------------------------  ---- ---  --- ---  -- --- ------
//!    1996-12-18T12:28:28           1996 Dec   na  18  12  28 28
//!    1986-01-18T12                 1986 Jan   na  18  12  00 00
//!    1986-01-18T12:19              1986 Jan   na  18  12  19 00
//!    1986-01-18T12:19:52.18        1986 Jan   na  18  12  19 52.18
//!    1986-01-18T12:19:52.18Z       1986 Jan   na  18  12  19 52.18
//!    1995-08T18:28:12              1995  na  008  na  18  28 12
//!    1995-08T18:28:12Z             1995  na  008  na  18  28 12
//!    1995-18T                      1995  na  018  na  00  00 00
//!    0000-01-01T                   1 BC Jan   na  01  00  00 00
//! ```
//!
//!  Calendar Formats.
//!
//!  
//!
//! ```text
//!    String                        Year   Mon DOM  HR Min  Sec
//!    ----------------------------  ----   --- ---  -- ---  ------
//!    Tue Aug  6 11:10:57  1996     1996   Aug  06  11  10  57
//!    1 DEC 1997 12:28:29.192       1997   Dec  01  12  28  29.192
//!    2/3/1996 17:18:12.002         1996   Feb  03  17  18  12.002
//!    Mar 2 12:18:17.287 1993       1993   Mar  02  12  18  17.287
//!    1992 11:18:28  3 Jul          1992   Jul  03  11  18  28
//!    June 12, 1989 01:21           1989   Jun  12  01  21  00
//!    1978/3/12 23:28:59.29         1978   Mar  12  23  28  59.29
//!    17JUN1982 18:28:28            1982   Jun  17  18  28  28
//!    13:28:28.128 1992 27 Jun      1992   Jun  27  13  28  28.128
//!    1972 27 jun 12:29             1972   Jun  27  12  29  00
//!    '93 Jan 23 12:29:47.289       1993*  Jan  23  12  29  47.289
//!    27 Jan 3, 19:12:28.182        2027*  Jan  03  19  12  28.182
//!    23 A.D. APR 4, 18:28:29.29    0023** Apr  04  18  28  29.29
//!    18 B.C. Jun 3, 12:29:28.291   -017** Jun  03  12  29  28.291
//!    29 Jun  30 12:29:29.298       2029+  Jun  30  12  29  29.298
//!    29 Jun '30 12:29:29.298       2030*  Jun  29  12  29  29.298
//! ```
//!
//!  Day of Year Formats.
//!
//!  
//!
//! ```text
//!    String                        Year  DOY HR Min Sec
//!    ----------------------------  ----  --- -- --- ------
//!    1997-162::12:18:28.827        1997  162 12  18 28.827
//!    162-1996/12:28:28.287         1996  162 12  28 28.287
//!    1993-321/12:28:28.287         1993  231 12  28 28.287
//!    1992 183// 12:18:19           1992  183 12  18 19
//!    17:28:01.287 1992-272//       1992  272 17  28 01.287
//!    17:28:01.282 272-1994//       1994  272 17  28 01.282
//!    '92-271/ 12:28:30.291         1992* 271 12  28 30.291
//!    92-182/ 18:28:28.281          1992* 182 18  28 28.281
//!    182-92/ 12:29:29.192          0182+ 092 12  29 29.192
//!    182-'92/ 12:28:29.182         1992  182 12  28 29.182
//! ```
//!
//!  Julian Date Strings.
//!
//!  
//!
//! ```text
//!    jd 28272.291                  Julian Date   28272.291
//!    2451515.2981 (JD)             Julian Date 2451515.2981
//!    2451515.2981 JD               Julian Date 2451515.2981
//! ```
//!
//!  Abbreviations Used in Tables
//!
//!  
//!
//! ```text
//!    na    --- Not Applicable
//!    Mon   --- Month
//!    DOY   --- Day of Year
//!    DOM   --- Day of Month
//!    Wkday --- Weekday
//!    Hr    --- Hour
//!    Min   --- Minutes
//!    Sec   --- Seconds
//! ```
//!
//!  *  *
//!
//!
//!  The default interpretation of a year that has been abbreviated to two
//! digits with or without a leading quote as in 'xy or xy (such as '92 or
//! 92) is to treat the year as 19xy if xy > 68 and to treat it as 20xy
//! otherwise. Thus '70 is interpreted as 1970 and '67 is treated as 2067.
//! However, you may change the "split point" and centuries through use of
//! the SPICE routine [TSETYR](crate::raw::tsetyr). See that routine for a discussion of how you
//! may reset the split point.
//!
//!  *  **
//!
//!
//!  All epochs are regarded as belonging to the Gregorian calendar. We
//! formally extend the Gregorian calendar backward and forward in time for
//! all epochs. If you have epochs belonging to the Julian Calendar,
//! consult the SPICELIB routines [TPARTV](crate::raw::tpartv) and [JUL2GR](crate::raw::jul2gr) for a discussion
//! concerning conversions to the Gregorian calendar and ET. The routines
//! [TIMDEF](crate::raw::timdef) and [STR2ET](crate::raw::str2et), used together, also support conversions from Julian
//! Calendar epochs to ET.
//!
//!  *  +
//!
//!
//!  When a day of year format or calendar format string is input and
//! neither of the integer components of the date is greater than 1000, the
//! first integer is regarded as being the year.
//!
//!  Any integer greater than 1000 is regarded as a year specification. Thus
//!    1001-1821//12:28:28 is interpreted as specifying two years and will be
//!    rejected as ambiguous.
//!
//!  
//!
//!
//!  
//! ###  Parsing Time Strings
//!
//!  A time string is parsed by first scanning the string from left to right
//!    and identifying recognizable substrings. (integers, punctuation marks,
//!    names of months, names of weekdays and time systems, time zones, etc.)
//!    These recognizable substrings are called the tokens of the input string.
//!    The meaning of some tokens are immediately determined. For example named
//!    months, weekdays and time systems have clear meanings. However, the
//!    meanings of numeric components must be deciphered from their magnitudes
//!    and location in the string relative to the immediately recognized
//!    components of the input string.
//!
//!  The following substrings are immediately recognizable.
//!
//!  
//!
//! *  1. All months (January, February, ... ) or any abbreviation of at least 3
//! letters;
//!
//!  *  2. All weekdays (Sunday, Monday, ... ) or any abbreviation of at least 3
//! letters;
//!
//!  *  3. Standard abbreviations of U.S. time zones: "EST", "EDT", "CST",
//! "CDT", "MST", "MDT", "PDT", "PST";
//!
//!  *  4. The abbreviations for eras: "B.C.", "BC", "A.D.", and "AD";
//!
//!  *  5. Time systems: "TT", "TDT", "TDB", "UTC" (Note that "ET" is not a
//! recognized time system);
//!
//!  *  6. Julian Date Label: "JD" (Note that "JED" is not a recognized Julian
//! Date Label);
//!
//!  *  7. The 12-hour clock labels: "A.M.", "AM", "P.M." and "PM";
//!
//!  *  8. Time Zones expressed as UTC offsets: UTC+HR:MN, UTC-HR:MN where HR is an
//! unsigned integer between 0 and 12 inclusive; MN is an unsigned integer
//! between 0 and 59 inclusive.
//!
//!  With the exception of months, all items above may be enclosed in
//!    parentheses. For example "TDB" and "(TDB)" are both recognized as
//!    the same time system.
//!
//!  The case of the letters in these substrings does not matter. For example
//!    all of the various ways of writing "TDB" ( "TDB", "tDB", ...
//!    "tdb") are recognized as "TDB".
//!
//!  It is not necessary to leave space between the various substrings. For
//!    example "JDTDT" and "JDUTC" are recognized as "JD" followed by
//!    "TDT" and "JD" followed by "UTC" respectively.
//!
//!  To determine the meaning of the numeric tokens in the input string, a
//!    set of transformation rules are applied to the full set of tokens in the
//!    string. These transformations are repeated until the meaning of every
//!    token has been determined or until further transformations yield no new
//!    clues into the meaning of the numeric tokens. Here is an overview of the
//!    rules that are applied to the various tokens in the string.
//!
//!  
//!
//! *  1. Unless the substring "JD" or "jd" is present, the string is assumed to
//! be a calendar format (day-month-year or year and day of year). If the
//! substring JD or jd is present, the string is assumed to represent a Julian
//! date.
//!
//!  *  2. If the Julian date specifier is not present, any integer greater than 999
//! is regarded as being a year specification.
//!
//!  *  3. A dash "-" can represent a minus sign only if it precedes the first digit
//! in the string and the string contains the Julian date specifier (JD). (No
//! negative years, months, days, etc. are allowed).
//!
//!  *  4. Numeric components of a time string must be separated by a character that
//! is not a digit or decimal point. Only one decimal component is allowed. For
//! example 1994219.12819 is sometimes interpreted as the 219th day of 1994 +
//! 0.12819 days. The SPICE time subsystem does not support such strings.
//!
//!  *  5. No exponential components are allowed. For example you can't specify the
//! Julian date of J2000 as 2.451545E6. You also can't input 1993 Jun 23
//! 23:00:01.202E-4 and have to explicitly list all zeros that follow the
//! decimal point: i.e. 1993 Jun 23 23:00:00.0001202.
//!
//!  *  6. The single colon (:) when used to separate numeric components of a string
//! is interpreted as separating Hours, Minutes, and Seconds of time.
//!
//!  *  7. If a double slash (//) or double colon (::) follows a pair of integers,
//! those integers are assumed to represent the year and day of year.
//!
//!  *  8. A quote followed by an integer less than 100 is regarded as an abbreviated
//! year. For example: '93 would be regarded as the 93rd year of the reference
//! century. See the SPICELIB routine [TEXPYR](crate::raw::texpyr) for further discussion of
//! abbreviated years.
//!
//!  *  9. An integer followed by "B.C." or "A.D." is regarded as a year in the era
//! associated with that abbreviation.
//!
//!  *  10. All dates are regarded as belonging to the extended Gregorian Calendar (the
//! Gregorian calendar is the calendar currently used by western society). See
//! the routine [TIMDEF](crate::raw::timdef) to modify this behavior.
//!
//!  *  11. If the ISO date-time separator (T) is present in the string ISO allowed
//! token patterns are examined for a match with the current token list. If no
//! match is found the search is abandoned and appropriate diagnostic messages
//! are generated. Historically the interpretation of ISO formatted time
//! strings deviates from the ISO standard in allowing two digit years and
//! expanding years in the 0 to 99 range the same way as is done for non ISO
//! formatted strings. Due to this interpretation it is impossible to specify
//! times in years in the 0 A.D. to 99 A.D. range using ISO formatted strings
//! on the input.
//!
//!  *  12. iIf two delimiters are found in succession in the time string, the time
//! string is diagnosed as an erroneous string. (Delimiters are comma, white
//! space, dash, slash, period, or day of year mark. The day of year mark is a
//! pair of forward slashes or a pair of colons.)
//!
//!  *  Note the delimiters do not have to be the same. The pair of characters ",-"
//! counts as two successive delimiters.
//!
//!  *  13. White space and commas serve only to delimit tokens in the input string.
//! They do not affect the meaning of any of the tokens.
//!
//!  *  14. If an integer is greater than 1000 (and the "JD" label is not present, the
//! integer is regarded as a year.
//!
//!  *  15. When the size of the integer components does not clearly specify a year the
//! following patterns are assumed
//!
//!  ```text
//!             Calendar Format
//!  
//!                Year Month Day
//!                Month Day Year
//!                Year Day Month
//!  
//!                where Month is the name of a month, not its numeric
//!                value.
//!  
//!                When integer components are separated by slashes (/)
//!                as in 3/4/5. Month, Day, Year is assumed (2005 March 4)
//!  
//!             Day of Year Format.
//!  
//!                If a day of year marker is present (// or ::) the
//!                pattern
//!  
//!                  I-I// or I-I:: (where I stands for an integer)
//!  
//!                is interpreted as Year Day-of-Year. However, I-I/ is
//!                regarded as ambiguous.
//! ```
//!  Once the various tokens have been determined and a meaning attached to
//!    them, the Time subsystem uses the tokens to construct the double
//!    precision number giving the number of seconds past J2000 that
//!    corresponds to input string. However, not all tokens or token
//!    combinations are allowed by the routines.
//!
//!  
//!
//!
//!  
//! ###  Labels (A.M. and P.M.)
//!
//!  If you add more information to the string, the time parser makes a more
//!    informed interpretation of the time string. For example:
//!
//!  
//!
//! ```text
//!    1988 June 13, 3:29:48 P.M.
//! ```
//!
//!  is still regarded as a UTC epoch. However, with the addition of the
//!    "P.M." label it is now interpreted as the same epoch as the unlabeled
//!    epoch 1988 June 13, 15:29:48. Similarly
//!
//!  
//!
//! ```text
//!    1988 June 13, 12:29:48 A.M.
//! ```
//!
//!  is interpreted as
//!
//!  
//!
//! ```text
//!    1988 June 13, 00:29:48
//! ```
//!
//!  on the 24-hour clock.
//!
//!  12:00 A.M. corresponds to Midnight (00:00 on the 24-hour clock). 12:00
//!    P.M. corresponds to Noon (12:00 on the 24-hour clock).
//!
//!  
//!
//!
//!  
//! ###  Labels (Time Zones)
//!
//!  You may add still further indicators to the string. For example
//!
//!  
//!
//! ```text
//!    1988 June 13, 3:29:48 P.M. PST
//! ```
//!
//!  is interpreted as an epoch in the Pacific Standard Time system. This is
//!    equivalent to
//!
//!  
//!
//! ```text
//!    1988 June 13, 23:29:48 UTC
//! ```
//!
//!  All of the standard abbreviations for U.S. time zones are recognized by
//!    the time parser.
//!
//!  
//!
//! ```text
//!    EST   --- Eastern Standard Time  ( UTC-5:00 )
//!    CST   --- Central Standard Time  ( UTC-6:00 )
//!    MST   --- Mountain Standard Time ( UTC-7:00 )
//!    PST   --- Pacific Standard Time  ( UTC-8:00 )
//!  
//!    EDT   --- Eastern Daylight Time  ( UTC-4:00 )
//!    CDT   --- Central Daylight Time  ( UTC-5:00 )
//!    MDT   --- Mountain Daylight Time ( UTC-6:00 )
//!    PDT   --- Pacific Daylight Time  ( UTC-7:00 )
//! ```
//!
//!  In addition, any other time zone may be specified by representing its
//!    offset from UTC.
//!
//!  To specify an offset from UTC you need to create an offset label. The
//!    label starts with the letters 'UTC' followed by a '+' for time zones
//!    east of Greenwich and '-' for time zones west of Greenwich. This is
//!    followed by the number of hours to add or subtract from UTC. This is
//!    optionally followed by a colon ':' and the number of minutes to add or
//!    subtract to get the local time zone. Thus to specify the time zone of
//!    Calcutta (which is 5 and 1/2 hours ahead of UTC) you would specify the
//!    time zone to be UTC+5:30. To specify the time zone of Newfoundland
//!    (which is 3 and 1/2 hours behind UTC) use the offset notation UTC-3:30.
//!
//!  
//!
//!
//!  
//! ###  Labels ( TDB, TT, and UTC )
//!
//!  In addition to specifying time zones you may specify that the string be
//!    interpreted as a formal calendar representation in either the
//!    Barycentric Dynamical Time system (TDB) or the Terrestrial Time system
//!    (TT).
//!
//!  In these systems there are no leapseconds; every day has exactly 86400
//!    seconds. TDB times are written as
//!
//!  
//!
//! ```text
//!    1988 June 13, 12:29:48 TDB
//! ```
//!
//!  TT times are written as:
//!
//!  
//!
//! ```text
//!    1988 June 13, 12:29:48 TT
//! ```
//!
//!  To add clarity or to override any changes you happen to make to the
//!    default behavior of ET2STR (see below) you may add the label "UTC" to
//!    any time string.
//!
//!  
//!
//! ```text
//!    1998 Jun 13, 12:29:48 UTC
//! ```
//!
//!  Note that the system label may be placed anywhere in the time string.
//!    All of the following will be understood by the time parsing software:
//!
//!  
//!
//! ```text
//!    TDB 1988 June 13, 12:29:48
//!    1988 June 13, 12:29:48 TDB
//!    1988 June 13, TDB 12:29:48
//! ```
//!
//!     
//! ##  Changing Default Behavior
//!
//!  The three time string transformation routines can be adjusted at run
//!    time so that various built in defaults can be changed without re-writing
//!    any of the code for the routines.
//!
//!  
//!
//!
//!  
//! ###  Abbreviated Years
//!
//!  All string transformation routines treat abbreviated years in the same
//!    fashion. The default behavior is to map any abbreviated year into the
//!    range from 1968 to 2067. Thus the year 22 corresponds to 2022; 77
//!    corresponds to 1977. However, you may reset the lower end of this 100
//!    year range via the routine [TSETYR](crate::raw::tsetyr).
//!
//!  E.g., set the default range to be from 1972 to 2071:
//!
//!  
//!
//! ```text
//!    TSETYR ( 1972 )
//! ```
//!
//!  Note that this change affects the behavior of all string conversion
//!    routines.
//!
//!  
//!
//!
//!  
//! ###  Range of Time String Components
//!
//!  The routines [TPARSE](crate::raw::tparse) and [UTC2ET](crate::raw::utc2et) accept time strings whose numeric
//!    components are outside of the normal range of values used in time and
//!    calendar representations. For example strings such as
//!
//!  
//!
//! ```text
//!    1985 FEB 43 27:65:25  (equivalent to 1985 MAR 16 04:05:25)
//! ```
//!
//!  will be accepted as input.
//!
//!  You might wish to restrict the range of input strings so that this
//!    behavior is not allowed. The SPICELIB routine [TPARCH](crate::raw::tparch) exists for this
//!    purpose.
//!
//!  This routine lacks a wrapper interface in CSPICE but can be used in its
//!    f2c'd form (see cspice.req included in the CSPICE toolkit for details).
//!
//!  Example:
//!
//!  
//!
//! ```text
//!    TPARCH ( 'YES' )
//! ```
//!
//!  Call the routine early in your program, prior to any calls to routines [TPARSE](crate::raw::tparse) and [UTC2ET](crate::raw::utc2et). Then the components of calendar strings will be
//!    restricted so that all calendar components will be in the "expected"
//!    range.
//!
//!  Please refer to the header of [TPARSE](crate::raw::tparse) or SPICELIB routine [TPARCH](crate::raw::tparch) for
//!    information describing the exact ranges for the components.
//!
//!  [STR2ET](crate::raw::str2et) does not accept time strings whose components are outside the
//!    normal range used in conversation. You cannot alter this behavior
//!    without re-coding [STR2ET](crate::raw::str2et).
//!
//!  
//!
//!
//!  
//! ###  Default Time Systems, Time Zone, and Calendar
//!
//!  When a string is presented without a time system or time zone label the
//!    parsing logic assumes that the string represents a time in a default
//!    time zone or time system. If you take no action, the default time system
//!    is UTC. (There is no time zone offset; UTC is the same as UTC+00:00) You
//!    can override the default by simply including the time zone or time
//!    system of interest in the input time string. However, under some
//!    circumstances you may find that you almost always use the TDB time
//!    system. In such a case you would normally need to include the TDB label
//!    in the time string. Hence, the defaults used by the Time Subsystem might
//!    be a hindrance rather than a convenience. With this possibility in mind,
//!    the user may alter default behavior with regard to default time system
//!    or time zone. To change the default time system, time zone, or calendar
//!    use the appropriate "SET" routine:
//!
//!  
//!
//! ```text
//!    TIMDEF ( 'SET', ITEM, VALUE )
//! ```
//!
//!  ```text
//!    ``item''      ``value''
//!    ---------     --------------
//!    CALENDAR      GREGORIAN
//!                  JULIAN
//!                  MIXED
//!  
//!    SYSTEM        TDB
//!                  TT
//!                  TDT
//!                  UTC
//!  
//!    ZONE          EST
//!                  EDT
//!                  CST
//!                  CDT
//!                  MST
//!                  MDT
//!                  PST
//!                  PDT
//!                  UTC+HR
//!                  UTC-HR       ( 0 <= HR < 13 )
//!                  UTC+HR:MN    ( 0 <= MN < 60 )
//!                  UTC-HR:MN
//! ```
//!  The case of item is not significant.
//!
//!  Keep in mind that if you specify a time zone or time system label in the
//!    input time string the default time zone or system is not used. The label
//!    in the string is used to determine the time zone or time system.
//!
//!  
//!
//!
//!  
//! ###  Changing the Time System
//!
//!  Three time systems are supported: UTC, TDB, TDT, and TT. To change the
//!    default system to one of these three systems, use the appropriate
//!    routine call, i.e.:
//!
//!  
//!
//! ```text
//!    TIMDEF ( 'SET', 'SYSTEM', 'UTC' )
//!    TIMDEF ( 'SET', 'SYSTEM', 'TDB' )
//!    TIMDEF ( 'SET', 'SYSTEM', 'TDT' )
//!    TIMDEF ( 'SET', 'SYSTEM', 'TT' )
//! ```
//!
//!  Note that setting a time system turns off any default time zone you may
//!    have 'SET' using [TIMDEF](crate::raw::timdef).
//!
//!  
//!
//!
//!  
//! ###  Time Zones
//!
//!  The default time zone is simply Greenwich Mean Time (UTC+00:00). To
//!    change the default behavior of the Time Subsystem so that unlabeled
//!    strings are assumed to be referenced to a particular time zone (for
//!    example Pacific Standard Time) use the appropriate routine:
//!
//!  
//!
//! ```text
//!    TIMDEF ( 'SET', 'ZONE', 'PST' )
//! ```
//!
//!  Note that setting a time zone turns off any default time system you may
//!    have 'SET' via [TIMDEF](crate::raw::timdef).
//!
//!  
//!
//!
//!  
//! ###  Calendars
//!
//!  The default calendar used by the Time Subsystem is the Gregorian
//!    calendar. However, the Gregorian calendar did not exist until October
//!    15, 1582 (the prior day having date October 5, 1582). To complicate
//!    matters, many countries did not adopt the Gregorian calendar until
//!    centuries later. Prior to adoption of the Gregorian calendar most
//!    western societies used the Julian calendar. The generation of successive
//!    days is identical on the Julian and Gregorian calendars except for the
//!    determination of leap days in years exactly divisible by 100. On the
//!    Julian calendar, a leap day is inserted as the last day of February
//!    every 4 years. The Gregorian calendar adds a leap day as the last day of
//!    February every 4 years with the exception of years exactly divisible by
//!    100. Such years are leap years only if the year is evenly divisible by
//!    400. Thus the year 2000 is a leap year on the Gregorian calendar but
//!    1900 is not.
//!
//!  Both the Gregorian and Julian calendars can be extended forward and
//!    backward in time indefinitely. The default behavior uses the Gregorian
//!    calendar for all epochs. However, using the [TIMDEF](crate::raw::timdef) routine 'SET'
//!    capability, you can set the default calendar to one of three: GREGORIAN,
//!    JULIAN, or MIXED:
//!
//!  
//!
//! ```text
//!    TIMDEF ( 'SET', 'CALENDAR', 'GREGORIAN' )
//!    TIMDEF ( 'SET', 'CALENDAR', 'JULIAN'    )
//!    TIMDEF ( 'SET', 'CALENDAR', 'MIXED'     )
//! ```
//!
//!  The "MIXED" calendar assumes that calendar strings for epochs prior to
//!    October 5, 1582 belong to the Julian Calendar; strings for later epochs
//!    are assumed to belong to the Gregorian Calendar. The specification of a
//!    calendar does not affect a previous setting of a time system or time
//!    zone.
//!
//!  
//!
//!
//!  
//! ##  Usage example
//!
//!  The following program demonstrates use of the time conversion routines
//!    [STR2ET](crate::raw::str2et), TPICTR, TIMOUT and [ET2UTC](crate::raw::et2utc).
//!
//!  Note that the data necessary to convert between UTC and TDB are loaded
//!    into the kernel pool just once, typically during program initialization.
//!
//!  
//!
//! ```text
//!          PROGRAM TIME_T
//!    C
//!    C     Convert between UTC and TDB interactively, and convert TDB
//!    C     back to UTC in calendar format, DOY format, and as a
//!    C     Julian date.
//!    C
//!    C     Requires a Leapseconds Kernel.
//!    C
//!          INTEGER               FILEN
//!          PARAMETER           ( FILEN = 128 )
//!  
//!          INTEGER               LNSIZE
//!          PARAMETER           ( LNSIZE = 60 )
//!  
//!  
//!          CHARACTER*(8)         ANSWER
//!          CHARACTER*(FILEN)     KERNEL
//!  
//!          CHARACTER*(LNSIZE)    DOY
//!          CHARACTER*(LNSIZE)    ERROR
//!          CHARACTER*(LNSIZE)    EXAMP1
//!          CHARACTER*(LNSIZE)    EXAMP2
//!          CHARACTER*(LNSIZE)    JDUTC
//!          CHARACTER*(LNSIZE)    PICTR1
//!          CHARACTER*(LNSIZE)    PICTR2
//!          CHARACTER*(LNSIZE)    PST
//!          CHARACTER*(LNSIZE)    STR
//!          CHARACTER*(LNSIZE)    UTC
//!  
//!          DOUBLE PRECISION      ET
//!  
//!          LOGICAL               OK
//!  
//!    C
//!    C     Get the name of the Leapseconds Kernel file.
//!    C
//!          WRITE (*,*)  'We need to load a Leapseconds Kernel.'
//!          CALL PROMPT ('Kernel Name: ', KERNEL )
//!  
//!    C
//!    C     Load the Leapseconds Kernel into the kernel pool.
//!    C
//!          CALL FURNSH ( KERNEL )
//!  
//!  
//!    C
//!    C     Create pictures for producing strings similar to
//!    C     those below.
//!    C
//!          EXAMP1 = 'Fri Oct 04, 08:57:28.000 (UTC) 1996'
//!          EXAMP2 = 'Fri Oct 04, 08:57:28.000 (PST) 1996'
//!  
//!          CALL TPICTR ( EXAMP1, PICTR1, OK, ERROR )
//!          CALL TPICTR ( EXAMP2, PICTR2, OK, ERROR )
//!  
//!  
//!    C
//!    C     Compute result for each new UTC epoch.
//!    C
//!          ANSWER = 'Y'
//!  
//!          DO WHILE (      ( ANSWER(1:1) .EQ. 'Y' )
//!         .           .OR. ( ANSWER(1:1) .EQ. 'y' )  )
//!  
//!             WRITE (*,*) ' '
//!             CALL PROMPT ( 'Enter a time: ', STR )
//!  
//!             CALL STR2ET ( STR, ET )
//!  
//!             WRITE (*,*) ' '
//!             WRITE (*,*) 'Input time converts to TDB ' //
//!         .               '(sec past J2000)', ET
//!  
//!  
//!             CALL TIMOUT ( ET, PICTR1,    UTC   )
//!             CALL TIMOUT ( ET, PICTR2,    PST   )
//!             CALL ET2UTC ( ET, 'ISOC', 3, DOY   )
//!             CALL ET2UTC ( ET, 'J',    7, JDUTC )
//!  
//!             WRITE (*,*) ' '
//!             WRITE (*,*) 'ET converts back to'
//!             WRITE (*,*) ' '
//!             WRITE (*,*) UTC
//!             WRITE (*,*) PST
//!             WRITE (*,*) ' '
//!             WRITE (*,*) DOY
//!             WRITE (*,*) JDUTC
//!  
//!             WRITE (*,*) ' '
//!             CALL PROMPT ('Do you wish to continue?', ANSWER )
//!  
//!          END DO
//!  
//!          END
//! ```
//!
//!     
//! #  Appendix A. Summary of Time Subsystem Routines
//!
//!  
//!
//! ```text
//!    B1900()                Constant
//!    B1950()                Constant
//!    J1900()                Constant
//!    J1950()                Constant
//!    J2000()                Constant
//!    J2100()                Constant
//!    JYEAR()                Constant
//!    SPD()                  Constant
//!    TYEAR()                Constant
//!    DELTET                 Delta TDB, TDB - UTC
//!    ET2LST                 TDB to Local Solar Time
//!    ET2UTC                 TDB to UTC
//!    ETCAL                  Convert TDB to Calendar format
//!    GR2JUL                 Gregorian to Julian Calendar
//!    JUL2GR                 Julian to Gregorian Calendar
//!    LSPCN                  Longitude of the sun, planetocentric
//!    STR2ET                 String to TDB
//!    TCHCKD                 Time components are checked
//!    TCHECK                 Time Check
//!    TEXPYR                 Expand year
//!    TIMDEF                 Set/get time software defaults
//!    TIMOUT                 TDB to string time Output
//!    TPARSE                 Parse a UTC time string
//!    TPARTV                 Parse to a time vector
//!    TPICTR                 Create a Time Format Picture
//!    TSETYR                 Set year expansion boundaries
//!    TTRANS                 Time transformation
//!    UNITIM                 Uniform time scale transformation
//!    UTC2ET                 UTC to TDB
//! ```
//!
//!     
//! #  Appendix B. Non-native text files
//!
//!  Starting with the N0057 release of the Fortran SPICE Toolkit, SPICELIB,
//!    (March, 2004) the data loading mechanism detects and prohibits loading
//!    text kernel files containing lines terminated with EOF character(s)
//!    non-native to the platform on which the Toolkit was compiled. If a
//!    non-native EOL terminator is detected in the first 132 characters of a
//!    text kernel, the execution is stopped and an error message is displayed.
//!    This feature does not work with files that are smaller that 132 bytes or
//!    have the first line longer that 132 characters.
//!
//!  All other SPICE Toolkit language implementations can read non-native
//!    text files.
//!
//!  
//!
//!
//!  
//! #  Appendix C. Parsing Time Strings
//!
//!  This appendix gives a detailed account of how the SPICELIB routine
//!    [TPARTV](crate::raw::tpartv) analyzes and assigns meaning to the components of a time string.
//!    [TPARTV](crate::raw::tpartv) is the "foundation" routine relied upon by all other routines
//!    that have time strings as input arguments.
//!
//!  This appendix is not for everyone. Unless you need to understand in
//!    great detail how parsing of strings is performed, you can safely skip
//!    this appendix. The discussion below is quite technical and mirrors very
//!    closely the code in [TPARTV](crate::raw::tpartv) that handles the parsing of time strings.
//!
//!  
//!
//!
//!  
//! ##  An Outline of the Parser
//!
//!  The first step in processing a time string is to scan it from left to
//!    right identifying various substrings. If a substring is encountered that
//!    cannot be identified, attempts to further process the string are
//!    abandoned.
//!
//!  Having identified the components in the string as integers, months,
//!    weekdays, time systems, etc. an internal representation of the string is
//!    constructed. This representation is simply a list of the identified
//!    substrings in the order they are encountered. Each item in the list is
//!    called a token.
//!
//!  Working with the list of tokens, various rules are applied to remove
//!    some tokens and combine others into new tokens. The process of
//!    combination and removal of tokens continues until all tokens belong to a
//!    special set of "meaningful" tokens or until no further combinations
//!    and removals can be performed. If processing stops before all tokens are
//!    meaningful, a diagnostic message is created and the string is regarded
//!    as un-parsable. If all of the tokens are meaningful, a compatibility
//!    check is performed on the tokens to make sure that they unambiguously
//!    specify an epoch.
//!
//!  Once it is clear that an unambiguous epoch has been specified, the
//!    substrings corresponding to the meaningful tokens are converted into
//!    numeric representations or are noted so that the time conversion
//!    software can properly interpret the numeric components.
//!
//!  Almost all of the work of manipulating tokens is carried out by SPICE
//!    private routines. These routines are not considered part of the SPICE
//!    public interface. Feel free to read and copy these routines. However, we
//!    strongly recommend that you not call these routines in your own code
//!    since we do not guarantee backward compatibility of these routines in
//!    future releases of the Toolkit.
//!
//!  
//!
//!
//!  
//! ##  Tokenizing the Input String
//!
//!  The first step in parsing a time string is to decompose it into
//!    recognizable substring components. This decomposition is done as
//!    follows:
//!
//!  Starting with the next unexamined character (on the first pass this is
//!    the first character in the string), scan from left to right looking for
//!    one of the following classes of substrings:
//!
//!  
//!
//! *  1. a maximal sequence of digits forming an unsigned integer.
//!
//!  *  2. a maximal sequence of space characters
//!
//!  *  3. a tab character
//!
//!  *  4. a weekday (or abbreviation of a weekday of at least 3 letters)
//!
//!  *  5. a month name (or abbreviation of a month name of at least 3 letters)
//!
//!  *  6. a time zone ( standard U.S. abbreviations)
//!
//!  *  7. a positive UTC offset specifier ( 'UTC+' )
//!
//!  *  8. a negative UTC offset specifier ( 'UTC-' )
//!
//!  *  9. a time system (TT, TDT, TDB, UTC)
//!
//!  *  10. an era specifier ( 'A.D.', 'B.C.', 'AD', 'BC' )
//!
//!  *  11. a 12-hour clock specifier ( 'A.M.', 'P.M.', 'AM', 'PM' )
//!
//!  *  12. a Julian date specifier ( 'JD' )
//!
//!  *  13. a day of year specifier ( '::' or '//' )
//!
//!  *  14. a period '.'
//!
//!  *  15. a dash '-'
//!
//!  *  16. a slash '/'
//!
//!  *  17. a colon ':'
//!
//!  *  18. a left parenthesis '('
//!
//!  *  19. a right parenthesis ')'
//!
//!  *  20. a single quote character (')
//!
//!  Once the next substring has been identified, its boundaries and
//!    classification are stored in the next available location in the buffer
//!    reserved for the tokenized representation of the time string.
//!
//!  The steps above are then repeated until the entire substring has been
//!    tokenized or a failure to recognize some substring occurs. If a failure
//!    occurs the location in the string is noted and a diagnostic message is
//!    created indicating the failure in the attempt to parse the string.
//!
//!  When the tokenization is finished, there will be a list of tokens from
//!    which a string can be constructed that lists the class of each token.
//!    Each class of token is represented by a single character. By placing
//!    these characters in a string a simple list of token classes is
//!    maintained. The characters used for the remainder of this discussion are
//!    listed below.
//!
//!  
//!
//! ```text
//!    Q  stands for the quote character
//!    [  stands for the left parenthesis character
//!    ]  stands for the right parenthesis character
//!    ,  stands for the comma character
//!    -  stands for the dash character
//!    .  stands for the decimal point character
//!    /  stands for the slash character
//!    :  stands for the colon character
//!    N  stands for one of the symbols A.M. or P.M.
//!    O  stands for the symbol UTC+
//!    Z  stands for a time zone such as PDT, PSD, CDT,
//!    b  stands for a block of white space (spaces or tabs)
//!    d  stands for the day of year marker (// or ::)
//!    e  stands for the era (B.C. or A.D.)
//!    j  stands for Julian date
//!    m  stands for a month
//!    o  stands for the symbol UTC-
//!    s  stands for a time system (UTC, TT, TDT, TDB)
//!    t  stands the ISO date-T-time separator
//!    w  stands for the day of the week
//!    i  stands for a sequence of digits
//!    x  stands for a character to ignore
//! ```
//!
//!  Thus the list of token classifications corresponding to
//!
//!  
//!
//! ```text
//!       1995 Jan 12 12:28:28
//! ```
//!
//!  will be
//!
//!  
//!
//! ```text
//!       ibmbibi:i:i
//! ```
//!
//!     
//! ##  Combining and Removing Tokens
//!
//!  Once an internal tokenized representation of the time string has been
//!    created, the internal representation is manipulated so that the meaning
//!    of the tokens is gradually discovered.
//!
//!  There are 3 basic operations that can be performed on the tokenized
//!    representation:
//!
//!  
//!
//! *  1. A token can be "removed" from the representation based on its
//! classification. This removal can be wholesale as in "remove all tokens
//! corresponding to the blank character", or it can be positional as in
//! "remove the last token classified as a blank."
//!
//!  *  2. A sequence of tokens can be combined into a single new token with a
//! potentially new classification. For example you might have a subsequence of
//! token classifications such as 'i.i' in the tokenized representation that
//! corresponds to an unsigned integer, a period, and another unsigned integer.
//! Under suitable circumstances this sequence 'i.i' might be replaced by 'n'
//! (for number).
//!
//!  *  3. A single token can be reclassified. For example you might have a token
//! whose classification is 'i' for 'unsigned integer' and have it reclassified
//! as an hour 'H'
//!
//!     
//! ##  Initial Token Processing
//!
//!  The first phase of processing the tokenized time discovers any UTC
//!    offsets in the input string, abbreviated months, decimal numbers, and
//!    removes white space. The process proceeds as follows:
//!
//!  
//!
//! *  1. Token sequences that represent UTC time offsets are combined to form a
//! single token with a new classification. (The character used for this new
//! kind of token is 'Z'.)
//!
//!  *  2. Months or weekdays that are followed by a period are combined to form a
//! single token (month or weekday respectively). The motivation for this
//! combination is to allow abbreviations such as "Jan." It also allows
//! strings such as "January."
//!
//!  *  3. The right most sequence of tokens of the form "i.i",
//! (integer-period-integer) or "i." (integer-period) is combined to form a
//! single token "n" (number). This combination is performed only once in the
//! token resolution process.
//!
//!  *  4. All blanks ("b") are removed from the tokenization.
//!
//!     
//! ###  Julian Dates
//!
//!  The string is now examined to see if the Julian date specifier 'JD' is
//!    present. If so the following operations are performed. If no Julian date
//!    specifier is present, the steps below are skipped and processing resumes
//!    under the section "Calendar Dates."
//!
//!  
//!
//! *  1. Any token sequence of the form '\[s]' ( left parenthesis - time system -
//! right parenthesis) is transformed to the sequence '*s*'. The '*' token is
//! then removed. This leaves just the time system (TT, TDT, TDB, or UTC)
//! specification in the tokenization.
//!
//!  * Note: Whenever a character in the token classification is replaced by '*',
//! the next step is to remove all tokens classified as '*' from the token
//! list. In the remainder of the discussion, we will not add the sentence
//! describing the removal of all asterisks. It will be implicit that the
//! asterisk is always removed after it is placed in the token list.
//!
//!  *  2. If the token sequence '\[j]' (left parenthesis - Julian date specifier -
//! right parenthesis) is present, it is replaced by '*j*'
//!
//!  *  3. If no number token, 'n', (see above) is present in the tokenization, the
//! left most integer ('i') is reclassified as a number ( 'n' ).
//!
//!  *  4. If the token sequence '-n' ( dash - number ) appears in the token list, it
//! is combined and classified as a number ('n'). This allows for the input of
//! negative Julian dates.
//!
//!  *  5. The Julian date specifier 'j' is noted and removed from the token list.
//!
//!  *  6. Any system token ('s') present in the token list is noted and removed.
//!
//!  *  7. The numeric components of the string are converted to double precision
//! values and the token list is checked for unresolved tokens. (The only thing
//! that should be in the token list at this point is a single numeric token.)
//!
//!  *  8. The parsing process halts. Either the string was successfully parsed and a
//! double precision value for the Julian date has been constructed or there
//! were unresolved tokens in the token list and a diagnostic message has been
//! created.
//!
//!     
//! ###  Calendar Dates
//!
//!  If the Julian date specifier was not present in the token list, we
//!    assume that the string and token list represents a calendar date format.
//!    One consequence of this assumption is that the dash '-' is now assumed
//!    to be just a punctuation mark and not part of a number. ISO formats are
//!    given first priority in the scheme of token resolution. Note that ISO
//!    formats do not allow the inclusion of time systems, time zones, eras, or
//!    12-hour clocks.
//!
//!  Any integer class tokens ('i') whose corresponding substrings represent
//!    integers greater than or equal to 1000 are reclassified as years ('Y').
//!
//!  
//!
//!
//!  
//! ###  ISO Formats
//!
//!  If the ISO separator token 'T' is present, the string is treated as an
//!    ISO format string. If the token list matches one of the token patterns
//!    in the left column it is transformed to the corresponding item in the
//!    right column by removing punctuation and making the indicated
//!    transformations. The ISO time string may also end with the "Z" suffix.
//!    There is no difference in the interpretation with or without the suffix.
//!
//!  
//!
//! ```text
//!    Token list      Transformation
//!    -----------     --------------
//!    Y-i-iT          YmD
//!    Y-i-iTi         YmDH
//!    Y-i-iTi:i       YmDHM
//!    Y-i-iTi:i:i     YmDHMS
//!    Y-i-iTi:i:n     YmDHMS
//!    Y-i-iTi:n       YmDHM
//!    Y-i-iTn         YmDH
//!    Y-iT            Yy
//!    Y-iTi           YyH
//!    Y-iTi:i         YyHM
//!    Y-iTi:i:i       YyHMS
//!    Y-iTi:i:n       YyHMS
//!    Y-iTi:n         YyHM
//!    Y-iTn           YyH
//!    i-i-iT          YmD
//!    i-i-iTi         YmDH
//!    i-i-iTi:i       YmDHM
//!    i-i-iTi:i:i     YmDHMS
//!    i-i-iTi:i:n     YmDHMS
//!    i-i-iTi:n       YmDHM
//!    i-i-iTn         YmDH
//!    i-iT            Yy
//!    i-iTi           YyH
//!    i-iTi:i         YyHM
//!    i-iTi:i:i       YyHMS
//!    i-iTi:i:n       YyHMS
//!    i-iTi:n         YyHM
//!    i-iTn           YyH
//!  
//!                    Y  ---  Year
//!                    m  ---  Month
//!                    D  ---  Day of Month
//!                    y  ---  Day of Year
//!                    H  ---  Hour
//!                    M  ---  Minute
//!                    S  ---  Second
//! ```
//!
//!  If the token list contains the ISO separator ('T') but the list does not
//!    match one of the patters shown above, the input string is regarded as
//!    erroneous.
//!
//!  
//!
//!
//!  
//! ###  Other Calendar Formats
//!
//!  If the ISO separator is not part of the token list, we next do what we
//!    can to recognize years and note the presence of modifiers (time zone
//!    specification, era, 12-hour clock etc.)
//!
//!  
//!
//! *  1. If a two digit integer is preceded by the quote character ('), the pair of
//! tokens is combined to a single token and reclassified as a year.
//!
//!  *  2. The following token transformations are performed:
//!
//!  ```text
//!    [e]  ---> *e* (parenthesized era to era)
//!    [w]  ---> *w* (parenthesized weekday to weekday)
//!    [N]  ---> *N* (parenthesized AM/PM to AM/PM)
//!    [Z]  ---> *Z* (parenthesized time zone to time zone)
//!    [s]  ---> *s* (parenthesized time system to time system)
//!    ie,  ---> Ye  (integer-era  to Year-era)
//! ```
//!  *  3. Eras, weekdays, AM/PM, time zones, time systems are noted and removed from
//! the token list.
//!
//!  *  4. The string is examined for redundant commas, dashes, slashes periods, etc.
//! If any are found the string is regarded as erroneous.
//!
//!     
//! ###  Built in Representations
//!
//!  Having processed the token list to this point, we check to see if what
//!    remains is one of those in a large set of immediately recognized token
//!    lists. The complete list is shown below. As in the case of ISO formats,
//!    the left item is the token list, the right item is the transformation
//!    after removing delimiters. Note that the letter 'd' stands for a
//!    day-of-year delimiter ( '//' or '::' ).
//!
//!  
//!
//! ```text
//!    Token list      Transformation
//!    -----------     --------------
//!    Y-i-it          YmD
//!    Y-i-iti         YmDH
//!    Y-i-iti:i       YmDHM
//!    Y-i-iti:i:i     YmDHMS
//!    Y-i-iti:i:n     YmDHMS
//!    Y-i-iti:n       YmDHM
//!    Y-i-itn         YmDH
//!    Y-i/            Yy
//!    Y-i/i:i         YyHM
//!    Y-i/i:i:i       YyHMS
//!    Y-i/i:i:n       YyHMS
//!    Y-i/i:n         YyHM
//!    Y-id            Yy
//!    Y-idi:i         YyHM
//!    Y-idi:i:i       YyHMS
//!    Y-idi:i:n       YyHMS
//!    Y-idi:n         YyHM
//!    Y-it            Yy
//!    Y-iti           YyH
//!    Y-iti:i         YyHM
//!    Y-iti:i:i       YyHMS
//!    Y-iti:i:n       YyHMS
//!    Y-iti:n         YyHM
//!    Y-itn           YyH
//!    Yid             Yy
//!    Yidi:i          YyHM
//!    Yidi:i:i        YyHMS
//!    Yidi:i:n        YyHMS
//!    Yidi:n          YyHM
//!    Yii             YmD
//!    Yiii            YmDH
//!    Yiii:i          YmDHM
//!    Yiii:i:i        YmDHMS
//!    Yiii:i:n        YmDHMS
//!    Yiii:n          YmDHM
//!    Yiiii           YmDHM
//!    Yiiiii          YmDHMS
//!    Yiiiin          YmDHMS
//!    Yiiin           YmDHM
//!    Yiin            YmDH
//!    Yim             YDm
//!    Yimi            YDmH
//!    Yimi:i          YDmHM
//!    Yimi:i:i        YDmHMS
//!    Yimi:i:n        YDmHMS
//!    Yimi:n          YDmHM
//!    Yimn            YDmH
//!    Yin             YmD
//!    Ymi             YmD
//!    Ymii            YmDH
//!    Ymii:i          YmDHM
//!    Ymii:i:i        YmDHMS
//!    Ymii:i:n        YmDHMS
//!    Ymii:n          YmDHM
//!    Ymin            YmDH
//!    Ymn             YmD
//!    Ynm             YDm
//!    i-Y/            yY
//!    i-Y/i:i         yYHM
//!    i-Y/i:i:i       yYHMS
//!    i-Y/i:i:n       yYHMS
//!    i-Y/i:n         yYHM
//!    i-Yd            yY
//!    i-Ydi:i         yYHM
//!    i-Ydi:i:i       yYHMS
//!    i-Ydi:i:n       yYHMS
//!    i-Ydi:n         yYHM
//!    i-i-Y           mDY
//!    i-i-Yi:i        mDYHM
//!    i-i-Yi:i:i      mDYHMS
//!    i-i-Yi:i:n      mDYHMS
//!    i-i-Yi:n        mDYHM
//!    i-i-it          YmD
//!    i-i-iti         YmDH
//!    i-i-iti:i       YmDHM
//!    i-i-iti:i:i     YmDHMS
//!    i-i-iti:i:n     YmDHMS
//!    i-i-iti:n       YmDHM
//!    i-i-itn         YmDH
//!    i-i/i:i         YyHM
//!    i-i/i:i:i       YyHMS
//!    i-i/i:i:n       YyHMS
//!    i-i/i:n         YyHM
//!    i-idi:i         YyHM
//!    i-idi:i:i       YyHMS
//!    i-idi:i:n       YyHMS
//!    i-idi:n         YyHM
//!    i-it            Yy
//!    i-iti           YyH
//!    i-iti:i         YyHM
//!    i-iti:i:i       YyHMS
//!    i-iti:i:n       YyHMS
//!    i-iti:n         YyHM
//!    i-itn           YyH
//!    i/i/Y           mDY
//!    i/i/Y/i:n       mDYHM
//!    i/i/Yi:i        mDYHM
//!    i/i/Yi:i:i      mDYHMS
//!    i/i/Yi:i:n      mDYHMS
//!    i/i/i           mDY
//!    i/i/ii:i        mDYHM
//!    i/i/ii:i:i      mDYHMS
//!    i/i/ii:i:n      mDYHMS
//!    i/i/ii:n        mDYHM
//!    i/i/ii:n        mDYHM
//!    i:i:ii-i-Y      HMSmDY
//!    i:i:ii/i/Y      HMSmDY
//!    i:i:ii/i/i      HMSmDY
//!    i:i:iimY        HMSDmY
//!    i:i:imiY        HMSmDY
//!    i:i:ni-i-Y      HMSmDY
//!    i:i:ni/i/Y      HMSmDY
//!    i:i:ni/i/i      HMSmDY
//!    i:i:nimY        HMSDmY
//!    i:i:nmiY        HMSmDY
//!    i:ii-i-Y        HMmDY
//!    i:ii/i/Y        HMmDY
//!    i:ii/i/i        HMmDY
//!    i:iimY          HMDmY
//!    i:imiY          HMmDY
//!    i:ni-i-Y        HMmDY
//!    i:ni/i/Y        HMmDY
//!    i:ni/i/i        HMmDY
//!    i:nimY          HMDmY
//!    i:nmiY          HMmDY
//!    iYd             yY
//!    iYdi:i          yYHM
//!    iYdi:i:i        yYHMS
//!    iYdi:i:n        yYHMS
//!    iYdi:n          yYHM
//!    iiY             mDY
//!    iiYi            mDYH
//!    iiYi:i          mDYHM
//!    iiYi:i:i        mDYHMS
//!    iiYi:i:n        mDYHMS
//!    iiYi:n          mDYHM
//!    iiYn            mDYH
//!    iid             Yy
//!    iidi:i          YyHM
//!    iidi:i:i        YyHMS
//!    iidi:i:n        YyHMS
//!    iidi:n          YyHM
//!    iim             YDm
//!    iimi            YDmH
//!    iimi:i          YDmHM
//!    iimi:i:i        YDmHMS
//!    iimi:i:n        YDmHMS
//!    iimi:n          YDmHM
//!    iimii           YDmHM
//!    iimiii          YDmHMS
//!    iimiin          YDmHMS
//!    iimin           YDmHM
//!    iimn            YDmH
//!    imY             DmY
//!    imYi            DmYH
//!    imYi:i          DmYHM
//!    imYi:i:i        DmYHMS
//!    imYi:i:n        DmYHMS
//!    imYi:n          DmYHM
//!    imYn            DmYH
//!    imi             YmD
//!    imi:i:iY        DmHMSY
//!    imi:i:nY        DmHMSY
//!    imi:iY          DmHMY
//!    imi:nY          DmHMY
//!    imii            YmDH
//!    imii:i          YmDHM
//!    imii:i:i        YmDHMS
//!    imii:i:n        YmDHMS
//!    imii:n          YmDHM
//!    imiii           YmDHM
//!    imiiii          YmDHMS
//!    imiiin          YmDHMS
//!    imiin           YmDHM
//!    imin            YmDH
//!    imn             YmD
//!    inY             mDY
//!    inm             YDm
//!    miY             mDY
//!    miYi            mDYH
//!    miYi:i          mDYHM
//!    miYi:i:i        mDYHMS
//!    miYi:i:n        mDYHMS
//!    miYi:n          mDYHM
//!    miYn            mDYH
//!    mii             mDY
//!    mii:i:iY        mDHMSY
//!    mii:i:nY        mDHMSY
//!    mii:iY          mDHMY
//!    mii:nY          mDHMY
//!    miii            mDYH
//!    miii:i          mDYHM
//!    miii:i:i        mDYHMS
//!    miii:i:n        mDYHMS
//!    miii:n          mDYHM
//!    miiii           mDYHM
//!    miiiii          mDYHMS
//!    miiiin          mDYHMS
//!    miiin           mDYHM
//!    miin            mDYH
//!    mnY             mDY
//!    mni             mDY
//!    nmY             DmY
//! ```
//!
//!  If the token list agrees with one of the items in the above list, the
//!    double precision value corresponding to each token is computed and the
//!    parsing process halts with success.
//!
//!  
//!
//!
//!  
//! ##  Last Resort Production Rules
//!
//!  If the token list did not match one of the built-in patterns above,
//!    several checks are performed to see if there is redundant information in
//!    the token list (duplicate time systems, eras, etc.) If any such
//!    duplicate items are located, the input string is diagnosed as erroneous.
//!
//!  Assuming that the error checks just discussed do not produce an error
//!    diagnosis, the string is processed according to the following rules:
//!
//!  
//!
//! *  1. Commas, dashes, and slashes are removed from the token list. The resulting
//! token list is then compared once more against the list of token patterns
//! above. If there is a successful match, the parsing process halts with
//! success.
//!
//!  *  2. The following list of transformations are attempted in the order indicated.
//!
//!  ```text
//!    i:i:i:n  ---> D*H*M*S (days, hours, minutes, seconds)
//!    i:i:i:i  ---> D*H*M*S (days, hours, minutes, seconds)
//!    i:i:n    ---> H*M*S   (hours, minutes, seconds)
//!    i:i:i    ---> H*M*S   (hours, minutes, seconds)
//!    i:n      ---> H*M     (hours, minutes)
//!    i:i      ---> H*M     (hours, minutes)
//! ```
//!  *  3. All colons are removed from the token list.
//!
//!  *  4. The following list of transformations are attempted in the order indicated.
//! The characters "\<" and ">" mean the transformation is performed
//! only if the token list occurs at the beginning or end respectively of the
//! token list.
//!
//!  ```text
//!    <miiH ---> mDY  (month, day, year)
//!    <mi   ---> mD   (month, day)
//!    Siim> ---> SYDm (seconds, year, day, month)
//!    im>   ---> Dm   (day, month)
//!    miY>  ---> mDY  (month, day, year)
//!    Ymi   ---> YmD  (year, month, day)
//!    Smi   ---> SmD  (seconds, month, day)
//!    Mmi   ---> MmD  (minutes, month, day)
//!    imY   ---> DmY  (day, month, year)
//!    imH   ---> DmH  (day, month, hour)
//!    Yid   ---> Yy*  (year, day-of-year)
//!    iYd   ---> yY*  (day-of-year, year)
//!    Ydi   ---> Y*y  (year, day-of-year)
//! ```
//!  *  5. The token list is now examined to determine whether any unresolved numeric
//! tokens remain. If unresolved numeric tokens are present, the input string
//! is diagnosed as erroneous. If no unresolved components remain, the token
//! list is checked for consistency. For example there can be only one of each
//! type of token, and there must be a sufficient number of tokens present to
//! unambiguously determine the epoch.
//!
//!     
//! ###  Conclusion
//!
//!  As can be surmised from the preceding discussion, it is very difficult
//!    to give a complete list of all token patterns that might yield a parsed
//!    time string. Nevertheless, we feel that the approach taken and the
//!    transformations applied will yield correct and consistent
//!    interpretations of the many ways people choose to represent time.
//!
//!  
//!
//!
//!  
//! #  Appendix D: Document Revision History
//!
//!  
//!
//!
//!  
//! ##  2021 SEP 10 by E. D. Wright and M. Costa Sitja
//!
//!  Corrected typo and updated allowed string formats tables.
//!
//!  Updates corresponding to additional interfaces from J. Diaz del Rio.
//!    Description edited to use "TT" rather than "TDT", "TDB" rather than
//!    "ET".
//!
//!  Edits to ISO time string format description.
//!
//!  
//!
//!
//!  
//! ##  2017 MAR 06 by E. D. Wright.
//!
//!  Extensive edits to document for clarity and to remove duplicate
//!    information.
//!
//!  Edited "Document Revision History" chapter to show a consistent
//!    format.
//!
//!  
//!
//!
//!  
//! ##  2015 SEP 09 by N. J. Bachman.
//!
//!  For Mice, added mention of Mice routines cspice_timdef_set and
//!    cspice_timdef_get.
//!
//!  Corrected typo in the start date for the applicable period of the
//!    Gregorian calendar when the MIXED calendar option is selected. The date
//!    was changed to October 5, 1582.
//!
//!  Corrected formatting of the Utility Routines section: the ASCII versions
//!    of this document for Mice and Icy displayed function names truncated to
//!    8 characters (the problem did not occur in the HTML versions).
//!
//!  
//!
//!
//!  
//! ##  2012 JUN 14 by E. D. Wright.
//!
//!  Edit to leapseconds description of the UT1 to UTC deviation. The correct
//!    IERS value is 0.9 seconds, the previous version of the document quoted
//!    0.7 seconds.
//!
//!  
//!
//!
//!  
//! ##  2009 APR 09 2009 by E. D. Wright, B. V. Semenov.
//!
//!  Adapted for Icy and Mice.
//!
//!  Added a note about the SPICE file identification word for LSK files.
//!
//!  
//!
//!
//!  
//! ##  2004 DEC 23 by NAIF.
//!
//!  Replaced [LDPOOL](crate::raw::ldpool) and other lower level loader routines with [FURNSH](crate::raw::furnsh)
//!    throughout the document.
//!
//!  
//!
//!
//!  
//! ##  2004 FEB 02 by NAIF.
//!
//!  Performed a spell-check on text.
//!
//!  
//!
//!
//!  
//! ##  1997 NOV 18 by E. D. Wright.
//!
//!  This edition of the TIME required reading is cast for the C version of
//!    the SPICELIB library, CSPICE.
//!
//!  
//!
//!
//!  
//! ###  CSPICE naming conventions
//!
//!  The CSPICE library is an implementation of the FORTRAN SPICELIB library
//!    in C. CSPICE is composed of C routines translated to C from FORTRAN by
//!    f2c, and a set of wrapper functions which allow a more C native
//!    interface to the f2c'd routines.
//!
//!  
//!
//! *  A routine name which ends in an underscore, "_", is an f2c translated
//! routine (pckopn_).
//!
//!  *  A routine name ending in and underscore c, "_c", is a wrapper routine
//! (mxm_c). It is strongly suggested that the user calls a wrapper routine
//! whenever available as opposed to the f2c translated counterpart.
//!
//!  *  A routine name in all capital letters ([SPKEZR](crate::raw::spkezr)) is a SPICELIB FORTRAN
//! routine.
//!
//!  *  Arguments in routine calls with the suffix "_len" or prefix "len" are
//! the lengths of the strings in the argument list.
//!
//!     
//! ##  1997 JUL 22 by NAIF.
//!
//!  This edition of TIME Required Reading documents the routine [ET2LST](crate::raw::et2lst). This
//!    routine allows user's to easily convert Ephemeris Time (Barycentric
//!    Dynamical Time) to the local solar time at a user specified longitude on
//!    the surface of an object.
//!
//!  In addition to the new routine [ET2LST](crate::raw::et2lst), we document a slight extension of
//!    the set of time strings that are recognized by the SPICE time software.
//!    This extension is documented in Appendix B.
//!
//!  
//!
//!
//!  
//! ##  1996 OCT 15 by NAIF.
//!
//!  This edition of TIME Required Reading is a substantial revision to the
//!    previous edition; this reflects a major enhancement of the SPICE time
//!    software. This version describes the new time related software that was
//!    included in version N0046 of SPICE . We also draw distinctions between
//!    the various levels of time conversion software that are available to
//!    Toolkit users.
//!
//!  The following routines are new as of version N0046.
//!
//!  
//!
//! ```text
//!    STR2ET      TSETYR      TTRANS      JUL2GR
//!    TIMOUT      TIMDEF      TPARTV      GR2JUL
//!    TPICTR      TCHCKD      TCHECK      TEXPYR
//! ```
//!
//!     
//! ##  1994 JUN 30 by NAIF.
//!
//!  This version differs substantially from the previous version of 13 April
//!    1992. Much of the description of the time software has been redone and
//!    sections added to describe how to modify time string parsing behavior
//!    and the conversion between uniform time systems.
//!
//!  
//!
//!
//!  
//! ##  1992 APR 13 by NAIF.
//!
//!  This version differs from the previous version of 10 April 1991 in that
//!    it discusses the new routine, [UNITIM](crate::raw::unitim), for converting between additive
//!    numeric time systems.
//!
//!  
//!
//!
//!     
