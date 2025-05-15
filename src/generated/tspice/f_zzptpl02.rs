//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VERIDX: i32 = 1;
const LLBIDX: i32 = (VERIDX + 1);
const LLEIDX: i32 = (LLBIDX + 1);
const NULPTR: i32 = -1;
const BWDIDX: i32 = 1;
const FWDIDX: i32 = (BWDIDX + 1);
const IBSIDX: i32 = (FWDIDX + 1);
const ISZIDX: i32 = (IBSIDX + 1);
const DBSIDX: i32 = (ISZIDX + 1);
const DSZIDX: i32 = (DBSIDX + 1);
const CBSIDX: i32 = (DSZIDX + 1);
const CSZIDX: i32 = (CBSIDX + 1);
const DLADSZ: i32 = CSZIDX;
const FMTVER: i32 = 1000000;
const NCHREC: i32 = 1024;
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
const XFRACT: f64 = 0.0000000001;
const KEYXFR: i32 = 1;
const SGREED: f64 = 0.00000001;
const KEYSGR: i32 = (KEYXFR + 1);
const SGPADM: f64 = 0.0000000001;
const KEYSPM: i32 = (KEYSGR + 1);
const PTMEMM: f64 = 0.0000001;
const KEYPTM: i32 = (KEYSPM + 1);
const ANGMRG: f64 = 0.000000000001;
const KEYAMG: i32 = (KEYPTM + 1);
const LONALI: f64 = 0.000000000001;
const KEYLAL: i32 = (KEYAMG + 1);
const DSK0: &[u8] = b"zzptpl02_0.bds";
const PCK0: &[u8] = b"zzptpl02.tpc";
const NAMLEN: i32 = 32;
const LNSIZE: i32 = 80;

struct SaveVars {
    FRNAME: Vec<u8>,
    LABEL: Vec<u8>,
    BOXCTR: StackArray<f64, 3>,
    C: f64,
    DEFMRG: f64,
    DELTA: f64,
    DSKDSC: StackArray<f64, 24>,
    LR: f64,
    LT: f64,
    LZ: f64,
    MARGIN: f64,
    MAXR: f64,
    P: StackArray<f64, 3>,
    P2: StackArray<f64, 3>,
    NORMAL: StackArray<f64, 3>,
    SCALE: f64,
    VERTS: StackArray2D<f64, 9>,
    XV2: StackArray2D<f64, 9>,
    XVERTS: StackArray2D<f64, 9>,
    BODYID: i32,
    DLADSC: StackArray<i32, 8>,
    HANDLE: i32,
    N: i32,
    NXTDSC: StackArray<i32, 8>,
    NLAT: i32,
    NLON: i32,
    NP: i32,
    NV: i32,
    PLATE: StackArray<i32, 3>,
    PLID: i32,
    SURFID: i32,
    XPLATE: StackArray<i32, 3>,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FRNAME = vec![b' '; NAMLEN as usize];
        let mut LABEL = vec![b' '; LNSIZE as usize];
        let mut BOXCTR = StackArray::<f64, 3>::new(1..=3);
        let mut C: f64 = 0.0;
        let mut DEFMRG: f64 = 0.0;
        let mut DELTA: f64 = 0.0;
        let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
        let mut LR: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut LZ: f64 = 0.0;
        let mut MARGIN: f64 = 0.0;
        let mut MAXR: f64 = 0.0;
        let mut P = StackArray::<f64, 3>::new(1..=3);
        let mut P2 = StackArray::<f64, 3>::new(1..=3);
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut SCALE: f64 = 0.0;
        let mut VERTS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XV2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XVERTS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut BODYID: i32 = 0;
        let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut HANDLE: i32 = 0;
        let mut N: i32 = 0;
        let mut NXTDSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NP: i32 = 0;
        let mut NV: i32 = 0;
        let mut PLATE = StackArray::<i32, 3>::new(1..=3);
        let mut PLID: i32 = 0;
        let mut SURFID: i32 = 0;
        let mut XPLATE = StackArray::<i32, 3>::new(1..=3);
        let mut FOUND: bool = false;

        Self {
            FRNAME,
            LABEL,
            BOXCTR,
            C,
            DEFMRG,
            DELTA,
            DSKDSC,
            LR,
            LT,
            LZ,
            MARGIN,
            MAXR,
            P,
            P2,
            NORMAL,
            SCALE,
            VERTS,
            XV2,
            XVERTS,
            BODYID,
            DLADSC,
            HANDLE,
            N,
            NXTDSC,
            NLAT,
            NLON,
            NP,
            NV,
            PLATE,
            PLID,
            SURFID,
            XPLATE,
            FOUND,
        }
    }
}

//$Procedure F_ZZPTPL02 ( ZZPTPL02 tests )
pub fn F_ZZPTPL02(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Saved values
    //
    // Save variables in order to avoid stack room problems.
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZPTPL02", ctx)?;

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // The following simple cases are meant to assist debugging.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Set-up: create tessellated ellipsoid DSK.", ctx)?;

    //
    // Make sure no kernels are loaded before we start.
    //
    spicelib::KCLEAR(ctx)?;

    //
    // Create, load, and discard generic test PCK.
    //
    testutil::TSTPCK(PCK0, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create DSK.
    //
    // TARGET = 'MARS'
    fstr::assign(&mut save.FRNAME, b"IAU_MARS");
    save.BODYID = 499;
    save.SURFID = 1;

    save.NLON = 40;
    save.NLAT = 20;

    if spicelib::EXISTS(DSK0, ctx)? {
        spicelib::DELFIL(DSK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::T_ELDS2Z(
        save.BODYID,
        save.SURFID,
        &save.FRNAME,
        save.NLON,
        save.NLAT,
        DSK0,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Append a segment for Phobos. Make this segment large enough
    // so that the buffering capability in ZZPTPL02 will be
    // exercised.
    //
    // TARGET = 'Phobos'
    fstr::assign(&mut save.FRNAME, b"IAU_PHOBOS");
    save.BODYID = 401;
    save.SURFID = 1;

    save.NLON = 200;
    save.NLAT = 100;

    testutil::T_ELDS2Z(
        save.BODYID,
        save.SURFID,
        &save.FRNAME,
        save.NLON,
        save.NLAT,
        DSK0,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Mars segment: map plate centroids to plates.", ctx)?;

    //
    // This should be easy....
    //
    // Get DLA descriptor.
    //
    spicelib::DASOPR(DSK0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DLABFS(
        save.HANDLE,
        save.DLADSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Get the DSK descriptor, which will be needed a bit later.
    //
    spicelib::DSKGD(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get vertex and plate counts.
    //
    spicelib::DSKZ02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        &mut save.NV,
        &mut save.NP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.NP {
        //
        // Get Ith plate and its vertices.
        //
        spicelib::DSKP02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            1,
            &mut save.N,
            save.XPLATE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=3 {
            spicelib::DSKV02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.XPLATE[J],
                1,
                &mut save.N,
                save.XVERTS.subarray_mut([1, J]),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Compute the plate's centroid.
        //
        save.C = (1.0 / 3 as f64);

        spicelib::VLCOM3(
            save.C,
            save.XVERTS.subarray([1, 1]),
            save.C,
            save.XVERTS.subarray([1, 2]),
            save.C,
            save.XVERTS.subarray([1, 3]),
            save.P.as_slice_mut(),
        );

        //
        // Find the plate associated with P. We're expecting plate I.
        //
        spicelib::ZZPTPL02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            save.DSKDSC.as_slice(),
            save.P.as_slice(),
            &mut save.PLID,
            save.PLATE.as_slice_mut(),
            save.VERTS.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        //
        // Make sure the centroid was found on the correct plate.
        //
        fstr::assign(&mut save.LABEL, b"centroid PLID #.");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(&save.LABEL, save.PLID, b"=", I, 0, OK, ctx)?;

        //
        // Check the returned plate.
        //
        fstr::assign(&mut save.LABEL, b"plate PLID #.");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.LABEL,
            save.PLATE.as_slice(),
            b"=",
            save.XPLATE.as_slice(),
            3,
            OK,
            ctx,
        )?;

        //
        // Check the returned vertices. We expect an exact match.
        //
        fstr::assign(&mut save.LABEL, b"vertices PLID #.");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.VERTS.as_slice(),
            b"=",
            save.XVERTS.as_slice(),
            3,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Mars segment: map off-plate points near plate centroids to plates.",
        ctx,
    )?;

    for I in 1..=save.NP {
        //
        // Get Ith plate and its vertices.
        //
        spicelib::DSKP02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            1,
            &mut save.N,
            save.XPLATE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=3 {
            spicelib::DSKV02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.XPLATE[J],
                1,
                &mut save.N,
                save.XVERTS.subarray_mut([1, J]),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Compute the plate's centroid.
        //
        save.C = (1.0 / 3 as f64);

        spicelib::VLCOM3(
            save.C,
            save.XVERTS.subarray([1, 1]),
            save.C,
            save.XVERTS.subarray([1, 2]),
            save.C,
            save.XVERTS.subarray([1, 3]),
            save.P.as_slice_mut(),
        );

        //
        // Get the normal vector of plate I; add a small upward
        // displacement

        //
        spicelib::DSKN02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            save.NORMAL.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.SCALE = 0.0000000001;

        spicelib::VLCOM(
            1.0,
            save.P.as_slice(),
            save.SCALE,
            save.NORMAL.as_slice(),
            save.P2.as_slice_mut(),
        );

        //
        // Find the plate associated with P2. We're expecting plate I.
        //
        spicelib::ZZPTPL02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            save.DSKDSC.as_slice(),
            save.P2.as_slice(),
            &mut save.PLID,
            save.PLATE.as_slice_mut(),
            save.VERTS.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        //
        // Make sure the offst centroid was found on the correct plate.
        //
        fstr::assign(&mut save.LABEL, b"(up) offset centroid PLID #.");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(&save.LABEL, save.PLID, b"=", I, 0, OK, ctx)?;

        //
        // Check the returned plate.
        //
        fstr::assign(&mut save.LABEL, b"(up) plate PLID #.");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.LABEL,
            save.PLATE.as_slice(),
            b"=",
            save.XPLATE.as_slice(),
            3,
            OK,
            ctx,
        )?;

        //
        // Check the returned vertices. We expect an exact match.
        //
        fstr::assign(&mut save.LABEL, b"(up) vertices PLID #.");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.VERTS.as_slice(),
            b"=",
            save.XVERTS.as_slice(),
            3,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Add a small downward displacement to the centroid.
        //
        spicelib::DSKN02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            save.NORMAL.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VLCOM(
            1.0,
            save.P.as_slice(),
            -save.SCALE,
            save.NORMAL.as_slice(),
            save.P2.as_slice_mut(),
        );

        //
        // Find the plate associated with P2. We're expecting plate I.
        //
        spicelib::ZZPTPL02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            save.DSKDSC.as_slice(),
            save.P2.as_slice(),
            &mut save.PLID,
            save.PLATE.as_slice_mut(),
            save.VERTS.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        //
        // Make sure the offset centroid was found on the correct plate.
        //
        fstr::assign(&mut save.LABEL, b"(down) offset centroid PLID #.");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(&save.LABEL, save.PLID, b"=", I, 0, OK, ctx)?;

        //
        // Check the returned plate.
        //
        fstr::assign(&mut save.LABEL, b"(down) plate PLID #.");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.LABEL,
            save.PLATE.as_slice(),
            b"=",
            save.XPLATE.as_slice(),
            3,
            OK,
            ctx,
        )?;

        //
        // Check the returned vertices. We expect an exact match.
        //
        fstr::assign(&mut save.LABEL, b"(down) vertices PLID #.");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.VERTS.as_slice(),
            b"=",
            save.XVERTS.as_slice(),
            3,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Mars segment: attempt to map off-plate points near centroids to plates, using offsets that are too large. No matches should be found.", ctx)?;

    //
    // Get default membership margin.
    //
    spicelib::DSKGTL(KEYPTM, &mut save.DEFMRG, ctx)?;

    //
    // Get the segment's bounding radius.
    //
    spicelib::ZZLATBOX(
        save.DSKDSC.subarray(MN1IDX),
        save.BOXCTR.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.MAXR,
        ctx,
    )?;

    //
    // Derive the applicable margin.
    //
    save.MARGIN = (save.DEFMRG * save.MAXR);

    for I in 1..=save.NP {
        //
        // Get Ith plate and its vertices.
        //
        spicelib::DSKP02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            1,
            &mut save.N,
            save.XPLATE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=3 {
            spicelib::DSKV02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.XPLATE[J],
                1,
                &mut save.N,
                save.XVERTS.subarray_mut([1, J]),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Compute the plate's centroid.
        //
        save.C = (1.0 / 3 as f64);

        spicelib::VLCOM3(
            save.C,
            save.XVERTS.subarray([1, 1]),
            save.C,
            save.XVERTS.subarray([1, 2]),
            save.C,
            save.XVERTS.subarray([1, 3]),
            save.P.as_slice_mut(),
        );

        //
        // Get the normal vector of plate I; add a small upward
        // displacement

        //
        spicelib::DSKN02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            save.NORMAL.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.SCALE = ((2 as f64) * save.MARGIN);

        spicelib::VLCOM(
            1.0,
            save.P.as_slice(),
            save.SCALE,
            save.NORMAL.as_slice(),
            save.P2.as_slice_mut(),
        );

        //
        // Try to find the plate associated with P2. We expect no match.
        //
        spicelib::ZZPTPL02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            save.DSKDSC.as_slice(),
            save.P2.as_slice(),
            &mut save.PLID,
            save.PLATE.as_slice_mut(),
            save.VERTS.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

        //
        // Add a small downward displacement to the centroid.
        //
        spicelib::DSKN02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            save.NORMAL.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VLCOM(
            1.0,
            save.P.as_slice(),
            -save.SCALE,
            save.NORMAL.as_slice(),
            save.P2.as_slice_mut(),
        );

        //
        // Try to find the plate associated with P2. We expect no match.
        //
        spicelib::ZZPTPL02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            save.DSKDSC.as_slice(),
            save.P2.as_slice(),
            &mut save.PLID,
            save.PLATE.as_slice_mut(),
            save.VERTS.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Mars: map plate interior points near vertices to plates.",
        ctx,
    )?;

    //
    // We're still working with the same segment as last time.
    //
    for I in 1..=save.NP {
        //
        // Get Ith plate and its vertices.
        //
        spicelib::DSKP02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            1,
            &mut save.N,
            save.XPLATE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=3 {
            spicelib::DSKV02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.XPLATE[J],
                1,
                &mut save.N,
                save.XVERTS.subarray_mut([1, J]),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Shrink the plate very slightly. The vertices of the new plate
        // will be inside the boundary of the original.
        //
        // We can't put the vertices too near the edges of the original
        // plates, or the expansion of other plates may cause those
        // plates to "capture" our vertices.
        //
        // Note the sign of DELTA.
        //
        save.DELTA = -0.000000001;

        spicelib::PLTEXP(save.XVERTS.as_slice(), save.DELTA, save.XV2.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Find the plates associated with the vertices of the scaled
        // plate.
        //
        for J in 1..=3 {
            spicelib::VEQU(save.XV2.subarray([1, J]), save.P.as_slice_mut());
            //
            // Find the plate associated with P. We're expecting plate I.
            //
            spicelib::ZZPTPL02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.DSKDSC.as_slice(),
                save.P.as_slice(),
                &mut save.PLID,
                save.PLATE.as_slice_mut(),
                save.VERTS.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            //
            // Make sure the vertex was found on the correct plate.
            //
            fstr::assign(&mut save.LABEL, b"vertex # PLID #.");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(&save.LABEL, save.PLID, b"=", I, 0, OK, ctx)?;

            //
            // Check the returned plate.
            //
            fstr::assign(&mut save.LABEL, b"vertex # plate PLID #.");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAI(
                &save.LABEL,
                save.PLATE.as_slice(),
                b"=",
                save.XPLATE.as_slice(),
                3,
                OK,
                ctx,
            )?;

            //
            // Check the returned vertices. We expect an exact match.
            //
            fstr::assign(&mut save.LABEL, b"vertex # vertices PLID #.");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.VERTS.as_slice(),
                b"=",
                save.XVERTS.as_slice(),
                3,
                0.0,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Mars: map plate interior points near vertices to plates. Turn off default plate expansion and use a very small shrinkage factor.", ctx)?;

    //
    // Shut down plate expansion.
    //
    spicelib::DSKSTL(KEYXFR, 0.0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We're still working with the same segment as last time.
    //
    for I in 1..=save.NP {
        //
        // Get Ith plate and its vertices.
        //
        spicelib::DSKP02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            1,
            &mut save.N,
            save.XPLATE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=3 {
            spicelib::DSKV02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.XPLATE[J],
                1,
                &mut save.N,
                save.XVERTS.subarray_mut([1, J]),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Shrink the plate very slightly. The vertices of the new plate
        // will be inside the boundary of the original.
        //
        // Use a very small scale factor. This factor won't work with
        // default plate expansion turned on.
        //
        // Note the sign of DELTA.
        //
        save.DELTA = -0.000000000001;

        spicelib::PLTEXP(save.XVERTS.as_slice(), save.DELTA, save.XV2.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Find the plates associated with the vertices of the scaled
        // plate.
        //
        for J in 1..=3 {
            spicelib::VEQU(save.XV2.subarray([1, J]), save.P.as_slice_mut());
            //
            // Find the plate associated with P. We're expecting plate I.
            //
            spicelib::ZZPTPL02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.DSKDSC.as_slice(),
                save.P.as_slice(),
                &mut save.PLID,
                save.PLATE.as_slice_mut(),
                save.VERTS.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            //
            // Make sure the vertex was found on the correct plate.
            //
            fstr::assign(&mut save.LABEL, b"vertex # PLID #.");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(&save.LABEL, save.PLID, b"=", I, 0, OK, ctx)?;

            //
            // Check the returned plate.
            //
            fstr::assign(&mut save.LABEL, b"vertex # plate PLID #.");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAI(
                &save.LABEL,
                save.PLATE.as_slice(),
                b"=",
                save.XPLATE.as_slice(),
                3,
                OK,
                ctx,
            )?;

            //
            // Check the returned vertices. We expect an exact match.
            //
            fstr::assign(&mut save.LABEL, b"vertex # vertices PLID #.");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.VERTS.as_slice(),
                b"=",
                save.XVERTS.as_slice(),
                3,
                0.0,
                OK,
                ctx,
            )?;
        }
    }

    //
    // Restore default plate expansion.
    //
    spicelib::DSKSTL(KEYXFR, XFRACT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Phobos segment: map plate centroids to plates.", ctx)?;

    //
    // Make sure that a segment change is not a problem.
    //
    // Get DLA descriptor.
    //
    spicelib::DLAFNS(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.NXTDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::MOVEI(save.NXTDSC.as_slice(), DLADSZ, save.DLADSC.as_slice_mut());

    //
    // Get the DSK descriptor, which will be needed a bit later.
    //
    spicelib::DSKGD(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get vertex and plate counts.
    //
    spicelib::DSKZ02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        &mut save.NV,
        &mut save.NP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.NP {
        //
        // Get Ith plate and its vertices.
        //
        spicelib::DSKP02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            1,
            &mut save.N,
            save.XPLATE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=3 {
            spicelib::DSKV02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.XPLATE[J],
                1,
                &mut save.N,
                save.XVERTS.subarray_mut([1, J]),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Compute the plate's centroid.
        //
        save.C = (1.0 / 3 as f64);

        spicelib::VLCOM3(
            save.C,
            save.XVERTS.subarray([1, 1]),
            save.C,
            save.XVERTS.subarray([1, 2]),
            save.C,
            save.XVERTS.subarray([1, 3]),
            save.P.as_slice_mut(),
        );

        //
        // Find the plate associated with P. We're expecting plate I.
        //
        spicelib::ZZPTPL02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            save.DSKDSC.as_slice(),
            save.P.as_slice(),
            &mut save.PLID,
            save.PLATE.as_slice_mut(),
            save.VERTS.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        //
        // Make sure the centroid was found on the correct plate.
        //
        fstr::assign(&mut save.LABEL, b"centroid PLID #.");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(&save.LABEL, save.PLID, b"=", I, 0, OK, ctx)?;

        //
        // Check the returned plate.
        //
        fstr::assign(&mut save.LABEL, b"plate PLID #.");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.LABEL,
            save.PLATE.as_slice(),
            b"=",
            save.XPLATE.as_slice(),
            3,
            OK,
            ctx,
        )?;

        //
        // Check the returned vertices. We expect an exact match.
        //
        fstr::assign(&mut save.LABEL, b"vertices PLID #.");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.VERTS.as_slice(),
            b"=",
            save.XVERTS.as_slice(),
            3,
            0.0,
            OK,
            ctx,
        )?;
    }

    //**********************************************************************
    //
    //     Error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case setup: get valid descriptors.", ctx)?;
    //
    // Get DLA descriptor.
    //
    spicelib::DLABFS(
        save.HANDLE,
        save.DLADSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Get the DSK descriptor, which will be needed a bit later.
    //
    spicelib::DSKGD(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note: exceptions are only for bad segment parameters,
    // bad DSK descriptor and file read errors.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid file handle", ctx)?;

    spicelib::ZZPTPL02(
        (save.HANDLE + 1),
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        save.P.as_slice(),
        &mut save.PLID,
        save.PLATE.as_slice_mut(),
        save.VERTS.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDHANDLE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad coordinate system.", ctx)?;

    save.DSKDSC[SYSIDX] = -1 as f64;

    spicelib::ZZPTPL02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        save.P.as_slice(),
        &mut save.PLID,
        save.PLATE.as_slice_mut(),
        save.VERTS.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // Restore coordinate system.
    //
    save.DSKDSC[SYSIDX] = LATSYS as f64;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    //
    // Close DSK file; open was performed using DASOPR.
    //
    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
