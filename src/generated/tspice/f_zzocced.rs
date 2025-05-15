//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TOTAL1: i32 = -3;
const ANNLR1: i32 = -2;
const PARTL1: i32 = -1;
const NOOCC: i32 = 0;
const PARTL2: i32 = 1;
const ANNLR2: i32 = 2;
const TOTAL2: i32 = 3;
const LNSIZE: i32 = 255;

//$Procedure      F_ZZOCCED ( Test ellipsoid occultation routine )
pub fn F_ZZOCCED(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TITLE = [b' '; LNSIZE as usize];
    let mut CENTR1 = StackArray::<f64, 3>::new(1..=3);
    let mut CENTR2 = StackArray::<f64, 3>::new(1..=3);
    let mut MTEMP = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut OAX1 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut OAX2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut OCTR1 = StackArray::<f64, 3>::new(1..=3);
    let mut OCTR2 = StackArray::<f64, 3>::new(1..=3);
    let mut OVIEW = StackArray::<f64, 3>::new(1..=3);
    let mut RMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SEMAX1 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SEMAX2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut VIEWPT = StackArray::<f64, 3>::new(1..=3);
    let mut CODE: i32 = 0;
    let mut XCODE: i32 = 0;

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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZOCCED", ctx)?;

    //
    //     We're going to start out with some very basic cases involving
    //     spheres.  These will exercise the bounding code logic.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Disjoint spheres.  This should be handled using maximum bounding cones.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Set up the semi-axis matrices.
    //
    spicelib::IDENT(SEMAX1.as_slice_mut());

    spicelib::CLEARD(9, SEMAX2.as_slice_mut());

    SEMAX2[[1, 1]] = 3.0;
    SEMAX2[[2, 2]] = 3.0;
    SEMAX2[[3, 3]] = 3.0;

    //
    // Assign the centers of the spheres.
    //
    CENTR1[1] = 0.0;
    CENTR1[2] = -2.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = 0.0;
    CENTR2[2] = 4.0;
    CENTR2[3] = 0.0;

    //
    // Assign the viewing point.
    //
    VIEWPT[1] = 10.0;
    VIEWPT[2] = 0.0;
    VIEWPT[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect no occultation to be found.
    //
    XCODE = NOOCC;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Total occultation of first object by the second. This should be handled using minimum and maximum bounding cones.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign the centers of the spheres.
    //
    CENTR1[1] = 0.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = 5.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation of the first sphere
    // by the second to be found.
    //
    XCODE = TOTAL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Total occultation of second object by the first. Switch semi-axis matrices.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation of the second sphere
    // by the first to be found.
    //
    XCODE = TOTAL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Annular transit of first body across the second. This should be handled using minimum and maximum bounding cones.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign the centers of the spheres.
    //
    CENTR1[1] = 4.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = -2.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit of the first sphere
    // across the second to be found.
    //
    XCODE = ANNLR2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Annular transit of second body across the first. Switch arguments.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular occultation of the first sphere
    // by the second to be found.
    //
    XCODE = ANNLR1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Partial occultation of first body by the second.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    CENTR1[1] = -4.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = 2.0;
    CENTR2[2] = 3.0;
    CENTR2[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation of the first sphere
    // by the second to be found.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Partial occultation of second body by the first. Switch centers and semi-axis matrices.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation of the second sphere
    // by the first to be found.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // At this point, we've done all we can with spherical targets.
    // We'll make the smaller ellipsoid prolate with vertical
    // sem-axis length 1.5.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is prolate:  disjoint case");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[3, 3]] = 1.5;

    CENTR1[1] = 0.0;
    CENTR1[2] = -1.1;
    CENTR1[3] = 0.0;

    CENTR2[1] = 0.0;
    CENTR2[2] = 3.1;
    CENTR2[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect no occultation to be found.
    //
    XCODE = NOOCC;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Occultation is total. This is a minimum angular separation case requiring a call to ZZASRYEL.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 4.0;
    SEMAX1[[3, 3]] = 1.5;

    SEMAX2[[1, 1]] = 5.0;

    CENTR1[1] = -5.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = 6.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Set the viewing point far away to avoid parallax problems.
    //
    VIEWPT[1] = 50.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation of the first ellipsoid to be found.
    //
    XCODE = TOTAL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Occultation is total. This is a minimum angular separation case requiring a call to ZZASRYEL.  Switch roles of first and second targets.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation of the second ellipsoid to be found.
    //
    XCODE = TOTAL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid presents larger limb than large ellipsoid; occultation of small ellipsoid is annular. This is a minimum angular separation case requiring a call to ZZASRYEL.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 4.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 1.5;

    SEMAX2[[1, 1]] = 5.0;
    SEMAX2[[2, 2]] = 0.5;
    SEMAX2[[3, 3]] = 0.5;

    CENTR1[1] = -5.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = 6.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Set the viewing point far away to avoid parallax problems.
    //
    VIEWPT[1] = 50.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular occultation of the first ellipsoid to be
    // found.
    //
    XCODE = ANNLR1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid presents larger limb than large ellipsoid; occultation of small ellipsoid is annular. This is a minimum angular separation case requiring a call to ZZASRYEL. Switch roles of first and second targets.");

    testutil::TCASE(&TITLE, ctx)?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular occultation of the second ellipsoid to be
    // found.
    //
    XCODE = ANNLR2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid presents larger limb than large ellipsoid; occultation of large ellipsoid is total. This is a minimum angular separation case requiring a call to ZZASRYEL.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 4.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 1.5;

    SEMAX2[[1, 1]] = 5.0;
    SEMAX2[[2, 2]] = 0.5;
    SEMAX2[[3, 3]] = 0.5;

    CENTR1[1] = 5.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = -6.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Set the viewing point far away to avoid parallax problems.
    //
    VIEWPT[1] = 50.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation of the second ellipsoid to be found.
    //
    XCODE = TOTAL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid presents larger limb than large ellipsoid; occultation of large ellipsoid is total. This is a minimum angular separation case requiring a call to ZZASRYEL. Switch roles of first and second targets.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation of the first ellipsoid to be found.
    //
    XCODE = TOTAL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Small ellipsoid is oblate:  partial occultation of small ellipsoid by large ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 0.75;

    SEMAX2[[1, 1]] = 3.0;
    SEMAX2[[2, 2]] = 3.0;
    SEMAX2[[3, 3]] = 3.0;

    CENTR1[1] = 0.0;
    CENTR1[2] = -0.8;
    CENTR1[3] = 0.0;

    CENTR2[1] = 5.0;
    CENTR2[2] = 3.1;
    CENTR2[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the large ellipsoid in front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  partial occultation of small ellipsoid by large ellipsoid.  Switch argument positions.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the large ellipsoid in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Small ellipsoid is oblate:  partial occultation of large ellipsoid by small ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 0.75;

    SEMAX2[[1, 1]] = 3.0;
    SEMAX2[[2, 2]] = 3.0;
    SEMAX2[[3, 3]] = 3.0;

    CENTR1[1] = 5.0;
    CENTR1[2] = -0.8;
    CENTR1[3] = 0.0;

    CENTR2[1] = 0.0;
    CENTR2[2] = 3.1;
    CENTR2[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the small ellipsoid in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  partial occultation of large ellipsoid by small ellipsoid.  Switch positions of arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[3, 3]] = 0.75;

    CENTR1[1] = 5.0;
    CENTR1[2] = -0.8;
    CENTR1[3] = 0.0;

    CENTR2[1] = 0.0;
    CENTR2[2] = 3.1;
    CENTR2[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the small ellipsoid in front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Small ellipsoid is prolate:  annular transit of small ellipsoid across large ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::CLEARD(9, SEMAX1.as_slice_mut());
    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 1.5;

    spicelib::CLEARD(9, SEMAX2.as_slice_mut());
    SEMAX2[[1, 1]] = 3.0;
    SEMAX2[[2, 2]] = 3.0;
    SEMAX2[[3, 3]] = 4.5;

    CENTR1[1] = 1.5;
    CENTR1[2] = 1.1;
    CENTR1[3] = 0.0;

    CENTR2[1] = -3.0;
    CENTR2[2] = 3.0;
    CENTR2[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the small ellipsoid in front.
    //
    XCODE = ANNLR2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is prolate:  annular transit of small ellipsoid across large ellipsoid. Switch positions of arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[3, 3]] = 1.5;
    SEMAX2[[3, 3]] = 4.5;

    CENTR1[1] = 1.5;
    CENTR1[2] = 1.1;
    CENTR1[3] = 0.0;

    CENTR2[1] = -3.0;
    CENTR2[2] = 3.0;
    CENTR2[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the small ellipsoid in front.
    //
    XCODE = ANNLR1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  total occultation of big ellipsoid by small ellipsoid.  This case requires determination of maximum angular separation.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 3.0;
    SEMAX1[[3, 3]] = 0.75;

    SEMAX2[[1, 1]] = 4.0;
    SEMAX2[[2, 2]] = 0.1;
    SEMAX2[[3, 3]] = 0.2;

    CENTR1[1] = 4.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = -2.0;
    CENTR2[2] = 2.0;
    CENTR2[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation with the small ellipsoid in front.
    //
    XCODE = TOTAL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  total occultation of big ellipsoid by small ellipsoid.  This case requires determination of maximum angular separation. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation with the small ellipsoid in front.
    //
    XCODE = TOTAL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  annular transit by big ellipsoid across small ellipsoid.  This case requires determination of maximum angular separation.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 3.0;
    SEMAX1[[3, 3]] = 0.75;

    SEMAX2[[1, 1]] = 4.0;
    SEMAX2[[2, 2]] = 0.1;
    SEMAX2[[3, 3]] = 0.15;

    CENTR1[1] = -2.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = 4.0;
    CENTR2[2] = 2.0;
    CENTR2[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the big ellipsoid in front.
    //
    XCODE = ANNLR1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  annular transit by big ellipsoid across small ellipsoid.  This case requires determination of maximum angular separation.Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the big ellipsoid in front.
    //
    XCODE = ANNLR2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  partial occultation of small ellipsoid by big ellipsoid.  This case requires determination of maximum angular separation.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 3.0;
    SEMAX1[[3, 3]] = 0.75;

    SEMAX2[[1, 1]] = 4.0;
    SEMAX2[[2, 2]] = 1.0;
    SEMAX2[[3, 3]] = 2.0;

    CENTR1[1] = -2.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = 4.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the big ellipsoid in front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  partial occultation of small ellipsoid by big ellipsoid.  This case requires determination of maximum angular separation. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the big ellipsoid in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  partial occultation of large ellipsoid by small ellipsoid.  This case requires determination of maximum angular separation.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 3.0;
    SEMAX1[[3, 3]] = 0.75;

    SEMAX2[[1, 1]] = 4.0;
    SEMAX2[[2, 2]] = 1.0;
    SEMAX2[[3, 3]] = 2.0;

    CENTR1[1] = 4.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = -2.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the small ellipsoid in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  partial occultation of large ellipsoid by small ellipsoid.   This case requires determination of maximum angular separation. Switch positions of arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the small ellipsoid in front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Potato chip test #1:  annular transit across large ellipsoid by small ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    //
    //    Side view of "potato chip" ellipsoids:
    //
    //
    //       Annular transit:
    //
    //                              *
    //                        *   *
    //                      *   *
    //                        *
    //                      *
    // * viewing point    *
    //                  *
    //                *
    //              *
    //            *
    //          *
    //

    // Observe that the center of the small ellipsoid is actually further
    // from the viewing point than the center of the large ellipsoid.
    //
    //
    // Set up the semi-axis matrices.
    //

    //
    // The first ellipsoid is the small potato chip.
    //
    SEMAX1[[1, 1]] = 0.00000001;
    SEMAX1[[2, 1]] = 0.0;
    SEMAX1[[3, 1]] = 0.00000001;

    SEMAX1[[1, 2]] = 0.0;
    SEMAX1[[2, 2]] = f64::sqrt(2.0);
    SEMAX1[[3, 2]] = 0.0;

    SEMAX1[[1, 3]] = -1.0;
    SEMAX1[[2, 3]] = 0.0;
    SEMAX1[[3, 3]] = 1.0;

    //
    // The large chip is 10 times larger.
    //
    SEMAX2[[1, 1]] = 0.0000001;
    SEMAX2[[2, 1]] = 0.0;
    SEMAX2[[3, 1]] = 0.0000001;

    SEMAX2[[1, 2]] = 0.0;
    SEMAX2[[2, 2]] = (10.0 * f64::sqrt(2.0));
    SEMAX2[[3, 2]] = 0.0;

    SEMAX2[[1, 3]] = -10.0;
    SEMAX2[[2, 3]] = 0.0;
    SEMAX2[[3, 3]] = 10.0;

    //
    // Assign the centers of the targets.
    //
    CENTR1[1] = -3.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 7.0;

    CENTR2[1] = 0.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Assign the viewing point.
    //
    VIEWPT[1] = 50.0;
    VIEWPT[2] = 0.0;
    VIEWPT[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the first ellipsoid in front.
    //
    XCODE = ANNLR2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Potato chip test #2:  annular transit across large ellipsoid by small ellipsoid. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // We expect an annular transit with the second ellipsoid in front.
    //
    XCODE = ANNLR1;

    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Potato chip test #3:  total occultation of small ellipsoid by large ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;
    //
    //
    //       Total occultation:
    //
    //                              *
    //                            *   *
    //                          *   *
    //                        *
    //                      *
    // * viewing point    *
    //                  *
    //                *
    //              *
    //            *
    //          *
    //

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = -11.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 7.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation with the second ellipsoid in front.
    //
    XCODE = TOTAL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Potato chip test #4:  total occultation of small ellipsoid by large ellipsoid.Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation with the first ellipsoid in front.
    //
    XCODE = TOTAL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Potato chip test #5:  partial occultation of small ellipsoid by large ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    //
    //       Partial occultation, small chip is in back:
    //
    //                                  *
    //                              * *
    //                            *
    //                          *
    //                        *
    //                      *
    // * viewing point    *
    //                  *
    //                *
    //              *
    //            *
    //          *
    //

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = -13.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 11.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the second ellipsoid in
    // front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Potato chip test #6:  partial occultation of small ellipsoid by large ellipsoid.Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the first ellipsoid in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(
        &mut TITLE,
        b"Potato chip test #7:  partial occultation of large ellipsoid by small ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    //
    //       Partial occultation, small chip is in front:
    //
    //
    //                           *  *
    //                         *  *
    //                          *
    //                        *
    //                      *
    // * viewing point    *
    //                  *
    //                *
    //              *
    //            *
    //          *
    //
    //
    //    Assign the center of the small target.
    //
    CENTR1[1] = -7.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 9.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the first ellipsoid in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Potato chip test #8:  partial occultation of large ellipsoid by small ellipsoid.Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the second ellipsoid in
    // front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Potato chip test #9:  no occultation of large ellipsoid by small ellipsoid.  Small target is in front.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    //       No occultation, small chip is in front:
    //
    //
    //                           *
    //                         *    *
    //                            *
    //                          *
    //                        *
    //                      *
    // * viewing point    *
    //                  *
    //                *
    //              *
    //            *
    //          *
    //
    //
    //    Assign the center of the small target.
    //
    CENTR1[1] = -7.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 11.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect no occultation.
    //
    XCODE = NOOCC;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Potato chip test #10:  no occultation of large ellipsoid by small ellipsoid.  Small target is in front.Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect no occultation.
    //
    XCODE = NOOCC;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Potato chip test #11:  no occultation of large ellipsoid by small ellipsoid.  Small target is in back.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    //       No occultation, small chip is in back:
    //
    //
    //                                  *
    //                                *
    //                              *
    //                            *
    //                          *
    //                        *
    //                      *
    // * viewing point    *
    //                  *
    //                *
    //              *
    //            *
    //          *
    //

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = -13.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 13.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect no occultation.
    //
    XCODE = NOOCC;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Potato chip test #12:  no occultation of large ellipsoid by small ellipsoid.  Small target is in back.Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect no occultation.
    //
    XCODE = NOOCC;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // The next series of tests exercises the logical branches of
    // ZZOCCED that deal with cases where the maximum angular separation
    // between the limb of the small target and the ray from the viewing
    // point to center of the large target is sought.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(
        &mut TITLE,
        b"Maximum angular separation case #1:  large ellipsoid totally occults small ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Set up the semi-axis matrices.
    //

    //
    // The first ellipsoid is elongated in the y-direction and flattened
    // in the z-direction.  The ellipsoid is situated near the +z
    // portion of the limb of the large ellipsoid. This prevents a
    // simple bounding cone test from proving the ellipsoid is occulted.
    //
    SEMAX1[[1, 1]] = 0.01;
    SEMAX1[[2, 1]] = 0.0;
    SEMAX1[[3, 1]] = 0.0;

    SEMAX1[[1, 2]] = 0.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 2]] = 0.0;

    SEMAX1[[1, 3]] = 0.0;
    SEMAX1[[2, 3]] = 0.0;
    SEMAX1[[3, 3]] = 0.01;

    //
    // The large target is a large sphere. This ensures that the linear
    // transformation performed by ZZOCCED doesn't change the shape of
    // either target.
    //
    SEMAX2[[1, 1]] = 10.0;
    SEMAX2[[2, 1]] = 0.0;
    SEMAX2[[3, 1]] = 0.0;

    SEMAX2[[1, 2]] = 0.0;
    SEMAX2[[2, 2]] = 10.0;
    SEMAX2[[3, 2]] = 0.0;

    SEMAX2[[1, 3]] = 0.0;
    SEMAX2[[2, 3]] = 0.0;
    SEMAX2[[3, 3]] = 10.0;

    //
    // Assign the centers of the targets.
    //
    CENTR1[1] = -12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 9.25;

    CENTR2[1] = 0.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Assign the viewing point.  We set the viewing point far back
    // to reduce parallax.
    //
    VIEWPT[1] = 1000.0;
    VIEWPT[2] = 0.0;
    VIEWPT[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation by the second target.
    //
    XCODE = TOTAL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #2:  large ellipsoid totally occults small ellipsoid.Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation by the first target.
    //
    XCODE = TOTAL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #3:  small ellipsoid is in annular transit across large ellipsoid.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = 12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 9.25;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the first target in front.
    //
    XCODE = ANNLR2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #4:  small ellipsoid is in annular transit across large ellipsoid.Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the second target in front.
    //
    XCODE = ANNLR1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(
        &mut TITLE,
        b"Maximum angular separation case #5:  small ellipsoid partially occults large ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;
    //
    // Assign the semi-axes of the small target.
    //

    SEMAX1[[1, 1]] = 0.001;
    SEMAX1[[2, 1]] = 0.0;
    SEMAX1[[3, 1]] = 0.0;

    SEMAX1[[1, 2]] = 0.0;
    SEMAX1[[2, 2]] = 4.5;
    SEMAX1[[3, 2]] = 0.0;

    SEMAX1[[1, 3]] = 0.0;
    SEMAX1[[2, 3]] = 0.0;
    SEMAX1[[3, 3]] = 0.001;

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = 12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 9.25;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the first target in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #6:  small ellipsoid partially occults large ellipsoid. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the second target in front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #7:  small ellipsoid is partially occulted by large ellipsoid.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = -12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 9.25;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the second target in front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #8:  small ellipsoid is partially occulted by large ellipsoid. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = -12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 9.25;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the first target in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // The next set of cases all have negative maximum angular
    // separation between the limb of the small target and the ray from
    // the viewing point to center of the large target:  the ray
    // passes through the plane region bounded by the limb of the
    // small target.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #9:  large ellipsoid totally occults small ellipsoid. Maximum angular separation is negative.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Set up the semi-axis matrices.
    //
    // The small ellipsoid is elongated in the y-direction and flattened
    // in the z-direction.  The ellipsoid is situated near the x-z plane
    // with the center displaced in the +z direction. The small
    // ellipsoid is penetrated by the -x axis. The small ellipsoid has
    // sufficient extent in the y direction such that, if it were
    // rotated 90 degrees about the x-axis, it would not be totally
    // occulted.  This prevents a simple bounding cone test from proving
    // the ellipsoid is occulted.
    //
    SEMAX1[[1, 1]] = 0.01;
    SEMAX1[[2, 1]] = 0.0;
    SEMAX1[[3, 1]] = 0.0;

    SEMAX1[[1, 2]] = 0.0;
    SEMAX1[[2, 2]] = 9.25;
    SEMAX1[[3, 2]] = 0.0;

    SEMAX1[[1, 3]] = 0.0;
    SEMAX1[[2, 3]] = 0.0;
    SEMAX1[[3, 3]] = 1.0;

    //
    // The large target is a large sphere. This ensures that the linear
    // transformation performed by ZZOCCED doesn't change the shape of
    // either target.
    //
    SEMAX2[[1, 1]] = 10.0;
    SEMAX2[[2, 1]] = 0.0;
    SEMAX2[[3, 1]] = 0.0;

    SEMAX2[[1, 2]] = 0.0;
    SEMAX2[[2, 2]] = 10.0;
    SEMAX2[[3, 2]] = 0.0;

    SEMAX2[[1, 3]] = 0.0;
    SEMAX2[[2, 3]] = 0.0;
    SEMAX2[[3, 3]] = 10.0;

    //
    // Assign the centers of the targets.
    //
    CENTR1[1] = -12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 1.0;

    CENTR2[1] = 0.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Assign the viewing point.  We set the viewing point far back
    // to reduce parallax.
    //
    VIEWPT[1] = 1000.0;
    VIEWPT[2] = 0.0;
    VIEWPT[3] = 0.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation by the second target.
    //
    XCODE = TOTAL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #10:  large ellipsoid totally occults small ellipsoid. Maximum angular separation is negative. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation by the first target.
    //
    XCODE = TOTAL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #11:  small ellipsoid is in annular transit across large ellipsoid; maximum angular separation is negative.");

    testutil::TCASE(&TITLE, ctx)?;

    // Assign the center of the small target.
    //
    CENTR1[1] = 12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 1.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the first target in front.
    //
    XCODE = ANNLR2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #12:  small ellipsoid is in annular transit across large ellipsoid; maximum angular separation is negative. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the second target in front.
    //
    XCODE = ANNLR1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #13:  small ellipsoid partially occults large ellipsoid; maximum angular separation is negative.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign the semi-axes of the small target.
    //

    SEMAX1[[1, 1]] = 0.01;
    SEMAX1[[2, 1]] = 0.0;
    SEMAX1[[3, 1]] = 0.0;

    SEMAX1[[1, 2]] = 0.0;
    SEMAX1[[2, 2]] = 10.25;
    SEMAX1[[3, 2]] = 0.0;

    SEMAX1[[1, 3]] = 0.0;
    SEMAX1[[2, 3]] = 0.0;
    SEMAX1[[3, 3]] = 1.0;

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = 12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 1.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the first target in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #14:  small ellipsoid partially occults large ellipsoid; maximum angular separation is negative. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the second target in front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #15:  small ellipsoid is partially occulted by large ellipsoid; maximum angular separation is negative.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign the semi-axes of the small target.
    //

    SEMAX1[[1, 1]] = 0.01;
    SEMAX1[[2, 1]] = 0.0;
    SEMAX1[[3, 1]] = 0.0;

    SEMAX1[[1, 2]] = 0.0;
    SEMAX1[[2, 2]] = 10.25;
    SEMAX1[[3, 2]] = 0.0;

    SEMAX1[[1, 3]] = 0.0;
    SEMAX1[[2, 3]] = 0.0;
    SEMAX1[[3, 3]] = 1.0;

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = -12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 1.0;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the second target in front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #16:  small ellipsoid is partially occulted by large ellipsoid; maximum angular separation is negative. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the first target in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    // *******
    // *******
    // *******
    // *******  At this point, we've finished with the simple cases that
    // *******  exercise all of the classification logic.  These simple
    // *******  cases had the semi-axis matrices aligned with the standard
    // *******  basis; they also employed other simplifications such as
    // *******  placing the viewing location on the x-axis.
    // *******
    // *******  Now we'll repeat the tests with inputs transformed as
    // *******  follows:
    // *******
    // *******     - The semi-axis matrices of the ellipsoids will have
    // *******       their columns permuted so that these matrices won't
    // *******       have diagonal form.
    // *******
    // *******       * The semi-axes of the first ellipsoid will be permuted
    // *******         by a (312) permutation.
    // *******
    // *******       * The semi-axes of the second ellipsoid will be permuted
    // *******         by a (231) permutation.
    // *******
    // *******     - All vectors, including the columns of the permuted
    // *******       semi-axis matrices, will be rotated by a non-trivial
    // *******       3-1-3 rotation.
    // *******
    // *******     - The viewing location and the ellipsoid centers will
    // *******       be translated by an offset vector.
    // *******
    // *******  All of these tranformations preserve the viewing geometry,
    // *******  so each test case should produce the same occultation
    // *******  classification as the corresponding case above using "simple"
    // *******  inputs.
    // *******

    //
    //     We're going to start out with some very basic cases involving
    //     spheres.  These will exercise the bounding code logic.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Disjoint spheres.  This should be handled using maximum bounding cones.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Set up the semi-axis matrices.
    //
    spicelib::IDENT(SEMAX1.as_slice_mut());

    spicelib::CLEARD(9, SEMAX2.as_slice_mut());

    SEMAX2[[1, 1]] = 3.0;
    SEMAX2[[2, 2]] = 3.0;
    SEMAX2[[3, 3]] = 3.0;

    //
    // Assign the centers of the spheres.
    //
    CENTR1[1] = 0.0;
    CENTR1[2] = -2.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = 0.0;
    CENTR2[2] = 4.0;
    CENTR2[3] = 0.0;

    //
    // Assign the viewing point.
    //
    VIEWPT[1] = 10.0;
    VIEWPT[2] = 0.0;
    VIEWPT[3] = 0.0;

    //
    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect no occultation to be found.
    //
    XCODE = NOOCC;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Total occultation of first object by the second. This should be handled using minimum and maximum bounding cones.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign the centers of the spheres.
    //
    CENTR1[1] = 0.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = 5.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation of the first sphere
    // by the second to be found.
    //
    XCODE = TOTAL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Total occultation of second object by the first. Switch targets.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation of the second sphere
    // by the first to be found.
    //
    XCODE = TOTAL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Annular transit of first body across the second. This should be handled using minimum and maximum bounding cones.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign the centers of the spheres.
    //
    CENTR1[1] = 4.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = -2.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit of the first sphere
    // across the second to be found.
    //
    XCODE = ANNLR2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Annular transit of second body across the first. Switch arguments.",
    );

    testutil::TCASE(&TITLE, ctx)?;
    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular occultation of the first sphere
    // by the second to be found.
    //
    XCODE = ANNLR1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Partial occultation of first body by the second.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    CENTR1[1] = -4.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = 2.0;
    CENTR2[2] = 3.0;
    CENTR2[3] = 0.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation of the first sphere
    // by the second to be found.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Partial occultation of second body by the first. Switch centers and semi-axis matrices.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation of the second sphere
    // by the first to be found.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // At this point, we've done all we can with spherical targets.
    // We'll make the smaller ellipsoid prolate with vertical
    // sem-axis length 1.5.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is prolate:  disjoint case");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[3, 3]] = 1.5;

    CENTR1[1] = 0.0;
    CENTR1[2] = -1.1;
    CENTR1[3] = 0.0;

    CENTR2[1] = 0.0;
    CENTR2[2] = 3.1;
    CENTR2[3] = 0.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect no occultation to be found.
    //
    XCODE = NOOCC;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Occultation is total. This is a minimum angular separation case requiring a call to ZZASRYEL.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 4.0;
    SEMAX1[[3, 3]] = 1.5;

    SEMAX2[[1, 1]] = 5.0;

    CENTR1[1] = -5.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = 6.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Set the viewing point far away to avoid parallax problems.
    //
    VIEWPT[1] = 50.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation of the first ellipsoid to be found.
    //
    XCODE = TOTAL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Occultation is total. This is a minimum angular separation case requiring a call to ZZASRYEL.  Switch roles of first and second targets.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation of the second ellipsoid to be found.
    //
    XCODE = TOTAL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid presents larger limb than large ellipsoid; occultation of small ellipsoid is annular. This is a minimum angular separation case requiring a call to ZZASRYEL.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 4.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 1.5;

    SEMAX2[[1, 1]] = 5.0;
    SEMAX2[[2, 2]] = 0.5;
    SEMAX2[[3, 3]] = 0.5;

    CENTR1[1] = -5.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = 6.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Set the viewing point far away to avoid parallax problems.
    //
    VIEWPT[1] = 50.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular occultation of the first ellipsoid to be
    // found.
    //
    XCODE = ANNLR1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid presents larger limb than large ellipsoid; occultation of small ellipsoid is annular. This is a minimum angular separation case requiring a call to ZZASRYEL. Switch roles of first and second targets.");

    testutil::TCASE(&TITLE, ctx)?;
    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular occultation of the second ellipsoid to be
    // found.
    //
    XCODE = ANNLR2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid presents larger limb than large ellipsoid; occultation of large ellipsoid is total. This is a minimum angular separation case requiring a call to ZZASRYEL.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 4.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 1.5;

    SEMAX2[[1, 1]] = 5.0;
    SEMAX2[[2, 2]] = 0.5;
    SEMAX2[[3, 3]] = 0.5;

    CENTR1[1] = 5.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = -6.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Set the viewing point far away to avoid parallax problems.
    //
    VIEWPT[1] = 50.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation of the second ellipsoid to be found.
    //
    XCODE = TOTAL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid presents larger limb than large ellipsoid; occultation of large ellipsoid is total. This is a minimum angular separation case requiring a call to ZZASRYEL. Switch roles of first and second targets.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation of the first ellipsoid to be found.
    //
    XCODE = TOTAL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Small ellipsoid is oblate:  partial occultation of small ellipsoid by large ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 0.75;

    SEMAX2[[1, 1]] = 3.0;
    SEMAX2[[2, 2]] = 3.0;
    SEMAX2[[3, 3]] = 3.0;

    CENTR1[1] = 0.0;
    CENTR1[2] = -0.8;
    CENTR1[3] = 0.0;

    CENTR2[1] = 5.0;
    CENTR2[2] = 3.1;
    CENTR2[3] = 0.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the large ellipsoid in front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  partial occultation of small ellipsoid by large ellipsoid.  Switch argument positions.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the large ellipsoid in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Small ellipsoid is oblate:  partial occultation of large ellipsoid by small ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 0.75;

    SEMAX2[[1, 1]] = 3.0;
    SEMAX2[[2, 2]] = 3.0;
    SEMAX2[[3, 3]] = 3.0;

    CENTR1[1] = 5.0;
    CENTR1[2] = -0.8;
    CENTR1[3] = 0.0;

    CENTR2[1] = 0.0;
    CENTR2[2] = 3.1;
    CENTR2[3] = 0.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the small ellipsoid in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  partial occultation of large ellipsoid by small ellipsoid.  Switch positions of arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[3, 3]] = 0.75;

    CENTR1[1] = 5.0;
    CENTR1[2] = -0.8;
    CENTR1[3] = 0.0;

    CENTR2[1] = 0.0;
    CENTR2[2] = 3.1;
    CENTR2[3] = 0.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the small ellipsoid in front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Small ellipsoid is prolate:  annular transit of small ellipsoid across large ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::CLEARD(9, SEMAX1.as_slice_mut());
    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 1.5;

    spicelib::CLEARD(9, SEMAX2.as_slice_mut());
    SEMAX2[[1, 1]] = 3.0;
    SEMAX2[[2, 2]] = 3.0;
    SEMAX2[[3, 3]] = 4.5;

    CENTR1[1] = 1.5;
    CENTR1[2] = 1.1;
    CENTR1[3] = 0.0;

    CENTR2[1] = -3.0;
    CENTR2[2] = 3.0;
    CENTR2[3] = 0.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the small ellipsoid in front.
    //
    XCODE = ANNLR2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is prolate:  annular transit of small ellipsoid across large ellipsoid. Switch positions of arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[3, 3]] = 1.5;
    SEMAX2[[3, 3]] = 4.5;

    CENTR1[1] = 1.5;
    CENTR1[2] = 1.1;
    CENTR1[3] = 0.0;

    CENTR2[1] = -3.0;
    CENTR2[2] = 3.0;
    CENTR2[3] = 0.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the small ellipsoid in front.
    //
    XCODE = ANNLR1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  total occultation of big ellipsoid by small ellipsoid.  This case requires determination of maximum angular separation.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 3.0;
    SEMAX1[[3, 3]] = 0.75;

    SEMAX2[[1, 1]] = 4.0;
    SEMAX2[[2, 2]] = 0.1;
    SEMAX2[[3, 3]] = 0.2;

    CENTR1[1] = 4.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = -2.0;
    CENTR2[2] = 2.0;
    CENTR2[3] = 0.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation with the small ellipsoid in front.
    //
    XCODE = TOTAL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  total occultation of big ellipsoid by small ellipsoid.  This case requires determination of maximum angular separation. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation with the small ellipsoid in front.
    //
    XCODE = TOTAL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  annular transit by big ellipsoid across small ellipsoid.  This case requires determination of maximum angular separation.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 3.0;
    SEMAX1[[3, 3]] = 0.75;

    SEMAX2[[1, 1]] = 4.0;
    SEMAX2[[2, 2]] = 0.1;
    SEMAX2[[3, 3]] = 0.15;

    CENTR1[1] = -2.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = 4.0;
    CENTR2[2] = 2.0;
    CENTR2[3] = 0.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the big ellipsoid in front.
    //
    XCODE = ANNLR1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  annular transit by big ellipsoid across small ellipsoid.  This case requires determination of maximum angular separation.Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the big ellipsoid in front.
    //
    XCODE = ANNLR2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  partial occultation of small ellipsoid by big ellipsoid.  This case requires determination of maximum angular separation.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 3.0;
    SEMAX1[[3, 3]] = 0.75;

    SEMAX2[[1, 1]] = 4.0;
    SEMAX2[[2, 2]] = 1.0;
    SEMAX2[[3, 3]] = 2.0;

    CENTR1[1] = -2.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = 4.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the big ellipsoid in front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  partial occultation of small ellipsoid by big ellipsoid.  This case requires determination of maximum angular separation. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the big ellipsoid in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  partial occultation of large ellipsoid by small ellipsoid.  This case requires determination of maximum angular separation.");

    testutil::TCASE(&TITLE, ctx)?;

    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 3.0;
    SEMAX1[[3, 3]] = 0.75;

    SEMAX2[[1, 1]] = 4.0;
    SEMAX2[[2, 2]] = 1.0;
    SEMAX2[[3, 3]] = 2.0;

    CENTR1[1] = 4.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 0.0;

    CENTR2[1] = -2.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the small ellipsoid in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Small ellipsoid is oblate:  partial occultation of large ellipsoid by small ellipsoid.   This case requires determination of maximum angular separation. Switch positions of arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the small ellipsoid in front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Potato chip test #1:  annular transit across large ellipsoid by small ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    //
    //    Side view of "potato chip" ellipsoids:
    //
    //
    //       Annular transit:
    //
    //                              *
    //                        *   *
    //                      *   *
    //                        *
    //                      *
    // * viewing point    *
    //                  *
    //                *
    //              *
    //            *
    //          *
    //

    // Observe that the center of the small ellipsoid is actually further
    // from the viewing point than the center of the large ellipsoid.
    //
    //
    // Set up the semi-axis matrices.
    //

    //
    // The first ellipsoid is the small potato chip.
    //
    SEMAX1[[1, 1]] = 0.00000001;
    SEMAX1[[2, 1]] = 0.0;
    SEMAX1[[3, 1]] = 0.00000001;

    SEMAX1[[1, 2]] = 0.0;
    SEMAX1[[2, 2]] = f64::sqrt(2.0);
    SEMAX1[[3, 2]] = 0.0;

    SEMAX1[[1, 3]] = -1.0;
    SEMAX1[[2, 3]] = 0.0;
    SEMAX1[[3, 3]] = 1.0;

    //
    // The large chip is 10 times larger.
    //
    SEMAX2[[1, 1]] = 0.0000001;
    SEMAX2[[2, 1]] = 0.0;
    SEMAX2[[3, 1]] = 0.0000001;

    SEMAX2[[1, 2]] = 0.0;
    SEMAX2[[2, 2]] = (10.0 * f64::sqrt(2.0));
    SEMAX2[[3, 2]] = 0.0;

    SEMAX2[[1, 3]] = -10.0;
    SEMAX2[[2, 3]] = 0.0;
    SEMAX2[[3, 3]] = 10.0;

    //
    // Assign the centers of the targets.
    //
    CENTR1[1] = -3.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 7.0;

    CENTR2[1] = 0.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Assign the viewing point.
    //
    VIEWPT[1] = 50.0;
    VIEWPT[2] = 0.0;
    VIEWPT[3] = 0.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the first ellipsoid in front.
    //
    XCODE = ANNLR2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Potato chip test #2:  annular transit across large ellipsoid by small ellipsoid. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // We expect an annular transit with the second ellipsoid in front.
    //
    XCODE = ANNLR1;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Potato chip test #3:  total occultation of small ellipsoid by large ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;
    //
    //
    //       Total occultation:
    //
    //                              *
    //                            *   *
    //                          *   *
    //                        *
    //                      *
    // * viewing point    *
    //                  *
    //                *
    //              *
    //            *
    //          *
    //

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = -11.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 7.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation with the second ellipsoid in front.
    //
    XCODE = TOTAL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Potato chip test #4:  total occultation of small ellipsoid by large ellipsoid.Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation with the first ellipsoid in front.
    //
    XCODE = TOTAL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Potato chip test #5:  partial occultation of small ellipsoid by large ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    //
    //       Partial occultation, small chip is in back:
    //
    //                                  *
    //                              * *
    //                            *
    //                          *
    //                        *
    //                      *
    // * viewing point    *
    //                  *
    //                *
    //              *
    //            *
    //          *
    //

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = -13.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 11.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the second ellipsoid in
    // front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Potato chip test #6:  partial occultation of small ellipsoid by large ellipsoid.Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the first ellipsoid in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(
        &mut TITLE,
        b"Potato chip test #7:  partial occultation of large ellipsoid by small ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    //
    //       Partial occultation, small chip is in front:
    //
    //
    //                           *  *
    //                         *  *
    //                          *
    //                        *
    //                      *
    // * viewing point    *
    //                  *
    //                *
    //              *
    //            *
    //          *
    //
    //
    //    Assign the center of the small target.
    //
    CENTR1[1] = -7.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 9.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the first ellipsoid in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Potato chip test #8:  partial occultation of large ellipsoid by small ellipsoid.Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the second ellipsoid in
    // front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Potato chip test #9:  no occultation of large ellipsoid by small ellipsoid.  Small target is in front.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    //       No occultation, small chip is in front:
    //
    //
    //                           *
    //                         *    *
    //                            *
    //                          *
    //                        *
    //                      *
    // * viewing point    *
    //                  *
    //                *
    //              *
    //            *
    //          *
    //
    //
    //    Assign the center of the small target.
    //
    CENTR1[1] = -7.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 11.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect no occultation.
    //
    XCODE = NOOCC;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Potato chip test #10:  no occultation of large ellipsoid by small ellipsoid.  Small target is in front.Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect no occultation.
    //
    XCODE = NOOCC;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Potato chip test #11:  no occultation of large ellipsoid by small ellipsoid.  Small target is in back.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    //       No occultation, small chip is in back:
    //
    //
    //                                  *
    //                                *
    //                              *
    //                            *
    //                          *
    //                        *
    //                      *
    // * viewing point    *
    //                  *
    //                *
    //              *
    //            *
    //          *
    //

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = -13.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 13.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect no occultation.
    //
    XCODE = NOOCC;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Potato chip test #12:  no occultation of large ellipsoid by small ellipsoid.  Small target is in back.Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect no occultation.
    //
    XCODE = NOOCC;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // The next series of tests exercises the logical branches of
    // ZZOCCED that deal with cases where the maximum angular separation
    // between the limb of the small target and the ray from the viewing
    // point to center of the large target is sought.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(
        &mut TITLE,
        b"Maximum angular separation case #1:  large ellipsoid totally occults small ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Set up the semi-axis matrices.
    //

    //
    // The first ellipsoid is elongated in the y-direction and flattened
    // in the z-direction.  The ellipsoid is situated near the +z
    // portion of the limb of the large ellipsoid. This prevents a
    // simple bounding cone test from proving the ellipsoid is occulted.
    //
    SEMAX1[[1, 1]] = 0.01;
    SEMAX1[[2, 1]] = 0.0;
    SEMAX1[[3, 1]] = 0.0;

    SEMAX1[[1, 2]] = 0.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 2]] = 0.0;

    SEMAX1[[1, 3]] = 0.0;
    SEMAX1[[2, 3]] = 0.0;
    SEMAX1[[3, 3]] = 0.01;

    //
    // The large target is a large sphere. This ensures that the linear
    // transformation performed by ZZOCCED doesn't change the shape of
    // either target.
    //
    SEMAX2[[1, 1]] = 10.0;
    SEMAX2[[2, 1]] = 0.0;
    SEMAX2[[3, 1]] = 0.0;

    SEMAX2[[1, 2]] = 0.0;
    SEMAX2[[2, 2]] = 10.0;
    SEMAX2[[3, 2]] = 0.0;

    SEMAX2[[1, 3]] = 0.0;
    SEMAX2[[2, 3]] = 0.0;
    SEMAX2[[3, 3]] = 10.0;

    //
    // Assign the centers of the targets.
    //
    CENTR1[1] = -12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 9.25;

    CENTR2[1] = 0.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Assign the viewing point.  We set the viewing point far back
    // to reduce parallax.
    //
    VIEWPT[1] = 1000.0;
    VIEWPT[2] = 0.0;
    VIEWPT[3] = 0.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation by the second target.
    //
    XCODE = TOTAL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #2:  large ellipsoid totally occults small ellipsoid.Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation by the first target.
    //
    XCODE = TOTAL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #3:  small ellipsoid is in annular transit across large ellipsoid.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = 12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 9.25;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the first target in front.
    //
    XCODE = ANNLR2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #4:  small ellipsoid is in annular transit across large ellipsoid.Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the second target in front.
    //
    XCODE = ANNLR1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(
        &mut TITLE,
        b"Maximum angular separation case #5:  small ellipsoid partially occults large ellipsoid.",
    );

    testutil::TCASE(&TITLE, ctx)?;
    //
    // Assign the semi-axes of the small target.
    //

    SEMAX1[[1, 1]] = 0.001;
    SEMAX1[[2, 1]] = 0.0;
    SEMAX1[[3, 1]] = 0.0;

    SEMAX1[[1, 2]] = 0.0;
    SEMAX1[[2, 2]] = 4.5;
    SEMAX1[[3, 2]] = 0.0;

    SEMAX1[[1, 3]] = 0.0;
    SEMAX1[[2, 3]] = 0.0;
    SEMAX1[[3, 3]] = 0.001;

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = 12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 9.25;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the first target in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #6:  small ellipsoid partially occults large ellipsoid. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the second target in front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #7:  small ellipsoid is partially occulted by large ellipsoid.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = -12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 9.25;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the second target in front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #8:  small ellipsoid is partially occulted by large ellipsoid. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = -12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 9.25;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the first target in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // The next set of cases all have negative maximum angular
    // separation between the limb of the small target and the ray from
    // the viewing point to center of the large target:  the ray
    // passes through the plane region bounded by the limb of the
    // small target.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #9:  large ellipsoid totally occults small ellipsoid. Maximum angular separation is negative.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Set up the semi-axis matrices.
    //
    // The small ellipsoid is elongated in the y-direction and flattened
    // in the z-direction.  The ellipsoid is situated near the x-z plane
    // with the center displaced in the +z direction. The small
    // ellipsoid is penetrated by the -x axis. The small ellipsoid has
    // sufficient extent in the y direction such that, if it were
    // rotated 90 degrees about the x-axis, it would not be totally
    // occulted.  This prevents a simple bounding cone test from proving
    // the ellipsoid is occulted.
    //
    SEMAX1[[1, 1]] = 0.01;
    SEMAX1[[2, 1]] = 0.0;
    SEMAX1[[3, 1]] = 0.0;

    SEMAX1[[1, 2]] = 0.0;
    SEMAX1[[2, 2]] = 9.25;
    SEMAX1[[3, 2]] = 0.0;

    SEMAX1[[1, 3]] = 0.0;
    SEMAX1[[2, 3]] = 0.0;
    SEMAX1[[3, 3]] = 1.0;

    //
    // The large target is a large sphere. This ensures that the linear
    // transformation performed by ZZOCCED doesn't change the shape of
    // either target.
    //
    SEMAX2[[1, 1]] = 10.0;
    SEMAX2[[2, 1]] = 0.0;
    SEMAX2[[3, 1]] = 0.0;

    SEMAX2[[1, 2]] = 0.0;
    SEMAX2[[2, 2]] = 10.0;
    SEMAX2[[3, 2]] = 0.0;

    SEMAX2[[1, 3]] = 0.0;
    SEMAX2[[2, 3]] = 0.0;
    SEMAX2[[3, 3]] = 10.0;

    //
    // Assign the centers of the targets.
    //
    CENTR1[1] = -12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 1.0;

    CENTR2[1] = 0.0;
    CENTR2[2] = 0.0;
    CENTR2[3] = 0.0;

    //
    // Assign the viewing point.  We set the viewing point far back
    // to reduce parallax.
    //
    VIEWPT[1] = 1000.0;
    VIEWPT[2] = 0.0;
    VIEWPT[3] = 0.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation by the second target.
    //
    XCODE = TOTAL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #10:  large ellipsoid totally occults small ellipsoid. Maximum angular separation is negative. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a total occultation by the first target.
    //
    XCODE = TOTAL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #11:  small ellipsoid is in annular transit across large ellipsoid; maximum angular separation is negative.");

    testutil::TCASE(&TITLE, ctx)?;

    // Assign the center of the small target.
    //
    CENTR1[1] = 12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 1.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the first target in front.
    //
    XCODE = ANNLR2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #12:  small ellipsoid is in annular transit across large ellipsoid; maximum angular separation is negative. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an annular transit with the second target in front.
    //
    XCODE = ANNLR1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #13:  small ellipsoid partially occults large ellipsoid; maximum angular separation is negative.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign the semi-axes of the small target.
    //

    SEMAX1[[1, 1]] = 0.01;
    SEMAX1[[2, 1]] = 0.0;
    SEMAX1[[3, 1]] = 0.0;

    SEMAX1[[1, 2]] = 0.0;
    SEMAX1[[2, 2]] = 10.25;
    SEMAX1[[3, 2]] = 0.0;

    SEMAX1[[1, 3]] = 0.0;
    SEMAX1[[2, 3]] = 0.0;
    SEMAX1[[3, 3]] = 1.0;

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = 12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 1.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the first target in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #14:  small ellipsoid partially occults large ellipsoid; maximum angular separation is negative. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the second target in front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #15:  small ellipsoid is partially occulted by large ellipsoid; maximum angular separation is negative.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign the semi-axes of the small target.
    //

    SEMAX1[[1, 1]] = 0.01;
    SEMAX1[[2, 1]] = 0.0;
    SEMAX1[[3, 1]] = 0.0;

    SEMAX1[[1, 2]] = 0.0;
    SEMAX1[[2, 2]] = 10.25;
    SEMAX1[[3, 2]] = 0.0;

    SEMAX1[[1, 3]] = 0.0;
    SEMAX1[[2, 3]] = 0.0;
    SEMAX1[[3, 3]] = 1.0;

    //
    // Assign the center of the small target.
    //
    CENTR1[1] = -12.0;
    CENTR1[2] = 0.0;
    CENTR1[3] = 1.0;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the second target in front.
    //
    XCODE = PARTL1;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    fstr::assign(&mut TITLE, b"Maximum angular separation case #16:  small ellipsoid is partially occulted by large ellipsoid; maximum angular separation is negative. Switch arguments.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Apply rotations and translations to the inputs.
    //
    XINPUT(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        OVIEW.as_slice_mut(),
        OCTR1.as_slice_mut(),
        OAX1.as_slice_mut(),
        OCTR2.as_slice_mut(),
        OAX2.as_slice_mut(),
        ctx,
    )?;
    //
    // Classify the occultation.
    //
    CODE = spicelib::ZZOCCED(
        OVIEW.as_slice(),
        OCTR2.as_slice(),
        OAX2.as_slice(),
        OCTR1.as_slice(),
        OAX1.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect a partial occultation with the first target in front.
    //
    XCODE = PARTL2;

    testutil::CHCKSI(b"ZZOCCED", CODE, b"=", XCODE, 0, OK, ctx)?;

    // *******
    // *******
    // *******
    // *******
    // *******
    // *******  Error handling cases:
    // *******
    // *******
    // *******
    // *******
    // *******

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Viewing point is inside outer bounding sphere of first ellipsoid, but outside the ellipsoid itself. Make sure NO error is signaled.");
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::VPACK(0.0, -2.0, 3.0, VIEWPT.as_slice_mut());
    spicelib::VPACK(0.0, -2.0, 0.0, CENTR1.as_slice_mut());
    spicelib::VPACK(0.0, 2.0, 0.0, CENTR2.as_slice_mut());

    spicelib::CLEARD(9, SEMAX1.as_slice_mut());
    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 4.0;

    spicelib::CLEARD(9, SEMAX2.as_slice_mut());
    SEMAX2[[1, 1]] = 1.0;
    SEMAX2[[2, 2]] = 1.0;
    SEMAX2[[3, 3]] = 1.0;

    spicelib::ROTATE(spicelib::HALFPI(ctx), 2, RMAT.as_slice_mut(), ctx);

    spicelib::MXM(RMAT.as_slice(), SEMAX1.as_slice(), MTEMP.as_slice_mut());
    spicelib::MOVED(MTEMP.as_slice(), 9, SEMAX1.as_slice_mut());

    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Viewing point is inside first ellipsoid.");
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::VPACK(1.0, 2.0, 3.0, VIEWPT.as_slice_mut());
    spicelib::VPACK(1.0, 2.1, 3.0, CENTR1.as_slice_mut());
    spicelib::VPACK(1.0, -2.1, 3.0, CENTR2.as_slice_mut());

    spicelib::CLEARD(9, SEMAX1.as_slice_mut());
    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 1.0;

    spicelib::CLEARD(9, SEMAX2.as_slice_mut());
    SEMAX2[[1, 1]] = 1.0;
    SEMAX2[[2, 2]] = 1.0;
    SEMAX2[[3, 3]] = 1.0;

    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTDISJOINT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Viewing point is inside second ellipsoid.");
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::VPACK(1.0, -2.0, 3.0, VIEWPT.as_slice_mut());
    spicelib::VPACK(1.0, 2.1, 3.0, CENTR1.as_slice_mut());
    spicelib::VPACK(1.0, -2.1, 3.0, CENTR2.as_slice_mut());

    spicelib::CLEARD(9, SEMAX1.as_slice_mut());
    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 1.0;

    spicelib::CLEARD(9, SEMAX2.as_slice_mut());
    SEMAX2[[1, 1]] = 1.0;
    SEMAX2[[2, 2]] = 1.0;
    SEMAX2[[3, 3]] = 1.0;

    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTDISJOINT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Target bodies intersect");
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::VPACK(10.0, -2.0, 3.0, VIEWPT.as_slice_mut());
    spicelib::VPACK(1.0, 2.1, 3.0, CENTR1.as_slice_mut());
    spicelib::VPACK(1.0, 1.1, 3.0, CENTR2.as_slice_mut());

    spicelib::CLEARD(9, SEMAX1.as_slice_mut());
    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 1.0;

    spicelib::CLEARD(9, SEMAX2.as_slice_mut());
    SEMAX2[[1, 1]] = 1.0;
    SEMAX2[[2, 2]] = 1.0;
    SEMAX2[[3, 3]] = 1.0;

    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTDISJOINT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Non-positive radii");
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::VPACK(10.0, -2.0, 3.0, VIEWPT.as_slice_mut());
    spicelib::VPACK(1.0, 2.1, 3.0, CENTR1.as_slice_mut());
    spicelib::VPACK(1.0, 1.1, 3.0, CENTR2.as_slice_mut());

    spicelib::CLEARD(9, SEMAX1.as_slice_mut());
    SEMAX1[[1, 1]] = 0.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 1.0;

    spicelib::CLEARD(9, SEMAX2.as_slice_mut());
    SEMAX2[[1, 1]] = 1.0;
    SEMAX2[[2, 2]] = 1.0;
    SEMAX2[[3, 3]] = 1.0;

    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 0.0;

    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 0.0;

    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    SEMAX1[[3, 3]] = 1.0;

    SEMAX2[[1, 1]] = 0.0;

    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    SEMAX2[[1, 1]] = 1.0;
    SEMAX2[[2, 2]] = 0.0;

    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    SEMAX2[[2, 2]] = 1.0;
    SEMAX2[[3, 3]] = 0.0;

    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    SEMAX2[[3, 3]] = 1.0;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"First matrix is not a rotation");
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::VPACK(10.0, -2.0, 3.0, VIEWPT.as_slice_mut());
    spicelib::VPACK(1.0, 2.1, 3.0, CENTR1.as_slice_mut());
    spicelib::VPACK(1.0, -1.1, 3.0, CENTR2.as_slice_mut());

    spicelib::CLEARD(9, SEMAX1.as_slice_mut());
    SEMAX1[[1, 1]] = -1.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 1.0;

    spicelib::CLEARD(9, SEMAX2.as_slice_mut());
    SEMAX2[[1, 1]] = 1.0;
    SEMAX2[[2, 2]] = 1.0;
    SEMAX2[[3, 3]] = 1.0;

    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTAROTATION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Second matrix is not a rotation");
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::VPACK(10.0, -2.0, 3.0, VIEWPT.as_slice_mut());
    spicelib::VPACK(1.0, 2.1, 3.0, CENTR1.as_slice_mut());
    spicelib::VPACK(1.0, 1.1, 3.0, CENTR2.as_slice_mut());

    spicelib::CLEARD(9, SEMAX1.as_slice_mut());
    SEMAX1[[1, 1]] = 1.0;
    SEMAX1[[2, 2]] = 1.0;
    SEMAX1[[3, 3]] = 1.0;

    spicelib::CLEARD(9, SEMAX2.as_slice_mut());
    SEMAX2[[1, 1]] = 1.0;
    SEMAX2[[2, 2]] = 1.0;
    SEMAX2[[3, 3]] = -1.0;

    CODE = spicelib::ZZOCCED(
        VIEWPT.as_slice(),
        CENTR1.as_slice(),
        SEMAX1.as_slice(),
        CENTR2.as_slice(),
        SEMAX2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTAROTATION)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
