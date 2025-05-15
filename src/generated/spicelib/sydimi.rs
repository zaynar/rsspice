//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Return the dimension of a symbol
///
/// Return the dimension of a particular symbol in an integer symbol
/// table. If the symbol is not found, the function returns the
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
///  TABVAL   are the components of an integer symbol table. The table
///           may or may not contain the symbol NAME.
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
///     books   -->   5
///                   8
///     erasers -->   6
///     pencils -->  12
///     pens    -->  10
///                  12
///                  24
///
///  Let NUMVAL be equal to the dimension of the symbols in the table.
///  The following code returns the values of NUMVAL indicated in the
///  table.
///
///  NUMVAL = SYDIMI ( 'books',    TABSYM, TABPTR, TABVAL )
///  NUMVAL = SYDIMI ( 'pencils',  TABSYM, TABPTR, TABVAL )
///  NUMVAL = SYDIMI ( 'pens',     TABSYM, TABPTR, TABVAL )
///  NUMVAL = SYDIMI ( 'erasers',  TABSYM, TABPTR, TABVAL )
///  NUMVAL = SYDIMI ( 'tablets',  TABSYM, TABPTR, TABVAL )
///
///
///  ----SYMBOL----------NUMVAL------
///  | books        |       2       |
///  | pencils      |       1       |
///  | pens         |       3       |
///  | erasers      |       1       |
///  | tablets      |       0       |
///  --------------------------------
///
///  Note that the dimension of "tablets" is zero. This is due to the
///  fact that "tablets" is not in the symbol table.
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
pub fn sydimi(
    ctx: &mut SpiceContext,
    name: &str,
    tabsym: CharArray,
    tabptr: &[i32],
    tabval: &[i32],
) -> crate::Result<i32> {
    let ret = SYDIMI(name.as_bytes(), tabsym, tabptr, tabval, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure SYDIMI ( Return the dimension of a symbol )
pub fn SYDIMI(
    NAME: &[u8],
    TABSYM: CharArray,
    TABPTR: &[i32],
    TABVAL: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<i32> {
    let TABSYM = DummyCharArray::new(TABSYM, None, LBCELL..);
    let TABPTR = DummyArray::new(TABPTR, LBCELL..);
    let mut SYDIMI: i32 = 0;
    let mut NSYM: i32 = 0;
    let mut LOCSYM: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling
    //
    if RETURN(ctx) {
        SYDIMI = 0;
        return Ok(SYDIMI);
    } else {
        CHKIN(b"SYDIMI", ctx)?;
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
        SYDIMI = 0;
    } else {
        SYDIMI = TABPTR[LOCSYM];
    }

    CHKOUT(b"SYDIMI", ctx)?;
    Ok(SYDIMI)
}
