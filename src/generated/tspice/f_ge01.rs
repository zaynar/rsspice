//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const PCK: &[u8] = b"test.tpc";
const SPK: &[u8] = b"test.bsp";
const UTC: &[u8] = b"1999 JAN 1";
const LOOSE: f64 = 0.0000001;
const MED: f64 = 0.00000000001;
const TIGHT: f64 = 0.000000000001;
const VTIGHT: f64 = 0.00000000000001;
const MSGLEN: i32 = 240;

//$Procedure F_GE01 ( SPICE higher-level geometry routine tests )
pub fn F_GE01(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TITLE = [b' '; MSGLEN as usize];
    let mut ALT: f64 = 0.0;
    let mut EMISSN: f64 = 0.0;
    let mut ET: f64 = 0.0;
    let mut EXPPNT = StackArray::<f64, 3>::new(1..=3);
    let mut F: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut PHASE: f64 = 0.0;
    let mut RAD: f64 = 0.0;
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut RE: f64 = 0.0;
    let mut RP: f64 = 0.0;
    let mut SOLAR: f64 = 0.0;
    let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
    let mut SUNALT: f64 = 0.0;
    let mut SUNLAT: f64 = 0.0;
    let mut SUNLON: f64 = 0.0;
    let mut SUNRAD: f64 = 0.0;
    let mut SUNSTA = StackArray::<f64, 6>::new(1..=6);
    let mut DIM: i32 = 0;
    let mut HANDLE: i32 = 0;

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
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_GE01", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup:  load kernels.", ctx)?;

    //
    // Leapseconds:  Note that the LSK is deleted after loading, so we
    // don't have to clean it up later.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create and load a PCK file. Delete the file afterwards.
    //
    testutil::TSTPCK(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load an SPK file as well.
    //
    testutil::TSTSPK(SPK, true, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Test SUBPT.  Find the sub-solar point of the sun on the Earth using the INTERCEPT definition.");

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::STR2ET(UTC, &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SUBPT(
        b"INTERCEPT",
        b"EARTH",
        ET,
        b"NONE",
        b"SUN",
        SPOINT.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::RECLAT(SPOINT.as_slice(), &mut RAD, &mut LON, &mut LAT);

    //
    // Get the state of the sun in Earth bodyfixed coordinates at ET.
    //
    spicelib::SPKGEO(
        10,
        ET,
        b"IAU_EARTH",
        399,
        SUNSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::RECLAT(SUNSTA.as_slice(), &mut SUNRAD, &mut SUNLON, &mut SUNLAT);

    //
    // Make sure the directional coordinates match up.
    //
    testutil::CHCKSD(b"Sub point lon", LON, b"~", SUNLON, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"Sub point lat", LAT, b"~", SUNLAT, TIGHT, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Test SUBPT.  Find the sub-solar point of the sun on the Earth using the NEAR POINT definition.");

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::STR2ET(UTC, &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure that strings representing integers are parsed correctly.
    //
    spicelib::SUBPT(
        b"NEAR POINT",
        b"399",
        ET,
        b"NONE",
        b"10",
        SPOINT.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We'll need the radii of the earth.
    //
    spicelib::BODVRD(b"EARTH", b"RADII", 3, &mut DIM, RADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    RE = RADII[1];
    RP = RADII[3];

    F = ((RE - RP) / RE);

    spicelib::RECGEO(SPOINT.as_slice(), RE, F, &mut LON, &mut LAT, &mut ALT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the state of the sun in Earth bodyfixed coordinates at ET.
    //
    spicelib::SPKGEO(
        10,
        ET,
        b"IAU_EARTH",
        399,
        SUNSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::RECGEO(
        SUNSTA.as_slice(),
        RE,
        F,
        &mut SUNLON,
        &mut SUNLAT,
        &mut SUNALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure the directional coordinates match up.
    //
    testutil::CHCKSD(b"Sub point lon", LON, b"~", SUNLON, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"Sub point lat", LAT, b"~", SUNLAT, TIGHT, OK, ctx)?;

    //
    // SUBPT error cases:
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid method.", ctx)?;

    spicelib::SUBPT(
        b"NARPOINT",
        b"399",
        ET,
        b"NONE",
        b"10",
        SPOINT.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DUBIOUSMETHOD)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid observer name.", ctx)?;

    spicelib::SUBPT(
        b"Nearpoint",
        b"earth",
        ET,
        b"NONE",
        b"sn",
        SPOINT.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid target name.", ctx)?;

    spicelib::SUBPT(
        b"Nearpoint",
        b"erth",
        ET,
        b"NONE",
        b"sun",
        SPOINT.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Observer is target.", ctx)?;

    spicelib::SUBPT(
        b"Nearpoint",
        b"earth",
        ET,
        b"NONE",
        b"earth",
        SPOINT.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    //     ILLUM tests follow.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Test ILLUM.  Find the illumination angles on the earth as seen from the moon, evaluated at the sub-moon point (NEARPOINT method).");

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SUBPT(
        b"Nearpoint",
        b"earth",
        ET,
        b"NONE",
        b"moon",
        SPOINT.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ILLUM(
        b"EARTH",
        ET,
        b"NONE",
        b"MOON",
        SPOINT.as_slice(),
        &mut PHASE,
        &mut SOLAR,
        &mut EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should have an emission angle of zero.
    //
    testutil::CHCKSD(b"Emission angle", EMISSN, b"~", 0.0, TIGHT, OK, ctx)?;

    //
    // The phase angle should match the solar incidence angle.
    //
    testutil::CHCKSD(b"Phase angle", PHASE, b"~", SOLAR, TIGHT, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Repeat tests with integer codes.", ctx)?;

    spicelib::SUBPT(
        b"Nearpoint",
        b"399",
        ET,
        b"NONE",
        b"301",
        SPOINT.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ILLUM(
        b"399",
        ET,
        b"NONE",
        b"301",
        SPOINT.as_slice(),
        &mut PHASE,
        &mut SOLAR,
        &mut EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should have an emission angle of zero.
    //
    testutil::CHCKSD(b"Emission angle", EMISSN, b"~", 0.0, TIGHT, OK, ctx)?;

    //
    // The phase angle should match the solar incidence angle.
    //
    testutil::CHCKSD(b"Phase angle", PHASE, b"~", SOLAR, TIGHT, OK, ctx)?;

    //
    //     Now make the sun the observer:  test the solar incidence
    //     angle.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Test ILLUM.  Find the illumination angles on the earth as seen from the sun, evaluated at the sub-sun point (NEARPOINT method).");

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SUBPT(
        b"Nearpoint",
        b"earth",
        ET,
        b"NONE",
        b"sun",
        SPOINT.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ILLUM(
        b"EARTH",
        ET,
        b"NONE",
        b"sun",
        SPOINT.as_slice(),
        &mut PHASE,
        &mut SOLAR,
        &mut EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should have an solar incidence angle of zero.
    //
    testutil::CHCKSD(b"Solar inc. angle", SOLAR, b"~", 0.0, TIGHT, OK, ctx)?;

    //
    // The phase angle should match the solar incidence angle.
    //
    testutil::CHCKSD(b"Phase angle", PHASE, b"~", SOLAR, TIGHT, OK, ctx)?;

    //
    //     ILLUM error cases:
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid observer name.", ctx)?;

    spicelib::ILLUM(
        b"earth",
        ET,
        b"NONE",
        b"son",
        SPOINT.as_slice(),
        &mut PHASE,
        &mut SOLAR,
        &mut EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid target name.", ctx)?;

    spicelib::ILLUM(
        b"erth",
        ET,
        b"NONE",
        b"sun",
        SPOINT.as_slice(),
        &mut PHASE,
        &mut SOLAR,
        &mut EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Observer is target.", ctx)?;

    spicelib::ILLUM(
        b"SUN",
        ET,
        b"NONE",
        b"sun",
        SPOINT.as_slice(),
        &mut PHASE,
        &mut SOLAR,
        &mut EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No body-fixed frame associated with target.", ctx)?;

    spicelib::ILLUM(
        b"mars express",
        ET,
        b"NONE",
        b"sun",
        SPOINT.as_slice(),
        &mut PHASE,
        &mut SOLAR,
        &mut EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOFRAME)", OK, ctx)?;

    //
    //     SUBSOL tests follow.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Test SUBSOL.  Find the sub-solar point of the sun on the Earth using the NEARPOINT definition.");

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::STR2ET(UTC, &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SUBSOL(
        b"NEARPOINT",
        b"EARTH",
        ET,
        b"NONE",
        b"SUN",
        SPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SUBPT(
        b"NEARPOINT",
        b"EARTH",
        ET,
        b"NONE",
        b"SUN",
        EXPPNT.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::RECLAT(SPOINT.as_slice(), &mut RAD, &mut LON, &mut LAT);

    //
    // Make sure the surface points match up.
    //
    testutil::CHCKAD(
        b"Geometric sub solar point",
        SPOINT.as_slice(),
        b"~~/",
        EXPPNT.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Test SUBSOL.  Repeat test using integer codes.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::STR2ET(UTC, &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SUBSOL(
        b"NEARPOINT",
        b"399",
        ET,
        b"NONE",
        b"10",
        SPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SUBPT(
        b"NEARPOINT",
        b"EARTH",
        ET,
        b"NONE",
        b"SUN",
        EXPPNT.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::RECLAT(SPOINT.as_slice(), &mut RAD, &mut LON, &mut LAT);

    //
    // Make sure the surface points match up.
    //
    testutil::CHCKAD(
        b"Geometric sub solar point",
        SPOINT.as_slice(),
        b"~~/",
        EXPPNT.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Test SUBSOL.  Find the sub-solar point of the sun on the Earth using the INTERCEPT definition.");

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::STR2ET(UTC, &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SUBSOL(
        b"INTERCEPT",
        b"EARTH",
        ET,
        b"NONE",
        b"SUN",
        SPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SUBPT(
        b"INTERCEPT",
        b"EARTH",
        ET,
        b"NONE",
        b"SUN",
        EXPPNT.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::RECLAT(SPOINT.as_slice(), &mut RAD, &mut LON, &mut LAT);

    //
    // Make sure the surface points match up.
    //
    testutil::CHCKAD(
        b"Geometric sub solar point",
        SPOINT.as_slice(),
        b"~~/",
        EXPPNT.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid method.", ctx)?;

    spicelib::SUBSOL(
        b"NARPOINT",
        b"399",
        ET,
        b"NONE",
        b"10",
        SPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DUBIOUSMETHOD)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid observer name.", ctx)?;

    spicelib::SUBSOL(
        b"Nearpoint",
        b"earth",
        ET,
        b"NONE",
        b"sn",
        SPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid target name.", ctx)?;

    spicelib::SUBSOL(
        b"Nearpoint",
        b"erth",
        ET,
        b"NONE",
        b"sun",
        SPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Observer is target.", ctx)?;

    spicelib::SUBSOL(
        b"Nearpoint",
        b"earth",
        ET,
        b"NONE",
        b"earth",
        SPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::SPKUEF(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
