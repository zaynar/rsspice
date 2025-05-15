//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SRFIDX: i32 = 1;
const CTRIDX: i32 = (SRFIDX + 1);
const CLSIDX: i32 = (CTRIDX + 1);
const TYPIDX: i32 = (CLSIDX + 1);
const FRMIDX: i32 = (TYPIDX + 1);
const SYSIDX: i32 = (FRMIDX + 1);
const PARIDX: i32 = (SYSIDX + 1);
const NSYPAR: i32 = 10;
const MN1IDX: i32 = (PARIDX + NSYPAR);
const MX1IDX: i32 = (MN1IDX + 1);
const MN2IDX: i32 = (MX1IDX + 1);
const MX2IDX: i32 = (MN2IDX + 1);
const MN3IDX: i32 = (MX2IDX + 1);
const MX3IDX: i32 = (MN3IDX + 1);
const BTMIDX: i32 = (MX3IDX + 1);
const ETMIDX: i32 = (BTMIDX + 1);
const DSKDSZ: i32 = ETMIDX;
const SVFCLS: i32 = 1;
const GENCLS: i32 = 2;
const LATSYS: i32 = 1;
const CYLSYS: i32 = 2;
const RECSYS: i32 = 3;
const PDTSYS: i32 = 4;

//$Procedure ZZDSKSGR ( DSK, return segment bounding radius )
pub fn ZZDSKSGR(DSKDSC: &[f64], ctx: &mut Context) -> f2rust_std::Result<f64> {
    let DSKDSC = DummyArray::new(DSKDSC, 1..);
    let mut ZZDSKSGR: f64 = 0.0;
    let mut BDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
    let mut F: f64 = 0.0;
    let mut MAG = StackArray::<f64, 3>::new(1..=3);
    let mut MINR: f64 = 0.0;
    let mut RE: f64 = 0.0;
    let mut RP: f64 = 0.0;
    let mut B: i32 = 0;
    let mut CORSYS: i32 = 0;
    let mut J: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //

    //
    // Set an initial return value.
    //
    ZZDSKSGR = -1.0;

    //
    // The radius calculation depends on the coordinate system.
    //
    CORSYS = intrinsics::IDNINT(DSKDSC[SYSIDX]);

    if (CORSYS == LATSYS) {
        //
        // Fetch the minimum radius from the descriptor.
        //
        MINR = DSKDSC[MN3IDX];

        if (MINR <= 0.0) {
            CHKIN(b"ZZDSKSGR", ctx)?;
            SETMSG(b"Minimum radius was *.", ctx);
            ERRDP(b"*", MINR, ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
            CHKOUT(b"ZZDSKSGR", ctx)?;
            return Ok(ZZDSKSGR);
        }

        //
        // This is as simple as it gets. The radius bounds
        // correspond to the third coordinate in the descriptor.
        //
        ZZDSKSGR = DSKDSC[MX3IDX];
    } else if (CORSYS == PDTSYS) {
        //
        // Fetch the equatorial radius from the descriptor.
        //
        RE = DSKDSC[PARIDX];

        if (RE <= 0.0) {
            CHKIN(b"ZZDSKSGR", ctx)?;
            SETMSG(b"Equatorial radius was *.", ctx);
            ERRDP(b"*", RE, ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
            CHKOUT(b"ZZDSKSGR", ctx)?;
            return Ok(ZZDSKSGR);
        }

        //
        // Fetch the flattening coefficient from the descriptor.
        //
        F = DSKDSC[(PARIDX + 1)];

        if ((F >= 0.0) && (F < 1.0)) {
            //
            // This is the oblate case.
            //
            // The maximum radius of an oblate planetodetic boundary
            // occurs on the X-Y plane at the maximum height.
            //
            ZZDSKSGR = (DSKDSC[MX3IDX] + RE);
        } else if (F < 0.0) {
            //
            // This is the prolate case.
            //
            // The maximum radius of an prolate planetodetic boundary
            // occurs on the poles at the maximum height.
            //
            RE = DSKDSC[PARIDX];
            RP = (RE * (1.0 - F));
            ZZDSKSGR = (DSKDSC[MX3IDX] + RP);
        } else {
            //
            // We have an invalid flattening coefficient.
            //
            // If the flattening coefficient is greater than one, the
            // polar radius computed below is negative. If it's equal to
            // one, the polar radius is zero. Either case is a problem, so
            // signal an error and check out.
            //
            CHKIN(b"ZZDSKSGR", ctx)?;
            SETMSG(b"Flattening coefficient was *.", ctx);
            ERRDP(b"*", F, ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
            CHKOUT(b"ZZDSKSGR", ctx)?;
            return Ok(ZZDSKSGR);
        }
    } else if (CORSYS == RECSYS) {
        //
        // The bounding cell of the segment has its extreme value at
        // a corner. Just take the maximum of these values.
        //
        // First copy the bounds into an appropriately dimensioned
        // array.
        //
        MOVED(DSKDSC.subarray(MN1IDX), 6, BDS.as_slice_mut());

        B = (MN1IDX - 1);

        for I in 1..=3 {
            J = ((B + (2 * I)) - 1);

            MAG[I] = intrinsics::DMAX1(&[f64::abs(DSKDSC[J]), f64::abs(DSKDSC[(J + 1)])]);
        }

        ZZDSKSGR = VNORM(MAG.as_slice());
    } else {
        //
        // Never heard of this coordinate system.
        //
        CHKIN(b"ZZDSKSGR", ctx)?;
        SETMSG(b"The coordinate system code # is not recognized.", ctx);
        ERRINT(b"#", CORSYS, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZDSKSGR", ctx)?;
        return Ok(ZZDSKSGR);
    }

    Ok(ZZDSKSGR)
}
