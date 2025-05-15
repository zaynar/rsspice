//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Order the components of a single symbol
///
/// Order the components of a single symbol in a double precision
/// symbol table. The components are sorted in increasing order.
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
///  NAME       I   Name of the symbol whose components are to be
///                 ordered.
///  TABSYM,
///  TABPTR,
///  TABVAL    I-O  Components of the symbol table.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the symbol whose components are to be
///           ordered.
///
///  TABSYM,
///  TABPTR,
///  TABVAL   are the components of a double precision symbol table.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TABSYM,
///  TABPTR,
///  TABVAL   are the components of a double precision symbol table.
///
///           On output, Tte components of the symbol are sorted in
///           increasing order. If NAME is not in the symbol table, the
///           symbol table is not modified.
/// ```
///
/// # Particulars
///
/// ```text
///  If the symbol NAME is not in the symbol table, the symbol table
///  is not modified.
/// ```
///
/// # Examples
///
/// ```text
///  The contents of the symbol table are:
///
///     BODY4_POLE_RA -->    3.17681D2
///                          1.08D-1
///                          0.0D0
///     DELTA_T_A     -->    3.2184D1
///     K             -->    1.657D-3
///     MEAN_ANOM     -->    6.239996D0
///                          1.99096871D-7
///     ORBIT_ECC     -->    1.671D-2
///
///  The call,
///
///     CALL SYORDD ( 'BODY4_POLE_RA', TABSYM, TABPTR, TABVAL )
///
///  modifies the contents of the symbol table to be:
///
///     BODY4_POLE_RA -->    0.0D0
///                          1.08D-1
///                          3.17681D2
///     DELTA_T_A     -->    3.2184D1
///     K             -->    1.657D-3
///     MEAN_ANOM     -->    6.239996D0
///                          1.99096871D-7
///     ORBIT_ECC     -->    1.671D-2
///
///  Note that the call,
///
///     CALL SYORDD ( 'BODY4_PRIME', TABSYM, TABPTR, TABVAL )
///
///  will not modify the symbol table because the symbol "BODY4_PRIME"
///  is not in the symbol table.
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
/// -    SPICELIB Version 1.1.0, 08-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (HAN)
/// ```
pub fn syordd(
    ctx: &mut SpiceContext,
    name: &str,
    tabsym: CharArray,
    tabptr: &[i32],
    tabval: &mut [f64],
) -> crate::Result<()> {
    SYORDD(name.as_bytes(), tabsym, tabptr, tabval, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SYORDD ( Order the components of a single symbol )
pub fn SYORDD(
    NAME: &[u8],
    TABSYM: CharArray,
    TABPTR: &[i32],
    TABVAL: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let TABSYM = DummyCharArray::new(TABSYM, None, LBCELL..);
    let TABPTR = DummyArray::new(TABPTR, LBCELL..);
    let mut TABVAL = DummyArrayMut::new(TABVAL, LBCELL..);
    let mut NSYM: i32 = 0;
    let mut LOCSYM: i32 = 0;
    let mut LOCVAL: i32 = 0;
    let mut N: i32 = 0;

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
        CHKIN(b"SYORDD", ctx)?;
    }

    //
    // How many symbols?
    //
    NSYM = CARDC(TABSYM.as_arg(), ctx)?;

    //
    // Is this symbol even in the table?
    //
    LOCSYM = BSRCHC(NAME, NSYM, TABSYM.subarray(1));

    //
    // If so, sort the components in place.
    //
    if (LOCSYM > 0) {
        LOCVAL = (SUMAI(TABPTR.subarray(1), (LOCSYM - 1)) + 1);
        N = TABPTR[LOCSYM];

        SHELLD(TABPTR[LOCSYM], TABVAL.subarray_mut(LOCVAL));
    }

    CHKOUT(b"SYORDD", ctx)?;
    Ok(())
}
