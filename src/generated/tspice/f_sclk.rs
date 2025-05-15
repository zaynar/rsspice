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
const SCLK: &[u8] = b"test.tsc";
const SCLK2: &[u8] = b"test2.tsc";
const SCLK3: &[u8] = b"test3.tsc";
const VTIGHT: f64 = 0.00000000000001;
const MED: f64 = 0.000000001;
const MSGLEN: i32 = 240;
const LNSIZE: i32 = 80;
const NAMLEN: i32 = 41;
const TIMLEN: i32 = 40;
const LBCELL: i32 = -5;
const NCOM: i32 = 3;
const DELIM: &[u8] = b":";
const FLDWID: i32 = 25;

struct SaveVars {
    CLKNAM: Vec<u8>,
    COMS: ActualCharArray,
    DLCHRS: Vec<u8>,
    ERRMSG: Vec<u8>,
    FSTR: ActualCharArray,
    TITLE: Vec<u8>,
    CLKSTR: Vec<u8>,
    XCLKST: Vec<u8>,
    CLKOFF: f64,
    COEFFS: ActualArray2D<f64>,
    DELTA: f64,
    DIVDND: f64,
    DIVSR: StackArray<f64, 4>,
    ET: f64,
    ET0: f64,
    ETOFF: f64,
    MAJOR: f64,
    MODULI: StackArray<f64, 10>,
    OFFSET: StackArray<f64, 10>,
    PSTART: ActualArray<f64>,
    PSTOP: ActualArray<f64>,
    TDT: f64,
    SCLKDP: f64,
    XCLKDP: f64,
    XET: f64,
    XSTART: ActualArray<f64>,
    XSTOP: ActualArray<f64>,
    XTDT: f64,
    CLKID: i32,
    CLKTYP: i32,
    DELCDE: i32,
    FIELD: StackArray<i32, 4>,
    HANDLE: i32,
    I: i32,
    J: i32,
    N: i32,
    NCOEFF: i32,
    NFIELD: i32,
    NPARTS: i32,
    PARTNO: i32,
    Q: i32,
    REM: i32,
    W: i32,
    ERROR: bool,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CLKNAM = vec![b' '; NAMLEN as usize];
        let mut COMS = ActualCharArray::new(LNSIZE, LBCELL..=NCOM);
        let mut DLCHRS = vec![b' '; NDELIM as usize];
        let mut ERRMSG = vec![b' '; MSGLEN as usize];
        let mut FSTR = ActualCharArray::new(FLDWID, 1..=4);
        let mut TITLE = vec![b' '; MSGLEN as usize];
        let mut CLKSTR = vec![b' '; TIMLEN as usize];
        let mut XCLKST = vec![b' '; TIMLEN as usize];
        let mut CLKOFF: f64 = 0.0;
        let mut COEFFS = ActualArray2D::<f64>::new(1..=3, 1..=MXCOEF);
        let mut DELTA: f64 = 0.0;
        let mut DIVDND: f64 = 0.0;
        let mut DIVSR = StackArray::<f64, 4>::new(1..=4);
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ETOFF: f64 = 0.0;
        let mut MAJOR: f64 = 0.0;
        let mut MODULI = StackArray::<f64, 10>::new(1..=MXNFLD);
        let mut OFFSET = StackArray::<f64, 10>::new(1..=MXNFLD);
        let mut PSTART = ActualArray::<f64>::new(1..=MXPART);
        let mut PSTOP = ActualArray::<f64>::new(1..=MXPART);
        let mut TDT: f64 = 0.0;
        let mut SCLKDP: f64 = 0.0;
        let mut XCLKDP: f64 = 0.0;
        let mut XET: f64 = 0.0;
        let mut XSTART = ActualArray::<f64>::new(1..=MXPART);
        let mut XSTOP = ActualArray::<f64>::new(1..=MXPART);
        let mut XTDT: f64 = 0.0;
        let mut CLKID: i32 = 0;
        let mut CLKTYP: i32 = 0;
        let mut DELCDE: i32 = 0;
        let mut FIELD = StackArray::<i32, 4>::new(1..=4);
        let mut HANDLE: i32 = 0;
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut N: i32 = 0;
        let mut NCOEFF: i32 = 0;
        let mut NFIELD: i32 = 0;
        let mut NPARTS: i32 = 0;
        let mut PARTNO: i32 = 0;
        let mut Q: i32 = 0;
        let mut REM: i32 = 0;
        let mut W: i32 = 0;
        let mut ERROR: bool = false;
        let mut FOUND: bool = false;

        fstr::assign(&mut DLCHRS, DELIMS);

        Self {
            CLKNAM,
            COMS,
            DLCHRS,
            ERRMSG,
            FSTR,
            TITLE,
            CLKSTR,
            XCLKST,
            CLKOFF,
            COEFFS,
            DELTA,
            DIVDND,
            DIVSR,
            ET,
            ET0,
            ETOFF,
            MAJOR,
            MODULI,
            OFFSET,
            PSTART,
            PSTOP,
            TDT,
            SCLKDP,
            XCLKDP,
            XET,
            XSTART,
            XSTOP,
            XTDT,
            CLKID,
            CLKTYP,
            DELCDE,
            FIELD,
            HANDLE,
            I,
            J,
            N,
            NCOEFF,
            NFIELD,
            NPARTS,
            PARTNO,
            Q,
            REM,
            W,
            ERROR,
            FOUND,
        }
    }
}

//$Procedure F_SCLK ( SCLK tests )
pub fn F_SCLK(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_SCLK", ctx)?;

    //*****************************************************************
    //
    //     Error cases
    //
    //*****************************************************************

    //
    // Check handling of lookup failures. Routines covered below:
    //
    //
    //    SCDECD
    //    SCE2C
    //    SCE2S
    //    SCENCD
    //    SCPART
    //    SCS2E
    //    SCTYPE
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check handling of lookup failure in SCPART.", ctx)?;

    //
    // We should see a lookup failure error on two successive calls
    // to SCPART if no data are available for a given clock.
    //
    save.CLKID = -888;

    spicelib::SCPART(
        save.CLKID,
        &mut save.N,
        save.PSTART.as_slice_mut(),
        save.PSTOP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    spicelib::SCPART(
        save.CLKID,
        &mut save.N,
        save.PSTART.as_slice_mut(),
        save.PSTOP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check handling of lookup failure in SCTYPE.", ctx)?;

    //
    // We should see a lookup failure error on two successive calls
    // to SCTYPE if no data are available for a given clock.
    //
    save.CLKID = -888;

    save.CLKTYP = spicelib::SCTYPE(save.CLKID, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    save.CLKTYP = spicelib::SCTYPE(save.CLKID, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Suppress "unused variable" warnings.
    //
    save.CLKTYP = spicelib::TOUCHI(save.CLKTYP);

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check handling of lookup failure in SCENCD.", ctx)?;

    //
    // We should see a lookup failure error on two successive calls
    // to SCENCD if no data are available for a given clock.
    //
    save.CLKID = -888;
    fstr::assign(&mut save.CLKSTR, b"1/987654321.4321");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check handling of lookup failure in SCDECD.", ctx)?;

    //
    // We should see a lookup failure error on two successive calls
    // to SCENCD if no data are available for a given clock.
    //
    save.CLKID = -888;
    save.SCLKDP = 1000.0;

    spicelib::SCDECD(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    spicelib::SCDECD(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check handling of lookup failure in SCS2E.", ctx)?;

    //
    // We should see a lookup failure error on two successive calls
    // to SCS2E if no data are available for a given clock.
    //
    save.CLKID = -888;
    save.SCLKDP = 1000.0;

    fstr::assign(&mut save.CLKSTR, b"1/987654321.4321");

    spicelib::SCS2E(save.CLKID, &save.CLKSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    spicelib::SCS2E(save.CLKID, &save.CLKSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check handling of lookup failure in SCE2S.", ctx)?;

    //
    // We should see a lookup failure error on two successive calls
    // to SCS if no data are available for a given clock.
    //
    save.CLKID = -888;
    save.SCLKDP = 1000.0;

    fstr::assign(&mut save.CLKSTR, b"1/987654321.4321");

    spicelib::SCE2S(save.CLKID, save.ET, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    spicelib::SCE2S(save.CLKID, save.ET, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check handling of lookup failure in SCE2C.", ctx)?;

    //
    // We should see a lookup failure error on two successive calls
    // to SCS if no data are available for a given clock.
    //
    save.CLKID = -888;
    save.SCLKDP = 1000.0;

    fstr::assign(&mut save.CLKSTR, b"1/987654321.4321");

    spicelib::SCE2C(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    spicelib::SCE2C(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

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

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check clock type via SCTYPE", ctx)?;

    save.CLKID = -9;

    save.I = spicelib::SCTYPE(save.CLKID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"Clock type", save.I, b"=", 1, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Fetch partition boundaries via SCPART", ctx)?;

    save.CLKID = -9;

    spicelib::SCPART(
        save.CLKID,
        &mut save.NPARTS,
        save.PSTART.as_slice_mut(),
        save.PSTOP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NPARTS", save.NPARTS, b"=", 1, 0, OK, ctx)?;

    //
    // These values come from the SCLK kernel described above.
    //
    save.XSTART[1] = 0.0;
    save.XSTOP[1] = 100000000000000.0;

    testutil::CHCKAD(
        b"PSTART",
        save.PSTART.as_slice(),
        b"=",
        save.XSTART.as_slice(),
        save.NPARTS,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"PSTOP",
        save.PSTOP.as_slice(),
        b"=",
        save.XSTOP.as_slice(),
        save.NPARTS,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Encode SCLK strings via SCENCD", ctx)?;

    save.CLKID = -9;
    fstr::assign(&mut save.CLKSTR, b"1/987654321.4321");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Note that we're expecting exact agreement.
    //
    save.XCLKDP = 9876543214321.0;
    testutil::CHCKSD(b"1) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // Now repeat the test, this time excluding the partition
    // number.
    //
    fstr::assign(&mut save.CLKSTR, b"987654321.4321");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 9876543214321.0;
    testutil::CHCKSD(b"2) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // Now repeat the test, this time excluding the partition
    // number and using a smaller count in the leftmost field.
    //
    fstr::assign(&mut save.CLKSTR, b"4321.4321");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 43214321.0;
    testutil::CHCKSD(b"3) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Encode SCLK strings via SCENCD using all supported delimiters.",
        ctx,
    )?;

    save.CLKID = -9;
    fstr::assign(&mut save.CLKSTR, b"1/987654321.4321");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Note that we're expecting exact agreement.
    //
    save.XCLKDP = 9876543214321.0;
    testutil::CHCKSD(b"1) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"1/987654321:4321");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    save.XCLKDP = 9876543214321.0;
    testutil::CHCKSD(b"2) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"1/987654321-4321");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    save.XCLKDP = 9876543214321.0;
    testutil::CHCKSD(b"3) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"1/987654321,4321");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    save.XCLKDP = 9876543214321.0;
    testutil::CHCKSD(b"4) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"1/987654321 4321");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    save.XCLKDP = 9876543214321.0;
    testutil::CHCKSD(b"5) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Encode SCLK strings via SCPARS", ctx)?;

    save.CLKID = -9;
    fstr::assign(&mut save.CLKSTR, b"1/987654321.4321");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"1) ERROR", save.ERROR, false, OK, ctx)?;

    testutil::CHCKSC(b"1) ERRMSG", &save.ERRMSG, b"=", b" ", OK, ctx)?;

    //
    // Note that we're expecting exact agreement.
    //
    save.XCLKDP = 9876543214321.0;
    testutil::CHCKSD(b"1) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // Now repeat the test, this time excluding the partition
    // number.
    //
    fstr::assign(&mut save.CLKSTR, b"987654321.4321");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"2) ERROR", save.ERROR, false, OK, ctx)?;

    testutil::CHCKSC(b"2) ERRMSG", &save.ERRMSG, b"=", b" ", OK, ctx)?;

    save.XCLKDP = 9876543214321.0;
    testutil::CHCKSD(b"2) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // Now repeat the test, this time excluding the partition
    // number and using a smaller count in the leftmost field.
    //
    fstr::assign(&mut save.CLKSTR, b"4321.4321");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 43214321.0;
    testutil::CHCKSD(b"3) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Encode SCLK strings via SCPARS, using partitions greater than 1.",
        ctx,
    )?;

    save.CLKID = -99;
    fstr::assign(&mut save.CLKSTR, b"2/0000000:0:0:2");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"1) ERROR", save.ERROR, false, OK, ctx)?;

    testutil::CHCKSC(b"1) ERRMSG", &save.ERRMSG, b"=", b" ", OK, ctx)?;

    //
    // Note that we're expecting exact agreement.
    //
    save.XCLKDP = 720000.0;
    testutil::CHCKSD(b"1) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    save.CLKID = -999;
    fstr::assign(&mut save.CLKSTR, b"3/0000005:3:2:4");

    spicelib::SCPARS(
        save.CLKID,
        &save.CLKSTR,
        &mut save.ERROR,
        &mut save.ERRMSG,
        &mut save.SCLKDP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"2) ERROR", save.ERROR, false, OK, ctx)?;

    testutil::CHCKSC(b"2) ERRMSG", &save.ERRMSG, b"=", b" ", OK, ctx)?;

    //
    // Note that we're expecting exact agreement.
    //
    save.XCLKDP = 1440720.0;
    testutil::CHCKSD(b"2) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Encode an invalid SCLK string via SCPARS", ctx)?;

    save.CLKID = -9;
    fstr::assign(&mut save.CLKSTR, b"1/987654321.XXXX");

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

    testutil::CHCKSC(
        b"ERRMSG",
        &save.ERRMSG,
        b"=",
        b"Could not parse SCLK component XXXX from 987654321.XXXX as a number.",
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Encode SCLK duration strings via SCTIKS", ctx)?;

    save.CLKID = -9;

    fstr::assign(&mut save.CLKSTR, b"987654321.4321");

    spicelib::SCTIKS(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that we're expecting exact agreement.
    //
    save.XCLKDP = 9876543214321.0;
    testutil::CHCKSD(b"1) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // Now repeat the test, this time using a smaller count
    // in the leftmost field.
    //
    fstr::assign(&mut save.CLKSTR, b"4321.4321");

    spicelib::SCTIKS(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 43214321.0;
    testutil::CHCKSD(b"2) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Decode SCLK tick values via SCDECD", ctx)?;

    save.CLKID = -9;
    fstr::assign(&mut save.CLKSTR, b"1/987654321.4321");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCDECD(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // We're expecting exact agreement.
    //
    fstr::assign(&mut save.XCLKST, b"1/987654321.4321");
    testutil::CHCKSC(b"1) CLKSTR", &save.CLKSTR, b"=", &save.XCLKST, OK, ctx)?;

    //
    // Now repeat the test, this time using smaller
    // tick value.
    //
    fstr::assign(&mut save.CLKSTR, b"4321.4321");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCDECD(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // We're expecting exact agreement.
    //
    fstr::assign(&mut save.XCLKST, b"1/000004321.4321");
    testutil::CHCKSC(b"2) CLKSTR", &save.CLKSTR, b"=", &save.XCLKST, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Decode SCLK tick values via SCDECD using each supported output delimiter",
        ctx,
    )?;

    save.CLKID = -9;

    //
    // Fetch the current output delimiter code.
    //
    spicelib::GIPOOL(
        b"SCLK01_OUTPUT_DELIM_9",
        1,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.DELCDE),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"1/987654321.4321");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NDELIM;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // The delimiter array declared in sclk.inc has been
            // ordered so that the Ith character has code I.
            // Insert this code into the kernel pool.
            //
            spicelib::PIPOOL(b"SCLK01_OUTPUT_DELIM_9", 1, &[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Create the expected output string.
            //
            fstr::assign(&mut save.XCLKST, b"1/987654321#4321");

            spicelib::REPLCH(
                &save.XCLKST.to_vec(),
                b"#",
                fstr::substr(&save.DLCHRS, save.I..=save.I),
                &mut save.XCLKST,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SCDECD(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // We're expecting exact agreement.
            //
            fstr::assign(&mut save.TITLE, b"#) CLKSTR");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);
            testutil::CHCKSC(&save.TITLE, &save.CLKSTR, b"=", &save.XCLKST, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Restore the original output delimiter code.
    //
    spicelib::PIPOOL(b"SCLK01_OUTPUT_DELIM_9", 1, &[save.DELCDE], ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Decode SCLK tick values via SCFMT", ctx)?;

    save.CLKID = -9;

    fstr::assign(&mut save.CLKSTR, b"987654321.4321");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCFMT(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // We're expecting exact agreement.
    //
    fstr::assign(&mut save.XCLKST, b"987654321.4321");
    testutil::CHCKSC(b"CLKSTR", &save.CLKSTR, b"=", &save.XCLKST, OK, ctx)?;

    //
    // Now repeat the test, this time using smaller
    // tick value.
    //
    fstr::assign(&mut save.CLKSTR, b"4321.4321");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCFMT(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // We're expecting exact agreement.
    //
    fstr::assign(&mut save.XCLKST, b"000004321.4321");
    testutil::CHCKSC(b"2) CLKSTR", &save.CLKSTR, b"=", &save.XCLKST, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Convert ET values to integer ticks via SCE2T", ctx)?;

    save.CLKID = -9;

    spicelib::STR2ET(b"1980 JAN 1 00:00:00 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // First try the ET value corresponding to clock start.
    //
    spicelib::SCE2T(save.CLKID, save.ET0, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 0.0;
    testutil::CHCKSD(b"SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Convert ET values to continuous ticks via SCE2C", ctx)?;

    save.CLKID = -9;

    spicelib::STR2ET(b"1980 JAN 1 00:00:00 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // First try the ET value corresponding to clock start.
    //
    spicelib::SCE2C(save.CLKID, save.ET0, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 0.0;
    testutil::CHCKSD(b"1) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // Now a greater ET value.
    //
    save.DELTA = (100000000.0 + 0.3333);

    save.ET = (save.ET0 + save.DELTA);

    spicelib::SCE2C(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = (save.DELTA * 10000.0);

    //
    // We should get some round-off here, so don't require
    // an exact match.
    //
    testutil::CHCKSD(
        b"2) SCLKDP",
        save.SCLKDP,
        b"~/",
        save.XCLKDP,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Convert continuous ticks to ET via SCT2E", ctx)?;

    save.CLKID = -9;

    spicelib::STR2ET(b"1980 JAN 1 00:00:00 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Use an ET value that corresponds to a fractional tick count.
    //
    save.DELTA = (100000000.0 + 0.3333);

    save.XET = (save.ET0 + save.DELTA);

    spicelib::SCE2C(save.CLKID, save.XET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCT2E(save.CLKID, save.SCLKDP, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should get some round-off here, so don't require
    // an exact match.
    //
    testutil::CHCKSD(b"2) ET", save.ET, b"~/", save.XET, VTIGHT, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Convert an SCLK string to ET via SCS2E", ctx)?;

    save.CLKID = -9;
    fstr::assign(&mut save.CLKSTR, b"1/987654321.4321");

    //
    // Compute the expected value using SCENCD and
    // SCT2E.
    //
    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCT2E(save.CLKID, save.SCLKDP, &mut save.XET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now try the conversion with SCS2E.
    //
    spicelib::SCS2E(save.CLKID, &save.CLKSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We're expecting exact agreement.
    //
    testutil::CHCKSD(b"ET", save.ET, b"=", save.XET, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Convert an ET value to an SCLK string via SCE2S.", ctx)?;

    //
    // Generate the ET value.
    //
    save.CLKID = -9;
    fstr::assign(&mut save.CLKSTR, b"1/987654321.4321");

    spicelib::SCS2E(save.CLKID, &save.CLKSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compute the expected value using SCE2T and SCDECD.
    //
    spicelib::SCE2T(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCDECD(save.CLKID, save.SCLKDP, &mut save.XCLKST, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now try the conversion with SCE2S.
    //
    spicelib::SCE2S(save.CLKID, save.ET, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We're expecting exact agreement.
    //
    testutil::CHCKSC(b"CLKSTR", &save.CLKSTR, b"=", &save.XCLKST, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clock -99: set expected values.", ctx)?;

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

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clock -99: Recover partition bounds via SCPART", ctx)?;

    save.CLKID = -99;

    spicelib::SCPART(
        save.CLKID,
        &mut save.N,
        save.PSTART.as_slice_mut(),
        save.PSTOP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", save.NPARTS, 0, OK, ctx)?;

    testutil::CHCKAD(
        b"PSTART",
        save.PSTART.as_slice(),
        b"=",
        save.XSTART.as_slice(),
        save.NPARTS,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"PSTOP",
        save.PSTOP.as_slice(),
        b"=",
        save.XSTOP.as_slice(),
        save.NPARTS,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clock -99: Convert tick coefficients to SCLK strings via SCDECD. Invert the conversion via SCENCD.", ctx)?;

    save.CLKID = -99;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOEFF;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SCDECD(save.CLKID, save.COEFFS[[1, save.I]], &mut save.CLKSTR, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Derive the expected string based on the SCLK kernel
            // structure: every record whose index is equivalent
            // to 1 mod 4 is at the start of a new partition. Each
            // record is 180000 ticks ahead of the previous one.
            //
            spicelib::RMAINI((save.I - 1), 4, &mut save.Q, &mut save.REM, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // The partition number is simply Q+1.
            //
            save.PARTNO = (save.Q + 1);

            spicelib::INTSTR(save.PARTNO, &mut save.XCLKST, ctx);

            spicelib::SUFFIX(b"/", 0, &mut save.XCLKST);
            //
            // Let J be the number of ticks past the
            // partition start reading. Let SCLKDP
            // be the actual clock reading; we obtain
            // SCLKDP by adding the partition start reading
            // to J.
            //
            save.J = intrinsics::IDNINT(((save.REM as f64) * 180000.0));
            save.SCLKDP = ((save.J as f64) + save.XSTART[save.PARTNO]);
            //
            // Convert SCLKDP to an SCLK string.
            //
            save.DIVSR[1] = 720.0;
            save.DIVSR[2] = 72.0;
            save.DIVSR[3] = 8.0;

            save.DIVDND = save.SCLKDP;

            for K in 1..=3 {
                save.FIELD[K] =
                    (intrinsics::IDNINT(save.DIVDND) / intrinsics::IDNINT(save.DIVSR[K]));

                spicelib::INTSTR(save.FIELD[K], &mut save.FSTR[K], ctx);
                spicelib::LJUST(&save.FSTR[K].to_vec(), &mut save.FSTR[K]);

                save.DIVDND = (save.DIVDND - ((save.FIELD[K] as f64) * save.DIVSR[K]));
            }

            save.FIELD[4] = intrinsics::IDNINT(save.DIVDND);

            spicelib::INTSTR(save.FIELD[4], &mut save.FSTR[4], ctx);

            //
            // Pad the first field on the left with zeros.
            //
            save.W = spicelib::LASTNB(&save.FSTR[1]);

            for K in 1..=(7 - save.W) {
                spicelib::PREFIX(b"0", 0, &mut save.FSTR[1]);
            }
            //
            // Append the field and delimiters to the expected
            // string.
            //
            for K in 1..=3 {
                spicelib::SUFFIX(&save.FSTR[K], 0, &mut save.XCLKST);
                spicelib::SUFFIX(b":", 0, &mut save.XCLKST);
            }

            spicelib::SUFFIX(&save.FSTR[4], 0, &mut save.XCLKST);

            fstr::assign(&mut save.TITLE, b"CLKSTR(#)");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            //
            // Look for an exact match.
            //
            testutil::CHCKSC(&save.TITLE, &save.CLKSTR, b"=", &save.XCLKST, OK, ctx)?;

            //
            // Now convert the string to a tick value.
            //
            spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Compare the tick value to the tick coefficient.
            //
            testutil::CHCKSD(
                b"SCLKDP",
                save.SCLKDP,
                b"=",
                save.COEFFS[[1, save.I]],
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clock -99: Convert tick coefficients to ET via SCT2E", ctx)?;

    save.CLKID = -99;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOEFF;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SCT2E(save.CLKID, save.COEFFS[[1, save.I]], &mut save.ET, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.XET = save.COEFFS[[2, save.I]];

            fstr::assign(&mut save.TITLE, b"ET(#)");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            //
            // Don't look for an exact match, since NPARSD, which is
            // used by the text kernel reader routines, can introduce
            // round-off error in integral inputs (!) depending on
            // their format.
            //
            testutil::CHCKSD(&save.TITLE, save.ET, b"~", save.XET, MED, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Clock -99: Convert tick coefficient midpoints to ET via SCT2E",
        ctx,
    )?;

    save.CLKID = -99;

    //
    // The tick offset value is constant, given the SCLK kernel
    // we've created.
    //
    save.CLKOFF = ((save.COEFFS[[1, 1]] + save.COEFFS[[1, 2]]) / 2 as f64);

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOEFF;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SCLKDP = (save.COEFFS[[1, save.I]] + save.CLKOFF);

            spicelib::SCT2E(save.CLKID, save.SCLKDP, &mut save.ET, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // The ET offset is based on the rate for the previous
            // (Ith) coefficient record.
            //
            save.XET = (save.COEFFS[[2, save.I]]
                + ((save.CLKOFF / save.MAJOR) * save.COEFFS[[3, save.I]]));

            fstr::assign(&mut save.TITLE, b"ET(#)");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            //
            // Look for tight, but not exact, match.
            //
            testutil::CHCKSD(&save.TITLE, save.ET, b"~/", save.XET, VTIGHT, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Clock -99: Convert TDB coefficients to ticks via SCE2T",
        ctx,
    )?;

    save.CLKID = -99;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOEFF;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SCE2T(save.CLKID, save.COEFFS[[2, save.I]], &mut save.SCLKDP, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.XCLKDP = save.COEFFS[[1, save.I]];

            fstr::assign(&mut save.TITLE, b"SCLKDP(#)");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            //
            // Look for an exact match.
            //
            testutil::CHCKSD(&save.TITLE, save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Clock -99: Convert tick coefficient midpoints to ET via SCT2E",
        ctx,
    )?;

    save.CLKID = -99;

    //
    // The tick offset value is constant, given the SCLK kernel
    // we've created.
    //
    save.CLKOFF = ((save.COEFFS[[1, 1]] + save.COEFFS[[1, 2]]) / 2 as f64);

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOEFF;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SCLKDP = (save.COEFFS[[1, save.I]] + save.CLKOFF);

            spicelib::SCT2E(save.CLKID, save.SCLKDP, &mut save.ET, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // The ET offset is based on the rate for the previous
            // (Ith) coefficient record.
            //
            save.XET = (save.COEFFS[[2, save.I]]
                + ((save.CLKOFF / save.MAJOR) * save.COEFFS[[3, save.I]]));

            fstr::assign(&mut save.TITLE, b"ET(#)");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            //
            // Look for tight, but not exact, match.
            //
            testutil::CHCKSD(&save.TITLE, save.ET, b"~/", save.XET, VTIGHT, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Clock -99: Convert ET coefficient midpoints to continuous ticks via SCE2C",
        ctx,
    )?;

    save.CLKID = -99;

    //
    // The ET offset value is constant, given the SCLK kernel
    // we've created.
    //
    save.ETOFF = ((save.COEFFS[[2, 1]] + save.COEFFS[[2, 2]]) / 2 as f64);

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOEFF;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.ET = (save.COEFFS[[2, save.I]] + save.ETOFF);

            spicelib::SCE2C(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // The tick offset is based on the rate for the previous
            // (Ith) coefficient record.
            //
            // The expected tick offset is the ET offset divided
            // by the applicable rate, which has units of seconds
            // per major field, times the ticks per major field.
            //
            save.XCLKDP =
                (save.COEFFS[[1, save.I]] + ((save.MAJOR * save.ETOFF) / save.COEFFS[[3, save.I]]));

            fstr::assign(&mut save.TITLE, b"SCLKDP (#)");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            //
            // Look for tight, but not exact, match.
            //
            testutil::CHCKSD(
                &save.TITLE,
                save.SCLKDP,
                b"~/",
                save.XCLKDP,
                VTIGHT,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Set expected values for clock -999:  ", ctx)?;

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

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clock -999: Encode SCLK strings via SCENCD.", ctx)?;

    save.CLKID = -999;
    fstr::assign(&mut save.CLKSTR, b"1/4:03:02:2");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Note that we're expecting exact agreement.
    //
    save.XCLKDP = 0.0;
    testutil::CHCKSD(b"1) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // Now repeat the test, this time excluding the partition
    // number.
    //
    fstr::assign(&mut save.CLKSTR, b"4:03:02:2");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 0.0;
    testutil::CHCKSD(b"2) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // Use a larger input value.
    //
    fstr::assign(&mut save.CLKSTR, b"1/5:03:02:2");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 720.0;
    testutil::CHCKSD(b"2) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // Use an input value in the second partition.
    //
    fstr::assign(&mut save.CLKSTR, b"2/1393:11:10:3");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 720720.0;
    testutil::CHCKSD(b"2) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Clock -999: Encode SCLK string with overflow field values via SCENCD.",
        ctx,
    )?;

    save.CLKID = -999;
    fstr::assign(&mut save.CLKSTR, b"1/4:03:02:11");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Note that we're expecting exact agreement.
    //
    save.XCLKDP = 9.0;
    testutil::CHCKSD(b"1) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"1/4:13:02:11");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Note that we're expecting exact agreement.
    //
    save.XCLKDP = 729.0;
    testutil::CHCKSD(b"2) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // Overflow in the second and fourth fields:
    //
    fstr::assign(&mut save.CLKSTR, b"1/4:13:02:11");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Note that we're expecting exact agreement.
    //
    save.XCLKDP = 729.0;
    testutil::CHCKSD(b"2) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // Overflow in the second, third and fourth fields:
    //
    fstr::assign(&mut save.CLKSTR, b"1/4:13:12:11");

    spicelib::SCENCD(save.CLKID, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Note that we're expecting exact agreement.
    //
    save.XCLKDP = 809.0;
    testutil::CHCKSD(b"2) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clock -999: Decode SCLK tick values via SCDECD.", ctx)?;

    save.CLKID = -999;
    fstr::assign(&mut save.XCLKST, b"1/00000004:03:02:2");

    spicelib::SCENCD(save.CLKID, &save.XCLKST, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCDECD(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"1) CLKSTR", &save.CLKSTR, b"=", &save.XCLKST, OK, ctx)?;

    //
    // Use a larger input value.
    //
    fstr::assign(&mut save.XCLKST, b"1/00000005:03:02:2");

    spicelib::SCENCD(save.CLKID, &save.XCLKST, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCDECD(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"2) CLKSTR", &save.CLKSTR, b"=", &save.XCLKST, OK, ctx)?;

    //
    // Use an input value in the second partition.
    //
    fstr::assign(&mut save.XCLKST, b"2/00001393:11:10:3");

    spicelib::SCENCD(save.CLKID, &save.XCLKST, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCDECD(save.CLKID, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"3) CLKSTR", &save.CLKSTR, b"=", &save.XCLKST, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Clock -999: Convert TDT values to integer ticks via SCE2T",
        ctx,
    )?;

    save.CLKID = -999;

    spicelib::STR2ET(b"2000 JAN 1 12:00:00 TDT", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // First try the TDT value corresponding to clock start.
    //
    spicelib::SCE2T(save.CLKID, save.ET0, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 0.0;
    testutil::CHCKSD(b"1) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // First now try a greater TDT value: 1000 seconds past clock
    // start.
    //
    spicelib::STR2ET(b"2000 JAN 1 12:16:40 TDT", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCE2T(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = 360000.0;
    testutil::CHCKSD(b"2) SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Clock -999: Convert TDT values to continuous ticks via SCE2C",
        ctx,
    )?;

    save.CLKID = -999;

    spicelib::STR2ET(b"2000 JAN 1 12:00:00.125 TDT", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // First try the TDT value corresponding to 1/8 second
    // past clock start.
    //
    spicelib::SCE2C(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP = ((0.125 / save.COEFFS[[3, 1]]) * 720.0);

    testutil::CHCKSD(
        b"1) SCLKDP",
        save.SCLKDP,
        b"~",
        save.XCLKDP,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Now a greater TDT value.
    //
    save.TDT = (1000.0 + 0.3333);

    save.ET = spicelib::UNITIM(save.TDT, b"TDT", b"TDB", ctx)?;

    spicelib::SCE2C(save.CLKID, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XCLKDP =
        (save.COEFFS[[1, 3]] + (((save.TDT - save.COEFFS[[2, 3]]) / save.COEFFS[[3, 3]]) * 720.0));

    //
    // We should get some round-off here, so don't require
    // an exact match.
    //
    testutil::CHCKSD(
        b"2) SCLKDP",
        save.SCLKDP,
        b"~/",
        save.XCLKDP,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Clock -999: Convert continuous ticks to TDT via SCT2E",
        ctx,
    )?;

    save.CLKID = -999;

    spicelib::STR2ET(b"2000 JAN 1 12:00:00 TDT", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Use a TDT value that corresponds to a fractional
    // tick count.
    //
    save.DELTA = (1000.0 + 0.3333);

    save.XTDT = (spicelib::UNITIM(save.ET0, b"TDB", b"TDT", ctx)? + save.DELTA);
    save.XET = spicelib::UNITIM(save.XTDT, b"TDT", b"TDB", ctx)?;

    spicelib::SCE2C(save.CLKID, save.XET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCT2E(save.CLKID, save.SCLKDP, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TDT = spicelib::UNITIM(save.ET, b"TDB", b"TDT", ctx)?;

    //
    // We should get some round-off here, so don't require
    // an exact match.
    //
    testutil::CHCKSD(b"2) TDT", save.XTDT, b"~/", save.XTDT, VTIGHT, OK, ctx)?;

    //
    // Test entry points of SCTRAN.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCN2ID: convert GLL clock name to ID", ctx)?;

    spicelib::SCN2ID(b"GLL SCLK", &mut save.CLKID, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CLKID", save.CLKID, b"=", -77, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCN2ID: convert unknown clock name to ID", ctx)?;

    save.CLKID = 7;
    spicelib::SCN2ID(b"XYZ SCLK", &mut save.CLKID, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    testutil::CHCKSI(b"CLKID", save.CLKID, b"=", 7, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCN2ID: convert recognized non-clock name to ID", ctx)?;

    spicelib::SCN2ID(
        b"JUPITER BARYCENTER SCLK",
        &mut save.CLKID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CLKID", save.CLKID, b"=", 5, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCID2N: convert CASSINI clock ID to name", ctx)?;

    spicelib::SCID2N(-82, &mut save.CLKNAM, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"CLKNAM", &save.CLKNAM, b"=", b"CASSINI SCLK", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SCID2N: convert unknown clock ID to name", ctx)?;

    spicelib::SCID2N(999999, &mut save.CLKNAM, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

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
