//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const QSIZ: i32 = 4;
const QAVSIZ: i32 = 7;
const CK1DTP: i32 = 1;
const CK1RSZ: i32 = 8;
const CK2DTP: i32 = 2;
const CK2RSZ: i32 = 10;
const CK3DTP: i32 = 3;
const CK3RSZ: i32 = 17;
const CK4DTP: i32 = 4;
const CK4PCD: f64 = 128.0;
const CK4MXD: i32 = 18;
const CK4SFT: i32 = 10;
const CK4RSZ: i32 = (((CK4MXD + 1) * QAVSIZ) + CK4SFT);
const CK5DTP: i32 = 5;
const CK5MXD: i32 = 23;
const CK5MET: i32 = 4;
const CK5MXP: i32 = 14;
const CK5RSZ: i32 = (((CK5MXD + 1) * CK5MXP) + CK5MET);
const CK6DTP: i32 = 6;
const CK6MXD: i32 = 23;
const CK6MET: i32 = 4;
const CK6PS3: i32 = 7;
const CK6RSZ: i32 = (((CK6MXD + 1) * (CK6PS3 + 1)) + CK6MET);
const CKMRSZ: i32 = CK5RSZ;
const MAXDEG: i32 = 23;
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const C06TP0: i32 = 0;
const C06TP1: i32 = (C06TP0 + 1);
const C06TP2: i32 = (C06TP1 + 1);
const C06TP3: i32 = (C06TP2 + 1);
const C06NST: i32 = 4;
const C06PS0: i32 = 8;
const C06PS1: i32 = 4;
const C06PS2: i32 = 14;
const C06PS3: i32 = 7;
const C06MXZ: i32 = C06PS2;
const C06MNZ: i32 = C06PS1;
const MAXRSZ: i32 = (4 + ((MAXDEG + 1) * (C06PS3 + 1)));
const TIGHT: f64 = 0.000000000001;
const VTIGHT: f64 = 0.00000000000001;
const LNSIZE: i32 = 240;
const MAXREC: i32 = 10000;
const MXPKSZ: i32 = C06MXZ;
const RECSIZ: i32 = CKMRSZ;
const PBUFSZ: i32 = (MXPKSZ * MAXREC);

struct SaveVars {
    TITLE: Vec<u8>,
    ALPHA: f64,
    ANGLE: f64,
    ANGLE0: f64,
    AV: StackArray<f64, 3>,
    AXIS: StackArray<f64, 3>,
    C: f64,
    CLKBUF: ActualArray<f64>,
    CLKOUT: f64,
    CMAT: StackArray2D<f64, 9>,
    DAV: StackArray<f64, 3>,
    DELTA: f64,
    DQ: StackArray<f64, 4>,
    PACKTS: ActualArray<f64>,
    PREVQ: StackArray<f64, 4>,
    QANGLE: f64,
    QAXIS: StackArray<f64, 3>,
    QMAT: StackArray2D<f64, 9>,
    QQANGL: f64,
    QSTATE: StackArray<f64, 8>,
    QUAT: StackArray<f64, 4>,
    RATE: f64,
    REC2: ActualArray<f64>,
    RECORD: ActualArray<f64>,
    S: f64,
    SCLKDP: f64,
    SPAN: f64,
    TDELTA: f64,
    TOL: f64,
    W: f64,
    XAV: StackArray<f64, 3>,
    XSCLK: f64,
    XCMAT: StackArray2D<f64, 9>,
    XQUAT: StackArray<f64, 4>,
    DEGREE: i32,
    NSAMP: i32,
    PKTSIZ: i32,
    PTR: i32,
    SIZE: i32,
    SUBTYP: i32,
    WINSIZ: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut ALPHA: f64 = 0.0;
        let mut ANGLE: f64 = 0.0;
        let mut ANGLE0: f64 = 0.0;
        let mut AV = StackArray::<f64, 3>::new(1..=3);
        let mut AXIS = StackArray::<f64, 3>::new(1..=3);
        let mut C: f64 = 0.0;
        let mut CLKBUF = ActualArray::<f64>::new(1..=MAXREC);
        let mut CLKOUT: f64 = 0.0;
        let mut CMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut DAV = StackArray::<f64, 3>::new(1..=3);
        let mut DELTA: f64 = 0.0;
        let mut DQ = StackArray::<f64, 4>::new(0..=3);
        let mut PACKTS = ActualArray::<f64>::new(1..=PBUFSZ);
        let mut PREVQ = StackArray::<f64, 4>::new(0..=3);
        let mut QANGLE: f64 = 0.0;
        let mut QAXIS = StackArray::<f64, 3>::new(1..=3);
        let mut QMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut QQANGL: f64 = 0.0;
        let mut QSTATE = StackArray::<f64, 8>::new(1..=8);
        let mut QUAT = StackArray::<f64, 4>::new(0..=3);
        let mut RATE: f64 = 0.0;
        let mut REC2 = ActualArray::<f64>::new(1..=RECSIZ);
        let mut RECORD = ActualArray::<f64>::new(1..=RECSIZ);
        let mut S: f64 = 0.0;
        let mut SCLKDP: f64 = 0.0;
        let mut SPAN: f64 = 0.0;
        let mut TDELTA: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut W: f64 = 0.0;
        let mut XAV = StackArray::<f64, 3>::new(1..=3);
        let mut XSCLK: f64 = 0.0;
        let mut XCMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XQUAT = StackArray::<f64, 4>::new(0..=3);
        let mut DEGREE: i32 = 0;
        let mut NSAMP: i32 = 0;
        let mut PKTSIZ: i32 = 0;
        let mut PTR: i32 = 0;
        let mut SIZE: i32 = 0;
        let mut SUBTYP: i32 = 0;
        let mut WINSIZ: i32 = 0;

        Self {
            TITLE,
            ALPHA,
            ANGLE,
            ANGLE0,
            AV,
            AXIS,
            C,
            CLKBUF,
            CLKOUT,
            CMAT,
            DAV,
            DELTA,
            DQ,
            PACKTS,
            PREVQ,
            QANGLE,
            QAXIS,
            QMAT,
            QQANGL,
            QSTATE,
            QUAT,
            RATE,
            REC2,
            RECORD,
            S,
            SCLKDP,
            SPAN,
            TDELTA,
            TOL,
            W,
            XAV,
            XSCLK,
            XCMAT,
            XQUAT,
            DEGREE,
            NSAMP,
            PKTSIZ,
            PTR,
            SIZE,
            SUBTYP,
            WINSIZ,
        }
    }
}

//$Procedure      F_ZZCKE06 ( Family of tests for ZZCKE06 )
pub fn F_ZZCKE06(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //
    // Save all variables to avoid CSPICE stack overflow
    // problems.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZCKE06", ctx)?;

    //**********************************************************************
    //
    //
    //     Error test cases
    //
    //
    //**********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Invalid subtype", ctx)?;

    spicelib::CLEARD(RECSIZ, save.RECORD.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SUBTYP = -1;
    save.WINSIZ = 11;
    save.SCLKDP = 0.0;
    save.RATE = 1.0;

    save.RECORD[1] = save.SCLKDP;
    save.RECORD[2] = (save.SUBTYP as f64);
    save.RECORD[3] = (save.WINSIZ as f64);
    save.RECORD[4] = save.RATE;

    support::ZZCKE06(
        save.RECORD.as_slice_mut(),
        save.QSTATE.as_slice_mut(),
        &mut save.CLKOUT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Hermite subtype with invalid quaternion sequence", ctx)?;

    save.SCLKDP = 0.0;
    save.TDELTA = 10000.0;
    save.RATE = 0.0001;

    save.SUBTYP = 0;
    save.DEGREE = 11;
    save.WINSIZ = ((save.DEGREE + 1) / 2);
    save.PKTSIZ = C06PS0;

    save.RECORD[1] = save.SCLKDP;
    save.RECORD[2] = (save.SUBTYP as f64);
    save.RECORD[3] = (save.WINSIZ as f64);
    save.RECORD[4] = save.RATE;

    for I in 1..=save.WINSIZ {
        save.SCLKDP = (((I - 1) as f64) * save.TDELTA);
        save.ALPHA = ((spicelib::RPD(ctx) * 100 as f64) / save.TDELTA);
        save.ANGLE = (save.ALPHA * intrinsics::pow((I - 1), 2) as f64);

        spicelib::AXISAR(save.AXIS.as_slice(), -save.ANGLE, save.CMAT.as_slice_mut());

        save.C = f64::cos((save.ANGLE / 2 as f64));
        save.S = f64::sin((save.ANGLE / 2 as f64));

        if (I == 3) {
            save.QUAT[0] = -save.C;
        } else {
            save.QUAT[0] = save.C;
        }

        spicelib::VSCL(save.S, save.AXIS.as_slice(), save.QUAT.subarray_mut(1));

        save.CLKBUF[I] = save.SCLKDP;
        save.PTR = (((I - 1) * save.PKTSIZ) + 1);

        spicelib::MOVED(save.QUAT.as_slice(), 4, save.PACKTS.subarray_mut(save.PTR));

        //
        // Compute the quaternion derivative with respect
        // to TDB; store it after the quaternion.
        //
        // As in the previous case, the angular rate
        // in radians/second is
        //
        save.W =
            (((save.RATE * 2 as f64) * save.ALPHA) * (save.SCLKDP / f64::powi(save.TDELTA, 2)));

        save.DQ[0] = -((0.5 * save.S) * save.W);

        spicelib::VSCL(
            ((0.5 * save.C) * save.W),
            save.AXIS.as_slice(),
            save.DQ.subarray_mut(1),
        );

        spicelib::MOVED(
            save.DQ.as_slice(),
            4,
            save.PACKTS.subarray_mut((save.PTR + 4)),
        );
    }

    spicelib::MOVED(
        save.PACKTS.as_slice(),
        (save.PKTSIZ * save.WINSIZ),
        save.RECORD.subarray_mut(5),
    );

    save.PTR = ((save.PKTSIZ * save.WINSIZ) + 5);

    spicelib::MOVED(
        save.CLKBUF.as_slice(),
        save.WINSIZ,
        save.RECORD.subarray_mut(save.PTR),
    );

    support::ZZCKE06(
        save.RECORD.as_slice_mut(),
        save.QSTATE.as_slice_mut(),
        &mut save.CLKOUT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADQUATSIGN)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Subtype 0 divide-by zero case", ctx)?;

    save.SCLKDP = 0.0;
    save.TDELTA = 10000.0;
    save.RATE = 0.0001;

    save.SUBTYP = 0;
    save.DEGREE = 11;
    save.WINSIZ = ((save.DEGREE + 1) / 2);
    save.PKTSIZ = C06PS0;

    spicelib::CLEARD(RECSIZ, save.RECORD.as_slice_mut());
    spicelib::CLEARD(PBUFSZ, save.PACKTS.as_slice_mut());

    save.RECORD[1] = save.SCLKDP;
    save.RECORD[2] = (save.SUBTYP as f64);
    save.RECORD[3] = (save.WINSIZ as f64);
    save.RECORD[4] = save.RATE;

    spicelib::MOVED(
        save.PACKTS.as_slice(),
        (save.PKTSIZ * save.WINSIZ),
        save.RECORD.subarray_mut(5),
    );

    save.PTR = ((save.PKTSIZ * save.WINSIZ) + 5);

    for I in 1..=save.WINSIZ {
        save.CLKBUF[I] = ((I as f64) * save.TDELTA);
    }

    spicelib::MOVED(
        save.CLKBUF.as_slice(),
        save.WINSIZ,
        save.RECORD.subarray_mut(save.PTR),
    );

    support::ZZCKE06(
        save.RECORD.as_slice_mut(),
        save.QSTATE.as_slice_mut(),
        &mut save.CLKOUT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DIVIDEBYZERO)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Subtype 1 divide-by zero case", ctx)?;

    save.SCLKDP = 0.0;
    save.TDELTA = 10000.0;
    save.RATE = 0.0001;

    save.SUBTYP = 1;
    save.DEGREE = 11;
    save.WINSIZ = (save.DEGREE + 1);
    save.PKTSIZ = C06PS1;

    spicelib::CLEARD(RECSIZ, save.RECORD.as_slice_mut());
    spicelib::CLEARD(PBUFSZ, save.PACKTS.as_slice_mut());

    save.RECORD[1] = save.SCLKDP;
    save.RECORD[2] = (save.SUBTYP as f64);
    save.RECORD[3] = (save.WINSIZ as f64);
    save.RECORD[4] = save.RATE;

    spicelib::MOVED(
        save.PACKTS.as_slice(),
        (save.PKTSIZ * save.WINSIZ),
        save.RECORD.subarray_mut(5),
    );

    save.PTR = ((save.PKTSIZ * save.WINSIZ) + 5);

    for I in 1..=save.WINSIZ {
        save.CLKBUF[I] = ((I as f64) * save.TDELTA);
    }

    spicelib::MOVED(
        save.CLKBUF.as_slice(),
        save.WINSIZ,
        save.RECORD.subarray_mut(save.PTR),
    );

    support::ZZCKE06(
        save.RECORD.as_slice_mut(),
        save.QSTATE.as_slice_mut(),
        &mut save.CLKOUT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DIVIDEBYZERO)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Subtype 2 divide-by zero case", ctx)?;

    save.SCLKDP = 0.0;
    save.TDELTA = 10000.0;
    save.RATE = 0.0001;

    save.SUBTYP = 2;
    save.DEGREE = 11;
    save.WINSIZ = ((save.DEGREE + 1) / 2);
    save.PKTSIZ = C06PS2;

    spicelib::CLEARD(RECSIZ, save.RECORD.as_slice_mut());
    spicelib::CLEARD(PBUFSZ, save.PACKTS.as_slice_mut());

    save.RECORD[1] = save.SCLKDP;
    save.RECORD[2] = (save.SUBTYP as f64);
    save.RECORD[3] = (save.WINSIZ as f64);
    save.RECORD[4] = save.RATE;

    spicelib::MOVED(
        save.PACKTS.as_slice(),
        (save.PKTSIZ * save.WINSIZ),
        save.RECORD.subarray_mut(5),
    );

    save.PTR = ((save.PKTSIZ * save.WINSIZ) + 5);

    for I in 1..=save.WINSIZ {
        save.CLKBUF[I] = ((I as f64) * save.TDELTA);
    }

    spicelib::MOVED(
        save.CLKBUF.as_slice(),
        save.WINSIZ,
        save.RECORD.subarray_mut(save.PTR),
    );

    support::ZZCKE06(
        save.RECORD.as_slice_mut(),
        save.QSTATE.as_slice_mut(),
        &mut save.CLKOUT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DIVIDEBYZERO)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Subtype 3 divide-by zero case", ctx)?;

    save.SCLKDP = 0.0;
    save.TDELTA = 10000.0;
    save.RATE = 0.0001;

    save.SUBTYP = 3;
    save.DEGREE = 11;
    save.WINSIZ = (save.DEGREE + 1);
    save.PKTSIZ = C06PS3;

    spicelib::CLEARD(RECSIZ, save.RECORD.as_slice_mut());
    spicelib::CLEARD(PBUFSZ, save.PACKTS.as_slice_mut());

    save.RECORD[1] = save.SCLKDP;
    save.RECORD[2] = (save.SUBTYP as f64);
    save.RECORD[3] = (save.WINSIZ as f64);
    save.RECORD[4] = save.RATE;

    spicelib::MOVED(
        save.PACKTS.as_slice(),
        (save.PKTSIZ * save.WINSIZ),
        save.RECORD.subarray_mut(5),
    );

    save.PTR = ((save.PKTSIZ * save.WINSIZ) + 5);

    for I in 1..=save.WINSIZ {
        save.CLKBUF[I] = ((I as f64) * save.TDELTA);
    }

    spicelib::MOVED(
        save.CLKBUF.as_slice(),
        save.WINSIZ,
        save.RECORD.subarray_mut(save.PTR),
    );

    support::ZZCKE06(
        save.RECORD.as_slice_mut(),
        save.QSTATE.as_slice_mut(),
        &mut save.CLKOUT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DIVIDEBYZERO)", OK, ctx)?;

    //**********************************************************************
    //
    //
    //     Normal test cases
    //
    //
    //**********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Test using type 6, subtype 1 attitude record.", ctx)?;

    //
    // We'll use a fixed axis and a rotation angle that
    // increases quadratically with time.
    //
    // Our time step will be 1 second (10000 ticks )
    //
    spicelib::VPACK(1.0, 2.0, 3.0, save.AXIS.as_slice_mut());
    spicelib::VHATIP(save.AXIS.as_slice_mut());

    save.SCLKDP = 0.0;
    save.TDELTA = 10000.0;
    save.RATE = 0.0001;

    save.SUBTYP = 1;
    save.DEGREE = 11;
    save.WINSIZ = (save.DEGREE + 1);
    save.PKTSIZ = C06PS1;

    for I in 1..=save.WINSIZ {
        save.SCLKDP = (((I - 1) as f64) * save.TDELTA);
        save.ALPHA = ((spicelib::RPD(ctx) * 100 as f64) / save.TDELTA);
        save.ANGLE = (save.ALPHA * intrinsics::pow((I - 1), 2) as f64);

        spicelib::AXISAR(save.AXIS.as_slice(), -save.ANGLE, save.CMAT.as_slice_mut());

        save.C = f64::cos((save.ANGLE / 2 as f64));
        save.S = f64::sin((save.ANGLE / 2 as f64));

        save.QUAT[0] = save.C;
        spicelib::VSCL(save.S, save.AXIS.as_slice(), save.QUAT.subarray_mut(1));

        save.CLKBUF[I] = save.SCLKDP;
        save.PTR = (((I - 1) * save.PKTSIZ) + 1);

        spicelib::MOVED(save.QUAT.as_slice(), 4, save.PACKTS.subarray_mut(save.PTR));
    }

    save.NSAMP = 300;
    save.SPAN = (save.CLKBUF[save.WINSIZ] - save.CLKBUF[1]);
    save.DELTA = (save.SPAN / save.NSAMP as f64);

    for I in 1..=save.NSAMP {
        //
        // Set the sample time.
        //
        save.SCLKDP = (((I - 1) as f64) * save.DELTA);

        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(&mut save.TITLE, b"Subtype 1 test for I = #; SCLKDP #");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.SCLKDP,
            14,
            &mut save.TITLE,
            ctx,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Create a type 6 pointing record using the SCLK, subtype, and
        // packet count. We have to re-create the record for each call
        // because the CK evaluators use it destructively.
        //
        save.RECORD[1] = save.SCLKDP;
        save.RECORD[2] = (save.SUBTYP as f64);
        save.RECORD[3] = (save.WINSIZ as f64);
        save.RECORD[4] = save.RATE;

        spicelib::MOVED(
            save.PACKTS.as_slice(),
            (save.PKTSIZ * save.WINSIZ),
            save.RECORD.subarray_mut(5),
        );

        save.PTR = ((save.PKTSIZ * save.WINSIZ) + 5);

        spicelib::MOVED(
            save.CLKBUF.as_slice(),
            save.WINSIZ,
            save.RECORD.subarray_mut(save.PTR),
        );

        save.SIZE = (4 + ((save.PKTSIZ + 1) * save.WINSIZ));

        spicelib::MOVED(save.RECORD.as_slice(), save.SIZE, save.REC2.as_slice_mut());

        //
        // Compute the angular velocity at the sample time.
        // Units are radians/second.
        //
        //    We have
        //
        //       ANGLE     =   ALPHA * (I-1)**2
        //
        //       SCLKDP    =   (I-1)*TDELTA
        //
        //    so
        //
        //       ANGLE     =   ALPHA * (SCLKDP/TDELTA)**2
        //
        //    and
        //
        //       d(ANGLE)
        //       --------- =   2 * ALPHA * SCLKDP / TDELTA**2
        //       d(SCLKDP)
        //

        save.W =
            (((save.RATE * 2 as f64) * save.ALPHA) * (save.SCLKDP / f64::powi(save.TDELTA, 2)));

        spicelib::VSCL(save.W, save.AXIS.as_slice(), save.AV.as_slice_mut());

        //
        // Compute the attitude quaternion and its derivative
        // at SCLKDP.
        //
        support::ZZCKE06(
            save.RECORD.as_slice_mut(),
            save.QSTATE.as_slice_mut(),
            &mut save.CLKOUT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Create C-matrix and AV from the quaternion state.
        //
        spicelib::Q2M(save.QSTATE.as_slice(), save.CMAT.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::QDQ2AV(
            save.QSTATE.as_slice(),
            save.QSTATE.subarray(5),
            save.AV.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::MOVED(save.QSTATE.as_slice(), 4, save.QUAT.as_slice_mut());

        //
        // Compute the expected C-matrix and angular velocity.
        //
        spicelib::CKE06(
            true,
            save.REC2.as_slice_mut(),
            save.XCMAT.as_slice_mut(),
            save.XAV.as_slice_mut(),
            &mut save.XSCLK,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check C-matrix derived from ZZCKE06 output state.
        //
        save.TOL = TIGHT;

        testutil::CHCKAD(
            b"CMAT",
            save.CMAT.as_slice(),
            b"~~/",
            save.XCMAT.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check C-matrix quotient rotation angle.
        //
        spicelib::MTXM(
            save.CMAT.as_slice(),
            save.XCMAT.as_slice(),
            save.QMAT.as_slice_mut(),
        );

        spicelib::RAXISA(
            save.QMAT.as_slice(),
            save.QAXIS.as_slice_mut(),
            &mut save.QANGLE,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
        testutil::CHCKSD(b"quotient angle", save.QANGLE, b"~", 0.0, save.TOL, OK, ctx)?;

        //
        // Check angular velocity derived from ZZCKE06 output state.
        //
        save.TOL = TIGHT;
        testutil::CHCKAD(
            b"AV",
            save.AV.as_slice(),
            b"~~/",
            save.XAV.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check output SCLK from ZZCKE06.
        //
        testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"~", save.XSCLK, save.TOL, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Test using type 6, subtype 0 attitude record.", ctx)?;

    //
    // We use the packet set from the previous test.
    //
    save.SUBTYP = 0;

    save.DEGREE = 11;
    save.WINSIZ = ((save.DEGREE + 1) / 2);
    save.PKTSIZ = C06PS0;

    for I in 1..=save.WINSIZ {
        save.SCLKDP = (((I - 1) as f64) * save.TDELTA);
        save.ALPHA = ((spicelib::RPD(ctx) * 100 as f64) / save.TDELTA);
        save.ANGLE = (save.ALPHA * intrinsics::pow((I - 1), 2) as f64);

        spicelib::AXISAR(save.AXIS.as_slice(), -save.ANGLE, save.CMAT.as_slice_mut());

        save.C = f64::cos((save.ANGLE / 2 as f64));
        save.S = f64::sin((save.ANGLE / 2 as f64));

        save.QUAT[0] = save.C;
        spicelib::VSCL(save.S, save.AXIS.as_slice(), save.QUAT.subarray_mut(1));

        save.CLKBUF[I] = save.SCLKDP;
        save.PTR = (((I - 1) * save.PKTSIZ) + 1);

        spicelib::MOVED(save.QUAT.as_slice(), 4, save.PACKTS.subarray_mut(save.PTR));

        //
        // Compute the quaternion derivative with respect
        // to TDB; store it after the quaternion.
        //
        // As in the previous case, the angular rate
        // in radians/second is
        //
        save.W =
            (((save.RATE * 2 as f64) * save.ALPHA) * (save.SCLKDP / f64::powi(save.TDELTA, 2)));

        save.DQ[0] = -((0.5 * save.S) * save.W);

        spicelib::VSCL(
            ((0.5 * save.C) * save.W),
            save.AXIS.as_slice(),
            save.DQ.subarray_mut(1),
        );

        spicelib::MOVED(
            save.DQ.as_slice(),
            4,
            save.PACKTS.subarray_mut((save.PTR + 4)),
        );
    }

    save.NSAMP = 300;
    save.SPAN = (save.CLKBUF[save.WINSIZ] - save.CLKBUF[1]);
    save.DELTA = (save.SPAN / save.NSAMP as f64);

    for I in 1..=save.NSAMP {
        //
        // Set the sample time.
        //
        save.SCLKDP = (((I - 1) as f64) * save.DELTA);

        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(&mut save.TITLE, b"Subtype 0 test for I = #; SCLKDP #");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.SCLKDP,
            14,
            &mut save.TITLE,
            ctx,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Create a type 6 pointing record using the SCLK, subtype, and
        // packet count. We have to re-create the record for each call
        // because the CK evaluators use it destructively.
        //
        save.RECORD[1] = save.SCLKDP;
        save.RECORD[2] = (save.SUBTYP as f64);
        save.RECORD[3] = (save.WINSIZ as f64);
        save.RECORD[4] = save.RATE;

        spicelib::MOVED(
            save.PACKTS.as_slice(),
            (save.PKTSIZ * save.WINSIZ),
            save.RECORD.subarray_mut(5),
        );

        save.PTR = ((save.PKTSIZ * save.WINSIZ) + 5);

        spicelib::MOVED(
            save.CLKBUF.as_slice(),
            save.WINSIZ,
            save.RECORD.subarray_mut(save.PTR),
        );

        save.SIZE = (4 + ((save.PKTSIZ + 1) * save.WINSIZ));

        spicelib::MOVED(save.RECORD.as_slice(), save.SIZE, save.REC2.as_slice_mut());

        //
        // Compute the angular velocity at the sample time.
        // Units are radians/second.
        //
        //    We have
        //
        //       ANGLE     =   ALPHA * (I-1)**2
        //
        //       SCLKDP    =   (I-1)*TDELTA
        //
        //    so
        //
        //       ANGLE     =   ALPHA * (SCLKDP/TDELTA)**2
        //
        //    and
        //
        //       d(ANGLE)
        //       --------- =   2 * ALPHA * SCLKDP / TDELTA**2
        //       d(SCLKDP)
        //

        save.W =
            (((save.RATE * 2 as f64) * save.ALPHA) * (save.SCLKDP / f64::powi(save.TDELTA, 2)));

        spicelib::VSCL(save.W, save.AXIS.as_slice(), save.AV.as_slice_mut());

        //
        // Compute the attitude quaternion and its derivative
        // at SCLKDP.
        //
        support::ZZCKE06(
            save.RECORD.as_slice_mut(),
            save.QSTATE.as_slice_mut(),
            &mut save.CLKOUT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Create C-matrix and AV from the quaternion state.
        //
        spicelib::Q2M(save.QSTATE.as_slice(), save.CMAT.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::QDQ2AV(
            save.QSTATE.as_slice(),
            save.QSTATE.subarray(5),
            save.AV.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::MOVED(save.QSTATE.as_slice(), 4, save.QUAT.as_slice_mut());

        //
        // Compute the expected C-matrix and angular velocity.
        //
        spicelib::CKE06(
            true,
            save.REC2.as_slice_mut(),
            save.XCMAT.as_slice_mut(),
            save.XAV.as_slice_mut(),
            &mut save.XSCLK,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check C-matrix derived from ZZCKE06 output state.
        //
        save.TOL = TIGHT;

        testutil::CHCKAD(
            b"CMAT",
            save.CMAT.as_slice(),
            b"~~/",
            save.XCMAT.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check C-matrix quotient rotation angle.
        //
        spicelib::MTXM(
            save.CMAT.as_slice(),
            save.XCMAT.as_slice(),
            save.QMAT.as_slice_mut(),
        );

        spicelib::RAXISA(
            save.QMAT.as_slice(),
            save.QAXIS.as_slice_mut(),
            &mut save.QANGLE,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
        testutil::CHCKSD(b"quotient angle", save.QANGLE, b"~", 0.0, save.TOL, OK, ctx)?;

        //
        // Check angular velocity derived from ZZCKE06 output state.
        //
        save.TOL = TIGHT;
        testutil::CHCKAD(
            b"AV",
            save.AV.as_slice(),
            b"~~/",
            save.XAV.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check output SCLK from ZZCKE06.
        //
        testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"~", save.XSCLK, save.TOL, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Test using type 6, subtype 2 attitude record.", ctx)?;

    //
    // We use the packet set from the previous test.
    //
    save.SUBTYP = 2;

    save.DEGREE = 11;
    save.WINSIZ = ((save.DEGREE + 1) / 2);
    save.PKTSIZ = C06PS2;

    for I in 1..=save.WINSIZ {
        save.SCLKDP = (((I - 1) as f64) * save.TDELTA);
        save.ALPHA = ((spicelib::RPD(ctx) * 100 as f64) / save.TDELTA);
        save.ANGLE = (save.ALPHA * intrinsics::pow((I - 1), 2) as f64);

        spicelib::AXISAR(save.AXIS.as_slice(), -save.ANGLE, save.CMAT.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.C = f64::cos((save.ANGLE / 2 as f64));
        save.S = f64::sin((save.ANGLE / 2 as f64));

        save.QUAT[0] = save.C;
        spicelib::VSCL(save.S, save.AXIS.as_slice(), save.QUAT.subarray_mut(1));

        save.CLKBUF[I] = save.SCLKDP;
        save.PTR = (((I - 1) * save.PKTSIZ) + 1);

        spicelib::MOVED(save.QUAT.as_slice(), 4, save.PACKTS.subarray_mut(save.PTR));

        //
        // Compute the quaternion derivative with respect
        // to TDB; store it after the quaternion.
        //
        // As in the previous case, the angular rate
        // in radians/second is
        //
        save.W =
            (((save.RATE * 2 as f64) * save.ALPHA) * (save.SCLKDP / f64::powi(save.TDELTA, 2)));

        save.DQ[0] = -((0.5 * save.S) * save.W);

        spicelib::VSCL(
            ((0.5 * save.C) * save.W),
            save.AXIS.as_slice(),
            save.DQ.subarray_mut(1),
        );

        spicelib::MOVED(
            save.DQ.as_slice(),
            4,
            save.PACKTS.subarray_mut((save.PTR + 4)),
        );

        //
        // Next up is angular velocity.
        //
        spicelib::QDQ2AV(
            save.QUAT.as_slice(),
            save.DQ.as_slice(),
            save.AV.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VEQU(
            save.AV.as_slice(),
            save.PACKTS.subarray_mut((save.PTR + C06PS0)),
        );

        //
        // We need angular acceleration. The angular velocity
        // direction is fixed; we just need the acceleration
        // of the rotation angle. We already have
        //
        //       d(ANGLE)
        //       --------- =   2 * ALPHA * SCLKDP / TDELTA**2
        //       d(SCLKDP)
        //
        // so
        //
        //
        //         d(ANGLE)
        //       ------------ =   2 * ALPHA / TDELTA**2
        //       d**2(SCLKDP)
        //
        //
        // and
        //
        //       d(ANGLE)
        //       ---------    =  ALPHA / (RATE*TDELTA)**2
        //       d**2(TDB)
        //
        //
        spicelib::VSCL(
            (save.ALPHA / f64::powi((save.RATE * save.TDELTA), 2)),
            save.AXIS.as_slice(),
            save.DAV.as_slice_mut(),
        );

        spicelib::VEQU(
            save.DAV.as_slice(),
            save.PACKTS.subarray_mut((save.PTR + 11)),
        );
    }

    save.NSAMP = 300;
    save.SPAN = (save.CLKBUF[save.WINSIZ] - save.CLKBUF[1]);
    save.DELTA = (save.SPAN / save.NSAMP as f64);

    for I in 1..=save.NSAMP {
        //
        // Set the sample time.
        //
        save.SCLKDP = (((I - 1) as f64) * save.DELTA);

        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(&mut save.TITLE, b"Subtype 2 test for I = #; SCLKDP #");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.SCLKDP,
            14,
            &mut save.TITLE,
            ctx,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Create a type 6 pointing record using the SCLK, subtype, and
        // packet count. We have to re-create the record for each call
        // because the CK evaluators use it destructively.
        //
        save.RECORD[1] = save.SCLKDP;
        save.RECORD[2] = (save.SUBTYP as f64);
        save.RECORD[3] = (save.WINSIZ as f64);
        save.RECORD[4] = save.RATE;

        spicelib::MOVED(
            save.PACKTS.as_slice(),
            (save.PKTSIZ * save.WINSIZ),
            save.RECORD.subarray_mut(5),
        );

        save.PTR = ((save.PKTSIZ * save.WINSIZ) + 5);

        spicelib::MOVED(
            save.CLKBUF.as_slice(),
            save.WINSIZ,
            save.RECORD.subarray_mut(save.PTR),
        );

        save.SIZE = (4 + ((save.PKTSIZ + 1) * save.WINSIZ));

        spicelib::MOVED(save.RECORD.as_slice(), save.SIZE, save.REC2.as_slice_mut());

        //
        // Compute the angular velocity at the sample time.
        // Units are radians/second.
        //
        //    We have
        //
        //       ANGLE     =   ALPHA * (I-1)**2
        //
        //       SCLKDP    =   (I-1)*TDELTA
        //
        //    so
        //
        //       ANGLE     =   ALPHA * (SCLKDP/TDELTA)**2
        //
        //    and
        //
        //       d(ANGLE)
        //       --------- =   2 * ALPHA * SCLKDP / TDELTA**2
        //       d(SCLKDP)
        //

        save.W =
            (((save.RATE * 2 as f64) * save.ALPHA) * (save.SCLKDP / f64::powi(save.TDELTA, 2)));

        spicelib::VSCL(save.W, save.AXIS.as_slice(), save.AV.as_slice_mut());

        //
        // Compute the attitude quaternion and its derivative
        // at SCLKDP.
        //
        support::ZZCKE06(
            save.RECORD.as_slice_mut(),
            save.QSTATE.as_slice_mut(),
            &mut save.CLKOUT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Create C-matrix and AV from the quaternion state.
        //
        spicelib::Q2M(save.QSTATE.as_slice(), save.CMAT.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::QDQ2AV(
            save.QSTATE.as_slice(),
            save.QSTATE.subarray(5),
            save.AV.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::MOVED(save.QSTATE.as_slice(), 4, save.QUAT.as_slice_mut());

        //
        // Compute the expected C-matrix and angular velocity.
        //
        spicelib::CKE06(
            true,
            save.REC2.as_slice_mut(),
            save.XCMAT.as_slice_mut(),
            save.XAV.as_slice_mut(),
            &mut save.XSCLK,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check C-matrix derived from ZZCKE06 output state.
        //
        save.TOL = VTIGHT;

        testutil::CHCKAD(
            b"CMAT",
            save.CMAT.as_slice(),
            b"~~/",
            save.XCMAT.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check C-matrix quotient rotation angle.
        //
        spicelib::MTXM(
            save.CMAT.as_slice(),
            save.XCMAT.as_slice(),
            save.QMAT.as_slice_mut(),
        );

        spicelib::RAXISA(
            save.QMAT.as_slice(),
            save.QAXIS.as_slice_mut(),
            &mut save.QANGLE,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
        testutil::CHCKSD(b"quotient angle", save.QANGLE, b"~", 0.0, save.TOL, OK, ctx)?;

        //
        // Check angular velocity derived from ZZCKE06 output state.
        //
        save.TOL = VTIGHT;
        testutil::CHCKAD(
            b"AV",
            save.AV.as_slice(),
            b"~~/",
            save.XAV.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check output SCLK from ZZCKE06.
        //
        testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"~", save.XSCLK, save.TOL, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Test using type 6, subtype 3 attitude record.", ctx)?;

    //
    // We use the packet set from the previous test.
    //
    save.SUBTYP = 3;

    save.DEGREE = 11;
    save.WINSIZ = (save.DEGREE + 1);
    save.PKTSIZ = C06PS3;

    for I in 1..=save.WINSIZ {
        save.SCLKDP = (((I - 1) as f64) * save.TDELTA);
        save.ALPHA = ((spicelib::RPD(ctx) * 100 as f64) / save.TDELTA);
        save.ANGLE = (save.ALPHA * intrinsics::pow((I - 1), 2) as f64);

        spicelib::AXISAR(save.AXIS.as_slice(), -save.ANGLE, save.CMAT.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.C = f64::cos((save.ANGLE / 2 as f64));
        save.S = f64::sin((save.ANGLE / 2 as f64));

        save.QUAT[0] = save.C;
        spicelib::VSCL(save.S, save.AXIS.as_slice(), save.QUAT.subarray_mut(1));

        save.CLKBUF[I] = save.SCLKDP;
        save.PTR = (((I - 1) * save.PKTSIZ) + 1);

        spicelib::MOVED(save.QUAT.as_slice(), 4, save.PACKTS.subarray_mut(save.PTR));

        //
        // Compute the quaternion derivative with respect
        // to TDB; store it after the quaternion.
        //
        // As in the previous case, the angular rate
        // in radians/second is
        //
        save.W =
            (((save.RATE * 2 as f64) * save.ALPHA) * (save.SCLKDP / f64::powi(save.TDELTA, 2)));

        save.DQ[0] = -((0.5 * save.S) * save.W);

        spicelib::VSCL(
            ((0.5 * save.C) * save.W),
            save.AXIS.as_slice(),
            save.DQ.subarray_mut(1),
        );

        spicelib::MOVED(
            save.DQ.as_slice(),
            4,
            save.PACKTS.subarray_mut((save.PTR + 4)),
        );

        //
        // Next up is angular velocity.
        //
        spicelib::QDQ2AV(
            save.QUAT.as_slice(),
            save.DQ.as_slice(),
            save.AV.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VEQU(save.AV.as_slice(), save.PACKTS.subarray_mut((save.PTR + 8)));
    }

    save.NSAMP = 300;
    save.SPAN = (save.CLKBUF[save.WINSIZ] - save.CLKBUF[1]);
    save.DELTA = (save.SPAN / save.NSAMP as f64);

    for I in 1..=save.NSAMP {
        //
        // Set the sample time.
        //
        save.SCLKDP = (((I - 1) as f64) * save.DELTA);

        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(&mut save.TITLE, b"Subtype 3 test for I = #; SCLKDP #");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.SCLKDP,
            14,
            &mut save.TITLE,
            ctx,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Create a type 6 pointing record using the SCLK, subtype, and
        // packet count. We have to re-create the record for each call
        // because the CK evaluators use it destructively.
        //
        save.RECORD[1] = save.SCLKDP;
        save.RECORD[2] = (save.SUBTYP as f64);
        save.RECORD[3] = (save.WINSIZ as f64);
        save.RECORD[4] = save.RATE;

        spicelib::MOVED(
            save.PACKTS.as_slice(),
            (save.PKTSIZ * save.WINSIZ),
            save.RECORD.subarray_mut(5),
        );

        save.PTR = ((save.PKTSIZ * save.WINSIZ) + 5);

        spicelib::MOVED(
            save.CLKBUF.as_slice(),
            save.WINSIZ,
            save.RECORD.subarray_mut(save.PTR),
        );

        save.SIZE = (4 + ((save.PKTSIZ + 1) * save.WINSIZ));

        spicelib::MOVED(save.RECORD.as_slice(), save.SIZE, save.REC2.as_slice_mut());

        //
        // Compute the angular velocity at the sample time.
        // Units are radians/second.
        //
        //    We have
        //
        //       ANGLE     =   ALPHA * (I-1)**2
        //
        //       SCLKDP    =   (I-1)*TDELTA
        //
        //    so
        //
        //       ANGLE     =   ALPHA * (SCLKDP/TDELTA)**2
        //
        //    and
        //
        //       d(ANGLE)
        //       --------- =   2 * ALPHA * SCLKDP / TDELTA**2
        //       d(SCLKDP)
        //

        save.W =
            (((save.RATE * 2 as f64) * save.ALPHA) * (save.SCLKDP / f64::powi(save.TDELTA, 2)));

        spicelib::VSCL(save.W, save.AXIS.as_slice(), save.AV.as_slice_mut());

        //
        // Compute the attitude quaternion and its derivative
        // at SCLKDP.
        //
        support::ZZCKE06(
            save.RECORD.as_slice_mut(),
            save.QSTATE.as_slice_mut(),
            &mut save.CLKOUT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Create C-matrix and AV from the quaternion state.
        //
        spicelib::Q2M(save.QSTATE.as_slice(), save.CMAT.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::QDQ2AV(
            save.QSTATE.as_slice(),
            save.QSTATE.subarray(5),
            save.AV.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::MOVED(save.QSTATE.as_slice(), 4, save.QUAT.as_slice_mut());

        //
        // Compute the expected C-matrix and angular velocity.
        //
        spicelib::CKE06(
            true,
            save.REC2.as_slice_mut(),
            save.XCMAT.as_slice_mut(),
            save.XAV.as_slice_mut(),
            &mut save.XSCLK,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check C-matrix derived from ZZCKE06 output state.
        //
        save.TOL = VTIGHT;

        testutil::CHCKAD(
            b"CMAT",
            save.CMAT.as_slice(),
            b"~~/",
            save.XCMAT.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check C-matrix quotient rotation angle.
        //
        spicelib::MTXM(
            save.CMAT.as_slice(),
            save.XCMAT.as_slice(),
            save.QMAT.as_slice_mut(),
        );

        spicelib::RAXISA(
            save.QMAT.as_slice(),
            save.QAXIS.as_slice_mut(),
            &mut save.QANGLE,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
        testutil::CHCKSD(b"quotient angle", save.QANGLE, b"~", 0.0, save.TOL, OK, ctx)?;

        //
        // Check angular velocity derived from ZZCKE06 output state.
        //
        save.TOL = VTIGHT;
        testutil::CHCKAD(
            b"AV",
            save.AV.as_slice(),
            b"~~/",
            save.XAV.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check output SCLK from ZZCKE06.
        //
        testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"~", save.XSCLK, save.TOL, OK, ctx)?;
    }

    //**********************************************************************
    //
    //
    //     Continuity test cases
    //
    //
    //**********************************************************************

    //
    // M2Q output is discontinuous at the rotation angle pi. The output
    // of ZZCKE06 should be continuous.
    //
    // We'll repeat the previous set of tests using sequence of rotation
    // angles that start out negative and cross over the boundary.
    //
    // The total difference between the initial and final angle is about
    // 1.21 degrees. We'll start with a rotation angle of 180-0.6
    // degrees.
    //

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Continuity test using type 6, subtype 1 attitude record.",
        ctx,
    )?;

    spicelib::CLEARD(4, save.PREVQ.as_slice_mut());
    //
    // We'll use a fixed axis and a rotation angle that
    // increases quadratically with time.
    //
    // Our time step will be 1 second (10000 ticks )
    //
    spicelib::VPACK(1.0, 2.0, 3.0, save.AXIS.as_slice_mut());
    spicelib::VHATIP(save.AXIS.as_slice_mut());

    save.SCLKDP = 0.0;
    save.TDELTA = 10000.0;
    save.RATE = 0.0001;

    save.SUBTYP = 1;
    save.DEGREE = 11;
    save.WINSIZ = (save.DEGREE + 1);
    save.PKTSIZ = 4;

    save.ANGLE0 = ((180.0 - 0.6) * spicelib::RPD(ctx));

    for I in 1..=save.WINSIZ {
        save.SCLKDP = (((I - 1) as f64) * save.TDELTA);
        save.ALPHA = ((spicelib::RPD(ctx) * 100 as f64) / save.TDELTA);
        save.ANGLE = ((save.ALPHA * intrinsics::pow((I - 1), 2) as f64) + save.ANGLE0);

        spicelib::AXISAR(save.AXIS.as_slice(), -save.ANGLE, save.CMAT.as_slice_mut());

        save.C = f64::cos((save.ANGLE / 2 as f64));
        save.S = f64::sin((save.ANGLE / 2 as f64));

        save.QUAT[0] = save.C;
        spicelib::VSCL(save.S, save.AXIS.as_slice(), save.QUAT.subarray_mut(1));

        save.CLKBUF[I] = save.SCLKDP;
        save.PTR = (((I - 1) * save.PKTSIZ) + 1);

        spicelib::MOVED(save.QUAT.as_slice(), 4, save.PACKTS.subarray_mut(save.PTR));
    }

    save.NSAMP = 300;
    save.SPAN = (save.CLKBUF[save.WINSIZ] - save.CLKBUF[1]);
    save.DELTA = (save.SPAN / save.NSAMP as f64);

    for I in 1..=save.NSAMP {
        //
        // Set the sample time.
        //
        save.SCLKDP = (((I - 1) as f64) * save.DELTA);

        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(
            &mut save.TITLE,
            b"Subtype 1 continuity test for I = #; SCLKDP #",
        );
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.SCLKDP,
            14,
            &mut save.TITLE,
            ctx,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Create a type 6 pointing record using the SCLK, subtype, and
        // packet count. We have to re-create the record for each call
        // because the CK evaluators use it destructively.
        //
        save.RECORD[1] = save.SCLKDP;
        save.RECORD[2] = (save.SUBTYP as f64);
        save.RECORD[3] = (save.WINSIZ as f64);
        save.RECORD[4] = save.RATE;

        spicelib::MOVED(
            save.PACKTS.as_slice(),
            (save.PKTSIZ * save.WINSIZ),
            save.RECORD.subarray_mut(5),
        );

        save.PTR = ((save.PKTSIZ * save.WINSIZ) + 5);

        spicelib::MOVED(
            save.CLKBUF.as_slice(),
            save.WINSIZ,
            save.RECORD.subarray_mut(save.PTR),
        );

        save.SIZE = (4 + ((save.PKTSIZ + 1) * save.WINSIZ));

        spicelib::MOVED(save.RECORD.as_slice(), save.SIZE, save.REC2.as_slice_mut());

        //
        // Compute the angular velocity at the sample time.
        // Units are radians/second.
        //
        //    We have
        //
        //       ANGLE     =   ALPHA * (I-1)**2
        //
        //       SCLKDP    =   (I-1)*TDELTA
        //
        //    so
        //
        //       ANGLE     =   ALPHA * (SCLKDP/TDELTA)**2
        //
        //    and
        //
        //       d(ANGLE)
        //       --------- =   2 * ALPHA * SCLKDP / TDELTA**2
        //       d(SCLKDP)
        //

        save.W =
            (((save.RATE * 2 as f64) * save.ALPHA) * (save.SCLKDP / f64::powi(save.TDELTA, 2)));

        spicelib::VSCL(save.W, save.AXIS.as_slice(), save.AV.as_slice_mut());

        //
        // Compute the attitude quaternion and its derivative
        // at SCLKDP.
        //
        support::ZZCKE06(
            save.RECORD.as_slice_mut(),
            save.QSTATE.as_slice_mut(),
            &mut save.CLKOUT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Create C-matrix and AV from the quaternion state.
        //
        spicelib::Q2M(save.QSTATE.as_slice(), save.CMAT.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::QDQ2AV(
            save.QSTATE.as_slice(),
            save.QSTATE.subarray(5),
            save.AV.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::MOVED(save.QSTATE.as_slice(), 4, save.QUAT.as_slice_mut());

        //
        // Compute the expected C-matrix and angular velocity.
        //
        spicelib::CKE06(
            true,
            save.REC2.as_slice_mut(),
            save.XCMAT.as_slice_mut(),
            save.XAV.as_slice_mut(),
            &mut save.XSCLK,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check C-matrix derived from ZZCKE06 output state.
        //
        save.TOL = TIGHT;

        testutil::CHCKAD(
            b"CMAT",
            save.CMAT.as_slice(),
            b"~~/",
            save.XCMAT.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        spicelib::M2Q(save.XCMAT.as_slice(), save.XQUAT.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check C-matrix quotient rotation angle.
        //
        spicelib::MTXM(
            save.CMAT.as_slice(),
            save.XCMAT.as_slice(),
            save.QMAT.as_slice_mut(),
        );

        spicelib::RAXISA(
            save.QMAT.as_slice(),
            save.QAXIS.as_slice_mut(),
            &mut save.QANGLE,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
        testutil::CHCKSD(b"quotient angle", save.QANGLE, b"~", 0.0, save.TOL, OK, ctx)?;

        //
        // Check angular velocity derived from ZZCKE06 output state.
        //
        save.TOL = TIGHT;
        testutil::CHCKAD(
            b"AV",
            save.AV.as_slice(),
            b"~~/",
            save.XAV.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check output SCLK from ZZCKE06.
        //
        testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"~", save.XSCLK, save.TOL, OK, ctx)?;

        //
        // Check quaternion "continuity." We expect a rotation angle
        // of less than two degrees between the quaternion and its
        // predecessor. Note that the dot product of two quaternions
        // Q1, Q2 is the real part of their quotient (Q1_conj * Q2).
        //
        if (I > 1) {
            save.QQANGL = (f64::acos(spicelib::BRCKTD(
                spicelib::VDOTG(save.QUAT.as_slice(), save.PREVQ.as_slice(), 4),
                -1.0,
                1.0,
            )) * spicelib::DPR(ctx));

            testutil::CHCKSD(b"QQANGL", save.QQANGL, b"<", 2.0, 0.0, OK, ctx)?;
        }

        //
        // Save the quaternion for the continuity check on the
        // next pass.
        //
        spicelib::MOVED(save.QUAT.as_slice(), 4, save.PREVQ.as_slice_mut());
    }

    //
    //---- Case -------------------------------------------------------------
    //
    //
    testutil::TCASE(
        b"Continuity test using type 6, subtype 0 attitude record.",
        ctx,
    )?;

    //
    // We use the packet set from the previous test.
    //
    save.SUBTYP = 0;

    save.DEGREE = 11;
    save.WINSIZ = ((save.DEGREE + 1) / 2);
    save.PKTSIZ = C06PS0;

    save.ANGLE0 = ((180.0 - 0.6) * spicelib::RPD(ctx));

    for I in 1..=save.WINSIZ {
        save.SCLKDP = (((I - 1) as f64) * save.TDELTA);
        save.ALPHA = ((spicelib::RPD(ctx) * 100 as f64) / save.TDELTA);
        save.ANGLE = ((save.ALPHA * intrinsics::pow((I - 1), 2) as f64) + save.ANGLE0);

        spicelib::AXISAR(save.AXIS.as_slice(), -save.ANGLE, save.CMAT.as_slice_mut());

        save.C = f64::cos((save.ANGLE / 2 as f64));
        save.S = f64::sin((save.ANGLE / 2 as f64));

        save.QUAT[0] = save.C;
        spicelib::VSCL(save.S, save.AXIS.as_slice(), save.QUAT.subarray_mut(1));

        save.CLKBUF[I] = save.SCLKDP;
        save.PTR = (((I - 1) * save.PKTSIZ) + 1);

        spicelib::MOVED(save.QUAT.as_slice(), 4, save.PACKTS.subarray_mut(save.PTR));

        //
        // Compute the quaternion derivative with respect
        // to TDB; store it after the quaternion.
        //
        // As in the previous case, the angular rate
        // in radians/second is
        //
        save.W =
            (((save.RATE * 2 as f64) * save.ALPHA) * (save.SCLKDP / f64::powi(save.TDELTA, 2)));

        save.DQ[0] = -((0.5 * save.S) * save.W);

        spicelib::VSCL(
            ((0.5 * save.C) * save.W),
            save.AXIS.as_slice(),
            save.DQ.subarray_mut(1),
        );

        spicelib::MOVED(
            save.DQ.as_slice(),
            4,
            save.PACKTS.subarray_mut((save.PTR + 4)),
        );
    }

    save.NSAMP = 300;
    save.SPAN = (save.CLKBUF[save.WINSIZ] - save.CLKBUF[1]);
    save.DELTA = (save.SPAN / save.NSAMP as f64);

    for I in 1..=save.NSAMP {
        //
        // Set the sample time.
        //
        save.SCLKDP = (((I - 1) as f64) * save.DELTA);

        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(
            &mut save.TITLE,
            b"Subtype 0 continuity test for I = #; SCLKDP #",
        );
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.SCLKDP,
            14,
            &mut save.TITLE,
            ctx,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Create a type 6 pointing record using the SCLK, subtype, and
        // packet count. We have to re-create the record for each call
        // because the CK evaluators use it destructively.
        //
        save.RECORD[1] = save.SCLKDP;
        save.RECORD[2] = (save.SUBTYP as f64);
        save.RECORD[3] = (save.WINSIZ as f64);
        save.RECORD[4] = save.RATE;

        spicelib::MOVED(
            save.PACKTS.as_slice(),
            (save.PKTSIZ * save.WINSIZ),
            save.RECORD.subarray_mut(5),
        );

        save.PTR = ((save.PKTSIZ * save.WINSIZ) + 5);

        spicelib::MOVED(
            save.CLKBUF.as_slice(),
            save.WINSIZ,
            save.RECORD.subarray_mut(save.PTR),
        );

        save.SIZE = (4 + ((save.PKTSIZ + 1) * save.WINSIZ));

        spicelib::MOVED(save.RECORD.as_slice(), save.SIZE, save.REC2.as_slice_mut());

        //
        // Compute the angular velocity at the sample time.
        // Units are radians/second.
        //
        //    We have
        //
        //       ANGLE     =   ALPHA * (I-1)**2
        //
        //       SCLKDP    =   (I-1)*TDELTA
        //
        //    so
        //
        //       ANGLE     =   ALPHA * (SCLKDP/TDELTA)**2
        //
        //    and
        //
        //       d(ANGLE)
        //       --------- =   2 * ALPHA * SCLKDP / TDELTA**2
        //       d(SCLKDP)
        //

        save.W =
            (((save.RATE * 2 as f64) * save.ALPHA) * (save.SCLKDP / f64::powi(save.TDELTA, 2)));

        spicelib::VSCL(save.W, save.AXIS.as_slice(), save.AV.as_slice_mut());

        //
        // Compute the attitude quaternion and its derivative
        // at SCLKDP.
        //
        support::ZZCKE06(
            save.RECORD.as_slice_mut(),
            save.QSTATE.as_slice_mut(),
            &mut save.CLKOUT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Create C-matrix and AV from the quaternion state.
        //
        spicelib::Q2M(save.QSTATE.as_slice(), save.CMAT.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::QDQ2AV(
            save.QSTATE.as_slice(),
            save.QSTATE.subarray(5),
            save.AV.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::MOVED(save.QSTATE.as_slice(), 4, save.QUAT.as_slice_mut());

        //
        // Compute the expected C-matrix and angular velocity.
        //
        spicelib::CKE06(
            true,
            save.REC2.as_slice_mut(),
            save.XCMAT.as_slice_mut(),
            save.XAV.as_slice_mut(),
            &mut save.XSCLK,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check C-matrix derived from ZZCKE06 output state.
        //
        save.TOL = TIGHT;

        testutil::CHCKAD(
            b"CMAT",
            save.CMAT.as_slice(),
            b"~~/",
            save.XCMAT.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check C-matrix quotient rotation angle.
        //
        spicelib::MTXM(
            save.CMAT.as_slice(),
            save.XCMAT.as_slice(),
            save.QMAT.as_slice_mut(),
        );

        spicelib::RAXISA(
            save.QMAT.as_slice(),
            save.QAXIS.as_slice_mut(),
            &mut save.QANGLE,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
        testutil::CHCKSD(b"quotient angle", save.QANGLE, b"~", 0.0, save.TOL, OK, ctx)?;

        //
        // Check angular velocity derived from ZZCKE06 output state.
        //
        save.TOL = TIGHT;
        testutil::CHCKAD(
            b"AV",
            save.AV.as_slice(),
            b"~~/",
            save.XAV.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check output SCLK from ZZCKE06.
        //
        testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"~", save.XSCLK, save.TOL, OK, ctx)?;

        //
        // Check quaternion "continuity." We expect a rotation angle
        // of less than two degrees between the quaternion and its
        // predecessor. Note that the dot product of two quaternions
        // Q1, Q2 is the real part of their quotient (Q1_conj * Q2).
        //
        if (I > 1) {
            save.QQANGL = (f64::acos(spicelib::BRCKTD(
                spicelib::VDOTG(save.QUAT.as_slice(), save.PREVQ.as_slice(), 4),
                -1.0,
                1.0,
            )) * spicelib::DPR(ctx));

            testutil::CHCKSD(b"QQANGL", save.QQANGL, b"<", 2.0, 0.0, OK, ctx)?;
        }

        //
        // Save the quaternion for the continuity check on the
        // next pass.
        //
        spicelib::MOVED(save.QUAT.as_slice(), 4, save.PREVQ.as_slice_mut());
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Continuity test using type 6, subtype 2 attitude record.",
        ctx,
    )?;

    //
    // We use the packet set from the previous test.
    //
    save.SUBTYP = 2;

    save.DEGREE = 11;
    save.WINSIZ = ((save.DEGREE + 1) / 2);
    save.PKTSIZ = C06PS2;

    save.ANGLE0 = ((180.0 - 0.6) * spicelib::RPD(ctx));

    for I in 1..=save.WINSIZ {
        save.SCLKDP = (((I - 1) as f64) * save.TDELTA);
        save.ALPHA = ((spicelib::RPD(ctx) * 100 as f64) / save.TDELTA);
        save.ANGLE = ((save.ALPHA * intrinsics::pow((I - 1), 2) as f64) + save.ANGLE0);

        spicelib::AXISAR(save.AXIS.as_slice(), -save.ANGLE, save.CMAT.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.C = f64::cos((save.ANGLE / 2 as f64));
        save.S = f64::sin((save.ANGLE / 2 as f64));

        save.QUAT[0] = save.C;
        spicelib::VSCL(save.S, save.AXIS.as_slice(), save.QUAT.subarray_mut(1));

        save.CLKBUF[I] = save.SCLKDP;
        save.PTR = (((I - 1) * save.PKTSIZ) + 1);

        spicelib::MOVED(save.QUAT.as_slice(), 4, save.PACKTS.subarray_mut(save.PTR));

        //
        // Compute the quaternion derivative with respect
        // to TDB; store it after the quaternion.
        //
        // As in the previous case, the angular rate
        // in radians/second is
        //
        save.W =
            (((save.RATE * 2 as f64) * save.ALPHA) * (save.SCLKDP / f64::powi(save.TDELTA, 2)));

        save.DQ[0] = -((0.5 * save.S) * save.W);

        spicelib::VSCL(
            ((0.5 * save.C) * save.W),
            save.AXIS.as_slice(),
            save.DQ.subarray_mut(1),
        );

        spicelib::MOVED(
            save.DQ.as_slice(),
            4,
            save.PACKTS.subarray_mut((save.PTR + 4)),
        );

        //
        // Next up is angular velocity.
        //
        spicelib::QDQ2AV(
            save.QUAT.as_slice(),
            save.DQ.as_slice(),
            save.AV.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VEQU(save.AV.as_slice(), save.PACKTS.subarray_mut((save.PTR + 8)));

        //
        // We need angular acceleration. The angular velocity
        // direction is fixed; we just need the acceleration
        // of the rotation angle. We already have
        //
        //       d(ANGLE)
        //       --------- =   2 * ALPHA * SCLKDP / TDELTA**2
        //       d(SCLKDP)
        //
        // so
        //
        //
        //         d(ANGLE)
        //       ------------ =   2 * ALPHA / TDELTA**2
        //       d**2(SCLKDP)
        //
        //
        // and
        //
        //       d(ANGLE)
        //       ---------    =  ALPHA / (RATE*TDELTA)**2
        //       d**2(TDB)
        //
        //
        spicelib::VSCL(
            (save.ALPHA / f64::powi((save.RATE * save.TDELTA), 2)),
            save.AXIS.as_slice(),
            save.DAV.as_slice_mut(),
        );

        spicelib::VEQU(
            save.DAV.as_slice(),
            save.PACKTS.subarray_mut((save.PTR + 11)),
        );
    }

    save.NSAMP = 300;
    save.SPAN = (save.CLKBUF[save.WINSIZ] - save.CLKBUF[1]);
    save.DELTA = (save.SPAN / save.NSAMP as f64);

    for I in 1..=save.NSAMP {
        //
        // Set the sample time.
        //
        save.SCLKDP = (((I - 1) as f64) * save.DELTA);

        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(
            &mut save.TITLE,
            b"Subtype 2 continuity test for I = #; SCLKDP #",
        );
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.SCLKDP,
            14,
            &mut save.TITLE,
            ctx,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Create a type 6 pointing record using the SCLK, subtype, and
        // packet count. We have to re-create the record for each call
        // because the CK evaluators use it destructively.
        //
        save.RECORD[1] = save.SCLKDP;
        save.RECORD[2] = (save.SUBTYP as f64);
        save.RECORD[3] = (save.WINSIZ as f64);
        save.RECORD[4] = save.RATE;

        spicelib::MOVED(
            save.PACKTS.as_slice(),
            (save.PKTSIZ * save.WINSIZ),
            save.RECORD.subarray_mut(5),
        );

        save.PTR = ((save.PKTSIZ * save.WINSIZ) + 5);

        spicelib::MOVED(
            save.CLKBUF.as_slice(),
            save.WINSIZ,
            save.RECORD.subarray_mut(save.PTR),
        );

        save.SIZE = (4 + ((save.PKTSIZ + 1) * save.WINSIZ));

        spicelib::MOVED(save.RECORD.as_slice(), save.SIZE, save.REC2.as_slice_mut());

        //
        // Compute the angular velocity at the sample time.
        // Units are radians/second.
        //
        //    We have
        //
        //       ANGLE     =   ALPHA * (I-1)**2
        //
        //       SCLKDP    =   (I-1)*TDELTA
        //
        //    so
        //
        //       ANGLE     =   ALPHA * (SCLKDP/TDELTA)**2
        //
        //    and
        //
        //       d(ANGLE)
        //       --------- =   2 * ALPHA * SCLKDP / TDELTA**2
        //       d(SCLKDP)
        //

        save.W =
            (((save.RATE * 2 as f64) * save.ALPHA) * (save.SCLKDP / f64::powi(save.TDELTA, 2)));

        spicelib::VSCL(save.W, save.AXIS.as_slice(), save.AV.as_slice_mut());

        //
        // Compute the attitude quaternion and its derivative
        // at SCLKDP.
        //
        support::ZZCKE06(
            save.RECORD.as_slice_mut(),
            save.QSTATE.as_slice_mut(),
            &mut save.CLKOUT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Create C-matrix and AV from the quaternion state.
        //
        spicelib::Q2M(save.QSTATE.as_slice(), save.CMAT.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::QDQ2AV(
            save.QSTATE.as_slice(),
            save.QSTATE.subarray(5),
            save.AV.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::MOVED(save.QSTATE.as_slice(), 4, save.QUAT.as_slice_mut());

        //
        // Compute the expected C-matrix and angular velocity.
        //
        spicelib::CKE06(
            true,
            save.REC2.as_slice_mut(),
            save.XCMAT.as_slice_mut(),
            save.XAV.as_slice_mut(),
            &mut save.XSCLK,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check C-matrix derived from ZZCKE06 output state.
        //
        save.TOL = VTIGHT;

        testutil::CHCKAD(
            b"CMAT",
            save.CMAT.as_slice(),
            b"~~/",
            save.XCMAT.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check C-matrix quotient rotation angle.
        //
        spicelib::MTXM(
            save.CMAT.as_slice(),
            save.XCMAT.as_slice(),
            save.QMAT.as_slice_mut(),
        );

        spicelib::RAXISA(
            save.QMAT.as_slice(),
            save.QAXIS.as_slice_mut(),
            &mut save.QANGLE,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
        testutil::CHCKSD(b"quotient angle", save.QANGLE, b"~", 0.0, save.TOL, OK, ctx)?;

        //
        // Check angular velocity derived from ZZCKE06 output state.
        //
        save.TOL = VTIGHT;
        testutil::CHCKAD(
            b"AV",
            save.AV.as_slice(),
            b"~~/",
            save.XAV.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check output SCLK from ZZCKE06.
        //
        testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"~", save.XSCLK, save.TOL, OK, ctx)?;

        //
        // Check quaternion "continuity." We expect a rotation angle
        // of less than two degrees between the quaternion and its
        // predecessor. Note that the dot product of two quaternions
        // Q1, Q2 is the real part of their quotient (Q1_conj * Q2).
        //
        if (I > 1) {
            save.QQANGL = (f64::acos(spicelib::BRCKTD(
                spicelib::VDOTG(save.QUAT.as_slice(), save.PREVQ.as_slice(), 4),
                -1.0,
                1.0,
            )) * spicelib::DPR(ctx));

            testutil::CHCKSD(b"QQANGL", save.QQANGL, b"<", 2.0, 0.0, OK, ctx)?;
        }

        //
        // Save the quaternion for the continuity check on the
        // next pass.
        //
        spicelib::MOVED(save.QUAT.as_slice(), 4, save.PREVQ.as_slice_mut());
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Continuity test using type 6, subtype 3 attitude record.",
        ctx,
    )?;

    //
    // We use the packet set from the previous test.
    //
    save.SUBTYP = 3;

    save.DEGREE = 11;
    save.WINSIZ = (save.DEGREE + 1);
    save.PKTSIZ = C06PS3;

    save.ANGLE0 = ((180.0 - 0.6) * spicelib::RPD(ctx));

    for I in 1..=save.WINSIZ {
        save.SCLKDP = (((I - 1) as f64) * save.TDELTA);
        save.ALPHA = ((spicelib::RPD(ctx) * 100 as f64) / save.TDELTA);
        save.ANGLE = ((save.ALPHA * intrinsics::pow((I - 1), 2) as f64) + save.ANGLE0);

        spicelib::AXISAR(save.AXIS.as_slice(), -save.ANGLE, save.CMAT.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.C = f64::cos((save.ANGLE / 2 as f64));
        save.S = f64::sin((save.ANGLE / 2 as f64));

        save.QUAT[0] = save.C;
        spicelib::VSCL(save.S, save.AXIS.as_slice(), save.QUAT.subarray_mut(1));

        save.CLKBUF[I] = save.SCLKDP;
        save.PTR = (((I - 1) * save.PKTSIZ) + 1);

        spicelib::MOVED(save.QUAT.as_slice(), 4, save.PACKTS.subarray_mut(save.PTR));

        //
        // Compute the quaternion derivative with respect
        // to TDB; store it after the quaternion.
        //
        // As in the previous case, the angular rate
        // in radians/second is
        //
        save.W =
            (((save.RATE * 2 as f64) * save.ALPHA) * (save.SCLKDP / f64::powi(save.TDELTA, 2)));

        save.DQ[0] = -((0.5 * save.S) * save.W);

        spicelib::VSCL(
            ((0.5 * save.C) * save.W),
            save.AXIS.as_slice(),
            save.DQ.subarray_mut(1),
        );

        spicelib::MOVED(
            save.DQ.as_slice(),
            4,
            save.PACKTS.subarray_mut((save.PTR + 4)),
        );

        //
        // Next up is angular velocity.
        //
        spicelib::QDQ2AV(
            save.QUAT.as_slice(),
            save.DQ.as_slice(),
            save.AV.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VEQU(save.AV.as_slice(), save.PACKTS.subarray_mut((save.PTR + 8)));
    }

    save.NSAMP = 300;
    save.SPAN = (save.CLKBUF[save.WINSIZ] - save.CLKBUF[1]);
    save.DELTA = (save.SPAN / save.NSAMP as f64);

    for I in 1..=save.NSAMP {
        //
        // Set the sample time.
        //
        save.SCLKDP = (((I - 1) as f64) * save.DELTA);

        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(
            &mut save.TITLE,
            b"Subtype 3 continuity test for I = #; SCLKDP #",
        );
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.SCLKDP,
            14,
            &mut save.TITLE,
            ctx,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Create a type 6 pointing record using the SCLK, subtype, and
        // packet count. We have to re-create the record for each call
        // because the CK evaluators use it destructively.
        //
        save.RECORD[1] = save.SCLKDP;
        save.RECORD[2] = (save.SUBTYP as f64);
        save.RECORD[3] = (save.WINSIZ as f64);
        save.RECORD[4] = save.RATE;

        spicelib::MOVED(
            save.PACKTS.as_slice(),
            (save.PKTSIZ * save.WINSIZ),
            save.RECORD.subarray_mut(5),
        );

        save.PTR = ((save.PKTSIZ * save.WINSIZ) + 5);

        spicelib::MOVED(
            save.CLKBUF.as_slice(),
            save.WINSIZ,
            save.RECORD.subarray_mut(save.PTR),
        );

        save.SIZE = (4 + ((save.PKTSIZ + 1) * save.WINSIZ));

        spicelib::MOVED(save.RECORD.as_slice(), save.SIZE, save.REC2.as_slice_mut());

        //
        // Compute the angular velocity at the sample time.
        // Units are radians/second.
        //
        //    We have
        //
        //       ANGLE     =   ALPHA * (I-1)**2
        //
        //       SCLKDP    =   (I-1)*TDELTA
        //
        //    so
        //
        //       ANGLE     =   ALPHA * (SCLKDP/TDELTA)**2
        //
        //    and
        //
        //       d(ANGLE)
        //       --------- =   2 * ALPHA * SCLKDP / TDELTA**2
        //       d(SCLKDP)
        //

        save.W =
            (((save.RATE * 2 as f64) * save.ALPHA) * (save.SCLKDP / f64::powi(save.TDELTA, 2)));

        spicelib::VSCL(save.W, save.AXIS.as_slice(), save.AV.as_slice_mut());

        //
        // Compute the attitude quaternion and its derivative
        // at SCLKDP.
        //
        support::ZZCKE06(
            save.RECORD.as_slice_mut(),
            save.QSTATE.as_slice_mut(),
            &mut save.CLKOUT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Create C-matrix and AV from the quaternion state.
        //
        spicelib::Q2M(save.QSTATE.as_slice(), save.CMAT.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::QDQ2AV(
            save.QSTATE.as_slice(),
            save.QSTATE.subarray(5),
            save.AV.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::MOVED(save.QSTATE.as_slice(), 4, save.QUAT.as_slice_mut());

        //
        // Compute the expected C-matrix and angular velocity.
        //
        spicelib::CKE06(
            true,
            save.REC2.as_slice_mut(),
            save.XCMAT.as_slice_mut(),
            save.XAV.as_slice_mut(),
            &mut save.XSCLK,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check C-matrix derived from ZZCKE06 output state.
        //
        save.TOL = VTIGHT;

        testutil::CHCKAD(
            b"CMAT",
            save.CMAT.as_slice(),
            b"~~/",
            save.XCMAT.as_slice(),
            9,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check C-matrix quotient rotation angle.
        //
        spicelib::MTXM(
            save.CMAT.as_slice(),
            save.XCMAT.as_slice(),
            save.QMAT.as_slice_mut(),
        );

        spicelib::RAXISA(
            save.QMAT.as_slice(),
            save.QAXIS.as_slice_mut(),
            &mut save.QANGLE,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = VTIGHT;
        testutil::CHCKSD(b"quotient angle", save.QANGLE, b"~", 0.0, save.TOL, OK, ctx)?;

        //
        // Check angular velocity derived from ZZCKE06 output state.
        //
        save.TOL = VTIGHT;
        testutil::CHCKAD(
            b"AV",
            save.AV.as_slice(),
            b"~~/",
            save.XAV.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check output SCLK from ZZCKE06.
        //
        testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"~", save.XSCLK, save.TOL, OK, ctx)?;

        //
        // Check quaternion "continuity." We expect a rotation angle
        // of less than two degrees between the quaternion and its
        // predecessor. Note that the dot product of two quaternions
        // Q1, Q2 is the real part of their quotient (Q1_conj * Q2).
        //
        if (I > 1) {
            save.QQANGL = (f64::acos(spicelib::BRCKTD(
                spicelib::VDOTG(save.QUAT.as_slice(), save.PREVQ.as_slice(), 4),
                -1.0,
                1.0,
            )) * spicelib::DPR(ctx));

            testutil::CHCKSD(b"QQANGL", save.QQANGL, b"<", 2.0, 0.0, OK, ctx)?;
        }

        //
        // Save the quaternion for the continuity check on the
        // next pass.
        //
        spicelib::MOVED(save.QUAT.as_slice(), 4, save.PREVQ.as_slice_mut());
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
