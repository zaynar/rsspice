//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Set the values associated with a symbol
///
/// Set the values of a particular symbol in an integer symbol table.
/// If the symbol already exists, the previous values associated with
/// it are removed, otherwise a new symbol is created.
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
///  NAME     is the name of the symbol whose associated values are to
///           be set.
///
///           If NAME has values associated with it, they are removed,
///           and the elements of VALUES become the values associated
///           with NAME. If NAME is not in the symbol table, a new
///           symbol is created, provided there is room in the symbol
///           table.
///
///  VALUES   are the new values associated with the symbol NAME.
///
///  N        is the number of elements in the VALUES array.
///
///  TABSYM,
///  TABPTR,
///  TABVAL   are the components of an integer symbol table.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TABSYM,
///  TABPTR,
///  TABVAL   are the components of an integer symbol table.
///
///           If NAME has values associated with it, they are removed,
///           and the elements of VALUES become the values associated
///           with NAME. If NAME is not in the symbol table, a new
///           symbol is created, provided there is room in the symbol
///           table.
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
///     books   -->   5
///     erasers -->   6
///     pencils -->  12
///                  23
///     pens    -->  10
///                  12
///                  24
///
///  If VALUES contains the elements,
///
///       12
///       24
///       36
///
///  the call
///
///     CALL SYPUTI ( 'desks', VALUES, 3, TABSYM, TABPTR, TABVAL )
///
///  modifies the contents of the symbol table to be:
///
///     books   -->   5
///     desks   -->  12
///                  24
///                  36
///     erasers -->   6
///     pencils -->  12
///                  23
///     pens    -->  10
///                  12
///                  24
///  The call,
///
///     CALL SYPUTI ( 'pens', VALUES, 3, TABSYM, TABPTR, TABVAL )
///
///  modifies the contents of the symbol table to be:
///
///     books   -->   5
///     desks   -->  12
///                  24
///                  36
///     erasers -->   6
///     pencils -->  12
///                  23
///     pens    -->  12
///                  24
///                  36
///
///  Note that the previous values associated with "pens" have been
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
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (HAN) (NJB)
/// ```
pub fn syputi(
    ctx: &mut SpiceContext,
    name: &str,
    values: &[i32],
    n: i32,
    tabsym: CharArrayMut,
    tabptr: &mut [i32],
    tabval: &mut [i32],
) -> crate::Result<()> {
    SYPUTI(
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

//$Procedure SYPUTI ( Set the values associated with a symbol )
pub fn SYPUTI(
    NAME: &[u8],
    VALUES: &[i32],
    N: i32,
    TABSYM: CharArrayMut,
    TABPTR: &mut [i32],
    TABVAL: &mut [i32],
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
        CHKIN(b"SYPUTI", ctx)?;
    }

    //
    // Check to see if the number of values is a valid quantity.
    //
    if (N < 1) {
        SETMSG(
            b"SYPUTI: The dimension of the values array isless than one.",
            ctx,
        );
        SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        CHKOUT(b"SYPUTI", ctx)?;
        return Ok(());
    }

    //
    // How many symbols to start with?
    //
    NSYM = CARDC(TABSYM.as_arg(), ctx)?;
    NPTR = CARDI(TABPTR.as_slice(), ctx)?;
    NVAL = CARDI(TABVAL.as_slice(), ctx)?;

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
            b"SYPUTI: Addition of the new symbol # causes an overflow in the name table.",
            ctx,
        );
        ERRCH(b"#", NAME, ctx);
        SIGERR(b"SPICE(NAMETABLEFULL)", ctx)?;
    } else if ((NPTR + NEWSYM) > SIZEI(TABPTR.as_slice(), ctx)?) {
        SETMSG(
            b"SYPUTI: Addition of the new symbol # causes an overflow in the pointer table.",
            ctx,
        );
        ERRCH(b"#", NAME, ctx);
        SIGERR(b"SPICE(POINTERTABLEFULL)", ctx)?;
    } else if ((NVAL + NEWVAL) > SIZEI(TABVAL.as_slice(), ctx)?) {
        SETMSG(
            b"SYPUTC: Addition of the new symbol # causes an overflow in the value table.",
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
            REMLAI(DIMVAL, LOCVAL, TABVAL.subarray_mut(1), &mut NVAL, ctx)?;
            SCARDI(NVAL, TABVAL.as_slice_mut(), ctx)?;

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
        INSLAI(
            VALUES.as_slice(),
            N,
            LOCVAL,
            TABVAL.subarray_mut(1),
            &mut NVAL,
            ctx,
        )?;
        SCARDI(NVAL, TABVAL.as_slice_mut(), ctx)?;
    }

    CHKOUT(b"SYPUTI", ctx)?;
    Ok(())
}
