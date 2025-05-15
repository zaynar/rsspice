//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Delete a symbol from the symbol table
///
/// Delete a symbol from a character symbol table. The symbol and its
/// associated values are deleted.
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
///  NAME       I   Name of the symbol to be deleted.
///  TABSYM,
///  TABPTR,
///  TABVAL    I-O  Components of the symbol table.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the symbol to be deleted from the symbol
///           table.
///
///           If the symbol does not exist, the symbol table remains
///           unchanged. This subroutine is case sensitive. NAME must
///           the symbol exactly.
///
///  TABSYM,
///  TABPTR,
///  TABVAL   are the components of a character symbol table.
///
///           On input, the table may or may not contain the symbol
///           NAME.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TABSYM,
///  TABPTR,
///  TABVAL   are the components of a character symbol table.
///
///           On output, the symbol table no longer contains the symbol
///           NAME or its associated values. If NAME is not a symbol,
///           the components of the symbol table remain unchanged.
/// ```
///
/// # Examples
///
/// ```text
///  In the following example the subroutine SYDELC is used to delete
///  the symbol "BOHR" and its values from the symbol table.
///
///  The contents of the symbol table are:
///
///     BOHR      -->   HYDROGEN ATOM
///     EINSTEIN  -->   SPECIAL RELATIVITY
///                     PHOTOELECTRIC EFFECT
///                     BROWNIAN MOTION
///     FERMI     -->   NUCLEAR FISSION
///
///  The call
///
///     CALL SYDELC ( 'BOHR', TABSYM, TABPTR, TABVAL )
///
///  deletes the symbol "BOHR" from the symbol table. The components
///  of the symbol table on output are:
///
///     EINSTEIN  -->   SPECIAL RELATIVITY
///                     PHOTOELECTRIC EFFECT
///                     BROWNIAN MOTION
///     FERMI     -->   NUCLEAR FISSION
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
pub fn sydelc(
    ctx: &mut SpiceContext,
    name: &str,
    tabsym: CharArrayMut,
    tabptr: &mut [i32],
    tabval: CharArrayMut,
) -> crate::Result<()> {
    SYDELC(name.as_bytes(), tabsym, tabptr, tabval, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SYDELC ( Delete a symbol from the symbol table )
pub fn SYDELC(
    NAME: &[u8],
    TABSYM: CharArrayMut,
    TABPTR: &mut [i32],
    TABVAL: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut TABSYM = DummyCharArrayMut::new(TABSYM, None, LBCELL..);
    let mut TABPTR = DummyArrayMut::new(TABPTR, LBCELL..);
    let mut TABVAL = DummyCharArrayMut::new(TABVAL, None, LBCELL..);
    let mut NSYM: i32 = 0;
    let mut NPTR: i32 = 0;
    let mut NVAL: i32 = 0;
    let mut LOCSYM: i32 = 0;
    let mut LOCVAL: i32 = 0;
    let mut DIMVAL: i32 = 0;

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
        CHKIN(b"SYDELC", ctx)?;
    }

    //
    // How many symbols to start with?
    //
    NSYM = CARDC(TABSYM.as_arg(), ctx)?;
    NPTR = CARDI(TABPTR.as_slice(), ctx)?;
    NVAL = CARDC(TABVAL.as_arg(), ctx)?;

    //
    // Is this symbol even in the table?
    //
    LOCSYM = BSRCHC(NAME, NSYM, TABSYM.subarray(1));

    //
    // If it's not in the table, we're done. If it is, we can proceed
    // without fear of overflow.
    //
    if (LOCSYM > 0) {
        LOCVAL = (SUMAI(TABPTR.subarray(1), (LOCSYM - 1)) + 1);
        DIMVAL = TABPTR[LOCSYM];

        REMLAC(1, LOCSYM, TABSYM.subarray_mut(1), &mut NSYM, ctx)?;
        SCARDC(NSYM, TABSYM.as_arg_mut(), ctx)?;

        REMLAI(1, LOCSYM, TABPTR.subarray_mut(1), &mut NPTR, ctx)?;
        SCARDI(NPTR, TABPTR.as_slice_mut(), ctx)?;

        REMLAC(DIMVAL, LOCVAL, TABVAL.subarray_mut(1), &mut NVAL, ctx)?;
        SCARDC(NVAL, TABVAL.as_arg_mut(), ctx)?;
    }

    CHKOUT(b"SYDELC", ctx)?;
    Ok(())
}
