//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Set the value associated with a symbol
///
/// Set the value of a particular symbol in a double precision symbol
/// table. If the symbol already exists, the previous values
/// associated with it are removed, otherwise a new symbol is created.
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
///                 set.
///  VALUE      I   Associated value of the symbol NAME.
///  TABSYM,
///  TABPTR,
///  TABVAL    I-O  Components of the symbol table.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the symbol whose associated value is to
///           be set.
///
///           If NAME has values associated with it, they are removed,
///           and VALUE becomes the only value associated with NAME. If
///           NAME is not in the symbol table, a new symbol is created,
///           provided there is room in the symbol table.
///
///  VALUE    is the new value associated with the symbol NAME.
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
///           If NAME has values associated with it, they are removed,
///           and VALUE becomes the only value associated with NAME. If
///           NAME is not in the symbol table, a new symbol is created,
///           provided there is room in the symbol table.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the addition of a new symbol causes an overflow in the
///      name table, the error SPICE(NAMETABLEFULL) is signaled.
///
///  2)  If the addition of a new symbol causes an overflow in the
///      pointer table, the error SPICE(POINTERTABLEFULL) is signaled.
///
///  3)  If the addition of a new symbol causes an overflow in the
///      value table, the error SPICE(VALUETABLEFULL) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  If NAME has values associated with it, they are removed, and VALUE
///  becomes the only value associated with NAME. If NAME is not in the
///  symbol table, a new symbol is created, provided there is room in
///  the symbol table.
/// ```
///
/// # Examples
///
/// ```text
///  The contents of the symbol table are:
///
///     DELTA_T_A -->    3.2184D1
///     K         -->    1.657D-3
///     MEAN_ANOM -->    6.239996D0
///                      1.99096871D-7
///     ORBIT_ECC -->    1.671D-2
///
///  The call,
///
///  CALL SYSETD ( 'ORBIT_ECC', 1.67125D-2, TABSYM, TABPTR, TABVAL )
///
///  modifies the contents of the symbol table to be:
///
///     DELTA_T_A -->    3.2184D1
///     K         -->    1.657D-3
///     MEAN_ANOM -->    6.239996D0
///                      1.99096871D-7
///     ORBIT_ECC -->    1.67125D-2
///
///  Note that the previous value associated with the symbol
///  "ORBIT_ECC" has been deleted, and now the value 1.67125D-2
///  is associated with the symbol.
///
///
///  The next call,
///
///  CALL SYSETD ( 'EB', 1.671D-2, TABSYM, TABPTR, TABVAL )
///
///  modifies the contents of the symbol table to be:
///
///     DELTA_T_A -->    3.2184D1
///     EB        -->    1.671D-2
///     K         -->    1.657D-3
///     MEAN_ANOM -->    6.239996D0
///                      1.99096871D-7
///     ORBIT_ECC -->    1.67125D-2
///
///  Note that the new symbol "EB" was created by the last call.
///  A new symbol is created only if there is room in the symbol
///  table.
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
/// -    SPICELIB Version 1.1.0, 17-JUN-2021 (JDR)
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
pub fn sysetd(
    ctx: &mut SpiceContext,
    name: &str,
    value: f64,
    tabsym: CharArrayMut,
    tabptr: &mut [i32],
    tabval: &mut [f64],
) -> crate::Result<()> {
    SYSETD(
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

//$Procedure SYSETD ( Set the value associated with a symbol )
pub fn SYSETD(
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
    let mut NPTR: i32 = 0;
    let mut NVAL: i32 = 0;
    let mut LOCSYM: i32 = 0;
    let mut LOCVAL: i32 = 0;
    let mut DIMVAL: i32 = 0;
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
        CHKIN(b"SYSETD", ctx)?;
    }

    //
    // How many symbols to start with?
    //
    NSYM = CARDC(TABSYM.as_arg(), ctx)?;
    NPTR = CARDI(TABPTR.as_slice(), ctx)?;
    NVAL = CARDD(TABVAL.as_slice(), ctx)?;

    //
    // Where does this symbol belong? Is it already in the table?
    //
    LOCSYM = LSTLEC(NAME, NSYM, TABSYM.subarray(1));
    OLDSYM = ((LOCSYM != 0) && fstr::eq(TABSYM.get(LOCSYM), NAME));

    //
    // If it's already in the table, there's no chance of overflow.
    // Leave the name where it is. Remove all but one of the existing
    // values, replacing that with the new value. And set the dimension
    // to one.
    //
    if OLDSYM {
        LOCVAL = (SUMAI(TABPTR.subarray(1), (LOCSYM - 1)) + 1);
        DIMVAL = TABPTR[LOCSYM];

        if (DIMVAL > 1) {
            REMLAD((DIMVAL - 1), LOCVAL, TABVAL.subarray_mut(1), &mut NVAL, ctx)?;
            SCARDD(NVAL, TABVAL.as_slice_mut(), ctx)?;
        }

        TABPTR[LOCSYM] = 1;
        TABVAL[LOCVAL] = VALUE;

    //
    // Otherwise, we can't proceed unless we know that we have enough
    // room for one extra addition in all three tables.
    //
    } else if (NSYM >= SIZEC(TABSYM.as_arg(), ctx)?) {
        SETMSG(
            b"SYSETD: Addition of the new symbol # causes an overflow in the name table.",
            ctx,
        );
        ERRCH(b"#", NAME, ctx);
        SIGERR(b"SPICE(NAMETABLEFULL)", ctx)?;
    } else if (NPTR >= SIZEI(TABPTR.as_slice(), ctx)?) {
        SETMSG(
            b"SYSETD: Addition of the new symbol # causes an overflow in the pointer table.",
            ctx,
        );
        ERRCH(b"#", NAME, ctx);
        SIGERR(b"SPICE(POINTERTABLEFULL)", ctx)?;
    } else if (NVAL >= SIZED(TABVAL.as_slice(), ctx)?) {
        SETMSG(
            b"SYSETD: Addition of the new symbol # causes an overflow in the value table.",
            ctx,
        );
        ERRCH(b"#", NAME, ctx);
        SIGERR(b"SPICE(VALUETABLEFULL)", ctx)?;

    //
    // If there's room, add the new name to the name table. Give the
    // symbol dimension one, and put the value in the right place.
    //
    } else {
        INSLAC(
            CharArray::from_ref(NAME),
            1,
            (LOCSYM + 1),
            TABSYM.subarray_mut(1),
            &mut NSYM,
            ctx,
        )?;
        SCARDC(NSYM, TABSYM.as_arg_mut(), ctx)?;

        INSLAI(
            &[1],
            1,
            (LOCSYM + 1),
            TABPTR.subarray_mut(1),
            &mut NPTR,
            ctx,
        )?;
        SCARDI(NPTR, TABPTR.as_slice_mut(), ctx)?;

        LOCVAL = (SUMAI(TABPTR.subarray(1), LOCSYM) + 1);

        INSLAD(&[VALUE], 1, LOCVAL, TABVAL.subarray_mut(1), &mut NVAL, ctx)?;
        SCARDD(NVAL, TABVAL.as_slice_mut(), ctx)?;
    }

    CHKOUT(b"SYSETD", ctx)?;
    Ok(())
}
