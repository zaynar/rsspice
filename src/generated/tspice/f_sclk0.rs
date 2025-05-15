//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
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
const VTIGHT: f64 = 0.00000000000001;
const DVALSZ: i32 = 50;
const IVALSZ: i32 = 50;
const KVNMLN: i32 = 32;
const LNSIZE: i32 = 80;
const NAMLEN: i32 = 50;
const TIMLEN: i32 = 40;
const LBCELL: i32 = -5;
const NCOM: i32 = 3;
const SHRTLN: i32 = 5;

struct SaveVars {
    CLKNAM: Vec<u8>,
    CLKSTR: Vec<u8>,
    COMS: ActualCharArray,
    ERRMSG: Vec<u8>,
    ITEM: Vec<u8>,
    KVNAME: Vec<u8>,
    SHORT: Vec<u8>,
    SHRTMS: Vec<u8>,
    XMSG: Vec<u8>,
    COEFFS: ActualArray2D<f64>,
    DELTA: f64,
    DVALS: StackArray<f64, 50>,
    ET: f64,
    KERNID: f64,
    MAJOR: f64,
    MODULI: StackArray<f64, 10>,
    OFFSET: StackArray<f64, 10>,
    PSTART: ActualArray<f64>,
    PSTOP: ActualArray<f64>,
    TDT: f64,
    TICKS: f64,
    TOL: f64,
    SCLKDP: f64,
    XCLKDP: f64,
    XET: f64,
    XSTART: ActualArray<f64>,
    XSTOP: ActualArray<f64>,
    CLKID: i32,
    CLKTYP: i32,
    FIELD: StackArray<i32, 4>,
    HANDLE: i32,
    I: i32,
    IVAL: i32,
    IVALS: StackArray<i32, 50>,
    N: i32,
    NCOEFF: i32,
    NFIELD: i32,
    NPARTS: i32,
    ERROR: bool,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CLKNAM = vec![b' '; NAMLEN as usize];
        let mut CLKSTR = vec![b' '; TIMLEN as usize];
        let mut COMS = ActualCharArray::new(LNSIZE, LBCELL..=NCOM);
        let mut ERRMSG = vec![b' '; LMSGLN as usize];
        let mut ITEM = vec![b' '; KVNMLN as usize];
        let mut KVNAME = vec![b' '; KVNMLN as usize];
        let mut SHORT = vec![b' '; SHRTLN as usize];
        let mut SHRTMS = vec![b' '; SMSGLN as usize];
        let mut XMSG = vec![b' '; LMSGLN as usize];
        let mut COEFFS = ActualArray2D::<f64>::new(1..=3, 1..=(MXCOEF + 1));
        let mut DELTA: f64 = 0.0;
        let mut DVALS = StackArray::<f64, 50>::new(1..=DVALSZ);
        let mut ET: f64 = 0.0;
        let mut KERNID: f64 = 0.0;
        let mut MAJOR: f64 = 0.0;
        let mut MODULI = StackArray::<f64, 10>::new(1..=MXNFLD);
        let mut OFFSET = StackArray::<f64, 10>::new(1..=MXNFLD);
        let mut PSTART = ActualArray::<f64>::new(1..=(MXPART + 1));
        let mut PSTOP = ActualArray::<f64>::new(1..=(MXPART + 1));
        let mut TDT: f64 = 0.0;
        let mut TICKS: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut SCLKDP: f64 = 0.0;
        let mut XCLKDP: f64 = 0.0;
        let mut XET: f64 = 0.0;
        let mut XSTART = ActualArray::<f64>::new(1..=(MXPART + 1));
        let mut XSTOP = ActualArray::<f64>::new(1..=(MXPART + 1));
        let mut CLKID: i32 = 0;
        let mut CLKTYP: i32 = 0;
        let mut FIELD = StackArray::<i32, 4>::new(1..=4);
        let mut HANDLE: i32 = 0;
        let mut I: i32 = 0;
        let mut IVAL: i32 = 0;
        let mut IVALS = StackArray::<i32, 50>::new(1..=IVALSZ);
        let mut N: i32 = 0;
        let mut NCOEFF: i32 = 0;
        let mut NFIELD: i32 = 0;
        let mut NPARTS: i32 = 0;
        let mut ERROR: bool = false;
        let mut FOUND: bool = false;

        Self {
            CLKNAM,
            CLKSTR,
            COMS,
            ERRMSG,
            ITEM,
            KVNAME,
            SHORT,
            SHRTMS,
            XMSG,
            COEFFS,
            DELTA,
            DVALS,
            ET,
            KERNID,
            MAJOR,
            MODULI,
            OFFSET,
            PSTART,
            PSTOP,
            TDT,
            TICKS,
            TOL,
            SCLKDP,
            XCLKDP,
            XET,
            XSTART,
            XSTOP,
            CLKID,
            CLKTYP,
            FIELD,
            HANDLE,
            I,
            IVAL,
            IVALS,
            N,
            NCOEFF,
            NFIELD,
            NPARTS,
            ERROR,
            FOUND,
        }
    }
}

//$Procedure F_SCLK0 ( SCLK error handling tests )
pub fn F_SCLK0(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_SCLK0", ctx)?;

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

    {
        let m1__: i32 = 2;
        let m2__: i32 = 4;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.MAJOR = (save.MAJOR * save.MODULI[save.I]);
            save.I += m3__;
        }
    }

    save.OFFSET[1] = 0 as f64;
    save.OFFSET[2] = 0 as f64;
    save.OFFSET[3] = 0 as f64;
    save.OFFSET[4] = 0 as f64;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NPARTS;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XSTART[save.I] = save.I as f64;
            save.XSTOP[save.I] = (720000 + save.I) as f64;
            save.I += m3__;
        }
    }

    save.DELTA = 0.000000001;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOEFF;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.COEFFS[[1, save.I]] = (((save.I - 1) as f64) * 180000.0);

            save.COEFFS[[2, save.I]] = ((save.COEFFS[[1, save.I]] * 2 as f64) / save.MAJOR);

            save.COEFFS[[3, save.I]] = (2.0 + ((save.I as f64) * save.DELTA));

            save.I += m3__;
        }
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

    {
        let m1__: i32 = 2;
        let m2__: i32 = 4;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.MAJOR = (save.MAJOR * save.MODULI[save.I]);
            save.I += m3__;
        }
    }

    save.OFFSET[1] = 4 as f64;
    save.OFFSET[2] = 3 as f64;
    save.OFFSET[3] = 2 as f64;
    save.OFFSET[4] = 1 as f64;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NPARTS;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XSTART[save.I] = save.I as f64;
            save.XSTOP[save.I] = (720000 + save.I) as f64;
            save.I += m3__;
        }
    }

    //
    // Make the second partition start with a forward jump.
    //
    save.XSTART[2] = (save.XSTART[2] + 1000000.0);
    save.XSTOP[2] = (save.XSTOP[2] + 1000000.0);

    save.DELTA = 0.000000001;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOEFF;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.COEFFS[[1, save.I]] = (((save.I - 1) as f64) * 180000.0);

            save.COEFFS[[2, save.I]] = ((save.COEFFS[[1, save.I]] * 2 as f64) / save.MAJOR);

            save.COEFFS[[3, save.I]] = (2.0 + ((save.I as f64) * save.DELTA));

            save.I += m3__;
        }
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

    //*********************************************************
    //
    //
    //     End of kernel creation code
    //
    //
    //*********************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCDECD: ticks not in any partition", ctx)?;

    save.CLKID = -99;
    save.SCLKDP = -1.0;

    spicelib::SCDECD(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Output string is supposed to be blank upon failure.
    //
    testutil::CHCKSC(b"CLKSTR (-1.D0)", &save.CLKSTR, b"=", b" ", OK, ctx)?;

    save.SCLKDP = 720720001.0;

    spicelib::SCDECD(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    testutil::CHCKSC(b"CLKSTR (720721002.D0)", &save.CLKSTR, b"=", b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCDECD: output string truncated, caught in SCFMT", ctx)?;

    save.CLKID = -99;
    save.SCLKDP = 720720000.0;

    spicelib::SCDECD(save.CLKID, save.SCLKDP, &mut save.SHORT, ctx)?;

    testutil::CHCKXC(true, b"SPICE(SCLKTRUNCATED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCDECD: output string truncated, caught in SCDECD", ctx)?;

    save.CLKID = -99;
    save.SCLKDP = 720720000.0;

    spicelib::SCDECD(
        save.CLKID,
        save.SCLKDP,
        fstr::substr_mut(&mut save.CLKSTR, 1..=15),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SCLKTRUNCATED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCDECD: SCPART failure", ctx)?;

    //
    // Force a failure of SCPART.
    //
    spicelib::DVPOOL(b"SCLK_PARTITION_END_99", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CLKID = -99;
    save.SCLKDP = 720720000.0;

    spicelib::SCDECD(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCENCD: SCPART failure", ctx)?;

    //
    // Force a failure of SCPART.
    //
    spicelib::DVPOOL(b"SCLK_PARTITION_END_99", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CLKID = -99;
    fstr::assign(&mut save.CLKSTR, b"9999999:9:8:7");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCENCD: SCTIKS failure", ctx)?;

    save.CLKID = -99;

    //
    // Create clock string with too many fields.
    //
    fstr::assign(&mut save.CLKSTR, b"0000000:9:8:7:6");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSCLKSTRING)", OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCENCD: blank input string", ctx)?;

    save.CLKID = -99;

    fstr::assign(&mut save.CLKSTR, b" ");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSCLKSTRING)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCENCD: time string ends with slash", ctx)?;

    save.CLKID = -99;

    spicelib::SCENCD(save.CLKID, b" /", &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSCLKSTRING)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCENCD: no time string following slash", ctx)?;

    save.CLKID = -99;

    spicelib::SCENCD(save.CLKID, b"1/ ", &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSCLKSTRING)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCENCD: No partition number preceding slash", ctx)?;

    save.CLKID = -99;

    fstr::assign(&mut save.CLKSTR, b"/0000000:9:8:7");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADPARTNUMBER)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCENCD: Non-numeric partition token preceding slash", ctx)?;

    save.CLKID = -99;

    fstr::assign(&mut save.CLKSTR, b"x/0000000:9:8:7");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADPARTNUMBER)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCENCD: partition number out of range", ctx)?;

    save.CLKID = -99;

    //
    // Try partition numbers that are
    //
    //    negative
    //    zero
    //    too large
    //
    fstr::assign(&mut save.CLKSTR, b"-1/0000000:9:8:7");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADPARTNUMBER)", OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"0/0000000:9:8:7");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADPARTNUMBER)", OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"1002/0000000:9:8:7");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADPARTNUMBER)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCENCD: clock count not in indicated partition", ctx)?;

    save.CLKID = -99;

    fstr::assign(&mut save.CLKSTR, b"1/0001001:0:0:0");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOTINPART)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCENCD: clock count not in any partition", ctx)?;

    save.CLKID = -99;

    fstr::assign(&mut save.CLKSTR, b"0000000:0:0:0");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOPARTITION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPARS: SCPART failure", ctx)?;

    //
    // Force a failure of SCPART.
    //
    spicelib::DVPOOL(b"SCLK_PARTITION_END_99", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CLKID = -99;
    fstr::assign(&mut save.CLKSTR, b"9999999:9:8:7");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPARS: SCTIKS failure", ctx)?;

    save.CLKID = -99;

    //
    // Create clock string with too many fields.
    //
    fstr::assign(&mut save.CLKSTR, b"0000000:9:8:7:6");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;

    fstr::assign(
        &mut save.XMSG,
        b"Input clock string 0000000:9:8:7:6 has 5 fields; maximum for this spacecraft clock is 4.",
    );

    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPARS: unsupported clock type", ctx)?;

    save.CLKID = -101;

    fstr::assign(&mut save.KVNAME, b"SCLK_DATA_TYPE_101");

    spicelib::PIPOOL(&save.KVNAME, 1, &[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCLKDP = 0.0;

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPARS: blank input string", ctx)?;

    save.CLKID = -99;

    fstr::assign(&mut save.CLKSTR, b" ");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;

    fstr::assign(&mut save.XMSG, b"Input spacecraft clock string is blank.");

    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPARS: input string ends with slash", ctx)?;

    save.CLKID = -99;

    spicelib::SCPARS(
        save.CLKID,
        b" /",
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;

    fstr::assign(&mut save.XMSG, b"SCLK string ends with slash.");

    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPARS: no time string following slash", ctx)?;

    save.CLKID = -99;

    fstr::assign(&mut save.CLKSTR, b"1/ ");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;

    fstr::assign(
        &mut save.XMSG,
        b"Non partition part of the input clock string is blank.",
    );

    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPARS: No partition number preceding slash", ctx)?;

    save.CLKID = -99;

    fstr::assign(&mut save.CLKSTR, b"/0000000:9:8:7");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;

    fstr::assign(
        &mut save.XMSG,
        b"Unable to parse the partition number from SCLK string /0000000:9:8:7.",
    );

    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPARS: Non-numeric partition token preceding slash", ctx)?;

    save.CLKID = -99;

    fstr::assign(&mut save.CLKSTR, b"x/0000000:9:8:7");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;

    fstr::assign(
        &mut save.XMSG,
        b"Unable to parse the partition number from SCLK string x/0000000:9:8:7.",
    );

    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPARS: partition number out of range", ctx)?;

    save.CLKID = -99;

    //
    // Try partition numbers that are
    //
    //    negative
    //    zero
    //    too large
    //
    fstr::assign(&mut save.CLKSTR, b"-1/0000000:9:8:7");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;

    fstr::assign(&mut save.XMSG, b"Partition number -1 taken from SCLK string -1/0000000:9:8:7 is not in acceptable range 1 to 1001.");

    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"0/0000000:9:8:7");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;

    fstr::assign(&mut save.XMSG, b"Partition number 0 taken from SCLK string 0/0000000:9:8:7 is not in acceptable range 1 to 1001.");

    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"1002/0000000:9:8:7");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;

    fstr::assign(&mut save.XMSG, b"Partition number 1002 taken from SCLK string 1002/0000000:9:8:7 is not in acceptable range 1 to 1001.");

    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPARS: clock count not in indicated partition", ctx)?;

    save.CLKID = -99;

    fstr::assign(&mut save.CLKSTR, b"1/0001001:0:0:0");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;

    fstr::assign(
        &mut save.XMSG,
        b"SCLK count from 0001001:0:0:0 does not fall in the boundaries of partition number 1.",
    );

    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPARS: clock count not in any partition", ctx)?;

    save.CLKID = -99;

    fstr::assign(&mut save.CLKSTR, b"0000000:0:0:0");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;

    fstr::assign(&mut save.XMSG, b"SCLK count 0000000:0:0:0 does not fall in the boundaries of any of the partitions for spacecraft -99.");

    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SCPARS: non-error exception: fields are blank, but string is not blank.",
        ctx,
    )?;

    save.CLKID = -999;

    //
    // The spec of SCTK01 says that blank fields are treated as though
    // they contain the offset corresponding to that field. This makes
    // the values of these fields effectively zero.
    //
    fstr::assign(&mut save.CLKSTR, b":::3");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"ERROR", save.ERROR, false, OK, ctx)?;
    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", b" ", OK, ctx)?;

    save.XCLKDP = 1.0;

    save.TOL = 0.0;
    testutil::CHCKSD(
        b"SCLKDP (A)",
        save.SCLKDP,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.CLKSTR, b" : : : 3");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"ERROR", save.ERROR, false, OK, ctx)?;
    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", b" ", OK, ctx)?;

    save.XCLKDP = 1.0;

    save.TOL = 0.0;
    testutil::CHCKSD(
        b"SCLKDP (B)",
        save.SCLKDP,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.CLKSTR, b"0000004: : :3");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"ERROR", save.ERROR, false, OK, ctx)?;
    testutil::CHCKSC(b"ERRMST", &save.ERRMSG, b"=", b" ", OK, ctx)?;

    save.XCLKDP = 1.0;

    save.TOL = 0.0;
    testutil::CHCKSD(
        b"SCLKDP (C)",
        save.SCLKDP,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.CLKSTR, b"0000004: :4: 2");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"ERROR", save.ERROR, false, OK, ctx)?;
    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", b" ", OK, ctx)?;

    save.XCLKDP = 16.0;

    save.TOL = 0.0;
    testutil::CHCKSD(
        b"SCLKDP (D)",
        save.SCLKDP,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.CLKSTR, b"0000004:6: : 2");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"ERROR", save.ERROR, false, OK, ctx)?;
    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", b" ", OK, ctx)?;

    save.XCLKDP = 216.0;

    save.TOL = 0.0;
    testutil::CHCKSD(
        b"SCLKDP (E)",
        save.SCLKDP,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.CLKSTR, b"0000008: : : 2");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"ERROR", save.ERROR, false, OK, ctx)?;
    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", b" ", OK, ctx)?;

    save.XCLKDP = 2880.0;

    save.TOL = 0.0;
    testutil::CHCKSD(
        b"SCLKDP (E)",
        save.SCLKDP,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPS01: moduli not found by SCLD01", ctx)?;

    save.CLKID = -9;

    spicelib::DVPOOL(b"SCLK01_MODULI_9", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"1/000000000.0001");

    spicelib::SCPS01(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Restore data for this clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPS01: offsets not found by SCLD01", ctx)?;

    save.CLKID = -9;

    spicelib::DVPOOL(b"SCLK01_OFFSETS_9", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"1/000000000.0001");

    spicelib::SCPS01(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Restore data for this clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPS01: offsets not found by SCLD01", ctx)?;

    save.CLKID = -9;

    spicelib::DVPOOL(b"SCLK01_OFFSETS_9", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"1/000000000.0001");

    spicelib::SCPS01(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Restore data for this clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPS01: field count not found by SCLI01", ctx)?;

    save.CLKID = -9;

    spicelib::DVPOOL(b"SCLK01_N_FIELDS_9", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"1/000000000.0001");

    spicelib::SCPS01(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Restore data for this clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPS01: blank input string", ctx)?;

    save.CLKID = -999;

    fstr::assign(&mut save.CLKSTR, b" ");

    spicelib::SCPS01(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;

    fstr::assign(
        &mut save.XMSG,
        b"Non partition part of the input clock string is blank.",
    );

    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPS01: components out of range", ctx)?;

    save.CLKID = -999;

    fstr::assign(&mut save.CLKSTR, b"0000000:3:2:2");

    spicelib::SCPS01(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"ERROR (A)", save.ERROR, true, OK, ctx)?;

    fstr::assign(
        &mut save.XMSG,
        b"Component number 1, 0000000 in the SCLK string  0000000:3:2:2 is invalid.",
    );

    testutil::CHCKSC(b"ERRMSG (A)", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"0000004:1:2:2");

    spicelib::SCPS01(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"ERROR (B)", save.ERROR, true, OK, ctx)?;

    fstr::assign(
        &mut save.XMSG,
        b"Component number 2, 1 in the SCLK string  0000004:1:2:2 is invalid.",
    );

    testutil::CHCKSC(b"ERRMSG (B)", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPS01: non-numeric clock component", ctx)?;

    save.CLKID = -9;

    fstr::assign(&mut save.CLKSTR, b"000000000.x001");

    spicelib::SCPS01(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(
        &mut save.XMSG,
        b"Could not parse SCLK component x001 from 000000000.x001 as a number.",
    );

    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;
    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCPS01: too many fields in clock string", ctx)?;

    save.CLKID = -9;

    fstr::assign(&mut save.CLKSTR, b"000000000.001.00");

    spicelib::SCPS01(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.XMSG, b"Input clock string 000000000.001.00 has 3 fields; maximum for this spacecraft clock is 2.");

    testutil::CHCKSL(b"ERROR", save.ERROR, true, OK, ctx)?;
    testutil::CHCKSC(b"ERRMSG", &save.ERRMSG, b"=", &save.XMSG, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCFMT: unsupported clock type", ctx)?;

    save.CLKID = -101;

    fstr::assign(&mut save.KVNAME, b"SCLK_DATA_TYPE_101");

    spicelib::PIPOOL(&save.KVNAME, 1, &[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCLKDP = 0.0;

    spicelib::SCFMT(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCT2E: unsupported clock type", ctx)?;

    save.CLKID = -101;

    fstr::assign(&mut save.KVNAME, b"SCLK_DATA_TYPE_101");

    spicelib::PIPOOL(&save.KVNAME, 1, &[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCLKDP = 0.0;

    spicelib::SCT2E(save.CLKID, save.SCLKDP, &mut save.ET, ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCTIKS: unsupported clock type", ctx)?;

    save.CLKID = -101;

    fstr::assign(&mut save.KVNAME, b"SCLK_DATA_TYPE_101");

    spicelib::PIPOOL(&save.KVNAME, 1, &[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"0000000:0:0:0");

    spicelib::SCTIKS(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCTK01: blank input string", ctx)?;

    save.CLKID = -99;

    fstr::assign(&mut save.CLKSTR, b" ");

    spicelib::SCTK01(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSCLKSTRING)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCTK01: too many fields in input string", ctx)?;

    save.CLKID = -99;

    fstr::assign(&mut save.CLKSTR, b"9:9:8:7:6");

    spicelib::SCTK01(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSCLKSTRING)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCTK01: input string component is not a number", ctx)?;

    save.CLKID = -99;

    fstr::assign(&mut save.CLKSTR, b"0000000:w:7:6");

    spicelib::SCTK01(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSCLKSTRING)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SCTK01: input string component is invalid after subtracting offset",
        ctx,
    )?;

    //
    // Note: change of clock
    //
    save.CLKID = -999;

    fstr::assign(&mut save.CLKSTR, b"00000004:4:7:0");

    spicelib::SCTK01(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSCLKSTRING)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SCTK01: non-error exception: fields are blank, but string is not blank.",
        ctx,
    )?;

    save.CLKID = -999;

    //
    // The spec of SCTK01 says that blank fields are treated as though
    // they contain the offset corresponding to that field. This makes
    // the values of these fields effectively zero.
    //
    fstr::assign(&mut save.CLKSTR, b":::");

    spicelib::SCTK01(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 0.0;

    save.TOL = 0.0;
    testutil::CHCKSD(
        b"SCLKDP (A)",
        save.SCLKDP,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.CLKSTR, b" : : : ");

    spicelib::SCTK01(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 0.0;

    save.TOL = 0.0;
    testutil::CHCKSD(
        b"SCLKDP (B)",
        save.SCLKDP,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.CLKSTR, b"0000004: : :2");

    spicelib::SCTK01(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 1.0;

    save.TOL = 0.0;
    testutil::CHCKSD(
        b"SCLKDP (C)",
        save.SCLKDP,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.CLKSTR, b"0000004: :4: ");

    spicelib::SCTK01(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 16.0;

    save.TOL = 0.0;
    testutil::CHCKSD(
        b"SCLKDP (D)",
        save.SCLKDP,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.CLKSTR, b"0000004:6: : ");

    spicelib::SCTK01(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 216.0;

    save.TOL = 0.0;
    testutil::CHCKSD(
        b"SCLKDP (E)",
        save.SCLKDP,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.CLKSTR, b"0000008: : : ");

    spicelib::SCTK01(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 2880.0;

    save.TOL = 0.0;
    testutil::CHCKSD(
        b"SCLKDP (E)",
        save.SCLKDP,
        b"=",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCTK01: exercise pass 1 logic after error", ctx)?;

    //
    // Cause error:
    //
    save.CLKID = -999;

    fstr::assign(&mut save.CLKSTR, b"0000004:4:7:0");

    spicelib::SCTK01(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSCLKSTRING)", OK, ctx)?;

    //
    // Call routine with valid inputs:
    //
    // Note: offsets for this clock are 4, 3, 2, 1, so the string below
    // is equivalent to
    //
    //    '0000999:9:8:7'
    //
    // after offsets are subtracted.
    //
    fstr::assign(&mut save.CLKSTR, b"0001003:12:10:8");

    spicelib::SCTK01(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"SCLKDP", save.SCLKDP, b"=", 719999.0, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCTK01: ZZSCUP01 failure", ctx)?;

    //
    // Cause failure.
    //
    spicelib::DVPOOL(b"SCLK01_MODULI_999", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CLKID = -999;

    fstr::assign(&mut save.CLKSTR, b"0001003:12:10:8");

    spicelib::SCTK01(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    spicelib::SCEC01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Check recovery.
    //
    spicelib::LDPOOL(SCLK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"0001003:12:10:8");

    spicelib::SCTK01(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"SCLKDP", save.SCLKDP, b"=", 719999.0, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCFM01: negative ticks", ctx)?;

    save.CLKID = -999;

    save.SCLKDP = -1 as f64;

    spicelib::SCFM01(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCFM01: output string truncation", ctx)?;

    save.CLKID = -999;

    save.SCLKDP = 719999.0;

    spicelib::SCFM01(save.CLKID, save.SCLKDP, &mut save.SHORT, ctx)?;

    testutil::CHCKXC(true, b"SPICE(SCLKTRUNCATED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCFM01: exercise pass 1 logic after error", ctx)?;

    //
    // Cause error:
    //
    save.SCLKDP = -1 as f64;

    spicelib::SCFM01(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Call routine with valid inputs:
    //
    save.SCLKDP = 719999.0;

    spicelib::SCFM01(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"CLKSTR", &save.CLKSTR, b"=", b"00001003:12:10:8", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCFM01: ZZSCUP01 failure", ctx)?;

    //
    // Cause failure.
    //
    spicelib::DVPOOL(b"SCLK01_MODULI_999", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"0001003:12:10:8");

    save.SCLKDP = 719999.0;

    spicelib::SCFM01(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Restore data for clock.
    //
    spicelib::LDPOOL(SCLK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCLKDP = 719999.0;

    spicelib::SCFM01(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"CLKSTR", &save.CLKSTR, b"=", b"00001003:12:10:8", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCTE01: ticks out of range", ctx)?;

    save.CLKID = -999;

    save.SCLKDP = -1.0;

    spicelib::SCTE01(save.CLKID, save.SCLKDP, &mut save.ET, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.SCLKDP = (((4 as f64) * 720000.0) + 1.0);

    spicelib::SCTE01(save.CLKID, save.SCLKDP, &mut save.ET, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCTE01: invalid clock rate", ctx)?;

    //
    // Note: change of clock
    //
    save.CLKID = -9;

    save.SCLKDP = 1.0;

    fstr::assign(&mut save.KVNAME, b"SCLK01_COEFFICIENTS_9");

    spicelib::GDPOOL(
        &save.KVNAME,
        1,
        3,
        &mut save.N,
        save.COEFFS.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Use 0 rate.
    //
    save.COEFFS[[3, 1]] = 0.0;

    spicelib::PDPOOL(&save.KVNAME, 3, save.COEFFS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCTE01(save.CLKID, save.SCLKDP, &mut save.ET, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Use negative rate.
    //
    save.COEFFS[[3, 1]] = -1.0;

    spicelib::PDPOOL(&save.KVNAME, 3, save.COEFFS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCTE01(save.CLKID, save.SCLKDP, &mut save.ET, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Restore data for clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCTE01: exercise pass 1 logic after error", ctx)?;

    //
    // Cause error:
    //
    save.CLKID = -999;

    save.SCLKDP = -1.0;

    spicelib::SCTE01(save.CLKID, save.SCLKDP, &mut save.ET, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    //
    // Check recovery.
    //
    save.SCLKDP = 180000.0;

    spicelib::SCTE01(save.CLKID, save.SCLKDP, &mut save.ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TDT = 500.0;

    save.XET = spicelib::UNITIM(save.TDT, b"TDT", b"TDB", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"ET", save.ET, b"~/", save.XET, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCTE01: ZZSCUP01 failure", ctx)?;

    //
    // Cause failure.
    //
    spicelib::DVPOOL(b"SCLK01_MODULI_999", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCLKDP = 180000.0;

    spicelib::SCTE01(save.CLKID, save.SCLKDP, &mut save.ET, ctx)?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Check recovery.
    //
    spicelib::LDPOOL(SCLK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCLKDP = 180000.0;

    spicelib::SCTE01(save.CLKID, save.SCLKDP, &mut save.ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TDT = 500.0;

    save.XET = spicelib::UNITIM(save.TDT, b"TDT", b"TDB", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"ET", save.ET, b"~/", save.XET, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCET01: computed ticks out of range", ctx)?;

    save.CLKID = -999;

    save.ET = -1.0;

    spicelib::SCET01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.ET = 10000000000000.0;

    spicelib::SCET01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCET01: ET is less than least coefficient but corresponding tick value rounds up to zero. This is a non-error exception.", ctx)?;

    save.CLKID = -999;

    save.ET = -0.000001;

    spicelib::SCET01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"SCLKDP", save.SCLKDP, b"~", 0.0, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCET01: invalid clock rate", ctx)?;

    //
    // Note: change of clock
    //
    save.CLKID = -9;

    save.ET = 1.0;

    fstr::assign(&mut save.KVNAME, b"SCLK01_COEFFICIENTS_9");

    spicelib::GDPOOL(
        &save.KVNAME,
        1,
        3,
        &mut save.N,
        save.COEFFS.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Use 0 rate.
    //
    save.COEFFS[[3, 1]] = 0.0;

    spicelib::PDPOOL(&save.KVNAME, 3, save.COEFFS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCET01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Use negative rate.
    //
    save.COEFFS[[3, 1]] = -1.0;

    spicelib::PDPOOL(&save.KVNAME, 3, save.COEFFS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCET01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Restore data for clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCET01: exercise pass 1 logic after error", ctx)?;

    //
    // Cause error:
    //
    // Note: change of clock
    //
    save.CLKID = -999;

    save.ET = -1.0;

    spicelib::SCET01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    //
    // Check recovery.
    //
    save.TDT = 500.0;

    save.ET = spicelib::UNITIM(save.TDT, b"TDT", b"TDB", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 180000.0;

    spicelib::SCET01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(
        b"SCLKDP",
        save.SCLKDP,
        b"~/",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCET01: ZZSCUP01 failure", ctx)?;

    //
    // Cause failure.
    //
    spicelib::DVPOOL(b"SCLK01_MODULI_999", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CLKID = -999;

    save.ET = 1.0;

    spicelib::SCET01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Check recovery.
    //
    spicelib::LDPOOL(SCLK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TDT = 500.0;

    save.ET = spicelib::UNITIM(save.TDT, b"TDT", b"TDB", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 180000.0;

    spicelib::SCET01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(
        b"SCLKDP",
        save.SCLKDP,
        b"~/",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCEC01: invalid clock rate", ctx)?;

    //
    // Note: change of clock
    //
    save.CLKID = -9;

    save.ET = 1.0;

    fstr::assign(&mut save.KVNAME, b"SCLK01_COEFFICIENTS_9");

    spicelib::GDPOOL(
        &save.KVNAME,
        1,
        3,
        &mut save.N,
        save.COEFFS.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Use 0 rate.
    //
    save.COEFFS[[3, 1]] = 0.0;

    spicelib::PDPOOL(&save.KVNAME, 3, save.COEFFS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCEC01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Use negative rate.
    //
    save.COEFFS[[3, 1]] = -1.0;

    spicelib::PDPOOL(&save.KVNAME, 3, save.COEFFS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCEC01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Restore data for clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCEC01: computed ticks out of range", ctx)?;

    save.CLKID = -999;

    save.ET = -1.0;

    spicelib::SCEC01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.ET = 10000000000000.0;

    spicelib::SCEC01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCEC01: exercise pass 1 logic after error", ctx)?;

    //
    // Cause error:
    //
    // Note: change of clock
    //
    save.CLKID = -999;

    save.ET = -1.0;

    spicelib::SCEC01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    //
    // Check recovery.
    //
    save.TDT = 500.0;

    save.ET = spicelib::UNITIM(save.TDT, b"TDT", b"TDB", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 180000.0;

    spicelib::SCEC01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(
        b"SCLKDP",
        save.SCLKDP,
        b"~/",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCEC01: ZZSCUP01 failure", ctx)?;

    //
    // Cause failure.
    //
    spicelib::DVPOOL(b"SCLK01_MODULI_999", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CLKID = -999;

    save.ET = 1.0;

    spicelib::SCEC01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Check recovery.
    //
    spicelib::LDPOOL(SCLK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TDT = 500.0;

    save.ET = spicelib::UNITIM(save.TDT, b"TDT", b"TDB", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 180000.0;

    spicelib::SCEC01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(
        b"SCLKDP",
        save.SCLKDP,
        b"~/",
        save.XCLKDP,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCTY01: clock type not found", ctx)?;

    //
    // Note: change of clock
    //
    save.CLKID = 1;

    save.ET = 1.0;

    spicelib::SCTY01(save.CLKID, &mut save.CLKTYP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCTY01: exercise pass 1 logic after error", ctx)?;

    //
    // Cause error:
    //
    // Note: change of clock
    //
    save.CLKID = 1;

    save.ET = 1.0;

    spicelib::SCTY01(save.CLKID, &mut save.CLKTYP, ctx)?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Check recovery.
    //
    save.CLKID = -999;

    spicelib::SCTY01(save.CLKID, &mut save.CLKTYP, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"CLKTYP", save.CLKTYP, b"=", 1, 0, OK, ctx)?;

    //
    // Check use of database for second call using same clock,
    // with no kernel pool update.
    //
    save.CLKID = -999;

    spicelib::SCTY01(save.CLKID, &mut save.CLKTYP, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"CLKTYP", save.CLKTYP, b"=", 1, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCTY01: ZZSCAD01 error", ctx)?;

    //
    // Cause error:
    //
    // Note: change of clock
    //
    save.CLKID = -999;

    save.ET = 1.0;

    spicelib::DVPOOL(b"SCLK01_MODULI_999", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCTY01(save.CLKID, &mut save.CLKTYP, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Restore data.
    //
    spicelib::LDPOOL(SCLK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check recovery.
    //
    save.CLKID = -999;

    spicelib::SCTY01(save.CLKID, &mut save.CLKTYP, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"CLKTYP", save.CLKTYP, b"=", 1, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLI01: number of fields not found", ctx)?;

    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK01_N_FIELDS");
    spicelib::SCLI01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.IVAL),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLI01: output delimiter code not found", ctx)?;

    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK01_OUTPUT_DELIM");
    spicelib::SCLI01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.IVAL),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLI01: time system code not found", ctx)?;

    //
    // NOTE: time system code is optional. If not present, the number
    // of values returned should be zero. No error is signaled.
    //
    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK01_TIME_SYSTEM");
    spicelib::SCLI01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.IVAL),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLI01: clock data type not found", ctx)?;

    //
    // NOTE: SCLU01 doesn't have size or range checks for the clock
    // type.
    //
    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK_DATA_TYPE");
    spicelib::SCLI01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.IVAL),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLI01: data item has character type", ctx)?;

    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK01_N_FIELDS");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_-1", 0, &mut save.KVNAME);

    spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(b"XYZ"), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLI01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        save.IVALS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADKERNELVARTYPE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLD01: coefficients not found", ctx)?;

    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK01_COEFFICIENTS");
    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        save.COEFFS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLD01: partition starts not found", ctx)?;

    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK_PARTITION_START");
    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        save.PSTART.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLD01: partition stops not found", ctx)?;

    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK_PARTITION_STOP");
    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        save.PSTOP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLD01: moduli not found", ctx)?;

    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK01_MODULI");
    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        save.MODULI.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLD01: offsets not found", ctx)?;

    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK01_OFFSETS");
    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        save.OFFSET.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLD01: kernel ID not found", ctx)?;

    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK01_KERNEL_ID");
    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.KERNID),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLD01: data item has character type", ctx)?;

    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK01_MODULI");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_-1", 0, &mut save.KVNAME);

    spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(b"XYZ"), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.KERNID),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADKERNELVARTYPE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLI01: output array size check", ctx)?;

    //
    // We need try only one kernel variable for this test.
    //
    save.CLKID = -9;

    fstr::assign(&mut save.ITEM, b"SCLK01_N_FIELDS");

    spicelib::SCLI01(
        &save.ITEM,
        save.CLKID,
        0,
        &mut save.N,
        std::slice::from_mut(&mut save.IVAL),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ARRAYTOOSMALL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLD01: output array size check", ctx)?;

    //
    // We need try only one kernel variable for this test.
    //
    save.CLKID = -9;

    fstr::assign(&mut save.ITEM, b"SCLK01_COEFFICIENTS");

    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        0,
        &mut save.N,
        save.COEFFS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ARRAYTOOSMALL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SCLI01: output delimiter kernel variable has size > 1 where scalar is expected.",
        ctx,
    )?;

    save.CLKID = -9;

    save.ET = 1.0;

    fstr::assign(&mut save.ITEM, b"SCLK01_OUTPUT_DELIM");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_9", 0, &mut save.KVNAME);

    save.IVALS[1] = 1;
    save.IVALS[2] = 2;

    spicelib::PIPOOL(&save.KVNAME, 2, save.IVALS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLI01(
        &save.ITEM,
        save.CLKID,
        IVALSZ,
        &mut save.N,
        save.IVALS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SIZEOUTOFRANGE)", OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SCLI01: field count kernel variable has size > 1 where scalar is expected.",
        ctx,
    )?;

    save.CLKID = -9;

    save.ET = 1.0;

    fstr::assign(&mut save.ITEM, b"SCLK01_N_FIELDS");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_9", 0, &mut save.KVNAME);

    save.IVALS[1] = 1;
    save.IVALS[2] = 2;

    spicelib::PIPOOL(&save.KVNAME, 2, save.IVALS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLI01(
        &save.ITEM,
        save.CLKID,
        IVALSZ,
        &mut save.N,
        save.IVALS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SIZEOUTOFRANGE)", OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SCLI01: time system kernel variable has size > 1 where scalar is expected.",
        ctx,
    )?;

    save.CLKID = -9;

    save.ET = 1.0;

    fstr::assign(&mut save.ITEM, b"SCLK01_TIME_SYSTEM");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_9", 0, &mut save.KVNAME);

    save.IVALS[1] = 1;
    save.IVALS[2] = 2;

    spicelib::PIPOOL(&save.KVNAME, 2, save.IVALS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLI01(
        &save.ITEM,
        save.CLKID,
        IVALSZ,
        &mut save.N,
        save.IVALS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SIZEOUTOFRANGE)", OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SCLD01: kernel ID kernel variable has size > 1 where scalar is expected.",
        ctx,
    )?;

    save.CLKID = -9;

    save.ET = 1.0;

    fstr::assign(&mut save.ITEM, b"SCLK01_KERNEL_ID");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_9", 0, &mut save.KVNAME);

    save.DVALS[1] = 1.0;
    save.DVALS[2] = 2.0;

    spicelib::PDPOOL(&save.KVNAME, 2, save.DVALS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        DVALSZ,
        &mut save.N,
        save.DVALS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SIZEOUTOFRANGE)", OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLD01: moduli kernel variable has size > MXNFLD.", ctx)?;

    save.CLKID = -9;

    save.ET = 1.0;

    fstr::assign(&mut save.ITEM, b"SCLK01_MODULI");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_9", 0, &mut save.KVNAME);

    spicelib::FILLD(10.0, DVALSZ, save.DVALS.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, DVALSZ, save.DVALS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        DVALSZ,
        &mut save.N,
        save.DVALS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SIZEOUTOFRANGE)", OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLD01: offset kernel variable has size > MXNFLD.", ctx)?;

    save.CLKID = -9;

    save.ET = 1.0;

    fstr::assign(&mut save.ITEM, b"SCLK01_OFFSETS");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_9", 0, &mut save.KVNAME);

    spicelib::FILLD(10.0, DVALSZ, save.DVALS.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, DVALSZ, save.DVALS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        DVALSZ,
        &mut save.N,
        save.DVALS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SIZEOUTOFRANGE)", OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SCLD01: partition start kernel variable has size > MXPART.",
        ctx,
    )?;

    save.CLKID = -9;

    save.ET = 1.0;

    fstr::assign(&mut save.ITEM, b"SCLK_PARTITION_START");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_9", 0, &mut save.KVNAME);

    spicelib::FILLD(10.0, (MXPART + 1), save.XSTART.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, (MXPART + 1), save.XSTART.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        (MXPART + 1),
        &mut save.N,
        save.PSTART.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SIZEOUTOFRANGE)", OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SCLD01: partition stop kernel variable has size > MXPART.",
        ctx,
    )?;

    save.CLKID = -9;

    save.ET = 1.0;

    fstr::assign(&mut save.ITEM, b"SCLK_PARTITION_END");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_9", 0, &mut save.KVNAME);

    spicelib::FILLD(10.0, (MXPART + 1), save.XSTOP.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, (MXPART + 1), save.XSTOP.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        (MXPART + 1),
        &mut save.N,
        save.PSTOP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SIZEOUTOFRANGE)", OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLD01: coefficient kernel variable size check", ctx)?;

    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK01_COEFFICIENTS");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_-1", 0, &mut save.KVNAME);

    spicelib::CLEARD((3 * (MXCOEF + 1)), save.COEFFS.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.COEFFS.as_slice_mut());

    //
    // Try a coefficient set that's too small.
    //
    spicelib::PDPOOL(&save.KVNAME, 1, save.COEFFS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        3,
        &mut save.N,
        save.COEFFS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SIZEOUTOFRANGE)", OK, ctx)?;

    //
    // Try a coefficient set that's too large.
    //
    spicelib::PDPOOL(
        &save.KVNAME,
        (3 * (MXCOEF + 1)),
        save.COEFFS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        (3 * (MXCOEF + 1)),
        &mut save.N,
        save.COEFFS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SIZEOUTOFRANGE)", OK, ctx)?;

    //
    // Try a coefficient set that has size not a multiple of 3.
    //
    spicelib::PDPOOL(&save.KVNAME, 5, save.COEFFS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        MXCOEF,
        &mut save.N,
        save.COEFFS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSIZE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLI01: delimiter code out of range", ctx)?;

    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK01_OUTPUT_DELIM");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_-1", 0, &mut save.KVNAME);

    save.IVAL = 0;

    spicelib::PIPOOL(&save.KVNAME, 1, &[save.IVAL], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLI01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.IVAL),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.IVAL = (NDELIM + 1);

    spicelib::PIPOOL(&save.KVNAME, 1, &[save.IVAL], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLI01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.IVAL),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLI01: field count out of range", ctx)?;

    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK01_N_FIELDS");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_-1", 0, &mut save.KVNAME);

    save.IVAL = 0;

    spicelib::PIPOOL(&save.KVNAME, 1, &[save.IVAL], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLI01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.IVAL),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.IVAL = (MXNFLD + 1);

    spicelib::PIPOOL(&save.KVNAME, 1, &[save.IVAL], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLI01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.IVAL),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLI01: time system code out of range", ctx)?;

    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK01_TIME_SYSTEM");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_-1", 0, &mut save.KVNAME);

    save.IVAL = 0;

    spicelib::PIPOOL(&save.KVNAME, 1, &[save.IVAL], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLI01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.IVAL),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.IVAL = (MXTSYS + 1);

    spicelib::PIPOOL(&save.KVNAME, 1, &[save.IVAL], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLI01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.IVAL),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SCLD01: offset kernel variable has size not equal to field count.",
        ctx,
    )?;

    save.CLKID = -9;

    save.ET = 1.0;

    fstr::assign(&mut save.ITEM, b"SCLK01_OFFSETS");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_9", 0, &mut save.KVNAME);

    spicelib::FILLD(0.0, DVALSZ, save.DVALS.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, 1, save.DVALS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        DVALSZ,
        &mut save.N,
        save.DVALS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSIZE)", OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLD01: offset kernel variable size comparison fails due to missing field count variable.", ctx)?;

    save.CLKID = -9;

    save.ET = 1.0;

    fstr::assign(&mut save.ITEM, b"SCLK01_N_FIELDS");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_9", 0, &mut save.KVNAME);

    spicelib::DVPOOL(&save.KVNAME, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.ITEM, b"SCLK01_OFFSETS");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_9", 0, &mut save.KVNAME);

    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        DVALSZ,
        &mut save.N,
        save.DVALS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SCLD01: moduli kernel variable has size not equal to field count.",
        ctx,
    )?;

    save.CLKID = -9;

    save.ET = 1.0;

    fstr::assign(&mut save.ITEM, b"SCLK01_MODULI");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_9", 0, &mut save.KVNAME);

    spicelib::FILLD(10.0, DVALSZ, save.DVALS.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, 1, save.DVALS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        DVALSZ,
        &mut save.N,
        save.DVALS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSIZE)", OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLD01: moduli kernel variable size comparison fails due to missing field count variable.", ctx)?;

    save.CLKID = -9;

    save.ET = 1.0;

    fstr::assign(&mut save.ITEM, b"SCLK01_N_FIELDS");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_9", 0, &mut save.KVNAME);

    spicelib::DVPOOL(&save.KVNAME, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.ITEM, b"SCLK01_MODULI");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_9", 0, &mut save.KVNAME);

    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        DVALSZ,
        &mut save.N,
        save.DVALS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Restore kernel variables for this clock.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLD01: invalid moduli check", ctx)?;

    save.CLKID = 1;

    fstr::assign(&mut save.ITEM, b"SCLK01_MODULI");

    fstr::assign(&mut save.KVNAME, &save.ITEM);
    spicelib::SUFFIX(b"_-1", 0, &mut save.KVNAME);

    spicelib::VPACK(10.0, 0.0, 10.0, save.MODULI.as_slice_mut());

    spicelib::PDPOOL(&save.KVNAME, 3, save.MODULI.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        3,
        &mut save.N,
        save.MODULI.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"All SCLK routines: exercise return on entry logic", ctx)?;

    fstr::assign(&mut save.SHRTMS, b"SPICE(TESTMESSAGE)");
    //
    // Signal an error. We'll try to recover this short message
    // after calling a sequence of SCLK routines.
    //
    save.CLKID = -9;
    fstr::assign(&mut save.CLKNAM, b"-9 SCLK");
    fstr::assign(&mut save.CLKSTR, b"1/000000000.0000");
    save.SCLKDP = 0.0;
    save.ET = 0.0;

    spicelib::SIGERR(&save.SHRTMS, ctx)?;

    spicelib::SC01(
        save.CLKID,
        &save.CLKSTR,
        save.TICKS,
        save.SCLKDP,
        save.ET,
        save.NPARTS,
        save.PSTART.as_slice(),
        save.PSTOP.as_slice(),
        save.CLKTYP,
        ctx,
    )?;
    spicelib::SCDECD(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    spicelib::SCE2C(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;
    spicelib::SCE2S(save.CLKID, save.ET, &mut save.CLKSTR, ctx)?;
    spicelib::SCE2T(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;
    spicelib::SCEC01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;
    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    spicelib::SCET01(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;
    spicelib::SCFM01(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    spicelib::SCFMT(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    spicelib::SCID2N(save.CLKID, &mut save.CLKNAM, &mut save.FOUND, ctx)?;
    spicelib::SCLD01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        save.DVALS.as_slice_mut(),
        ctx,
    )?;
    spicelib::SCLI01(
        &save.ITEM,
        save.CLKID,
        1,
        &mut save.N,
        save.IVALS.as_slice_mut(),
        ctx,
    )?;
    spicelib::SCLU01(
        b"XXX",
        1,
        10,
        save.N,
        save.FIELD.as_slice(),
        save.PSTART.as_slice(),
        ctx,
    )?;
    spicelib::SCN2ID(&save.CLKNAM, &mut save.CLKID, &mut save.FOUND, ctx)?;
    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    spicelib::SCPART(
        save.CLKID,
        &mut save.NPARTS,
        save.PSTART.as_slice_mut(),
        save.PSTOP.as_slice_mut(),
        ctx,
    )?;
    spicelib::SCPR01(
        save.CLKID,
        &mut save.NPARTS,
        save.PSTART.as_slice_mut(),
        save.PSTOP.as_slice_mut(),
        ctx,
    )?;
    spicelib::SCPS01(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    spicelib::SCS2E(save.CLKID, &save.CLKSTR, &mut save.ET, ctx)?;
    spicelib::SCT2E(save.CLKID, save.SCLKDP, &mut save.ET, ctx)?;
    spicelib::SCTE01(save.CLKID, save.SCLKDP, &mut save.ET, ctx)?;
    spicelib::SCTIKS(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    spicelib::SCTK01(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    spicelib::SCTRAN(b"XYZ", save.CLKID, save.FOUND, ctx)?;
    spicelib::SCTY01(save.CLKID, &mut save.I, ctx)?;
    save.I = spicelib::SCTYPE(save.CLKID, ctx)?;

    testutil::CHCKXC(true, &save.SHRTMS, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SC01: call to umbrella routine", ctx)?;

    //
    // Inputs and outputs are arbitrary in this call.
    //
    spicelib::SC01(
        save.CLKID,
        &save.CLKSTR,
        save.TICKS,
        save.SCLKDP,
        save.ET,
        save.NPARTS,
        save.PSTART.as_slice(),
        save.PSTOP.as_slice(),
        save.CLKTYP,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BOGUSENTRY)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCLU01: call to umbrella routine", ctx)?;

    //
    // Inputs and outputs are arbitrary in this call.
    //
    spicelib::SCLU01(
        b"XXX",
        1,
        10,
        save.N,
        save.FIELD.as_slice(),
        save.PSTART.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BOGUSENTRY)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCTRAN: call to umbrella routine", ctx)?;

    //
    // Inputs and outputs are arbitrary in this call.
    //
    save.CLKID = 1;
    spicelib::SCTRAN(b"XYZ", save.CLKID, save.FOUND, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BOGUSENTRY)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::KCLEAR(ctx)?;

    spicelib::DELFIL(CK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // CALL BYEBYE (' ' )

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
