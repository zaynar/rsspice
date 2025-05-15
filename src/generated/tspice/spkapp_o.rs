//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CORLEN: i32 = 5;
const NFLAGS: i32 = 9;
const IXNONE: i32 = 1;
const IXLT: i32 = (IXNONE + 1);
const IXLTS: i32 = (IXLT + 1);
const IXCN: i32 = (IXLTS + 1);
const IXCNS: i32 = (IXCN + 1);
const IXXLT: i32 = (IXCNS + 1);
const IXXLTS: i32 = (IXXLT + 1);
const IXXCN: i32 = (IXXLTS + 1);
const IXXCNS: i32 = (IXXCN + 1);

struct SaveVars {
    FLAGS: ActualCharArray,
    PRVCOR: Vec<u8>,
    FIRST: bool,
    USECN: bool,
    USESTL: bool,
    USELT: bool,
    XMIT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FLAGS = ActualCharArray::new(CORLEN, 1..=NFLAGS);
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut FIRST: bool = false;
        let mut USECN: bool = false;
        let mut USESTL: bool = false;
        let mut USELT: bool = false;
        let mut XMIT: bool = false;

        FIRST = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"NONE"),
                Val::C(b"LT"),
                Val::C(b"LT+S"),
                Val::C(b"CN"),
                Val::C(b"CN+S"),
                Val::C(b"XLT"),
                Val::C(b"XLT+S"),
                Val::C(b"XCN"),
                Val::C(b"XCN+S"),
            ]
            .into_iter();
            FLAGS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        fstr::assign(&mut PRVCOR, b" ");

        Self {
            FLAGS,
            PRVCOR,
            FIRST,
            USECN,
            USESTL,
            USELT,
            XMIT,
        }
    }
}

//$Procedure SPKAPP_O ( S/P Kernel, apparent state )
pub fn SPKAPP_O(
    TARG: i32,
    ET: f64,
    REF: &[u8],
    SOBS: &[f64],
    ABCORR: &[u8],
    STARG: &mut [f64],
    LT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SOBS = DummyArray::new(SOBS, 1..=6);
    let mut STARG = DummyArrayMut::new(STARG, 1..=6);
    let mut CORR = [b' '; CORLEN as usize];
    let mut CORR2 = [b' '; CORLEN as usize];
    let mut SAPOS = StackArray::<f64, 3>::new(1..=3);
    let mut TSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut I: i32 = 0;
    let mut LTSIGN: i32 = 0;
    let mut MAXITR: i32 = 0;
    let mut REFID: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Indices of flags in the FLAGS array:
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
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"SPKAPP_O", ctx)?;
    }

    if (save.FIRST || fstr::ne(ABCORR, &save.PRVCOR)) {
        //
        // The aberration correction flag differs from the value it
        // had on the previous call, if any.  Analyze the new flag.
        //
        // Remove leading and embedded white space from the aberration
        // correction flag, then convert to upper case.
        //
        spicelib::CMPRSS(b" ", 0, ABCORR, &mut CORR2);
        spicelib::UCASE(&CORR2, &mut CORR, ctx);

        //
        // Locate the flag in our list of flags.
        //
        I = spicelib::ISRCHC(&CORR, NFLAGS, save.FLAGS.as_arg());

        if (I == 0) {
            spicelib::SETMSG(b"Requested aberration correction # is not supported.", ctx);
            spicelib::ERRCH(b"#", ABCORR, ctx);
            spicelib::SIGERR(b"SPICE(SPKINVALIDOPTION)", ctx)?;
            spicelib::CHKOUT(b"SPKAPP_O", ctx)?;
            return Ok(());
        }

        //
        // The aberration correction flag is recognized; save it.
        //
        fstr::assign(&mut save.PRVCOR, ABCORR);

        //
        // Set logical flags indicating the attributes of the requested
        // correction.
        //
        save.XMIT = (I > IXCNS);

        save.USELT = ((((I == IXLT) || (I == IXLTS)) || (I == IXXLT)) || (I == IXXLTS));

        save.USESTL = ((I > 1) && spicelib::ODD(I));

        save.USECN = ((((I == IXCN) || (I == IXCNS)) || (I == IXXCN)) || (I == IXXCNS));

        save.FIRST = false;
    }

    //
    // See if the reference frame is a recognized inertial frame.
    //
    spicelib::IRFNUM(REF, &mut REFID, ctx);

    if (REFID == 0) {
        spicelib::SETMSG(
            b"The requested frame \'#\' is not a recognized inertial frame. ",
            ctx,
        );
        spicelib::ERRCH(b"#", REF, ctx);
        spicelib::SIGERR(b"SPICE(BADFRAME)", ctx)?;
        spicelib::CHKOUT(b"SPKAPP_O", ctx)?;
        return Ok(());
    }

    //
    // Determine the sign of the light time offset.
    //
    if save.XMIT {
        LTSIGN = 1;
    } else {
        LTSIGN = -1;
    }

    //
    // Find the geometric state of the target body with respect to the
    // solar system barycenter. Subtract the state of the observer
    // to get the relative state. Use this to compute the one-way
    // light time.
    //
    spicelib::SPKSSB(TARG, ET, REF, STARG.as_slice_mut(), ctx)?;
    spicelib::VSUBG(STARG.as_slice(), SOBS.as_slice(), 6, TSTATE.as_slice_mut());
    spicelib::MOVED(TSTATE.as_slice(), 6, STARG.as_slice_mut());

    *LT = (spicelib::VNORM(STARG.as_slice()) / spicelib::CLIGHT());

    //
    // To correct for light time, find the state of the target body
    // at the current epoch minus the one-way light time. Note that
    // the observer remains where he is.
    //
    if save.USELT {
        MAXITR = 1;
    } else if save.USECN {
        MAXITR = 3;
    } else {
        MAXITR = 0;
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXITR;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SPKSSB(
                TARG,
                (ET + ((LTSIGN as f64) * *LT)),
                REF,
                STARG.as_slice_mut(),
                ctx,
            )?;
            spicelib::VSUBG(STARG.as_slice(), SOBS.as_slice(), 6, TSTATE.as_slice_mut());
            spicelib::MOVED(TSTATE.as_slice(), 6, STARG.as_slice_mut());
            *LT = (spicelib::VNORM(STARG.as_slice()) / spicelib::CLIGHT());

            I += m3__;
        }
    }

    //
    // At this point, STARG contains the light time corrected
    // state of the target relative to the observer.
    //
    // If stellar aberration correction is requested, perform it now.
    //
    // Stellar aberration corrections are not applied to the target's
    // velocity.
    //
    if save.USESTL {
        if save.XMIT {
            //
            // This is the transmission case.
            //
            // Compute the position vector obtained by applying
            // "reception" stellar aberration to STARG.
            //
            spicelib::STLABX(
                STARG.as_slice(),
                SOBS.subarray(4),
                SAPOS.as_slice_mut(),
                ctx,
            )?;
            spicelib::VEQU(SAPOS.as_slice(), STARG.as_slice_mut());
        } else {
            //
            // This is the reception case.
            //
            // Compute the position vector obtained by applying
            // "reception" stellar aberration to STARG.
            //
            spicelib::STELAB(
                STARG.as_slice(),
                SOBS.subarray(4),
                SAPOS.as_slice_mut(),
                ctx,
            )?;
            spicelib::VEQU(SAPOS.as_slice(), STARG.as_slice_mut());
        }
    }

    spicelib::CHKOUT(b"SPKAPP_O", ctx)?;
    Ok(())
}
