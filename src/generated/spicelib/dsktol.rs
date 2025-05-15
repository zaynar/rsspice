//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const XFRACT: f64 = 0.0000000001;
const KEYXFR: i32 = 1;
const SGREED: f64 = 0.00000001;
const KEYSGR: i32 = (KEYXFR + 1);
const SGPADM: f64 = 0.0000000001;
const KEYSPM: i32 = (KEYSGR + 1);
const PTMEMM: f64 = 0.0000001;
const KEYPTM: i32 = (KEYSPM + 1);
const ANGMRG: f64 = 0.000000000001;
const KEYAMG: i32 = (KEYPTM + 1);
const LONALI: f64 = 0.000000000001;
const KEYLAL: i32 = (KEYAMG + 1);
const NDSPAR: i32 = 6;
const NAMLEN: i32 = 6;

struct SaveVars {
    NAMES: ActualCharArray,
    DPPARS: StackArray<f64, 6>,
    ISFIXD: StackArray<bool, 6>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NAMES = ActualCharArray::new(NAMLEN, 1..=NDSPAR);
        let mut DPPARS = StackArray::<f64, 6>::new(1..=NDSPAR);
        let mut ISFIXD = StackArray::<bool, 6>::new(1..=NDSPAR);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(XFRACT),
                Val::D(SGREED),
                Val::D(SGPADM),
                Val::D(PTMEMM),
                Val::D(ANGMRG),
                Val::D(LONALI),
            ]
            .into_iter();
            DPPARS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(true),
                Val::L(true),
            ]
            .into_iter();
            ISFIXD
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"XFRACT"),
                Val::C(b"SGREED"),
                Val::C(b"SGPADM"),
                Val::C(b"PTMEMM"),
                Val::C(b"ANGMRG"),
                Val::C(b"LONALI"),
            ]
            .into_iter();
            NAMES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            NAMES,
            DPPARS,
            ISFIXD,
        }
    }
}

/// DSK, tolerance umbrella
///
/// Umbrella routine for DSK tolerance and margin parameter access
/// routines.
///
/// # Required Reading
///
/// * [DSK](crate::required_reading::dsk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  ENTRY POINTS
///  --------  ---  --------------------------------------------------
///  KEYWRD     I   DSKGTL, DSKSTL
///  DPVAL     I-O  DSKGTL, DSKSTL
/// ```
///
/// # Detailed Input
///
/// ```text
///  See the entry points for descriptions of their input arguments.
/// ```
///
/// # Detailed Output
///
/// ```text
///  See the entry points for descriptions of their output arguments.
/// ```
///
/// # Parameters
///
/// ```text
///  See the include file
///
///     dsktol.inc
///
///  for descriptions and values of the tolerance or margin parameters
///  accessed by the entry points of this routine, and of the keyword
///  parameters used to refer to them.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If this routine is called directly, the error
///      SPICE(BOGUSENTRY) is signaled.
///
///  2)  See the entry points for descriptions of exceptions specific
///      to those entry points.
/// ```
///
/// # Particulars
///
/// ```text
///  The routines in this package serve to centralize numeric
///  tolerance and margin values used by the DSK subsystem. The
///  subsystem retrieves values from this package to use at run time.
///
///  The entry points of this routine are
///
///     DSKGTL {DSK, get tolerance value}
///     DSKSTL {DSK, set tolerance value}
///
///  To minimize run time overhead, the "keywords" used by these
///  routines to identify parameters are actually integer codes.
///
///  SPICE users may override certain values maintained by this
///  package; others values are fixed.
///
///  One use of this system would be to disable the "greedy"
///  algorithms used to prevent "false miss" ray-surface intercepts
///  results. Setting to zero the margins used for this purpose would
///  accomplish this.
///
///  It is recommended that any change to the tolerance values made at
///  run time be programmed only by SPICE experts.
/// ```
///
/// # Examples
///
/// ```text
///  See the entry points.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The default settings used by the DSK subsystem should
///      be overridden only by expert SPICE users.
///
///  2)  The entry points of this routine do not check the
///      validity of new parameter values supplied by the
///      calling application.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 09-JUL-2020 (JDR)
///
///         Edited the entry points and umbrella headers to comply with
///         NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 01-AUG-2016 (NJB)
/// ```
pub fn dsktol(ctx: &mut SpiceContext, keywrd: i32, dpval: f64) -> crate::Result<()> {
    DSKTOL(keywrd, dpval, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DSKTOL ( DSK, tolerance umbrella )
pub fn DSKTOL(KEYWRD: i32, DPVAL: f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved values
    //

    //
    // Initial values
    //
    // Caution: this array must be declared in
    // the order defined by the keywords declared
    // in dsktol.inc.
    //

    CHKIN(b"DSKTOL", ctx)?;
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"DSKTOL", ctx)?;
    Ok(())
}

/// DSK, get tolerance
///
/// Retrieve the value of a specified DSK tolerance or margin
/// parameter.
///
/// # Required Reading
///
/// * [DSK](crate::required_reading::dsk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  KEYWRD     I   Code specifying parameter to retrieve.
///  DPVAL      O   Value of parameter.
/// ```
///
/// # Detailed Input
///
/// ```text
///  KEYWRD   is an integer code specifying the parameter to
///           retrieve. See the include file dsktol.inc for
///           a description of the possible keywords.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DPVAL    is the value of the parameter specified by KEYWRD.
/// ```
///
/// # Parameters
///
/// ```text
///  See the include file
///
///     dsktol.inc
///
///  for descriptions and values of the tolerance or margin parameters
///  accessed by this routine, and of the keyword parameters used to
///  refer to them.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input keyword is not recognized, the error
///      SPICE(INDEXOUTOFRANGE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The routines in this package serve to centralize numeric
///  tolerance and margin values used by the DSK subsystem.
///  The subsystem retrieves values from this package to use
///  at run time.
///
///  The entry points of this routine are
///
///     DSKGTL {DSK, get tolerance value}
///     DSKSTL {DSK, set tolerance value}
///
///  To minimize run time overhead, the "keywords" used by
///  these routines to identify parameters are actually
///  integer codes.
///
///  SPICE users may override certain values maintained by
///  this package; others values are fixed. It is recommended
///  that any change to the tolerance values made at run
///  time be performed only by expert SPICE users.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Obtain and display the DSK type 2 plate expansion fraction.
///
///
///     Example code begins here.
///
///
///           PROGRAM DSKGTL_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include files
///     C
///           INCLUDE 'dsktol.inc'
///
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      DPVAL
///
///           CALL DSKGTL ( KEYXFR, DPVAL )
///
///           WRITE(*,*) 'Plate expansion fraction = ', DPVAL
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Plate expansion fraction =    1.0000000000000000E-010
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 09-JUL-2020 (JDR)
///
///         Updated the header to comply with NAIF standard. Added
///         complete example code.
///
/// -    SPICELIB Version 1.0.0, 01-AUG-2016 (NJB)
/// ```
pub fn dskgtl(ctx: &mut SpiceContext, keywrd: i32, dpval: &mut f64) -> crate::Result<()> {
    DSKGTL(keywrd, dpval, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DSKGTL ( DSK, get tolerance )
pub fn DSKGTL(KEYWRD: i32, DPVAL: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Use discovery check-in.
    //
    if ((KEYWRD < 1) || (KEYWRD > NDSPAR)) {
        CHKIN(b"DSKGTL", ctx)?;
        SETMSG(b"Valid keyword range is 1:#; keyword was #.", ctx);
        ERRINT(b"#", NDSPAR, ctx);
        ERRINT(b"#", KEYWRD, ctx);
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"DSKGTL", ctx)?;
        return Ok(());
    }

    *DPVAL = save.DPPARS[KEYWRD];

    Ok(())
}

/// DSK, set tolerance
///
/// Set the value of a specified DSK tolerance or margin parameter.
///
/// # Required Reading
///
/// * [DSK](crate::required_reading::dsk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  KEYWRD     I   Code specifying parameter to modify.
///  DPVAL      I   New value of parameter.
/// ```
///
/// # Detailed Input
///
/// ```text
///  KEYWRD   is an integer code specifying the parameter to
///           set. See the include file dsktol.inc for
///           a description of the possible keywords.
///
///  DPVAL    is the new value of the parameter specified by KEYWRD.
///           This value will be retrieved by future calls to
///           to DSKGTL made to retrieve the specified parameter.
///
///           <<< Use extreme caution. This routine performs no
///           checks on DPVAL. >>>
/// ```
///
/// # Detailed Output
///
/// ```text
///  None.
///
///  This routine operates by side effects.
/// ```
///
/// # Parameters
///
/// ```text
///  See the include file
///
///     dsktol.inc
///
///  for descriptions and values of the tolerance or margin parameters
///  accessed by this routine, and of the keyword parameters used to
///  refer to them.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input keyword is not recognized, the error
///      SPICE(INDEXOUTOFRANGE) is signaled.
///
///  2)  If an attempt is made to modify a fixed parameter,
///      the error SPICE(IMMUTABLEVALUE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The routines in this package serve to centralize numeric
///  tolerance and margin values used by the DSK subsystem.
///  The subsystem retrieves values from this package to use
///  at run time.
///
///  The entry points of this routine are
///
///     DSKGTL {DSK, get tolerance value}
///     DSKSTL {DSK, set tolerance value}
///
///  To minimize run time overhead, the "keywords" used by
///  these routines to identify parameters are actually
///  integer codes.
///
///  SPICE users may override certain values maintained by
///  this package; others values are fixed. It is recommended
///  that any change to the tolerance values made at run
///  time be performed only by expert SPICE users.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Obtain, display, and update the DSK type 2 plate expansion
///     fraction.
///
///
///     Example code begins here.
///
///
///           PROGRAM DSKSTL_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include files
///     C
///           INCLUDE 'dsktol.inc'
///
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      DPVAL
///
///           CALL DSKGTL ( KEYXFR, DPVAL )
///
///           WRITE(*,*) 'Default plate expansion fraction = ', DPVAL
///
///     C
///     C     Update the parameter.
///     C
///           CALL DSKSTL ( KEYXFR, 1.D-8 )
///
///     C
///     C     Verify the update.
///     C
///           CALL DSKGTL ( KEYXFR, DPVAL )
///
///           WRITE(*,*) 'New plate expansion fraction     = ', DPVAL
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Default plate expansion fraction =    1.0000000000000000E-010
///      New plate expansion fraction     =    1.0000000000000000E-008
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The default settings used by the DSK subsystem should
///      be overridden only by expert SPICE users.
///
///  2)  This routine does not check the  validity of new parameter
///      values supplied by the calling application.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 09-JUL-2020 (JDR)
///
///         Updated the header to comply with NAIF standard. Added
///         complete example code.
///
/// -    SPICELIB Version 1.0.0, 01-AUG-2016 (NJB)
/// ```
pub fn dskstl(ctx: &mut SpiceContext, keywrd: i32, dpval: f64) -> crate::Result<()> {
    DSKSTL(keywrd, dpval, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DSKSTL ( DSK, set tolerance )
pub fn DSKSTL(KEYWRD: i32, DPVAL: f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DSKSTL", ctx)?;

    if ((KEYWRD < 1) || (KEYWRD > NDSPAR)) {
        SETMSG(b"Valid keyword range is 1:#; keyword was #.", ctx);
        ERRINT(b"#", NDSPAR, ctx);
        ERRINT(b"#", KEYWRD, ctx);
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"DSKSTL", ctx)?;
        return Ok(());
    }

    if save.ISFIXD[KEYWRD] {
        SETMSG(b"The parameter # cannot be modified.", ctx);
        ERRCH(b"#", &save.NAMES[KEYWRD], ctx);
        SIGERR(b"SPICE(IMMUTABLEVALUE)", ctx)?;
        CHKOUT(b"DSKSTL", ctx)?;
        return Ok(());
    }

    //
    // We have a valid parameter index. We don't check
    // the new parameter value; the user presumably knows
    // the reason for change.
    //
    save.DPPARS[KEYWRD] = DPVAL;

    CHKOUT(b"DSKSTL", ctx)?;
    Ok(())
}
