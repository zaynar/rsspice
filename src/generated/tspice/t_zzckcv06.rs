//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

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
const LBCELL: i32 = -5;
const ND: i32 = 2;
const NI: i32 = 6;

//$Procedure T_ZZCKCV06 (Private --- C-kernel segment coverage, type 06)
pub fn T_ZZCKCV06(
    HANDLE: i32,
    DESCR: &[f64],
    SCLKID: i32,
    TOL: f64,
    TIMSYS: &[u8],
    SCHEDL: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..);
    let mut SCHEDL = DummyArrayMut::new(SCHEDL, LBCELL..);
    let mut BEGIN: f64 = 0.0;
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut ET: f64 = 0.0;
    let mut FINISH: f64 = 0.0;
    let mut IVLBDS = StackArray::<f64, 2>::new(1..=2);
    let mut LSTEPC: f64 = 0.0;
    let mut RATE: f64 = 0.0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut NINTVL: i32 = 0;
    let mut NREC: i32 = 0;
    let mut SUBTYP: i32 = 0;
    let mut WINSIZ: i32 = 0;
    let mut ISTDB: bool = false;

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
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_ZZCKCV06", ctx)?;

    //
    // Check tolerance value.
    //
    if (TOL < 0.0) {
        spicelib::SETMSG(b"Tolerance must be non-negative; actual value was #.", ctx);
        spicelib::ERRDP(b"#", TOL, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"T_ZZCKCV06", ctx)?;
        return Ok(());
    }

    //
    // Set a logical flag indicating whether the time system is SCLK.
    //
    ISTDB = spicelib::EQSTR(TIMSYS, b"TDB");

    //
    // Check time system.
    //
    if !ISTDB {
        if !spicelib::EQSTR(TIMSYS, b"SCLK") {
            spicelib::SETMSG(
                b"Time system spec TIMSYS was #; allowed values are SCLK and TDB.",
                ctx,
            );
            spicelib::ERRCH(b"#", TIMSYS, ctx);
            spicelib::SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
            spicelib::CHKOUT(b"T_ZZCKCV06", ctx)?;
            return Ok(());
        }
    }

    //
    // Unpack the descriptor.
    //
    spicelib::DAFUS(
        DESCR.as_slice(),
        ND,
        NI,
        DC.as_slice_mut(),
        IC.as_slice_mut(),
    );

    //
    // Fetch the mini-segment count from the segment.
    //
    spicelib::CKNM06(HANDLE, DESCR.as_slice(), &mut NINTVL, ctx)?;

    //
    // Each mini-segment contributes a coverage interval to the
    // total coverage of the segment. Since mini-segments can
    // contain gaps, we need to examine not only the interpolation
    // interval bounds but the final epochs of the mini-segments.
    //
    // Now loop over the mini-segments and find the contribution
    // from each one.
    //
    for I in 1..=NINTVL {
        //
        // Find the interval bounds for this mini-segment.
        //
        spicelib::CKMP06(
            HANDLE,
            DESCR.as_slice(),
            I,
            &mut RATE,
            &mut SUBTYP,
            &mut WINSIZ,
            &mut NREC,
            IVLBDS.as_slice_mut(),
            &mut LSTEPC,
            ctx,
        )?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"T_ZZCKCV06", ctx)?;
            return Ok(());
        }

        BEGIN = IVLBDS[1];
        //
        // The smaller of LSTEPC and IVLBDS(2) is the
        // end of the mini-segment's coverage.
        //
        FINISH = intrinsics::DMIN1(&[LSTEPC, IVLBDS[2]]);
        //
        // Truncate the interval using the segment bounds.
        //
        BEGIN = intrinsics::DMAX1(&[BEGIN, DC[1]]);
        FINISH = intrinsics::DMIN1(&[FINISH, DC[2]]);

        //
        // Adjust the interval using the tolerance. Empty
        // intervals *do not get expanded*; this choice is
        // consistent with the type 6 reading algorithm.
        //
        if (BEGIN <= FINISH) {
            if (TOL > 0.0) {
                BEGIN = intrinsics::DMAX1(&[(BEGIN - TOL), 0.0]);
                FINISH = (FINISH + TOL);
            }
        }

        //
        // Convert the time to TDB if necessary.
        //
        if ISTDB {
            spicelib::SCT2E(SCLKID, BEGIN, &mut ET, ctx)?;
            BEGIN = ET;

            spicelib::SCT2E(SCLKID, FINISH, &mut ET, ctx)?;
            FINISH = ET;

            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(b"T_ZZCKCV06", ctx)?;
                return Ok(());
            }
        }

        //
        // Insert the interval into the window.
        //
        if (BEGIN <= FINISH) {
            spicelib::WNINSD(BEGIN, FINISH, SCHEDL.as_slice_mut(), ctx)?;

            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(b"T_ZZCKCV06", ctx)?;
                return Ok(());
            }
        }
    }

    spicelib::CHKOUT(b"T_ZZCKCV06", ctx)?;
    Ok(())
}
