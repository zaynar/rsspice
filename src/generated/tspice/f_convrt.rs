//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const AU: f64 = (149597870613.68887 / 1000.0);
const FEET: f64 = 0.3048;
const INCHES: f64 = 0.0254;
const SMILES: f64 = 1609.344;
const NMILES: f64 = 1852.0;
const YARDS: f64 = 0.9144;
const VTIGHT: f64 = 0.00000000000001;
const LBLLEN: i32 = 50;
const NAMLEN: i32 = 32;
const NDIST: i32 = 17;
const NANG: i32 = 7;
const NTIME: i32 = 7;

struct SaveVars {
    ANAMES: ActualCharArray,
    DNAMES: ActualCharArray,
    LABEL: Vec<u8>,
    NAME: Vec<u8>,
    TNAMES: ActualCharArray,
    ANGVAL: StackArray<f64, 7>,
    DSTVAL: StackArray<f64, 17>,
    OUTVAL: f64,
    SCALE: f64,
    TIMVAL: StackArray<f64, 7>,
    TOL: f64,
    XVAL: f64,
    NUNIT: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ANAMES = ActualCharArray::new(NAMLEN, 1..=NANG);
        let mut DNAMES = ActualCharArray::new(NAMLEN, 1..=NDIST);
        let mut LABEL = vec![b' '; LBLLEN as usize];
        let mut NAME = vec![b' '; NAMLEN as usize];
        let mut TNAMES = ActualCharArray::new(NAMLEN, 1..=NTIME);
        let mut ANGVAL = StackArray::<f64, 7>::new(1..=NANG);
        let mut DSTVAL = StackArray::<f64, 17>::new(1..=NDIST);
        let mut OUTVAL: f64 = 0.0;
        let mut SCALE: f64 = 0.0;
        let mut TIMVAL = StackArray::<f64, 7>::new(1..=NTIME);
        let mut TOL: f64 = 0.0;
        let mut XVAL: f64 = 0.0;
        let mut NUNIT: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"radians"),
                Val::C(b"degrees"),
                Val::C(b"arcminutes"),
                Val::C(b"arcseconds"),
                Val::C(b"hourangle"),
                Val::C(b"minuteangle"),
                Val::C(b"secondangle"),
            ]
            .into_iter();
            ANAMES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"meters"),
                Val::C(b"km"),
                Val::C(b"cm"),
                Val::C(b"mm"),
                Val::C(b"lightsecs"),
                Val::C(b"au"),
                Val::C(b"m"),
                Val::C(b"kilometers"),
                Val::C(b"centimeters"),
                Val::C(b"millimeters"),
                Val::C(b"feet"),
                Val::C(b"inches"),
                Val::C(b"statute_miles"),
                Val::C(b"nautical_miles"),
                Val::C(b"yards"),
                Val::C(b"lightyears"),
                Val::C(b"parsecs"),
            ]
            .into_iter();
            DNAMES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"seconds"),
                Val::C(b"minutes"),
                Val::C(b"hours"),
                Val::C(b"days"),
                Val::C(b"julian_years"),
                Val::C(b"tropical_years"),
                Val::C(b"years"),
            ]
            .into_iter();
            TNAMES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ANAMES,
            DNAMES,
            LABEL,
            NAME,
            TNAMES,
            ANGVAL,
            DSTVAL,
            OUTVAL,
            SCALE,
            TIMVAL,
            TOL,
            XVAL,
            NUNIT,
        }
    }
}

//$Procedure F_CONVRT ( CONVRT tests )
pub fn F_CONVRT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // AU value from convrt.for, converted to km:
    //

    //
    // Other constants from convrt.for:
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
    testutil::TOPEN(b"F_CONVRT", ctx)?;

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup", ctx)?;
    //
    // Since we can call functions at run time, it's more convenient
    // to initialize some constants here than in a DATA statement.
    //
    // Start with angular units.
    //
    // We'll use RADIANs as our base value. Note that CONVRT
    // uses DEGREES.
    //
    save.ANGVAL[1] = 1.0;
    save.ANGVAL[2] = spicelib::RPD(ctx);
    save.ANGVAL[3] = (save.ANGVAL[2] / 60.0);
    save.ANGVAL[4] = (save.ANGVAL[3] / 60.0);
    save.ANGVAL[5] = (15.0 * save.ANGVAL[2]);
    save.ANGVAL[6] = (save.ANGVAL[5] / 60.0);
    save.ANGVAL[7] = (save.ANGVAL[6] / 60.0);

    //
    // Initialize distance units. We'll use KM as our base unit.
    // Note that CONVRT uses meters.
    //
    save.DSTVAL[1] = 0.001;
    save.DSTVAL[2] = 1.0;
    save.DSTVAL[3] = 0.00001;
    save.DSTVAL[4] = 0.000001;
    save.DSTVAL[5] = spicelib::CLIGHT();
    save.DSTVAL[6] = AU;
    save.DSTVAL[7] = 0.001;
    save.DSTVAL[8] = 1.0;
    save.DSTVAL[9] = 0.00001;
    save.DSTVAL[10] = 0.000001;
    save.DSTVAL[11] = (FEET * 0.001);
    save.DSTVAL[12] = (INCHES * 0.001);
    save.DSTVAL[13] = (SMILES * 0.001);
    save.DSTVAL[14] = (NMILES * 0.001);
    save.DSTVAL[15] = (YARDS * 0.001);
    save.DSTVAL[16] = (spicelib::CLIGHT() * spicelib::JYEAR());
    save.DSTVAL[17] = (AU / f64::sin((spicelib::RPD(ctx) / 3600 as f64)));

    //
    // Initialize time units. We'll use seconds as our base unit.
    //
    save.TIMVAL[1] = 1.0;
    save.TIMVAL[2] = 60.0;
    save.TIMVAL[3] = 3600.0;
    save.TIMVAL[4] = spicelib::SPD();
    save.TIMVAL[5] = spicelib::JYEAR();
    save.TIMVAL[6] = spicelib::TYEAR();
    save.TIMVAL[7] = spicelib::JYEAR();

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Convert each time unit to seconds; use a non-identity input value.",
        ctx,
    )?;

    save.NUNIT = NTIME;

    save.SCALE = 7.0;

    for I in 1..=save.NUNIT {
        spicelib::CONVRT(
            save.SCALE,
            &save.TNAMES[I],
            b"SECONDS",
            &mut save.OUTVAL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.LABEL, b"Time units <#> in seconds.");
        spicelib::REPMC(&save.LABEL.to_vec(), b"#", &save.TNAMES[I], &mut save.LABEL);

        save.XVAL = (save.TIMVAL[I] * save.SCALE);

        if spicelib::EQSTR(&save.TNAMES[I], b"TROPICAL_YEARS") {
            save.TOL = 0.0000000001;
        } else {
            save.TOL = VTIGHT;
        }

        testutil::CHCKSD(
            &save.LABEL,
            save.OUTVAL,
            b"~/",
            save.XVAL,
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Convert each time unit to seconds; use a non-identity input value. Change case of input string.", ctx)?;

    save.NUNIT = NTIME;

    save.SCALE = 7.0;

    for I in 1..=save.NUNIT {
        spicelib::UCASE(&save.TNAMES[I], &mut save.NAME, ctx);

        spicelib::CONVRT(save.SCALE, &save.NAME, b"SECONDS", &mut save.OUTVAL, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.LABEL, b"Time units <#> in seconds.");
        spicelib::REPMC(&save.LABEL.to_vec(), b"#", &save.TNAMES[I], &mut save.LABEL);

        save.XVAL = (save.TIMVAL[I] * save.SCALE);

        if spicelib::EQSTR(&save.TNAMES[I], b"TROPICAL_YEARS") {
            save.TOL = 0.0000000001;
        } else {
            save.TOL = VTIGHT;
        }

        testutil::CHCKSD(
            &save.LABEL,
            save.OUTVAL,
            b"~/",
            save.XVAL,
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Convert seconds to each time unit; use a non-identity input value.",
        ctx,
    )?;

    save.NUNIT = NTIME;

    save.SCALE = (1.0 / 3.0);

    for I in 1..=save.NUNIT {
        spicelib::CONVRT(
            save.SCALE,
            b"SECONDS",
            &save.TNAMES[I],
            &mut save.OUTVAL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.LABEL, b"Seconds in time units <#>.");
        spicelib::REPMC(&save.LABEL.to_vec(), b"#", &save.TNAMES[I], &mut save.LABEL);

        save.XVAL = (save.SCALE / save.TIMVAL[I]);

        if spicelib::EQSTR(&save.TNAMES[I], b"TROPICAL_YEARS") {
            save.TOL = 0.0000000001;
        } else {
            save.TOL = VTIGHT;
        }

        testutil::CHCKSD(
            &save.LABEL,
            save.OUTVAL,
            b"~/",
            save.XVAL,
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Convert seconds to each time unit; use a non-identity input value. Change case of output string.", ctx)?;

    save.NUNIT = NTIME;

    save.SCALE = (1.0 / 3.0);

    for I in 1..=save.NUNIT {
        spicelib::UCASE(&save.TNAMES[I], &mut save.NAME, ctx);

        spicelib::CONVRT(save.SCALE, b"SECONDS", &save.NAME, &mut save.OUTVAL, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.LABEL, b"Seconds in time units <#>.");
        spicelib::REPMC(&save.LABEL.to_vec(), b"#", &save.TNAMES[I], &mut save.LABEL);

        save.XVAL = (save.SCALE / save.TIMVAL[I]);

        if spicelib::EQSTR(&save.TNAMES[I], b"TROPICAL_YEARS") {
            save.TOL = 0.0000000001;
        } else {
            save.TOL = VTIGHT;
        }

        testutil::CHCKSD(
            &save.LABEL,
            save.OUTVAL,
            b"~/",
            save.XVAL,
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Convert each distance unit to km; use a non-identity input value.",
        ctx,
    )?;

    save.NUNIT = NDIST;

    save.SCALE = 7.0;

    for I in 1..=save.NUNIT {
        spicelib::CONVRT(save.SCALE, &save.DNAMES[I], b"KM", &mut save.OUTVAL, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.LABEL, b"Distance units <#> in km.");
        spicelib::REPMC(&save.LABEL.to_vec(), b"#", &save.DNAMES[I], &mut save.LABEL);

        save.XVAL = (save.DSTVAL[I] * save.SCALE);

        save.TOL = VTIGHT;

        testutil::CHCKSD(
            &save.LABEL,
            save.OUTVAL,
            b"~/",
            save.XVAL,
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Convert each distance unit to km; use a non-identity input value. Change case of input string.", ctx)?;

    save.NUNIT = NDIST;

    save.SCALE = 7.0;

    for I in 1..=save.NUNIT {
        spicelib::UCASE(&save.DNAMES[I], &mut save.NAME, ctx);

        spicelib::CONVRT(save.SCALE, &save.NAME, b"KM", &mut save.OUTVAL, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.LABEL, b"Distance units <#> in km.");
        spicelib::REPMC(&save.LABEL.to_vec(), b"#", &save.DNAMES[I], &mut save.LABEL);

        save.XVAL = (save.DSTVAL[I] * save.SCALE);

        save.TOL = VTIGHT;

        testutil::CHCKSD(
            &save.LABEL,
            save.OUTVAL,
            b"~/",
            save.XVAL,
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Convert km to each distance unit; use a non-identity input value.",
        ctx,
    )?;

    save.NUNIT = NDIST;

    save.SCALE = (1.0 / 3.0);

    for I in 1..=save.NUNIT {
        spicelib::CONVRT(save.SCALE, b"KM", &save.DNAMES[I], &mut save.OUTVAL, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.LABEL, b"KM in distance units <#>.");
        spicelib::REPMC(&save.LABEL.to_vec(), b"#", &save.DNAMES[I], &mut save.LABEL);

        save.XVAL = (save.SCALE / save.DSTVAL[I]);

        save.TOL = VTIGHT;

        testutil::CHCKSD(
            &save.LABEL,
            save.OUTVAL,
            b"~/",
            save.XVAL,
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Convert km to each distance unit; use a non-identity input value. Change case of input string.", ctx)?;

    save.NUNIT = NDIST;

    save.SCALE = (1.0 / 3.0);

    for I in 1..=save.NUNIT {
        spicelib::UCASE(&save.DNAMES[I], &mut save.NAME, ctx);

        spicelib::CONVRT(save.SCALE, b"KM", &save.NAME, &mut save.OUTVAL, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.LABEL, b"KM in distance units <#>.");
        spicelib::REPMC(&save.LABEL.to_vec(), b"#", &save.DNAMES[I], &mut save.LABEL);

        save.XVAL = (save.SCALE / save.DSTVAL[I]);

        save.TOL = VTIGHT;

        testutil::CHCKSD(
            &save.LABEL,
            save.OUTVAL,
            b"~/",
            save.XVAL,
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Convert each angular unit to radians; use a non-identity input value.",
        ctx,
    )?;

    save.NUNIT = NANG;

    save.SCALE = 11.0;

    for I in 1..=save.NUNIT {
        spicelib::CONVRT(
            save.SCALE,
            &save.ANAMES[I],
            b"RADIANS",
            &mut save.OUTVAL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.LABEL, b"Angular units <#> in radians.");
        spicelib::REPMC(&save.LABEL.to_vec(), b"#", &save.ANAMES[I], &mut save.LABEL);

        save.XVAL = (save.ANGVAL[I] * save.SCALE);

        save.TOL = VTIGHT;

        testutil::CHCKSD(
            &save.LABEL,
            save.OUTVAL,
            b"~/",
            save.XVAL,
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Convert each angular unit to radians; use a non-identity input value. Change case of input string.", ctx)?;

    save.NUNIT = NANG;

    save.SCALE = 7.0;

    for I in 1..=save.NUNIT {
        spicelib::UCASE(&save.ANAMES[I], &mut save.NAME, ctx);

        spicelib::CONVRT(save.SCALE, &save.NAME, b"RADIANS", &mut save.OUTVAL, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.LABEL, b"Angular units <#> in radians.");
        spicelib::REPMC(&save.LABEL.to_vec(), b"#", &save.ANAMES[I], &mut save.LABEL);

        save.XVAL = (save.ANGVAL[I] * save.SCALE);

        save.TOL = VTIGHT;

        testutil::CHCKSD(
            &save.LABEL,
            save.OUTVAL,
            b"~/",
            save.XVAL,
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Convert radians to each angular unit; use a non-identity input value.",
        ctx,
    )?;

    save.NUNIT = NANG;

    save.SCALE = (1.0 / 3.0);

    for I in 1..=save.NUNIT {
        spicelib::CONVRT(
            save.SCALE,
            b"RADIANS",
            &save.ANAMES[I],
            &mut save.OUTVAL,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.LABEL, b"RADIANS in angular units <#>.");
        spicelib::REPMC(&save.LABEL.to_vec(), b"#", &save.ANAMES[I], &mut save.LABEL);

        save.XVAL = (save.SCALE / save.ANGVAL[I]);

        save.TOL = VTIGHT;

        testutil::CHCKSD(
            &save.LABEL,
            save.OUTVAL,
            b"~/",
            save.XVAL,
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Convert radians to each angular unit; use a non-identity input value. Change case of input string.", ctx)?;

    save.NUNIT = NANG;

    save.SCALE = (1.0 / 3.0);

    for I in 1..=save.NUNIT {
        spicelib::UCASE(&save.ANAMES[I], &mut save.NAME, ctx);

        spicelib::CONVRT(save.SCALE, b"RADIANS", &save.NAME, &mut save.OUTVAL, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.LABEL, b"RADIANS in angular units <#>.");
        spicelib::REPMC(&save.LABEL.to_vec(), b"#", &save.ANAMES[I], &mut save.LABEL);

        save.XVAL = (save.SCALE / save.ANGVAL[I]);

        save.TOL = VTIGHT;

        testutil::CHCKSD(
            &save.LABEL,
            save.OUTVAL,
            b"~/",
            save.XVAL,
            save.TOL,
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
    testutil::TCASE(b"Incompatible units.", ctx)?;

    spicelib::CONVRT(1.0, b"RADIANS", b"KM", &mut save.OUTVAL, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INCOMPATIBLEUNITS)", OK, ctx)?;

    spicelib::CONVRT(1.0, b"RADIANS", b"SECONDS", &mut save.OUTVAL, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INCOMPATIBLEUNITS)", OK, ctx)?;

    spicelib::CONVRT(1.0, b"SECONDS", b"M", &mut save.OUTVAL, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INCOMPATIBLEUNITS)", OK, ctx)?;

    spicelib::CONVRT(1.0, b"SECONDS", b"RADIANS", &mut save.OUTVAL, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INCOMPATIBLEUNITS)", OK, ctx)?;

    spicelib::CONVRT(1.0, b"DEGREES", b"M", &mut save.OUTVAL, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INCOMPATIBLEUNITS)", OK, ctx)?;

    spicelib::CONVRT(1.0, b"DEGREES", b"HOURS", &mut save.OUTVAL, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INCOMPATIBLEUNITS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Unrecognized units.", ctx)?;

    spicelib::CONVRT(1.0, b"SECONDS", b"Z", &mut save.OUTVAL, ctx)?;
    testutil::CHCKXC(true, b"SPICE(UNITSNOTREC)", OK, ctx)?;

    spicelib::CONVRT(1.0, b"X", b"RADIANS", &mut save.OUTVAL, ctx)?;
    testutil::CHCKXC(true, b"SPICE(UNITSNOTREC)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
