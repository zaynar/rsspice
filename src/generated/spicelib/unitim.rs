//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const LBCELL: i32 = -5;
const NEEDED: i32 = 4;
const LNGVAR: i32 = 16;
const MISLEN: i32 = 20;
const TYPLEN: i32 = 8;
const NTDT: i32 = 5;
const NTDB: i32 = 4;
const NRECOG: i32 = (NTDB + NTDT);
const GPSTAI: f64 = 19.0;

struct SaveVars {
    BSLASH: Vec<u8>,
    VARS: ActualCharArray,
    MISSED: ActualCharArray,
    RECOG: ActualCharArray,
    TYPTDB: ActualCharArray,
    TYPTDT: ActualCharArray,
    DTA: f64,
    EB: f64,
    JD2000: f64,
    K: f64,
    M: StackArray<f64, 2>,
    SECSPD: f64,
    USRCTR: StackArray<i32, 2>,
    FIRST: bool,
    NODATA: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BSLASH = vec![b' '; 1 as usize];
        let mut VARS = ActualCharArray::new(LNGVAR, 1..=NEEDED);
        let mut MISSED = ActualCharArray::new(MISLEN, 1..=NEEDED);
        let mut RECOG = ActualCharArray::new(TYPLEN, LBCELL..=NRECOG);
        let mut TYPTDB = ActualCharArray::new(TYPLEN, LBCELL..=NTDB);
        let mut TYPTDT = ActualCharArray::new(TYPLEN, LBCELL..=NTDT);
        let mut DTA: f64 = 0.0;
        let mut EB: f64 = 0.0;
        let mut JD2000: f64 = 0.0;
        let mut K: f64 = 0.0;
        let mut M = StackArray::<f64, 2>::new(0..=1);
        let mut SECSPD: f64 = 0.0;
        let mut USRCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut FIRST: bool = false;
        let mut NODATA: bool = false;

        FIRST = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"DELTET/DELTA_T_A, #"),
                Val::C(b"DELTET/K, #"),
                Val::C(b"DELTET/EB, #"),
                Val::C(b"DELTET/M, #"),
            ]
            .into_iter();
            MISSED
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        NODATA = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"DELTET/DELTA_T_A"),
                Val::C(b"DELTET/K"),
                Val::C(b"DELTET/EB"),
                Val::C(b"DELTET/M"),
            ]
            .into_iter();
            VARS.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            BSLASH,
            VARS,
            MISSED,
            RECOG,
            TYPTDB,
            TYPTDT,
            DTA,
            EB,
            JD2000,
            K,
            M,
            SECSPD,
            USRCTR,
            FIRST,
            NODATA,
        }
    }
}

/// Uniform time scale transformation
///
/// Transform time from one uniform scale to another. The uniform
/// time scales are TAI, GPS, TT, TDT, TDB, ET, JED, JDTDB, JDTDT.
///
/// # Required Reading
///
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  EPOCH      I   An epoch.
///  INSYS      I   The time scale associated with the input EPOCH.
///  OUTSYS     I   The time scale associated with the function value.
///
///  The function returns the d.p. in OUTSYS that is equivalent to the
///  EPOCH on the INSYS time scale.
/// ```
///
/// # Detailed Input
///
/// ```text
///  EPOCH    is an epoch relative to the INSYS time scale.
///
///  INSYS    is a time scale. Acceptable values are:
///
///              'TAI'     International Atomic Time.
///              'TDB'     Barycentric Dynamical Time.
///              'TDT'     Terrestrial Dynamical Time.
///              'TT'      Terrestrial Time, identical to TDT.
///              'ET'      Ephemeris time (in the SPICE system, this is
///                        equivalent to TDB).
///              'JDTDB'   Julian Date relative to TDB.
///              'JDTDT'   Julian Date relative to TDT.
///              'JED'     Julian Ephemeris date (in the SPICE system
///                        this is equivalent to JDTDB).
///              'GPS'     Global Positioning System Time.
///
///           The routine is not sensitive to the case of the
///           characters in INSYS; 'tai' 'Tai' and 'TAI' are all
///           equivalent from the point of view of this routine.
///
///  OUTSYS   is the time scale to which EPOCH should be converted.
///           Acceptable values are the same as for INSYS. The
///           routine is not sensitive to the case of OUTSYS.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the time in the system specified by OUTSYS
///  that is equivalent to the EPOCH in the INSYS time scale.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  The kernel pool must contain the variables:
///
///         'DELTET/DELTA_T_A'
///         'DELTET/K'
///         'DELTET/EB'
///         'DELTET/M'
///
///      If these are not present, the error SPICE(MISSINGTIMEINFO) is
///      signaled. (These variables are typically inserted into the
///      kernel pool by loading a leapseconds kernel with the SPICE
///      routine FURNSH.)
///
///  2)  If the names of either the input or output time types are
///      unrecognized, the error SPICE(BADTIMETYPE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  We use the term uniform time scale to refer to those
///  representations of time that are numeric (each epoch is
///  represented by a number) and additive. A numeric time system is
///  additive if given the representations, E1 and E2, of any pair of
///  successive epochs, the time elapsed between the epochs is given by
///  E2 - E1.
///
///  Given an epoch in one of the uniform time scales specified by
///  INSYS, the function returns the equivalent representation in the
///  scale specified by OUTSYS. A list of the recognized uniform time
///  scales is given in the detailed input for INSYS.
/// ```
///
/// # Examples
///
/// ```text
///  To convert an epoch with respect to the International Atomic
///  Time (TAI) scale to ET (Barycentric Dynamical Time), make the
///  following assignment.
///
///        ET = UNITIM ( TAI, 'TAI', 'ET' )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The appropriate variable must be loaded into the SPICE kernel
///      pool (normally by loading a leapseconds kernel with FURNSH)
///      prior to calling this routine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.5.0, 05-SEP-2020 (EDW) (JDR)
///
///         Added time system name 'TT' (Terrestrial Time) as alternate
///         assignment of 'TDT' (Terrestrial Dynamical Time).
///
///         Included GPS time system mapping.
///
///         Edited the header to comply with NAIF standard.
///
///         Removed references to FURNSH, CLPOOL, KCLEAR, UNLOAD, and
///         Required Reading documents and tutorials from the "variables
///         not present" long error message.
///
/// -    SPICELIB Version 1.4.0, 09-SEP-2013 (BVS)
///
///         Updated to keep track of the POOL counter and call ZZCVPOOL.
///
/// -    SPICELIB Version 1.3.0, 05-MAR-2009 (NJB)
///
///         This routine now keeps track of whether its kernel pool
///         look-up failed. If so, a kernel pool lookup is attempted on
///         the next call to this routine. This change is an enhancement,
///         not a bug fix (unlike similar modifications in SCLK routines).
///
/// -    SPICELIB Version 1.2.1, 15-NOV-2006 (EDW) (NJB)
///
///         Replaced references to LDPOOL with references to FURNSH.
///         Replaced references to RTPOOL with references to GDPOOL.
///         Enhanced long error message associated with missing kernel
///         variables.
///
/// -    SPICELIB Version 1.2.0, 17-FEB-1999 (WLT)
///
///         Added a second call to SWPOOL in the event some required
///         kernel pool variable is not supplied.
///
/// -    SPICELIB Version 1.1.0, 17-MAY-1994 (HAN)
///
///         If the value of the function RETURN is .TRUE. upon execution of
///         this module, this function is assigned a default value of
///         either 0, 0.0D0, .FALSE., or blank depending on the type of
///         the function.
///
/// -    SPICELIB Version 1.0.0, 28-MAR-1992 (WLT)
/// ```
pub fn unitim(ctx: &mut SpiceContext, epoch: f64, insys: &str, outsys: &str) -> crate::Result<f64> {
    let ret = UNITIM(
        epoch,
        insys.as_bytes(),
        outsys.as_bytes(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure UNITIM ( Uniform time scale transformation )
pub fn UNITIM(
    EPOCH: f64,
    INSYS: &[u8],
    OUTSYS: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut UNITIM: f64 = 0.0;
    let mut MYIN = [b' '; TYPLEN as usize];
    let mut MYOUT = [b' '; TYPLEN as usize];
    let mut TYPES = ActualCharArray::new(TYPLEN, LBCELL..=2);
    let mut MYTIME: f64 = 0.0;
    let mut TDB: f64 = 0.0;
    let mut TDT: f64 = 0.0;
    let mut N: i32 = 0;
    let mut FOUND = StackArray::<bool, 4>::new(1..=NEEDED);
    let mut INTDB: bool = false;
    let mut INTDT: bool = false;
    let mut OUTTDB: bool = false;
    let mut OUTTDT: bool = false;
    let mut UPDATE: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // LBCELL is the bottom slot of a cell.
    //
    //
    // NEEDED is the number of kernel pool variables needed by this
    // routine.
    //

    //
    // LNGVAR is the length of the longest kernel pool variable name
    // that is used by this routine.
    //

    //
    // MISLEN is the length required by the MISSED array of strings
    // used for error messages.
    //
    //
    // TYPLEN is the maximum length allowed for names of uniform
    // time types.
    //

    //
    // NTDT is the number of time types based on terrestrial dynamical
    // time (TDT).
    //

    //
    // NTDB is the number of time types base on barycentric dynamical
    // time (TDB).
    //

    //
    // NRECOG is the total number of recognized types.
    //

    //
    // Constant shift between GPS time system and TAI time system.
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
        UNITIM = 0.0;
        return Ok(UNITIM);
    }

    CHKIN(b"UNITIM", ctx)?;

    if save.FIRST {
        save.FIRST = false;
        //
        // Initialize the backslash character.  We use this for error
        // message construction.
        //
        fstr::assign(&mut save.BSLASH, &intrinsics::CHAR(92));

        //
        // Set up the parameters that we are going to need often.
        //
        save.SECSPD = SPD();
        save.JD2000 = J2000();

        //
        // Initialize the sets that we will use.
        //
        fstr::assign(save.TYPTDT.get_mut(1), b"JDTDT");
        fstr::assign(save.TYPTDT.get_mut(2), b"TAI");
        fstr::assign(save.TYPTDT.get_mut(3), b"TDT");
        fstr::assign(save.TYPTDT.get_mut(4), b"GPS");
        fstr::assign(save.TYPTDT.get_mut(5), b"TT");

        fstr::assign(save.TYPTDB.get_mut(1), b"ET");
        fstr::assign(save.TYPTDB.get_mut(2), b"JDTDB");
        fstr::assign(save.TYPTDB.get_mut(3), b"JED");
        fstr::assign(save.TYPTDB.get_mut(4), b"TDB");

        VALIDC(NTDT, NTDT, save.TYPTDT.as_arg_mut(), ctx)?;
        VALIDC(NTDB, NTDB, save.TYPTDB.as_arg_mut(), ctx)?;

        SSIZEC((NTDT + NTDB), save.RECOG.as_arg_mut(), ctx)?;
        UNIONC(
            save.TYPTDT.as_arg(),
            save.TYPTDB.as_arg(),
            save.RECOG.as_arg_mut(),
            ctx,
        )?;

        //
        // Initialize the local POOL counter to user value.
        //
        ZZCTRUIN(save.USRCTR.as_slice_mut(), ctx);

        //
        // Set up the kernel pool watchers
        //
        SWPOOL(b"UNITIM", NEEDED, save.VARS.as_arg(), ctx)?;
    }

    //
    // Check to see if any of the kernel items required by this
    // routine have been updated since the last call to this
    // entry point.
    //
    ZZCVPOOL(b"UNITIM", save.USRCTR.as_slice_mut(), &mut UPDATE, ctx)?;

    if (UPDATE || save.NODATA) {
        //
        // Fetch all of the time parameters from the kernel pool.
        //
        GDPOOL(
            b"DELTET/DELTA_T_A",
            1,
            1,
            &mut N,
            std::slice::from_mut(&mut save.DTA),
            &mut FOUND[1],
            ctx,
        )?;
        GDPOOL(
            b"DELTET/K",
            1,
            1,
            &mut N,
            std::slice::from_mut(&mut save.K),
            &mut FOUND[2],
            ctx,
        )?;
        GDPOOL(
            b"DELTET/EB",
            1,
            1,
            &mut N,
            std::slice::from_mut(&mut save.EB),
            &mut FOUND[3],
            ctx,
        )?;
        GDPOOL(
            b"DELTET/M",
            1,
            2,
            &mut N,
            save.M.as_slice_mut(),
            &mut FOUND[4],
            ctx,
        )?;

        if FAILED(ctx) {
            save.NODATA = true;
            UNITIM = 0.0;

            CHKOUT(b"UNITIM", ctx)?;
            return Ok(UNITIM);
        }

        //
        // If anything wasn't found, it's an error dude.
        //
        if SOMFLS(FOUND.as_slice(), NEEDED) {
            save.NODATA = true;

            //
            // If we didn't get all of the things we needed for time
            // conversion, we need to reset the watch.  Otherwise
            // subsequent calls to this routine will never have the
            // needed data.
            //
            SWPOOL(b"UNITIM", NEEDED, save.VARS.as_arg(), ctx)?;

            SETMSG(&fstr::concat(&fstr::concat(&fstr::concat(&fstr::concat(&fstr::concat(&fstr::concat(b"The following variables, needed to convert between the input uniform time scales, were not found in the kernel pool: # Your program may have failed to load a leapseconds kernel. Other possible causes of this problem include loading an invalid leapseconds kernel---one that lacks an initial ", &save.BSLASH), b"begindata "), b"marker or final newline character, or is "), b"otherwise corrupted---or deleting previous"), b"ly loaded kernel pool variables via calls "), b"to routines that clear the kernel pool. "), ctx);

            for I in 1..=NEEDED {
                if !FOUND[I] {
                    ERRCH(b"#", &save.MISSED[I], ctx);
                }
            }

            ERRCH(b", #", b".", ctx);
            SIGERR(b"SPICE(MISSINGTIMEINFO)", ctx)?;

            CHKOUT(b"UNITIM", ctx)?;
            UNITIM = EPOCH;

            return Ok(UNITIM);
        }

        //
        // At this point the kernel data checks are done.
        //
        save.NODATA = false;
    }

    //
    // Normalize the IN and OUT scale variables
    //
    UCASE(INSYS, &mut MYIN, ctx);
    UCASE(OUTSYS, &mut MYOUT, ctx);

    SSIZEC(2, TYPES.as_arg_mut(), ctx)?;
    INSRTC(&MYIN, TYPES.as_arg_mut(), ctx)?;
    INSRTC(&MYOUT, TYPES.as_arg_mut(), ctx)?;

    //
    // We will work with a local copy of EPOCH.
    //
    MYTIME = EPOCH;

    //
    // First make sure both types are recognized.
    //
    if !SETC(TYPES.as_arg(), b"<", save.RECOG.as_arg(), ctx)? {
        SETMSG(b"The time types recognized by UNITIM are: TAI, GPS, TT, TDT, JDTDT, TDB, ET, JED, JDTDB. At least one of the inputs (#, #) was not in the list of recognized types. ", ctx);

        ERRCH(b"#", &MYIN, ctx);
        ERRCH(b"#", &MYOUT, ctx);
        SIGERR(b"SPICE(BADTIMETYPE)", ctx)?;
        CHKOUT(b"UNITIM", ctx)?;

        UNITIM = EPOCH;
        return Ok(UNITIM);
    }

    //
    // If the input and output types are the same, just copy the input
    // epoch to the output and call it quits.
    //
    if fstr::eq(&MYIN, &MYOUT) {
        UNITIM = MYTIME;

        CHKOUT(b"UNITIM", ctx)?;
        return Ok(UNITIM);
    }

    //
    // Determine the base types of the input and output types.
    //
    INTDT = ELEMC(&MYIN, save.TYPTDT.as_arg(), ctx)?;
    OUTTDT = ELEMC(&MYOUT, save.TYPTDT.as_arg(), ctx)?;
    INTDB = !INTDT;
    OUTTDB = !OUTTDT;

    //
    // The two types, TDT and TDB, will be used as the fundamental
    // base used in conversions.
    //
    //    TAI, GPS, and JDTDT will be converted to TDT
    //    JED and JDTDB will be converted to TDB.
    //    ET means TDB; TT means TDT.
    //
    //
    if fstr::eq(&MYIN, b"TAI") {
        MYTIME = (MYTIME + save.DTA);
    } else if fstr::eq(&MYIN, b"GPS") {
        MYTIME = (MYTIME + (save.DTA + GPSTAI));
    } else if fstr::eq(&MYIN, b"JDTDT") {
        MYTIME = ((MYTIME - save.JD2000) * save.SECSPD);
    } else if fstr::eq(&MYIN, b"JED") {
        MYTIME = ((MYTIME - save.JD2000) * save.SECSPD);
    } else if fstr::eq(&MYIN, b"JDTDB") {
        MYTIME = ((MYTIME - save.JD2000) * save.SECSPD);
    }

    //
    // At this point, MYTIME has been converted from its input
    // to one of the base types.
    //
    // Next change type from TDB to TDT or vice versa, if
    // required.  (The time is already in TDT or TDB).
    //
    if (INTDT && OUTTDB) {
        TDT = MYTIME;
        TDB = (TDT
            + (save.K
                * f64::sin(
                    ((save.M[0] + (save.M[1] * TDT))
                        + (save.EB * f64::sin((save.M[0] + (save.M[1] * TDT))))),
                )));

        MYTIME = TDB;
    } else if (INTDB && OUTTDT) {
        //
        // What we have to do here is invert the formula used to get
        // TDB from TDT that was used above.
        //
        // Of course solving the equation
        //
        //    TDB = TDT + K*SIN { M0 + M1*TDT + EB*SIN( MO + M1*TDT ) }
        //
        // analytically for TDT if given TDB is no piece of cake.
        // However, we can get as close as we want to TDT if
        // we notice a few tricks.  First, let's let f(t) denote the
        // function
        //
        //    f(t) = SIN( M0 + M1*t + EB*SIN( M0 + M1*t ) )
        //
        // With this simpler notation we can rewrite our problem
        // as that of solving the equation
        //
        //    y = t + K*f(t)
        //
        // for t given y.  Whichever t satisfies this equation will be
        // unique. The uniqueness of the solution is ensured because the
        // expression on the right-hand side of the equation is
        // monotone increasing in t.
        //
        // Let's suppose that t is the solution, then the following
        // is true.
        //
        //    t = y - K*f(t)
        //
        // but we can also replace the t on the right hand side of the
        // equation by y - K*f(t).  Thus
        //
        //    t = y - K*f( y - K*f(t))
        //
        //      = y - K*f( y - K*f( y - K*f(t)))
        //
        //      = y - K*f( y - K*f( y - K*f( y - K*f(t))))
        //
        //      = y - K*f( y - K*f( y - K*f( y - K*f( y - K*f(t)))))
        //      .
        //      .
        //      .
        //      = y - K*f( y - K*f( y - K*f( y - K*f( y - K*f(y - ... )))
        //
        // and so on, for as long as we have patience to perform the
        // substitutions.
        //
        // The point of doing this recursive substitution is that we
        // hope to move t to an insignificant part of the computation.
        // This would seem to have a reasonable chance of success since
        // K is a small number and f is bounded by 1.
        //
        // Following this idea, we will attempt to solve for t using
        // the recursive method outlined below.
        //
        // We will make our first guess at t, call it t_0.
        //
        //  t_0 = y
        //
        // Our next guess, t_1, is given by:
        //
        //  t_1 = y - K*f(t_0)
        //
        // And so on:
        //
        //  t_2 = y - K*f(t_1)        [ = y - K*f(y - K*f(y))            ]
        //  t_3 = y - K*f(t_2)        [ = y - K*f(y - K*f(y - K*f(y)))   ]
        //      .
        //      .
        //      .
        //  t_n = y - K*f(t_(n-1))    [ = y - K*f(y - K*f(y - K*f(y...)))]
        //
        // The questions to ask at this point are:
        //
        //    1) Do the t_i's converge?
        //    2) If they converge, do they converge to t?
        //    3) If they converge to t, how fast do they get there?
        //
        // 1) The sequence of approximations converges.
        //
        //    | t_n - t_(n-1) | =    [ y - K*f( t_(n-1) ) ]
        //                        -  [ y - K*f( t_(n-2) ) ]
        //
        //                      =  K*[ f( t_(n-2) ) - f( t_(n-1) ) ]
        //
        //    The function f has an important property. The absolute
        //    value of its derivative is always less than M1*(1+EB).
        //    This means that for any pair of real numbers s,t
        //
        //       | f(t) - f(s) |  < M1*(1+EB)*| t - s |.
        //
        //    From this observation, we can see that
        //
        //      | t_n - t_(n-1) | < K*M1*(1+EB)*| t_(n-1) - t_(n-2) |
        //
        //    With this fact available, we could (with a bit more work)
        //    conclude that the sequence of t_i's converges and that
        //    it converges at a rate that is at least as fast as the
        //    sequence L, L**2, L**3, ....
        //
        //    Where L = K*M1*(1+EB) << 1.
        //
        //  2) If we let t be the limit of the t_i's then it follows
        //     that
        //
        //        t = y - K*f(t).
        //
        //     or that
        //
        //        y = t + K*f(t).
        //
        //  3) As we already pointed out, the sequence of t_i's
        //     converges at least as fast as the geometric series
        //     L, L**2, ...
        //
        //
        // Since K*M1*(1+EB) is quite small (on the order of 10**-9)
        // 3 iterations should get us as close as we can get to the
        // solution for TDT
        //

        TDB = MYTIME;
        TDT = TDB;

        for I in 1..=3 {
            TDT = (TDB
                - (save.K
                    * f64::sin(
                        ((save.M[0] + (save.M[1] * TDT))
                            + (save.EB * f64::sin((save.M[0] + (save.M[1] * TDT))))),
                    )));
        }

        MYTIME = TDT;
    }

    //
    // Now MYTIME is in the base type of the requested output.
    // If further conversion is required, we do it here.
    //
    if fstr::eq(&MYOUT, b"TAI") {
        MYTIME = (MYTIME - save.DTA);
    } else if fstr::eq(&MYOUT, b"GPS") {
        MYTIME = (MYTIME - (save.DTA + GPSTAI));
    } else if fstr::eq(&MYOUT, b"JDTDT") {
        MYTIME = ((MYTIME / save.SECSPD) + save.JD2000);
    } else if fstr::eq(&MYOUT, b"JED") {
        MYTIME = ((MYTIME / save.SECSPD) + save.JD2000);
    } else if fstr::eq(&MYOUT, b"JDTDB") {
        MYTIME = ((MYTIME / save.SECSPD) + save.JD2000);
    }

    UNITIM = MYTIME;

    CHKOUT(b"UNITIM", ctx)?;
    Ok(UNITIM)
}
