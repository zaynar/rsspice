//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Fetch the Nth symbol in the table
///
/// Fetch the Nth symbol in an integer symbol table.
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
///  TABVAL   are the components of an integer symbol table.
///           The NTH symbol may or may not be in the symbol
///           table. The symbol table is not modified by this
///           subroutine.
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
///  1)  If there is an issue while reading the components of a
///      integer symbol table, an error is signaled by a routine in
///      the call tree of this routine. This normally indicates that
///      the integer symbol table is corrupted.
/// ```
///
/// # Examples
///
/// ```text
///  The contents of the symbol table are:
///
///      books   -->   5
///      erasers -->   6
///      pencils -->  12
///      pens    -->  10
///                   12
///                   24
///
///   The calls,
///
///   CALL SYFETI (  2,  TABSYM, TABPTR, TABVAL, NAME, FOUND )
///   CALL SYFETI (  3,  TABSYM, TABPTR, TABVAL, NAME, FOUND )
///   CALL SYFETI ( -1,  TABSYM, TABPTR, TABVAL, NAME, FOUND )
///   CALL SYFETI (  6,  TABSYM, TABPTR, TABVAL, NAME, FOUND )
///
///   result in the values for NAME and FOUND:
///
///      NAME        FOUND
///      ----------  -------
///      erasers      .TRUE.
///      pencils      .TRUE.
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
pub fn syfeti(
    ctx: &mut SpiceContext,
    nth: i32,
    tabsym: CharArray,
    tabptr: &[i32],
    tabval: &[i32],
    name: &mut str,
    found: &mut bool,
) -> crate::Result<()> {
    SYFETI(
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

//$Procedure SYFETI ( Fetch the Nth symbol in the table )
pub fn SYFETI(
    NTH: i32,
    TABSYM: CharArray,
    TABPTR: &[i32],
    TABVAL: &[i32],
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
        CHKIN(b"SYFETI", ctx)?;
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

    CHKOUT(b"SYFETI", ctx)?;
    Ok(())
}
