//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

struct SaveVars {
    CENTRY: i32,
    LBOUND: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CENTRY: i32 = 0;
        let mut LBOUND: i32 = 0;

        CENTRY = 1900;
        LBOUND = 1969;

        Self { CENTRY, LBOUND }
    }
}

/// Time --- Expand year
///
/// Expand an abbreviated year to a full year specification.
///
/// # Required Reading
///
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  YEAR      I-O  The year of some epoch abbreviated/expanded.
/// ```
///
/// # Detailed Input
///
/// ```text
///  YEAR     is an "abbreviated year." In other words the 98 of
///           1998,  05 of 2005, etc.
/// ```
///
/// # Detailed Output
///
/// ```text
///  YEAR     is the expansion of the abbreviated year according
///           to the lower bound established in the entry point
///           TSETYR. By default if YEAR is 69 to 99, the output
///           is 1900 + the input value of YEAR. If YEAR is 0 to 68
///           the output value of YEAR is 2000 + the input value of
///           YEAR.
///
///           See the entry point TSETRY to modify this behavior.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If on input YEAR is not in the inclusive interval from
///      0 to 99, YEAR is returned unchanged.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows all of the SPICE time subsystem to handle
///  uniformly the expansion of "abbreviated" years.  (i.e. the
///  remainder after dividing the actual year by 100).
///
///  By using this routine together with the routine TSETYR you
///  can recover the actual year to associate with an abbreviation.
///
///  The default behavior is as follows
///
///     YEAR Input      YEAR Output
///     ----------      -----------
///         00              2000
///         01              2001
///          .                .
///          .                .
///          .                .
///         66              2066
///         67              2067
///         68              2068
///         69              1969
///          .                .
///          .                .
///          .                .
///         99              1999
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Demonstrate the default behavior of routine TEXPYR and then
///     modify it in order to set the lower bound of the expansion to
///     1980.
///
///     Example code begins here.
///
///
///           PROGRAM TEXPYR_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NYEARS
///           PARAMETER           ( NYEARS = 10 )
///
///     C
///     C     Local variables.
///     C
///           INTEGER               I
///           INTEGER               EXYEAR
///           INTEGER               YEARS  ( NYEARS )
///
///     C
///     C     Set the input years.
///     C
///           DATA                  YEARS /  0,  1, 68, 69, 70,
///          .                              78, 79, 80, 81, 99  /
///
///     C
///     C     Display the default behavior.
///     C
///           WRITE(*,'(A)') 'Default behavior:'
///           WRITE(*,*)
///
///           WRITE(*,'(A)') 'In  Expansion'
///           WRITE(*,'(A)') '--  ---------'
///
///           DO I=1, NYEARS
///
///              EXYEAR = YEARS(I)
///              CALL TEXPYR ( EXYEAR )
///
///              WRITE(*,'(I2.2,5X,I4)') YEARS(I), EXYEAR
///
///           END DO
///
///     C
///     C     Set up the lower bound for the expansion of abbreviated
///     C     years to 1980.
///     C
///           CALL TSETYR ( 1980 )
///
///     C
///     C     Display the new behavior.
///     C
///           WRITE(*,*)
///           WRITE(*,'(A)') 'Lower bound for expansion set to 1980:'
///           WRITE(*,*)
///
///           WRITE(*,'(A)') 'In  Expansion'
///           WRITE(*,'(A)') '--  ---------'
///
///           DO I=1, NYEARS
///
///              EXYEAR = YEARS(I)
///              CALL TEXPYR ( EXYEAR )
///
///              WRITE(*,'(I2.2,5X,I4)') YEARS(I), EXYEAR
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Default behavior:
///
///     In  Expansion
///     --  ---------
///     00     2000
///     01     2001
///     68     2068
///     69     1969
///     70     1970
///     78     1978
///     79     1979
///     80     1980
///     81     1981
///     99     1999
///
///     Lower bound for expansion set to 1980:
///
///     In  Expansion
///     --  ---------
///     00     2000
///     01     2001
///     68     2068
///     69     2069
///     70     2070
///     78     2078
///     79     2079
///     80     1980
///     81     1981
///     99     1999
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 23-SEP-2020 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         example code.
///
///         Added TIME to the list of Required Readings.
///
/// -    SPICELIB Version 2.0.0, 18-NOV-1997 (WLT)
///
///         The default century was changed from 1950-2049 to 1969-2068
///
/// -    SPICELIB Version 1.0.0, 08-APR-1996 (WLT)
/// ```
pub fn texpyr(ctx: &mut SpiceContext, year: &mut i32) {
    TEXPYR(year, ctx.raw_context());
}

//$Procedure TEXPYR ( Time --- Expand year )
pub fn TEXPYR(YEAR: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if ((*YEAR >= 100) || (*YEAR < 0)) {
        return;
    }

    *YEAR = (*YEAR + save.CENTRY);

    if (*YEAR < save.LBOUND) {
        *YEAR = (*YEAR + 100);
    }
}

/// Time --- set year expansion boundaries
///
/// Set the lower bound on the 100 year range.
///
/// # Required Reading
///
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  YEAR       I   Lower bound on the 100 year interval of expansion
/// ```
///
/// # Detailed Input
///
/// ```text
///  YEAR     is the year associated with the lower bound on all year
///           expansions computed by the SPICELIB routine TEXPYR. For
///           example if YEAR is 1980, then the range of years that can
///           be abbreviated is from 1980 to 2079.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If YEAR is less than 1, no action is taken.
/// ```
///
/// # Particulars
///
/// ```text
///  This entry point is used to set the range to which years
///  abbreviated to the last two digits will be expanded, allowing all
///  of the SPICE time subsystem routines to handle uniformly the
///  expansion those "abbreviated" years (i.e. the remainder after
///  dividing the actual year by 100.) The input supplied to this
///  routine represents the lower bound of the expansion interval. The
///  upper bound of the expansion interval is YEAR + 99.
///
///  The default expansion interval is from 1969 to 2068.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Suppose that you need to manipulate time strings and that
///     you want to treat years components in the range from 0 to 99
///     as being abbreviations for years in the range from
///     1980 to 2079 (provided that the years are not modified by
///     an ERA substring). The example code below shows how you
///     could go about this.
///
///     Use the LSK kernel below to load the leap seconds and time
///     constants required for the conversions.
///
///        naif0012.tls
///
///
///     Example code begins here.
///
///
///           PROGRAM TSETYR_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               DATELN
///           PARAMETER           ( DATELN = 11 )
///
///           INTEGER               NTSTRS
///           PARAMETER           ( NTSTRS = 7 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(DATELN)    DATE   (NTSTRS)
///           CHARACTER*(DATELN)    TIMSTR
///
///           DOUBLE PRECISION      ET
///
///           INTEGER               I
///
///     C
///     C     Assign an array of calendar dates.
///     C
///           DATA                  DATE   / '00 JAN 21',
///          .                               '01 FEB 22',
///          .                               '48 MAR 23',
///          .                               '49 APR 24',
///          .                               '79 JUL 14',
///          .                               '80 FEB 02',
///          .                               '99 DEC 31' /
///
///     C
///     C     Load the required LSK.
///     C
///           CALL FURNSH ( 'naif0012.tls' )
///
///     C
///     C     Set up the lower bound for the
///     C     expansion of abbreviated years.
///     C
///           CALL TSETYR ( 1980 )
///
///     C
///     C     Expand the years in input time strings.
///     C
///           WRITE(*,*) 'Time string    Expansion'
///           WRITE(*,*) '-----------    -----------'
///
///           DO I = 1, NTSTRS
///
///              CALL STR2ET ( DATE(I), ET )
///              CALL TIMOUT ( ET, 'YYYY MON DD', TIMSTR )
///
///              WRITE(*,*) DATE(I), '    ', TIMSTR
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Time string    Expansion
///      -----------    -----------
///      00 JAN 21      2000 JAN 21
///      01 FEB 22      2001 FEB 22
///      48 MAR 23      2048 MAR 23
///      49 APR 24      2049 APR 24
///      79 JUL 14      2079 JUL 14
///      80 FEB 02      1980 FEB 02
///      99 DEC 31      1999 DEC 31
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 23-SEP-2020 (JDR)
///
///         Fixed bug: Added check for "YEAR" to be positive in order to
///         update the lower bound for the expansion.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
///         Added TIME to the list of Required Readings. Extended
///         description in $Particulars to further describe the intended
///         use of this routine.
///
/// -    SPICELIB Version 2.0.0, 18-NOV-1997 (WLT)
///
///         The default century was change from 1950-2049 to 1969-2068.
///
/// -    SPICELIB Version 1.0.0, 08-APR-1996 (WLT)
/// ```
pub fn tsetyr(ctx: &mut SpiceContext, year: i32) {
    TSETYR(year, ctx.raw_context());
}

//$Procedure TSETYR ( Time --- set year expansion boundaries )
pub fn TSETYR(YEAR: i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if (YEAR > 0) {
        save.CENTRY = ((YEAR / 100) * 100);
        save.LBOUND = YEAR;
    }
}
