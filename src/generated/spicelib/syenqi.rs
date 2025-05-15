//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Enqueue a value onto a symbol
///
/// Enqueue a value onto a particular symbol in an integer
/// symbol table. If the symbol is not in the table, a new one
/// is created.
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
///  NAME       I   Name of the symbol onto which the value is
///                 enqueued.
///  VALUE      I   Value to be enqueued.
///  TABSYM,
///  TABPTR,
///  TABVAL    I-O  Components of the symbol table.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the symbol onto which the value is to
///           be enqueued. If NAME is not in the symbol table, a new
///           symbol having the value VALUE is created.
///
///  VALUE    is the value to be enqueued onto the symbol, NAME.
///           The value is inserted in the value table after the
///           last value associated with the symbol.
///
///  TABSYM,
///  TABPTR,
///  TABVAL   are the components of an integer symbol table.
///           The symbol NAME may or may not be in the symbol
///           table.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TABSYM,
///  TABPTR,
///  TABVAL   are the components of an integer symbol table.
///
///           On output, the value table contains the new value in
///           addition to the old values associated with the symbol
///           NAME. The pointer table is updated to reflect the change
///           in the dimension of the symbol.
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
///  If the symbol NAME is not in the symbol table, a new symbol is
///  created which has the value VALUE.
/// ```
///
/// # Examples
///
/// ```text
///  The contents of the symbol table are:
///
///     books   -->   5
///     erasers -->   6
///     pencils -->  12
///     pens    -->  10
///                  12
///                  24
///
///  The call,
///
///     CALL SYENQI ( 'books', 12, TABSYM, TABPTR, TABVAL )
///
///  produces the symbol table:
///
///     books   -->   5
///                  12
///     erasers -->   6
///     pencils -->  12
///     pens    -->  10
///                  12
///                  24
///
///  The next call,
///
///     CALL SYENQI ( 'desks', 23, TABSYM, TABPTR, TABVAL )
///
///  then produces the symbol table:
///
///     books   -->   5
///                  12
///     desks   -->  23
///     erasers -->   6
///     pencils -->  12
///     pens    -->  10
///                  12
///                  24
///
///  Notice that the symbol "desks" was created by the last call.
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
pub fn syenqi(
    ctx: &mut SpiceContext,
    name: &str,
    value: i32,
    tabsym: CharArrayMut,
    tabptr: &mut [i32],
    tabval: &mut [i32],
) -> crate::Result<()> {
    SYENQI(
        name.as_bytes(),
        value,
        tabsym,
        tabptr,
        tabval,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SYENQI ( Enqueue a value onto a symbol )
pub fn SYENQI(
    NAME: &[u8],
    VALUE: i32,
    TABSYM: CharArrayMut,
    TABPTR: &mut [i32],
    TABVAL: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut TABSYM = DummyCharArrayMut::new(TABSYM, None, LBCELL..);
    let mut TABPTR = DummyArrayMut::new(TABPTR, LBCELL..);
    let mut TABVAL = DummyArrayMut::new(TABVAL, LBCELL..);
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
        CHKIN(b"SYENQI", ctx)?;
    }

    //
    // How many symbols to start with?
    //
    NSYM = CARDC(TABSYM.as_arg(), ctx)?;
    NVAL = CARDI(TABVAL.as_slice(), ctx)?;

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
        SYSETI(
            NAME,
            VALUE,
            TABSYM.as_arg_mut(),
            TABPTR.as_slice_mut(),
            TABVAL.as_slice_mut(),
            ctx,
        )?;

    //
    // If it is in the table, we can't proceed unless we know that we
    // have enough room for one extra addition in the value table.
    //
    } else if (NVAL >= SIZEI(TABVAL.as_slice(), ctx)?) {
        SETMSG(b"SYENQI: The addition of the value $ to the symbol # causes an overflow in the value table.", ctx);
        ERRINT(b"$", VALUE, ctx);
        ERRCH(b"#", NAME, ctx);
        SIGERR(b"SPICE(VALUETABLEFULL)", ctx)?;

    //
    // If there's room, add the new value to the value table. Add one
    // to the dimension, and put the value in the right place.
    //
    } else {
        LOCVAL = (SUMAI(TABPTR.subarray(1), LOCSYM) + 1);

        INSLAI(&[VALUE], 1, LOCVAL, TABVAL.subarray_mut(1), &mut NVAL, ctx)?;
        SCARDI(NVAL, TABVAL.as_slice_mut(), ctx)?;

        TABPTR[LOCSYM] = (TABPTR[LOCSYM] + 1);
    }

    CHKOUT(b"SYENQI", ctx)?;
    Ok(())
}
