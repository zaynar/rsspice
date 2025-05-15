//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure F_ZZGETELM ( Family of tests for ZZGETELM)
pub fn F_ZZGETELM(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut LINES = ActualCharArray::new(80, 1..=2);
    let mut ERROR = [b' '; 128 as usize];
    let mut FRSTYR: i32 = 0;
    let mut EPOCH: f64 = 0.0;
    let mut ELEMS = StackArray::<f64, 10>::new(1..=10);
    let mut ERRSIG: bool = false;

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGETELM", ctx)?;

    //
    // Create and load  leapseconds kernel.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Assign test values.
    //
    FRSTYR = 1969;

    //
    // Check for a positive process on a correct TLE set.
    //

    testutil::TCASE(b"Confirm processing of a good TLE", ctx)?;

    fstr::assign(
        LINES.get_mut(1),
        b"1 25544U 98067A   98324.89267077  .00616830  11572-4  29139-3 0    84",
    );
    fstr::assign(
        LINES.get_mut(2),
        b"2 25544  51.5947 165.1012 0122649  89.2072 272.3014 16.05443269   101",
    );

    spicelib::ZZGETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        &mut ERRSIG,
        &mut ERROR,
        ctx,
    )?;
    testutil::CHCKSL(b"OK 1", ERRSIG, true, OK, ctx)?;

    //
    // Call the user level routine. Expect no errors.
    //
    spicelib::GETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now test the error checks. Too long. Longest allowed line: 69.
    //
    testutil::TCASE(b"TLE too long", ctx)?;

    fstr::assign(
        LINES.get_mut(1),
        b"1 25544U 98067A   98324.89267077  .00616830  11572-4  29139-3 0    84",
    );
    fstr::assign(
        LINES.get_mut(2),
        b"2 25544  51.5947 165.1012 0122649  89.2072 272.3014 16.05443269       101",
    );

    spicelib::ZZGETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        &mut ERRSIG,
        &mut ERROR,
        ctx,
    )?;
    testutil::CHCKSL(b"OK 3", ERRSIG, false, OK, ctx)?;

    spicelib::GETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADTLE)", OK, ctx)?;

    //
    // Now test the error checks. Too short. Shortest allowed line: 68.
    //
    testutil::TCASE(b"TLE too short", ctx)?;

    fstr::assign(
        LINES.get_mut(1),
        b"1 25544U 98067A   98324.89267077  .00616830  11572-4  29139-3 0 84",
    );
    fstr::assign(
        LINES.get_mut(2),
        b"2 25544  51.5947 165.1012 0122649  89.2072 272.3014 16.05443269 101",
    );

    spicelib::ZZGETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        &mut ERRSIG,
        &mut ERROR,
        ctx,
    )?;
    testutil::CHCKSL(b"OK 3", ERRSIG, false, OK, ctx)?;

    spicelib::GETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADTLE)", OK, ctx)?;

    //
    // Check bounds on numerical values.
    //

    //
    // Inclination [0,180]
    //
    testutil::TCASE(b"Inclination low", ctx)?;

    fstr::assign(
        LINES.get_mut(1),
        b"1 25544U 98067A   98324.89267077  .00616830  11572-4  29139-3 0    84",
    );
    fstr::assign(
        LINES.get_mut(2),
        b"2 25544 -51.5947 165.1012 0122649  89.2072 272.3014 16.05443269   101",
    );

    spicelib::ZZGETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        &mut ERRSIG,
        &mut ERROR,
        ctx,
    )?;
    testutil::CHCKSL(b"OK 4", ERRSIG, false, OK, ctx)?;

    spicelib::GETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADTLE)", OK, ctx)?;

    testutil::TCASE(b"Inclination high", ctx)?;

    fstr::assign(
        LINES.get_mut(1),
        b"1 25544U 98067A   98324.89267077  .00616830  11572-4  29139-3 0    84",
    );
    fstr::assign(
        LINES.get_mut(2),
        b"2 25544 181.5947 165.1012 0122649  89.2072 272.3014 16.05443269   101",
    );

    spicelib::ZZGETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        &mut ERRSIG,
        &mut ERROR,
        ctx,
    )?;
    testutil::CHCKSL(b"OK 5", ERRSIG, false, OK, ctx)?;

    spicelib::GETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADTLE)", OK, ctx)?;

    //
    // RA ascending node [0,360)
    //
    testutil::TCASE(b"RA node low", ctx)?;

    fstr::assign(
        LINES.get_mut(1),
        b"1 25544U 98067A   98324.89267077  .00616830  11572-4  29139-3 0    84",
    );
    fstr::assign(
        LINES.get_mut(2),
        b"2 25544  51.5947 -65.1012 0122649  89.2072 272.3014 16.05443269   101",
    );

    spicelib::ZZGETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        &mut ERRSIG,
        &mut ERROR,
        ctx,
    )?;
    testutil::CHCKSL(b"OK 4", ERRSIG, false, OK, ctx)?;

    spicelib::GETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADTLE)", OK, ctx)?;

    testutil::TCASE(b"RA node high", ctx)?;

    fstr::assign(
        LINES.get_mut(1),
        b"1 25544U 98067A   98324.89267077  .00616830  11572-4  29139-3 0    84",
    );
    fstr::assign(
        LINES.get_mut(2),
        b"2 25544  51.5947 365.1012 0122649  89.2072 272.3014 16.05443269   101",
    );

    spicelib::ZZGETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        &mut ERRSIG,
        &mut ERROR,
        ctx,
    )?;
    testutil::CHCKSL(b"OK 4", ERRSIG, false, OK, ctx)?;

    spicelib::GETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADTLE)", OK, ctx)?;

    //
    // Arg periapsis [0,360)
    //
    testutil::TCASE(b"Arg periapsis low", ctx)?;

    fstr::assign(
        LINES.get_mut(1),
        b"1 25544U 98067A   98324.89267077  .00616830  11572-4  29139-3 0    84",
    );
    fstr::assign(
        LINES.get_mut(2),
        b"2 25544  51.5947  65.1012 0122649 -89.2072 272.3014 16.05443269   101",
    );

    spicelib::ZZGETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        &mut ERRSIG,
        &mut ERROR,
        ctx,
    )?;
    testutil::CHCKSL(b"OK 4", ERRSIG, false, OK, ctx)?;

    spicelib::GETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADTLE)", OK, ctx)?;

    testutil::TCASE(b"Arg periapsis high", ctx)?;

    fstr::assign(
        LINES.get_mut(1),
        b"1 25544U 98067A   98324.89267077  .00616830  11572-4  29139-3 0    84",
    );
    fstr::assign(
        LINES.get_mut(2),
        b"2 25544  51.5947  65.1012 0122649 389.2072 272.3014 16.05443269   101",
    );

    spicelib::ZZGETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        &mut ERRSIG,
        &mut ERROR,
        ctx,
    )?;
    testutil::CHCKSL(b"OK 4", ERRSIG, false, OK, ctx)?;

    spicelib::GETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADTLE)", OK, ctx)?;

    //
    // Mean anomoly [0,360)
    //
    testutil::TCASE(b"Mean anomoly low", ctx)?;

    fstr::assign(
        LINES.get_mut(1),
        b"1 25544U 98067A   98324.89267077  .00616830  11572-4  29139-3 0    84",
    );
    fstr::assign(
        LINES.get_mut(2),
        b"2 25544  51.5947 165.1012 0122649  89.2072 -72.3014 16.05443269   101",
    );

    spicelib::ZZGETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        &mut ERRSIG,
        &mut ERROR,
        ctx,
    )?;
    testutil::CHCKSL(b"OK 4", ERRSIG, false, OK, ctx)?;

    spicelib::GETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADTLE)", OK, ctx)?;

    testutil::TCASE(b"Mean anomoly high", ctx)?;

    fstr::assign(
        LINES.get_mut(1),
        b"1 25544U 98067A   98324.89267077  .00616830  11572-4  29139-3 0    84",
    );
    fstr::assign(
        LINES.get_mut(2),
        b"2 25544  51.5947 165.1012 0122649  89.2072 360.3014 16.05443269   101",
    );

    spicelib::ZZGETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        &mut ERRSIG,
        &mut ERROR,
        ctx,
    )?;
    testutil::CHCKSL(b"OK 4", ERRSIG, false, OK, ctx)?;

    spicelib::GETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADTLE)", OK, ctx)?;

    //
    // Mean motion (0,20)
    //
    testutil::TCASE(b"Mean motion low", ctx)?;

    fstr::assign(
        LINES.get_mut(1),
        b"1 25544U 98067A   98324.89267077  .00616830  11572-4  29139-3 0    84",
    );
    fstr::assign(
        LINES.get_mut(2),
        b"2 25544  51.5947 165.1012 0122649  89.2072 272.3014 -16.05443269   101",
    );

    spicelib::ZZGETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        &mut ERRSIG,
        &mut ERROR,
        ctx,
    )?;
    testutil::CHCKSL(b"OK 4", ERRSIG, false, OK, ctx)?;

    spicelib::GETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADTLE)", OK, ctx)?;

    testutil::TCASE(b"Mean motion high", ctx)?;

    fstr::assign(
        LINES.get_mut(1),
        b"1 25544U 98067A   98324.89267077  .00616830  11572-4  29139-3 0    84",
    );
    fstr::assign(
        LINES.get_mut(2),
        b"2 25544  51.5947 165.1012 0122649  89.2072 272.3014 20.05443269   101",
    );

    spicelib::ZZGETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        &mut ERRSIG,
        &mut ERROR,
        ctx,
    )?;
    testutil::CHCKSL(b"OK 4", ERRSIG, false, OK, ctx)?;

    spicelib::GETELM(
        FRSTYR,
        LINES.as_arg(),
        &mut EPOCH,
        ELEMS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADTLE)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
