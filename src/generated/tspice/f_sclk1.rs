//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NDELIM: i32 = 5;
const DELIMS: &[u8; NDELIM as usize] = &fstr::extend_const::<{ NDELIM as usize }>(b".:-, ");
const MXPART: i32 = 9999;
const PARTLN: i32 = 5;
const MXCOEF: i32 = 100000;
const MXNFLD: i32 = 10;
const DPLEN: i32 = 30;
const MXTSYS: i32 = 2;
const CK: &[u8] = b"test.bc";
const DELIM: &[u8] = b":";
const SCLK: &[u8] = b"test.tsc";
const SCLK2: &[u8] = b"test2.tsc";
const SCLK3: &[u8] = b"test3.tsc";
const TIGHT: f64 = 0.000000000001;
const LNSIZE: i32 = 80;
const TIMLEN: i32 = 40;
const LBCELL: i32 = -5;
const NCOM: i32 = 3;

struct SaveVars {
    COMS: ActualCharArray,
    CLKST1: Vec<u8>,
    CLKST2: Vec<u8>,
    CLKST3: Vec<u8>,
    XCLKS1: Vec<u8>,
    XCLKS2: Vec<u8>,
    XCLKS3: Vec<u8>,
    BEGTIM: f64,
    CLKDP1: f64,
    CLKDP2: f64,
    CLKDP3: f64,
    COEFFS: ActualArray2D<f64>,
    DELTA: f64,
    ENDTIM: f64,
    ET1: f64,
    ET2: f64,
    ET3: f64,
    FRAC: f64,
    MAJOR: f64,
    MODULI: StackArray<f64, 10>,
    OFFSET: StackArray<f64, 10>,
    PSTAR1: ActualArray<f64>,
    PSTAR2: ActualArray<f64>,
    PSTAR3: ActualArray<f64>,
    PSTOP1: ActualArray<f64>,
    PSTOP2: ActualArray<f64>,
    PSTOP3: ActualArray<f64>,
    TDELT0: f64,
    TDELT1: f64,
    TDT: f64,
    TOL: f64,
    TVEC: StackArray<f64, 6>,
    XCLKDP: f64,
    XET1: f64,
    XET2: f64,
    XET3: f64,
    XSTART: ActualArray<f64>,
    XSTOP: ActualArray<f64>,
    CLKID: i32,
    HANDLE: i32,
    NCOEFF: i32,
    NFIELD: i32,
    NITR: i32,
    NPART1: i32,
    NPART2: i32,
    NPART3: i32,
    NPARTS: i32,
    XNPART: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut COMS = ActualCharArray::new(LNSIZE, LBCELL..=NCOM);
        let mut CLKST1 = vec![b' '; TIMLEN as usize];
        let mut CLKST2 = vec![b' '; TIMLEN as usize];
        let mut CLKST3 = vec![b' '; TIMLEN as usize];
        let mut XCLKS1 = vec![b' '; TIMLEN as usize];
        let mut XCLKS2 = vec![b' '; TIMLEN as usize];
        let mut XCLKS3 = vec![b' '; TIMLEN as usize];
        let mut BEGTIM: f64 = 0.0;
        let mut CLKDP1: f64 = 0.0;
        let mut CLKDP2: f64 = 0.0;
        let mut CLKDP3: f64 = 0.0;
        let mut COEFFS = ActualArray2D::<f64>::new(1..=3, 1..=(MXCOEF + 1));
        let mut DELTA: f64 = 0.0;
        let mut ENDTIM: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut ET2: f64 = 0.0;
        let mut ET3: f64 = 0.0;
        let mut FRAC: f64 = 0.0;
        let mut MAJOR: f64 = 0.0;
        let mut MODULI = StackArray::<f64, 10>::new(1..=MXNFLD);
        let mut OFFSET = StackArray::<f64, 10>::new(1..=MXNFLD);
        let mut PSTAR1 = ActualArray::<f64>::new(1..=MXPART);
        let mut PSTAR2 = ActualArray::<f64>::new(1..=MXPART);
        let mut PSTAR3 = ActualArray::<f64>::new(1..=MXPART);
        let mut PSTOP1 = ActualArray::<f64>::new(1..=MXPART);
        let mut PSTOP2 = ActualArray::<f64>::new(1..=MXPART);
        let mut PSTOP3 = ActualArray::<f64>::new(1..=MXPART);
        let mut TDELT0: f64 = 0.0;
        let mut TDELT1: f64 = 0.0;
        let mut TDT: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut TVEC = StackArray::<f64, 6>::new(1..=6);
        let mut XCLKDP: f64 = 0.0;
        let mut XET1: f64 = 0.0;
        let mut XET2: f64 = 0.0;
        let mut XET3: f64 = 0.0;
        let mut XSTART = ActualArray::<f64>::new(1..=MXPART);
        let mut XSTOP = ActualArray::<f64>::new(1..=MXPART);
        let mut CLKID: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut NCOEFF: i32 = 0;
        let mut NFIELD: i32 = 0;
        let mut NITR: i32 = 0;
        let mut NPART1: i32 = 0;
        let mut NPART2: i32 = 0;
        let mut NPART3: i32 = 0;
        let mut NPARTS: i32 = 0;
        let mut XNPART: i32 = 0;
        let mut FOUND: bool = false;

        Self {
            COMS,
            CLKST1,
            CLKST2,
            CLKST3,
            XCLKS1,
            XCLKS2,
            XCLKS3,
            BEGTIM,
            CLKDP1,
            CLKDP2,
            CLKDP3,
            COEFFS,
            DELTA,
            ENDTIM,
            ET1,
            ET2,
            ET3,
            FRAC,
            MAJOR,
            MODULI,
            OFFSET,
            PSTAR1,
            PSTAR2,
            PSTAR3,
            PSTOP1,
            PSTOP2,
            PSTOP3,
            TDELT0,
            TDELT1,
            TDT,
            TOL,
            TVEC,
            XCLKDP,
            XET1,
            XET2,
            XET3,
            XSTART,
            XSTOP,
            CLKID,
            HANDLE,
            NCOEFF,
            NFIELD,
            NITR,
            NPART1,
            NPART2,
            NPART3,
            NPARTS,
            XNPART,
            FOUND,
        }
    }
}

//$Procedure F_SCLK1 ( SCLK interleaved computation tests )
pub fn F_SCLK1(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // We use a longer, non-standard length for ABCORR because we
    // want to include embedded blanks for testing.
    //

    //
    // Saved variables
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_SCLK1", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup:  create kernels.", ctx)?;

    spicelib::KCLEAR(ctx)?;

    if spicelib::EXISTS(SCLK, ctx)? {
        spicelib::DELFIL(SCLK, ctx)?;
    }

    if spicelib::EXISTS(SCLK2, ctx)? {
        spicelib::DELFIL(SCLK2, ctx)?;
    }

    if spicelib::EXISTS(SCLK3, ctx)? {
        spicelib::DELFIL(SCLK3, ctx)?;
    }

    //
    // Leapseconds:  Note that the LSK is deleted after loading, so we
    // don't have to clean it up later.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create and load an SCLK kernel. Keep the file; we'll be
    // re-loading it later. We'll delete it at the end of
    // the routine. W
    //
    testutil::TSTCK3(CK, SCLK, false, true, true, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The following tests that use SCLK ID -9 presume the SCLK kernel
    // we just created contains the following assignments:
    //
    //    SCLK_KERNEL_ID                = ( @28-OCT-1994        )
    //
    //    SCLK_DATA_TYPE_9              = ( 1 )
    //
    //    SCLK01_TIME_SYSTEM_9          = ( 1 )
    //    SCLK01_N_FIELDS_9             = ( 2 )
    //    SCLK01_MODULI_9               = ( 1000000000     10000 )
    //    SCLK01_OFFSETS_9              = ( 0         0 )
    //    SCLK01_OUTPUT_DELIM_9         = ( 1 )
    //
    //    SCLK_PARTITION_START_9        = ( 0.0000000000000E+00 )
    //    SCLK_PARTITION_END_9          = ( 1.00000000E+14      )
    //    SCLK01_COEFFICIENTS_9         = ( 0.00000000E+00
    //    @01-JAN-1980-00:00:00.000
    //    1  )
    //
    //
    //    DELTET/DELTA_T_A       =   32.184
    //    DELTET/K               =    1.657D-3
    //    DELTET/EB              =    1.671D-2
    //    DELTET/M               = (  6.239996D0 1.99096871D-7 )
    //
    //    CK_-9999_SCLK          =   -9
    //    CK_-9999_SPK           =   -9
    //
    //    CK_-10000_SCLK         =   -9
    //    CK_-10000_SPK          =   -9
    //
    //    CK_-10001_SCLK         =   -9
    //    CK_-10001_SPK          =   -9
    //
    //
    // We're going to create a second new kernel to exercise other
    // aspects of the SCLK system. The kernel contents will be as
    // follows:

    //
    // KPL/SCLK
    //
    // \begindata
    //
    //    SCLK_KERNEL_ID                  = ( @01-JAN-2000/12:00:00 )
    //
    //    SCLK_DATA_TYPE_-99              = ( 1 )
    //
    //    SCLK01_TIME_SYSTEM_-99          = ( 1 )
    //    SCLK01_N_FIELDS_-99             = ( 4 )
    //    SCLK01_MODULI_-99               = ( 10000000 10 9 8 )
    //    SCLK01_OFFSETS_-99              = ( 0 0 0 0 )
    //    SCLK01_OUTPUT_DELIM_-99         = ( 2 )
    //
    //    SCLK_PARTITION_START_-99        = ( 1.0,
    //                                        2.0,
    //                                        3.0,
    //                                        ...
    //                                        1001.0 )
    //
    //    SCLK_PARTITION_END_-99          = ( 720001.0,
    //                                        720002.0,
    //                                        ...
    //                                        721001.0 )
    //
    //    SCLK01_COEFFICIENTS_-99         = (
    //
    //            0.0    0.0000000000000000E+00    0.2000000001000000E+01
    //       180000.0    0.5000000000000000E+03    0.2000000002000000E+01
    //       360000.0    0.1000000000000000E+04    0.2000000003000000E+01
    //                            ...
    //    720000000.0    0.2000000000000000E+07    0.2000004001000000E+01
    //
    //    )
    //
    // \begintext
    //
    // The above kernel has 1001 partitions and 4001 coefficient records.
    //
    testutil::TCASE(b"Create second SCLK kernel with 1001 partitions.", ctx)?;

    spicelib::SSIZEC(0, save.COMS.as_arg_mut(), ctx)?;

    save.CLKID = -99;

    save.NPARTS = 1001;
    save.NCOEFF = 4001;

    save.NFIELD = 4;

    save.MODULI[1] = 10000000 as f64;
    save.MODULI[2] = 10 as f64;
    save.MODULI[3] = 9 as f64;
    save.MODULI[4] = 8 as f64;

    save.MAJOR = 1.0;

    for I in 2..=4 {
        save.MAJOR = (save.MAJOR * save.MODULI[I]);
    }

    save.OFFSET[1] = 0 as f64;
    save.OFFSET[2] = 0 as f64;
    save.OFFSET[3] = 0 as f64;
    save.OFFSET[4] = 0 as f64;

    for I in 1..=save.NPARTS {
        save.XSTART[I] = I as f64;
        save.XSTOP[I] = (720000 + I) as f64;
    }

    save.DELTA = 0.000000001;

    for I in 1..=save.NCOEFF {
        save.COEFFS[[1, I]] = (((I - 1) as f64) * 180000.0);

        save.COEFFS[[2, I]] = ((save.COEFFS[[1, I]] * 2 as f64) / save.MAJOR);

        save.COEFFS[[3, I]] = (2.0 + ((I as f64) * save.DELTA));
    }

    testutil::T_WCLK01(
        SCLK2,
        save.COMS.as_arg(),
        save.CLKID,
        b"@01-JAN-2000/12:00:00",
        b"TDB",
        save.NFIELD,
        save.MODULI.as_slice(),
        save.OFFSET.as_slice(),
        DELIM,
        save.NPARTS,
        save.XSTART.as_slice(),
        save.XSTOP.as_slice(),
        save.NCOEFF,
        save.COEFFS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We're going to create a third new kernel to exercise other
    // aspects of the SCLK system. This kernel uses TDT as the parallel
    // time system. The kernel contents will be as follows:

    //
    // KPL/SCLK
    //
    // \begindata
    //
    //    SCLK_KERNEL_ID                   = ( @01-JAN-2001/12:00:00 )
    //
    //    SCLK_DATA_TYPE_-999              = ( 1 )
    //
    //    SCLK01_TIME_SYSTEM_-999          = ( 2 )
    //    SCLK01_N_FIELDS_-999             = ( 4 )
    //    SCLK01_MODULI_-999               = ( 10000000 10 9 8 )
    //    SCLK01_OFFSETS_-999              = ( 4 3 2 1 )
    //    SCLK01_OUTPUT_DELIM_-999         = ( 2 )
    //
    //    SCLK_PARTITION_START_-999        = ( 1.0,
    //                                         1000002.0,
    //                                         3.0,
    //                                         4.0  )
    //
    //    SCLK_PARTITION_END_-999          = ( 720001.0,
    //                                         1720002.0,
    //                                         720003.0,
    //                                         720004.0 )
    //
    //    SCLK01_COEFFICIENTS_-999         = (
    //
    //
    //            0.0    0.0000000000000000E+00    0.2000000001000000E+01
    //       180000.0    0.5000000000000000E+03    0.2000000002000000E+01
    //       360000.0    0.1000000000000000E+04    0.2000000003000000E+01
    //                            ...
    //      2700000.0    0.7500000000000000E+04    0.2000000016000000E+01
    //
    //    )
    //
    // \begintext
    //
    // The above kernel has 4 partitions and 16 coefficient records.
    //
    testutil::TCASE(
        b"Create third SCLK kernel with 4 partitions. Time system is TDT.",
        ctx,
    )?;

    spicelib::SSIZEC(0, save.COMS.as_arg_mut(), ctx)?;

    save.CLKID = -999;

    save.NPARTS = 4;
    save.NCOEFF = 16;

    save.NFIELD = 4;

    save.MODULI[1] = 10000000 as f64;
    save.MODULI[2] = 10 as f64;
    save.MODULI[3] = 9 as f64;
    save.MODULI[4] = 8 as f64;

    save.MAJOR = 1.0;

    for I in 2..=4 {
        save.MAJOR = (save.MAJOR * save.MODULI[I]);
    }

    save.OFFSET[1] = 4 as f64;
    save.OFFSET[2] = 3 as f64;
    save.OFFSET[3] = 2 as f64;
    save.OFFSET[4] = 1 as f64;

    for I in 1..=save.NPARTS {
        save.XSTART[I] = I as f64;
        save.XSTOP[I] = (720000 + I) as f64;
    }

    //
    // Make the second partition start with a forward jump.
    //
    save.XSTART[2] = (save.XSTART[2] + 1000000.0);
    save.XSTOP[2] = (save.XSTOP[2] + 1000000.0);

    save.DELTA = 0.000000001;

    for I in 1..=save.NCOEFF {
        save.COEFFS[[1, I]] = (((I - 1) as f64) * 180000.0);

        save.COEFFS[[2, I]] = ((save.COEFFS[[1, I]] * 2 as f64) / save.MAJOR);

        save.COEFFS[[3, I]] = (2.0 + ((I as f64) * save.DELTA));
    }

    testutil::T_WCLK01(
        SCLK3,
        save.COMS.as_arg(),
        save.CLKID,
        b"@01-JAN-2001/12:00:00",
        b"TDT",
        save.NFIELD,
        save.MODULI.as_slice(),
        save.OFFSET.as_slice(),
        DELIM,
        save.NPARTS,
        save.XSTART.as_slice(),
        save.XSTOP.as_slice(),
        save.NCOEFF,
        save.COEFFS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LDPOOL(SCLK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // ********************************************************
    //
    //
    //     End of kernel creation code
    //
    //
    // ********************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCENCD: interleave computations using three clocks; compare run time to that of non-interleaved computations.", ctx)?;

    //
    // Set SCLK strings to indicate offsets of 1 tick from clock start.
    //
    // Field offsets and non-zero partition start times account for the
    // field values used for clocks -99 and -999.
    //
    // Field offsets also account for the field widths that are larger
    // by 1 character than what would be implied by the corresponding
    // moduli. For clock -999, the ranges of valid field values are:
    //
    //      field 1 (leftmost):     4 : 10000003
    //      field 2 :               3 : 12
    //      field 3 :               2 : 10
    //      field 4 :               1 : 9
    //
    fstr::assign(&mut save.XCLKS1, b"1/000000000.0001");
    fstr::assign(&mut save.XCLKS2, b"1/0000000:0:0:2");
    fstr::assign(&mut save.XCLKS3, b"1/00000004:03:02:3");

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.NITR = 10000;

    for I in 1..=save.NITR {
        save.CLKID = -9;
        spicelib::SCENCD(save.CLKID, &save.XCLKS1, &mut save.CLKDP1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLKID = -99;
        spicelib::SCENCD(save.CLKID, &save.XCLKS2, &mut save.CLKDP2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLKID = -999;
        spicelib::SCENCD(save.CLKID, &save.XCLKS3, &mut save.CLKDP3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ENDTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.TDELT0 = (save.ENDTIM - save.BEGTIM);

    //
    // Check final SCLK values. All values should be 1 tick. Note that
    // offsets and non-zero partition start values affect interpretation
    // of the SCLK strings used here.
    //
    save.XCLKDP = 1.0;
    save.TOL = 0.0;

    testutil::CHCKSD(b"CLKDP1", save.CLKDP1, b"=", save.XCLKDP, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"CLKDP2", save.CLKDP2, b"=", save.XCLKDP, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"CLKDP3", save.CLKDP3, b"=", save.XCLKDP, save.TOL, OK, ctx)?;

    //
    // Now perform the computations without interleaving.
    //
    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    for I in 1..=save.NITR {
        save.CLKID = -9;
        spicelib::SCENCD(save.CLKID, &save.XCLKS1, &mut save.CLKDP1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NITR {
        save.CLKID = -99;
        spicelib::SCENCD(save.CLKID, &save.XCLKS2, &mut save.CLKDP2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NITR {
        save.CLKID = -999;
        spicelib::SCENCD(save.CLKID, &save.XCLKS3, &mut save.CLKDP3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ENDTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.TDELT1 = (save.ENDTIM - save.BEGTIM);

    //
    // Verify that we have no more than a 50% penalty incurred by
    // interleaving. We expect a much smaller penalty, but this
    // value should allow tests to pass on all platforms while
    // catching gross implementation errors.
    //
    // Performance tests are executed only on platforms on which
    // ZZCPUTIM can report times with fractional seconds.
    //

    if (save.TDELT1 > 0.0) {
        save.FRAC = (save.TDELT0 / save.TDELT1);

        // CALL CHCKSD ( 'FRAC', FRAC, '<', 1.5D0, 0.D0, OK )
    }

    //
    // Check final SCLK values. All values should be 1 tick. Note that
    // offsets and non-zero partition start values affect interpretation
    // of the SCLK strings used here.
    //
    save.XCLKDP = 1.0;
    save.TOL = 0.0;

    testutil::CHCKSD(b"CLKDP1", save.CLKDP1, b"=", save.XCLKDP, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"CLKDP2", save.CLKDP2, b"=", save.XCLKDP, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"CLKDP3", save.CLKDP3, b"=", save.XCLKDP, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCDECD: interleave computations using three clocks; compare run time to that of non-interleaved computations.", ctx)?;

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.NITR = 10000;

    for I in 1..=save.NITR {
        save.CLKID = -9;
        spicelib::SCDECD(save.CLKID, save.CLKDP1, &mut save.CLKST1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLKID = -99;
        spicelib::SCDECD(save.CLKID, save.CLKDP2, &mut save.CLKST2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLKID = -999;
        spicelib::SCDECD(save.CLKID, save.CLKDP3, &mut save.CLKST3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ENDTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.TDELT0 = (save.ENDTIM - save.BEGTIM);

    //
    // Check final SCLK strings.
    //
    testutil::CHCKSC(b"CLKST1", &save.CLKST1, b"=", &save.XCLKS1, OK, ctx)?;
    testutil::CHCKSC(b"CLKST2", &save.CLKST2, b"=", &save.XCLKS2, OK, ctx)?;
    testutil::CHCKSC(b"CLKST3", &save.CLKST3, b"=", &save.XCLKS3, OK, ctx)?;

    //
    // Now perform the computations without interleaving.
    //
    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    for I in 1..=save.NITR {
        save.CLKID = -9;
        spicelib::SCDECD(save.CLKID, save.CLKDP1, &mut save.CLKST1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NITR {
        save.CLKID = -99;
        spicelib::SCDECD(save.CLKID, save.CLKDP2, &mut save.CLKST2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NITR {
        save.CLKID = -999;
        spicelib::SCDECD(save.CLKID, save.CLKDP3, &mut save.CLKST3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ENDTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.TDELT1 = (save.ENDTIM - save.BEGTIM);

    //
    // Verify that we have no more than a 50% penalty incurred by
    // interleaving.
    //
    // Performance tests are executed only on platforms on which
    // ZZCPUTIM can report times with fractional seconds.
    //

    if (save.TDELT1 > 0.0) {
        save.FRAC = (save.TDELT0 / save.TDELT1);

        // CALL CHCKSD ( 'FRAC', FRAC, '<', 1.5D0, 0.D0, OK )
    }
    //
    // Check final SCLK strings.
    //
    testutil::CHCKSC(b"CLKST1", &save.CLKST1, b"=", &save.XCLKS1, OK, ctx)?;
    testutil::CHCKSC(b"CLKST2", &save.CLKST2, b"=", &save.XCLKS2, OK, ctx)?;
    testutil::CHCKSC(b"CLKST3", &save.CLKST3, b"=", &save.XCLKS3, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCS2E: interleave computations using three clocks; compare run time to that of non-interleaved computations.", ctx)?;

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.NITR = 10000;

    for I in 1..=save.NITR {
        save.CLKID = -9;
        spicelib::SCS2E(save.CLKID, &save.XCLKS1, &mut save.ET1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLKID = -99;
        spicelib::SCS2E(save.CLKID, &save.XCLKS2, &mut save.ET2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLKID = -999;
        spicelib::SCS2E(save.CLKID, &save.XCLKS3, &mut save.ET3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ENDTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.TDELT0 = (save.ENDTIM - save.BEGTIM);

    //
    // Check ET values.
    //
    // Clock -9 has a rate of 1 TDB second per most significant field.
    // This is equivalent to 1.e-5 seconds per tick.
    //
    spicelib::STR2ET(b"1980 JAN 1 00:00:00.00001 TDB", &mut save.XET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKSD(b"ET1 (A)", save.ET1, b"~/", save.XET1, save.TOL, OK, ctx)?;

    //
    // Clock -99 has, for the first 500 TDB seconds after clock start,
    // a rate of 2.000000001 TDB seconds per most significant field.
    // This is equivalent to 2.000000001/720 TDB seconds per tick.
    //
    // Clock start is at the J2000 epoch.
    //
    save.XET2 = (2.000000001 / 720 as f64);

    save.TOL = TIGHT;

    testutil::CHCKSD(b"ET2 (A)", save.ET2, b"~/", save.XET2, save.TOL, OK, ctx)?;

    //
    // Clock -999 has, for the first 500 TDT seconds after clock start,
    // a rate of 2.000000001 TDT seconds per most significant field.
    // This is equivalent to 2.000000001/720 TDT seconds per tick.
    //
    // Clock start is at the J2000 epoch.
    //
    save.TDT = (2.000000001 / 720 as f64);

    save.XET3 = spicelib::UNITIM(save.TDT, b"TDT", b"TDB", ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKSD(b"ET3 (A)", save.ET3, b"~/", save.XET3, save.TOL, OK, ctx)?;

    //
    // Now perform the computations without interleaving.
    //
    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    for I in 1..=save.NITR {
        save.CLKID = -9;
        spicelib::SCS2E(save.CLKID, &save.CLKST1, &mut save.XET1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NITR {
        save.CLKID = -99;
        spicelib::SCS2E(save.CLKID, &save.CLKST2, &mut save.XET2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NITR {
        save.CLKID = -999;
        spicelib::SCS2E(save.CLKID, &save.CLKST3, &mut save.XET3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ENDTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.TDELT1 = (save.ENDTIM - save.BEGTIM);

    //
    // Verify that we incur no more than a 50% penalty by interleaving.
    // Normally the penalty will be less than 10%, but variations in
    // system performance can lead to large excursions in measured run
    // times. We want to avoid spurious failure reports.
    //
    // Performance tests are executed only on platforms on which
    // ZZCPUTIM can report times with fractional seconds.
    //

    if (save.TDELT1 > 0.0) {
        save.FRAC = (save.TDELT0 / save.TDELT1);

        // CALL CHCKSD ( 'FRAC', FRAC, '<', 1.5D0, 0.D0, OK )
    }

    testutil::CHCKSD(b"ET1 (B)", save.ET1, b"~/", save.XET1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"ET2 (B)", save.ET2, b"~/", save.XET2, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"ET3 (B)", save.ET3, b"~/", save.XET3, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCE2S: interleave computations using three clocks; compare run time to that of non-interleaved computations.", ctx)?;

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.NITR = 10000;

    for I in 1..=save.NITR {
        save.CLKID = -9;
        spicelib::SCE2S(save.CLKID, save.XET1, &mut save.CLKST1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLKID = -99;
        spicelib::SCE2S(save.CLKID, save.XET2, &mut save.CLKST2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLKID = -999;
        spicelib::SCE2S(save.CLKID, save.XET3, &mut save.CLKST3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ENDTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.TDELT0 = (save.ENDTIM - save.BEGTIM);

    testutil::CHCKSC(b"CLKST1 (A)", &save.CLKST1, b"=", &save.XCLKS1, OK, ctx)?;
    testutil::CHCKSC(b"CLKST2 (A)", &save.CLKST2, b"=", &save.XCLKS2, OK, ctx)?;
    testutil::CHCKSC(b"CLKST3 (A)", &save.CLKST3, b"=", &save.XCLKS3, OK, ctx)?;

    //
    // Now perform the computations without interleaving.
    //
    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    for I in 1..=save.NITR {
        save.CLKID = -9;
        spicelib::SCE2S(save.CLKID, save.XET1, &mut save.CLKST1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NITR {
        save.CLKID = -99;
        spicelib::SCE2S(save.CLKID, save.XET2, &mut save.CLKST2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NITR {
        save.CLKID = -999;
        spicelib::SCE2S(save.CLKID, save.XET3, &mut save.CLKST3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ENDTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.TDELT1 = (save.ENDTIM - save.BEGTIM);
    //
    // Verify that we incur no more than a 50% penalty by interleaving.
    //
    // Performance tests are executed only on platforms on which
    // ZZCPUTIM can report times with fractional seconds.
    //

    if (save.TDELT1 > 0.0) {
        save.FRAC = (save.TDELT0 / save.TDELT1);

        // CALL CHCKSD ( 'FRAC', FRAC, '<', 1.5D0, 0.D0, OK )
    }

    testutil::CHCKSC(b"CLKST1 (B)", &save.CLKST1, b"=", &save.XCLKS1, OK, ctx)?;
    testutil::CHCKSC(b"CLKST2 (B)", &save.CLKST2, b"=", &save.XCLKS2, OK, ctx)?;
    testutil::CHCKSC(b"CLKST3 (B)", &save.CLKST3, b"=", &save.XCLKS3, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCE2C: interleave computations using three clocks; compare run time to that of non-interleaved computations.", ctx)?;

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.NITR = 10000;

    for I in 1..=save.NITR {
        save.CLKID = -9;
        spicelib::SCE2C(save.CLKID, save.XET1, &mut save.CLKDP1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLKID = -99;
        spicelib::SCE2C(save.CLKID, save.XET2, &mut save.CLKDP2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLKID = -999;
        spicelib::SCE2C(save.CLKID, save.XET3, &mut save.CLKDP3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ENDTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.TDELT0 = (save.ENDTIM - save.BEGTIM);

    //
    // Check tick values.
    //
    save.XCLKDP = 1.0;

    //
    // For clock -9, the start epoch is 1980 JAN 1 TDB, so we expect
    // only 0.1 microsecond precision. That corresponds to 0.01 ticks.
    //
    save.TOL = 0.01;

    testutil::CHCKSD(
        b"CLKDP1 (A)",
        save.CLKDP1,
        b"~",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // For clocks -99 and -999, the start epoch is J2000 TDB, so we
    // expect no extreme loss of precision.
    //
    save.TOL = TIGHT;
    testutil::CHCKSD(
        b"CLKDP2 (A)",
        save.CLKDP2,
        b"~/",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(
        b"CLKDP3 (A)",
        save.CLKDP3,
        b"~/",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Now perform the computations without interleaving.
    //

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    for I in 1..=save.NITR {
        save.CLKID = -9;
        spicelib::SCE2C(save.CLKID, save.ET1, &mut save.CLKDP1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NITR {
        save.CLKID = -99;
        spicelib::SCE2C(save.CLKID, save.ET2, &mut save.CLKDP2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NITR {
        save.CLKID = -999;
        spicelib::SCE2C(save.CLKID, save.ET3, &mut save.CLKDP3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ENDTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.TDELT1 = (save.ENDTIM - save.BEGTIM);

    //
    // Verify that we incur no more than a 50% penalty by interleaving.
    //
    // Performance tests are executed only on platforms on which
    // ZZCPUTIM can report times with fractional seconds.
    //

    if (save.TDELT1 > 0.0) {
        save.FRAC = (save.TDELT0 / save.TDELT1);

        // CALL CHCKSD ( 'FRAC', FRAC, '<', 1.5D0, 0.D0, OK )
    }
    //
    // Check tick values.
    //
    save.TOL = 0.01;
    testutil::CHCKSD(
        b"CLKDP1 (B)",
        save.CLKDP1,
        b"~",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    save.TOL = TIGHT;
    testutil::CHCKSD(
        b"CLKDP2 (B)",
        save.CLKDP2,
        b"~/",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"CLKDP3 (B)",
        save.CLKDP3,
        b"~/",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCE2T: interleave computations using three clocks; compare run time to that of non-interleaved computations.", ctx)?;

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.NITR = 10000;

    for I in 1..=save.NITR {
        save.CLKID = -9;
        spicelib::SCE2T(save.CLKID, save.ET1, &mut save.CLKDP1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLKID = -99;
        spicelib::SCE2T(save.CLKID, save.ET2, &mut save.CLKDP2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLKID = -999;
        spicelib::SCE2T(save.CLKID, save.ET3, &mut save.CLKDP3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ENDTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.TDELT0 = (save.ENDTIM - save.BEGTIM);

    //
    // Check tick values.
    //
    save.XCLKDP = 1.0;

    //
    // We should be able to expect exact matches in all cases.
    //
    save.TOL = 0.0;

    testutil::CHCKSD(
        b"CLKDP1 (A)",
        save.CLKDP1,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"CLKDP2 (A)",
        save.CLKDP2,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"CLKDP3 (A)",
        save.CLKDP3,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Now perform the computations without interleaving.
    //

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    for I in 1..=save.NITR {
        save.CLKID = -9;
        spicelib::SCE2T(save.CLKID, save.ET1, &mut save.CLKDP1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NITR {
        save.CLKID = -99;
        spicelib::SCE2T(save.CLKID, save.ET2, &mut save.CLKDP2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NITR {
        save.CLKID = -999;
        spicelib::SCE2T(save.CLKID, save.ET3, &mut save.CLKDP3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ENDTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.TDELT1 = (save.ENDTIM - save.BEGTIM);

    //
    // Verify that we incur no more than a 50% penalty by interleaving.
    //
    // Performance tests are executed only on platforms on which
    // ZZCPUTIM can report times with fractional seconds.
    //

    if (save.TDELT1 > 0.0) {
        save.FRAC = (save.TDELT0 / save.TDELT1);

        // CALL CHCKSD ( 'FRAC', FRAC, '<', 1.5D0, 0.D0, OK )
    }
    //
    // We should be able to expect exact matches in all cases.
    //
    save.TOL = 0.0;

    testutil::CHCKSD(
        b"CLKDP1 (B)",
        save.CLKDP1,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"CLKDP2 (B)",
        save.CLKDP2,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"CLKDP3 (B)",
        save.CLKDP3,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCT2E: interleave computations using three clocks; compare run time to that of non-interleaved computations.", ctx)?;

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.NITR = 10000;

    for I in 1..=save.NITR {
        save.CLKID = -9;
        spicelib::SCT2E(save.CLKID, save.CLKDP1, &mut save.ET1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLKID = -99;
        spicelib::SCT2E(save.CLKID, save.CLKDP2, &mut save.ET2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLKID = -999;
        spicelib::SCT2E(save.CLKID, save.CLKDP3, &mut save.ET3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ENDTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.TDELT0 = (save.ENDTIM - save.BEGTIM);

    //
    // Check ET values.
    //
    save.TOL = TIGHT;

    testutil::CHCKSD(b"ET1 (A)", save.ET1, b"~/", save.XET1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"ET2 (A)", save.ET2, b"~/", save.XET2, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"ET3 (A)", save.ET3, b"~/", save.XET3, save.TOL, OK, ctx)?;

    //
    // Now perform the computations without interleaving.
    //

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    for I in 1..=save.NITR {
        save.CLKID = -9;
        spicelib::SCT2E(save.CLKID, save.CLKDP1, &mut save.ET1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NITR {
        save.CLKID = -99;
        spicelib::SCT2E(save.CLKID, save.CLKDP2, &mut save.ET2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NITR {
        save.CLKID = -999;
        spicelib::SCT2E(save.CLKID, save.CLKDP3, &mut save.ET3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ENDTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.TDELT1 = (save.ENDTIM - save.BEGTIM);

    //
    // Verify that we incur no more than a 50% penalty by interleaving.
    //
    // Performance tests are executed only on platforms on which
    // ZZCPUTIM can report times with fractional seconds.
    //

    if (save.TDELT1 > 0.0) {
        save.FRAC = (save.TDELT0 / save.TDELT1);

        // CALL CHCKSD ( 'FRAC', FRAC, '<', 1.5D0, 0.D0, OK )
    }
    //
    // Check ET values.
    //
    save.TOL = TIGHT;

    testutil::CHCKSD(b"ET1 (B)", save.ET1, b"~/", save.XET1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"ET2 (B)", save.ET2, b"~/", save.XET2, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"ET3 (B)", save.ET3, b"~/", save.XET3, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPART: interleave lookups using three clocks; compare run time to that of non-interleaved lookups.", ctx)?;

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.NITR = 10000;

    for I in 1..=save.NITR {
        save.CLKID = -9;
        spicelib::SCPART(
            save.CLKID,
            &mut save.NPART1,
            save.PSTAR1.as_slice_mut(),
            save.PSTOP1.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLKID = -99;
        spicelib::SCPART(
            save.CLKID,
            &mut save.NPART2,
            save.PSTAR2.as_slice_mut(),
            save.PSTOP2.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CLKID = -999;
        spicelib::SCPART(
            save.CLKID,
            &mut save.NPART3,
            save.PSTAR3.as_slice_mut(),
            save.PSTOP3.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ENDTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.TDELT0 = (save.ENDTIM - save.BEGTIM);

    //
    // Check the final partition sets.
    //
    spicelib::GDPOOL(
        b"SCLK_PARTITION_START_9",
        1,
        MXPART,
        &mut save.XNPART,
        save.XSTART.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (START 1A)", save.FOUND, true, OK, ctx)?;

    testutil::CHCKAD(
        b"PSTAR1",
        save.PSTAR1.as_slice(),
        b"=",
        save.XSTART.as_slice(),
        save.XNPART,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::GDPOOL(
        b"SCLK_PARTITION_END_9",
        1,
        MXPART,
        &mut save.XNPART,
        save.XSTOP.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (STOP 1A)", save.FOUND, true, OK, ctx)?;

    testutil::CHCKAD(
        b"PSTOP1",
        save.PSTOP1.as_slice(),
        b"=",
        save.XSTOP.as_slice(),
        save.XNPART,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::GDPOOL(
        b"SCLK_PARTITION_START_99",
        1,
        MXPART,
        &mut save.XNPART,
        save.XSTART.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (START 2A)", save.FOUND, true, OK, ctx)?;

    testutil::CHCKAD(
        b"PSTAR2",
        save.PSTAR2.as_slice(),
        b"=",
        save.XSTART.as_slice(),
        save.XNPART,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::GDPOOL(
        b"SCLK_PARTITION_END_99",
        1,
        MXPART,
        &mut save.XNPART,
        save.XSTOP.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (STOP 2A)", save.FOUND, true, OK, ctx)?;

    testutil::CHCKAD(
        b"PSTOP2",
        save.PSTOP2.as_slice(),
        b"=",
        save.XSTOP.as_slice(),
        save.XNPART,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::GDPOOL(
        b"SCLK_PARTITION_START_999",
        1,
        MXPART,
        &mut save.XNPART,
        save.XSTART.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (START 3A)", save.FOUND, true, OK, ctx)?;

    testutil::CHCKAD(
        b"PSTAR3",
        save.PSTAR3.as_slice(),
        b"=",
        save.XSTART.as_slice(),
        save.XNPART,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::GDPOOL(
        b"SCLK_PARTITION_END_999",
        1,
        MXPART,
        &mut save.XNPART,
        save.XSTOP.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (STOP 3A)", save.FOUND, true, OK, ctx)?;

    testutil::CHCKAD(
        b"PSTOP3",
        save.PSTOP3.as_slice(),
        b"=",
        save.XSTOP.as_slice(),
        save.XNPART,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Now perform the computations without interleaving.
    //
    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    for I in 1..=save.NITR {
        save.CLKID = -9;
        spicelib::SCPART(
            save.CLKID,
            &mut save.NPART1,
            save.PSTAR1.as_slice_mut(),
            save.PSTOP1.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NITR {
        save.CLKID = -99;
        spicelib::SCPART(
            save.CLKID,
            &mut save.NPART2,
            save.PSTAR2.as_slice_mut(),
            save.PSTOP2.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NITR {
        save.CLKID = -999;
        spicelib::SCPART(
            save.CLKID,
            &mut save.NPART3,
            save.PSTAR3.as_slice_mut(),
            save.PSTOP3.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ENDTIM = (((save.TVEC[4] * 3600.0) + (save.TVEC[5] * 60.0)) + save.TVEC[6]);

    save.TDELT1 = (save.ENDTIM - save.BEGTIM);

    //
    // Verify that we incur no more than a 50% penalty by interleaving.
    //
    // Performance tests are executed only on platforms on which
    // ZZCPUTIM can report times with fractional seconds.
    //

    if (save.TDELT1 > 0.0) {
        save.FRAC = (save.TDELT0 / save.TDELT1);

        // CALL CHCKSD ( 'FRAC', FRAC, '<', 1.5D0, 0.D0, OK )
    }

    //
    // Check the final partition sets.
    //
    spicelib::GDPOOL(
        b"SCLK_PARTITION_START_9",
        1,
        MXPART,
        &mut save.XNPART,
        save.XSTART.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (START 1B)", save.FOUND, true, OK, ctx)?;

    testutil::CHCKAD(
        b"PSTAR1",
        save.PSTAR1.as_slice(),
        b"=",
        save.XSTART.as_slice(),
        save.XNPART,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::GDPOOL(
        b"SCLK_PARTITION_END_9",
        1,
        MXPART,
        &mut save.XNPART,
        save.XSTOP.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (STOP 1B)", save.FOUND, true, OK, ctx)?;

    testutil::CHCKAD(
        b"PSTOP1",
        save.PSTOP1.as_slice(),
        b"=",
        save.XSTOP.as_slice(),
        save.XNPART,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::GDPOOL(
        b"SCLK_PARTITION_START_99",
        1,
        MXPART,
        &mut save.XNPART,
        save.XSTART.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (START 2B)", save.FOUND, true, OK, ctx)?;

    testutil::CHCKAD(
        b"PSTAR2",
        save.PSTAR2.as_slice(),
        b"=",
        save.XSTART.as_slice(),
        save.XNPART,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::GDPOOL(
        b"SCLK_PARTITION_END_99",
        1,
        MXPART,
        &mut save.XNPART,
        save.XSTOP.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (STOP 2B)", save.FOUND, true, OK, ctx)?;

    testutil::CHCKAD(
        b"PSTOP2",
        save.PSTOP2.as_slice(),
        b"=",
        save.XSTOP.as_slice(),
        save.XNPART,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::GDPOOL(
        b"SCLK_PARTITION_START_999",
        1,
        MXPART,
        &mut save.XNPART,
        save.XSTART.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (START 3B)", save.FOUND, true, OK, ctx)?;

    testutil::CHCKAD(
        b"PSTAR3",
        save.PSTAR3.as_slice(),
        b"=",
        save.XSTART.as_slice(),
        save.XNPART,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::GDPOOL(
        b"SCLK_PARTITION_END_999",
        1,
        MXPART,
        &mut save.XNPART,
        save.XSTOP.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (STOP 3B)", save.FOUND, true, OK, ctx)?;

    testutil::CHCKAD(
        b"PSTOP3",
        save.PSTOP3.as_slice(),
        b"=",
        save.XSTOP.as_slice(),
        save.XNPART,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::KCLEAR(ctx)?;

    spicelib::DELFIL(CK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SCLK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
