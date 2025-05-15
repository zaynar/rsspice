//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Fetch the Nth symbol in the table
///
/// Fetch the Nth symbol in a double precision symbol table.
///
/// # Required Reading
///
/// * [SYMBOLS](crate::required_reading::symbols)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NTH        I   Index of symbol to be fetched.
///  TABSYM,
///  TABPTR,
///  TABVAL     I   Components of the symbol table.
///  NAME       O   Name of the NTH symbol in the symbol table.
///  FOUND      O   .TRUE. if the NTH symbol is in the symbol table.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NTH      is the index of the symbol to be fetched. If the NTH
///           symbol does not exist, FOUND is .FALSE.
///
///  TABSYM,
///  TABPTR,
///  TABVAL   are the components of a double precision symbol table.
///           The NTH symbol may or may not be in the symbol table.
///           The symbol table is not modified by this subroutine.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NAME     is the name of the NTH symbol in the symbol table.
///
///  FOUND    is .TRUE. if the NTH symbol is in the symbol table.
///           If the NTH symbol is not in the table, FOUND is .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If there is an issue while reading the components of a double
///      precision symbol table, an error is signaled by a routine in
///      the call tree of this routine. This normally indicates that
///      the double precision symbol table is corrupted.
/// ```
///
/// # Examples
///
/// ```text
///  The contents of the symbol table are:
///
///      DELTA_T_A -->   32.184
///      K         -->    1.657D-3
///      MEAN_ANOM -->    6.239996D0
///                       1.99096871D-7
///      ORBIT_ECC -->    1.671D-2
///
///   The calls,
///
///   CALL SYFETD (  2,  TABSYM, TABPTR, TABVAL, NAME, FOUND )
///   CALL SYFETD (  3,  TABSYM, TABPTR, TABVAL, NAME, FOUND )
///   CALL SYFETD ( -1,  TABSYM, TABPTR, TABVAL, NAME, FOUND )
///   CALL SYFETD (  5,  TABSYM, TABPTR, TABVAL, NAME, FOUND )
///
///   result in the values for NAME and FOUND:
///
///      NAME        FOUND
///      ----------  -------
///      K            .TRUE.
///      MEAN_ANOM    .TRUE.
///                  .FALSE.
///                  .FALSE.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 16-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added entry #1
///         in $Exceptions section.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (HAN) (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -     Beta Version 1.1.0, 17-FEB-1989 (NJB)
///
///          Declaration of the unused variable SUMAI removed.
/// ```
pub fn syfetd(
    ctx: &mut SpiceContext,
    nth: i32,
    tabsym: CharArray,
    tabptr: &[i32],
    tabval: &[f64],
    name: &mut str,
    found: &mut bool,
) -> crate::Result<()> {
    SYFETD(
        nth,
        tabsym,
        tabptr,
        tabval,
        fstr::StrBytes::new(name).as_mut(),
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SYFETD ( Fetch the Nth symbol in the table )
pub fn SYFETD(
    NTH: i32,
    TABSYM: CharArray,
    TABPTR: &[i32],
    TABVAL: &[f64],
    NAME: &mut [u8],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let TABSYM = DummyCharArray::new(TABSYM, None, LBCELL..);
    let mut NSYM: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SYFETD", ctx)?;
    }

    //
    // How many symbols to start with?
    //
    NSYM = CARDC(TABSYM.as_arg(), ctx)?;

    //
    // If the value of NTH is out of range, that's a problem.
    //
    if ((NTH < 1) || (NTH > NSYM)) {
        *FOUND = false;

    //
    // Otherwise, we can proceed without fear of error. Merely locate
    // and return the appropriate component from the values table.
    //
    } else {
        *FOUND = true;
        fstr::assign(NAME, TABSYM.get(NTH));
    }

    CHKOUT(b"SYFETD", ctx)?;
    Ok(())
}
