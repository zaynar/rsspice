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
const TIGHT: f64 = 0.0000000000001;

//$Procedure      F_ZZDSKSGR ( Test ZZDSKSGR )
pub fn F_ZZDSKSGR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
    let mut CORNER = StackArray::<f64, 3>::new(1..=3);
    let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
    let mut F: f64 = 0.0;
    let mut R: f64 = 0.0;
    let mut RE: f64 = 0.0;
    let mut RP: f64 = 0.0;
    let mut TOL: f64 = 0.0;
    let mut XR: f64 = 0.0;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZDSKSGR", ctx)?;

    //
    // ZZDSKSGR error cases:
    //

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Error case: unrecognized coordinate system code.", ctx)?;

    spicelib::CLEARD(DSKDSZ, DSKDSC.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    DSKDSC[SYSIDX] = (-1 as f64);

    R = spicelib::ZZDSKSGR(DSKDSC.as_slice(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Error case: bad flattening coefficient.", ctx)?;

    spicelib::CLEARD(DSKDSZ, DSKDSC.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    DSKDSC[SYSIDX] = (PDTSYS as f64);

    RE = 3000.0;
    F = (1.0 + 0.0000000000001);
    DSKDSC[PARIDX] = RE;
    DSKDSC[(PARIDX + 1)] = F;

    R = spicelib::ZZDSKSGR(DSKDSC.as_slice(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Error case: negative equatorial radius.", ctx)?;

    spicelib::CLEARD(DSKDSZ, DSKDSC.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    DSKDSC[SYSIDX] = (PDTSYS as f64);

    RE = -3000.0;
    F = 0.0;
    DSKDSC[PARIDX] = RE;
    DSKDSC[(PARIDX + 1)] = F;

    R = spicelib::ZZDSKSGR(DSKDSC.as_slice(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Error case: negative minimum radius.", ctx)?;

    spicelib::CLEARD(DSKDSZ, DSKDSC.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    DSKDSC[SYSIDX] = (LATSYS as f64);

    DSKDSC[MN3IDX] = -1.0;
    DSKDSC[MX3IDX] = 1.0;

    R = spicelib::ZZDSKSGR(DSKDSC.as_slice(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // ZZDSKSGR normal cases:
    //

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Find radius for rectangular boundary.", ctx)?;

    spicelib::CLEARD(DSKDSZ, DSKDSC.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    DSKDSC[SYSIDX] = (RECSYS as f64);

    DSKDSC[MN1IDX] = -7.0;
    DSKDSC[MX1IDX] = 3.0;
    DSKDSC[MN2IDX] = 5.0;
    DSKDSC[MX2IDX] = 6.0;
    DSKDSC[MN3IDX] = -1.0;
    DSKDSC[MX3IDX] = 3.0;

    //
    // Compute expected radius. This is the magnitude of the longest
    // vector from the origin to a corner of the bounding box.
    //
    // Use the dumbest possible brute force approach. This is
    // done to have a different algorithm from that in the
    // SPICELIB routine.
    //
    spicelib::MOVED(DSKDSC.subarray(MN1IDX), 6, BDS.as_slice_mut());

    XR = 0.0;

    for I in 1..=2 {
        CORNER[1] = BDS[[I, 1]];

        for J in 1..=2 {
            CORNER[2] = BDS[[J, 2]];

            for K in 1..=2 {
                CORNER[3] = BDS[[K, 3]];

                XR = intrinsics::DMAX1(&[XR, spicelib::VNORM(CORNER.as_slice())]);
            }
        }
    }

    R = spicelib::ZZDSKSGR(DSKDSC.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    TOL = TIGHT;

    testutil::CHCKSD(b"R", R, b"~/", XR, TOL, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Find radius for latitudinal boundary.", ctx)?;

    spicelib::CLEARD(DSKDSZ, DSKDSC.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    DSKDSC[SYSIDX] = (LATSYS as f64);

    DSKDSC[MN1IDX] = -(spicelib::PI(ctx) / 4 as f64);
    DSKDSC[MX1IDX] = (spicelib::PI(ctx) / 2 as f64);
    DSKDSC[MN2IDX] = (spicelib::PI(ctx) / 4 as f64);
    DSKDSC[MX2IDX] = (spicelib::PI(ctx) / 3 as f64);
    DSKDSC[MN3IDX] = 1000.0;
    DSKDSC[MX3IDX] = 2000.0;

    XR = DSKDSC[MX3IDX];

    R = spicelib::ZZDSKSGR(DSKDSC.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    TOL = 0.0;

    testutil::CHCKSD(b"R", R, b"~/", XR, TOL, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Find radius for oblate planetodetic boundary.", ctx)?;

    spicelib::CLEARD(DSKDSZ, DSKDSC.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    DSKDSC[SYSIDX] = (PDTSYS as f64);

    RE = 3000.0;
    RP = 2000.0;
    F = ((RE - RP) / RE);
    DSKDSC[PARIDX] = RE;
    DSKDSC[(PARIDX + 1)] = F;

    DSKDSC[MN1IDX] = -(spicelib::PI(ctx) / 4 as f64);
    DSKDSC[MX1IDX] = (spicelib::PI(ctx) / 2 as f64);
    DSKDSC[MN2IDX] = (spicelib::PI(ctx) / 4 as f64);
    DSKDSC[MX2IDX] = (spicelib::PI(ctx) / 3 as f64);
    DSKDSC[MN3IDX] = -10.0;
    DSKDSC[MX3IDX] = 20.0;

    //
    // The largest distance from the origin is on the equator
    // at max altitude.
    //
    XR = (RE + DSKDSC[MX3IDX]);

    R = spicelib::ZZDSKSGR(DSKDSC.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    TOL = TIGHT;

    testutil::CHCKSD(b"R", R, b"~/", XR, TOL, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
