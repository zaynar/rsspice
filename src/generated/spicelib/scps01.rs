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
const ERRLEN: i32 = 240;
const NAMLEN: i32 = 60;
const MSGLEN: i32 = 320;
const INITSC: i32 = 0;
const INITID: f64 = -100000000000000000000.0;
const NFLIDX: i32 = 1;
const OFFIDX: i32 = (NFLIDX + 1);
const MODIDX: i32 = (OFFIDX + 1);
const NKITEM: i32 = MODIDX;

struct SaveVars {
    CMP: ActualCharArray,
    STRERR: Vec<u8>,
    NAMLST: ActualCharArray,
    CMPTKS: StackArray<f64, 10>,
    CMPVAL: StackArray<f64, 10>,
    MODULI: StackArray<f64, 10>,
    OFFSET: StackArray<f64, 10>,
    N: i32,
    NFIELD: i32,
    PNTR: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CMP = ActualCharArray::new(DPLEN, 1..=MXNFLD);
        let mut STRERR = vec![b' '; ERRLEN as usize];
        let mut NAMLST = ActualCharArray::new(NAMLEN, 1..=NKITEM);
        let mut CMPTKS = StackArray::<f64, 10>::new(1..=MXNFLD);
        let mut CMPVAL = StackArray::<f64, 10>::new(1..=MXNFLD);
        let mut MODULI = StackArray::<f64, 10>::new(1..=MXNFLD);
        let mut OFFSET = StackArray::<f64, 10>::new(1..=MXNFLD);
        let mut N: i32 = 0;
        let mut NFIELD: i32 = 0;
        let mut PNTR: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"SCLK01_N_FIELDS"),
                Val::C(b"SCLK01_OFFSETS"),
                Val::C(b"SCLK01_MODULI"),
            ]
            .into_iter();
            NAMLST
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            CMP,
            STRERR,
            NAMLST,
            CMPTKS,
            CMPVAL,
            MODULI,
            OFFSET,
            N,
            NFIELD,
            PNTR,
        }
    }
}

/// Convert type 1 SCLK string to ticks
///
/// Convert a character representation of a type 1 spacecraft clock
/// count to ticks.
///
/// # Required Reading
///
/// * [SCLK](crate::required_reading::sclk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  SC         I   NAIF spacecraft ID code.
///  CLKSTR     I   Character representation of a clock count.
///  ERROR      O   Parsing error flag.
///  MSG        O   Output message for parsing error.
///  TICKS      O   Number of ticks represented by the clock count.
///  MXNFLD     P   Maximum number of allowed fields in an SCLK string.
///  DELIMS     P   The accepted delimiters of an SCLK string.
///  DPLEN      P   Maximum width of a clock field.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SC       is a NAIF spacecraft identification code. See the
///           `Examples' section below, and also the NAIF_IDS
///           required reading file for a complete list of body ID
///           codes.
///
///
///  CLKSTR   on input is the character representation of a
///           spacecraft clock count (SCLK), without a partition
///           number.
///
///           Using Galileo as an example, a SCLK string without
///           a partition number has the form
///
///                          wwwwwwww:xx:y:z
///
///           where z is a mod-8 counter (values 0-7) which
///           increments approximately once every 8 1/3 ms., y is a
///           mod-10 counter (values 0-9) which increments once
///           every time z turns over, i.e., approximately once every
///           66 2/3 ms., xx is a mod-91 (values 0-90) counter
///           which increments once every time y turns over, i.e.,
///           once every 2/3 seconds. wwwwwwww is the Real-Time
///           Image Count (RIM), which increments once every time
///           xx turns over, i.e., once every 60 2/3 seconds. The
///           roll-over expression for the RIM is 16777215, which
///           corresponds to approximately 32 years.
///
///           wwwwwwww, xx, y, and z are referred to interchangeably
///           as the fields or components of the spacecraft count.
///           SCLK components may be separated by any of the
///           single character delimiters in the string DELIMS, with
///           any number of spaces separating the components and
///           the delimiters. The presence of the RIM component
///           is required. Successive components may be omitted, and
///           in such cases are assumed to represent zero values.
///
///           Values for the individual components may exceed the
///           maximum expected values. For instance, '0:0:0:9' is
///           an acceptable Galileo clock string, and indicates the
///           same time interval as '0:0:1:1'.
///
///           Consecutive delimiters containing no intervening digits
///           are treated as if they delimit zero components, except
///           in the case of blanks.  Consecutive blanks are treated
///           as a single blank.
///
///           Trailing zeros should always be included to match the
///           length of the counter.  For example, a Galileo clock
///           count of '25684.90' should not be represented as
///           '25684.9'.
///
///           Some spacecraft clock components have offset, or
///           starting, values different from zero.  For example,
///           with an offset value of 1, a mod 20 counter would
///           cycle from 1 to 20 instead of from 0 to 19.
///
///           See the SCLK required reading for a detailed
///           description of the Galileo, Mars Observer, and Voyager
///           clock formats.
///
///           See the `Examples' section in SCPS01, below.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ERROR    is .TRUE. if an error occurred parsing the input clock
///           string and converting it to ticks.
///
///  MSG      is the message generated if an error occurred parsing
///           the input clock string.
///
///  TICKS    is the number of "ticks" corresponding to the input
///           spacecraft clock string CLKSTR.  "Ticks" are the units
///           in which encoded SCLK strings are represented.
///
///           A typical Galileo SCLK string looks like
///
///                        'wwwwwwww xx y z',
///
///           as described above. Since z is the mod-8 (one tick)
///           counter, the number of ticks represented by y is 8*y.
///           And since y is the mod-10 counter, the number of ticks
///           represented by xx is 10*8*xx. The total number of
///           ticks represented by the above string is
///
///                         wwwwwwww( 7280 ) +
///                               xx(   80 ) +
///                                y(    8 ) +
///                                z
///
///           Clock strings for other spacecraft are converted in
///           a similar manner.
///
///           See $Examples below.
/// ```
///
/// # Parameters
///
/// ```text
///  See the INCLUDE file sclk.inc.
/// ```
///
/// # Exceptions
///
/// ```text
///  In the case of any SPICELIB error occurring, ERROR is set to
///  .TRUE. and MSG to 'SPICELIB error detected.'.
///
///  1)  This routine assumes that that an SCLK kernel appropriate to
///      the spacecraft clock identified by the input argument SC has
///      been loaded. If an SCLK kernel has not been loaded, does not
///      contain all of the required data, or contains invalid data, an
///      error is signaled by a routine in the call tree of this
///      routine. The output argument TICKS will not be modified. and
///      MSG
///
///      The variables that must be set by the SCLK kernel are:
///
///         -  The number of fields in an (unabridged) SCLK string
///         -  The output delimiter code
///         -  The parallel time system code
///         -  The moduli of the fields of an SCLK string
///         -  The offsets for each clock field.
///         -  The SCLK coefficients array
///         -  The partition start times
///         -  The partition end times
///
///  2)  When using SCLK kernels that map SCLK to a time system other
///      than ET (also called barycentric dynamical time---`TDB'), it
///      is necessary to have a leapseconds kernel loaded at the time
///      this routine is called. If a leapseconds kernel is required
///      for conversion between SCLK and ET but is not loaded, an error
///      is signaled by a routine in the call tree of this routine. The
///      output argument TICKS will not be modified.
///
///      The time system that an SCLK kernel maps SCLK to is indicated
///      by the variable SCLK_TIME_SYSTEM_nn in the kernel, where nn
///      is the negative of the NAIF integer code for the spacecraft.
///      The time system used in a kernel is TDB if and only if the
///      variable is assigned the value 1.
///
///  3)  If any of the following kernel variables have invalid values,
///      an error is signaled by a routine in the call tree of this
///      routine:
///
///         -  The time system code
///         -  The number of SCLK coefficients
///         -  The number of partition start times
///         -  The number of partition end times
///         -  The number of fields of a SCLK string
///         -  The number of moduli for a SCLK string
///
///      If the number of values for any item read from the kernel
///      pool exceeds the maximum allowed value, it is may not be
///      possible to diagnose the error correctly, since overwriting
///      of memory may occur. This particular type of error is not
///      diagnosed by this routine.
///
///  4)  The input argument CLKSTR may be invalid for a variety of
///      reasons:
///
///         -- One of the extracted clock components cannot be parsed
///            as an integer
///
///         -- CLKSTR contains too many components
///
///         -- the value  of one of the components is less than the
///            offset value
///
///      If any of these conditions is detected, no error is signaled.
///      ERROR is set to .TRUE. and MSG contains the specific issue.
///      The output argument TICKS will not be modified.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine converts a character string representation of a
///  spacecraft clock count into the number of ticks represented
///  by the clock count. An important distinction between this type
///  of conversion and that carried out by SCENCD is that this routine
///  treats spacecraft clock times as representations of time
///  intervals, not absolute times.
///
///  This routine does not make use of any partition information.
///  See SCENCD for details on how to make use of partition numbers.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Below are some examples illustrating various inputs and the
///      resulting outputs for the Galileo spacecraft.
///
///      CLKSTR                TICKS
///      ----------------      --------------------
///      '0:0:0:1'             1
///      '0:0:1'               8
///      '0:1'                 80
///      '1'                   7280
///      '1 0 0 0'             7280
///      '1,0,0,0'             7280
///      '1:90'                14480
///      '1:9'                 8000
///      '1:09'                8000
///      '0-0-10'              80   |--  Third component is supposed
///      '0-1-0'               80   |    to be a mod-10 count.
///      '0/1/0'               Error: '/' is not an accepted delimiter.
///      '1: 00 : 0 : 1'       7281
///      '1:::1'               7281
///      '1.1.1.1.1'           Error: Too many components
///      '1.1.1.1.'            Error: The last delimiter signals that
///                                   a fifth component will follow.
///
///
///      The following examples are for the Voyager 2 spacecraft. Note
///      that the last component of the Voyager clock has an offset
///      value of 1.
///
///      CLKSTR                TICKS
///      ----------------      --------------------
///      '0.0.001'             0
///      '0:0:002'             1
///      '0:01'                800
///      '1'                   48000
///      '1.0'                 48000
///      '1.0.0'               Error: The 3rd component is never 0.
///      '0.0:100'             99
///      '0-60-1'              48000
///      '1-1-1'               48800
///      '1-1-2'               48801
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  An SCLK kernel appropriate to the spacecraft clock identified
///      by SC must be loaded at the time this routine is called.
///
///  2)  If the SCLK kernel used with this routine does not map SCLK
///      directly to barycentric dynamical time, a leapseconds kernel
///      must be loaded at the time this routine is called.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 20-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
///         Added introductory paragraph to $Exceptions section, split
///         entry #1 into two, and added description of behavior in entry
///         #4.
///
/// -    SPICELIB Version 1.1.0, 11-FEB-2008 (NJB)
///
///         Global parameters are now declared in the Fortran
///         INCLUDE file sclk.inc.
///
/// -    SPICELIB Version 1.0.0, 25-FEB-1993 (JML)
/// ```
pub fn scps01(
    ctx: &mut SpiceContext,
    sc: i32,
    clkstr: &str,
    error: &mut bool,
    msg: &mut str,
    ticks: &mut f64,
) -> crate::Result<()> {
    SCPS01(
        sc,
        clkstr.as_bytes(),
        error,
        fstr::StrBytes::new(msg).as_mut(),
        ticks,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SCPS01 ( Convert type 1 SCLK string to ticks )
pub fn SCPS01(
    SC: i32,
    CLKSTR: &[u8],
    ERROR: &mut bool,
    MSG: &mut [u8],
    TICKS: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Following are parameters for the indices within the
    // array NAMLST of the kernel variable names.
    //

    //
    // Local variables
    //

    //
    // Save everything
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SCPS01", ctx)?;
    }

    //
    // Start off with the error flag and message set for a regular
    // SPICE error.
    //
    *ERROR = true;
    fstr::assign(MSG, b"SPICELIB error detected.");

    //
    // Our first piece of business is to look up all of the data
    // we require from the kernel pool. We must form the names
    // of the items we want using the input S/C ID code. The items
    // we need are:
    //
    //    -  The number of fields in an (unabridged) SCLK string
    //    -  The moduli of the fields of an SCLK string
    //    -  The offsets for each clock field.
    //

    SCLI01(
        &save.NAMLST[NFLIDX],
        SC,
        MXNFLD,
        &mut save.N,
        std::slice::from_mut(&mut save.NFIELD),
        ctx,
    )?;
    SCLD01(
        &save.NAMLST[MODIDX],
        SC,
        MXNFLD,
        &mut save.N,
        save.MODULI.as_slice_mut(),
        ctx,
    )?;
    SCLD01(
        &save.NAMLST[OFFIDX],
        SC,
        MXNFLD,
        &mut save.N,
        save.OFFSET.as_slice_mut(),
        ctx,
    )?;

    //
    // Don't try to continue if we had a lookup error.
    //
    if FAILED(ctx) {
        CHKOUT(b"SCPS01", ctx)?;
        return Ok(());
    }

    //
    // If our clock string is blank, we can stop now.
    //
    if fstr::eq(CLKSTR, b" ") {
        fstr::assign(
            MSG,
            b"Non partition part of the input clock string is blank.",
        );

        *ERROR = true;

        CHKOUT(b"SCPS01", ctx)?;
        return Ok(());
    }

    //
    // Determine how many ticks is each field is worth.
    //
    save.CMPTKS[save.NFIELD] = 1.0;

    for I in intrinsics::range((save.NFIELD - 1), 1, -1) {
        save.CMPTKS[I] = (save.CMPTKS[(I + 1)] * save.MODULI[(I + 1)]);
    }

    //
    // Parse the clock components from the input string. There should
    // be at most NFIELD of them, but, in order to check for too long
    // a clock string, we'll let LPARSM take up to MXNFLD components and
    // then test for an error.
    //
    LPARSM(CLKSTR, DELIMS, MXNFLD, &mut save.N, save.CMP.as_arg_mut());

    //
    // If the string has too many fields for the specified spacecraft
    // then signal an error.
    //
    if (save.N > save.NFIELD) {
        *ERROR = true;

        fstr::assign(
            MSG,
            b"Input clock string # has # fields; maximum for this spacecraft clock is #.",
        );

        REPMC(&MSG.to_vec(), b"#", CLKSTR, MSG);
        REPMI(&MSG.to_vec(), b"#", save.N, MSG, ctx);
        REPMI(&MSG.to_vec(), b"#", save.NFIELD, MSG, ctx);

        CHKOUT(b"SCPS01", ctx)?;
        return Ok(());
    }

    //
    // Convert each of the components into numbers.  Error if any
    // of the conversions screw up.
    //
    for I in 1..=save.N {
        if fstr::eq(save.CMP.get(I), b" ") {
            save.CMPVAL[I] = save.OFFSET[I];
        } else {
            NPARSD(
                &save.CMP[I],
                &mut save.CMPVAL[I],
                &mut save.STRERR,
                &mut save.PNTR,
                ctx,
            );
        }

        if fstr::ne(&save.STRERR, b" ") {
            *ERROR = true;

            fstr::assign(MSG, b"Could not parse SCLK component # from # as a number.");

            REPMC(&MSG.to_vec(), b"#", &save.CMP[I], MSG);
            REPMC(&MSG.to_vec(), b"#", CLKSTR, MSG);

            CHKOUT(b"SCPS01", ctx)?;
            return Ok(());
        }

        //
        // Subtract off the offset value so that we can do base ten
        // arithmetic.  Also, if any of the components become negative
        // as a result of the subtraction, then that component must
        // have been invalid.
        //
        save.CMPVAL[I] = (save.CMPVAL[I] - save.OFFSET[I]);

        if (f64::round(save.CMPVAL[I]) < 0.0) {
            *ERROR = true;

            fstr::assign(
                MSG,
                b"Component number #, # in the SCLK string  # is invalid.",
            );

            REPMI(&MSG.to_vec(), b"#", I, MSG, ctx);
            REPMC(&MSG.to_vec(), b"#", &save.CMP[I], MSG);
            REPMC(&MSG.to_vec(), b"#", CLKSTR, MSG);

            CHKOUT(b"SCPS01", ctx)?;
            return Ok(());
        }
    }

    //
    // Convert to ticks by multiplying the value of each component by
    // the number of ticks each component count represents, and then
    // add up the results.
    //
    *TICKS = 0.0;

    for I in 1..=save.N {
        *TICKS = (*TICKS + (save.CMPVAL[I] * save.CMPTKS[I]));
    }

    *ERROR = false;

    fstr::assign(MSG, b" ");

    CHKOUT(b"SCPS01", ctx)?;
    Ok(())
}
