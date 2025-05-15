//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Return the dimension of a symbol
///
/// Return the dimension of a particular symbol in a double precision
/// symbol table. If the symbol is not found, the function returns the
/// value zero.
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
///  NAME       I   Name of the symbol whose dimension is desired.
///  TABSYM,
///  TABPTR,
///  TABVAL     I   Components of the symbol table.
///
///  The function returns the dimension of the symbol NAME. If NAME is
///  not in the symbol table, the function returns the value zero.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the symbol whose dimension is to be
///           returned. If the symbol is not in the symbol table, the
///           function returns the value zero. This function is case
///           sensitive, NAME must match a symbol exactly.
///
///  TABSYM,
///  TABPTR,
///  TABVAL   are the components of a double precision symbol table.
///           The table may or may not contain the symbol NAME.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the dimension of the symbol NAME. The
///  dimension of a symbol is the number of values associated with that
///  symbol. If NAME is not in the symbol table, the function returns
///  the value zero.
/// ```
///
/// # Examples
///
/// ```text
///  The contents of the symbol table are:
///
///     DELTA_T_A -->   32.184
///     K         -->    1.657D-3
///     MEAN_ANOM -->    6.239996D0
///                      1.99096871D-7
///
///
///  Let NUMVAL be equal to the dimension of the symbols in the table.
///  The following code returns the values of NUMVAL indicated in the
///  table.
///
///  NUMVAL = SYDIMD ( 'MEAN_ANOM',   TABSYM, TABPTR, TABVAL )
///  NUMVAL = SYDIMD ( 'K',           TABSYM, TABPTR, TABVAL )
///  NUMVAL = SYDIMD ( 'DELTA_T_A',   TABSYM, TABPTR, TABVAL )
///  NUMVAL = SYDIMD ( 'BODY10_AXES', TABSYM, TABPTR, TABVAL )
///
///  ----SYMBOL----------NUMVAL------
///  | MEAN_ANOM    |       2       |
///  | K            |       1       |
///  | DELTA_T_A    |       1       |
///  | BODY10_AXES  |       0       |
///  --------------------------------
///
///  Note that the dimension of "BODY10_AXES" is zero. This is due to
///  the fact that "BODY10_AXES" is not in the symbol table.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 07-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Updated
///         $Brief_I/O to indicate that TABSYM, TABPTR, TABVAL are input
///         arguments.
///
/// -    SPICELIB Version 1.1.0, 17-MAY-1994 (HAN)
///
///         If the value of the function RETURN is .TRUE. upon execution of
///         this module, this function is assigned a default value of
///         either 0, 0.0D0, .FALSE., or blank depending on the type of
///         the function.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///          Comment section for permuted index source lines was added
///          following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (HAN)
/// ```
pub fn sydimd(
    ctx: &mut SpiceContext,
    name: &str,
    tabsym: CharArray,
    tabptr: &[i32],
    tabval: &[f64],
) -> crate::Result<i32> {
    let ret = SYDIMD(name.as_bytes(), tabsym, tabptr, tabval, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure SYDIMD ( Return the dimension of a symbol )
pub fn SYDIMD(
    NAME: &[u8],
    TABSYM: CharArray,
    TABPTR: &[i32],
    TABVAL: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<i32> {
    let TABSYM = DummyCharArray::new(TABSYM, None, LBCELL..);
    let TABPTR = DummyArray::new(TABPTR, LBCELL..);
    let mut SYDIMD: i32 = 0;
    let mut NSYM: i32 = 0;
    let mut LOCSYM: i32 = 0;

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
        SYDIMD = 0;
        return Ok(SYDIMD);
    } else {
        CHKIN(b"SYDIMD", ctx)?;
    }

    //
    // How many symbols to start with?
    //
    NSYM = CARDC(TABSYM.as_arg(), ctx)?;

    //
    // Is this symbol even in the table?
    //
    LOCSYM = BSRCHC(NAME, NSYM, TABSYM.subarray(1));

    //
    // If it's not in the table, return zero. Otherwise, look up
    // the dimension directly.
    //
    if (LOCSYM == 0) {
        SYDIMD = 0;
    } else {
        SYDIMD = TABPTR[LOCSYM];
    }

    CHKOUT(b"SYDIMD", ctx)?;
    Ok(SYDIMD)
}
