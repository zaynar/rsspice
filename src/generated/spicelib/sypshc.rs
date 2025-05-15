//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Push a value onto a particular symbol
///
/// Push a value onto a particular symbol in a character symbol table.
/// The previous value(s) associated with the symbol is extended at
/// the front. A new symbol is created if necessary.
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
///  NAME       I   Name of the symbol onto which the value is to be
///                 pushed.
///  VALUE      I   Value that is to be pushed onto the symbol NAME.
///  TABSYM,
///  TABPTR,
///  TABVAL    I-O  Components of the symbol table.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the symbol onto which the value is to
///           be pushed.
///
///           If NAME is not in the symbol table, a new symbol is
///           created.
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
///           The value is added to the symbol table at the front of
///           the previous value(s) associated with the symbol NAME. If
///           NAME is not originally in the symbol table, a new symbol
///           is created.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the addition of the new value to the symbol table causes an
///      overflow in the value table, the error SPICE(VALUETABLEFULL)
///      is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  If the symbol NAME is not in the symbol table, a new symbol
///  is created.
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
///     PAULI     -->   EXCLUSION PRINCIPLE
///
///  The call,
///
///     CALL SYPSHC ( 'PAULI', 'NEUTRINO', TABSYM, TABPTR, TABVAL )
///
///  modifies the contents of the symbol table to be:
///
///     BOHR      -->   HYDROGEN ATOM
///     EINSTEIN  -->   SPECIAL RELATIVITY
///                     PHOTOELECTRIC EFFECT
///                     BROWNIAN MOTION
///     FERMI     -->   NUCLEAR FISSION
///     PAULI     -->   NEUTRINO
///                     EXCLUSION PRINCIPLE
///
///  The next call,
///
///     CALL SYPSHC ( 'MILLIKAN', 'PHOTOELECTRIC EFFECT',
///    .               TABSYM,     TABPTR,                TABVAL )
///
///  modifies the contents of the symbol table to be:
///
///     BOHR      -->   HYDROGEN ATOM
///     EINSTEIN  -->   SPECIAL RELATIVITY
///                     PHOTOELECTRIC EFFECT
///                     BROWNIAN MOTION
///     FERMI     -->   NUCLEAR FISSION
///     MILLIKAN  -->   PHOTOELECTRIC EFFECT
///     PAULI     -->   NEUTRINO
///                     EXCLUSION PRINCIPLE
///
///  Note that a new symbol "MILLIKAN" was created by the last call.
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
pub fn sypshc(
    ctx: &mut SpiceContext,
    name: &str,
    value: &str,
    tabsym: CharArrayMut,
    tabptr: &mut [i32],
    tabval: CharArrayMut,
) -> crate::Result<()> {
    SYPSHC(
        name.as_bytes(),
        value.as_bytes(),
        tabsym,
        tabptr,
        tabval,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SYPSHC ( Push a value onto a particular symbol )
pub fn SYPSHC(
    NAME: &[u8],
    VALUE: &[u8],
    TABSYM: CharArrayMut,
    TABPTR: &mut [i32],
    TABVAL: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut TABSYM = DummyCharArrayMut::new(TABSYM, None, LBCELL..);
    let mut TABPTR = DummyArrayMut::new(TABPTR, LBCELL..);
    let mut TABVAL = DummyCharArrayMut::new(TABVAL, None, LBCELL..);
    let mut NSYM: i32 = 0;
    let mut NVAL: i32 = 0;
    let mut LOCSYM: i32 = 0;
    let mut LOCVAL: i32 = 0;
    let mut OLDSYM: bool = false;

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
        CHKIN(b"SYPSHC", ctx)?;
    }

    //
    // How many symbols to start with?
    //
    NSYM = CARDC(TABSYM.as_arg(), ctx)?;
    NVAL = CARDC(TABVAL.as_arg(), ctx)?;

    //
    // Where does this symbol belong? Is it already in the table?
    //
    LOCSYM = LSTLEC(NAME, NSYM, TABSYM.subarray(1));
    OLDSYM = ((LOCSYM != 0) && fstr::eq(TABSYM.get(LOCSYM), NAME));

    //
    // If it's not already in the table, use SET to create a brand new
    // symbol.
    //
    if !OLDSYM {
        SYSETC(
            NAME,
            VALUE,
            TABSYM.as_arg_mut(),
            TABPTR.as_slice_mut(),
            TABVAL.as_arg_mut(),
            ctx,
        )?;

    //
    // If it is in the table, we can't proceed unless we know that we
    // have enough room for one extra addition in the value table.
    //
    } else if (NVAL >= SIZEC(TABVAL.as_arg(), ctx)?) {
        SETMSG(b"SYPSHC: The addition of the value $ to the symbol # causes an overflow in the value table.", ctx);
        ERRCH(b"$", VALUE, ctx);
        ERRCH(b"#", NAME, ctx);
        SIGERR(b"SPICE(VALUETABLEFULL)", ctx)?;

    //
    // If there's room, add the new value to the value table. Add one
    // to the dimension, and put the value in the right place.
    //
    } else {
        LOCVAL = (SUMAI(TABPTR.subarray(1), (LOCSYM - 1)) + 1);

        INSLAC(
            CharArray::from_ref(VALUE),
            1,
            LOCVAL,
            TABVAL.subarray_mut(1),
            &mut NVAL,
            ctx,
        )?;
        SCARDC(NVAL, TABVAL.as_arg_mut(), ctx)?;

        TABPTR[LOCSYM] = (TABPTR[LOCSYM] + 1);
    }

    CHKOUT(b"SYPSHC", ctx)?;
    Ok(())
}
