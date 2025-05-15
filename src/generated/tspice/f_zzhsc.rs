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
const WDSIZE: i32 = 32;
const SIZIDX: i32 = 0;
const FREIDX: i32 = -1;
const LBPOOL: i32 = -5;
const LBCELL: i32 = -5;
const HASHSZ: i32 = 5003;

//$Procedure F_ZZHSC ( Family of tests for ZZHSC )
pub fn F_ZZHSC(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut HEDLST = ActualArray::<i32>::new(1..=HASHSZ);
    let mut COLLST = ActualArray::<i32>::new(LBPOOL..=HASHSZ);
    let mut ITEMS = ActualCharArray::new(WDSIZE, 1..=HASHSZ);
    let mut ITEMAT: i32 = 0;
    let mut NEW: bool = false;
    let mut TITEMS = ActualCharArray::new(WDSIZE, 1..=HASHSZ);
    let mut IDS = ActualArray::<i32>::new(1..=HASHSZ);
    let mut PIDS = ActualArray::<i32>::new(1..=HASHSZ);
    let mut HSHVAL = ActualArray::<i32>::new(LBCELL..=HASHSZ);
    let mut EXPUSD: i32 = 0;
    let mut EXPFRE: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut K: i32 = 0;
    let mut HWORD = [b' '; WDSIZE as usize];
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
    // INTEGER               INTMAX

    //
    // Local parameters
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
    testutil::TOPEN(b"F_ZZHSC", ctx)?;

    //
    // Initialize items and IDs -- random values.
    //
    //  SEED = -1
    //
    //  DO I = 1, HASHSZ
    //
    //     IDS(I) = I * 1000
    //
    //     TITEMS(I) = ' '
    //
    //     DO J = 1, NINT( T_RANDD( 2.D0, 20.D0, SEED ) )
    //
    //        K = NINT( T_RANDD( 2.D0, 8.D0, SEED ) )
    //
    //        CALL SUFFIX( CHAR( MOD( I*K, 94 ) + 33 ), 0, TITEMS(I) )
    //
    //     END DO
    //
    //     HWORD = '_#'
    //     CALL REPMI ( HWORD, '#', I, HWORD )
    //     CALL SUFFIX( HWORD, 0, TITEMS(I) )
    //
    //  END DO

    //
    // Initialize items and IDs -- NAIF name like values.
    //
    // First save all names built into the toolkit and a few
    // names based on each of them.
    //
    spicelib::ZZIDMAP(BLTCOD.as_slice_mut(), BLTNAM.as_arg_mut());

    J = 1;
    {
        let m1__: i32 = 1;
        let m2__: i32 = NPERM;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if ((BLTCOD[I] > 0) && (J <= (HASHSZ - 9))) {
                spicelib::LJUCRS(0, &BLTNAM[I], &mut TITEMS[J], ctx);
                spicelib::REPMC(b"IAU_#", b"#", &TITEMS[J].to_vec(), &mut TITEMS[(J + 1)]);
                spicelib::REPMC(b"#_FIXED", b"#", &TITEMS[J].to_vec(), &mut TITEMS[(J + 2)]);
                spicelib::REPMC(b"#_TOPO", b"#", &TITEMS[J].to_vec(), &mut TITEMS[(J + 3)]);
                spicelib::REPMC(b"#_1_SITE", b"#", &TITEMS[J].to_vec(), &mut TITEMS[(J + 4)]);
                spicelib::REPMC(b"#_2_SITE", b"#", &TITEMS[J].to_vec(), &mut TITEMS[(J + 5)]);
                spicelib::REPMC(b"#_3_SITE", b"#", &TITEMS[J].to_vec(), &mut TITEMS[(J + 6)]);
                spicelib::REPMC(b"#_ASITE", b"#", &TITEMS[J].to_vec(), &mut TITEMS[(J + 7)]);
                spicelib::REPMC(b"#_BSITE", b"#", &TITEMS[J].to_vec(), &mut TITEMS[(J + 8)]);
                spicelib::REPMC(b"#_CSITE", b"#", &TITEMS[J].to_vec(), &mut TITEMS[(J + 9)]);
                J = (J + 10);
            }
            if ((BLTCOD[I] < 0) && (J <= (HASHSZ - 39))) {
                spicelib::LJUCRS(0, &BLTNAM[I], &mut TITEMS[J], ctx);
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = 9;
                    let m3__: i32 = 1;
                    K = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        spicelib::REPMC(
                            &fstr::concat(
                                &fstr::concat(
                                    &fstr::concat(
                                        b"#_",
                                        &intrinsics::CHAR(((intrinsics::ICHAR(b"A") + K) - 1)),
                                    ),
                                    b"_",
                                ),
                                b"CAM",
                            ),
                            b"#",
                            &TITEMS[J].to_vec(),
                            &mut TITEMS[(J + K)],
                        );
                        K += m3__;
                    }
                }
                {
                    let m1__: i32 = 10;
                    let m2__: i32 = 19;
                    let m3__: i32 = 1;
                    K = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        spicelib::REPMC(
                            &fstr::concat(
                                &fstr::concat(
                                    &fstr::concat(
                                        b"#_",
                                        &intrinsics::CHAR(((intrinsics::ICHAR(b"A") + K) - 1)),
                                    ),
                                    b"_",
                                ),
                                b"SPE",
                            ),
                            b"#",
                            &TITEMS[J].to_vec(),
                            &mut TITEMS[(J + K)],
                        );
                        K += m3__;
                    }
                }
                {
                    let m1__: i32 = 20;
                    let m2__: i32 = 29;
                    let m3__: i32 = 1;
                    K = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        spicelib::REPMC(
                            &fstr::concat(
                                &fstr::concat(
                                    &fstr::concat(
                                        b"#_",
                                        &intrinsics::CHAR(((intrinsics::ICHAR(b"A") + K) - 20)),
                                    ),
                                    b"_",
                                ),
                                b"DETECT",
                            ),
                            b"#",
                            &TITEMS[J].to_vec(),
                            &mut TITEMS[(J + K)],
                        );
                        K += m3__;
                    }
                }
                {
                    let m1__: i32 = 30;
                    let m2__: i32 = 39;
                    let m3__: i32 = 1;
                    K = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        spicelib::REPMC(
                            &fstr::concat(
                                &fstr::concat(
                                    &fstr::concat(
                                        b"#_",
                                        &intrinsics::CHAR(((intrinsics::ICHAR(b"A") + K) - 20)),
                                    ),
                                    b"_",
                                ),
                                b"SENSOR",
                            ),
                            b"#",
                            &TITEMS[J].to_vec(),
                            &mut TITEMS[(J + K)],
                        );
                        K += m3__;
                    }
                }
                J = (J + 40);
            }
            I += m3__;
        }
    }

    //
    // Eliminate duplicate values.
    //
    K = (J - 1);
    spicelib::RMDUPC(&mut K, TITEMS.as_arg_mut());
    J = (K + 1);

    //
    // Finish by adding some asteroid names at the end to fill up the
    // hash.
    //
    {
        let m1__: i32 = J;
        let m2__: i32 = HASHSZ;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::REPMI(b"ASTEROID_#", b"#", (2500000 + I), &mut TITEMS[I], ctx);
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
    // Calculate hash values for all names in the buffer and store them
    // in a set to get the expected numbers of occupied and free head
    // nodes.
    //
    spicelib::SSIZEI(HASHSZ, HSHVAL.as_slice_mut(), ctx)?;
    {
        let m1__: i32 = 1;
        let m2__: i32 = HASHSZ;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            J = spicelib::ZZHASH2(&TITEMS[I], HASHSZ, ctx)?;
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

    spicelib::ZZHSCINI(HASHSZ, HEDLST.as_slice_mut(), COLLST.as_slice_mut(), ctx)?;

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
            spicelib::ZZHSCADD(
                HEDLST.as_slice_mut(),
                COLLST.as_slice_mut(),
                ITEMS.as_arg_mut(),
                &TITEMS[I],
                &mut ITEMAT,
                &mut NEW,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"NEW", NEW, true, OK, ctx)?;
            testutil::CHCKSI(b"ITEMAT", ITEMAT, b"=", I, 0, OK, ctx)?;
            testutil::CHCKSC(b"ITEMS(ITEMAT)", &ITEMS[ITEMAT], b"=", &TITEMS[I], OK, ctx)?;
            testutil::CHCKSI(b"FIRST", COLLST[FREIDX], b"=", (I + 1), 0, OK, ctx)?;

            if (ITEMAT == I) {
                PIDS[ITEMAT] = IDS[I];
            }

            spicelib::ZZHSCAVL(COLLST.as_slice(), &mut AVAIL);
            testutil::CHCKSI(b"AVAIL", AVAIL, b"=", (HASHSZ - I), 0, OK, ctx)?;

            I += m3__;
        }
    }

    //
    // Get information about hash.
    //
    testutil::TCASE(b"Get info about the hash.", ctx)?;

    spicelib::ZZHSCINF(
        HEDLST.as_slice(),
        COLLST.as_slice(),
        ITEMS.as_arg(),
        b"HASH SIZE",
        &mut I,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"HASH SIZE", I, b"=", HASHSZ, 0, OK, ctx)?;

    spicelib::ZZHSCINF(
        HEDLST.as_slice(),
        COLLST.as_slice(),
        ITEMS.as_arg(),
        b"USED HEADNODE COUNT",
        &mut I,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"USED HEADNODE COUNT", I, b"=", EXPUSD, 0, OK, ctx)?;

    spicelib::ZZHSCINF(
        HEDLST.as_slice(),
        COLLST.as_slice(),
        ITEMS.as_arg(),
        b"UNUSED HEADNODE COUNT",
        &mut I,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"UNUSED HEADNODE COUNT", I, b"=", EXPFRE, 0, OK, ctx)?;

    spicelib::ZZHSCINF(
        HEDLST.as_slice(),
        COLLST.as_slice(),
        ITEMS.as_arg(),
        b"USED ITEM COUNT",
        &mut I,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"USED ITEM COUNT", I, b"=", HASHSZ, 0, OK, ctx)?;

    spicelib::ZZHSCINF(
        HEDLST.as_slice(),
        COLLST.as_slice(),
        ITEMS.as_arg(),
        b"UNUSED ITEM COUNT",
        &mut I,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"UNUSED ITEM COUNT", I, b"=", 0, 0, OK, ctx)?;

    spicelib::ZZHSCINF(
        HEDLST.as_slice(),
        COLLST.as_slice(),
        ITEMS.as_arg(),
        b"LONGEST LIST SIZE",
        &mut I,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"LONGEST LIST SIZE", I, b"=", 6, 0, OK, ctx)?;

    spicelib::ZZHSCINF(
        HEDLST.as_slice(),
        COLLST.as_slice(),
        ITEMS.as_arg(),
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
            spicelib::ZZHSCADD(
                HEDLST.as_slice_mut(),
                COLLST.as_slice_mut(),
                ITEMS.as_arg_mut(),
                &TITEMS[I],
                &mut ITEMAT,
                &mut NEW,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"NEW", NEW, false, OK, ctx)?;
            testutil::CHCKSI(b"ITEMAT", ITEMAT, b"=", I, 0, OK, ctx)?;
            testutil::CHCKSC(b"ITEMS(ITEMAT)", &ITEMS[ITEMAT], b"=", &TITEMS[I], OK, ctx)?;
            testutil::CHCKSI(b"FIRST", COLLST[FREIDX], b"=", (HASHSZ + 1), 0, OK, ctx)?;

            if (ITEMAT == I) {
                testutil::CHCKSI(b"PIDS(ITEMAT)", PIDS[ITEMAT], b"=", IDS[I], 0, OK, ctx)?;
            }

            spicelib::ZZHSCAVL(COLLST.as_slice(), &mut AVAIL);
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
            spicelib::ZZHSCCHK(
                HEDLST.as_slice(),
                COLLST.as_slice(),
                ITEMS.as_arg(),
                &TITEMS[I],
                &mut ITEMAT,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSI(b"ITEMAT", ITEMAT, b"=", I, 0, OK, ctx)?;
            testutil::CHCKSC(b"ITEMS(ITEMAT)", &ITEMS[ITEMAT], b"=", &TITEMS[I], OK, ctx)?;
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
            fstr::assign(&mut HWORD, TITEMS.get(I));

            spicelib::PREFIX(b"z", 0, &mut HWORD);

            spicelib::ZZHSCCHK(
                HEDLST.as_slice(),
                COLLST.as_slice(),
                ITEMS.as_arg(),
                &HWORD,
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

    spicelib::ZZHSCADD(
        HEDLST.as_slice_mut(),
        COLLST.as_slice_mut(),
        ITEMS.as_arg_mut(),
        &HWORD,
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

    spicelib::ZZHSCINI(-1, HEDLST.as_slice_mut(), COLLST.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIVISOR)", OK, ctx)?;
    testutil::CHCKSI(b"SIZE", COLLST[SIZIDX], b"=", HASHSZ, 0, OK, ctx)?;
    testutil::CHCKSI(b"FIRST", COLLST[FREIDX], b"=", (HASHSZ + 1), 0, OK, ctx)?;

    //
    // This case worked prior to N0067 update to ZZHSCINI as it
    // was trapped by ZZHASH2 before an attempt to assign hash
    // elements beywond its size was made by the initialization loop,
    // causing a seg fault. It was disabled for N0067.
    //
    // Exception: reset hash with very big count.
    //
    //  CALL TCASE ( 'Exception: reset with too large size.' )
    //
    //  CALL ZZHSCINI ( INTMAX(), HEDLST, COLLST )
    //
    //  CALL CHCKXC ( .TRUE., 'SPICE(INVALIDDIVISOR)', OK )
    //  CALL CHCKSI ( 'SIZE',  COLLST(SIZIDX), '=', HASHSZ, 0, OK )
    //  CALL CHCKSI ( 'FIRST', COLLST(FREIDX), '=', HASHSZ+1, 0, OK )

    //
    // Reset hash.
    //
    testutil::TCASE(b"Reset hash.", ctx)?;

    spicelib::ZZHSCINI(
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

    spicelib::ZZHSCAVL(COLLST.as_slice(), &mut AVAIL);
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
            spicelib::ZZHSCCHK(
                HEDLST.as_slice(),
                COLLST.as_slice(),
                ITEMS.as_arg(),
                &TITEMS[I],
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

    T_ELAPSD(false, b"no report on first call", b"running", ctx)?;

    spicelib::ZZHSCINI(HASHSZ, HEDLST.as_slice_mut(), COLLST.as_slice_mut(), ctx)?;

    T_ELAPSD(REPORT, b"initializing hash", b"running", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = HASHSZ;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZHSCADD(
                HEDLST.as_slice_mut(),
                COLLST.as_slice_mut(),
                ITEMS.as_arg_mut(),
                &TITEMS[I],
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
            spicelib::ZZHSCADD(
                HEDLST.as_slice_mut(),
                COLLST.as_slice_mut(),
                ITEMS.as_arg_mut(),
                &TITEMS[I],
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
            spicelib::ZZHSCCHK(
                HEDLST.as_slice(),
                COLLST.as_slice(),
                ITEMS.as_arg(),
                &TITEMS[I],
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
            fstr::assign(&mut HWORD, TITEMS.get(I));
            spicelib::SUFFIX(b"z", 0, &mut HWORD);
            spicelib::ZZHSCCHK(
                HEDLST.as_slice(),
                COLLST.as_slice(),
                ITEMS.as_arg(),
                &HWORD,
                &mut ITEMAT,
                ctx,
            )?;
            I += m3__;
        }
    }

    T_ELAPSD(REPORT, b"checking items not in hash", b"running", ctx)?;

    spicelib::ZZHSCINI(
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
            spicelib::ZZHSCCHK(
                HEDLST.as_slice(),
                COLLST.as_slice(),
                ITEMS.as_arg(),
                &TITEMS[I],
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
