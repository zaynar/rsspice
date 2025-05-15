//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CNVTOL: f64 = 0.000001;
const NWMAX: i32 = 15;
const NWDIST: i32 = 5;
const NWSEP: i32 = 5;
const NWRR: i32 = 5;
const NWUDS: i32 = 5;
const NWPA: i32 = 5;
const NWILUM: i32 = 5;
const ADDWIN: f64 = 0.5;
const FRMNLN: i32 = 32;
const FOVTLN: i32 = 40;
const FTCIRC: &[u8] = b"CIRCLE";
const FTELLI: &[u8] = b"ELLIPSE";
const FTPOLY: &[u8] = b"POLYGON";
const FTRECT: &[u8] = b"RECTANGLE";
const ANNULR: &[u8] = b"ANNULAR";
const ANY: &[u8] = b"ANY";
const PARTL: &[u8] = b"PARTIAL";
const FULL: &[u8] = b"FULL";
const DSSHAP: &[u8] = b"DSK";
const EDSHAP: &[u8] = b"ELLIPSOID";
const PTSHAP: &[u8] = b"POINT";
const RYSHAP: &[u8] = b"RAY";
const SPSHAP: &[u8] = b"SPHERE";
const NOCTYP: i32 = 4;
const OCLLN: i32 = 7;
const SHPLEN: i32 = 9;
const MAXVRT: i32 = 10000;
const CIRFOV: &[u8] = b"CIRCLE";
const ELLFOV: &[u8] = b"ELLIPSE";
const POLFOV: &[u8] = b"POLYGON";
const RECFOV: &[u8] = b"RECTANGLE";
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const SPK: &[u8] = b"gfevnt.bsp";
const PCK: &[u8] = b"gfevnt.pck";
const MEDTOL: f64 = 0.0001;
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 80;
const LBCELL: i32 = -5;
const MAXVAL: i32 = 20000;
const MAXPAR: i32 = 10;
const NENAMES: i32 = 3;
const NXNAMES: i32 = 4;
const NC: i32 = 7;
const NCORR: i32 = 9;

struct SaveVars {
    ENAMES: ActualCharArray,
    XNAMES: ActualCharArray,
    CNAMES: ActualCharArray,
    CORR: ActualCharArray,
    TITLE: Vec<u8>,
    TIME0: Vec<u8>,
    TIME1: Vec<u8>,
    STEP: f64,
    REFVAL: f64,
    ET0: f64,
    ET1: f64,
    ADJUST: f64,
    CNFINE: ActualArray<f64>,
    RESULT: ActualArray<f64>,
    WORK: ActualArray2D<f64>,
    HANDLE: i32,
    BAIL: bool,
    RPT: bool,
    EVENT: Vec<u8>,
    RELATE: Vec<u8>,
    QNPARS: i32,
    QPNAMS: ActualCharArray,
    QCPARS: ActualCharArray,
    QDPARS: StackArray<f64, 10>,
    QIPARS: StackArray<i32, 10>,
    QLPARS: StackArray<bool, 10>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ENAMES = ActualCharArray::new(LNSIZE, 1..=NENAMES);
        let mut XNAMES = ActualCharArray::new(LNSIZE, 1..=NXNAMES);
        let mut CNAMES = ActualCharArray::new(LNSIZE, 1..=NC);
        let mut CORR = ActualCharArray::new(LNSIZE, 1..=NCORR);
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut TIME0 = vec![b' '; LNSIZE as usize];
        let mut TIME1 = vec![b' '; LNSIZE as usize];
        let mut STEP: f64 = 0.0;
        let mut REFVAL: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut ADJUST: f64 = 0.0;
        let mut CNFINE = ActualArray::<f64>::new(LBCELL..=MAXVAL);
        let mut RESULT = ActualArray::<f64>::new(LBCELL..=MAXVAL);
        let mut WORK = ActualArray2D::<f64>::new(LBCELL..=MAXVAL, 1..=NWMAX);
        let mut HANDLE: i32 = 0;
        let mut BAIL: bool = false;
        let mut RPT: bool = false;
        let mut EVENT = vec![b' '; LNSIZE as usize];
        let mut RELATE = vec![b' '; LNSIZE as usize];
        let mut QNPARS: i32 = 0;
        let mut QPNAMS = ActualCharArray::new(LNSIZE, 1..=MAXPAR);
        let mut QCPARS = ActualCharArray::new(LNSIZE, 1..=MAXPAR);
        let mut QDPARS = StackArray::<f64, 10>::new(1..=MAXPAR);
        let mut QIPARS = StackArray::<i32, 10>::new(1..=MAXPAR);
        let mut QLPARS = StackArray::<bool, 10>::new(1..=MAXPAR);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b">"),
                Val::C(b"="),
                Val::C(b"<"),
                Val::C(b"LOCMAX"),
                Val::C(b"LOCMIN"),
                Val::C(b"ABSMAX"),
                Val::C(b"ABSMIN"),
            ]
            .into_iter();
            CNAMES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"NONE"),
                Val::C(b"lt"),
                Val::C(b" lt+s"),
                Val::C(b" cn"),
                Val::C(b" cn+s"),
                Val::C(b"XLT"),
                Val::C(b"XLT+S"),
                Val::C(b"XCN"),
                Val::C(b"XCN+S"),
            ]
            .into_iter();
            CORR.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"DISTANCE"),
                Val::C(b"ANGULAR SEPARATION"),
                Val::C(b"COORDINATE"),
            ]
            .into_iter();
            ENAMES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"ANGSPD"),
                Val::C(b"APPDIAM"),
                Val::C(b"PHASE"),
                Val::C(b"RNGRAT"),
            ]
            .into_iter();
            XNAMES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ENAMES,
            XNAMES,
            CNAMES,
            CORR,
            TITLE,
            TIME0,
            TIME1,
            STEP,
            REFVAL,
            ET0,
            ET1,
            ADJUST,
            CNFINE,
            RESULT,
            WORK,
            HANDLE,
            BAIL,
            RPT,
            EVENT,
            RELATE,
            QNPARS,
            QPNAMS,
            QCPARS,
            QDPARS,
            QIPARS,
            QLPARS,
        }
    }
}

//$Procedure F_GFEVNT ( GFEVNT family tests )
pub fn F_GFEVNT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

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
    // Geometric quantity name.
    //
    //

    //
    // Relational string
    //

    //
    // Quantity definition parameter arrays:
    //

    //
    // Routines to set step size, refine transition times
    // and report work.
    //

    //
    // Initial values
    //

    //
    // Coded and usable event conditions.
    //

    //
    // Coded but not yet usable event conditions. Use
    // these condition names to test unknown EVENT values.
    // Eventualy the elements of this list will exist in the
    // ENAMES list.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_GFEVNT", ctx)?;

    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    // Leapseconds:  Note that the LSK is deleted after loading, so we
    // don't have to clean it up later.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a PCK, load using FURNSH.
    //
    testutil::T_PCK08(PCK, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create an SPK, load using FURNSH.
    //
    testutil::TSTSPK(SPK, true, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a confinement window from ET0 and ET1.
    //
    fstr::assign(&mut save.TIME0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.TIME1, b"2000 APR 1  00:00:00 TDB");

    spicelib::STR2ET(&save.TIME0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.TIME1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXVAL, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXVAL, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Case 1
    //
    //   Test the implemented geometric quantities will cause an
    //   error signal for a QNPARS beyond the GFEVNT MAXPAR value.
    //
    fstr::assign(&mut save.RELATE, b"ABSMAX");
    save.QNPARS = (MAXPAR + 1);
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QCPARS.get_mut(1), b"MOON");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QCPARS.get_mut(3), b"LT+S");
    save.STEP = (1.0 * spicelib::SPD());
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    save.RPT = false;
    save.BAIL = false;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add 2 points to the confinement interval window.
    //
    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=NENAMES {
        fstr::assign(&mut save.EVENT, save.ENAMES.get(I));

        fstr::assign(
            &mut save.TITLE,
            &fstr::concat(b"QNPARS > MAXPARS: ", &save.EVENT),
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::GFEVNT(
            spicelib::GFSTEP,
            spicelib::GFREFN,
            &save.EVENT,
            save.QNPARS,
            save.QPNAMS.as_arg(),
            save.QCPARS.as_arg(),
            save.QDPARS.as_slice(),
            save.QIPARS.as_slice(),
            save.QLPARS.as_slice(),
            &save.RELATE,
            save.REFVAL,
            CNVTOL,
            save.ADJUST,
            save.CNFINE.as_slice(),
            save.RPT,
            spicelib::GFREPI,
            spicelib::GFREPU,
            spicelib::GFREPF,
            MAXVAL,
            NWMAX,
            save.WORK.as_slice_mut(),
            save.BAIL,
            spicelib::GFBAIL,
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;
    }

    //
    // Case 2
    //
    //   Test the unimplemented geometric quantities will cause an
    //   error signal.
    //
    fstr::assign(&mut save.RELATE, b"ABSMAX");
    save.QNPARS = 3;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QCPARS.get_mut(1), b"MOON");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QCPARS.get_mut(3), b"LT+S");
    save.STEP = (1.0 * spicelib::SPD());
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    save.RPT = false;
    save.BAIL = false;

    for I in 1..=NXNAMES {
        spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
        spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Add 2 points to the confinement interval window.
        //
        spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

        fstr::assign(&mut save.EVENT, save.XNAMES.get(I));

        fstr::assign(
            &mut save.TITLE,
            &fstr::concat(b"Unimplemented quantity: ", &save.EVENT),
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::GFEVNT(
            spicelib::GFSTEP,
            spicelib::GFREFN,
            &save.EVENT,
            save.QNPARS,
            save.QPNAMS.as_arg(),
            save.QCPARS.as_arg(),
            save.QDPARS.as_slice(),
            save.QIPARS.as_slice(),
            save.QLPARS.as_slice(),
            &save.RELATE,
            save.REFVAL,
            CNVTOL,
            save.ADJUST,
            save.CNFINE.as_slice(),
            save.RPT,
            spicelib::GFREPI,
            spicelib::GFREPU,
            spicelib::GFREPF,
            MAXVAL,
            NWMAX,
            save.WORK.as_slice_mut(),
            save.BAIL,
            spicelib::GFBAIL,
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;
    }

    //
    // Case 3
    //
    //   Cause an error signal due to an invalid relation operator.
    //
    fstr::assign(&mut save.EVENT, b"DISTANCE");
    save.QNPARS = 3;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QCPARS.get_mut(1), b"MOON");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QCPARS.get_mut(3), b"LT+S");
    save.STEP = (1.0 * spicelib::SPD());
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    save.RPT = false;
    save.BAIL = false;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.RELATE, b"===");

    testutil::TCASE(&fstr::concat(b"Invalid: ", &save.RELATE), ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.EVENT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.RELATE,
        save.REFVAL,
        CNVTOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        MAXVAL,
        NWMAX,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    //
    // Case 4
    //
    // Run a valid, simple search using each relation operator
    // and each aberration correction.
    //
    fstr::assign(&mut save.EVENT, b"DISTANCE");
    save.QNPARS = 3;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QCPARS.get_mut(1), b"MOON");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    save.STEP = (1.0 * spicelib::SPD());
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    save.RPT = false;
    save.BAIL = false;

    for I in 1..=NC {
        for J in 1..=NCORR {
            spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
            spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.RELATE, save.CNAMES.get(I));
            fstr::assign(save.QCPARS.get_mut(3), save.CORR.get(J));

            testutil::TCASE(
                &fstr::concat(
                    fstr::substr(&save.RELATE, 1..=7),
                    fstr::substr(save.CORR.get(J), 1..=6),
                ),
                ctx,
            )?;

            spicelib::GFEVNT(
                spicelib::GFSTEP,
                spicelib::GFREFN,
                &save.EVENT,
                save.QNPARS,
                save.QPNAMS.as_arg(),
                save.QCPARS.as_arg(),
                save.QDPARS.as_slice(),
                save.QIPARS.as_slice(),
                save.QLPARS.as_slice(),
                &save.RELATE,
                save.REFVAL,
                CNVTOL,
                save.ADJUST,
                save.CNFINE.as_slice(),
                save.RPT,
                spicelib::GFREPI,
                spicelib::GFREPU,
                spicelib::GFREPF,
                MAXVAL,
                NWMAX,
                save.WORK.as_slice_mut(),
                save.BAIL,
                spicelib::GFBAIL,
                save.RESULT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // Case 5
    //
    //   Event DISTANCE - Test for an error when QPNAMS has
    //   an empty element.
    //
    fstr::assign(&mut save.EVENT, b"DISTANCE");
    save.QNPARS = 3;
    save.STEP = (1.0 * spicelib::SPD());
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.RELATE, b"=");
    save.RPT = false;
    save.BAIL = false;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.QNPARS {
        fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
        fstr::assign(save.QCPARS.get_mut(1), b"MOON");
        fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
        fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
        fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
        fstr::assign(save.QCPARS.get_mut(3), b"LT+S");

        testutil::TCASE(&fstr::concat(b"Empty value: ", save.QPNAMS.get(I)), ctx)?;
        fstr::assign(save.QPNAMS.get_mut(I), b" ");

        spicelib::GFEVNT(
            spicelib::GFSTEP,
            spicelib::GFREFN,
            &save.EVENT,
            save.QNPARS,
            save.QPNAMS.as_arg(),
            save.QCPARS.as_arg(),
            save.QDPARS.as_slice(),
            save.QIPARS.as_slice(),
            save.QLPARS.as_slice(),
            &save.RELATE,
            save.REFVAL,
            CNVTOL,
            save.ADJUST,
            save.CNFINE.as_slice(),
            save.RPT,
            spicelib::GFREPI,
            spicelib::GFREPU,
            spicelib::GFREPF,
            MAXVAL,
            NWMAX,
            save.WORK.as_slice_mut(),
            save.BAIL,
            spicelib::GFBAIL,
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(MISSINGVALUE)", OK, ctx)?;
    }

    //
    // Case 6
    //
    //   Event ANGULAR SEPARATION - Test for an error when QPNAMS has
    //   an empty element.
    //
    fstr::assign(&mut save.EVENT, b"ANGULAR SEPARATION");
    save.QNPARS = 8;
    save.STEP = (1.0 * spicelib::SPD());
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.RELATE, b"=");
    save.RPT = false;
    save.BAIL = false;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.QNPARS {
        fstr::assign(save.QPNAMS.get_mut(1), b"TARGET1");
        fstr::assign(save.QCPARS.get_mut(1), b"MOON");
        fstr::assign(save.QPNAMS.get_mut(2), b"FRAME1");
        fstr::assign(save.QCPARS.get_mut(2), b"NULL");
        fstr::assign(save.QPNAMS.get_mut(3), b"SHAPE1");
        fstr::assign(save.QCPARS.get_mut(3), b"SPHERE");
        fstr::assign(save.QPNAMS.get_mut(4), b"TARGET2");
        fstr::assign(save.QCPARS.get_mut(4), b"EARTH");
        fstr::assign(save.QPNAMS.get_mut(5), b"FRAME2");
        fstr::assign(save.QCPARS.get_mut(5), b"NULL");
        fstr::assign(save.QPNAMS.get_mut(6), b"SHAPE2");
        fstr::assign(save.QCPARS.get_mut(6), b"SPHERE");
        fstr::assign(save.QPNAMS.get_mut(7), b"OBSERVER");
        fstr::assign(save.QCPARS.get_mut(7), b"SUN");
        fstr::assign(save.QPNAMS.get_mut(8), b"ABCORR");
        fstr::assign(save.QCPARS.get_mut(8), b"NONE");

        testutil::TCASE(&fstr::concat(b"Empty value: ", save.QPNAMS.get(I)), ctx)?;
        fstr::assign(save.QPNAMS.get_mut(I), b" ");

        spicelib::GFEVNT(
            spicelib::GFSTEP,
            spicelib::GFREFN,
            &save.EVENT,
            save.QNPARS,
            save.QPNAMS.as_arg(),
            save.QCPARS.as_arg(),
            save.QDPARS.as_slice(),
            save.QIPARS.as_slice(),
            save.QLPARS.as_slice(),
            &save.RELATE,
            save.REFVAL,
            CNVTOL,
            save.ADJUST,
            save.CNFINE.as_slice(),
            save.RPT,
            spicelib::GFREPI,
            spicelib::GFREPU,
            spicelib::GFREPF,
            MAXVAL,
            NWMAX,
            save.WORK.as_slice_mut(),
            save.BAIL,
            spicelib::GFBAIL,
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(MISSINGVALUE)", OK, ctx)?;
    }

    //
    // Case 7
    //
    //   Event COORDINATE - Test for an error when QPNAMS has
    //   an empty element.
    //
    fstr::assign(&mut save.EVENT, b"COORDINATE");
    save.QNPARS = 10;
    save.STEP = (1.0 * spicelib::SPD());
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.RELATE, b"=");
    save.RPT = false;
    save.BAIL = false;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.QNPARS {
        fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
        fstr::assign(save.QCPARS.get_mut(1), b"MOON");

        fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
        fstr::assign(save.QCPARS.get_mut(2), b"EARTH");

        fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
        fstr::assign(save.QCPARS.get_mut(3), b"NONE");

        fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
        fstr::assign(save.QCPARS.get_mut(4), b"RECTANGULAR");

        fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
        fstr::assign(save.QCPARS.get_mut(5), b"X");

        fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
        fstr::assign(save.QCPARS.get_mut(6), b"J2000");

        fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
        fstr::assign(save.QCPARS.get_mut(7), b"POSITION");

        fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
        fstr::assign(save.QCPARS.get_mut(8), b" ");

        fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
        fstr::assign(save.QCPARS.get_mut(9), b" ");

        fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");
        save.QDPARS[1] = 0.0;
        save.QDPARS[2] = 0.0;
        save.QDPARS[3] = 0.0;

        testutil::TCASE(&fstr::concat(b"Empty value: ", save.QPNAMS.get(I)), ctx)?;
        fstr::assign(save.QPNAMS.get_mut(I), b" ");

        spicelib::GFEVNT(
            spicelib::GFSTEP,
            spicelib::GFREFN,
            &save.EVENT,
            save.QNPARS,
            save.QPNAMS.as_arg(),
            save.QCPARS.as_arg(),
            save.QDPARS.as_slice(),
            save.QIPARS.as_slice(),
            save.QLPARS.as_slice(),
            &save.RELATE,
            save.REFVAL,
            CNVTOL,
            save.ADJUST,
            save.CNFINE.as_slice(),
            save.RPT,
            spicelib::GFREPI,
            spicelib::GFREPU,
            spicelib::GFREPF,
            MAXVAL,
            NWMAX,
            save.WORK.as_slice_mut(),
            save.BAIL,
            spicelib::GFBAIL,
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(MISSINGVALUE)", OK, ctx)?;
    }

    //
    // Case 8
    //
    //   Event DISTANCE - Test for an error when ADJUST !=0 for
    //   RELATE != 'ABSMAX' or 'ABSMIN
    //
    fstr::assign(&mut save.EVENT, b"DISTANCE");
    save.QNPARS = 3;
    save.STEP = (1.0 * spicelib::SPD());
    save.REFVAL = 0.0;
    save.RPT = false;
    save.BAIL = false;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QCPARS.get_mut(1), b"MOON");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QCPARS.get_mut(3), b"LT+S");

    save.ADJUST = 1.0;

    for I in 1..=(NC - 2) {
        spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
        spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.RELATE, save.CNAMES.get(I));

        fstr::assign(
            &mut save.TITLE,
            &fstr::concat(b"DISTANCE: Non-zero ADJUST ", &save.RELATE),
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::GFEVNT(
            spicelib::GFSTEP,
            spicelib::GFREFN,
            &save.EVENT,
            save.QNPARS,
            save.QPNAMS.as_arg(),
            save.QCPARS.as_arg(),
            save.QDPARS.as_slice(),
            save.QIPARS.as_slice(),
            save.QLPARS.as_slice(),
            &save.RELATE,
            save.REFVAL,
            CNVTOL,
            save.ADJUST,
            save.CNFINE.as_slice(),
            save.RPT,
            spicelib::GFREPI,
            spicelib::GFREPU,
            spicelib::GFREPF,
            MAXVAL,
            NWMAX,
            save.WORK.as_slice_mut(),
            save.BAIL,
            spicelib::GFBAIL,
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(INVALIDVALUE)", OK, ctx)?;
    }

    //
    // Case 9
    //
    //   Event ANGULAR SEPARATION - Test for an error when ADJUST !=0 for
    //   RELATE != 'ABSMAX'
    //
    fstr::assign(&mut save.EVENT, b"ANGULAR SEPARATION");
    save.QNPARS = 8;
    save.STEP = (1.0 * spicelib::SPD());
    save.REFVAL = 0.0;
    save.RPT = false;
    save.BAIL = false;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET1");
    fstr::assign(save.QCPARS.get_mut(1), b"MOON");
    fstr::assign(save.QPNAMS.get_mut(2), b"FRAME1");
    fstr::assign(save.QCPARS.get_mut(2), b"NULL");
    fstr::assign(save.QPNAMS.get_mut(3), b"SHAPE1");
    fstr::assign(save.QCPARS.get_mut(3), b"SPHERE");
    fstr::assign(save.QPNAMS.get_mut(4), b"TARGET2");
    fstr::assign(save.QCPARS.get_mut(4), b"EARTH");
    fstr::assign(save.QPNAMS.get_mut(5), b"FRAME2");
    fstr::assign(save.QCPARS.get_mut(5), b"NULL");
    fstr::assign(save.QPNAMS.get_mut(6), b"SHAPE2");
    fstr::assign(save.QCPARS.get_mut(6), b"SPHERE");
    fstr::assign(save.QPNAMS.get_mut(7), b"OBSERVER");
    fstr::assign(save.QCPARS.get_mut(7), b"SUN");
    fstr::assign(save.QPNAMS.get_mut(8), b"ABCORR");
    fstr::assign(save.QCPARS.get_mut(8), b"NONE");

    save.ADJUST = 1.0;

    for I in 1..=(NC - 2) {
        spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
        spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.RELATE, save.CNAMES.get(I));

        fstr::assign(
            &mut save.TITLE,
            &fstr::concat(b"ANGULAR SEPARATION: Non zero ADJUST ", &save.RELATE),
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::GFEVNT(
            spicelib::GFSTEP,
            spicelib::GFREFN,
            &save.EVENT,
            save.QNPARS,
            save.QPNAMS.as_arg(),
            save.QCPARS.as_arg(),
            save.QDPARS.as_slice(),
            save.QIPARS.as_slice(),
            save.QLPARS.as_slice(),
            &save.RELATE,
            save.REFVAL,
            CNVTOL,
            save.ADJUST,
            save.CNFINE.as_slice(),
            save.RPT,
            spicelib::GFREPI,
            spicelib::GFREPU,
            spicelib::GFREPF,
            MAXVAL,
            NWMAX,
            save.WORK.as_slice_mut(),
            save.BAIL,
            spicelib::GFBAIL,
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(INVALIDVALUE)", OK, ctx)?;
    }

    //
    // Case 10
    //
    //   Event COORDINATE - Test for an error when ADJUST !=0 for
    //   RELATE != 'ABSMAX'
    //
    fstr::assign(&mut save.EVENT, b"COORDINATE");
    save.QNPARS = 10;
    save.STEP = (1.0 * spicelib::SPD());
    save.REFVAL = 0.0;
    save.RPT = false;
    save.BAIL = false;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QCPARS.get_mut(1), b"MOON");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QCPARS.get_mut(4), b"RECTANGULAR");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QCPARS.get_mut(5), b"X");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QCPARS.get_mut(6), b"J2000");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QCPARS.get_mut(9), b" ");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");
    save.QDPARS[1] = 0.0;
    save.QDPARS[2] = 0.0;
    save.QDPARS[3] = 0.0;

    save.ADJUST = 1.0;

    for I in 1..=(NC - 2) {
        spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
        spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.RELATE, save.CNAMES.get(I));

        fstr::assign(
            &mut save.TITLE,
            &fstr::concat(b"COORDINATE: Non zero ADJUST ", &save.RELATE),
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::GFEVNT(
            spicelib::GFSTEP,
            spicelib::GFREFN,
            &save.EVENT,
            save.QNPARS,
            save.QPNAMS.as_arg(),
            save.QCPARS.as_arg(),
            save.QDPARS.as_slice(),
            save.QIPARS.as_slice(),
            save.QLPARS.as_slice(),
            &save.RELATE,
            save.REFVAL,
            CNVTOL,
            save.ADJUST,
            save.CNFINE.as_slice(),
            save.RPT,
            spicelib::GFREPI,
            spicelib::GFREPU,
            spicelib::GFREPF,
            MAXVAL,
            NWMAX,
            save.WORK.as_slice_mut(),
            save.BAIL,
            spicelib::GFBAIL,
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(INVALIDVALUE)", OK, ctx)?;
    }

    //
    // Case n
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::KCLEAR(ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
