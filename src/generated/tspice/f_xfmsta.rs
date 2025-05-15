//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const PCK: &[u8] = b"xfmsta.pck";
const NLINES: i32 = 10;
const RANDS: i32 = 1000;

struct SaveVars {
    COSYS: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut COSYS = ActualCharArray::new(30, 1..=6);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"RECTANGULAR")].into_iter();
            fstr::assign(COSYS.get_mut(1), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"CYLINDRICAL")].into_iter();
            fstr::assign(COSYS.get_mut(2), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"LATITUDINAL")].into_iter();
            fstr::assign(COSYS.get_mut(3), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"SPHERICAL")].into_iter();
            fstr::assign(COSYS.get_mut(4), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"GEODETIC")].into_iter();
            fstr::assign(COSYS.get_mut(5), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"PLANETOGRAPHIC")].into_iter();
            fstr::assign(COSYS.get_mut(6), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { COSYS }
    }
}

//$Procedure      F_XFMSTA ( Family of tests for XFMSTA )
pub fn F_XFMSTA(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DIM: i32 = 0;
    let mut LOWERP: f64 = 0.0;
    let mut LOWERV: f64 = 0.0;
    let mut TOL: f64 = 0.0;
    let mut UPPERP: f64 = 0.0;
    let mut UPPERV: f64 = 0.0;
    let mut BADSTA = StackArray::<f64, 6>::new(1..=6);
    let mut ISTATE = StackArray::<f64, 6>::new(1..=6);
    let mut OSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut R = StackArray::<f64, 3>::new(1..=3);
    let mut SAVRAD = StackArray::<f64, 3>::new(1..=3);
    let mut TEMP = StackArray::<f64, 6>::new(1..=6);
    let mut TEMP2 = StackArray::<f64, 6>::new(1..=6);
    let mut SDSIGN: i32 = 0;
    let mut SEDPOS: i32 = 0;
    let mut SEDVEL: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Other Functions
    //
    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Seed values for input state vector, input coordinate
    // system, and output coordinate system.
    //
    //
    // Coordinate Systems:
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_XFMSTA", ctx)?;

    testutil::TCASE(b"Setup: create and load the PCK.", ctx)?;
    //
    // Create a PCK, load using FURNSH.
    //
    testutil::T_PCK08(PCK, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Save the true radii value to a variable.
    //
    spicelib::BODVRD(b"EARTH", b"RADII", 3, &mut DIM, SAVRAD.as_slice_mut(), ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     Make sure incorrect coordinate frames are not accepted.
    //
    testutil::TCASE(
        b"Make sure an incorrect coordinate frame input is not accepted. ",
        ctx,
    )?;
    ISTATE[1] = 1.0;
    ISTATE[2] = 1.0;
    ISTATE[3] = 1.0;
    ISTATE[4] = 1.0;
    ISTATE[5] = 1.0;
    ISTATE[6] = 1.0;
    spicelib::XFMSTA(
        ISTATE.as_slice(),
        b"LATITUD",
        b"RECTANGULAR",
        b" ",
        OSTATE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(COORDSYSNOTREC)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     Make sure incorrect coordinate frames are not accepted.
    //
    testutil::TCASE(
        b"Make sure an incorrect coordinate frame output is not accepted. ",
        ctx,
    )?;

    spicelib::XFMSTA(
        ISTATE.as_slice(),
        b"CYLINDRICAL",
        b"RECTANLAR",
        b" ",
        OSTATE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(COORDSYSNOTREC)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     Make sure incorrect coordinate frames are not accepted.
    //
    testutil::TCASE(
        b"Make sure an incorrect coordinate frame input and output are not accepted. ",
        ctx,
    )?;

    spicelib::XFMSTA(
        ISTATE.as_slice(),
        b"CYLINDRCL",
        b"PLANETOGRPHIC",
        b" ",
        OSTATE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(COORDSYSNOTREC)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     Make sure identical input/output coordinate systems return the
    //     input.
    //
    testutil::TCASE(
        b"Make sure identical input and output coordinate systems return the input. ",
        ctx,
    )?;

    spicelib::XFMSTA(
        ISTATE.as_slice(),
        b"CYLINDRICAL",
        b"CYLINDRICAL",
        b" ",
        OSTATE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKAD(
        b"OSTATE",
        OSTATE.as_slice(),
        b"=",
        ISTATE.as_slice(),
        6,
        0.0000000000001,
        OK,
        ctx,
    )?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     For planetographic or geodetic, verify an equatorial radius of
    //     zero is not accepted.
    //
    testutil::TCASE(
        b"Verify an equatorial radius of zero is not accepted for geodetic/planetographic.",
        ctx,
    )?;

    R[1] = 0.0;
    R[2] = 6378.14;
    R[3] = 6356.75;
    spicelib::PDPOOL(b"BODY399_RADII", 3, R.as_slice(), ctx)?;

    spicelib::XFMSTA(
        ISTATE.as_slice(),
        b"GEODETIC",
        b"RECTANGULAR",
        b"EARTH",
        OSTATE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // Every time the kernel pool is altered, reset the value
    // after the test.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, SAVRAD.as_slice(), ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     For planetographic or geodetic, verify (a-b)/a overflow case.
    //
    testutil::TCASE(
        b"Verify numeric overflow case for flattening coeff for the geodetic case. ",
        ctx,
    )?;

    R[1] = 0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001;
    R[2] = 6378.14;
    R[3] = 10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    spicelib::PDPOOL(b"BODY399_RADII", 3, R.as_slice(), ctx)?;

    spicelib::XFMSTA(
        ISTATE.as_slice(),
        b"GEODETIC",
        b"RECTANGULAR",
        b"EARTH",
        OSTATE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDRADIUS)", OK, ctx)?;
    //
    // Every time the kernel pool is altered, reset the value
    // after the test.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, SAVRAD.as_slice(), ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     For planetographic or geodetic, verify the radii cannot be less
    //     than zero.
    //
    testutil::TCASE(b"Verify error signal on negative radii.", ctx)?;

    R[1] = -1.0;
    R[2] = 6378.14;
    R[3] = -1.0;
    spicelib::PDPOOL(b"BODY399_RADII", 3, R.as_slice(), ctx)?;

    spicelib::XFMSTA(
        ISTATE.as_slice(),
        b"GEODETIC",
        b"RECTANGULAR",
        b"EARTH",
        OSTATE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;
    //
    // Every time the kernel pool is altered, reset the value
    // after the test.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, SAVRAD.as_slice(), ctx)?;

    //
    //---- Case -----------------------------------------------------------
    //
    //     For planetographic or geodetic, verify the exception for bodies
    //     with unequal equatorial radii.
    //
    testutil::TCASE(b"Verify the exception for geodetic and planetographic conversions for bodies with unequal equatorial radii.", ctx)?;

    spicelib::XFMSTA(
        ISTATE.as_slice(),
        b"RECTANGULAR",
        b"GEODETIC",
        b"PHOBOS",
        TEMP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    spicelib::XFMSTA(
        ISTATE.as_slice(),
        b"RECTANGULAR",
        b"PLANETOGRAPHIC",
        b"PHOBOS",
        TEMP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    spicelib::XFMSTA(
        ISTATE.as_slice(),
        b"GEODETIC",
        b"RECTANGULAR",
        b"PHOBOS",
        TEMP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    spicelib::XFMSTA(
        ISTATE.as_slice(),
        b"PLANETOGRAPHIC",
        b"RECTANGULAR",
        b"PHOBOS",
        TEMP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     Make sure incorrect body names are not accepted (for geodetic)
    testutil::TCASE(
        b"Make sure incorrect body names are not accepted for the geodetic case. ",
        ctx,
    )?;

    spicelib::XFMSTA(
        ISTATE.as_slice(),
        b"GEODETIC",
        b"RECTANGULAR",
        b"ERTH",
        OSTATE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     Make sure incorrect body names are not accepted (planetographic)
    //
    testutil::TCASE(
        b"Make sure incorrect body names are not accepted for the planetographic case. ",
        ctx,
    )?;

    spicelib::XFMSTA(
        ISTATE.as_slice(),
        b"PLANETOGRAPHIC",
        b"RECTANGULAR",
        b"MRS",
        OSTATE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     Make sure NAIF ID strings are accepted
    //
    testutil::TCASE(b"Make sure NAIF ID strings are accepted. ", ctx)?;

    spicelib::XFMSTA(
        ISTATE.as_slice(),
        b"geodetic",
        b"rectangular",
        b"399",
        OSTATE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     Verify the conversion to non-rectangular does not work for
    //     (X=0, Y=0, Z, Vx, Vy, Vz)
    //
    testutil::TCASE(b"Verify the z-axis when converting to anything except rectangular is not allowed if only the position is on the z-axis.", ctx)?;

    ISTATE[1] = 0.0;
    ISTATE[2] = 0.0;
    ISTATE[3] = 0.5;
    ISTATE[4] = 0.2;
    ISTATE[5] = 0.1;
    ISTATE[6] = -0.2;

    for I in 2..=6 {
        spicelib::XFMSTA(
            ISTATE.as_slice(),
            &save.COSYS[1],
            &save.COSYS[I],
            b"EARTH",
            OSTATE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(INVALIDSTATE)", OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    //     Verify the conversion to non-rectangular works even if
    //     the Jacobian is not defined if both the position and velocity
    //     are along the z-axis.
    //
    testutil::TCASE(
        b"Verify the z-axis when converting to anything except rectangular ",
        ctx,
    )?;
    ISTATE[1] = 0.0;
    ISTATE[2] = 0.0;
    ISTATE[3] = 0.5;
    ISTATE[4] = 0.0;
    ISTATE[5] = 0.0;
    ISTATE[6] = 0.5;
    TEMP[4] = ISTATE[6];
    TEMP[5] = 0.0;
    TEMP[6] = 0.0;

    for I in 2..=6 {
        spicelib::XFMSTA(
            ISTATE.as_slice(),
            &save.COSYS[1],
            &save.COSYS[I],
            b"EARTH",
            OSTATE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (((I == 2) || (I == 5)) || (I == 6)) {
            //
            // If the output coordinate system is cylindrical,
            // geodetic, or planetographic, make sure the velocity
            // output is (0, 0, dz_dt).
            //
            testutil::CHCKAD(
                b"OSTATE",
                OSTATE.subarray(4),
                b"~",
                ISTATE.subarray(4),
                3,
                0.0000000000001,
                OK,
                ctx,
            )?;
        //
        // If the output coordinate system is spherical or
        // latitudinal, make sure the velocity output is
        // (dz_dt, 0, 0).
        //
        } else {
            testutil::CHCKAD(
                b"OSTATE",
                OSTATE.subarray(4),
                b"~",
                TEMP.subarray(4),
                3,
                0.0000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    //     Verify a zero vector case.
    //
    testutil::TCASE(b"Verify all combinations of coordinate systems ", ctx)?;

    ISTATE[1] = 0.0;
    ISTATE[2] = 0.0;
    ISTATE[3] = 0.0;
    ISTATE[4] = 0.0;
    ISTATE[5] = 0.0;
    ISTATE[6] = 0.0;

    spicelib::XFMSTA(
        ISTATE.as_slice(),
        b"CYLINDRICAL",
        b"RECTANGULAR",
        b" ",
        OSTATE.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"OSTATE",
        OSTATE.as_slice(),
        b"~",
        ISTATE.as_slice(),
        6,
        0.00000000000001,
        OK,
        ctx,
    )?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     Verify all possible combinations of TO and FROM
    //

    testutil::TCASE(b"Verify all combinations of coordinate systems ", ctx)?;
    ISTATE[1] = 1.0;
    ISTATE[2] = 0.5;
    ISTATE[3] = 0.5;
    ISTATE[4] = 0.2;
    ISTATE[5] = 0.1;
    ISTATE[6] = -0.2;

    for I in 1..=6 {
        for J in 1..=6 {
            if (I != J) {
                spicelib::XFMSTA(
                    ISTATE.as_slice(),
                    &save.COSYS[I],
                    &save.COSYS[J],
                    b"EARTH",
                    TEMP.as_slice_mut(),
                    ctx,
                )?;
                spicelib::XFMSTA(
                    TEMP.as_slice(),
                    &save.COSYS[J],
                    &save.COSYS[I],
                    b"EARTH",
                    OSTATE.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKAD(
                    b"OSTATE",
                    OSTATE.as_slice(),
                    b"~",
                    ISTATE.as_slice(),
                    6,
                    0.00000000001,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    //     Verify the overflow case of a non-rectangular input to
    //     any coordinate system (it will be translated to rectangular
    //     within the code). (Test 1)
    //
    testutil::TCASE(b"Bad velocity input -> rec ", ctx)?;

    BADSTA[1] = 300.0;
    BADSTA[2] = 0.5;
    BADSTA[3] = 0.5;
    BADSTA[4] = 10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    BADSTA[5] = 10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    BADSTA[6] = 10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;

    spicelib::XFMSTA(
        BADSTA.as_slice(),
        b"LATITUDINAL",
        b"RECTANGULAR",
        b" ",
        OSTATE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NUMERICOVERFLOW)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     Verify the overflow case of rectangular -> output.
    //     There are two different places in the code that can signal
    //     the near-singular error, which is why two test cases with
    //     different inputs must be run.  (Test 2)
    //
    testutil::TCASE(b"Bad velocity rec -> output ", ctx)?;

    BADSTA[1] = 300.0;
    BADSTA[2] = 0.5;
    BADSTA[3] = 0.5;
    BADSTA[4] = 10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    BADSTA[5] = 10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    BADSTA[6] = 10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;

    spicelib::XFMSTA(
        BADSTA.as_slice(),
        b"RECTANGULAR",
        b"LATITUDINAL",
        b" ",
        OSTATE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NUMERICOVERFLOW)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    //     Verify a series of random cases for real possible input.
    //     Distances on the order of 10^11 can be reached for missions
    //     like Voyager.  Calculations including Pluto are around that
    //     order.
    //
    //     The tolerance was chosen because the boundary values are on the
    //     order of 10^11.  The output values will match the expected on
    //     by x*10^-5, which can be greater than 1*10^-5.  The tolerance
    //     allows for all numbers on the order of 10^-5 by being 10^-4.
    //
    //     Rectangular is used as the 'to' and 'from' states since there
    //     are no angle limits for the input state.
    //
    testutil::TCASE(b"Verify a series of random input states from rectangular to every other coordinate system and back to rectangular.", ctx)?;

    SEDPOS = -1;
    SEDVEL = -1;
    SDSIGN = -1;

    LOWERP = -6.0;
    UPPERP = 11.0;

    LOWERV = -7.0;
    UPPERV = 7.0;

    TOL = 0.0001;

    for K in 1..=RANDS {
        //
        // T_RANDSIGN is used to determine a random sign for
        // the components of ISTATE.
        //
        ISTATE[1] = ((testutil::T_RANDSIGN(&mut SDSIGN, ctx) as f64)
            * f64::powf(10.0, testutil::T_RANDD(LOWERP, UPPERP, &mut SEDPOS, ctx)?));
        ISTATE[2] = ((testutil::T_RANDSIGN(&mut SDSIGN, ctx) as f64)
            * f64::powf(10.0, testutil::T_RANDD(LOWERP, UPPERP, &mut SEDPOS, ctx)?));
        ISTATE[3] = ((testutil::T_RANDSIGN(&mut SDSIGN, ctx) as f64)
            * f64::powf(10.0, testutil::T_RANDD(LOWERP, UPPERP, &mut SEDPOS, ctx)?));
        ISTATE[4] = ((testutil::T_RANDSIGN(&mut SDSIGN, ctx) as f64)
            * f64::powf(10.0, testutil::T_RANDD(LOWERV, UPPERV, &mut SEDVEL, ctx)?));
        ISTATE[5] = ((testutil::T_RANDSIGN(&mut SDSIGN, ctx) as f64)
            * f64::powf(10.0, testutil::T_RANDD(LOWERV, UPPERV, &mut SEDVEL, ctx)?));
        ISTATE[6] = ((testutil::T_RANDSIGN(&mut SDSIGN, ctx) as f64)
            * f64::powf(10.0, testutil::T_RANDD(LOWERV, UPPERV, &mut SEDVEL, ctx)?));

        for J in 2..=6 {
            spicelib::XFMSTA(
                ISTATE.as_slice(),
                b"RECTANGULAR",
                &save.COSYS[J],
                b"EARTH",
                TEMP.as_slice_mut(),
                ctx,
            )?;
            spicelib::XFMSTA(
                TEMP.as_slice(),
                &save.COSYS[J],
                b"RECTANGULAR",
                b"EARTH",
                OSTATE.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKAD(
                b"OSTATE",
                OSTATE.as_slice(),
                b"~",
                ISTATE.as_slice(),
                6,
                TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    //     Verify to and from every coordinate system works (except
    //     rectangular, which was already tested).  The test
    //     starts out in rectangular to ensure inputs of the right order
    //     are chosen.  The state is converted to a non-rectangular
    //     coordinate system (J), then converted from that coordinate system
    //     to another coordinate system (K) and back to (J).
    //
    testutil::TCASE(b"Verify a series of random input states from non-rectangular coordinate systems to every other non-rectangular coordinate system.", ctx)?;

    SEDPOS = -1;
    SEDVEL = -1;

    LOWERP = -3.0;
    UPPERP = 9.0;

    LOWERV = -2.0;
    UPPERV = 2.0;

    TOL = 0.1;

    for K in 1..=RANDS {
        //
        // T_RANDSIGN is used to determine a random sign for
        // the components of ISTATE.
        //
        ISTATE[1] = ((testutil::T_RANDSIGN(&mut SDSIGN, ctx) as f64)
            * f64::powf(10.0, testutil::T_RANDD(LOWERP, UPPERP, &mut SEDPOS, ctx)?));
        ISTATE[2] = ((testutil::T_RANDSIGN(&mut SDSIGN, ctx) as f64)
            * f64::powf(10.0, testutil::T_RANDD(LOWERP, UPPERP, &mut SEDPOS, ctx)?));
        ISTATE[3] = ((testutil::T_RANDSIGN(&mut SDSIGN, ctx) as f64)
            * f64::powf(10.0, testutil::T_RANDD(LOWERP, UPPERP, &mut SEDPOS, ctx)?));
        ISTATE[4] = ((testutil::T_RANDSIGN(&mut SDSIGN, ctx) as f64)
            * f64::powf(10.0, testutil::T_RANDD(LOWERV, UPPERV, &mut SEDVEL, ctx)?));
        ISTATE[5] = ((testutil::T_RANDSIGN(&mut SDSIGN, ctx) as f64)
            * f64::powf(10.0, testutil::T_RANDD(LOWERV, UPPERV, &mut SEDVEL, ctx)?));
        ISTATE[6] = ((testutil::T_RANDSIGN(&mut SDSIGN, ctx) as f64)
            * f64::powf(10.0, testutil::T_RANDD(LOWERV, UPPERV, &mut SEDVEL, ctx)?));

        for J in 2..=6 {
            //
            // Convert to a non-rectangular coordinate system.
            //
            spicelib::XFMSTA(
                ISTATE.as_slice(),
                b"RECTANGULAR",
                &save.COSYS[J],
                b"EARTH",
                TEMP.as_slice_mut(),
                ctx,
            )?;

            for L in 2..=6 {
                if (J != L) {
                    //
                    // Convert from the 'J' coordinate system to 'K'
                    // and then back to 'J'.  Check that the resulting
                    // state is within the tolerance of the initial
                    // state.
                    //
                    spicelib::XFMSTA(
                        TEMP.as_slice(),
                        &save.COSYS[J],
                        &save.COSYS[L],
                        b"EARTH",
                        TEMP2.as_slice_mut(),
                        ctx,
                    )?;
                    spicelib::XFMSTA(
                        TEMP2.as_slice(),
                        &save.COSYS[L],
                        &save.COSYS[J],
                        b"EARTH",
                        OSTATE.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    testutil::CHCKAD(
                        b"OSTATE",
                        OSTATE.as_slice(),
                        b"~",
                        TEMP.as_slice(),
                        6,
                        TOL,
                        OK,
                        ctx,
                    )?;
                }
            }
        }
    }

    //
    //
    //---- Case n -----------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::KCLEAR(ctx)?;

    spicelib::DELFIL(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
