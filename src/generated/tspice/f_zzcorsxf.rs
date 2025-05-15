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
const VTIGHT: f64 = 0.000000000001;
const LNSIZE: i32 = 80;

struct SaveVars {
    TITLE: Vec<u8>,
    CORXFM: StackArray2D<f64, 36>,
    DLT: f64,
    LTSIGN: f64,
    XFORM: StackArray2D<f64, 36>,
    XCORXF: StackArray2D<f64, 36>,
    XMIT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut CORXFM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut DLT: f64 = 0.0;
        let mut LTSIGN: f64 = 0.0;
        let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut XCORXF = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut XMIT: bool = false;

        Self {
            TITLE,
            CORXFM,
            DLT,
            LTSIGN,
            XFORM,
            XCORXF,
            XMIT,
        }
    }
}

//$Procedure      F_ZZCORSXF ( Test ZZCORSXF )
pub fn F_ZZCORSXF(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Save everything.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZCORSXF", ctx)?;

    //
    // ZZCORSXF is error free,so there are no error cases to
    // check.
    //

    //
    // Loop over the radiation direction cases.
    //
    save.DLT = 0.05;

    for I in 1..=2 {
        //
        //---- Case -------------------------------------------------------------
        //
        if (I == 1) {
            save.XMIT = true;
            save.LTSIGN = 1.0;
            fstr::assign(
                &mut save.TITLE,
                b"Initial matrix filled with index values. XMIT = .TRUE.",
            );
        } else {
            save.XMIT = false;
            save.LTSIGN = -1.0;
            fstr::assign(
                &mut save.TITLE,
                b"Initial matrix filled with index values. XMIT = .FALSE.",
            );
        }

        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Since ZZCORSXF doesn't check the input matrix, we don't
        // have to use a real state transformation. Instead, simply
        // fill the original matrix with identifiable, distinct
        // values.
        //
        for J in 1..=6 {
            for K in 1..=6 {
                save.XFORM[[K, J]] = (((J - 1) * 6) + K) as f64;
            }
        }

        //
        // Form the expected, transformed matrix.
        //
        spicelib::MOVED(save.XFORM.as_slice(), 36, save.XCORXF.as_slice_mut());

        for J in 1..=3 {
            for K in 4..=6 {
                save.XCORXF[[K, J]] = (save.XFORM[[K, J]] * (1.0 + (save.LTSIGN * save.DLT)));
            }
        }

        spicelib::ZZCORSXF(
            save.XMIT,
            save.DLT,
            save.XFORM.as_slice(),
            save.CORXFM.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"CORXFM",
            save.CORXFM.as_slice(),
            b"~",
            save.XCORXF.as_slice(),
            36,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
