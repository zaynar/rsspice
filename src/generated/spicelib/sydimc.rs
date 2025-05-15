//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Return the dimension of a symbol
///
/// Return the dimension of a particular symbol in a character symbol
/// table. If the symbol is not found, the function returns the value
/// zero.
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
///  TABVAL   are the components of a character symbol table. The table
///           may or may not contain the symbol NAME.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the dimension of the symbol NAME. The
///  dimension of a symbol is the number of values associated with
///  that symbol. If NAME is not in the symbol table, the function
///  returns the value zero.
/// ```
///
/// # Examples
///
/// ```text
///  The contents of the symbol table are:
///
///     BOHR      -->   HYDROGEN ATOM
///     EINSTEIN  -->   SPECIAL RELATIVITY
///                     PHOTOELECTRIC EFFECT
///                     BROWNIAN MOTION
///     FERMI     -->   NUCLEAR FISSION
///
///
///  Perhaps we want to know how many subjects are associated with
///  certain scientists. The following code returns the values of
///  NUMSUB indicated in the table.
///
///  NUMSUB = SYDIMC ( 'EINSTEIN', TABSYM, TABPTR, TABVAL )
///  NUMSUB = SYDIMC ( 'BOHR',     TABSYM, TABPTR, TABVAL )
///  NUMSUB = SYDIMC ( 'FERMI',    TABSYM, TABPTR, TABVAL )
///  NUMSUB = SYDIMC ( 'MILLIKAN', TABSYM, TABPTR, TABVAL )
///
///
///  ----SYMBOL----------NUMSUB------
///  | EINSTEIN     |       3       |
///  | BOHR         |       1       |
///  | FERMI        |       1       |
///  | MILLIKAN     |       0       |
///  --------------------------------
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
pub fn sydimc(
    ctx: &mut SpiceContext,
    name: &str,
    tabsym: CharArray,
    tabptr: &[i32],
    tabval: CharArray,
) -> crate::Result<i32> {
    let ret = SYDIMC(name.as_bytes(), tabsym, tabptr, tabval, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure SYDIMC ( Return the dimension of a symbol )
pub fn SYDIMC(
    NAME: &[u8],
    TABSYM: CharArray,
    TABPTR: &[i32],
    TABVAL: CharArray,
    ctx: &mut Context,
) -> f2rust_std::Result<i32> {
    let TABSYM = DummyCharArray::new(TABSYM, None, LBCELL..);
    let TABPTR = DummyArray::new(TABPTR, LBCELL..);
    let mut SYDIMC: i32 = 0;
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
        SYDIMC = 0;
        return Ok(SYDIMC);
    } else {
        CHKIN(b"SYDIMC", ctx)?;
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
        SYDIMC = 0;
    } else {
        SYDIMC = TABPTR[LOCSYM];
    }

    CHKOUT(b"SYDIMC", ctx)?;
    Ok(SYDIMC)
}
