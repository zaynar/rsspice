//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NMAX: i32 = 200;

struct SaveVars {
    MISSED: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut MISSED = ActualCharArray::new(20, 1..=5);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"DELTET/DELTA_T_A, #"),
                Val::C(b"DELTET/K, #"),
                Val::C(b"DELTET/EB, #"),
                Val::C(b"DELTET/M, #"),
                Val::C(b"DELTET/DELTA_AT, #"),
            ]
            .into_iter();
            MISSED
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { MISSED }
    }
}

/// Delta ET, ET - UTC
///
/// Return the value of Delta ET (ET-UTC) for an input epoch.
///
/// # Required Reading
///
/// * [TIME](crate::required_reading::time)
/// * [KERNEL](crate::required_reading::kernel)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  EPOCH      I   Input epoch (seconds past J2000).
///  EPTYPE     I   Type of input epoch ('UTC' or 'ET').
///  DELTA      O   Delta ET (ET-UTC) at input epoch.
/// ```
///
/// # Detailed Input
///
/// ```text
///  EPOCH    is the epoch at which "delta ET" is to be computed.
///           EPOCH may be either UTC or ephemeris seconds past
///           J2000, as specified by EPTYPE.
///
///  EPTYPE   is the type of input epoch. It may be either
///           of the following:
///
///              'UTC'    UTC seconds past J2000 UTC.
///
///              'ET'     Ephemeris seconds past J2000 TDB,
///                       also known as barycentric dynamical
///                       time (TDB).
/// ```
///
/// # Detailed Output
///
/// ```text
///  DELTA    is the value of
///
///              "delta ET" = ET - UTC
///
///           at the input epoch. This is added to UTC to give
///           ET, or subtracted from ET to give UTC. The routine
///           is reversible: that is, given the following calls,
///
///              CALL DELTET ( UTC,      'UTC', DEL1 )
///              CALL DELTET ( UTC+DEL1, 'ET',  DEL2 )
///
///           the expression
///
///              ( DEL1 .EQ. DEL2 )
///
///           is always .TRUE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input epoch is not recognized, the error
///      SPICE(INVALIDEPOCH) is signaled.
///
///  2)  If the variables necessary for the computation of DELTA
///      have not been loaded into the kernel pool, the error
///      SPICE(KERNELVARNOTFOUND) is signaled.
///
///  3)  If the number of leapseconds in the pool is greater than
///      the local leapseconds buffer size, the error
///      SPICE(BUFFEROVERFLOW) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The constants necessary for computing the offset are taken
///  from the kernel pool, where they are assumed to have been
///  loaded from a kernel file.
///
///  The tables are consulted to determine the number of leap seconds
///  preceding the input epoch. Also, an approximation to the periodic
///  yearly variation (which has an amplitude of just under two
///  milliseconds) in the difference between ET and TAI (Atomic Time)
///  is computed. The final value of Delta ET is given by
///
///     Delta ET = ( ET - TAI ) + leap seconds
/// ```
///
/// # Examples
///
/// ```text
///  The following example shows how DELTET may be used to convert
///  from UTC seconds past J2000 to ephemeris seconds past J2000.
///
///     CALL DELTET ( UTCSEC, 'UTC', DELTA )
///     ET = UTCSEC + DELTA
///
///  The following example shows how DELTET may be used to convert
///  from ephemeris seconds past J2000 to UTC seconds past J2000.
///
///     CALL DELTET ( ET, 'ET', DELTA )
///     UTCSEC = ET - DELTA
///
///  See the Time required reading time.req for further examples.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The routines STR2ET and ET2UTC are preferred for conversions
///      between UTC and ET. This routine is provided mainly as a
///      utility for STR2ET and ET2UTC.
///
///  2)  A leapseconds kernel containing leapseconds and relativistic
///      terms MUST be loaded prior to calling this subroutine.
///      Examples demonstrating how to load a kernel pool are included
///      in the Required Reading file time.req and in the $Examples
///      section of this header. For more general information about
///      kernel pools, please consult the Required Reading file
///      kernel.req.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  "The Astronomical Almanac for the Year 1990," United States
///       Naval Observatory, U.S. Government Printing Office,
///       Washington, D.C., 1989.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.M. Owen          (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 24-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary entries in $Revisions section.
///
///         Replaced reference to UTC2ET with STR2ET in $Restrictions
///         section.
///
/// -    SPICELIB Version 1.2.2, 18-APR-2014 (BVS)
///
///         Minor header edits.
///
/// -    SPICELIB Version 1.2.1, 18-MAY-2010 (BVS)
///
///         Removed "C$" marker from text in the header.
///
/// -    SPICELIB Version 1.2.0, 24-AUG-1998 (WLT)
///
///         The previous upgrade introduced an error in the fetch
///         of the variable DELTET/M from the kernel pool. This
///         error was corrected.
///
/// -    SPICELIB Version 1.1.0, 20-APR-1998 (NJB)
///
///         Calls to RTPOOL were replaced with calls to GDPOOL, which
///         does more robust error checking. Check for buffer overflow
///         was added. Local declarations were re-organized.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 1.1.0, 06-OCT-1988 (IMU)
///
///         Tim Colvin of Rand noticed that times returned by UTC2ET
///         and TPARSE differed by one second. Upon closer
///         inspection, crack NAIF staff members deduced that in fact
///         Mr. Colvin had not loaded the kernel pool, and were
///         surprised to learn that no error had occurred.
///
///         Multiple FOUND flags and a bevy of new error messages
///         were implemented to cope with this unfortunate oversight.
/// ```
pub fn deltet(
    ctx: &mut SpiceContext,
    epoch: f64,
    eptype: &str,
    delta: &mut f64,
) -> crate::Result<()> {
    DELTET(epoch, eptype.as_bytes(), delta, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DELTET ( Delta ET, ET - UTC )
pub fn DELTET(
    EPOCH: f64,
    EPTYPE: &[u8],
    DELTA: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TYPE = [b' '; 4];
    let mut DTYPE = [b' '; 1];
    let mut AET: f64 = 0.0;
    let mut DLEAP = ActualArray2D::<f64>::new(1..=2, 1..=NMAX);
    let mut DTA: f64 = 0.0;
    let mut EA: f64 = 0.0;
    let mut EB: f64 = 0.0;
    let mut ET: f64 = 0.0;
    let mut ETTAI: f64 = 0.0;
    let mut K: f64 = 0.0;
    let mut LEAPS: f64 = 0.0;
    let mut M = StackArray::<f64, 2>::new(0..=1);
    let mut MA: f64 = 0.0;
    let mut N: i32 = 0;
    let mut NLEAP: i32 = 0;
    let mut FOUND = StackArray::<bool, 5>::new(1..=5);

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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DELTET", ctx)?;
    }

    //
    // Convert the epoch type to uppercase, to simplify comparisons.
    //
    UCASE(EPTYPE, &mut TYPE, ctx);

    //
    // Extract the necessary constants from the kernel pool.
    // Leap seconds and their epochs are interleaved in DELTA_AT.
    //
    // DLEAP(1,i) is the number of leap seconds at DLEAP(2,i) UTC
    // seconds past J2000.
    //
    GDPOOL(
        b"DELTET/DELTA_T_A",
        1,
        1,
        &mut N,
        std::slice::from_mut(&mut DTA),
        &mut FOUND[1],
        ctx,
    )?;
    GDPOOL(
        b"DELTET/K",
        1,
        1,
        &mut N,
        std::slice::from_mut(&mut K),
        &mut FOUND[2],
        ctx,
    )?;
    GDPOOL(
        b"DELTET/EB",
        1,
        1,
        &mut N,
        std::slice::from_mut(&mut EB),
        &mut FOUND[3],
        ctx,
    )?;
    GDPOOL(
        b"DELTET/M",
        1,
        2,
        &mut N,
        M.as_slice_mut(),
        &mut FOUND[4],
        ctx,
    )?;

    //
    // Check that the number of leapseconds is not too great for our
    // buffer size (not likely).
    //
    DTPOOL(
        b"DELTET/DELTA_AT",
        &mut FOUND[5],
        &mut NLEAP,
        &mut DTYPE,
        ctx,
    )?;

    if (NLEAP > (2 * NMAX)) {
        SETMSG(
            b"Number of leapseconds, #, is greater than the number that can be buffered, #.",
            ctx,
        );
        ERRINT(b"#", (NLEAP / 2), ctx);
        ERRINT(b"#", NMAX, ctx);
        SIGERR(b"SPICE(BUFFERTOOSMALL)", ctx)?;
        CHKOUT(b"DELTET", ctx)?;
        return Ok(());
    }

    GDPOOL(
        b"DELTET/DELTA_AT",
        1,
        (2 * NMAX),
        &mut NLEAP,
        DLEAP.subarray_mut([1, 1]),
        &mut FOUND[5],
        ctx,
    )?;

    NLEAP = (NLEAP / 2);

    if !((((FOUND[1] && FOUND[2]) && FOUND[3]) && FOUND[4]) && FOUND[5]) {
        SETMSG(b"The following, needed to compute Delta ET (ET - UTC), could not be found in the kernel pool: #", ctx);

        for I in 1..=5 {
            if !FOUND[I] {
                ERRCH(b"#", &save.MISSED[I], ctx);
            }
        }

        ERRCH(b", #", b".", ctx);
        SIGERR(b"SPICE(KERNELVARNOTFOUND)", ctx)?;

        CHKOUT(b"DELTET", ctx)?;
        return Ok(());
    }

    //
    // There are two separate quantities to be determined. First,
    // the appropriate number of leap seconds. Second, the size of
    // the periodic term ET-TAI.
    //

    //
    // For epochs before the first leap second, return Delta ET at
    // the epoch of the leap second minus one second.
    //
    LEAPS = (DLEAP[[1, 1]] - 1 as f64);

    //
    // When counting leap seconds for UTC epochs, we can compare
    // directly against the values in DLEAP.
    //
    if fstr::eq(&TYPE, b"UTC") {
        for I in 1..=NLEAP {
            if (EPOCH >= DLEAP[[2, I]]) {
                LEAPS = DLEAP[[1, I]];
            }
        }

    //
    // For ET epochs, things are a little tougher. In order to compare
    // the input epoch against the epochs of the leap seconds, we need
    // to compute ET-TAI at each of the leap epochs. To make sure that
    // the computation is reversible, it is always done at the nearest
    // ET second (the "approximate ET", or AET).
    //
    // There must be a hundred ways to do this more efficiently.
    // For now, we'll settle for one that works.
    //
    } else if fstr::eq(&TYPE, b"ET") {
        for I in 1..=NLEAP {
            if (EPOCH > DLEAP[[2, I]]) {
                AET = f64::round(((DLEAP[[2, I]] + DTA) + DLEAP[[1, I]]));

                MA = (M[0] + (M[1] * AET));
                EA = (MA + (EB * f64::sin(MA)));
                ETTAI = (K * f64::sin(EA));

                ET = (((DLEAP[[2, I]] + DTA) + DLEAP[[1, I]]) + ETTAI);

                if (EPOCH >= ET) {
                    LEAPS = DLEAP[[1, I]];
                }
            }
        }

    //
    // Uh, those are the only choices.
    //
    } else {
        SETMSG(b"Epoch type was #", ctx);
        ERRCH(b"#", &TYPE, ctx);
        SIGERR(b"SPICE(INVALIDEPOCH)", ctx)?;

        CHKOUT(b"DELTET", ctx)?;
        return Ok(());
    }

    //
    // Add the constant offset, leap seconds, and the relativistic term
    // (as before, computed at the nearest ET second).
    //
    if fstr::eq(&TYPE, b"ET") {
        AET = f64::round(EPOCH);
    } else if fstr::eq(&TYPE, b"UTC") {
        AET = f64::round(((EPOCH + DTA) + LEAPS));
    }

    MA = (M[0] + (M[1] * AET));
    EA = (MA + (EB * f64::sin(MA)));
    ETTAI = (K * f64::sin(EA));

    *DELTA = ((DTA + LEAPS) + ETTAI);

    CHKOUT(b"DELTET", ctx)?;
    Ok(())
}
