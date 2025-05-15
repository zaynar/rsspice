//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Pop a value from a particular symbol
///
/// Pop a value associated with a particular symbol in a character
/// symbol table. The first value associated with the symbol is
/// removed, and subsequent values are moved forward.
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
///  NAME       I   Name of the symbol whose associated value is to be
///                 popped.
///  TABSYM,
///  TABPTR,
///  TABVAL    I-O  Components of the symbol table.
///  VALUE      O   Value that was popped.
///  FOUND      O   .TRUE. if the symbol exists, .FALSE. otherwise.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the symbol whose associated value is to
///           be popped.
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
///           On output, the value is removed from the symbol table,
///           and the remaining values associated with the symbol are
///           moved forward in the value table. If no other values are
///           associated with the symbol, the symbol is removed from
///           the symbol table.
///
///  VALUE    is the value that was popped. This value was the first
///           value in the symbol table that was associated with the
///           symbol NAME.
///
///  FOUND    is .TRUE. if NAME is in the symbol table, otherwise it is
///           .FALSE.
/// ```
///
/// # Particulars
///
/// ```text
///  If there are no remaining values associated with the symbol
///  after VALUE has been popped, the symbol is removed from the
///  symbol table.
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
///     CALL SYPOPC ( 'EINSTEIN', TABSYM, TABPTR, TABVAL,
///    .                                  VALUE,  FOUND  )
///
///  modifies the contents of the symbol table to be:
///
///     BOHR      -->   HYDROGEN ATOM
///     EINSTEIN  -->   PHOTOELECTRIC EFFECT
///                     BROWNIAN MOTION
///     FERMI     -->   NUCLEAR FISSION
///
///  FOUND is .TRUE., and VALUE is 'SPECIAL RELATIVITY'.
///
///
///  The next call,
///
///     CALL SYPOPC ( 'FERMI', TABSYM, TABPTR, TABVAL, VALUE, FOUND )
///
///  modifies the contents of the symbol table to be:
///
///     BOHR      -->   HYDROGEN ATOM
///     EINSTEIN  -->   SPECIAL RELATIVITY
///                     PHOTOELECTRIC EFFECT
///                     BROWNIAN MOTION
///
///   FOUND is .TRUE., and VALUE is 'NUCLEAR FISSION'. Note that
///   because "FERMI" had only one value associated with it, it was
///   removed from the symbol table.
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
pub fn sypopc(
    ctx: &mut SpiceContext,
    name: &str,
    tabsym: CharArrayMut,
    tabptr: &mut [i32],
    tabval: CharArrayMut,
    value: &mut str,
    found: &mut bool,
) -> crate::Result<()> {
    SYPOPC(
        name.as_bytes(),
        tabsym,
        tabptr,
        tabval,
        fstr::StrBytes::new(value).as_mut(),
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SYPOPC ( Pop a value from a particular symbol )
pub fn SYPOPC(
    NAME: &[u8],
    TABSYM: CharArrayMut,
    TABPTR: &mut [i32],
    TABVAL: CharArrayMut,
    VALUE: &mut [u8],
    FOUND: &mut bool,
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
        CHKIN(b"SYPOPC", ctx)?;
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
    // If it's not in the table, it's definitely a problem.
    //
    if (LOCSYM == 0) {
        *FOUND = false;

    //
    // If it is in the table, we can proceed without fear of overflow.
    //
    } else {
        *FOUND = true;

        //
        // Begin by saving and removing the initial value for this
        // symbol from the value table.
        //
        LOCVAL = (SUMAI(TABPTR.subarray(1), (LOCSYM - 1)) + 1);
        fstr::assign(VALUE, TABVAL.get(LOCVAL));

        REMLAC(1, LOCVAL, TABVAL.subarray_mut(1), &mut NVAL, ctx)?;
        SCARDC(NVAL, TABVAL.as_arg_mut(), ctx)?;

        //
        // If this was the sole value for the symbol, remove the
        // symbol from the name and pointer tables. Otherwise just
        // decrement the dimension.
        //
        if (TABPTR[LOCSYM] == 1) {
            REMLAC(1, LOCSYM, TABSYM.subarray_mut(1), &mut NSYM, ctx)?;
            SCARDC(NSYM, TABSYM.as_arg_mut(), ctx)?;

            REMLAI(1, LOCSYM, TABPTR.subarray_mut(1), &mut NPTR, ctx)?;
            SCARDI(NPTR, TABPTR.as_slice_mut(), ctx)?;
        } else {
            TABPTR[LOCSYM] = (TABPTR[LOCSYM] - 1);
        }
    }

    CHKOUT(b"SYPOPC", ctx)?;
    Ok(())
}
