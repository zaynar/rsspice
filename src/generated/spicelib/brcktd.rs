//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Bracket a d.p. value within an interval
///
/// Bracket a double precision number. That is, given a number and an
/// acceptable interval, make sure that the number is contained in the
/// interval. (If the number is already in the interval, leave it
/// alone. If not, set it to the nearest endpoint of the interval.)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NUMBER     I   Number to be bracketed.
///  END1       I   One of the bracketing endpoints for NUMBER.
///  END2       I   The other bracketing endpoint for NUMBER.
///
///  The function returns the bracketed number.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NUMBER   is the number to be bracketed. That is, the
///           value of NUMBER is constrained to lie in the
///           interval bounded by END1 and END2.
///
///  END1,
///  END2     are the lower and upper bounds for NUMBER. The
///           order is not important.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the bracketed number. That is NUMBER, if it
///  was already in the interval provided. Otherwise the returned
///  value is the nearest bound of the interval.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides a shorthand notation for code fragments
///  like the following:
///
///     IF ( END1 .LT. END2 ) THEN
///        IF      ( NUMBER .LT. END1 ) THEN
///           NUMBER = END1
///        ELSE IF ( NUMBER .GT. END2 ) THEN
///           NUMBER = END2
///        END IF
///     ELSE
///        IF      ( NUMBER .LT. END2 ) THEN
///           NUMBER = END2
///        ELSE IF ( NUMBER .GT. END1 ) THEN
///           NUMBER = END1
///        END IF
///     END IF
///
///  which occur frequently during the processing of program inputs.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) The following code example illustrates the operation of
///     BRCKTD.
///
///     Example code begins here.
///
///
///           PROGRAM BRCKTD_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           DOUBLE PRECISION        BRCKTD
///
///     C
///     C     Local parameters.
///     C
///           INTEGER                 LISTSZ
///           PARAMETER             ( LISTSZ = 4  )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION        END1    ( LISTSZ )
///           DOUBLE PRECISION        END2    ( LISTSZ )
///           DOUBLE PRECISION        NUMBER  ( LISTSZ )
///
///           INTEGER                 I
///
///     C
///     C     Set the values for the example.
///     C
///           DATA                    END1   /  1.D0,   1.D0,
///          .                                 10.D0, -10.D0  /
///           DATA                    END2   / 10.D0,  10.D0,
///          .                                -10.D0,  -1.D0  /
///           DATA                    NUMBER / -1.D0,  29.D0,
///          .                                  3.D0,   3.D0  /
///
///
///           WRITE(*,'(A)') ' Number  End1   End2   Bracketed'
///           WRITE(*,'(A)') ' ------  -----  -----  ---------'
///
///           DO I = 1, LISTSZ
///
///              WRITE(*,'(3F7.1,F11.1)') NUMBER(I), END1(I), END2(I),
///          .                 BRCKTD ( NUMBER(I), END1(I), END2(I) )
///
///           END DO
///
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Number  End1   End2   Bracketed
///      ------  -----  -----  ---------
///        -1.0    1.0   10.0        1.0
///        29.0    1.0   10.0       10.0
///         3.0   10.0  -10.0        3.0
///         3.0  -10.0   -1.0       -1.0
///
///
///  2) The following code example illustrates a typical use for
///     BRCKTD: force a star magnitude limit to be within a range.
///     Note that this code assumes that the user provided value
///     is a valid double precision number.
///
///
///     Example code begins here.
///
///
///           PROGRAM BRCKTD_EX2
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           DOUBLE PRECISION        BRCKTD
///
///     C
///     C     Local parameters.
///     C
///           INTEGER                 KWDSZ
///           PARAMETER             ( KWDSZ = 30   )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(KWDSZ)       USRIN
///
///           DOUBLE PRECISION        MAGLIN
///           DOUBLE PRECISION        MAGLOK
///
///     C
///     C     Prompt the user for the star magnitude.
///     C
///           CALL PROMPT ( 'Enter star magnitude: ', USRIN )
///
///     C
///     C     Convert the user input to double precision.
///     C
///           CALL PRSDP ( USRIN, MAGLIN )
///
///     C
///     C     Star magnitude must be in the range 0-10.
///     C
///           MAGLOK = BRCKTD ( MAGLIN, 0.D0, 10.D0 )
///
///     C
///     C     Display confirmation message.
///     C
///           IF ( MAGLIN .NE. MAGLOK ) THEN
///
///              WRITE(*,'(A,F4.1,A)') 'Provided star magnitude ',
///          .                 MAGLIN, ' is out of range (0-10).'
///
///           ELSE
///
///              WRITE(*,'(A,F4.1,A)') 'Provided star magnitude ',
///          .                 MAGLIN, ' is in range (0-10).'
///
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using '10.1' as user provided input, the output was:
///
///
///     Enter star magnitude: 10.1
///     Provided star magnitude 10.1 is out of range (0-10).
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 08-AUG-2021 (JDR) (BVS)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code examples based on existing code fragments.
///
///         Updated code fragment in $Particulars to show that the
///         order of endpoints is not important.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 1.1.0, 30-DEC-1988 (WLT)
///
///         The routine was modified so that the order of the endpoints
///         of the bracketing interval is not needed. The routine now
///         determines which is the left endpoint and which is the
///         right and acts appropriately.
/// ```
pub fn brcktd(number: f64, end1: f64, end2: f64) -> f64 {
    let ret = BRCKTD(number, end1, end2);
    ret
}

//$Procedure BRCKTD ( Bracket a d.p. value within an interval )
pub fn BRCKTD(NUMBER: f64, END1: f64, END2: f64) -> f64 {
    let mut BRCKTD: f64 = 0.0;

    //
    // What else is there to say?
    //
    if (END1 < END2) {
        BRCKTD = intrinsics::DMAX1(&[END1, intrinsics::DMIN1(&[END2, NUMBER])]);
    } else {
        BRCKTD = intrinsics::DMAX1(&[END2, intrinsics::DMIN1(&[END1, NUMBER])]);
    }

    BRCKTD
}
