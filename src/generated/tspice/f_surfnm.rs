//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.00000000000001;
const MSGLEN: i32 = 240;

struct SaveVars {
    ORIGIN: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 3 as usize))
                .chain([]);

            ORIGIN
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { ORIGIN }
    }
}

//$Procedure F_SURFNM ( SURFNM tests )
pub fn F_SURFNM(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TITLE = [b' '; MSGLEN as usize];
    let mut A: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut C: f64 = 0.0;
    let mut SRFPT = StackArray::<f64, 3>::new(1..=3);
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut XNORML = StackArray::<f64, 3>::new(1..=3);
    let mut SCALE: f64 = 0.0;
    let mut U = StackArray::<f64, 3>::new(1..=3);
    let mut X: f64 = 0.0;
    let mut Y: f64 = 0.0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Other functions
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

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_SURFNM", ctx)?;

    //
    //     Run some simple tests where the correct results can be
    //     determined by inspection.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"SURFNM simple case:  point is at top of unit sphere",
    );
    testutil::TCASE(&TITLE, ctx)?;

    A = 1.0;
    B = 1.0;
    C = 1.0;

    spicelib::VPACK(0.0, 0.0, 1.0, SRFPT.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, XNORML.as_slice_mut());

    spicelib::SURFNM(A, B, C, SRFPT.as_slice(), NORMAL.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"NORMAL",
        NORMAL.as_slice(),
        b"~~/",
        XNORML.as_slice(),
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
        b"SURFNM simple case:  point is (sqrt(3)/3,sqrt(3)/3,sqrt(3)/3) on unit sphere",
    );
    testutil::TCASE(&TITLE, ctx)?;

    A = 1.0;
    B = 1.0;
    C = 1.0;

    X = (f64::sqrt(3.0) / 3.0);

    spicelib::VPACK(X, X, X, SRFPT.as_slice_mut());
    spicelib::VPACK(X, X, X, XNORML.as_slice_mut());

    spicelib::SURFNM(A, B, C, SRFPT.as_slice(), NORMAL.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"NORMAL",
        NORMAL.as_slice(),
        b"~~/",
        XNORML.as_slice(),
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
        b"SURFNM simple case: repeat previous case, but scale down sphere by 1.D-200",
    );
    testutil::TCASE(&TITLE, ctx)?;

    SCALE = 0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001;

    A = SCALE;
    B = SCALE;
    C = SCALE;

    X = (f64::sqrt(3.0) / 3.0);

    Y = (X * SCALE);

    spicelib::VPACK(Y, Y, Y, SRFPT.as_slice_mut());
    spicelib::VPACK(X, X, X, XNORML.as_slice_mut());

    spicelib::SURFNM(A, B, C, SRFPT.as_slice(), NORMAL.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"NORMAL",
        NORMAL.as_slice(),
        b"~~/",
        XNORML.as_slice(),
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
        b"SURFNM simple case: repeat previous case, but scale up sphere by 1.D200",
    );
    testutil::TCASE(&TITLE, ctx)?;

    SCALE = 100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;

    A = SCALE;
    B = SCALE;
    C = SCALE;

    X = (f64::sqrt(3.0) / 3.0);

    Y = (X * SCALE);

    spicelib::VPACK(Y, Y, Y, SRFPT.as_slice_mut());
    spicelib::VPACK(X, X, X, XNORML.as_slice_mut());

    spicelib::SURFNM(A, B, C, SRFPT.as_slice(), NORMAL.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"NORMAL",
        NORMAL.as_slice(),
        b"~~/",
        XNORML.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Triaxial case: axis lengths are 1, 2, 3.  Point lies along direction vector (1, 0.3, 0.5)");

    testutil::TCASE(&TITLE, ctx)?;

    A = 1.0;
    B = 2.0;
    C = 3.0;

    spicelib::VPACK(1.0, 0.3, 0.5, U.as_slice_mut());

    //
    // Find the surface point on the specified line.
    //
    spicelib::SURFPT(
        save.ORIGIN.as_slice(),
        U.as_slice(),
        A,
        B,
        C,
        SRFPT.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compute the normal direction; unitize the expected normal vector.
    //
    spicelib::VPACK(
        (SRFPT[1] / f64::powi(A, 2)),
        (SRFPT[2] / f64::powi(B, 2)),
        (SRFPT[3] / f64::powi(C, 2)),
        XNORML.as_slice_mut(),
    );

    spicelib::VHATIP(XNORML.as_slice_mut());

    //
    // See whether SURFNM agrees.
    //
    spicelib::SURFNM(A, B, C, SRFPT.as_slice(), NORMAL.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"NORMAL",
        NORMAL.as_slice(),
        b"~~/",
        XNORML.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // SURFNM error cases:
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INEDPL: ellipsoid has one zero-length axis.", ctx)?;

    spicelib::VPACK(1.0, 0.0, 0.0, SRFPT.as_slice_mut());

    spicelib::SURFNM(0.0, 1.0, 1.0, SRFPT.as_slice(), NORMAL.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::SURFNM(1.0, 0.0, 1.0, SRFPT.as_slice(), NORMAL.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::SURFNM(1.0, 1.0, 0.0, SRFPT.as_slice(), NORMAL.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SURFNM: ellipsoid has one negative-length axis.", ctx)?;

    spicelib::SURFNM(-1.0, 1.0, 1.0, SRFPT.as_slice(), NORMAL.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::SURFNM(1.0, -1.0, 1.0, SRFPT.as_slice(), NORMAL.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::SURFNM(1.0, 1.0, -1.0, SRFPT.as_slice(), NORMAL.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
