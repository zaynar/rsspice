//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Create a duplicate of a symbol
///
/// Create a duplicate of a symbol within an integer symbol table.
/// If a symbol with the new name already exists, its components
/// are replaced.
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
///  NAME       I   Name of the symbol to be duplicated.
///  COPY       I   Name of the new symbol.
///  TABSYM,
///  TABPTR,
///  TABVAL    I-O  Components of the symbol table.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the symbol to be duplicated.
///
///           The components associated with NAME will be given to the
///           new symbol COPY. If NAME is not in the symbol table,
///           no duplicate symbol can be made.
///
///  COPY     is the name of the new symbol. If a symbol with the name
///           COPY already exists in the symbol table, its components
///           are replaced by the components of NAME.
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
///           On output, the symbol table contains a new symbol COPY
///           whose components are the same as the components of NAME.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the symbol NAME is not in the symbol table, the error
///      SPICE(NOSUCHSYMBOL) is signaled.
///
///  2)  If duplication of the symbol causes an overflow in the
///      name table, the error SPICE(NAMETABLEFULL) is signaled.
///
///  3)  If duplication of the symbol causes an overflow in the
///      pointer table, the error SPICE(POINTERTABLEFULL) is signaled.
///
///  4)  If duplication of the symbol causes an overflow in the
///      value table, the error SPICE(VALUETABLEFULL) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  If the symbol NAME is not in the symbol table, no duplicate symbol
///  can be made.
///
///  If the symbol COPY is already in the symbol table, its components
///  are replaced by the components of NAME.
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
///  The code,
///
///     CALL SYDUPI ( 'books', 'tablets', TABSYM, TABPTR, TABVAL )
///
///  produces the symbol table:
///
///     books   -->   5
///     erasers -->   6
///     pencils -->  12
///     pens    -->  10
///                  12
///                  24
///     tablets -->   5
///
///  The code,
///
///     CALL SYDUPC ( 'desks', 'chairs', TABSYM, TABPTR, TABVAL )
///
///  produces the error SPICE(NOSUCHSYMBOL) because the symbol
///  "desks" is not in the symbol table.
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
pub fn sydupi(
    ctx: &mut SpiceContext,
    name: &str,
    copy: &str,
    tabsym: CharArrayMut,
    tabptr: &mut [i32],
    tabval: &mut [i32],
) -> crate::Result<()> {
    SYDUPI(
        name.as_bytes(),
        copy.as_bytes(),
        tabsym,
        tabptr,
        tabval,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SYDUPI ( Create a duplicate of a symbol )
pub fn SYDUPI(
    NAME: &[u8],
    COPY: &[u8],
    TABSYM: CharArrayMut,
    TABPTR: &mut [i32],
    TABVAL: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut TABSYM = DummyCharArrayMut::new(TABSYM, None, LBCELL..);
    let mut TABPTR = DummyArrayMut::new(TABPTR, LBCELL..);
    let mut TABVAL = DummyArrayMut::new(TABVAL, LBCELL..);
    let mut NSYM: i32 = 0;
    let mut NPTR: i32 = 0;
    let mut NVAL: i32 = 0;
    let mut LOCSYM = StackArray::<i32, 2>::new(1..=2);
    let mut LOCVAL = StackArray::<i32, 2>::new(1..=2);
    let mut DIMVAL = StackArray::<i32, 2>::new(1..=2);
    let mut OLDSYM = StackArray::<bool, 2>::new(1..=2);
    let mut NEWSYM: i32 = 0;
    let mut NEWVAL: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SYDUPI", ctx)?;
    }

    //
    // How many symbols to start with?
    //
    NSYM = CARDC(TABSYM.as_arg(), ctx)?;
    NPTR = CARDI(TABPTR.as_slice(), ctx)?;
    NVAL = CARDI(TABVAL.as_slice(), ctx)?;

    //
    // Where do these symbols belong? Are they already in the table?
    //
    LOCSYM[1] = LSTLEC(NAME, NSYM, TABSYM.subarray(1));
    LOCSYM[2] = LSTLEC(COPY, NSYM, TABSYM.subarray(1));

    OLDSYM[1] = ((LOCSYM[1] != 0) && fstr::eq(TABSYM.get(LOCSYM[1]), NAME));

    OLDSYM[2] = ((LOCSYM[2] != 0) && fstr::eq(TABSYM.get(LOCSYM[2]), COPY));

    //
    // If the original symbol is not in the table, we can't make a copy.
    //
    if !OLDSYM[1] {
        SETMSG(
            b"SYDUPI: The symbol to be duplicated, #, is not in the symbol table.",
            ctx,
        );
        ERRCH(b"#", NAME, ctx);
        SIGERR(b"SPICE(NOSUCHSYMBOL)", ctx)?;

    //
    // Otherwise, we need to know the dimension, to check for overflow.
    //
    } else {
        LOCVAL[1] = (SUMAI(TABPTR.subarray(1), (LOCSYM[1] - 1)) + 1);
        DIMVAL[1] = TABPTR[LOCSYM[1]];

        //
        // If the new symbol already exists, we need to know its dimension
        // too, for the same reason.
        //
        if OLDSYM[2] {
            LOCVAL[2] = (SUMAI(TABPTR.subarray(1), (LOCSYM[2] - 1)) + 1);
            DIMVAL[2] = TABPTR[LOCSYM[2]];
            NEWSYM = 0;
        } else {
            LOCVAL[2] = (SUMAI(TABPTR.subarray(1), LOCSYM[2]) + 1);
            DIMVAL[2] = 0;
            NEWSYM = 1;
        }

        NEWVAL = (DIMVAL[1] - DIMVAL[2]);

        //
        // Can we make a copy without overflow?
        //
        if ((NSYM + NEWSYM) > SIZEC(TABSYM.as_arg(), ctx)?) {
            SETMSG(
                b"SYDUPI: Duplication of the symbol # causes an overflow in the name table.",
                ctx,
            );
            ERRCH(b"#", NAME, ctx);
            SIGERR(b"SPICE(NAMETABLEFULL)", ctx)?;
        } else if ((NPTR + NEWSYM) > SIZEI(TABPTR.as_slice(), ctx)?) {
            SETMSG(
                b"SYDUPI: Duplication of the symbol # causes an overflow in the pointer table.",
                ctx,
            );
            ERRCH(b"#", NAME, ctx);
            SIGERR(b"SPICE(POINTERTABLEFULL)", ctx)?;
        } else if ((NVAL + NEWVAL) > SIZEI(TABVAL.as_slice(), ctx)?) {
            SETMSG(
                b"SYDUPI: Duplication of the symbol # causes an overflow in the value table.",
                ctx,
            );
            ERRCH(b"#", NAME, ctx);
            SIGERR(b"SPICE(VALUETABLEFULL)", ctx)?;

        //
        // Looks like we can.
        //
        } else {
            //
            // If the copy exists, remove the current contents and
            // change the dimension. Otherwise add the new name and
            // dimension to the name and pointer tables.
            //
            if (DIMVAL[2] > 0) {
                REMLAI(DIMVAL[2], LOCVAL[2], TABVAL.subarray_mut(1), &mut NVAL, ctx)?;
                SCARDI(NVAL, TABVAL.as_slice_mut(), ctx)?;

                TABPTR[LOCSYM[2]] = DIMVAL[1];

                if (LOCVAL[1] > LOCVAL[2]) {
                    LOCVAL[1] = (LOCVAL[1] - DIMVAL[2]);
                }
            } else {
                INSLAC(
                    CharArray::from_ref(COPY),
                    1,
                    (LOCSYM[2] + 1),
                    TABSYM.subarray_mut(1),
                    &mut NSYM,
                    ctx,
                )?;
                SCARDC(NSYM, TABSYM.as_arg_mut(), ctx)?;

                INSLAI(
                    DIMVAL.subarray(1),
                    1,
                    (LOCSYM[2] + 1),
                    TABPTR.subarray_mut(1),
                    &mut NPTR,
                    ctx,
                )?;
                SCARDI(NPTR, TABPTR.as_slice_mut(), ctx)?;
            }

            //
            // In either case, allocate space for the new symbol values,
            // and copy them in one by one. (INSLAx won't work if the
            // copy is earlier in the table than the original.)
            //
            for I in intrinsics::range(NVAL, LOCVAL[2], -1) {
                TABVAL[(I + DIMVAL[1])] = TABVAL[I];
            }

            if (LOCVAL[1] > LOCVAL[2]) {
                LOCVAL[1] = (LOCVAL[1] + DIMVAL[1]);
            }

            for I in 0..=(DIMVAL[1] - 1) {
                TABVAL[(LOCVAL[2] + I)] = TABVAL[(LOCVAL[1] + I)];
            }

            SCARDI((NVAL + DIMVAL[1]), TABVAL.as_slice_mut(), ctx)?;
        }
    }

    CHKOUT(b"SYDUPI", ctx)?;
    Ok(())
}
