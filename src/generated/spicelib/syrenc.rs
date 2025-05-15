//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Rename an existing symbol
///
/// Rename an existing symbol in a character symbol table.
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
///  TABVAL   are components of the character symbol table.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TABSYM,
///  TABPTR,
///  TABVAL   are components of the character symbol table.
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
///     BOHR      -->   HYDROGEN ATOM
///     EINSTEIN  -->   SPECIAL RELATIVITY
///                     PHOTOELECTRIC EFFECT
///                     BROWNIAN MOTION
///     FERMI     -->   NUCLEAR FISSION
///     HAHN      -->   NUCLEAR FISSION
///     PAULI     -->   EXCLUSION PRINCIPLE
///                     NEUTRINO
///
///  The call,
///
///  CALL SYRENC ( 'FERMI', 'STRASSMAN', TABSYM, TABPTR, TABVAL )
///
///  modifies the contents of the symbol table to be:
///
///     BOHR      -->   HYDROGEN ATOM
///     EINSTEIN  -->   SPECIAL RELATIVITY
///                     PHOTOELECTRIC EFFECT
///                     BROWNIAN MOTION
///     HAHN      -->   NUCLEAR FISSION
///     PAULI     -->   EXCLUSION PRINCIPLE
///                     NEUTRINO
///     STRASSMAN -->   NUCLEAR FISSION
///
///
///  The next call,
///
///  CALL SYRENC ( 'HAHN', 'STRASSMAN', TABSYM, TABPTR, TABVAL )
///
///  modifies the contents of the symbol table to be:
///
///     BOHR      -->   HYDROGEN ATOM
///     EINSTEIN  -->   SPECIAL RELATIVITY
///                     PHOTOELECTRIC EFFECT
///                     BROWNIAN MOTION
///     PAULI     -->   EXCLUSION PRINCIPLE
///                     NEUTRINO
///     HAHN      -->   NUCLEAR FISSION
///
///  Note that the symbol "STRASSMAN" was deleted from the table,
///  and the symbol "HAHN" was then renamed to "STRASSMAN". If the
///  new symbol exists, it is deleted from the table before its name
///  is given to another symbol.
///
///
///  The next call,
///
///  CALL SYRENC ( 'FERMI', 'HAHN', TABSYM, TABPTR, TABVAL )
///
///  does not modify the contents of the symbol table. It signals
///  the error SPICE(NOSUCHSYMBOL) because the symbol "FERMI" does
///  not exist in the symbol table.
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
/// -    Beta Version 2.0.0, 28-DEC-1989 (HAN)
///
///         Changed the call to SYDELD to a call to SYDELC. The variable
///         TABVAL of type character was being passed to a dummy argument
///         of type double precision.
///
/// -    Beta Version 1.1.0, 17-FEB-1989 (NJB)
///
///         Declaration of the unused function SIZEC removed.
/// ```
pub fn syrenc(
    ctx: &mut SpiceContext,
    old: &str,
    new: &str,
    tabsym: CharArrayMut,
    tabptr: &mut [i32],
    tabval: CharArrayMut,
) -> crate::Result<()> {
    SYRENC(
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

//$Procedure SYRENC ( Rename an existing symbol )
pub fn SYRENC(
    OLD: &[u8],
    NEW: &[u8],
    TABSYM: CharArrayMut,
    TABPTR: &mut [i32],
    TABVAL: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut TABSYM = DummyCharArrayMut::new(TABSYM, None, LBCELL..);
    let mut TABPTR = DummyArrayMut::new(TABPTR, LBCELL..);
    let mut TABVAL = DummyCharArrayMut::new(TABVAL, None, LBCELL..);
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
        CHKIN(b"SYRENC", ctx)?;
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
        SETMSG(b"SYRENC: The symbol # is not in the symbol table.", ctx);
        ERRCH(b"#", OLD, ctx);
        SIGERR(b"SPICE(NOSUCHSYMBOL)", ctx)?;

    //
    // Are these the same symbol?
    //
    } else if fstr::ne(NEW, OLD) {
        //
        // If the new symbol already exists, delete it.
        //
        SYDELC(
            NEW,
            TABSYM.as_arg_mut(),
            TABPTR.as_slice_mut(),
            TABVAL.as_arg_mut(),
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

        SWAPAC(OLDDIM, OLDVAL, 0, NEWVAL, TABVAL.subarray_mut(1), ctx)?;

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

    CHKOUT(b"SYRENC", ctx)?;
    Ok(())
}
