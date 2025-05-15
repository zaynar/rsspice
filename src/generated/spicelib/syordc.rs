//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Order the components of a single symbol
///
/// Order the components of a single symbol in a character symbol
/// table. The components are ordered according to the ASCII collating
/// sequence.
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
///  TABVAL   are the components of a character symbol table.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TABSYM,
///  TABPTR,
///  TABVAL   are the components of a character symbol table.
///
///           On output, the components of the symbol are sorted
///           according to ASCII collating sequence. If NAME is not in
///           the symbol table, the symbol table is not modified.
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
///     BOHR      -->   HYDROGEN ATOM
///     EINSTEIN  -->   SPECIAL RELATIVITY
///                     PHOTOELECTRIC EFFECT
///                     BROWNIAN MOTION
///     FERMI     -->   NUCLEAR FISSION
///
///  The call,
///
///     CALL SYORDC ( 'EINSTEIN', TABSYM, TABPTR, TABVAL )
///
///  modifies the contents of the symbol table to be:
///
///     BOHR      -->   HYDROGEN ATOM
///     EINSTEIN  -->   BROWNIAN MOTION
///                     PHOTOELECTRIC EFFECT
///                     SPECIAL RELATIVITY
///     FERMI     -->   NUCLEAR FISSIONC
///
///
///  Note that the call,
///
///     CALL SYORDC ( 'MAXWELL', TABSYM, TABPTR, TABVAL )
///
///  will not modify the symbol table because the symbol "MAXWELL" is
///  not in the symbol table.
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
pub fn syordc(
    ctx: &mut SpiceContext,
    name: &str,
    tabsym: CharArray,
    tabptr: &[i32],
    tabval: CharArrayMut,
) -> crate::Result<()> {
    SYORDC(name.as_bytes(), tabsym, tabptr, tabval, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SYORDC ( Order the components of a single symbol )
pub fn SYORDC(
    NAME: &[u8],
    TABSYM: CharArray,
    TABPTR: &[i32],
    TABVAL: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let TABSYM = DummyCharArray::new(TABSYM, None, LBCELL..);
    let TABPTR = DummyArray::new(TABPTR, LBCELL..);
    let mut TABVAL = DummyCharArrayMut::new(TABVAL, None, LBCELL..);
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
        CHKIN(b"SYORDC", ctx)?;
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

        SHELLC(TABPTR[LOCSYM], TABVAL.subarray_mut(LOCVAL));
    }

    CHKOUT(b"SYORDC", ctx)?;
    Ok(())
}
