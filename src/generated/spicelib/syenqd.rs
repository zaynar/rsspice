//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Enqueue a value onto a symbol
///
/// Enqueue a value onto a particular symbol in a double precision
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
///  TABVAL   are the components of a double precision symbol table.
///           The symbol NAME may or may not be in the symbol
///           table.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TABSYM,
///  TABPTR,
///  TABVAL   are the components of a double precision symbol table.
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
///      DELTA_T_A -->   32.184
///      K         -->    1.657D-3
///      MEAN_ANOM -->    6.239996D0
///                       1.99096871D-7
///      ORBIT_ECC -->    1.671D-2
///
///   The call,
///
///      CALL SYENQD ( 'BODY399_POLE_RA', 0.0D0,
///     .              TABSYM, TABPTR, TABVAL    )
///
///   produces the symbol table:
///
///      BODY399_POLE_RA -->    0.0D0
///      DELTA_T_A       -->   32.184
///      K               -->    1.657D-3
///      MEAN_ANOM       -->    6.239996D0
///                             1.99096871D-7
///      ORBIT_ECC       -->    1.671D-2
///
///   Notice that the new symbol "BODY399_POLE_RA" has been created and
///   has the value 0.0D0 associated with it.
///
///   The next call,
///
///      CALL SYENQD ( 'BODY399_POLE_RA', -6.4061614D-1,
///     .               TABSYM, TABPTR, TABVAL           )
///
///      CALL SYENQD ( 'BODY399_POLE_RA', -8.386D-5,
///     .               TABSYM, TABPTR, TABVAL           )
///
///   then produces the symbol table:
///
///      BODY399_POLE_RA -->    0.0D0
///                            -6.4061614D-1
///                            -8.386D-5
///      DELTA_T_A       -->   32.184
///      K               -->    1.657D-3
///      MEAN_ANOM       -->    6.239996D0
///                             1.99096871D-7
///      ORBIT_ECC       -->    1.671D-2
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
pub fn syenqd(
    ctx: &mut SpiceContext,
    name: &str,
    value: f64,
    tabsym: CharArrayMut,
    tabptr: &mut [i32],
    tabval: &mut [f64],
) -> crate::Result<()> {
    SYENQD(
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

//$Procedure SYENQD ( Enqueue a value onto a symbol )
pub fn SYENQD(
    NAME: &[u8],
    VALUE: f64,
    TABSYM: CharArrayMut,
    TABPTR: &mut [i32],
    TABVAL: &mut [f64],
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
        CHKIN(b"SYENQD", ctx)?;
    }

    //
    // How many symbols to start with?
    //
    NSYM = CARDC(TABSYM.as_arg(), ctx)?;
    NVAL = CARDD(TABVAL.as_slice(), ctx)?;

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
        SYSETD(
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
    } else if (NVAL >= SIZED(TABVAL.as_slice(), ctx)?) {
        SETMSG(b"SYENQD: The addition of the value $ to the symbol # causes an overflow in the value table.", ctx);
        ERRDP(b"$", VALUE, ctx);
        ERRCH(b"#", NAME, ctx);
        SIGERR(b"SPICE(VALUETABLEFULL)", ctx)?;

    //
    // If there's room, add the new value to the value table. Add one
    // to the dimension, and put the value in the right place.
    //
    } else {
        LOCVAL = (SUMAI(TABPTR.subarray(1), LOCSYM) + 1);

        INSLAD(&[VALUE], 1, LOCVAL, TABVAL.subarray_mut(1), &mut NVAL, ctx)?;
        SCARDD(NVAL, TABVAL.as_slice_mut(), ctx)?;

        TABPTR[LOCSYM] = (TABPTR[LOCSYM] + 1);
    }

    CHKOUT(b"SYENQD", ctx)?;
    Ok(())
}
