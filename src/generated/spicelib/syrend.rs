//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Rename an existing symbol
///
/// Rename an existing symbol in a double precision symbol table.
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
///  OLD        I   Name of the symbol to be renamed.
///  NEW        I   New name of the symbol.
///  TABSYM,
///  TABPTR,
///  TABVAL    I-O  Components of the symbol table.
/// ```
///
/// # Detailed Input
///
/// ```text
///  OLD      is the name of the symbol to be renamed. If OLD is
///           not in the symbol table, the tables are not modified.
///
///  NEW      is the new name of the symbol. If the symbol NEW
///           already exists in the symbol table, it is deleted.
///           OLD is then renamed to NEW.
///
///  TABSYM,
///  TABPTR,
///  TABVAL   are components of the double precision symbol table.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TABSYM,
///  TABPTR,
///  TABVAL   are components of the double precision symbol table.
///           The values previously associated with OLD are now
///           associated with NEW. If OLD is not in the symbol
///           table, the symbol tables are not modified.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the symbol OLD is not in the symbol table, the error
///      SPICE(NOSUCHSYMBOL) is signaled.
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
///
///  The call,
///
///  CALL SYREND ( 'K', 'EB', TABSYM, TABPTR, TABVAL )
///
///  modifies the contents of the symbol table to be:
///
///     BODY4_POLE_RA -->    3.17681D2
///                          1.08D-1
///                          0.0D0
///     DELTA_T_A     -->    3.2184D1
///     EB            -->    1.657D-3
///     MEAN_ANOM     -->    6.239996D0
///                          1.99096871D-7
///     ORBIT_ECC     -->    1.671D-2
///                          1.08D-1
///                          0.0D0
///
///  The next call,
///
///  CALL SYREND ( 'EB', 'DELTA_T_A', TABSYM, TABPTR, TABVAL )
///
///  modifies the contents of the table to be:
///
///     BODY4_POLE_RA -->    3.17681D2
///                          1.08D-1
///                          0.0D0
///     DELTA_T_A     -->    1.657D-3
///     MEAN_ANOM     -->    6.239996D0
///                          1.99096871D-7
///     ORBIT_ECC     -->    1.671D-2
///                          1.08D-1
///                          0.0D0
///
///  Note that the symbol "DELTA_T_A" was deleted from the table,
///  and the symbol "EB" was then renamed to "DELTA_T_A". If the
///  new symbol exists, it is deleted from the table before its name
///  is given to another symbol.
///
///
///  The next call,
///
///  CALL SYREND ( 'K', 'EB', TABSYM, TABPTR, TABVAL )
///
///  does not modify the contents of the symbol table. It signals
///  the error SPICE(NOSUCHSYMBOL) because the symbol "K" does not
///  exist in the symbol table.
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
/// -    SPICELIB Version 1.1.0, 03-JUN-2021 (JDR)
///
///         Added IMPLICT NONE statement.
///
///         Edited the header to comply with NAIF standard. Fixed I/O type
///         of arguments TABSYM, TABPTR and TABVAL in $Brief_I/O table.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (HAN)
/// ```
///
/// # Revisions
///
/// ```text
/// -     Beta Version 1.1.0, 17-FEB-1989 (NJB)
///
///          Declaration of the unused function SIZEC removed.
/// ```
pub fn syrend(
    ctx: &mut SpiceContext,
    old: &str,
    new: &str,
    tabsym: CharArrayMut,
    tabptr: &mut [i32],
    tabval: &mut [f64],
) -> crate::Result<()> {
    SYREND(
        old.as_bytes(),
        new.as_bytes(),
        tabsym,
        tabptr,
        tabval,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SYREND ( Rename an existing symbol )
pub fn SYREND(
    OLD: &[u8],
    NEW: &[u8],
    TABSYM: CharArrayMut,
    TABPTR: &mut [i32],
    TABVAL: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut TABSYM = DummyCharArrayMut::new(TABSYM, None, LBCELL..);
    let mut TABPTR = DummyArrayMut::new(TABPTR, LBCELL..);
    let mut TABVAL = DummyArrayMut::new(TABVAL, LBCELL..);
    let mut NSYM: i32 = 0;
    let mut OLDLOC: i32 = 0;
    let mut OLDVAL: i32 = 0;
    let mut OLDDIM: i32 = 0;
    let mut NEWLOC: i32 = 0;
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
        CHKIN(b"SYREND", ctx)?;
    }

    //
    // Where was the old symbol?
    //
    NSYM = CARDC(TABSYM.as_arg(), ctx)?;
    OLDLOC = BSRCHC(OLD, NSYM, TABSYM.subarray(1));

    //
    // An overflow is simply not possible here. The only thing that can
    // go wrong is that the old symbol does not exist.
    //
    if (OLDLOC == 0) {
        SETMSG(b"SYREND: The symbol # is not in the symbol table.", ctx);
        ERRCH(b"#", OLD, ctx);
        SIGERR(b"SPICE(NOSUCHSYMBOL)", ctx)?;

    //
    // Are these the same symbol?
    //
    } else if fstr::ne(NEW, OLD) {
        //
        // If the new symbol already exists, delete it.
        //
        SYDELD(
            NEW,
            TABSYM.as_arg_mut(),
            TABPTR.as_slice_mut(),
            TABVAL.as_slice_mut(),
            ctx,
        )?;

        NSYM = CARDC(TABSYM.as_arg(), ctx)?;
        OLDLOC = BSRCHC(OLD, NSYM, TABSYM.subarray(1));

        //
        // Swap N elements at the old location with zero elements
        // at the new location.
        //
        NEWLOC = (LSTLEC(NEW, NSYM, TABSYM.subarray(1)) + 1);

        OLDVAL = (SUMAI(TABPTR.subarray(1), (OLDLOC - 1)) + 1);
        NEWVAL = (SUMAI(TABPTR.subarray(1), (NEWLOC - 1)) + 1);

        OLDDIM = TABPTR[OLDLOC];

        SWAPAD(OLDDIM, OLDVAL, 0, NEWVAL, TABVAL.subarray_mut(1), ctx)?;

        //
        // Move the name and dimension the same way.
        //
        SWAPAC(1, OLDLOC, 0, NEWLOC, TABSYM.subarray_mut(1), ctx)?;
        SWAPAI(1, OLDLOC, 0, NEWLOC, TABPTR.subarray_mut(1), ctx)?;

        if (OLDLOC < NEWLOC) {
            NEWLOC = (NEWLOC - 1);
        }

        fstr::assign(TABSYM.get_mut(NEWLOC), NEW);
    }

    CHKOUT(b"SYREND", ctx)?;
    Ok(())
}
