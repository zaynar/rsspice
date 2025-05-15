//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
const CTRSIZ: i32 = 2;
const NDELIM: i32 = 5;
const DELIMS: &[u8; NDELIM as usize] = &fstr::extend_const::<{ NDELIM as usize }>(b".:-, ");
const TDB: i32 = 1;
const TDT: i32 = 2;
const MXPART: i32 = 9999;
const MXCOEF: i32 = 100000;
const MXNFLD: i32 = 10;
const DPLEN: i32 = 30;
const IXNFLD: i32 = 1;
const IXDLIM: i32 = (IXNFLD + 1);
const IXTSYS: i32 = (IXDLIM + 1);
const IXNCOF: i32 = (IXTSYS + 1);
const IXNPRT: i32 = (IXNCOF + 1);
const IXBCOF: i32 = (IXNPRT + 1);
const IXBSTR: i32 = (IXBCOF + 1);
const IXBEND: i32 = (IXBSTR + 1);
const IXBMOD: i32 = (IXBEND + 1);
const IXBOFF: i32 = (IXBMOD + 1);
const NIVALS: i32 = IXBOFF;
const MXNCLK: i32 = 100;
const DBUFSZ: i32 = (((3 * MXCOEF) + (2 * MXPART)) + (2 * MXNFLD));
const IBUFSZ: i32 = (NIVALS * MXNCLK);
const LBSNGL: i32 = -5;
const CPLSIZ: i32 = ((MXNCLK - LBSNGL) + 1);
const CK: &[u8] = b"test.bc";
const DELIM: &[u8] = b":";
const SCLK: &[u8] = b"test.tsc";
const SCLK2: &[u8] = b"test2.tsc";
const SCLK3: &[u8] = b"test3.tsc";
const VTIGHT: f64 = 0.0000000000001;
const DVALSZ: i32 = (MXNFLD + 1);
const LBCELL: i32 = -5;
const LNSIZE: i32 = 80;
const NCOM: i32 = 3;
const TIMLEN: i32 = 40;
const MAXITR: i32 = 10000;
const NCLOCK: i32 = 3;
const KVNMLN: i32 = 32;

struct SaveVars {
    CLKSTR: Vec<u8>,
    COMS: ActualCharArray,
    KVNAME: Vec<u8>,
    LABEL: Vec<u8>,
    SHRTMS: Vec<u8>,
    XCLKST: Vec<u8>,
    COEFFS: ActualArray2D<f64>,
    DELTA: f64,
    DPBUFF: ActualArray<f64>,
    DVALS: StackArray<f64, 11>,
    ET: f64,
    ET0: f64,
    MAJOR: f64,
    MAXMOD: f64,
    MODULI: StackArray<f64, 10>,
    OFFSET: StackArray<f64, 10>,
    SCLKDP: f64,
    STEP: f64,
    TOL: f64,
    XCLKDP: f64,
    XET: f64,
    XOFF: StackArray<f64, 10>,
    XMOD: StackArray<f64, 10>,
    XSTART: ActualArray<f64>,
    XSTOP: ActualArray<f64>,
    XTICKS: ActualArray2D<f64>,
    CLKID: i32,
    CLKLST: StackArray<i32, 100>,
    CLKTYP: i32,
    CLOCKS: StackArray<i32, 3>,
    COFBAS: i32,
    DELCDE: i32,
    DPFREE: i32,
    DSTART: i32,
    ENDBAS: i32,
    HANDLE: i32,
    HDSCLK: StackArray<i32, 100>,
    IFREE: i32,
    INTBUF: ActualArray<i32>,
    ISTART: i32,
    MODBAS: i32,
    NCOEFF: i32,
    NFIELD: i32,
    NITER: i32,
    NPARTS: i32,
    NREC: i32,
    NSCAVL: i32,
    OFFBAS: i32,
    POLCTR: StackArray<i32, 2>,
    PRVSC: i32,
    SC: i32,
    SCBASE: StackArray<i32, 100>,
    SCLKAT: i32,
    SCPOOL: StackArray<i32, 106>,
    STRBAS: i32,
    TIMSYS: i32,
    XTYPE: i32,
    XIBUFF: StackArray<i32, 100>,
    UPDATE: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CLKSTR = vec![b' '; TIMLEN as usize];
        let mut COMS = ActualCharArray::new(LNSIZE, LBCELL..=NCOM);
        let mut KVNAME = vec![b' '; KVNMLN as usize];
        let mut LABEL = vec![b' '; LNSIZE as usize];
        let mut SHRTMS = vec![b' '; SMSGLN as usize];
        let mut XCLKST = vec![b' '; TIMLEN as usize];
        let mut COEFFS = ActualArray2D::<f64>::new(1..=3, 1..=(MXCOEF + 1));
        let mut DELTA: f64 = 0.0;
        let mut DPBUFF = ActualArray::<f64>::new(1..=DBUFSZ);
        let mut DVALS = StackArray::<f64, 11>::new(1..=DVALSZ);
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut MAJOR: f64 = 0.0;
        let mut MAXMOD: f64 = 0.0;
        let mut MODULI = StackArray::<f64, 10>::new(1..=MXNFLD);
        let mut OFFSET = StackArray::<f64, 10>::new(1..=MXNFLD);
        let mut SCLKDP: f64 = 0.0;
        let mut STEP: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut XCLKDP: f64 = 0.0;
        let mut XET: f64 = 0.0;
        let mut XOFF = StackArray::<f64, 10>::new(1..=MXNFLD);
        let mut XMOD = StackArray::<f64, 10>::new(1..=MXNFLD);
        let mut XSTART = ActualArray::<f64>::new(1..=(MXPART + 1));
        let mut XSTOP = ActualArray::<f64>::new(1..=(MXPART + 1));
        let mut XTICKS = ActualArray2D::<f64>::new(1..=MAXITR, 1..=NCLOCK);
        let mut CLKID: i32 = 0;
        let mut CLKLST = StackArray::<i32, 100>::new(1..=MXNCLK);
        let mut CLKTYP: i32 = 0;
        let mut CLOCKS = StackArray::<i32, 3>::new(1..=NCLOCK);
        let mut COFBAS: i32 = 0;
        let mut DELCDE: i32 = 0;
        let mut DPFREE: i32 = 0;
        let mut DSTART: i32 = 0;
        let mut ENDBAS: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut HDSCLK = StackArray::<i32, 100>::new(1..=MXNCLK);
        let mut IFREE: i32 = 0;
        let mut INTBUF = ActualArray::<i32>::new(1..=IBUFSZ);
        let mut ISTART: i32 = 0;
        let mut MODBAS: i32 = 0;
        let mut NCOEFF: i32 = 0;
        let mut NFIELD: i32 = 0;
        let mut NITER: i32 = 0;
        let mut NPARTS: i32 = 0;
        let mut NREC: i32 = 0;
        let mut NSCAVL: i32 = 0;
        let mut OFFBAS: i32 = 0;
        let mut POLCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut PRVSC: i32 = 0;
        let mut SC: i32 = 0;
        let mut SCBASE = StackArray::<i32, 100>::new(1..=MXNCLK);
        let mut SCLKAT: i32 = 0;
        let mut SCPOOL = StackArray::<i32, 106>::new(LBSNGL..=MXNCLK);
        let mut STRBAS: i32 = 0;
        let mut TIMSYS: i32 = 0;
        let mut XTYPE: i32 = 0;
        let mut XIBUFF = StackArray::<i32, 100>::new(1..=MXNCLK);
        let mut UPDATE: bool = false;

        Self {
            CLKSTR,
            COMS,
            KVNAME,
            LABEL,
            SHRTMS,
            XCLKST,
            COEFFS,
            DELTA,
            DPBUFF,
            DVALS,
            ET,
            ET0,
            MAJOR,
            MAXMOD,
            MODULI,
            OFFSET,
            SCLKDP,
            STEP,
            TOL,
            XCLKDP,
            XET,
            XOFF,
            XMOD,
            XSTART,
            XSTOP,
            XTICKS,
            CLKID,
            CLKLST,
            CLKTYP,
            CLOCKS,
            COFBAS,
            DELCDE,
            DPFREE,
            DSTART,
            ENDBAS,
            HANDLE,
            HDSCLK,
            IFREE,
            INTBUF,
            ISTART,
            MODBAS,
            NCOEFF,
            NFIELD,
            NITER,
            NPARTS,
            NREC,
            NSCAVL,
            OFFBAS,
            POLCTR,
            PRVSC,
            SC,
            SCBASE,
            SCLKAT,
            SCPOOL,
            STRBAS,
            TIMSYS,
            XTYPE,
            XIBUFF,
            UPDATE,
        }
    }
}

//$Procedure F_ZZSC01 ( SCLK type 01 database tests )
pub fn F_ZZSC01(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Other parameters
    //

    //
    // Local Variables
    //
    //
    // We use a longer, non-standard length for ABCORR because we
    // want to include embedded blanks for testing.
    //

    //
    // The coefficient array must support a test in which too many
    // coefficients are provided.
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
    testutil::TOPEN(b"F_ZZSC01", ctx)?;

    //*****************************************************************
    //
    //     Normal cases
    //
    //*****************************************************************

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

    spicelib::FURNSH(SCLK2, ctx)?;
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

    spicelib::FURNSH(SCLK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Tests of supporting private routines follow.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Initialize the database.", ctx)?;

    spicelib::ZZSCIN01(
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        &mut save.IFREE,
        &mut save.PRVSC,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The previous SCLK should be zero.
    //
    testutil::CHCKSI(b"PRVSC", save.PRVSC, b"=", 0, 0, OK, ctx)?;

    //
    // The integer and d.p. buffers should be empty: the first free
    // element of each should be at index 1.
    //
    testutil::CHCKSI(b"DPFREE", save.DPFREE, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSI(b"IFREE", save.IFREE, b"=", 1, 0, OK, ctx)?;

    //
    // The clock list should be empty.
    //
    spicelib::FILLI(0, MXNCLK, save.XIBUFF.as_slice_mut());
    testutil::CHCKAI(
        b"CLKLST",
        save.CLKLST.as_slice(),
        b"=",
        save.XIBUFF.as_slice(),
        MXNCLK,
        OK,
        ctx,
    )?;

    //
    // The array of collision list head nodes should be empty.
    //
    testutil::CHCKAI(
        b"HDSCLK",
        save.HDSCLK.as_slice(),
        b"=",
        save.XIBUFF.as_slice(),
        MXNCLK,
        OK,
        ctx,
    )?;

    //
    // The number of free nodes in the collision list pool should be
    // the pool size.
    //
    spicelib::ZZHSIAVL(save.SCPOOL.as_slice(), &mut save.NSCAVL);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NSCAVL", save.NSCAVL, b"=", MXNCLK, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Add data for clock -9 to the database.", ctx)?;

    //
    // Do a conversion using clock -9. This will populate the database.
    //
    save.SC = -9;
    save.ET = 0.0;

    spicelib::SCE2C(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Initialize the pool counter.
    //
    spicelib::ZZCTRUIN(save.POLCTR.as_slice_mut(), ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up information for clock -9.
    //
    spicelib::ZZSCUP01(
        save.SC,
        save.POLCTR.as_slice_mut(),
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        save.DPBUFF.as_slice_mut(),
        &mut save.IFREE,
        save.INTBUF.as_slice_mut(),
        save.SCBASE.as_slice_mut(),
        &mut save.PRVSC,
        &mut save.NFIELD,
        &mut save.DELCDE,
        &mut save.TIMSYS,
        &mut save.NCOEFF,
        &mut save.NPARTS,
        &mut save.COFBAS,
        &mut save.STRBAS,
        &mut save.ENDBAS,
        &mut save.MODBAS,
        &mut save.OFFBAS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of partitions.
    //
    testutil::CHCKSI(b"NPARTS", save.NPARTS, b"=", 1, 0, OK, ctx)?;

    //
    // Check the partitions.
    //
    save.XSTART[1] = 0.0;

    testutil::CHCKAD(
        b"STARTS",
        save.DPBUFF.subarray((save.STRBAS + 1)),
        b"~",
        save.XSTART.as_slice(),
        save.NPARTS,
        VTIGHT,
        OK,
        ctx,
    )?;

    save.XSTOP[1] = 100000000000000.0;

    testutil::CHCKAD(
        b"STOPS",
        save.DPBUFF.subarray((save.ENDBAS + 1)),
        b"~",
        save.XSTOP.as_slice(),
        save.NPARTS,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check the number of individual SCLK coefficients.
    //
    testutil::CHCKSI(b"NCOEFF", save.NCOEFF, b"=", 3, 0, OK, ctx)?;

    //
    // Check the  SCLK coefficients.
    //
    save.COEFFS[[1, 1]] = 0.0;

    spicelib::STR2ET(b"1-JAN-1980 TDB", &mut save.COEFFS[[2, 1]], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::ZZPCTRCK(save.POLCTR.as_slice_mut(), &mut save.UPDATE, ctx);

    save.COEFFS[[3, 1]] = 1.0;

    testutil::CHCKAD(
        b"COEFFS",
        save.DPBUFF.subarray((save.COFBAS + 1)),
        b"~/",
        save.COEFFS.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check the number of clock fields.
    //
    testutil::CHCKSI(b"NFIELD", save.NFIELD, b"=", 2, 0, OK, ctx)?;

    //
    // Check the clock moduli.
    //
    save.XMOD[1] = 1000000000.0;
    save.XMOD[2] = 10000.0;

    testutil::CHCKAD(
        b"MODULI",
        save.DPBUFF.subarray((save.MODBAS + 1)),
        b"=",
        save.XMOD.as_slice(),
        save.NFIELD,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check the clock offsets.
    //
    save.XOFF[1] = 0.0;
    save.XOFF[2] = 0.0;

    testutil::CHCKAD(
        b"OFFSETS",
        save.DPBUFF.subarray((save.OFFBAS + 1)),
        b"=",
        save.XOFF.as_slice(),
        save.NFIELD,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check the time system code.
    //
    testutil::CHCKSI(b"TIMSYS", save.TIMSYS, b"=", 1, 0, OK, ctx)?;

    //
    // Check the delimiter code.
    //
    testutil::CHCKSI(b"DELCDE", save.DELCDE, b"=", 1, 0, OK, ctx)?;

    //
    // Check the first free integer and d.p. indices in the respective
    // data buffers.
    //
    // All clocks use 10 integer buffer entries.
    //
    testutil::CHCKSI(b"IFREE", save.IFREE, b"=", 11, 0, OK, ctx)?;

    testutil::CHCKSI(
        b"DPFREE",
        save.DPFREE,
        b"=",
        (((save.NCOEFF + (2 * save.NPARTS)) + (2 * save.NFIELD)) + 1),
        0,
        OK,
        ctx,
    )?;

    //
    // Save the free values for the next test case.
    //
    save.ISTART = save.IFREE;
    save.DSTART = save.DPFREE;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Add data for clock -99 to the database.", ctx)?;

    //
    // Do a conversion using clock -99. This will populate the database.
    //
    save.SC = -99;
    save.ET = 0.0;

    spicelib::SCE2C(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up information for clock -99.
    //
    spicelib::ZZSCUP01(
        save.SC,
        save.POLCTR.as_slice_mut(),
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        save.DPBUFF.as_slice_mut(),
        &mut save.IFREE,
        save.INTBUF.as_slice_mut(),
        save.SCBASE.as_slice_mut(),
        &mut save.PRVSC,
        &mut save.NFIELD,
        &mut save.DELCDE,
        &mut save.TIMSYS,
        &mut save.NCOEFF,
        &mut save.NPARTS,
        &mut save.COFBAS,
        &mut save.STRBAS,
        &mut save.ENDBAS,
        &mut save.MODBAS,
        &mut save.OFFBAS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of partitions.
    //
    testutil::CHCKSI(b"NPARTS", save.NPARTS, b"=", 1001, 0, OK, ctx)?;

    //
    // Check the partitions.
    //
    for I in 1..=save.NPARTS {
        save.XSTART[I] = (I as f64);
        save.XSTOP[I] = (720000.0 + I as f64);
    }

    testutil::CHCKAD(
        b"STARTS",
        save.DPBUFF.subarray((save.STRBAS + 1)),
        b"=",
        save.XSTART.as_slice(),
        save.NPARTS,
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STOPS",
        save.DPBUFF.subarray((save.ENDBAS + 1)),
        b"=",
        save.XSTOP.as_slice(),
        save.NPARTS,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check the number of individual SCLK coefficients.
    //
    testutil::CHCKSI(b"NCOEFF", save.NCOEFF, b"=", 12003, 0, OK, ctx)?;

    //
    // Check the clock moduli.
    //
    save.XMOD[1] = 10000000.0;
    save.XMOD[2] = 10.0;
    save.XMOD[3] = 9.0;
    save.XMOD[4] = 8.0;

    testutil::CHCKAD(
        b"MODULI",
        save.DPBUFF.subarray((save.MODBAS + 1)),
        b"=",
        save.XMOD.as_slice(),
        save.NFIELD,
        0.0,
        OK,
        ctx,
    )?;
    //
    // Check the  SCLK coefficients.
    //
    save.MAJOR = 1.0;

    for I in 2..=4 {
        save.MAJOR = (save.MAJOR * save.XMOD[I]);
    }

    save.DELTA = 0.000000001;

    for I in 1..=(save.NCOEFF / 3) {
        save.COEFFS[[1, I]] = (((I - 1) as f64) * 180000.0);

        save.COEFFS[[2, I]] = ((save.COEFFS[[1, I]] * 2 as f64) / save.MAJOR);

        save.COEFFS[[3, I]] = (2.0 + ((I as f64) * save.DELTA));
    }

    testutil::CHCKAD(
        b"COEFFS",
        save.DPBUFF.subarray((save.COFBAS + 1)),
        b"~/",
        save.COEFFS.as_slice(),
        save.NCOEFF,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check the number of clock fields.
    //
    testutil::CHCKSI(b"NFIELD", save.NFIELD, b"=", 4, 0, OK, ctx)?;

    //
    // Check the clock offsets.
    //
    spicelib::CLEARD(save.NFIELD, save.XOFF.as_slice_mut());

    testutil::CHCKAD(
        b"OFFSETS",
        save.DPBUFF.subarray((save.OFFBAS + 1)),
        b"=",
        save.XOFF.as_slice(),
        save.NFIELD,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check the time system code.
    //
    testutil::CHCKSI(b"TIMSYS", save.TIMSYS, b"=", 1, 0, OK, ctx)?;

    //
    // Check the delimiter code.
    //
    testutil::CHCKSI(b"DELCDE", save.DELCDE, b"=", 2, 0, OK, ctx)?;

    //
    // Check the first free integer and d.p. indices in the respective
    // data buffers.
    //
    // All clocks use 10 integer buffer entries.
    //
    testutil::CHCKSI(b"IFREE", save.IFREE, b"=", (save.ISTART + 10), 0, OK, ctx)?;

    testutil::CHCKSI(
        b"DPFREE",
        save.DPFREE,
        b"=",
        (((save.DSTART + save.NCOEFF) + (2 * save.NPARTS)) + (2 * save.NFIELD)),
        0,
        OK,
        ctx,
    )?;

    //
    // Save the free values for the next test case.
    //
    save.ISTART = save.IFREE;
    save.DSTART = save.DPFREE;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Add data for clock -999 to the database.", ctx)?;

    //
    // Do a conversion using clock -999. This will populate the
    // database.
    //
    save.SC = -999;
    save.ET = 0.0;

    spicelib::ZZPCTRCK(save.POLCTR.as_slice_mut(), &mut save.UPDATE, ctx);

    spicelib::SCE2C(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up information for clock -999.
    //
    spicelib::ZZPCTRCK(save.POLCTR.as_slice_mut(), &mut save.UPDATE, ctx);

    spicelib::ZZSCUP01(
        save.SC,
        save.POLCTR.as_slice_mut(),
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        save.DPBUFF.as_slice_mut(),
        &mut save.IFREE,
        save.INTBUF.as_slice_mut(),
        save.SCBASE.as_slice_mut(),
        &mut save.PRVSC,
        &mut save.NFIELD,
        &mut save.DELCDE,
        &mut save.TIMSYS,
        &mut save.NCOEFF,
        &mut save.NPARTS,
        &mut save.COFBAS,
        &mut save.STRBAS,
        &mut save.ENDBAS,
        &mut save.MODBAS,
        &mut save.OFFBAS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of partitions.
    //
    testutil::CHCKSI(b"NPARTS", save.NPARTS, b"=", 4, 0, OK, ctx)?;

    //
    // Check the partitions.
    //
    for I in 1..=save.NPARTS {
        save.XSTART[I] = (I as f64);
        save.XSTOP[I] = (720000.0 + I as f64);
    }
    //
    // Make the second partition start with a forward jump.
    //
    save.XSTART[2] = (save.XSTART[2] + 1000000.0);
    save.XSTOP[2] = (save.XSTOP[2] + 1000000.0);

    testutil::CHCKAD(
        b"STARTS",
        save.DPBUFF.subarray((save.STRBAS + 1)),
        b"~/",
        save.XSTART.as_slice(),
        save.NPARTS,
        VTIGHT,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STOPS",
        save.DPBUFF.subarray((save.ENDBAS + 1)),
        b"~/",
        save.XSTOP.as_slice(),
        save.NPARTS,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check the number of individual SCLK coefficients.
    //
    testutil::CHCKSI(b"NCOEFF", save.NCOEFF, b"=", 48, 0, OK, ctx)?;

    //
    // Check the clock moduli.
    //
    save.XMOD[1] = 10000000.0;
    save.XMOD[2] = 10.0;
    save.XMOD[3] = 9.0;
    save.XMOD[4] = 8.0;

    testutil::CHCKAD(
        b"MODULI",
        save.DPBUFF.subarray((save.MODBAS + 1)),
        b"=",
        save.XMOD.as_slice(),
        save.NFIELD,
        0.0,
        OK,
        ctx,
    )?;
    //
    // Check the  SCLK coefficients.
    //
    save.MAJOR = 1.0;

    for I in 2..=4 {
        save.MAJOR = (save.MAJOR * save.XMOD[I]);
    }

    save.DELTA = 0.000000001;

    for I in 1..=(save.NCOEFF / 3) {
        save.COEFFS[[1, I]] = (((I - 1) as f64) * 180000.0);

        save.COEFFS[[2, I]] = ((save.COEFFS[[1, I]] * 2 as f64) / save.MAJOR);

        save.COEFFS[[3, I]] = (2.0 + ((I as f64) * save.DELTA));
    }

    testutil::CHCKAD(
        b"COEFFS",
        save.DPBUFF.subarray((save.COFBAS + 1)),
        b"~/",
        save.COEFFS.as_slice(),
        save.NCOEFF,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check the number of clock fields.
    //
    testutil::CHCKSI(b"NFIELD", save.NFIELD, b"=", 4, 0, OK, ctx)?;

    //
    // Check the clock offsets.
    //
    save.XOFF[1] = 4 as f64;
    save.XOFF[2] = 3 as f64;
    save.XOFF[3] = 2 as f64;
    save.XOFF[4] = 1 as f64;

    testutil::CHCKAD(
        b"OFFSETS",
        save.DPBUFF.subarray((save.OFFBAS + 1)),
        b"=",
        save.XOFF.as_slice(),
        save.NFIELD,
        0.0,
        OK,
        ctx,
    )?;
    //
    // Check the time system code. This one is TDT.
    //
    testutil::CHCKSI(b"TIMSYS", save.TIMSYS, b"=", 2, 0, OK, ctx)?;

    //
    // Check the delimiter code.
    //
    testutil::CHCKSI(b"DELCDE", save.DELCDE, b"=", 2, 0, OK, ctx)?;

    //
    // Check the first free integer and d.p. indices in the respective
    // data buffers.
    //
    // All clocks use 10 integer buffer entries.
    //
    testutil::CHCKSI(b"IFREE", save.IFREE, b"=", (save.ISTART + 10), 0, OK, ctx)?;

    testutil::CHCKSI(
        b"DPFREE",
        save.DPFREE,
        b"=",
        (((save.DSTART + save.NCOEFF) + (2 * save.NPARTS)) + (2 * save.NFIELD)),
        0,
        OK,
        ctx,
    )?;

    //
    // Interleaved clock reference tests follow.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Interleave calls to SCE2C and SCT2E using clocks -9, -99, and -999.",
        ctx,
    )?;

    //
    // Use an ET value common to the time ranges of all three clocks.
    //
    //    - The time range for clock -9 is from
    //
    //              1980 JAN 1, 00:00:00.000 (ET)
    //         to   2011 SEP 9, 01:46:40.000 (ET)
    //
    //    - The ET range for the coefficient records of clock -99 is
    //      from 0 to 2.e6 seconds past J2000 TDB.
    //
    //    - The ET range for the coefficient records of clock -999 is
    //      from 0 to 7500 seconds past J2000 TDB.
    //
    save.ET0 = 3600.0;
    save.STEP = 0.1;
    save.NITER = MAXITR;

    //
    // Compute expected results using non-interleaved calls.
    //
    save.CLOCKS[1] = -9;
    save.CLOCKS[2] = -99;
    save.CLOCKS[3] = -999;

    for I in 1..=NCLOCK {
        for J in 1..=save.NITER {
            save.ET = (save.ET0 + (((J - 1) as f64) * save.STEP));

            spicelib::SCE2C(save.CLOCKS[I], save.ET, &mut save.XTICKS[[J, I]], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // Now perform interleaved computation.
    //
    save.TOL = VTIGHT;

    for J in 1..=save.NITER {
        save.XET = (save.ET0 + (((J - 1) as f64) * save.STEP));

        //
        // Halfway through, touch the kernel pool. Computations should
        // not be affected.
        //
        if (J == (save.NITER / 2)) {
            spicelib::PDPOOL(b"XYZ", 1, &[1.0], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        for I in 1..=NCLOCK {
            spicelib::SCE2C(save.CLOCKS[I], save.XET, &mut save.SCLKDP, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (f64::abs((save.SCLKDP - save.XTICKS[[J, I]])) > save.TOL) {
                //
                // This code is unreachable unless the SCE2C call above
                // failed.
                //
                fstr::assign(&mut save.LABEL, b"SCLKDP(#,#)");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);

                testutil::CHCKSD(
                    &save.LABEL,
                    save.SCLKDP,
                    b"~/",
                    save.XTICKS[[J, I]],
                    save.TOL,
                    OK,
                    ctx,
                )?;
            }

            spicelib::SCT2E(save.CLOCKS[I], save.SCLKDP, &mut save.ET, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (f64::abs((save.ET - save.XET)) > save.TOL) {
                //
                // This code is unreachable unless the SCT2E call above
                // failed.
                //
                fstr::assign(&mut save.LABEL, b"ET(#,#)");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);

                testutil::CHCKSD(
                    &save.LABEL,
                    save.SCLKDP,
                    b"~/",
                    save.XTICKS[[J, I]],
                    save.TOL,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Interleave calls to SCE2T and SCT2E using clocks -9, -99, and -999.",
        ctx,
    )?;

    //
    // We'll use integral tick values produced by rounding the values
    // from the previous test case.
    //
    for I in 1..=NCLOCK {
        for J in 1..=save.NITER {
            save.XTICKS[[J, I]] = f64::round(save.XTICKS[[J, I]]);
        }
    }

    //
    // Now perform interleaved computation.
    //
    // We expect exact tick matches.
    //
    save.TOL = 0.0;

    for J in 1..=save.NITER {
        //
        // Halfway through, touch the kernel pool. Computations should
        // not be affected.
        //
        if (J == (save.NITER / 2)) {
            spicelib::PDPOOL(b"XYZ", 1, &[1.0], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        for I in 1..=NCLOCK {
            spicelib::SCT2E(save.CLOCKS[I], save.XTICKS[[J, I]], &mut save.ET, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SCE2T(save.CLOCKS[I], save.ET, &mut save.SCLKDP, ctx)?;

            if ((save.SCLKDP - save.XTICKS[[J, I]]) != 0.0) {
                //
                // This code is unreachable unless the SCE2T call above
                // failed.
                //
                fstr::assign(&mut save.LABEL, b"SCLKDP(#,#)");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);

                testutil::CHCKSD(
                    &save.LABEL,
                    save.SCLKDP,
                    b"~/",
                    save.XTICKS[[J, I]],
                    save.TOL,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Interleave calls to SCDECD and SCENCD using clocks -9, -99, and -999.",
        ctx,
    )?;

    //
    // Use the integral tick values for each clock produced by the
    // previous test case.
    //
    // We expect exact tick matches.
    //
    save.TOL = 0.0;

    for J in 1..=save.NITER {
        //
        // Halfway through, touch the kernel pool. Computations should
        // not be affected.
        //
        if (J == (save.NITER / 2)) {
            spicelib::PDPOOL(b"XYZ", 1, &[1.0], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        for I in 1..=NCLOCK {
            spicelib::SCDECD(save.CLOCKS[I], save.XTICKS[[J, I]], &mut save.CLKSTR, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SCENCD(save.CLOCKS[I], &save.CLKSTR, &mut save.SCLKDP, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (f64::abs((save.SCLKDP - save.XTICKS[[J, I]])) > save.TOL) {
                //
                // This code is unreachable unless the SCENCD or SCDECD
                // calls above failed.
                //
                fstr::assign(&mut save.LABEL, b"TICKS(#,#)");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);

                testutil::CHCKSD(
                    &save.LABEL,
                    save.SCLKDP,
                    b"~/",
                    save.XTICKS[[J, I]],
                    save.TOL,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Interleave calls to SCS2E and SCE2S using clocks -9, -99, and -999.",
        ctx,
    )?;

    //
    // Use the integral tick values for each clock produced by the
    // previous test case.
    //
    // We expect exact tick matches.
    //
    save.TOL = 0.0;

    for J in 1..=save.NITER {
        //
        // Halfway through, touch the kernel pool. Computations should
        // not be affected.
        //
        if (J == (save.NITER / 2)) {
            spicelib::PDPOOL(b"XYZ", 1, &[1.0], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        for I in 1..=NCLOCK {
            spicelib::SCDECD(save.CLOCKS[I], save.XTICKS[[J, I]], &mut save.CLKSTR, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SCS2E(save.CLOCKS[I], &save.CLKSTR, &mut save.ET, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SCE2S(save.CLOCKS[I], save.ET, &mut save.CLKSTR, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SCENCD(save.CLOCKS[I], &save.CLKSTR, &mut save.SCLKDP, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (f64::abs((save.SCLKDP - save.XTICKS[[J, I]])) > save.TOL) {
                //
                // This code is unreachable unless the sequence of calls
                // above failed.
                //
                fstr::assign(&mut save.LABEL, b"TICKS(#,#)");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);

                testutil::CHCKSD(
                    &save.LABEL,
                    save.SCLKDP,
                    b"~/",
                    save.XTICKS[[J, I]],
                    save.TOL,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Buffer and use the maximum number of clocks.", ctx)?;

    //
    // The kernel pool data additions will initialize the database.
    //

    for I in 1..=MXNCLK {
        save.SC = -(10000 * I);

        fstr::assign(&mut save.KVNAME, b"SCLK_DATA_TYPE_#");
        spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PIPOOL(&save.KVNAME, 1, &[1], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"SCLK01_TIME_SYSTEM_#");
        spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PIPOOL(&save.KVNAME, 1, &[TDB], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"SCLK01_N_FIELDS_#");
        spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PIPOOL(&save.KVNAME, 1, &[1], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.MAXMOD = 1000000.0;

        fstr::assign(&mut save.KVNAME, b"SCLK01_MODULI_#");
        spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PDPOOL(&save.KVNAME, 1, &[save.MAXMOD], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"SCLK01_OFFSETS_#");
        spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PDPOOL(&save.KVNAME, 1, &[0.0], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"SCLK01_OUTPUT_DELIM_#");
        spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PIPOOL(&save.KVNAME, 1, &[1], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"SCLK_PARTITION_START_#");
        spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PDPOOL(&save.KVNAME, 1, &[(I as f64)], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"SCLK_PARTITION_END_#");
        spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PDPOOL(&save.KVNAME, 1, &[save.MAXMOD], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.KVNAME, b"SCLK01_COEFFICIENTS_#");
        spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.COEFFS[[1, 1]] = 0.0;
        save.COEFFS[[2, 1]] = (I as f64);
        save.COEFFS[[3, 1]] = (2.0 * I as f64);

        spicelib::PDPOOL(&save.KVNAME, 3, save.COEFFS.as_slice(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Populate the database with data for each clock.
    //
    for I in 1..=MXNCLK {
        save.SC = -(10000 * I);

        spicelib::SCT2E(save.SC, 0.0, &mut save.ET, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Perform a sequence of time conversions for each clock.
    //
    save.NITER = 10;

    for I in 1..=save.NITER {
        save.XCLKDP = (I as f64);

        for J in 1..=MXNCLK {
            save.SC = -(10000 * I);

            spicelib::SCDECD(save.SC, save.XCLKDP, &mut save.CLKSTR, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SCS2E(save.SC, &save.CLKSTR, &mut save.ET, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SCE2T(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(b"SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;
        }
    }

    //
    // Leave the database as is to simplify the following test case.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Draw in data for another clock, when data for the maximum number of clocks are already in the database.", ctx)?;

    save.SC = -9;

    spicelib::SCDECD(save.SC, save.XCLKDP, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCS2E(save.SC, &save.CLKSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCE2T(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Buffer and use data for a clock using more than half the maximum number of d.p. values.",
        ctx,
    )?;

    //
    // This is a simple test case for a clock with a large number of
    // coefficients. See the next test case for a more comprehensive,
    // but more complicated clock specification.
    //
    // Use the first clock from the previous test case. Just
    // add a bunch of new coefficient records to its set.
    //
    save.SC = -10000;

    save.NREC = ((MXCOEF / 2) + 1);

    for I in 1..=save.NREC {
        save.COEFFS[[1, I]] = ((I - 1) as f64);
        save.COEFFS[[2, I]] = (((I - 1) as f64) * 2 as f64);
        save.COEFFS[[3, I]] = 0.5;
    }

    save.NCOEFF = (3 * save.NREC);

    fstr::assign(&mut save.KVNAME, b"SCLK01_COEFFICIENTS_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, save.NCOEFF, save.COEFFS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform a sequence of time conversions for the clock.
    //
    save.NITER = 10;

    for I in 1..=save.NITER {
        save.XCLKDP = (I as f64);

        spicelib::SCDECD(save.SC, save.XCLKDP, &mut save.CLKSTR, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SCS2E(save.SC, &save.CLKSTR, &mut save.ET, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SCE2T(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(b"SCLKDP", save.SCLKDP, b"=", save.XCLKDP, 0.0, OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Buffer and use data for clock that has the maximum number of coefficients, partitions, and fields.", ctx)?;

    //
    // Make sure we don't fill up the kernel pool.
    //
    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SC = -30000;

    //
    // Insert the clock definition into the kernel pool.
    //
    fstr::assign(&mut save.KVNAME, b"SCLK_DATA_TYPE_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(&save.KVNAME, 1, &[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"SCLK01_TIME_SYSTEM_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(&save.KVNAME, 1, &[TDB], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The clock has 10 fields, each with modulus 2.
    //
    fstr::assign(&mut save.KVNAME, b"SCLK01_N_FIELDS_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NFIELD = 10;
    spicelib::PIPOOL(&save.KVNAME, 1, &[save.NFIELD], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"SCLK01_MODULI_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FILLD(2.0, (save.NFIELD - 2), save.XMOD.subarray_mut(3));
    save.XMOD[1] = 10.0;
    save.XMOD[2] = 4.0;

    spicelib::PDPOOL(&save.KVNAME, save.NFIELD, save.XMOD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The offsets are all zero.
    //
    fstr::assign(&mut save.KVNAME, b"SCLK01_OFFSETS_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(save.NFIELD, save.XOFF.as_slice_mut());

    spicelib::PDPOOL(&save.KVNAME, save.NFIELD, save.XOFF.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"SCLK01_OUTPUT_DELIM_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Use the delimiter ':'
    //
    spicelib::PIPOOL(&save.KVNAME, 1, &[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create new coefficient records for this clock.
    //
    save.NREC = MXCOEF;

    //
    // Coefficient records are 1024 ticks apart on the SCLK time scale.
    // Nominal tick duration is 1/32 of a second. Major counts are
    // 2**10 ticks, or 32 seconds.
    //
    for I in 1..=save.NREC {
        save.COEFFS[[1, I]] = (((I - 1) as f64) * 1024.0);
        save.COEFFS[[2, I]] = (((I - 1) as f64) * 32.0);
        save.COEFFS[[3, I]] = 32.0;
    }

    save.NCOEFF = (3 * save.NREC);

    fstr::assign(&mut save.KVNAME, b"SCLK01_COEFFICIENTS_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, save.NCOEFF, save.COEFFS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create partition boundaries. Each partition will have just
    // 10240 ticks. The last partition will have an end time past
    // the epoch of the last record.
    //
    save.NPARTS = 9999;

    for I in 1..=save.NPARTS {
        save.XSTART[I] = 0.0;
        save.XSTOP[I] = 10240.0;
    }

    fstr::assign(&mut save.KVNAME, b"SCLK_PARTITION_START_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, save.NPARTS, save.XSTART.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"SCLK_PARTITION_END_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, save.NPARTS, save.XSTOP.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform a sequence of time conversions for the clock.
    //
    fstr::assign(&mut save.CLKSTR, b"9999/9:3:1:1:1:1:1:1:1:1");

    spicelib::SCENCD(save.SC, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"SCLKDP (A)",
        save.SCLKDP,
        b"=",
        ((((save.NPARTS - 1) * 10240) + 10239) as f64),
        0.0,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.XCLKST, &save.CLKSTR);
    spicelib::SCDECD(save.SC, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"CLKSTR (A)", &save.CLKSTR, b"=", &save.XCLKST, OK, ctx)?;

    spicelib::SCS2E(save.SC, &save.CLKSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"ET (A)",
        save.ET,
        b"~/",
        (save.SCLKDP / 32 as f64),
        VTIGHT,
        OK,
        ctx,
    )?;

    save.XCLKDP = save.SCLKDP;
    spicelib::SCE2C(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"SCLKDP (AC)",
        save.SCLKDP,
        b"~/",
        save.XCLKDP,
        VTIGHT,
        OK,
        ctx,
    )?;

    spicelib::SCE2T(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"SCLKDP (AT)",
        save.SCLKDP,
        b"~/",
        save.XCLKDP,
        VTIGHT,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.CLKSTR, b"1/0:0:0:0:0:0:0:0:0:0");
    spicelib::SCENCD(save.SC, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"SCLKDP (B)", save.SCLKDP, b"=", 0.0, 0.0, OK, ctx)?;

    fstr::assign(&mut save.XCLKST, &save.CLKSTR);
    spicelib::SCDECD(save.SC, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"CLKSTR (B)", &save.CLKSTR, b"=", &save.XCLKST, OK, ctx)?;

    spicelib::SCS2E(save.SC, &save.CLKSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"ET (B)",
        save.ET,
        b"~/",
        (save.SCLKDP / 32 as f64),
        VTIGHT,
        OK,
        ctx,
    )?;

    save.XCLKDP = save.SCLKDP;
    spicelib::SCE2C(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"SCLKDP (BC)",
        save.SCLKDP,
        b"~/",
        save.XCLKDP,
        VTIGHT,
        OK,
        ctx,
    )?;

    spicelib::SCE2T(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"SCLKDP (BT)",
        save.SCLKDP,
        b"~/",
        save.XCLKDP,
        VTIGHT,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.CLKSTR, b"1/9:3:1:1:1:1:1:1:1:1");
    spicelib::SCENCD(save.SC, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"SCLKDP (C)", save.SCLKDP, b"~/", 10239.0, VTIGHT, OK, ctx)?;

    fstr::assign(&mut save.XCLKST, &save.CLKSTR);
    spicelib::SCDECD(save.SC, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"CLKSTR (C)", &save.CLKSTR, b"=", &save.XCLKST, OK, ctx)?;

    spicelib::SCS2E(save.SC, &save.CLKSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"ET (C)",
        save.ET,
        b"~/",
        (save.SCLKDP / 32 as f64),
        VTIGHT,
        OK,
        ctx,
    )?;

    save.XCLKDP = save.SCLKDP;
    spicelib::SCE2C(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"SCLKDP (CC)",
        save.SCLKDP,
        b"~/",
        save.XCLKDP,
        VTIGHT,
        OK,
        ctx,
    )?;

    spicelib::SCE2T(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"SCLKDP (CT)",
        save.SCLKDP,
        b"~/",
        save.XCLKDP,
        VTIGHT,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.CLKSTR, b"2/0:0:0:0:0:0:0:0:0:0");
    spicelib::SCENCD(save.SC, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"SCLKDP (D)", save.SCLKDP, b"~/", 10240.0, VTIGHT, OK, ctx)?;

    fstr::assign(&mut save.XCLKST, &save.CLKSTR);
    spicelib::SCDECD(save.SC, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"CLKSTR (D)", &save.CLKSTR, b"=", &save.XCLKST, OK, ctx)?;

    spicelib::SCS2E(save.SC, &save.CLKSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"ET (D)",
        save.ET,
        b"~/",
        (save.SCLKDP / 32 as f64),
        VTIGHT,
        OK,
        ctx,
    )?;

    save.XCLKDP = save.SCLKDP;
    spicelib::SCE2C(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"SCLKDP (DC)",
        save.SCLKDP,
        b"~/",
        save.XCLKDP,
        VTIGHT,
        OK,
        ctx,
    )?;

    spicelib::SCE2T(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"SCLKDP (DT)",
        save.SCLKDP,
        b"~/",
        save.XCLKDP,
        VTIGHT,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.CLKSTR, b"9999/0:0:0:0:0:0:0:0:0:0");
    spicelib::SCENCD(save.SC, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"SCLKDP (E)",
        save.SCLKDP,
        b"~/",
        ((9998 as f64) * 10240.0),
        VTIGHT,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.XCLKST, &save.CLKSTR);
    spicelib::SCDECD(save.SC, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"CLKSTR (E)", &save.CLKSTR, b"=", &save.XCLKST, OK, ctx)?;

    spicelib::SCS2E(save.SC, &save.CLKSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"ET (E)",
        save.ET,
        b"~/",
        (save.SCLKDP / 32 as f64),
        VTIGHT,
        OK,
        ctx,
    )?;

    save.XCLKDP = save.SCLKDP;
    spicelib::SCE2C(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"SCLKDP (EC)",
        save.SCLKDP,
        b"~/",
        save.XCLKDP,
        VTIGHT,
        OK,
        ctx,
    )?;

    spicelib::SCE2T(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"SCLKDP (ET)",
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
    testutil::TCASE(b"Force ZZSCAD01 to make room in the d.p. buffer.", ctx)?;

    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Populate the database for the "big" clock -30000.
    //
    fstr::assign(&mut save.CLKSTR, b"1/9:3:1:1:1:1:1:1:1:1");
    spicelib::SCENCD(save.SC, &save.CLKSTR, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"SCLKDP (C)", save.SCLKDP, b"~/", 10239.0, VTIGHT, OK, ctx)?;

    fstr::assign(&mut save.XCLKST, &save.CLKSTR);
    spicelib::SCDECD(save.SC, save.SCLKDP, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"CLKSTR (C)", &save.CLKSTR, b"=", &save.XCLKST, OK, ctx)?;

    spicelib::SCS2E(save.SC, &save.CLKSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"ET (C)",
        save.ET,
        b"~/",
        (save.SCLKDP / 32 as f64),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Now do a time conversion for clock -99. The d.p. buffer will
    // need to be emptied in order to store data for this clock.
    //
    save.SC = -99;

    save.ET = 1.0;
    spicelib::SCE2C(save.SC, save.ET, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Fetch type of non-type-1 SCLK.", ctx)?;

    save.SC = -999999;

    fstr::assign(&mut save.KVNAME, b"SCLK_DATA_TYPE_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XTYPE = 7;

    spicelib::PIPOOL(&save.KVNAME, 1, &[save.XTYPE], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CLKTYP = spicelib::SCTYPE(save.SC, ctx)?;

    testutil::CHCKSI(b"CLKTYP", save.CLKTYP, b"=", save.XTYPE, 1, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZSCAD01: use a clock data set that lacks a time system kernel variable. This is not an error case.", ctx)?;

    //
    // Reload LSK to support STR2ET call.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The time system defaults to TDB. Use a clock for which TDB
    // is the actual associated time system.
    //
    spicelib::LDPOOL(SCLK, ctx)?;

    save.SC = -9;

    fstr::assign(&mut save.KVNAME, b"SCLK01_TIME_SYSTEM_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DVPOOL(&save.KVNAME, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Do a conversion that requires the associated time system.
    //
    spicelib::STR2ET(b"1980 JAN 1 00:00:01 TDB", &mut save.XET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.CLKSTR, b"1/000000001:0000");

    spicelib::SCS2E(save.SC, &save.CLKSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"ET", save.ET, b"~/", save.XET, save.TOL, OK, ctx)?;

    //
    // Restore the original clock data.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //
    //     Error cases
    //
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to use SCLK with too many coefficients.", ctx)?;

    //
    // Use the first clock from the previous test case.  We'll need
    // to clear the kernel pool so *it* doesn't overflow.
    //
    spicelib::KCLEAR(ctx)?;

    save.SC = -20000;

    save.NREC = (MXCOEF + 1);

    for I in 1..=save.NREC {
        save.COEFFS[[1, I]] = ((I - 1) as f64);
        save.COEFFS[[2, I]] = (((I - 1) as f64) * 4 as f64);
        save.COEFFS[[3, I]] = 0.25;
    }

    save.NCOEFF = (3 * save.NREC);

    fstr::assign(&mut save.KVNAME, b"SCLK01_COEFFICIENTS_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, save.NCOEFF, save.COEFFS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the rest of the SCLK parameters.
    //
    fstr::assign(&mut save.KVNAME, b"SCLK_DATA_TYPE_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(&save.KVNAME, 1, &[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"SCLK01_TIME_SYSTEM_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(&save.KVNAME, 1, &[TDB], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"SCLK01_N_FIELDS_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(&save.KVNAME, 1, &[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MAXMOD = 1000000.0;

    fstr::assign(&mut save.KVNAME, b"SCLK01_MODULI_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, 1, &[save.MAXMOD], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"SCLK01_OFFSETS_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, 1, &[0.0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"SCLK01_OUTPUT_DELIM_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(&save.KVNAME, 1, &[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"SCLK_PARTITION_START_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, 1, &[0.0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"SCLK_PARTITION_END_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, 1, &[save.MAXMOD], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now try a conversion.
    //
    // The error will be trapped in ZZSCAD01.
    //
    spicelib::SCDECD(save.SC, 0.0, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(true, b"SPICE(TOOMANYCOEFFS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Try to use SCLK with mismatched partition start time and stop time counts.",
        ctx,
    )?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Just perturb the partitions of clock -9.
    //
    spicelib::FURNSH(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SC = -9;

    save.NPARTS = 10;

    for I in 1..=save.NPARTS {
        save.XSTART[I] = 0.0;
        save.XSTOP[I] = (I as f64);
    }

    fstr::assign(&mut save.KVNAME, b"SCLK_PARTITION_START_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, save.NPARTS, save.XSTART.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"SCLK_PARTITION_END_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, (save.NPARTS - 1), save.XSTOP.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now try a conversion.
    //
    // The error will be trapped in ZZSCAD01.
    //
    spicelib::SCDECD(save.SC, 0.0, &mut save.CLKSTR, ctx)?;
    testutil::CHCKXC(true, b"SPICE(NUMPARTSUNEQUAL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZSCAD01: partition starts not found.", ctx)?;

    save.SC = -99;

    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"SCLK_PARTITION_START_99");

    spicelib::DVPOOL(&save.KVNAME, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSCAD01(
        save.SC,
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        save.DPBUFF.as_slice_mut(),
        &mut save.IFREE,
        save.INTBUF.as_slice_mut(),
        save.SCBASE.as_slice_mut(),
        &mut save.SCLKAT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZSCAD01: partition stops not found.", ctx)?;

    save.SC = -99;

    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"SCLK_PARTITION_END_99");

    spicelib::DVPOOL(&save.KVNAME, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSCAD01(
        save.SC,
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        save.DPBUFF.as_slice_mut(),
        &mut save.IFREE,
        save.INTBUF.as_slice_mut(),
        save.SCBASE.as_slice_mut(),
        &mut save.SCLKAT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZSCAD01: moduli not found.", ctx)?;

    save.SC = -99;

    fstr::assign(&mut save.KVNAME, b"SCLK01_MODULI_99");

    spicelib::DVPOOL(&save.KVNAME, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSCAD01(
        save.SC,
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        save.DPBUFF.as_slice_mut(),
        &mut save.IFREE,
        save.INTBUF.as_slice_mut(),
        save.SCBASE.as_slice_mut(),
        &mut save.SCLKAT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZSCAD01: offsets not found.", ctx)?;

    save.SC = -99;

    fstr::assign(&mut save.KVNAME, b"SCLK01_OFFSETS_99");

    spicelib::DVPOOL(&save.KVNAME, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSCAD01(
        save.SC,
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        save.DPBUFF.as_slice_mut(),
        &mut save.IFREE,
        save.INTBUF.as_slice_mut(),
        save.SCBASE.as_slice_mut(),
        &mut save.SCLKAT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZSCAD01: clock has too many partitions.", ctx)?;

    //
    // We'll just tweak the partitions for clock -9, since we
    // can easily restore the original kernel variables for that clock.
    //
    //
    // Create partition boundaries. Each partition will have just
    // 10240 ticks. The last partition will have an end time past
    // the epoch of the last record.
    //
    save.SC = -9;

    save.NPARTS = 10000;

    for I in 1..=save.NPARTS {
        save.XSTART[I] = 0.0;
        save.XSTOP[I] = 10240.0;
    }

    fstr::assign(&mut save.KVNAME, b"SCLK_PARTITION_START_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, save.NPARTS, save.XSTART.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"SCLK_PARTITION_END_#");
    spicelib::REPMI(&save.KVNAME.to_vec(), b"#", -save.SC, &mut save.KVNAME, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, save.NPARTS, save.XSTOP.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCE2C(save.SC, 1.0, &mut save.SCLKDP, ctx)?;
    testutil::CHCKXC(true, b"SPICE(TOOMANYPARTITIONS)", OK, ctx)?;

    //
    // Restore the correct parameters for clock -9.
    //
    spicelib::LDPOOL(SCLK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZSCAD01: moduli kernel variable too large.", ctx)?;

    save.SC = -99;

    fstr::assign(&mut save.KVNAME, b"SCLK01_MODULI_99");

    spicelib::FILLD(10.0, DVALSZ, save.DVALS.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, DVALSZ, save.DVALS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSCAD01(
        save.SC,
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        save.DPBUFF.as_slice_mut(),
        &mut save.IFREE,
        save.INTBUF.as_slice_mut(),
        save.SCBASE.as_slice_mut(),
        &mut save.SCLKAT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARTOOLARGE)", OK, ctx)?;

    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZSCAD01: moduli kernel variable is non-numeric", ctx)?;

    save.SC = -99;

    fstr::assign(&mut save.KVNAME, b"SCLK01_MODULI_99");

    spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(b"XYZ"), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSCAD01(
        save.SC,
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        save.DPBUFF.as_slice_mut(),
        &mut save.IFREE,
        save.INTBUF.as_slice_mut(),
        save.SCBASE.as_slice_mut(),
        &mut save.SCLKAT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADKERNELVARTYPE)", OK, ctx)?;

    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZSCAD01: IFREE out of range", ctx)?;

    save.SC = -99;

    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Start with a clean system.
    //
    spicelib::ZZSCIN01(
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        &mut save.IFREE,
        &mut save.PRVSC,
        ctx,
    )?;

    save.IFREE = (IBUFSZ + 2);

    spicelib::ZZSCAD01(
        save.SC,
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        save.DPBUFF.as_slice_mut(),
        &mut save.IFREE,
        save.INTBUF.as_slice_mut(),
        save.SCBASE.as_slice_mut(),
        &mut save.SCLKAT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BUG)", OK, ctx)?;

    save.IFREE = 0;

    spicelib::ZZSCAD01(
        save.SC,
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        save.DPBUFF.as_slice_mut(),
        &mut save.IFREE,
        save.INTBUF.as_slice_mut(),
        save.SCBASE.as_slice_mut(),
        &mut save.SCLKAT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BUG)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZSCAD01: DPFREE out of range", ctx)?;

    save.SC = -99;

    spicelib::LDPOOL(SCLK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Start with a clean system.
    //
    spicelib::ZZSCIN01(
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        &mut save.IFREE,
        &mut save.PRVSC,
        ctx,
    )?;

    save.DPFREE = (DBUFSZ + 2);

    spicelib::ZZSCAD01(
        save.SC,
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        save.DPBUFF.as_slice_mut(),
        &mut save.IFREE,
        save.INTBUF.as_slice_mut(),
        save.SCBASE.as_slice_mut(),
        &mut save.SCLKAT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BUG)", OK, ctx)?;

    save.DPFREE = 0;

    spicelib::ZZSCAD01(
        save.SC,
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        save.DPBUFF.as_slice_mut(),
        &mut save.IFREE,
        save.INTBUF.as_slice_mut(),
        save.SCBASE.as_slice_mut(),
        &mut save.SCLKAT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BUG)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZSCAD01: return on entry.", ctx)?;

    spicelib::ZZSCIN01(
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        &mut save.IFREE,
        &mut save.PRVSC,
        ctx,
    )?;

    fstr::assign(&mut save.SHRTMS, b"SPICE(TESTERROR)");

    spicelib::SIGERR(&save.SHRTMS, ctx)?;

    spicelib::ZZSCAD01(
        save.SC,
        save.HDSCLK.as_slice_mut(),
        save.SCPOOL.as_slice_mut(),
        save.CLKLST.as_slice_mut(),
        &mut save.DPFREE,
        save.DPBUFF.as_slice_mut(),
        &mut save.IFREE,
        save.INTBUF.as_slice_mut(),
        save.SCBASE.as_slice_mut(),
        &mut save.SCLKAT,
        ctx,
    )?;

    testutil::CHCKXC(true, &save.SHRTMS, OK, ctx)?;

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
