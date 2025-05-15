//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NDELIM: i32 = 5;
const DELIMS: &[u8; NDELIM as usize] = &fstr::extend_const::<{ NDELIM as usize }>(b".:-, ");
const MXPART: i32 = 9999;
const PARTLN: i32 = 5;
const MXCOEF: i32 = 100000;
const MXNFLD: i32 = 10;
const DPLEN: i32 = 30;
const MXTSYS: i32 = 2;
const MSGLEN: i32 = 320;
const KVNMLN: i32 = 32;
const MXCOFA: i32 = (3 * MXCOEF);
const NAMLEN: i32 = 80;
const NKITEM: i32 = 9;
const COFIDX: i32 = 1;
const NFLIDX: i32 = 4;
const OFFIDX: i32 = 5;
const MODIDX: i32 = 6;
const DELIDX: i32 = 7;
const SYSIDX: i32 = 9;

struct SaveVars {
    BVLMSG: Vec<u8>,
    NFDMSG: Vec<u8>,
    NUMMSG: Vec<u8>,
    NAMLST: ActualCharArray,
    LB: StackArray<i32, 9>,
    UB: StackArray<i32, 9>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BVLMSG = vec![b' '; MSGLEN as usize];
        let mut NFDMSG = vec![b' '; MSGLEN as usize];
        let mut NUMMSG = vec![b' '; MSGLEN as usize];
        let mut NAMLST = ActualCharArray::new(NAMLEN, 1..=NKITEM);
        let mut LB = StackArray::<i32, 9>::new(1..=NKITEM);
        let mut UB = StackArray::<i32, 9>::new(1..=NKITEM);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"SCLK01_COEFFICIENTS"),
                Val::I(3),
                Val::I(MXCOFA),
                Val::C(b"SCLK_PARTITION_START"),
                Val::I(1),
                Val::I(MXPART),
                Val::C(b"SCLK_PARTITION_END"),
                Val::I(1),
                Val::I(MXPART),
                Val::C(b"SCLK01_N_FIELDS"),
                Val::I(1),
                Val::I(1),
                Val::C(b"SCLK01_OFFSETS"),
                Val::I(1),
                Val::I(MXNFLD),
                Val::C(b"SCLK01_MODULI"),
                Val::I(1),
                Val::I(MXNFLD),
                Val::C(b"SCLK01_OUTPUT_DELIM"),
                Val::I(1),
                Val::I(1),
                Val::C(b"SCLK01_KERNEL_ID"),
                Val::I(1),
                Val::I(1),
                Val::C(b"SCLK01_TIME_SYSTEM"),
                Val::I(0),
                Val::I(1),
            ]
            .into_iter();
            for I in intrinsics::range(1, NKITEM, 1) {
                fstr::assign(NAMLST.get_mut(I), clist.next().unwrap().into_str());
                LB[I] = clist.next().unwrap().into_i32();
                UB[I] = clist.next().unwrap().into_i32();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        fstr::assign(&mut NFDMSG, b"# not found. Did you load the SCLK kernel?");
        fstr::assign(&mut NUMMSG, b"# values found for #: valid range is #:#.");
        fstr::assign(&mut BVLMSG, b"Invalid value found for #:  #.");

        Self {
            BVLMSG,
            NFDMSG,
            NUMMSG,
            NAMLST,
            LB,
            UB,
        }
    }
}

/// SCLK look up, type 1
///
/// Look up type 1 SCLK kernel data.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
/// * [SCLK](crate::required_reading::sclk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  ENTRY POINTS
///  --------  ---  --------------------------------------------------
///  NAME       I   SCLD01, SCLI01
///  SC         I   SCLD01, SCLI01
///  MAXNV      I   SCLD01, SCLI01
///  N          O   SCLD01, SCLI01
///  IVAL       O   SCLI01
///  DVAL       O   SCLD01
///  MXCOEF     P   SCLD01, SCLI01
///  MXPART     P   SCLD01, SCLI01
///  MXNFLD     P   SCLD01, SCLI01
///  NDELIM     P   SCLI01
///  MXTSYS     P   SCLI01
/// ```
///
/// # Detailed Input
///
/// ```text
///  See entry points SCLI01, SCLD01.
/// ```
///
/// # Detailed Output
///
/// ```text
///  See entry points SCLI01, SCLD01.
/// ```
///
/// # Parameters
///
/// ```text
///  See the INCLUDE file sclk.inc for descriptions and values
///  of the global parameters used by this routine and
///  its entry points.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If SCLU01 is called directly, the error SPICE(BOGUSENTRY) is
///      signaled.
///
///  See entry points SCLI01, SCLD01 for descriptions of exceptions
///  specific to those routines.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is a utility whose purpose is to localize error
///  checking for type 1 SCLK kernel pool lookups in a single place.
///
///  SLCU01 exists solely as an umbrella routine in which the
///  variables for its entry points are declared.  SCLU01 should never
///  be called directly.
/// ```
///
/// # Examples
///
/// ```text
///  See entry points SCLI01, SCLD01.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  SCLU01 handles lookups of type 1 SCLK data only.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.0.0, 01-DEC-2021 (NJB) (JDR)
///
///         New checks on item sizes have been added: sizes are now
///         compared against upper bounds as well as lower bounds.
///         Previously only lower bounds were used.
///
///         Bug fix: corrected index error in SCLD01 modulus range check.
///         Corrected comments about count checks in SCLI01. Made
///         cosmetic changes to code and comments in both SCLD01 and
///         SCLI01, and in this routine. Deleted unused parameter MXNCFF.
///
///         Edited the umbrella routine and all its entry points headers
///         to comply with NAIF standard.
///
/// -    SPICELIB Version 2.3.0, 05-FEB-2008 (NJB)
///
///         Values of parameters
///
///            MXCOEF, MXPART, MXNFLD, NDELIM, MXTSYS
///
///         are now provided by the INCLUDE file sclk.inc.
///
/// -    SPICELIB Version 2.2.0, 20-NOV-2006 (NJB) (EDW)
///
///         Entry points SCLI01 and SCLD01 were update to use kernel pool
///         fetch routines GIPOOL and GDPOOL respectively. Formerly these
///         entry points called the deprecated routine RTPOOL.
///
///         All headers have been updated to remove warnings about memory
///         corruption that could occur due to use of RTPOOL.
///
///         Header references to LDPOOL were replaced with references to
///         FURNSH.
///
/// -    SPICELIB Version 2.1.0, 19-OCT-1992 (NJB)
///
///         Entry points SCLI01 and SCLD01 were updated to fix a bug:
///         if a kernel pool lookup fails, the number of elements returned
///         N is now set to zero.
///
/// -    SPICELIB Version 2.0.0, 17-APR-1992 (NJB) (WLT)
///
///         Entry point SCLI01 was updated to handle a time
///         system specification for the `parallel' time system
///         in the SCLK kernel. Comment section for permuted index
///         source lines was added following the header.
///
/// -    SPICELIB Version 1.0.0, 06-SEP-1990 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.1.0, 19-OCT-1992 (NJB)
///
///         Entry points SCLI01 and SCLD01 were updated to fix a bug:
///         if a kernel pool lookup fails, the number of elements returned
///         N is now set to zero. Formerly, these routines returned
///         whatever value was returned by RTPOOL.  RTPOOL, however,
///         does not set N to zero when the data item requested from it
///         is not found.
///
/// -    SPICELIB Version 2.0.0, 17-APR-1992 (NJB) (WLT)
///
///         Entry point SCLI01 was updated to handle a time
///         system specification for the `parallel' time system
///         in the SCLK kernel. The update consists of these
///         changes:
///
///            -- The parameter MXTSYS is now defined.
///
///            -- The local saved variable NAMLST has been expanded
///               to include the name SCLK01_TIME_SYSTEM
///
///            -- The local saved variable LB has been expanded to
///               include the lower bound for the number of returned
///               values when SCLK01_TIME_SYSTEM_nn is looked up in
///               the kernel pool.
///
///            -- SCLI01 checks the value returned by RTPOOL when
///               SCLK01_TIME_SYSTEM_nn is looked up to verify that
///               it is within the range [1, MXTSYS].
///
///         Also, a comment section for permuted index source lines was
///         added following the header.
/// ```
pub fn sclu01(
    ctx: &mut SpiceContext,
    name: &str,
    sc: i32,
    maxnv: i32,
    n: i32,
    ival: &[i32],
    dval: &[f64],
) -> crate::Result<()> {
    SCLU01(name.as_bytes(), sc, maxnv, n, ival, dval, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SCLU01 ( SCLK look up, type 1 )
pub fn SCLU01(
    NAME: &[u8],
    SC: i32,
    MAXNV: i32,
    N: i32,
    IVAL: &[i32],
    DVAL: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // MXCOFA is the maximum size of the coefficient array.
    //

    //
    // COFIDX is the index of the coefficient name in NAMLST.
    //

    //
    // NFLIDX is the index of the SCLK field count in NAMLST.
    //

    //
    // OFFIDX is the index of the SCLK offsets in NAMLST.
    //

    //
    // MODIDX is the index of the SCLK moduli in NAMLST.
    //

    //
    // DELIDX is the index of the delimiter code name in NAMLST.  If
    // the declaration of NAMLST or assignment of values to NAMLST
    // changes, this parameter value may have to change.
    //

    //
    // SYSIDX is the index of the time system in NAMLST.
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Names of type 1 SCLK items and lower bounds on the number of
    // associated values.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SCLU01", ctx)?;

    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    CHKOUT(b"SCLU01", ctx)?;
    Ok(())
}

/// SCLK lookup of integer data, type 1
///
/// Look up integer type 1 SCLK data from the kernel pool.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
/// * [SCLK](crate::required_reading::sclk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NAME,
///  SC         I   Name of kernel data item, NAIF spacecraft ID code.
///  MAXNV      I   Maximum number of integer values to return.
///  N          O   Number of values actually returned.
///  IVAL       O   Returned integer values.
///  MXNFLD     P   Maximum number of fields in an SCLK string.
///  NDELIM     P   Maximum number of delimiter codes.
///  MXTSYS     P   Maximum number of supported parallel time systems.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME,
///  SC       are, respectively, a name and a NAIF integer code
///           of a spacecraft that together define the name of a
///           requested kernel data item. NAME is the full name
///           as it appears in the SCLK kernel, except that it
///           lacks the final underscore and spacecraft integer
///           code (actually, the negative of the spacecraft
///           code).  This routine combines NAME and SC to
///           make up the appropriate kernel variable name.
///
///           For example, to look up data associated with the
///           name
///
///              SCLK01_N_FIELDS_77
///
///           you would supply NAME as
///
///              SCLK01_N_FIELDS
///
///           and SC as -77.
///
///
///  MAXNV    is the maximum number of values to return.  MAXNV
///           is used to prevent SCLI01 from writing past the end
///           of the supplied array IVAL.
/// ```
///
/// # Detailed Output
///
/// ```text
///  N        is the number of values actually returned.
///
///  IVAL     is an array containing the requested integer
///           kernel data item.
/// ```
///
/// # Parameters
///
/// ```text
///  MXNFLD   is an upper bound on the number of fields in a
///           SCLK string.
///
///  NDELIM   is the number of delimiter codes.
///
///  MXTSYS   is the maximum number of supported parallel time
///           systems that SCLK values may be mapped to or from.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If item specified by NAME and SC is not found in the kernel
///      pool, and if the presence of the item is required, the error
///      SPICE(KERNELVARNOTFOUND) is signaled. The output arguments
///      are not modified.
///
///      If the specified item is not required, the output argument N
///      will take the value 0, and the output argument IVAL is not
///      modified.
///
///  2)  If the item specified by NAME and SC is found but does not
///      have numeric type, the error SPICE(BADKERNELVARTYPE) is
///      signaled.
///
///  3)  This routine can check certain data for validity. If any of
///      these items have invalid values, the error
///      SPICE(VALUEOUTOFRANGE) is signaled. The output arguments are
///      not modified. The values in question are:
///
///         -  The number of fields of a SCLK string
///         -  The number of delimiter codes
///         -  The output delimiter code
///         -  The time system code
///
///  4)  If the dimension of the requested item exceeds MAXNV, the
///      error SPICE(ARRAYTOOSMALL) is signaled.
///
///  5)  If the dimension of the requested item is outside of the
///      limits for that item, the error SPICE(INVALIDSIZE) is
///      signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The purpose of this routine is to localize error checking for
///  lookups of type 1 SCLK kernel pool data. This routine handles
///  lookups of integer data.
/// ```
///
/// # Examples
///
/// ```text
///  1)  To get the number of SCLK fields for the Galileo spacecraft
///      clock, you can use the code fragment below:
///
///         C
///         C     Load the SCLK kernel in question. We use a
///         C     made-up name for the kernel file; you would use
///         C     the actual name of your kernel file instead if you
///         C     were to carry out this procedure.
///         C
///               CALL FURNSH ( 'SAMPLE_GLL_SCLK.KER' )
///
///               SC   = -77
///               NAME = 'SCLK01_N_FIELDS'
///
///               CALL SCLI01 ( NAME, SC, MXNFLD, N, NFIELD )
///
///
///      After this subroutine call, NFIELD has the value 4.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  SCLI01 assumes that a SCLK kernel appropriate to the
///      spacecraft identified by SC has been loaded.
///
///  2)  SCLI01 handles lookups of type 1 SCLK data only.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.0.0, 01-DEC-2021 (NJB) (JDR)
///
///         New checks on item sizes have been added: sizes are now
///         compared against upper bounds as well as lower bounds.
///         Previously only lower bounds were used.
///
///         Corrected comments about count checks. Made cosmetic changes
///         to code and comments.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.2.0, 20-NOV-2006 (NJB) (EDW)
///
///         Routine was updated to use GIPOOL instead of RTPOOL. Header
///         has been updated to remove warnings about memory corruption and
///         to document exception handling for output buffer overflow
///         errors.
///
///         Header references to LDPOOL were replaced with references to
///         FURNSH.
///
/// -    SPICELIB Version 2.1.0, 19-OCT-1992 (NJB)
///
///         This entry point was updated to fix a bug: if a kernel pool
///         lookup fails, the number of elements returned N is now set to
///         zero.
///
/// -    SPICELIB Version 2.0.0, 17-APR-1992 (NJB) (WLT)
///
///         SCLI01 was updated to handle a time system specification for
///         the `parallel' time system in the SCLK kernel. Some
///         corrections and other minor enhancements were made to the
///         header. Comment section for permuted index source lines was
///         added following the header.
///
/// -    SPICELIB Version 1.0.0, 06-SEP-1990 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.1.0, 19-OCT-1992 (NJB)
///
///         This entry point was updated to fix a bug: if a kernel pool
///         lookup fails, the number of elements returned N is now set to
///         zero. Formerly, this routine returned whatever value was
///         returned by RTPOOL.  RTPOOL, however, does not set N to zero
///         when the data item requested from it is not found.
///
/// -    SPICELIB Version 2.0.0, 17-APR-1992 (NJB) (WLT)
///
///         Entry point SCLI01 was updated to handle a time
///         system specification for the `parallel' time system
///         in the SCLK kernel. The update consists of these
///         changes:
///
///            -- The parameter MXTSYS is now defined.
///
///            -- The local saved variable NAMLST has been expanded
///               to include the name SCLK01_TIME_SYSTEM
///
///            -- The local saved variable LB has been expanded to
///               include the lower bound for the number of returned
///               values when SCLK01_TIME_SYSTEM_nn is looked up in
///               the kernel pool.
///
///            -- SCLI01 checks the value returned by RTPOOL when
///               SCLK01_TIME_SYSTEM_nn is looked up to verify that
///               it is within the range [1, MXTSYS].
///
///         Also, a comment section for permuted index source lines was
///         added following the header.
///
///         The $Exceptions header section was updated accordingly.
///
///         Some corrections and other minor enhancements were made to the
///         header.
/// ```
pub fn scli01(
    ctx: &mut SpiceContext,
    name: &str,
    sc: i32,
    maxnv: i32,
    n: &mut i32,
    ival: &mut [i32],
) -> crate::Result<()> {
    SCLI01(name.as_bytes(), sc, maxnv, n, ival, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SCLI01 ( SCLK lookup of integer data, type 1 )
pub fn SCLI01(
    NAME: &[u8],
    SC: i32,
    MAXNV: i32,
    N: &mut i32,
    IVAL: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut IVAL = DummyArrayMut::new(IVAL, 1..);
    let mut ERRMSG = [b' '; MSGLEN as usize];
    let mut TMPNAM = [b' '; NAMLEN as usize];
    let mut TYPE = [b' '; 1 as usize];
    let mut I: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SCLI01", ctx)?;

    //
    // Form the name of the kernel pool data item, and do the lookup.
    // Note that eventually we should use a kernel pool lookup entry
    // that allows us to specify the maximum number of entries that
    // can be returned.
    //
    fstr::assign(&mut TMPNAM, NAME);

    SUFFIX(b"_#", 0, &mut TMPNAM);
    REPMI(&TMPNAM.clone(), b"#", -SC, &mut TMPNAM, ctx);

    //
    // Make sure we have enough room for the item in our output
    // array. Look up the dimension of the item.
    //
    DTPOOL(&TMPNAM, &mut FOUND, N, &mut TYPE, ctx)?;

    if (*N > MAXNV) {
        SETMSG(
            b"Item # for SCLK # has size # but output array has size #.",
            ctx,
        );
        ERRCH(b"#", &TMPNAM, ctx);
        ERRINT(b"#", SC, ctx);
        ERRINT(b"#", *N, ctx);
        ERRINT(b"#", MAXNV, ctx);
        SIGERR(b"SPICE(ARRAYTOOSMALL)", ctx)?;
        CHKOUT(b"SCLI01", ctx)?;
        return Ok(());
    }

    //
    // Check the data type of the variable.
    //
    if (FOUND && fstr::ne(&TYPE, b"N")) {
        SETMSG(
            b"Kernel variable # for spacecraft clock # does not have numeric type.",
            ctx,
        );
        ERRCH(b"#", &TMPNAM, ctx);
        ERRINT(b"#", SC, ctx);
        SIGERR(b"SPICE(BADKERNELVARTYPE)", ctx)?;
        CHKOUT(b"SCLI01", ctx)?;
        return Ok(());
    }

    GIPOOL(&TMPNAM, 1, MAXNV, N, IVAL.as_slice_mut(), &mut FOUND, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SCLI01", ctx)?;
        return Ok(());
    }

    //
    // Make sure we found what we were looking for, if the item
    // is required.
    //
    if !FOUND {
        //
        // Currently, the only item that is NOT required is the time
        // system specification.  In any case, no values will be returned.
        //
        *N = 0;

        if fstr::eq(NAME, save.NAMLST.get(SYSIDX)) {
            CHKOUT(b"SCLI01", ctx)?;
            return Ok(());
        } else {
            SETMSG(&save.NFDMSG, ctx);
            ERRCH(b"#", &TMPNAM, ctx);
            SIGERR(b"SPICE(KERNELVARNOTFOUND)", ctx)?;
            CHKOUT(b"SCLI01", ctx)?;
            return Ok(());
        }
    }

    //
    // Now we must check that the number of returned values is in the
    // appropriate range. We test for the following conditions:
    //
    //    - The SCLK field count kernel variable is a scalar.
    //
    //    - The delimiter code kernel variable is a scalar.
    //
    //    - The time system code kernel variable, if present, is a
    //      scalar. At this point in the code, the variable is known
    //      to be present.
    //
    // Note that the kernel pool doesn't allow a kernel variable to
    // not have associated values. Checks against the lower bound 1 are
    // done just to simplify the code.
    //
    // See if the input name is in the list of items we know about.
    // If it is, perform the bound checks that apply.
    //
    I = ISRCHC(NAME, NKITEM, save.NAMLST.as_arg());

    if (I != 0) {
        if ((*N < save.LB[I]) || (*N > save.UB[I])) {
            REPMI(&save.NUMMSG, b"#", *N, &mut ERRMSG, ctx);
            REPMC(&ERRMSG.clone(), b"#", &TMPNAM, &mut ERRMSG);
            REPMI(&ERRMSG.clone(), b"#", save.LB[I], &mut ERRMSG, ctx);
            REPMI(&ERRMSG.clone(), b"#", save.UB[I], &mut ERRMSG, ctx);
            SETMSG(&ERRMSG, ctx);
            SIGERR(b"SPICE(SIZEOUTOFRANGE)", ctx)?;
            CHKOUT(b"SCLI01", ctx)?;
            return Ok(());
        }
    }

    //
    // Check values of kernel variables:
    //
    //    - The output delimiter code is at least 1 and is not
    //      greater than the number of delimiters.
    //
    //    - The field count is at least 1 and is not greater than
    //      MXNFLD.
    //
    //    - The time system code is at least 1 and is not greater
    //      than MXTSYS.
    //
    if fstr::eq(NAME, save.NAMLST.get(DELIDX)) {
        if ((IVAL[1] < 1) || (IVAL[1] > NDELIM)) {
            REPMC(&save.BVLMSG, b"#", &TMPNAM, &mut ERRMSG);
            REPMI(&ERRMSG.clone(), b"#", IVAL[1], &mut ERRMSG, ctx);
            SETMSG(&ERRMSG, ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
            CHKOUT(b"SCLI01", ctx)?;
            return Ok(());
        }
    }

    if fstr::eq(NAME, save.NAMLST.get(NFLIDX)) {
        if ((IVAL[1] < 1) || (IVAL[1] > MXNFLD)) {
            REPMC(&save.BVLMSG, b"#", &TMPNAM, &mut ERRMSG);
            REPMI(&ERRMSG.clone(), b"#", IVAL[1], &mut ERRMSG, ctx);
            SETMSG(&ERRMSG, ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
            CHKOUT(b"SCLI01", ctx)?;
            return Ok(());
        }
    }

    if fstr::eq(NAME, save.NAMLST.get(SYSIDX)) {
        if ((IVAL[1] < 1) || (IVAL[1] > MXTSYS)) {
            REPMC(&save.BVLMSG, b"#", &TMPNAM, &mut ERRMSG);
            REPMI(&ERRMSG.clone(), b"#", IVAL[1], &mut ERRMSG, ctx);
            SETMSG(&ERRMSG, ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
            CHKOUT(b"SCLI01", ctx)?;
            return Ok(());
        }
    }

    CHKOUT(b"SCLI01", ctx)?;
    Ok(())
}

/// SCLK lookup of double precision data, type 1
///
/// Look up double precision type 1 SCLK data from the kernel pool.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
/// * [SCLK](crate::required_reading::sclk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NAME,
///  SC         I   Name of kernel data item, NAIF spacecraft ID code.
///  MAXNV      I   Maximum number of d.p. values to return.
///  N          O   Number of values actually returned.
///  DVAL       O   Requested kernel data item.
///  MXCOEF     P   Maximum number of coefficient sets in SCLK kernel.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME,
///  SC       are, respectively, a name and a NAIF integer code
///           of a spacecraft that together define the name of a
///           requested kernel data item. NAME is the full name
///           as it appears in the SCLK kernel, except that it
///           lacks the final underscore and spacecraft integer
///           code (actually, the negative of the spacecraft
///           code).  This routine combines NAME and SC to
///           make up the appropriate kernel variable name.
///
///           For example, to look up data associated with the
///           name
///
///              SCLK01_COEFFICIENTS_77
///
///           you would supply NAME as
///
///              SCLK01_COEFFICIENTS
///
///           and SC as -77.
///
///
///  MAXNV    is the maximum number of values to return.  MAXNV
///           is used to prevent SCLD01 from writing past the end
///           of the supplied array DVAL.
/// ```
///
/// # Detailed Output
///
/// ```text
///  N        is the number of values actually returned.
///
///  DVAL     is an array containing the requested double
///           precision kernel data item.
/// ```
///
/// # Parameters
///
/// ```text
///  MXCOEF   is the maximum number of coefficient sets in the
///           array COEFFS that defines the mapping between
///           encoded type 1 SCLK and a parallel time system.
///           This array has dimension 3 x MXCOEF. The value of
///           MXCOEF may be increased as required.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If item specified by NAME and SC is not found in the kernel
///      pool, the error SPICE(KERNELVARNOTFOUND) is signaled. The
///      output arguments are not modified.
///
///  2)  If the item specified by NAME and SC is found but does not
///      have numeric type, the error SPICE(BADKERNELVARTYPE) is
///      signaled.
///
///  3)  This routine can check certain data for validity. If any of
///      these items have invalid values, the error
///      SPICE(VALUEOUTOFRANGE) is signaled. The output arguments are
///      not modified. The values in question are:
///
///         - The number of coefficients.
///         - The number of partition start values.
///         - The number of partition end values.
///         - The number of moduli.
///         - The values of the moduli (lower bounds)
///         - The number of offsets.
///         - The number of kernel identifiers.
///
///  4)  If the partition times or SCLK coefficients themselves
///      are invalid, this routine does nothing about it. It is
///      simply not possible to detect all of the possible errors
///      that these data may be subject to.
///
///  5)  If the dimension of the requested item exceeds MAXNV, the
///      error SPICE(ARRAYTOOSMALL) is signaled.
///
///  6)  If the dimension of the requested item is outside of the
///      limits for that item, the error SPICE(INVALIDSIZE) is
///      signaled.
///
///  7)  If the dimension of the coefficient kernel variable is
///      not a multiple of 3, the error SPICE(INVALIDSIZE) is
///      signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The purpose of this routine is to localize error checking for
///  lookups of type 1 SCLK kernel pool data. This routine handles
///  lookups of double precision data.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Check a NAIF SCLK kernel for accuracy by converting the
///      encoded SCLK coefficients to strings with partition numbers
///      and converting the parallel times to UTC strings. Print out
///      the results in tabular form. In this example, the spacecraft
///      is Mars Observer, which has NAIF ID code -94. We could
///      make the program work for Galileo by using the NAIF ID code
///      -77 instead of -94.
///
///         C
///         C     Load the SCLK kernel in question, and also load
///         C     a leapseconds kernel. We use made-up names for the
///         C     kernel file; you would use the actual names of your
///         C     kernel files instead if you were to carry out this
///         C     procedure.
///         C
///               CALL FURNSH ( 'SAMPLE_MO_SCLK.KER' )
///               CALL FURNSH ( 'LEAPSECONDS.KER'    )
///
///               CONAME =  SCLK01_COEFFICIENTS
///               SC     =  -94
///
///         C
///         C     Grab the coefficients.
///         C
///               CALL SCLD01 ( CONAME, SC, 3*MXCOEF, NCOEFF, COEFFS )
///
///         C
///         C     The SCLK coefficients are in the first row of the
///         C     coefficients array; the parallel times are in the
///         C     second. Since the parallel time system used for MO
///         C     is terrestrial dynamical time (TDT), we will convert
///         C     the parallel time values to ET (TDB) first and then
///         C     convert the resulting times to UTC.
///         C
///         C     In a more robust algorithm, we'd look up the parallel
///         C     time system code used in the SCLK kernel rather than
///         C     assume that it is a particular system. We omit this
///         C     check for simplicity.
///         C
///         C     We decode the SCLK coefficients using SCDECD. Write
///         C     out the results to a file we'll call COMPARE.DAT.
///         C
///               OUTFIL = 'COMPARE.DAT'
///
///               CALL WRLINE ( OUTFIL, '    SCLK               UTC' )
///               CALL WRLINE ( OUTFIL, ' '                          )
///
///               DO I = 1, NCOEFF / 3
///
///                  CALL SCDECD ( -94,  COEFF(1,I),  CLKSTR )
///         C
///         C        Convert the parallel time coefficients, which are
///         C        given in TDT, to ET. UNITIM returns this value.
///         C
///                  CALL ET2UTC ( UNITIM ( COEFF(2,I), 'TDT', 'TDB' ),
///              .                 'D',
///              .                  3,
///              .                  UTC    )
///
///                  LINE = ' SCLK        UTC '
///
///                  CALL REPMC  ( LINE, 'SCLK', CLKSTR, LINE )
///                  CALL REPMC  ( LINE, 'UTC',  UTC,    LINE )
///
///                  CALL WRLINE ( OUTFIL, LINE )
///
///               END DO
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  SCLD01 assumes that a SCLK kernel appropriate to the
///      spacecraft identified by SC has been loaded.
///
///  2)  SCLD01 handles lookups of type 1 SCLK data only.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.0.0, 01-DEC-2021 (NJB) (JDR)
///
///         New checks on item sizes have been added: sizes are now
///         compared against upper bounds as well as lower bounds.
///         Previously only lower bounds were used. A check has been
///         added to verify that the coefficient count is a multiple
///         of 3.
///
///         Bug fix: corrected index error in modulus range check. Made
///         cosmetic changes to code and comments.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.2.0, 20-NOV-2006 (NJB) (EDW)
///
///         Routine was updated to use GDPOOL instead of RTPOOL. Header
///         has been updated to remove warnings about memory corruption and
///         to document exception handling for output buffer overflow
///         errors.
///
///         Header references to LDPOOL were replaced with references to
///         FURNSH.
///
/// -    SPICELIB Version 2.1.0, 19-OCT-1992 (NJB)
///
///         This entry point was updated to fix a bug: if a kernel pool
///         lookup fails, the number of elements returned N is now set to
///         zero.
///
/// -    SPICELIB Version 2.0.0, 17-APR-1992 (NJB) (WLT)
///
///         One constant was changed in the code for clarity; no functional
///         change results from this. Some corrections and other minor
///         enhancements were made to the header. Comment section for
///         permuted index source lines was added following the header.
///
/// -    SPICELIB Version 1.0.0, 06-SEP-1990 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.1.0, 19-OCT-1992 (NJB)
///
///         This entry point was updated to fix a bug: if a kernel pool
///         lookup fails, the number of elements returned N is now set to
///         zero. Formerly, this routine returned whatever value was
///         returned by RTPOOL.  RTPOOL, however, does not set N to zero
///         when the data item requested from it is not found.
///
/// -    SPICELIB Version 2.0.0, 17-APR-1992 (NJB) (WLT)
///
///         The constant 1 was changed to 1.D0 in the test for the
///         validity of the moduli for a spacecraft clock. The change
///         was made simply for clarity.
///
///         Some corrections and other minor enhancements were made to the
///         header. Comment section for permuted index source lines was
///         added following the header.
/// ```
pub fn scld01(
    ctx: &mut SpiceContext,
    name: &str,
    sc: i32,
    maxnv: i32,
    n: &mut i32,
    dval: &mut [f64],
) -> crate::Result<()> {
    SCLD01(name.as_bytes(), sc, maxnv, n, dval, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SCLD01 ( SCLK lookup of double precision data, type 1 )
pub fn SCLD01(
    NAME: &[u8],
    SC: i32,
    MAXNV: i32,
    N: &mut i32,
    DVAL: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DVAL = DummyArrayMut::new(DVAL, 1..);
    let mut ERRMSG = [b' '; MSGLEN as usize];
    let mut KVNAME = [b' '; KVNMLN as usize];
    let mut TMPNAM = [b' '; NAMLEN as usize];
    let mut TYPE = [b' '; 1 as usize];
    let mut I: i32 = 0;
    let mut NF: i32 = 0;
    let mut NFIELD: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SCLD01", ctx)?;

    //
    // Form the name of the kernel pool datum, and do the lookup.
    //
    fstr::assign(&mut TMPNAM, NAME);

    SUFFIX(b"_#", 0, &mut TMPNAM);
    REPMI(&TMPNAM.clone(), b"#", -SC, &mut TMPNAM, ctx);

    //
    // Make sure we have enough room for the item in our output
    // array. Look up the dimension of the item.
    //
    DTPOOL(&TMPNAM, &mut FOUND, N, &mut TYPE, ctx)?;

    if (*N > MAXNV) {
        SETMSG(b"Item # has size # but output array has size #.", ctx);
        ERRCH(b"#", &TMPNAM, ctx);
        ERRINT(b"#", *N, ctx);
        ERRINT(b"#", MAXNV, ctx);
        SIGERR(b"SPICE(ARRAYTOOSMALL)", ctx)?;
        CHKOUT(b"SCLD01", ctx)?;
        return Ok(());
    }

    //
    // Check the data type of the variable.
    //
    if (FOUND && fstr::ne(&TYPE, b"N")) {
        SETMSG(
            b"Kernel variable # for spacecraft clock # does not have numeric type.",
            ctx,
        );
        ERRCH(b"#", &TMPNAM, ctx);
        ERRINT(b"#", SC, ctx);
        SIGERR(b"SPICE(BADKERNELVARTYPE)", ctx)?;
        CHKOUT(b"SCLD01", ctx)?;
        return Ok(());
    }

    GDPOOL(&TMPNAM, 1, MAXNV, N, DVAL.as_slice_mut(), &mut FOUND, ctx)?;

    if FAILED(ctx) {
        //
        // This code should be unreachable but is provided for safety.
        //
        CHKOUT(b"SCLD01", ctx)?;
        return Ok(());
    }
    //
    // Make sure we found what we were looking for.
    //
    if !FOUND {
        //
        // No values are returned in this case.
        //
        *N = 0;

        SETMSG(&save.NFDMSG, ctx);
        ERRCH(b"#", &TMPNAM, ctx);
        SIGERR(b"SPICE(KERNELVARNOTFOUND)", ctx)?;
        CHKOUT(b"SCLD01", ctx)?;
        return Ok(());
    }

    //
    // Now we must check that the number of returned values is in the
    // appropriate range. We test for the following conditions:
    //
    //    - The number of coefficients is at least 3 and no more than
    //      3 * MXCOEF
    //
    //    - The number of partition start values is at least 1 and no
    //      more than MXPART
    //
    //    - The number of partition end values is at least 1 and no more
    //      than MXPART
    //
    //    - The number of moduli is at least 1, no more than MXNFLD,
    //      and matches the field count.
    //
    //    - The number of offsets is at least 1, no more than MXNFLD,
    //      and matches the field count.
    //
    // Note that the kernel pool doesn't allow a kernel variable to
    // not have associated values. Checks against the lower bound 1 are
    // done just to simplify the code.
    //
    // See if the input name is in the list of items we know about.
    // If it is, perform the bounds checks that apply.
    //
    I = ISRCHC(NAME, NKITEM, save.NAMLST.as_arg());

    if (I != 0) {
        if ((*N < save.LB[I]) || (*N > save.UB[I])) {
            REPMI(&save.NUMMSG, b"#", *N, &mut ERRMSG, ctx);
            REPMC(&ERRMSG.clone(), b"#", &TMPNAM, &mut ERRMSG);
            REPMI(&ERRMSG.clone(), b"#", save.LB[I], &mut ERRMSG, ctx);
            REPMI(&ERRMSG.clone(), b"#", save.UB[I], &mut ERRMSG, ctx);
            SETMSG(&ERRMSG, ctx);
            SIGERR(b"SPICE(SIZEOUTOFRANGE)", ctx)?;
            CHKOUT(b"SCLD01", ctx)?;
            return Ok(());
        }
    }

    if fstr::eq(NAME, save.NAMLST.get(COFIDX)) {
        if (((*N / 3) * 3) != *N) {
            SETMSG(
                b"Coefficient count for # must be multiple of 3 but was #.",
                ctx,
            );
            ERRCH(b"#", &TMPNAM, ctx);
            ERRINT(b"#", *N, ctx);
            SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
            CHKOUT(b"SCLD01", ctx)?;
            return Ok(());
        }
    }

    //
    // Check the values of the moduli themselves. Also check the
    // moduli count against the field count.
    //
    if fstr::eq(NAME, save.NAMLST.get(MODIDX)) {
        {
            let m1__: i32 = 1;
            let m2__: i32 = *N;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if (DVAL[I] < 1.0) {
                    REPMC(&save.BVLMSG, b"#", &TMPNAM, &mut ERRMSG);
                    REPMD(&ERRMSG.clone(), b"#", DVAL[I], 14, &mut ERRMSG, ctx);
                    SETMSG(&ERRMSG, ctx);
                    SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
                    CHKOUT(b"SCLD01", ctx)?;
                    return Ok(());
                }

                I += m3__;
            }
        }
    }

    if (fstr::eq(NAME, save.NAMLST.get(MODIDX)) || fstr::eq(NAME, save.NAMLST.get(OFFIDX))) {
        //
        // Get the field count for this clock.
        //
        fstr::assign(
            &mut KVNAME,
            fstr::substr(save.NAMLST.get(NFLIDX), 1..=KVNMLN),
        );

        SUFFIX(b"_#", 0, &mut KVNAME);
        REPMI(&KVNAME.clone(), b"#", -SC, &mut KVNAME, ctx);

        GIPOOL(
            &KVNAME,
            1,
            1,
            &mut NF,
            std::slice::from_mut(&mut NFIELD),
            &mut FOUND,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"SCLD01", ctx)?;
            return Ok(());
        }

        if !FOUND {
            SETMSG(b"Field count was not found for SCLK #.", ctx);
            ERRINT(b"#", SC, ctx);
            SIGERR(b"SPICE(KERNELVARNOTFOUND)", ctx)?;
            CHKOUT(b"SCLD01", ctx)?;
            return Ok(());
        }

        if (*N != NFIELD) {
            if fstr::eq(NAME, save.NAMLST.get(MODIDX)) {
                SETMSG(
                    b"Modulus count # does not match field count # for SCLK #.",
                    ctx,
                );
            } else {
                SETMSG(
                    b"Offset count # does not match field count # for SCLK #.",
                    ctx,
                );
            }

            ERRINT(b"#", *N, ctx);
            ERRINT(b"#", NFIELD, ctx);
            ERRINT(b"#", SC, ctx);
            SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
            CHKOUT(b"SCLD01", ctx)?;
            return Ok(());
        }
    }

    CHKOUT(b"SCLD01", ctx)?;
    Ok(())
}
