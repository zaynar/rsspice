//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Set the values associated with a symbol
///
/// Set the values of a particular symbol in a double precision
/// symbol table. If the symbol already exists, the previous values
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
///  NAME       I   Name of the symbol whose associated values are to
///                 be put into the symbol table.
///  VALUES     I   Values to be associated with the symbol NAME.
///  N          I   Number of values in VALUES.
///  TABSYM,
///  TABPTR,
///  TABVAL    I-O  Components of the symbol table.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the symbol whose associated values are
///           to be set.
///
///           If NAME has values associated with it, they are removed,
///           and the elements of VALUES become the values associated
///           with NAME. If NAME is not in the symbol table, a new
///           symbol is created, provided there is room in the symbol
///           table.
///
///  VALUES   are the new values associated with the symbol NAME.
///
///  N        is the number of elements in the VALUES array. If N < 1,
///           the symbol table is not modified.
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
///           If NAME has values associated with it, they are
///           removed, and the elements of VALUES become the
///           values associated with NAME. If NAME is not in the
///           symbol table, a new symbol is created, provided
///           there is room in the symbol table.
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
///  3)  If the addition of new values causes an overflow in the
///      value table, the error SPICE(VALUETABLEFULL) is signaled.
///
///  4)  If N < 1, the error SPICE(INVALIDARGUMENT) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This subroutine is like SYSETC, but SYPUTC allows **several**
///  values to be associated with a symbol.
///
///  If NAME has values associated with it, they are removed, and
///  the elements of VALUES become the values associated with NAME.
///  If NAME is not in the symbol table, a new symbol is created,
///  provided there is room in the symbol table.
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
///  If VALUES contains the elements,
///
///       3.17692D2
///       1.085D-1
///       1.000D-5
///
///  the call
///
///     CALL SYPUTC ( 'BODY4_POLE_RA', VALUES, 3,
///    .                               TABSYM, TABPTR, TABVAL )
///
///  modifies the contents of the symbol table to be:
///
///     BODY4_POLE_RA -->    3.17692D2
///                          1.085D-1
///                          1.000D-5
///     DELTA_T_A     -->    3.2184D1
///     K             -->    1.657D-3
///     MEAN_ANOM     -->    6.239996D0
///                          1.99096871D-7
///     ORBIT_ECC     -->    1.671D-2C
///
///  The call,
///
///     CALL SYPUTC ( 'K', VALUES, 3, TABSYM, TABPTR, TABVAL )
///
///  modifies the contents of the symbol table to be:
///
///     BODY4_POLE_RA -->    3.17692D2
///                          1.085D-1
///                          1.000D-5
///     DELTA_T_A     -->    3.2184D1
///     K             -->    3.17692D2
///                          1.085D-1
///                          1.000D-5
///     MEAN_ANOM     -->    6.239996D0
///                          1.99096871D-7
///     ORBIT_ECC     -->    1.671D-2
///
///  Note that the previous values associated with "K" have been
///  replaced by the values in VALUES.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
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
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.0.2, 06-AUG-1996 (WLT)
///
///         Fixed the error in the abstract noticed by Ian Jordan
///         at the University of Maryland, College Park.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (HAN) (NJB)
/// ```
pub fn syputd(
    ctx: &mut SpiceContext,
    name: &str,
    values: &[f64],
    n: i32,
    tabsym: CharArrayMut,
    tabptr: &mut [i32],
    tabval: &mut [f64],
) -> crate::Result<()> {
    SYPUTD(
        name.as_bytes(),
        values,
        n,
        tabsym,
        tabptr,
        tabval,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SYPUTD ( Set the values associated with a symbol )
pub fn SYPUTD(
    NAME: &[u8],
    VALUES: &[f64],
    N: i32,
    TABSYM: CharArrayMut,
    TABPTR: &mut [i32],
    TABVAL: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let VALUES = DummyArray::new(VALUES, 1..);
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
    let mut NEWSYM: i32 = 0;
    let mut NEWVAL: i32 = 0;

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
        CHKIN(b"SYPUTD", ctx)?;
    }

    //
    // Check to see if the number of values is a valid quantity.
    //
    if (N < 1) {
        SETMSG(
            b"SYPUTD: The dimension of the values array isless than one.",
            ctx,
        );
        SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        CHKOUT(b"SYPUTD", ctx)?;
        return Ok(());
    }

    //
    // How many symbols to start with?
    //
    NSYM = CARDC(TABSYM.as_arg(), ctx)?;
    NPTR = CARDI(TABPTR.as_slice(), ctx)?;
    NVAL = CARDD(TABVAL.as_slice(), ctx)?;

    //
    // Where does this symbol belong? is it already in the table?
    //
    LOCSYM = LSTLEC(NAME, NSYM, TABSYM.subarray(1));
    OLDSYM = ((LOCSYM != 0) && fstr::eq(TABSYM.get(LOCSYM), NAME));

    //
    // If the new symbol already exists, we need to know its dimension
    // to check for overflow.
    //
    if OLDSYM {
        LOCVAL = (SUMAI(TABPTR.subarray(1), (LOCSYM - 1)) + 1);
        DIMVAL = TABPTR[LOCSYM];
        NEWSYM = 0;
    } else {
        LOCVAL = (SUMAI(TABPTR.subarray(1), LOCSYM) + 1);
        DIMVAL = 0;
        NEWSYM = 1;
    }

    NEWVAL = (N - DIMVAL);

    //
    // Can we do this without overflow?
    //

    if ((NSYM + NEWSYM) > SIZEC(TABSYM.as_arg(), ctx)?) {
        SETMSG(
            b"SYPUTD: Addition of the new symbol # causes an overflow in the name table.",
            ctx,
        );
        ERRCH(b"#", NAME, ctx);
        SIGERR(b"SPICE(NAMETABLEFULL)", ctx)?;
    } else if ((NPTR + NEWSYM) > SIZEI(TABPTR.as_slice(), ctx)?) {
        SETMSG(
            b"SYPUTD: Addition of the new symbol # causes an overflow in the pointer table.",
            ctx,
        );
        ERRCH(b"#", NAME, ctx);
        SIGERR(b"SPICE(POINTERTABLEFULL)", ctx)?;
    } else if ((NVAL + NEWVAL) > SIZED(TABVAL.as_slice(), ctx)?) {
        SETMSG(
            b"SYPUTD: Addition of the new symbol # causes an overflow in the value table.",
            ctx,
        );
        ERRCH(b"#", NAME, ctx);
        SIGERR(b"SPICE(VALUETABLEFULL)", ctx)?;

    //
    // Looks like we can.
    //
    } else {
        //
        // If the symbol exists, remove the current contents and
        // change the dimension. Otherwise add the new name and
        // dimension to the name and pointer tables.
        //
        if (DIMVAL > 0) {
            REMLAD(DIMVAL, LOCVAL, TABVAL.subarray_mut(1), &mut NVAL, ctx)?;
            SCARDD(NVAL, TABVAL.as_slice_mut(), ctx)?;

            TABPTR[LOCSYM] = N;
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
                &[N],
                1,
                (LOCSYM + 1),
                TABPTR.subarray_mut(1),
                &mut NPTR,
                ctx,
            )?;
            SCARDI(NPTR, TABPTR.as_slice_mut(), ctx)?;
        }

        //
        // In either case, insert the values from the input array into
        // the value table.
        //
        INSLAD(
            VALUES.as_slice(),
            N,
            LOCVAL,
            TABVAL.subarray_mut(1),
            &mut NVAL,
            ctx,
        )?;
        SCARDD(NVAL, TABVAL.as_slice_mut(), ctx)?;
    }

    CHKOUT(b"SYPUTD", ctx)?;
    Ok(())
}
