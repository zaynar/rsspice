//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Insert an interval into a DP window
///
/// Insert an interval into a double precision window.
///
/// # Required Reading
///
/// * [WINDOWS](crate::required_reading::windows)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LEFT,
///  RIGHT      I   Left, right endpoints of new interval.
///  WINDOW    I-O  Input, output window.
/// ```
///
/// # Detailed Input
///
/// ```text
///  LEFT,
///  RIGHT    are the left and right endpoints of the interval to be
///           inserted.
///
///  WINDOW   on input, is a SPICE window containing zero or more
///           intervals.
/// ```
///
/// # Detailed Output
///
/// ```text
///  WINDOW   on output, is the original window following the insertion
///           of the interval from LEFT to RIGHT.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If LEFT is greater than RIGHT, the error SPICE(BADENDPOINTS)
///      is signaled.
///
///  2)  If the insertion of the interval causes an excess of elements,
///      the error SPICE(WINDOWEXCESS) is signaled.
///
///  3)  The cardinality of the input WINDOW must be even. Left
///      endpoints of stored intervals must be strictly greater than
///      preceding right endpoints. Right endpoints must be greater
///      than or equal to corresponding left endpoints. Invalid window
///      data are not diagnosed by this routine and may lead to
///      unpredictable results.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine inserts the interval from LEFT to RIGHT into the
///  input window. If the new interval overlaps any of the intervals
///  in the window, the intervals are merged. Thus, the cardinality
///  of the input window can actually decrease as the result of an
///  insertion. However, because inserting an interval that is
///  disjoint from the other intervals in the window can increase the
///  cardinality of the window, the routine signals an error.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as input,
///  the compiler and supporting libraries, and the machine specific
///  arithmetic implementation.
///
///  1) The following code example demonstrates how to insert an
///     interval into an existing double precision SPICE window, and
///     how to loop over all its intervals to extract their left and
///     right points.
///
///
///     Example code begins here.
///
///
///           PROGRAM WNINSD_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///           INTEGER               WNSIZE
///           PARAMETER           ( WNSIZE = 10 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      WINDOW      ( LBCELL:WNSIZE )
///           DOUBLE PRECISION      LEFT
///           DOUBLE PRECISION      RIGHT
///
///           INTEGER               I
///
///     C
///     C     Validate the window with size WNSIZE and zero elements.
///     C
///           CALL WNVALD( WNSIZE, 0, WINDOW )
///
///     C
///     C     Insert the intervals
///     C
///     C        [ 1, 3 ]  [ 7, 11 ]  [ 23, 27 ]
///     C
///     C     into WINDOW.
///     C
///           CALL WNINSD(  1.D0,  3.D0, WINDOW )
///           CALL WNINSD(  7.D0, 11.D0, WINDOW )
///           CALL WNINSD( 23.D0, 27.D0, WINDOW )
///
///     C
///     C     Loop over the number of intervals in WINDOW, output
///     C     the LEFT and RIGHT endpoints for each interval.
///     C
///           DO I=1, WNCARD(WINDOW)
///
///              CALL WNFETD( WINDOW, I, LEFT, RIGHT )
///
///              WRITE(*,'(A,I2,2(A,F8.3),A)') 'Interval', I,
///          .                  ' [', LEFT,',',  RIGHT, ']'
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
///     Interval 1 [   1.000,   3.000]
///     Interval 2 [   7.000,  11.000]
///     Interval 3 [  23.000,  27.000]
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.4.0, 25-AUG-2021 (JDR) (NJB)
///
///         Added IMPLICIT NONE statement.
///
///         Updated to remove unnecessary lines of code in the
///         Standard SPICE error handling CHKIN statements.
///
///         Edited the header to comply to NAIF standard. Added complete
///         code example, problem statement and solution. Added entry #3 in
///         $Exceptions section.
///
///         Removed irrelevant information related to other unary window
///         routines from $Particulars section.
///
/// -    SPICELIB Version 1.3.0, 04-MAR-1993 (KRG)
///
///         There was a bug when moving the intervals in the cell
///         to the right when inserting a new interval to the left
///         of the left most interval. The incrementing in the DO
///         loop was incorrect.
///
///         The loop used to read:
///
///            DO J = I-1, CARD
///               WINDOW(J+2) = WINDOW(J)
///            END DO
///
///         which squashed everything to the right of the first interval
///         with the values of the first interval.
///
///         The loop now reads:
///
///            DO J = CARD, I-1, -1
///               WINDOW(J+2) = WINDOW(J)
///            END DO
///
///         which correctly scoots the elements in reverse order,
///         preserving their values.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.3.0, 04-MAR-1993 (KRG)
///
///         There was a bug when moving the intervals in the cell
///         to the right when inserting a new interval to the left
///         of the left most interval. the incrementing in the DO
///         loop was incorrect.
///
///         The loop used to read:
///
///            DO J = I-1, CARD
///               WINDOW(J+2) = WINDOW(J)
///            END DO
///
///         which squashed everything to the right of the first interval
///         with the values of the first interval.
///
///         The loop now reads:
///
///            DO J = CARD, I-1, -1
///               WINDOW(J+2) = WINDOW(J)
///            END DO
///
///         which correctly scoots the elements in reverse order,
///         preserving their values.
///
/// -    Beta Version 1.2.0, 27-FEB-1989 (HAN)
///
///         Due to the calling sequence and functionality changes
///         in the routine EXCESS, the method of signaling an
///         excess of elements needed to be changed.
///
/// -    Beta Version 1.1.0, 17-FEB-1989 (HAN) (NJB)
///
///         Contents of the $Required_Reading section was
///         changed from "None." to "WINDOWS".  Also, the
///         declaration of the unused variable K was removed.
/// ```
pub fn wninsd(
    ctx: &mut SpiceContext,
    left: f64,
    right: f64,
    window: &mut [f64],
) -> crate::Result<()> {
    WNINSD(left, right, window, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WNINSD ( Insert an interval into a DP window )
pub fn WNINSD(
    LEFT: f64,
    RIGHT: f64,
    WINDOW: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut WINDOW = DummyArrayMut::new(WINDOW, LBCELL..);
    let mut SIZE: i32 = 0;
    let mut CARD: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"WNINSD", ctx)?;

    //
    // Get the size and cardinality of the window.
    //
    SIZE = SIZED(WINDOW.as_slice(), ctx)?;
    CARD = CARDD(WINDOW.as_slice(), ctx)?;

    //
    // Let's try the easy cases first. No input interval? No change.
    // Signal that an error has occurred and set the error message.
    //
    if (LEFT > RIGHT) {
        SETMSG(b"Left endpoint was *. Right endpoint was *.", ctx);
        ERRDP(b"*", LEFT, ctx);
        ERRDP(b"*", RIGHT, ctx);
        SIGERR(b"SPICE(BADENDPOINTS)", ctx)?;
        CHKOUT(b"WNINSD", ctx)?;
        return Ok(());

    //
    // Empty window? Input interval later than the end of the window?
    // Just insert the interval, if there's room.
    //
    } else if ((CARD == 0) || (LEFT > WINDOW[CARD])) {
        if (SIZE >= (CARD + 2)) {
            SCARDD((CARD + 2), WINDOW.as_slice_mut(), ctx)?;
            WINDOW[(CARD + 1)] = LEFT;
            WINDOW[(CARD + 2)] = RIGHT;
        } else {
            EXCESS(2, b"window", ctx)?;
            SIGERR(b"SPICE(WINDOWEXCESS)", ctx)?;
        }

        CHKOUT(b"WNINSD", ctx)?;
        return Ok(());
    }

    //
    // Now on to the tougher cases.
    //
    // Skip intervals which lie completely to the left of the input
    // interval. (The index I will always point to the right endpoint
    // of an interval).
    //
    I = 2;

    while ((I <= CARD) && (WINDOW[I] < LEFT)) {
        I = (I + 2);
    }

    //
    // There are three ways this can go. The new interval can:
    //
    //    1) lie entirely between the previous interval and the next.
    //
    //    2) overlap the next interval, but no others.
    //
    //    3) overlap more than one interval.
    //
    // Only the first case can possibly cause an overflow, since the
    // other two cases require existing intervals to be merged.
    //

    //
    // Case (1). If there's room, move succeeding intervals back and
    // insert the new one. If there isn't room, signal an error.
    //
    if (RIGHT < WINDOW[(I - 1)]) {
        if (SIZE >= (CARD + 2)) {
            {
                let m1__: i32 = CARD;
                let m2__: i32 = (I - 1);
                let m3__: i32 = -1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    WINDOW[(J + 2)] = WINDOW[J];
                    J += m3__;
                }
            }

            SCARDD((CARD + 2), WINDOW.as_slice_mut(), ctx)?;
            WINDOW[(I - 1)] = LEFT;
            WINDOW[I] = RIGHT;
        } else {
            EXCESS(2, b"window", ctx)?;
            SIGERR(b"SPICE(WINDOWEXCESS)", ctx)?;
            CHKOUT(b"WNINSD", ctx)?;
            return Ok(());
        }

    //
    // Cases (2) and (3).
    //
    } else {
        //
        // The left and right endpoints of the new interval may or
        // may not replace the left and right endpoints of the existing
        // interval.
        //
        WINDOW[(I - 1)] = intrinsics::DMIN1(&[LEFT, WINDOW[(I - 1)]]);
        WINDOW[I] = intrinsics::DMAX1(&[RIGHT, WINDOW[I]]);

        //
        // Skip any intervals contained in the one we modified.
        // (Like I, J always points to the right endpoint of an
        // interval.)
        //
        J = (I + 2);

        while ((J <= CARD) && (WINDOW[J] <= WINDOW[I])) {
            J = (J + 2);
        }

        //
        // If the modified interval extends into the next interval,
        // merge the two. (The modified interval grows to the right.)
        //
        if ((J <= CARD) && (WINDOW[I] >= WINDOW[(J - 1)])) {
            WINDOW[I] = WINDOW[J];
            J = (J + 2);
        }

        //
        // Move the rest of the intervals forward to take up the
        // spaces left by the absorbed intervals.
        //
        while (J <= CARD) {
            I = (I + 2);
            WINDOW[(I - 1)] = WINDOW[(J - 1)];
            WINDOW[I] = WINDOW[J];
            J = (J + 2);
        }

        SCARDD(I, WINDOW.as_slice_mut(), ctx)?;
    }

    CHKOUT(b"WNINSD", ctx)?;
    Ok(())
}
