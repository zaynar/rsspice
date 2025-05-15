//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Get a free logical unit
///
/// Return the number of a free logical unit.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UNIT       O   The number of a free logical unit.
/// ```
///
/// # Detailed Output
///
/// ```text
///  UNIT     is the number of a free logical unit (also called
///           an "external unit"). If no free units are available,
///           the value of UNIT is 0.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If there are no free logical units available, the error
///      SPICE(NOFREELOGICALUNIT) is signaled and UNIT is assigned the
///      value 0.
///
///  2)  This routine obtains a logical unit number from FNDLUN, which
///      executes a Fortran INQUIRE statement. If that statement fails
///      to execute properly, FNDLUN returns a negative unit number,
///      GETLUN assigns the value 0 to UNIT, and, the error
///      SPICE(INQUIREFAILED) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  GETLUN returns the number of the first (unreserved) unit not
///  currently connected to a file. It thus frees the user from
///  having to maintain an accounting of which units are open, which
///  are closed, and which are available.
///
///  This routine is related to the routines FNDLUN, RESLUN, and
///  FRELUN. Together, these routines support coordinated usage of
///  Fortran logical units.  GETLUN (Get a free logical unit) and
///  FNDLUN (Find a free logical unit) both have the function of
///  returning a logical unit number that is not reserved or already
///  in use. The principal difference between the functionality of
///  these routines is that GETLUN both returns a status code and
///  signals an error if a free unit is not found, while FNDLUN
///  merely returns a status code.
///
///  RESLUN is used to reserve logical unit numbers, so that they will
///  not be returned by GETLUN or FNDLUN; FRELUN frees logical units
///  previously reserved via calls to RESLUN.
///
///  Logical units 5-7 are reserved by default. Other units may be
///  reserved by calling RESLUN. Once reserved, units (except 5-7) may
///  be unreserved by calling FRELUN.
///
///  To reserve logical unit numbers for special use, refer to
///  RESLUN. To make reserved units available to GETLUN or FNDLUN,
///  refer to FRELUN.
///
///  A unit returned by GETLUN does NOT automatically become a
///  reserved unit. If the user wishes to reserve a unit found by
///  GETLUN, the call to GETLUN must be followed by a call to RESLUN.
///
///  This routine obtains a logical unit number via a call to FNDLUN.
///  FNDLUN uses an INQUIRE statement; if that statement doesn't
///  execute properly, GETLUN will signal the error. This arrangement
///  allows FNDLUN to be error free.
///
///  The range of possible unit numbers returned by GETLUN is dependent
///  on the parameters MINLUN and MAXLUN, which are defined in FNDLUN.
///
///  Note that although 0 is a valid logical unit number on some
///  systems, a value of 0 returned by GETLUN indicates that no free
///  logical unit was available, rather than that logical unit 0 is
///  available.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment illustrates the use of GETLUN.
///
///     CALL GETLUN ( UNIT )
///
///     IF ( UNIT .EQ. 0 ) THEN
///        RETURN
///     END IF
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine never returns a logical unit number of 0. The
///      value 0 is used to indicate that no free logical unit was
///      found.
/// ```
///
/// # Author and Institution
///
/// ```text
///  C.A. Curzon        (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.2, 18-MAY-2010 (BVS)
///
///         Removed "C$" marker from text in the header.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (CAC) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -     Beta Version 2.0.0, 24-FEB-1989 (HAN) (NJB)
///
///          This routine has been substantially re-written so as to
///          obtain a free logical unit number via a call to FNDLUN.
///
///          If there are no free logical units available, UNIT
///          is assigned the value 0, and an error is signaled.
///
///          The "Parameters" section was added to the header.
/// ```
pub fn getlun(ctx: &mut SpiceContext, unit: &mut i32) -> crate::Result<()> {
    GETLUN(unit, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GETLUN ( Get a free logical unit )
pub fn GETLUN(UNIT: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    //
    // Spicelib functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"GETLUN", ctx)?;
    }

    //
    // Find a free logical unit, if there's one to be had.
    //
    FNDLUN(UNIT, ctx)?;

    if (*UNIT == 0) {
        //
        // There are no free units to be had.  C'est la vie.  Signal an
        // error.
        //
        SETMSG(b"No free logical units are available.", ctx);
        SIGERR(b"SPICE(NOFREELOGICALUNIT)", ctx)?;
        CHKOUT(b"GETLUN", ctx)?;
        return Ok(());
    } else if (*UNIT < 0) {
        //
        // There are no free units to be had.  In this case, we know the
        // "INQUIRE" attempted by FNDLUN failed.  Assign 0 to the unit
        // number, and signal an error.
        //

        SETMSG(b"INQUIRE iostat was #.", ctx);
        ERRINT(b"#", -*UNIT, ctx);
        SIGERR(b"SPICE(INQUIREFAILED)", ctx)?;

        *UNIT = 0;

        CHKOUT(b"GETLUN", ctx)?;
        return Ok(());
    }

    CHKOUT(b"GETLUN", ctx)?;
    Ok(())
}
