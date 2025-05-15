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
const LBCELL: i32 = -5;
const ND: i32 = 2;
const NI: i32 = 6;

//$Procedure ZZCKCVR4 ( Private --- C-kernel segment coverage, type 04 )
pub fn ZZCKCVR4(
    HANDLE: i32,
    ARRBEG: i32,
    ARREND: i32,
    SCHEDL: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut SCHEDL = DummyArrayMut::new(SCHEDL, LBCELL..);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=5);
    let mut LEFT: f64 = 0.0;
    let mut RIGHT: f64 = 0.0;
    let mut VALUES = StackArray::<f64, 143>::new(1..=CK4RSZ);
    let mut ENDS = StackArray::<i32, 2>::new(1..=2);
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut NREC: i32 = 0;

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"ZZCKCVR4", ctx)?;

    //
    // Build a descriptor record that satisfies the requirements
    // of CKNR04 and SGFPKT.
    //
    // Note: This is a hack dependent on the implementation of
    // the generic segments routines.  But for C-kernels it
    // should always work, as ND and NI aren't changing any
    // time soon.
    //
    IC[1] = spicelib::INTMAX();
    IC[2] = spicelib::INTMAX();
    IC[3] = 4;
    IC[4] = spicelib::INTMAX();
    IC[5] = ARRBEG;
    IC[6] = ARREND;

    DC[1] = 0.0;
    DC[2] = 0.0;

    spicelib::DAFPS(ND, NI, DC.as_slice(), IC.as_slice(), DESCR.as_slice_mut());

    //
    // Determine the number of records in the array.
    //
    spicelib::CKNR04(HANDLE, DESCR.as_slice(), &mut NREC, ctx)?;

    for I in 1..=NREC {
        //
        // Extract each packet of pointing coefficients.
        //
        spicelib::SGFPKT(
            HANDLE,
            DESCR.as_slice(),
            I,
            I,
            VALUES.as_slice_mut(),
            ENDS.as_slice_mut(),
            ctx,
        )?;

        //
        // Compute the left and right end points of the interval
        // of coverage related to this packet.
        //
        LEFT = (VALUES[1] - VALUES[2]);
        RIGHT = (VALUES[1] + VALUES[2]);

        //
        // Store the results in the schedule.
        //
        spicelib::WNINSD(LEFT, RIGHT, SCHEDL.as_slice_mut(), ctx)?;
    }

    spicelib::CHKOUT(b"ZZCKCVR4", ctx)?;
    Ok(())
}
