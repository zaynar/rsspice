//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SCHCOD: i32 = 2;
const FILSIZ: i32 = 255;
const FTSIZE: i32 = 5000;
const MAXCLU: i32 = 1000;
const MAXVAL: i32 = 12;
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const FILEX: i32 = 100000;
const RECX: i32 = 1000;

struct SaveVars {
    CDATA: Vec<u8>,
    DAS: ActualCharArray,
    XCDATA: Vec<u8>,
    DDATA: StackArray<f64, 12>,
    XDDATA: StackArray<f64, 12>,
    CLNWDS: ActualArray<i32>,
    CLTYPS: ActualArray<i32>,
    HANDLE: ActualArray<i32>,
    IDATA: StackArray<i32, 12>,
    NCLUST: i32,
    NCOMRC: i32,
    NFILES: i32,
    NSCHEM: i32,
    RECNO: i32,
    XIDATA: StackArray<i32, 12>,
    FCLOSE: bool,
    SGRGAT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CDATA = vec![b' '; MAXVAL as usize];
        let mut DAS = ActualCharArray::new(FILSIZ, 1..=FTSIZE);
        let mut XCDATA = vec![b' '; MAXVAL as usize];
        let mut DDATA = StackArray::<f64, 12>::new(1..=MAXVAL);
        let mut XDDATA = StackArray::<f64, 12>::new(1..=MAXVAL);
        let mut CLNWDS = ActualArray::<i32>::new(1..=MAXCLU);
        let mut CLTYPS = ActualArray::<i32>::new(1..=MAXCLU);
        let mut HANDLE = ActualArray::<i32>::new(1..=FTSIZE);
        let mut IDATA = StackArray::<i32, 12>::new(1..=MAXVAL);
        let mut NCLUST: i32 = 0;
        let mut NCOMRC: i32 = 0;
        let mut NFILES: i32 = 0;
        let mut NSCHEM: i32 = 0;
        let mut RECNO: i32 = 0;
        let mut XIDATA = StackArray::<i32, 12>::new(1..=MAXVAL);
        let mut FCLOSE: bool = false;
        let mut SGRGAT: bool = false;

        Self {
            CDATA,
            DAS,
            XCDATA,
            DDATA,
            XDDATA,
            CLNWDS,
            CLTYPS,
            HANDLE,
            IDATA,
            NCLUST,
            NCOMRC,
            NFILES,
            NSCHEM,
            RECNO,
            XIDATA,
            FCLOSE,
            SGRGAT,
        }
    }
}

//$Procedure F_DASMUL ( DAS test, multiple DAS files )
pub fn F_DASMUL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved all.
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_DASMUL", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create DAS files.", ctx)?;

    save.NFILES = FTSIZE;

    save.NFILES = 5000;

    save.NCOMRC = 2;

    save.NCLUST = 3;

    save.CLTYPS[1] = DP;
    save.CLTYPS[2] = CHR;
    save.CLTYPS[3] = INT;

    save.CLNWDS[1] = 5;
    save.CLNWDS[2] = 7;
    save.CLNWDS[3] = 12;

    save.SGRGAT = false;
    save.FCLOSE = true;
    save.NSCHEM = SCHCOD;

    for I in 1..=save.NFILES {
        fstr::assign(save.DAS.get_mut(I), b"mult_test_#.das");
        spicelib::REPMI(&save.DAS[I].to_vec(), b"#", I, &mut save.DAS[I], ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if spicelib::EXISTS(&save.DAS[I], ctx)? {
            spicelib::DELFIL(&save.DAS[I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        TSTDAS(
            &save.DAS[I],
            b"DSK",
            save.NCOMRC,
            I,
            save.NCLUST,
            save.CLTYPS.as_slice(),
            save.CLNWDS.as_slice(),
            save.SGRGAT,
            save.FCLOSE,
            save.NSCHEM,
            &mut save.HANDLE[I],
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NFILES {
        spicelib::DASOPR(&save.DAS[I], &mut save.HANDLE[I], ctx)?;

        // CALL FURNSH ( DAS(I) )
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        // CALL KINFO ( DAS(I), FILTYP, SOURCE, HANDLE(I), FOUND )
        // CALL CHCKXC ( .FALSE., ' ', OK )
        // CALL CHCKSL ( 'FOUND', FOUND, .TRUE., OK )
    }

    //
    // Read data from open files.
    //
    for I in 1..=save.NFILES {
        for J in 1..=save.NCLUST {
            if (save.CLTYPS[J] == INT) {
                spicelib::DASRDI(
                    save.HANDLE[I],
                    1,
                    save.CLNWDS[J],
                    save.IDATA.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.RECNO = 3;

                for K in 1..=save.CLNWDS[J] {
                    save.XIDATA[K] = (((I * FILEX) + (RECX * save.RECNO)) + K);
                }

                testutil::CHCKAI(
                    b"IDATA",
                    save.IDATA.as_slice(),
                    b"=",
                    save.XIDATA.as_slice(),
                    save.CLNWDS[J],
                    OK,
                    ctx,
                )?;
            } else if (save.CLTYPS[J] == DP) {
                spicelib::DASRDD(
                    save.HANDLE[I],
                    1,
                    save.CLNWDS[J],
                    save.DDATA.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.RECNO = 1;

                for K in 1..=save.CLNWDS[J] {
                    save.XDDATA[K] = ((((I * FILEX) + (RECX * save.RECNO)) as f64) + (K as f64));
                }

                testutil::CHCKAD(
                    b"DDATA",
                    save.DDATA.as_slice(),
                    b"=",
                    save.XDDATA.as_slice(),
                    save.CLNWDS[J],
                    0.0,
                    OK,
                    ctx,
                )?;
            } else {
                spicelib::DASRDC(
                    save.HANDLE[I],
                    1,
                    save.CLNWDS[J],
                    1,
                    MAXVAL,
                    CharArrayMut::from_mut(&mut save.CDATA),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.RECNO = 2;

                for K in 1..=save.CLNWDS[J] {
                    save.XIDATA[K] = (((I * FILEX) + (RECX * save.RECNO)) + K);

                    fstr::assign(
                        fstr::substr_mut(&mut save.XCDATA, K..=K),
                        &intrinsics::CHAR(intrinsics::MOD(save.XIDATA[K], 128)),
                    );
                }

                testutil::CHCKSC(b"CDATA", &save.CDATA, b"=", &save.XCDATA, OK, ctx)?;
            }
        }
    }

    for I in 1..=save.NFILES {
        spicelib::DASCLS(save.HANDLE[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up: delete DAS files.", ctx)?;

    for I in 1..=save.NFILES {
        spicelib::DELFIL(&save.DAS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }
    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
