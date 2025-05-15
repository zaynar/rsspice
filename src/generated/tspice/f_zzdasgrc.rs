//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const NWD: i32 = 128;
const NWI: i32 = 256;
const NWC: i32 = 1024;
const CHARDT: i32 = 1;
const DPDT: i32 = 2;
const INTDT: i32 = 3;
const DSK: &[u8] = b"zzdasgrc_test.bds";
const NNDSK: &[u8] = b"nn_zzdasgrc_test.bds";
const PCK: &[u8] = b"zzdasgrc_test.tpc";
const BIGI3E: i32 = 1;
const LTLI3E: i32 = 2;
const LBLSIZ: i32 = 40;
const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;
const FRNMLN: i32 = 32;
const INTIX: i32 = 3;
const DPIX: i32 = 2;

struct SaveVars {
    FRNAME: Vec<u8>,
    IDWORD: Vec<u8>,
    IFNAME: Vec<u8>,
    LABEL: Vec<u8>,
    XIDW: Vec<u8>,
    XIFNAM: Vec<u8>,
    DREC: StackArray<f64, 128>,
    XDREC: StackArray<f64, 128>,
    B: i32,
    BODYID: i32,
    CLBASE: i32,
    CLSIZE: i32,
    E: i32,
    FREE: i32,
    HANDLE: i32,
    NNHAN: i32,
    IREC: StackArray<i32, 256>,
    LASTLA: StackArray<i32, 3>,
    LASTRC: StackArray<i32, 3>,
    LASTWD: StackArray<i32, 3>,
    NATBFF: i32,
    NCOMC: i32,
    NCOMR: i32,
    NLAT: i32,
    NLON: i32,
    NREAD: i32,
    NRESVC: i32,
    NRESVR: i32,
    NW: i32,
    OBFF: i32,
    RECNO: i32,
    REMAIN: i32,
    SURFID: i32,
    UNIT: i32,
    WORDNO: i32,
    XIREC: StackArray<i32, 256>,
    XNCOMC: i32,
    XNCOMR: i32,
    XNRSVC: i32,
    XNRSVR: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FRNAME = vec![b' '; FRNMLN as usize];
        let mut IDWORD = vec![b' '; IDWLEN as usize];
        let mut IFNAME = vec![b' '; IFNLEN as usize];
        let mut LABEL = vec![b' '; LBLSIZ as usize];
        let mut XIDW = vec![b' '; IDWLEN as usize];
        let mut XIFNAM = vec![b' '; IFNLEN as usize];
        let mut DREC = StackArray::<f64, 128>::new(1..=NWD);
        let mut XDREC = StackArray::<f64, 128>::new(1..=NWD);
        let mut B: i32 = 0;
        let mut BODYID: i32 = 0;
        let mut CLBASE: i32 = 0;
        let mut CLSIZE: i32 = 0;
        let mut E: i32 = 0;
        let mut FREE: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut NNHAN: i32 = 0;
        let mut IREC = StackArray::<i32, 256>::new(1..=NWI);
        let mut LASTLA = StackArray::<i32, 3>::new(1..=3);
        let mut LASTRC = StackArray::<i32, 3>::new(1..=3);
        let mut LASTWD = StackArray::<i32, 3>::new(1..=3);
        let mut NATBFF: i32 = 0;
        let mut NCOMC: i32 = 0;
        let mut NCOMR: i32 = 0;
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NREAD: i32 = 0;
        let mut NRESVC: i32 = 0;
        let mut NRESVR: i32 = 0;
        let mut NW: i32 = 0;
        let mut OBFF: i32 = 0;
        let mut RECNO: i32 = 0;
        let mut REMAIN: i32 = 0;
        let mut SURFID: i32 = 0;
        let mut UNIT: i32 = 0;
        let mut WORDNO: i32 = 0;
        let mut XIREC = StackArray::<i32, 256>::new(1..=NWI);
        let mut XNCOMC: i32 = 0;
        let mut XNCOMR: i32 = 0;
        let mut XNRSVC: i32 = 0;
        let mut XNRSVR: i32 = 0;

        Self {
            FRNAME,
            IDWORD,
            IFNAME,
            LABEL,
            XIDW,
            XIFNAM,
            DREC,
            XDREC,
            B,
            BODYID,
            CLBASE,
            CLSIZE,
            E,
            FREE,
            HANDLE,
            NNHAN,
            IREC,
            LASTLA,
            LASTRC,
            LASTWD,
            NATBFF,
            NCOMC,
            NCOMR,
            NLAT,
            NLON,
            NREAD,
            NRESVC,
            NRESVR,
            NW,
            OBFF,
            RECNO,
            REMAIN,
            SURFID,
            UNIT,
            WORDNO,
            XIREC,
            XNCOMC,
            XNCOMR,
            XNRSVC,
            XNRSVR,
        }
    }
}

//$Procedure F_ZZDASGRC ( DAS record fetch tests )
pub fn F_ZZDASGRC(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // These parameters must be kept consistent with zzddhman.inc.
    // That file is not included because of a parameter name conflict
    // with das.inc.
    //

    //
    // Other parameters
    //

    //
    // Local Variables
    //

    //
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZDASGRC", ctx)?;

    //***********************************************************************
    //
    //     Setup
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create supporting kernels.", ctx)?;

    if spicelib::EXISTS(PCK, ctx)? {
        spicelib::DELFIL(PCK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::TSTPCK(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create tessellated ellipsoid DSK.", ctx)?;

    save.BODYID = 499;
    save.SURFID = 1;
    fstr::assign(&mut save.FRNAME, b"IAU_MARS");

    save.NLON = 40;
    save.NLAT = 20;

    if spicelib::EXISTS(DSK, ctx)? {
        spicelib::DELFIL(DSK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::T_ELDS2Z(
        save.BODYID,
        save.SURFID,
        &save.FRNAME,
        save.NLON,
        save.NLAT,
        DSK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add a few comments to the DSK.
    //
    spicelib::DASOPW(DSK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASAC(
        save.HANDLE,
        1,
        CharArray::from_ref(b"Comment line number one out of one."),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create non-native version of DSK.", ctx)?;

    //
    // Get the native format code and the code of the format
    // to which the file will be transformed.
    //
    //
    spicelib::ZZDDHNFC(&mut save.NATBFF, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if (save.NATBFF == LTLI3E) {
        save.OBFF = BIGI3E;
    } else if (save.NATBFF == BIGI3E) {
        save.OBFF = LTLI3E;
    } else {
        spicelib::SETMSG(b"Found unexpected native BFF code #.", ctx);
        spicelib::ERRINT(b"#", save.NATBFF, ctx);
        spicelib::SIGERR(b"SPICE(UNSUPPORTEDBFF)", ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(NNDSK, ctx)? {
        spicelib::DELFIL(NNDSK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    T_BINGO(DSK, NNDSK, save.OBFF, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //***********************************************************************
    //
    //     Test ZZDASRFR
    //
    //***********************************************************************

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Open native DSK for read access; read file record.", ctx)?;

    spicelib::DASOPR(DSK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDASRFR(
        save.HANDLE,
        &mut save.IDWORD,
        &mut save.IFNAME,
        &mut save.NRESVR,
        &mut save.NRESVC,
        &mut save.NCOMR,
        &mut save.NCOMC,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.XIDW, b"DAS/DSK");
    fstr::assign(&mut save.XIFNAM, DSK);
    save.XNRSVR = 0;
    save.XNRSVC = 0;
    save.XNCOMR = 1;
    save.XNCOMC = 36;

    testutil::CHCKSC(b"IDWORD", &save.IDWORD, b"=", &save.XIDW, OK, ctx)?;
    testutil::CHCKSC(b"IFNAME", &save.IFNAME, b"=", &save.XIFNAM, OK, ctx)?;
    testutil::CHCKSI(b"NRESVR", save.NRESVR, b"=", save.XNRSVR, 0, OK, ctx)?;
    testutil::CHCKSI(b"NRESVC", save.NRESVC, b"=", save.XNRSVC, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOMR", save.NCOMR, b"=", save.XNCOMR, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOMC", save.NCOMC, b"=", save.XNCOMC, 0, OK, ctx)?;

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Open NON-native DSK for read access; read file record.",
        ctx,
    )?;

    spicelib::DASOPR(NNDSK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDASRFR(
        save.HANDLE,
        &mut save.IDWORD,
        &mut save.IFNAME,
        &mut save.NRESVR,
        &mut save.NRESVC,
        &mut save.NCOMR,
        &mut save.NCOMC,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Use the expected values from the previous test case.
    //
    testutil::CHCKSC(b"IDWORD", &save.IDWORD, b"=", &save.XIDW, OK, ctx)?;
    testutil::CHCKSC(b"IFNAME", &save.IFNAME, b"=", &save.XIFNAM, OK, ctx)?;
    testutil::CHCKSI(b"NRESVR", save.NRESVR, b"=", save.XNRSVR, 0, OK, ctx)?;
    testutil::CHCKSI(b"NRESVC", save.NRESVC, b"=", save.XNRSVC, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOMR", save.NCOMR, b"=", save.XNCOMR, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOMC", save.NCOMC, b"=", save.XNCOMC, 0, OK, ctx)?;

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    // We test only exception (2) of ZZDASRFR. The others should
    // be covered by tests of lower-level routines.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDASRFR: Invalid file handle", ctx)?;

    spicelib::ZZDASRFR(
        -10000,
        &mut save.IDWORD,
        &mut save.IFNAME,
        &mut save.NRESVR,
        &mut save.NRESVC,
        &mut save.NCOMR,
        &mut save.NCOMC,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOSUCHHANDLE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //  CALL TCASE ( 'ZZDASRFR: error reading file' )
    //
    //  This test has the undesirable side effect of leaving behind
    //  a file with a system-dependent file name. We exclude this
    //  case from standard TSPICE runs, but it can be run independently.
    //
    //
    //  Open the file for read access, then delete it.
    //
    //
    //  CALL DASOPR ( DSK, HANDLE )
    //  CALL CHCKXC ( .FALSE., ' ', OK )
    //
    //  CALL KILFIL ( DSK )
    //
    //  CALL ZZDASRFR ( HANDLE, IDWORD, IFNAME, NRESVR,
    // .                NRESVC, NCOMR,  NCOMC          )
    //  CALL CHCKXC ( .TRUE., 'SPICE(FILEREADFAILED)', OK )
    //

    //***********************************************************************
    //
    //     Test ZZDASGRD
    //
    //***********************************************************************

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check d.p. records read from native DSK.", ctx)?;
    //
    // This is a consistency check only: DASRDD uses ZZDASGRD.
    // The next test case uses the independent routine DASIOD.
    //

    //
    // Get the DSK's file summary info.
    //
    spicelib::DASOPR(DSK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASHFS(
        save.HANDLE,
        &mut save.NRESVR,
        &mut save.NRESVC,
        &mut save.NCOMR,
        &mut save.NCOMC,
        &mut save.FREE,
        save.LASTLA.as_slice_mut(),
        save.LASTRC.as_slice_mut(),
        save.LASTWD.as_slice_mut(),
        ctx,
    )?;

    //
    // Let NW be the number of d.p. data words in the file.
    //
    save.NW = save.LASTLA[DPIX];

    //
    // Read the d.p. data; compare with data that we fetch using
    // ZZDASGRD.
    //

    save.REMAIN = save.NW;
    save.B = 1;

    while ((save.REMAIN > 0) && !spicelib::FAILED(ctx)) {
        save.NREAD = intrinsics::MIN0(&[save.REMAIN, NWD]);
        save.E = ((save.B + save.NREAD) - 1);

        spicelib::DASRDD(save.HANDLE, save.B, save.E, save.XDREC.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Get the number of the record containing address B.
        //
        spicelib::DASA2L(
            save.HANDLE,
            DPIX,
            save.B,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Read the record at index RECNO.
        //
        spicelib::ZZDASGRD(save.HANDLE, save.RECNO, save.DREC.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We expect an exact match.
        //
        fstr::assign(&mut save.LABEL, b"DREC@ record #");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.RECNO, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.DREC.as_slice(),
            b"=",
            save.XDREC.as_slice(),
            save.NREAD,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Prepare for the next read, if any.
        //
        save.REMAIN = (save.REMAIN - save.NREAD);
        save.B = (save.B + save.NREAD);
    }

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check d.p. records read from native DSK. Use unit-oriented record reader to fetch expected record.", ctx)?;

    //
    // Get the DSK's file summary info.
    //
    spicelib::DASOPR(DSK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASHFS(
        save.HANDLE,
        &mut save.NRESVR,
        &mut save.NRESVC,
        &mut save.NCOMR,
        &mut save.NCOMC,
        &mut save.FREE,
        save.LASTLA.as_slice_mut(),
        save.LASTRC.as_slice_mut(),
        save.LASTWD.as_slice_mut(),
        ctx,
    )?;

    //
    // Let NW be the number of d.p. data words in the file.
    //
    save.NW = save.LASTLA[DPIX];

    //
    // Read the d.p. data; compare with data that we fetch using
    // ZZDASGRD.
    //

    save.REMAIN = save.NW;
    save.B = 1;

    while ((save.REMAIN > 0) && !spicelib::FAILED(ctx)) {
        save.NREAD = intrinsics::MIN0(&[save.REMAIN, NWD]);
        save.E = ((save.B + save.NREAD) - 1);

        //
        // Get the number of the record containing address B.
        //
        spicelib::DASA2L(
            save.HANDLE,
            DPIX,
            save.B,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Get an unlocked logical unit for the DSK file.
        // Read the expected record.
        //
        spicelib::ZZDDHHLU(save.HANDLE, b"DAS", false, &mut save.UNIT, ctx)?;

        spicelib::DASIOD(
            b"READ",
            save.UNIT,
            save.RECNO,
            save.XDREC.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Read the record at index RECNO.
        //
        spicelib::ZZDASGRD(save.HANDLE, save.RECNO, save.DREC.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We expect an exact match.
        //
        fstr::assign(&mut save.LABEL, b"DREC@ record #");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.RECNO, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.DREC.as_slice(),
            b"=",
            save.XDREC.as_slice(),
            save.NREAD,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Prepare for the next read, if any.
        //
        save.REMAIN = (save.REMAIN - save.NREAD);
        save.B = (save.B + save.NREAD);
    }

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check d.p. records read from non-native DSK.", ctx)?;

    //
    // Get the DSK's file summary info.
    //
    spicelib::DASOPR(NNDSK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASHFS(
        save.HANDLE,
        &mut save.NRESVR,
        &mut save.NRESVC,
        &mut save.NCOMR,
        &mut save.NCOMC,
        &mut save.FREE,
        save.LASTLA.as_slice_mut(),
        save.LASTRC.as_slice_mut(),
        save.LASTWD.as_slice_mut(),
        ctx,
    )?;

    //
    // Let NW be the number of d.p. data words in the file.
    //
    save.NW = save.LASTLA[DPIX];

    //
    // Read the d.p. data; compare with data that we fetch using
    // ZZDASGRD.
    //

    save.REMAIN = save.NW;
    save.B = 1;

    while ((save.REMAIN > 0) && !spicelib::FAILED(ctx)) {
        save.NREAD = intrinsics::MIN0(&[save.REMAIN, NWD]);
        save.E = ((save.B + save.NREAD) - 1);

        spicelib::DASRDD(save.HANDLE, save.B, save.E, save.XDREC.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Get the number of the record containing address B.
        //
        spicelib::DASA2L(
            save.HANDLE,
            DPIX,
            save.B,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Read the record at index RECNO.
        //
        spicelib::ZZDASGRD(save.HANDLE, save.RECNO, save.DREC.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We expect an exact match.
        //
        fstr::assign(&mut save.LABEL, b"DREC@ record #");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.RECNO, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.DREC.as_slice(),
            b"=",
            save.XDREC.as_slice(),
            save.NREAD,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Prepare for the next read, if any.
        //
        save.REMAIN = (save.REMAIN - save.NREAD);
        save.B = (save.B + save.NREAD);
    }

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check d.p. records read from non-native DSK, comparing directly to native data.",
        ctx,
    )?;

    //
    // Get the native DSK's file summary info.
    //
    spicelib::DASOPR(DSK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASHFS(
        save.HANDLE,
        &mut save.NRESVR,
        &mut save.NRESVC,
        &mut save.NCOMR,
        &mut save.NCOMC,
        &mut save.FREE,
        save.LASTLA.as_slice_mut(),
        save.LASTRC.as_slice_mut(),
        save.LASTWD.as_slice_mut(),
        ctx,
    )?;

    //
    // Open the non-native DSK for read access as well.
    //
    spicelib::DASOPR(NNDSK, &mut save.NNHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Let NW be the number of d.p. data words in the file.
    //
    save.NW = save.LASTLA[DPIX];

    //
    // Read the d.p. data from the native file; compare with data that
    // we fetch from the non-native file using ZZDASGRD.
    //

    save.REMAIN = save.NW;
    save.B = 1;

    while ((save.REMAIN > 0) && !spicelib::FAILED(ctx)) {
        save.NREAD = intrinsics::MIN0(&[save.REMAIN, NWD]);
        save.E = ((save.B + save.NREAD) - 1);

        spicelib::DASRDD(save.HANDLE, save.B, save.E, save.XDREC.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Get the number of the record containing address B. We expect
        // this record number to be invariant across BFFs.
        //
        spicelib::DASA2L(
            save.HANDLE,
            DPIX,
            save.B,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Read the record at index RECNO in the non-native file.
        //
        spicelib::ZZDASGRD(save.NNHAN, save.RECNO, save.DREC.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We expect an exact match.
        //
        fstr::assign(&mut save.LABEL, b"DREC@ record #");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.RECNO, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.DREC.as_slice(),
            b"=",
            save.XDREC.as_slice(),
            save.NREAD,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Prepare for the next read, if any.
        //
        save.REMAIN = (save.REMAIN - save.NREAD);
        save.B = (save.B + save.NREAD);
    }

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASCLS(save.NNHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    // We test only exception (1) of ZZDASGRD. The others should
    // be covered by tests of lower-level routines.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDASGRD: Invalid file handle", ctx)?;

    spicelib::ZZDASGRD(-10000, save.RECNO, save.DREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(NOSUCHHANDLE)", OK, ctx)?;

    //***********************************************************************
    //
    //     Test ZZDASGRI
    //
    //***********************************************************************

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check integer records read from native DSK.", ctx)?;
    //
    // This is a consistency check only: DASRDI uses ZZDASGRI.
    // The next test case uses the independent routine DASIOI.
    //

    //
    // Get the DSK's file summary info.
    //
    spicelib::DASOPR(DSK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASHFS(
        save.HANDLE,
        &mut save.NRESVR,
        &mut save.NRESVC,
        &mut save.NCOMR,
        &mut save.NCOMC,
        &mut save.FREE,
        save.LASTLA.as_slice_mut(),
        save.LASTRC.as_slice_mut(),
        save.LASTWD.as_slice_mut(),
        ctx,
    )?;

    //
    // Let NW be the number of integer data words in the file.
    //
    save.NW = save.LASTLA[INTIX];

    //
    // Read the integer data; compare with data that we fetch using
    // ZZDASGRI.
    //

    save.REMAIN = save.NW;
    save.B = 1;

    while ((save.REMAIN > 0) && !spicelib::FAILED(ctx)) {
        save.NREAD = intrinsics::MIN0(&[save.REMAIN, NWI]);
        save.E = ((save.B + save.NREAD) - 1);

        spicelib::DASRDI(save.HANDLE, save.B, save.E, save.XIREC.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Get the number of the record containing address B.
        //
        spicelib::DASA2L(
            save.HANDLE,
            INTIX,
            save.B,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Read the record at index RECNO.
        //
        spicelib::ZZDASGRI(save.HANDLE, save.RECNO, save.IREC.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We expect an exact match.
        //
        fstr::assign(&mut save.LABEL, b"IREC@ record #");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.RECNO, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.LABEL,
            save.IREC.as_slice(),
            b"=",
            save.XIREC.as_slice(),
            save.NREAD,
            OK,
            ctx,
        )?;

        //
        // Prepare for the next read, if any.
        //
        save.REMAIN = (save.REMAIN - save.NREAD);
        save.B = (save.B + save.NREAD);
    }

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check integer records read from native DSK.Use unit-oriented record reader to fetch expected record.", ctx)?;

    //
    // Get the DSK's file summary info.
    //
    spicelib::DASOPR(DSK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASHFS(
        save.HANDLE,
        &mut save.NRESVR,
        &mut save.NRESVC,
        &mut save.NCOMR,
        &mut save.NCOMC,
        &mut save.FREE,
        save.LASTLA.as_slice_mut(),
        save.LASTRC.as_slice_mut(),
        save.LASTWD.as_slice_mut(),
        ctx,
    )?;

    //
    // Let NW be the number of integer data words in the file.
    //
    save.NW = save.LASTLA[INTIX];

    //
    // Read the integer data; compare with data that we fetch using
    // ZZDASGRI.
    //

    save.REMAIN = save.NW;
    save.B = 1;

    while ((save.REMAIN > 0) && !spicelib::FAILED(ctx)) {
        save.NREAD = intrinsics::MIN0(&[save.REMAIN, NWI]);
        save.E = ((save.B + save.NREAD) - 1);

        //
        // Get the number of the record containing address B.
        //
        spicelib::DASA2L(
            save.HANDLE,
            INTIX,
            save.B,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Get an unlocked unit and read the expected record.
        //
        spicelib::ZZDDHHLU(save.HANDLE, b"DAS", false, &mut save.UNIT, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DASIOI(
            b"READ",
            save.UNIT,
            save.RECNO,
            save.XIREC.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Read the record at index RECNO.
        //
        spicelib::ZZDASGRI(save.HANDLE, save.RECNO, save.IREC.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We expect an exact match.
        //
        fstr::assign(&mut save.LABEL, b"IREC@ record #");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.RECNO, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.LABEL,
            save.IREC.as_slice(),
            b"=",
            save.XIREC.as_slice(),
            save.NREAD,
            OK,
            ctx,
        )?;

        //
        // Prepare for the next read, if any.
        //
        save.REMAIN = (save.REMAIN - save.NREAD);
        save.B = (save.B + save.NREAD);
    }

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check integer records read from non-native DSK.", ctx)?;

    //
    // Get the DSK's file summary info.
    //
    spicelib::DASOPR(NNDSK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASHFS(
        save.HANDLE,
        &mut save.NRESVR,
        &mut save.NRESVC,
        &mut save.NCOMR,
        &mut save.NCOMC,
        &mut save.FREE,
        save.LASTLA.as_slice_mut(),
        save.LASTRC.as_slice_mut(),
        save.LASTWD.as_slice_mut(),
        ctx,
    )?;

    //
    // Let NW be the number of integer data words in the file.
    //
    save.NW = save.LASTLA[INTIX];

    //
    // Read the integer data; compare with data that we fetch using
    // ZZDASGRI.
    //

    save.REMAIN = save.NW;
    save.B = 1;

    while ((save.REMAIN > 0) && !spicelib::FAILED(ctx)) {
        save.NREAD = intrinsics::MIN0(&[save.REMAIN, NWI]);
        save.E = ((save.B + save.NREAD) - 1);

        spicelib::DASRDI(save.HANDLE, save.B, save.E, save.XIREC.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Get the number of the record containing address B.
        //
        spicelib::DASA2L(
            save.HANDLE,
            INTIX,
            save.B,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Read the record at index RECNO.
        //
        spicelib::ZZDASGRI(save.HANDLE, save.RECNO, save.IREC.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We expect an exact match.
        //
        fstr::assign(&mut save.LABEL, b"IREC@ record #");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.RECNO, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.LABEL,
            save.IREC.as_slice(),
            b"=",
            save.XIREC.as_slice(),
            save.NREAD,
            OK,
            ctx,
        )?;

        //
        // Prepare for the next read, if any.
        //
        save.REMAIN = (save.REMAIN - save.NREAD);
        save.B = (save.B + save.NREAD);
    }

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check integer records read from non-native DSK, comparing directly to native data.",
        ctx,
    )?;

    //
    // Get the native DSK's file summary info.
    //
    spicelib::DASOPR(DSK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASHFS(
        save.HANDLE,
        &mut save.NRESVR,
        &mut save.NRESVC,
        &mut save.NCOMR,
        &mut save.NCOMC,
        &mut save.FREE,
        save.LASTLA.as_slice_mut(),
        save.LASTRC.as_slice_mut(),
        save.LASTWD.as_slice_mut(),
        ctx,
    )?;

    //
    // Open the non-native DSK for read access as well.
    //
    spicelib::DASOPR(NNDSK, &mut save.NNHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Let NW be the number integer data words in the file.
    //
    save.NW = save.LASTLA[INTIX];

    //
    // Read the integer data from the native file; compare with data that
    // we fetch from the non-native file using ZZDASGRI.
    //

    save.REMAIN = save.NW;
    save.B = 1;

    while ((save.REMAIN > 0) && !spicelib::FAILED(ctx)) {
        save.NREAD = intrinsics::MIN0(&[save.REMAIN, NWI]);
        save.E = ((save.B + save.NREAD) - 1);

        spicelib::DASRDI(save.HANDLE, save.B, save.E, save.XIREC.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Get the number of the record containing address B. We expect
        // this record number to be invariant across BFFs.
        //
        spicelib::DASA2L(
            save.HANDLE,
            INTIX,
            save.B,
            &mut save.CLBASE,
            &mut save.CLSIZE,
            &mut save.RECNO,
            &mut save.WORDNO,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Read the record at index RECNO in the non-native file.
        //
        spicelib::ZZDASGRI(save.NNHAN, save.RECNO, save.IREC.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We expect an exact match.
        //
        fstr::assign(&mut save.LABEL, b"IREC@ record #");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.RECNO, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.LABEL,
            save.IREC.as_slice(),
            b"=",
            save.XIREC.as_slice(),
            save.NREAD,
            OK,
            ctx,
        )?;

        //
        // Prepare for the next read, if any.
        //
        save.REMAIN = (save.REMAIN - save.NREAD);
        save.B = (save.B + save.NREAD);
    }

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASCLS(save.NNHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    // We test only exception (1) of ZZDASGRI. The others should
    // be covered by tests of lower-level routines.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDASGRI: Invalid file handle", ctx)?;

    spicelib::ZZDASGRI(-10000, save.RECNO, save.IREC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(NOSUCHHANDLE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up: delete kernels.", ctx)?;

    if spicelib::EXISTS(PCK, ctx)? {
        spicelib::DELFIL(PCK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::UNLOAD(DSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(NNDSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(NNDSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
