//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.00000000000001;
const FILSIZ: i32 = 255;
const LNSIZE: i32 = 80;

//$Procedure      F_BODVAR ( BODVAR family tests )
pub fn F_BODVAR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CVALS = ActualCharArray::new(LNSIZE, 1..=3);
    let mut PCK = [b' '; FILSIZ as usize];
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut XRADII = StackArray::<f64, 3>::new(1..=3);
    let mut N: i32 = 0;

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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_BODVAR", ctx)?;

    testutil::TCASE(b"Setup:  create full text PCK file.", ctx)?;

    fstr::assign(&mut PCK, b"test_0008.tpc");

    //
    // Create the PCK file, load it, and delete it.
    //
    testutil::T_PCK08(&PCK, true, false, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the radii of the earth from the kernel pool using
    // BODVRD.
    //
    testutil::TCASE(b"Look up earth radii using BODVRD", ctx)?;

    spicelib::VPACK(6378.14, 6378.14, 6356.75, XRADII.as_slice_mut());

    spicelib::BODVRD(b"EARTH", b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"radii count", N, b"=", 3, 0, OK, ctx)?;

    testutil::CHCKAD(
        b"Earth radii",
        RADII.as_slice(),
        b"~",
        XRADII.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Look up earth radii using BODVRD.  Name = 399", ctx)?;

    spicelib::VPACK(6378.14, 6378.14, 6356.75, XRADII.as_slice_mut());

    spicelib::BODVRD(b"399", b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"radii count", N, b"=", 3, 0, OK, ctx)?;

    testutil::CHCKAD(
        b"Earth radii",
        RADII.as_slice(),
        b"~",
        XRADII.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Error cases for BODVRD:
    //
    testutil::TCASE(b"Output array too small", ctx)?;

    spicelib::BODVRD(b"EARTH", b"RADII", 2, &mut N, RADII.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(ARRAYTOOSMALL)", OK, ctx)?;

    testutil::TCASE(b"Data type mismatch", ctx)?;

    fstr::assign(CVALS.get_mut(1), b"A");
    fstr::assign(CVALS.get_mut(2), b"B");
    fstr::assign(CVALS.get_mut(3), b"C");

    spicelib::PCPOOL(b"BODY399_SYMBOLIC_RADII", 3, CVALS.as_arg(), ctx)?;

    spicelib::BODVRD(
        b"EARTH",
        b"SYMBOLIC_RADII",
        3,
        &mut N,
        RADII.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TYPEMISMATCH)", OK, ctx)?;

    testutil::TCASE(b"Variable not present", ctx)?;

    spicelib::BODVRD(b"EARTH", b"radii", 2, &mut N, RADII.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"Body name not associated with code", ctx)?;

    spicelib::BODVRD(b"XYZ", b"RADII", 2, &mut N, RADII.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOTRANSLATION)", OK, ctx)?;

    //
    // Try again with BODVAR.
    //
    testutil::TCASE(b"Look up earth radii using BODVAR", ctx)?;
    spicelib::CLEARD(3, RADII.as_slice_mut());

    spicelib::BODVAR(399, b"RADII", &mut N, RADII.as_slice_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"Earth radii",
        RADII.as_slice(),
        b"~",
        XRADII.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Variable not present", ctx)?;

    spicelib::BODVAR(399, b"radii", &mut N, RADII.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
