//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXL: i32 = 36;
const MAXP: i32 = 150;
const NPERM: i32 = 692;
const MAXE: i32 = 853;
const NROOM: i32 = 14983;
const SIZIDX: i32 = 0;
const FREIDX: i32 = -1;
const LBPOOL: i32 = -5;
const LBCELL: i32 = -5;
const HASHSZ: i32 = 5003;

//$Procedure F_ZZHSI ( Family of tests for ZZHSI )
pub fn F_ZZHSI(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut HEDLST = ActualArray::<i32>::new(1..=HASHSZ);
    let mut COLLST = ActualArray::<i32>::new(LBPOOL..=HASHSZ);
    let mut ITEMS = ActualArray::<i32>::new(1..=HASHSZ);
    let mut ITEMAT: i32 = 0;
    let mut NEW: bool = false;
    let mut TITEMS = ActualArray::<i32>::new(1..=HASHSZ);
    let mut IDS = ActualArray::<i32>::new(1..=HASHSZ);
    let mut PIDS = ActualArray::<i32>::new(1..=HASHSZ);
    let mut HSHVAL = ActualArray::<i32>::new(LBCELL..=HASHSZ);
    let mut EXPUSD: i32 = 0;
    let mut EXPFRE: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut HINT: i32 = 0;
    let mut AVAIL: i32 = 0;
    let mut BLTCOD = ActualArray::<i32>::new(1..=NPERM);
    let mut BLTNAM = ActualCharArray::new(MAXL, 1..=NPERM);
    let mut REPORT: bool = false;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //
    //  DOUBLE PRECISION      T_RANDD

    //
    // Local Parameters
    //

    // PARAMETER           ( HASHSZ = 26003 )

    //
    // Local Variables
    //

    // INTEGER               SEED

    //
    // Set timing reporting. Set to .FALSE. to suppress screen output.
    //
    //  REPORT = .TRUE.
    REPORT = false;

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZHSI", ctx)?;

    //
    // Initialize items and IDs - random values.
    //
    //  SEED = -1
    //
    //  DO I = 1, HASHSZ
    //
    //     IDS(I) = I * 1000
    //
    //     TITEMS(I) = NINT( T_RANDD( 1.D0, 9.D0, SEED ) ) + 10 * I
    //
    //     IF ( MOD( TITEMS(I), 2 ) .EQ. 0 ) THEN
    //        TITEMS(I) = -TITEMS(I)
    //     END IF
    //
    //  END DO

    //
    // Initialize items and IDs -- NAIF ID like values.
    //
    // First save all unique IDs built into the toolkit.
    //
    spicelib::ZZIDMAP(BLTCOD.as_slice_mut(), BLTNAM.as_arg_mut());

    I = 1;
    J = 1;

    while ((I <= NPERM) && (J <= HASHSZ)) {
        if (I > 1) {
            if (BLTCOD[I] != BLTCOD[(I - 1)]) {
                TITEMS[J] = BLTCOD[I];
                J = (J + 1);
            }
        } else {
            TITEMS[J] = BLTCOD[I];
            J = (J + 1);
        }
        I = (I + 1);
    }

    //
    // Then save 63 extra IDs based on each unique each s/c ID.
    //
    if (J < HASHSZ) {
        {
            let m1__: i32 = 2;
            let m2__: i32 = NPERM;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if ((BLTCOD[I] < 0) && (BLTCOD[I] != BLTCOD[(I - 1)])) {
                    for K in 1..=9 {
                        if (J <= HASHSZ) {
                            TITEMS[J] = ((BLTCOD[I] * 1000) - (K * 100));
                            J = (J + 1);
                        }
                        if (J <= HASHSZ) {
                            TITEMS[J] = (((BLTCOD[I] * 1000) - (K * 100)) - 1);
                            J = (J + 1);
                        }
                        if (J <= HASHSZ) {
                            TITEMS[J] = (((BLTCOD[I] * 1000) - (K * 100)) - 2);
                            J = (J + 1);
                        }
                        if (J <= HASHSZ) {
                            TITEMS[J] = (((BLTCOD[I] * 1000) - (K * 100)) - 5);
                            J = (J + 1);
                        }
                        if (J <= HASHSZ) {
                            TITEMS[J] = (((BLTCOD[I] * 1000) - (K * 100)) - 10);
                            J = (J + 1);
                        }
                        if (J <= HASHSZ) {
                            TITEMS[J] = (((BLTCOD[I] * 1000) - (K * 100)) - 11);
                            J = (J + 1);
                        }
                        if (J <= HASHSZ) {
                            TITEMS[J] = (((BLTCOD[I] * 1000) - (K * 100)) - 12);
                            J = (J + 1);
                        }
                    }
                }
                I += m3__;
            }
        }
    }

    //
    // Then add some asteroid IDs at the end to fill up the hash.
    //
    {
        let m1__: i32 = J;
        let m2__: i32 = HASHSZ;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            TITEMS[I] = (2500000 + I);
            I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = HASHSZ;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            IDS[I] = (I * 1000);
            I += m3__;
        }
    }

    //
    // Calculate hash values for all IDs in the buffer and store them in
    // a set to get the expected numbers of occupied and free head
    // nodes.
    //
    spicelib::SSIZEI(HASHSZ, HSHVAL.as_slice_mut(), ctx)?;
    {
        let m1__: i32 = 1;
        let m2__: i32 = HASHSZ;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            J = spicelib::ZZHASHI(TITEMS[I], HASHSZ, ctx)?;
            spicelib::INSRTI(J, HSHVAL.as_slice_mut(), ctx)?;
            I += m3__;
        }
    }

    EXPUSD = spicelib::CARDI(HSHVAL.as_slice(), ctx)?;
    EXPFRE = (HASHSZ - EXPUSD);

    //
    // Initialize hash.
    //
    testutil::TCASE(b"Initialize hash.", ctx)?;

    spicelib::ZZHSIINI(HASHSZ, HEDLST.as_slice_mut(), COLLST.as_slice_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"SIZE", COLLST[SIZIDX], b"=", HASHSZ, 0, OK, ctx)?;
    testutil::CHCKSI(b"FIRST", COLLST[FREIDX], b"=", 1, 0, OK, ctx)?;
    {
        let m1__: i32 = 1;
        let m2__: i32 = HASHSZ;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            testutil::CHCKSI(b"HEDLST(I)", HEDLST[I], b"=", 0, 0, OK, ctx)?;
            I += m3__;
        }
    }

    //
    // Populate hash.
    //
    testutil::TCASE(b"Populate hash.", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = HASHSZ;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZHSIADD(
                HEDLST.as_slice_mut(),
                COLLST.as_slice_mut(),
                ITEMS.as_slice_mut(),
                TITEMS[I],
                &mut ITEMAT,
                &mut NEW,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"NEW", NEW, true, OK, ctx)?;
            testutil::CHCKSI(b"ITEMAT", ITEMAT, b"=", I, 0, OK, ctx)?;
            testutil::CHCKSI(b"ITEMS(ITEMAT)", ITEMS[ITEMAT], b"=", TITEMS[I], 0, OK, ctx)?;
            testutil::CHCKSI(b"FIRST", COLLST[FREIDX], b"=", (I + 1), 0, OK, ctx)?;

            if (ITEMAT == I) {
                PIDS[ITEMAT] = IDS[I];
            }

            spicelib::ZZHSIAVL(COLLST.as_slice(), &mut AVAIL);
            testutil::CHCKSI(b"AVAIL", AVAIL, b"=", (HASHSZ - I), 0, OK, ctx)?;

            I += m3__;
        }
    }

    //
    // Get information about hash.
    //
    testutil::TCASE(b"Get info about the hash.", ctx)?;

    spicelib::ZZHSIINF(
        HEDLST.as_slice(),
        COLLST.as_slice(),
        ITEMS.as_slice(),
        b"HASH SIZE",
        &mut I,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"HASH SIZE", I, b"=", HASHSZ, 0, OK, ctx)?;

    spicelib::ZZHSIINF(
        HEDLST.as_slice(),
        COLLST.as_slice(),
        ITEMS.as_slice(),
        b"USED HEADNODE COUNT",
        &mut I,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"USED HEADNODE COUNT", I, b"=", EXPUSD, 0, OK, ctx)?;

    spicelib::ZZHSIINF(
        HEDLST.as_slice(),
        COLLST.as_slice(),
        ITEMS.as_slice(),
        b"UNUSED HEADNODE COUNT",
        &mut I,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"UNUSED HEADNODE COUNT", I, b"=", EXPFRE, 0, OK, ctx)?;

    spicelib::ZZHSIINF(
        HEDLST.as_slice(),
        COLLST.as_slice(),
        ITEMS.as_slice(),
        b"USED ITEM COUNT",
        &mut I,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"USED ITEM COUNT", I, b"=", HASHSZ, 0, OK, ctx)?;

    spicelib::ZZHSIINF(
        HEDLST.as_slice(),
        COLLST.as_slice(),
        ITEMS.as_slice(),
        b"UNUSED ITEM COUNT",
        &mut I,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"UNUSED ITEM COUNT", I, b"=", 0, 0, OK, ctx)?;

    //
    // Note that the expected value in the LONGEST LIST SIZE check below
    // depend on IDs in the built-in name/ID mapping, which is likely to
    // change in each new toolkit version. Such change may or may not
    // result in a different expected value. If it does, it's not an
    // error but it means that this check must be updated.
    //
    spicelib::ZZHSIINF(
        HEDLST.as_slice(),
        COLLST.as_slice(),
        ITEMS.as_slice(),
        b"LONGEST LIST SIZE",
        &mut I,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"LONGEST LIST SIZE", I, b"=", 4, 0, OK, ctx)?;

    spicelib::ZZHSIINF(
        HEDLST.as_slice(),
        COLLST.as_slice(),
        ITEMS.as_slice(),
        b"Hakuna-Matata!",
        &mut I,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ITEMNOTRECOGNIZED)", OK, ctx)?;
    testutil::CHCKSI(b" ", I, b"=", 0, 0, OK, ctx)?;

    //
    // Add the same items again.
    //
    testutil::TCASE(b"Add the same items again.", ctx)?;

    {
        let m1__: i32 = HASHSZ;
        let m2__: i32 = 1;
        let m3__: i32 = -1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZHSIADD(
                HEDLST.as_slice_mut(),
                COLLST.as_slice_mut(),
                ITEMS.as_slice_mut(),
                TITEMS[I],
                &mut ITEMAT,
                &mut NEW,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"NEW", NEW, false, OK, ctx)?;
            testutil::CHCKSI(b"ITEMAT", ITEMAT, b"=", I, 0, OK, ctx)?;
            testutil::CHCKSI(b"ITEMS(ITEMAT)", ITEMS[ITEMAT], b"=", TITEMS[I], 0, OK, ctx)?;
            testutil::CHCKSI(b"FIRST", COLLST[FREIDX], b"=", (HASHSZ + 1), 0, OK, ctx)?;

            if (ITEMAT == I) {
                testutil::CHCKSI(b"PIDS(ITEMAT)", PIDS[ITEMAT], b"=", IDS[I], 0, OK, ctx)?;
            }

            spicelib::ZZHSIAVL(COLLST.as_slice(), &mut AVAIL);
            testutil::CHCKSI(b"AVAIL", AVAIL, b"=", 0, 0, OK, ctx)?;

            I += m3__;
        }
    }

    //
    // Check that items are in the hash.
    //
    testutil::TCASE(b"Check that items are in the hash.", ctx)?;

    {
        let m1__: i32 = HASHSZ;
        let m2__: i32 = 1;
        let m3__: i32 = -1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZHSICHK(
                HEDLST.as_slice(),
                COLLST.as_slice(),
                ITEMS.as_slice(),
                TITEMS[I],
                &mut ITEMAT,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSI(b"ITEMAT", ITEMAT, b"=", I, 0, OK, ctx)?;
            testutil::CHCKSI(b"ITEMS(ITEMAT)", ITEMS[ITEMAT], b"=", TITEMS[I], 0, OK, ctx)?;
            testutil::CHCKSI(b"FIRST", COLLST[FREIDX], b"=", (HASHSZ + 1), 0, OK, ctx)?;

            if (ITEMAT == I) {
                testutil::CHCKSI(b"PIDS(ITEMAT)", PIDS[ITEMAT], b"=", IDS[I], 0, OK, ctx)?;
            }

            I += m3__;
        }
    }

    //
    // Check that non-hashed items are not in the hash.
    //
    testutil::TCASE(b"Check that other items are not in the hash.", ctx)?;

    {
        let m1__: i32 = HASHSZ;
        let m2__: i32 = 1;
        let m3__: i32 = -1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            HINT = (TITEMS[HASHSZ] + (spicelib::INTMAX() / 2));

            spicelib::ZZHSICHK(
                HEDLST.as_slice(),
                COLLST.as_slice(),
                ITEMS.as_slice(),
                HINT,
                &mut ITEMAT,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSI(b"ITEMAT", ITEMAT, b"=", 0, 0, OK, ctx)?;

            I += m3__;
        }
    }

    //
    // Exception: try to add one more item to the hash.
    //
    testutil::TCASE(b"Exception: hash overflow.", ctx)?;

    spicelib::ZZHSIADD(
        HEDLST.as_slice_mut(),
        COLLST.as_slice_mut(),
        ITEMS.as_slice_mut(),
        HINT,
        &mut ITEMAT,
        &mut NEW,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(HASHISFULL)", OK, ctx)?;
    testutil::CHCKSI(b"FIRST", COLLST[FREIDX], b"=", (HASHSZ + 1), 0, OK, ctx)?;
    testutil::CHCKSI(b"ITEMAT", ITEMAT, b"=", 0, 0, OK, ctx)?;

    //
    // Exception: reset hash with negative count.
    //
    testutil::TCASE(b"Exception: reset with non-positive size.", ctx)?;

    spicelib::ZZHSIINI(-1, HEDLST.as_slice_mut(), COLLST.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIVISOR)", OK, ctx)?;
    testutil::CHCKSI(b"SIZE", COLLST[SIZIDX], b"=", HASHSZ, 0, OK, ctx)?;
    testutil::CHCKSI(b"FIRST", COLLST[FREIDX], b"=", (HASHSZ + 1), 0, OK, ctx)?;

    //
    // Reset hash.
    //
    testutil::TCASE(b"Reset hash.", ctx)?;

    spicelib::ZZHSIINI(
        (HASHSZ - 1),
        HEDLST.as_slice_mut(),
        COLLST.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"SIZE", COLLST[SIZIDX], b"=", (HASHSZ - 1), 0, OK, ctx)?;
    testutil::CHCKSI(b"FIRST", COLLST[FREIDX], b"=", 1, 0, OK, ctx)?;
    {
        let m1__: i32 = 1;
        let m2__: i32 = (HASHSZ - 1);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            testutil::CHCKSI(b"HEDLST(I)", HEDLST[I], b"=", 0, 0, OK, ctx)?;
            I += m3__;
        }
    }

    spicelib::ZZHSIAVL(COLLST.as_slice(), &mut AVAIL);
    testutil::CHCKSI(b"AVAIL", AVAIL, b"=", (HASHSZ - 1), 0, OK, ctx)?;

    //
    // Check that items are not in the empty hash.
    //
    testutil::TCASE(b"Check that items are not in empty hash.", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = HASHSZ;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZHSICHK(
                HEDLST.as_slice(),
                COLLST.as_slice(),
                ITEMS.as_slice(),
                TITEMS[I],
                &mut ITEMAT,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSI(b"ITEMAT", ITEMAT, b"=", 0, 0, OK, ctx)?;

            I += m3__;
        }
    }

    //
    // Timing test.
    //
    testutil::TCASE(b"Timing test.", ctx)?;

    T_ELAPSD(false, b"no report on first call", b"total", ctx)?;

    spicelib::ZZHSIINI(HASHSZ, HEDLST.as_slice_mut(), COLLST.as_slice_mut(), ctx)?;

    T_ELAPSD(REPORT, b"initializing hash", b"running", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = HASHSZ;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZHSIADD(
                HEDLST.as_slice_mut(),
                COLLST.as_slice_mut(),
                ITEMS.as_slice_mut(),
                TITEMS[I],
                &mut ITEMAT,
                &mut NEW,
                ctx,
            )?;
            I += m3__;
        }
    }

    T_ELAPSD(REPORT, b"populating hash", b"running", ctx)?;

    {
        let m1__: i32 = HASHSZ;
        let m2__: i32 = 1;
        let m3__: i32 = -1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZHSIADD(
                HEDLST.as_slice_mut(),
                COLLST.as_slice_mut(),
                ITEMS.as_slice_mut(),
                TITEMS[I],
                &mut ITEMAT,
                &mut NEW,
                ctx,
            )?;
            I += m3__;
        }
    }

    T_ELAPSD(REPORT, b"adding same items to hash", b"running", ctx)?;

    {
        let m1__: i32 = HASHSZ;
        let m2__: i32 = 1;
        let m3__: i32 = -1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZHSICHK(
                HEDLST.as_slice(),
                COLLST.as_slice(),
                ITEMS.as_slice(),
                TITEMS[I],
                &mut ITEMAT,
                ctx,
            )?;
            I += m3__;
        }
    }

    T_ELAPSD(REPORT, b"checking presence in hash", b"running", ctx)?;

    {
        let m1__: i32 = HASHSZ;
        let m2__: i32 = 1;
        let m3__: i32 = -1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            HINT = (TITEMS[I] + TITEMS[HASHSZ]);
            spicelib::ZZHSICHK(
                HEDLST.as_slice(),
                COLLST.as_slice(),
                ITEMS.as_slice(),
                HINT,
                &mut ITEMAT,
                ctx,
            )?;
            I += m3__;
        }
    }

    T_ELAPSD(REPORT, b"checking items not in hash", b"running", ctx)?;

    spicelib::ZZHSIINI(
        (HASHSZ - 1),
        HEDLST.as_slice_mut(),
        COLLST.as_slice_mut(),
        ctx,
    )?;

    T_ELAPSD(REPORT, b"resetting hash", b"running", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = HASHSZ;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZHSICHK(
                HEDLST.as_slice(),
                COLLST.as_slice(),
                ITEMS.as_slice(),
                TITEMS[I],
                &mut ITEMAT,
                ctx,
            )?;

            I += m3__;
        }
    }

    T_ELAPSD(REPORT, b"checking items in empty hash", b"running", ctx)?;

    //
    // This is good enough for now.
    //

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
