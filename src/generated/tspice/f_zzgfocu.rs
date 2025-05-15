//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const PCK: &[u8] = b"nat.tpc";
const SPK: &[u8] = b"nat.bsp";

struct SaveVars {
    ET: f64,
    NEWRAD: StackArray<f64, 3>,
    RADII: StackArray<f64, 3>,
    HANDLE: i32,
    N: i32,
    OCSTAT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ET: f64 = 0.0;
        let mut NEWRAD = StackArray::<f64, 3>::new(1..=3);
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut HANDLE: i32 = 0;
        let mut N: i32 = 0;
        let mut OCSTAT: bool = false;

        Self {
            ET,
            NEWRAD,
            RADII,
            HANDLE,
            N,
            OCSTAT,
        }
    }
}

//$Procedure      F_ZZGFOCU ( ZZGFOCU family tests )
pub fn F_ZZGFOCU(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Save everything.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFOCU", ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    testutil::NATSPK(SPK, true, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::NATPCK(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFOCU: bogus entry", ctx)?;

    save.ET = 0.0;

    spicelib::ZZGFOCU(
        b"Any",
        b"ALPHAx",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"SUN",
        b"LT",
        save.ET,
        save.OCSTAT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BOGUSENTRY)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFOCIN: Unrecognized targets or observer", ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"ALPHAx",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETAx",
        b"ellipsoid",
        b"BETAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"SUNx",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //

    testutil::TCASE(b"ZZGFOCIN: non-positive radii for back target", ctx)?;

    spicelib::BODVRD(
        b"ALPHA",
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MOVED(save.RADII.as_slice(), 3, save.NEWRAD.as_slice_mut());
    save.NEWRAD[1] = 0.0;

    spicelib::PDPOOL(b"BODY1000_RADII", 3, save.NEWRAD.as_slice(), ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"point",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::MOVED(save.RADII.as_slice(), 3, save.NEWRAD.as_slice_mut());
    save.NEWRAD[2] = -1.0;

    spicelib::PDPOOL(b"BODY1000_RADII", 3, save.NEWRAD.as_slice(), ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"point",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::MOVED(save.RADII.as_slice(), 3, save.NEWRAD.as_slice_mut());
    save.NEWRAD[3] = -1.0;

    spicelib::PDPOOL(b"BODY1000_RADII", 3, save.NEWRAD.as_slice(), ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"point",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // Restore original radii.
    //
    spicelib::PDPOOL(b"BODY1000_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //

    testutil::TCASE(b"ZZGFOCIN: non-positive radii for front target", ctx)?;

    spicelib::BODVRD(
        b"BETA",
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MOVED(save.RADII.as_slice(), 3, save.NEWRAD.as_slice_mut());
    save.NEWRAD[1] = 0.0;

    spicelib::PDPOOL(b"BODY2000_RADII", 3, save.NEWRAD.as_slice(), ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::MOVED(save.RADII.as_slice(), 3, save.NEWRAD.as_slice_mut());
    save.NEWRAD[2] = -1.0;

    spicelib::PDPOOL(b"BODY2000_RADII", 3, save.NEWRAD.as_slice(), ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::MOVED(save.RADII.as_slice(), 3, save.NEWRAD.as_slice_mut());
    save.NEWRAD[3] = -1.0;

    spicelib::PDPOOL(b"BODY2000_RADII", 3, save.NEWRAD.as_slice(), ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // Restore original radii.
    //
    spicelib::PDPOOL(b"BODY2000_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //

    testutil::TCASE(b"ZZGFOCIN: bad radius count for back target", ctx)?;

    spicelib::BODVRD(
        b"ALPHA",
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MOVED(save.RADII.as_slice(), 3, save.NEWRAD.as_slice_mut());
    save.NEWRAD[1] = 0.0;

    spicelib::PDPOOL(b"BODY1000_RADII", 2, save.NEWRAD.as_slice(), ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"point",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //
    // Restore original radii.
    //
    spicelib::PDPOOL(b"BODY1000_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //

    testutil::TCASE(b"ZZGFOCIN: bad radius count for front target", ctx)?;

    spicelib::BODVRD(
        b"BETA",
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MOVED(save.RADII.as_slice(), 3, save.NEWRAD.as_slice_mut());
    save.NEWRAD[1] = 0.0;

    spicelib::PDPOOL(b"BODY2000_RADII", 2, save.NEWRAD.as_slice(), ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //
    // Restore original radii.
    //
    spicelib::PDPOOL(b"BODY2000_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFOCIN: blank frames for ellipsoidal targets", ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"ALPHA",
        b"ELLIPSOID",
        b" ",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"ALPHA",
        b"ELLIPSOID",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b" ",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFOCIN: bad frame centers for ellipsoidal targets", ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"ALPHA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"ALPHA",
        b"ELLIPSOID",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //

    testutil::TCASE(b"ZZGFOCIN: bad shape combination", ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"point",
        b"BETAFIXED",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSHAPECOMBO)", OK, ctx)?;

    //---- Case -------------------------------------------------------------
    //

    testutil::TCASE(b"ZZGFOCIN: bad target shape", ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"sphere",
        b"BETAFIXED",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"ALPHA",
        b"sphere",
        b"ALPHAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //

    testutil::TCASE(b"ZZGFIIOC: ellipsoid case: participants coincide", ctx)?;

    //
    // Front ellipsoid target is observer.
    //
    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"BETA",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    // ET = 0.D0

    // CALL ZZGFIOCC ( ET, OCSTAT )
    // CALL CHCKXC ( .TRUE., 'SPICE(NOTDISJOINT)',   OK )

    //
    // Back ellipsoid target is observer.
    //
    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"ALPHA",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    // ET = 0.D0

    // CALL ZZGFIOCC ( ET, OCSTAT )
    // CALL CHCKXC ( .TRUE., 'SPICE(NOTDISJOINT)',   OK )

    //
    // Front ellipsoid target is back target.
    //
    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    // CALL CHCKXC ( .FALSE., ' ',   OK )
    // CALL CHCKXC ( .FALSE., ' ',   OK )

    // ET = 0.D0

    // CALL ZZGFIOCC ( ET, OCSTAT )
    // CALL CHCKXC ( .TRUE., 'SPICE(NOTDISJOINT)',   OK )

    //
    //---- Case -------------------------------------------------------------
    //

    testutil::TCASE(b"ZZGFIIOC: point target case: participants coincide", ctx)?;

    //
    // Front point target is observer.
    //
    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"point",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"BETA",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    // Back point target is observer.
    //
    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"ALPHA",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    // Front point target is back target.
    //
    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"point",
        b"BETAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    // Back point target is front target.
    //
    spicelib::ZZGFOCIN(
        b"Any",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"BETA",
        b"point",
        b"BETAFIXED",
        b"SUN",
        b"LT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //

    testutil::TCASE(b"Bad aberration correction values.", ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"ALPHA",
        b"point",
        b"NULL",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"SUN",
        b"S",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"ALPHA",
        b"point",
        b"NULL",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"SUN",
        b"XS",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"ALPHA",
        b"point",
        b"NULL",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"SUN",
        b"RLT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"ALPHA",
        b"point",
        b"NULL",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"SUN",
        b"XRLT",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZGFOCIN(
        b"Any",
        b"ALPHA",
        b"point",
        b"NULL",
        b"BETA",
        b"ELLIPSOID",
        b"BETAFIXED",
        b"SUN",
        b"z",
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //

    testutil::TCASE(b"Clean up. Unload and delete kernels.", ctx)?;

    //
    // Clean up the SPK file.
    //
    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
