//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VTIGHT: f64 = 0.00000000000001;
const LBCELL: i32 = -5;
const LNSIZE: i32 = 255;
const MAXV: i32 = 1000;
const MAXP: i32 = (2 * MAXV);

struct SaveVars {
    TITLE: Vec<u8>,
    A: f64,
    AREA: f64,
    B: f64,
    C: f64,
    CENTER: StackArray<f64, 3>,
    DELTA: f64,
    EDGE1: StackArray<f64, 3>,
    EDGE2: StackArray<f64, 3>,
    IVERTS: StackArray2D<f64, 9>,
    NORMAL: StackArray<f64, 3>,
    OFFSET: StackArray<f64, 3>,
    OVERTS: StackArray2D<f64, 9>,
    S: f64,
    V1: StackArray<f64, 3>,
    V2: StackArray<f64, 3>,
    V3: StackArray<f64, 3>,
    VOL: f64,
    VOUT: ActualArray<f64>,
    VOUT1: ActualArray<f64>,
    VRTCES: ActualArray2D<f64>,
    XAREA: f64,
    XNORML: StackArray<f64, 3>,
    XVERTS: StackArray2D<f64, 9>,
    XVOL: f64,
    NP: i32,
    NV: i32,
    PLATES: ActualArray2D<i32>,
    PLATS2: ActualArray2D<i32>,
    POUT: ActualArray<i32>,
    POUT1: ActualArray<i32>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut A: f64 = 0.0;
        let mut AREA: f64 = 0.0;
        let mut B: f64 = 0.0;
        let mut C: f64 = 0.0;
        let mut CENTER = StackArray::<f64, 3>::new(1..=3);
        let mut DELTA: f64 = 0.0;
        let mut EDGE1 = StackArray::<f64, 3>::new(1..=3);
        let mut EDGE2 = StackArray::<f64, 3>::new(1..=3);
        let mut IVERTS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
        let mut OVERTS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut S: f64 = 0.0;
        let mut V1 = StackArray::<f64, 3>::new(1..=3);
        let mut V2 = StackArray::<f64, 3>::new(1..=3);
        let mut V3 = StackArray::<f64, 3>::new(1..=3);
        let mut VOL: f64 = 0.0;
        let mut VOUT = ActualArray::<f64>::new(LBCELL..=(3 * MAXV));
        let mut VOUT1 = ActualArray::<f64>::new(LBCELL..=(3 * MAXV));
        let mut VRTCES = ActualArray2D::<f64>::new(1..=3, 1..=MAXV);
        let mut XAREA: f64 = 0.0;
        let mut XNORML = StackArray::<f64, 3>::new(1..=3);
        let mut XVERTS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XVOL: f64 = 0.0;
        let mut NP: i32 = 0;
        let mut NV: i32 = 0;
        let mut PLATES = ActualArray2D::<i32>::new(1..=3, 1..=MAXP);
        let mut PLATS2 = ActualArray2D::<i32>::new(1..=3, 1..=MAXP);
        let mut POUT = ActualArray::<i32>::new(LBCELL..=(3 * MAXP));
        let mut POUT1 = ActualArray::<i32>::new(LBCELL..=(3 * MAXP));

        Self {
            TITLE,
            A,
            AREA,
            B,
            C,
            CENTER,
            DELTA,
            EDGE1,
            EDGE2,
            IVERTS,
            NORMAL,
            OFFSET,
            OVERTS,
            S,
            V1,
            V2,
            V3,
            VOL,
            VOUT,
            VOUT1,
            VRTCES,
            XAREA,
            XNORML,
            XVERTS,
            XVOL,
            NP,
            NV,
            PLATES,
            PLATS2,
            POUT,
            POUT1,
        }
    }
}

//$Procedure F_PLT ( Plate routine tests )
pub fn F_PLT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //
    //  DOUBLE PRECISION      TIGHT
    //  PARAMETER           ( TIGHT  = 1.D-12 )

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
    testutil::TOPEN(b"F_PLT", ctx)?;

    //**********************************************************************
    //
    //     PLTAR tests
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"PLTAR Error: too few plates");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::CLEARD((3 * MAXV), save.VRTCES.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARI((3 * MAXP), save.PLATES.as_slice_mut());

    save.NV = 3;
    save.NP = -1;

    save.AREA = spicelib::PLTAR(
        save.NV,
        save.VRTCES.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADPLATECOUNT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"PLTAR Error: too few vertices");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NV = 2;
    save.NP = 1;

    save.AREA = spicelib::PLTAR(
        save.NV,
        save.VRTCES.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(TOOFEWVERTICES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"PLTAR Error: bad vertex index in plate");

    testutil::TCASE(&save.TITLE, ctx)?;
    //
    // Initialize cells.
    //
    spicelib::SSIZED((3 * MAXV), save.VOUT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEI((3 * MAXP), save.POUT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a plate set for a box.
    //
    save.A = 10.0;
    save.B = 20.0;
    save.C = 30.0;

    testutil::ZZPSBOX(
        save.A,
        save.B,
        save.C,
        save.VOUT.as_slice_mut(),
        save.POUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NP = (spicelib::CARDI(save.POUT.as_slice(), ctx)? / 3);
    save.NV = (spicelib::CARDD(save.VOUT.as_slice(), ctx)? / 3);

    //
    // Create a bad plate: first vertex index is 0.
    //
    spicelib::MOVEI(
        save.POUT.subarray(1),
        (3 * save.NP),
        save.PLATS2.as_slice_mut(),
    );

    save.PLATS2[[1, 1]] = 0;

    save.AREA = spicelib::PLTAR(
        save.NV,
        save.VRTCES.as_slice(),
        save.NP,
        save.PLATS2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    //
    // Restore the original plate. It's simplest to copy the
    // plate set. Set another invalid vertex index.
    //
    spicelib::MOVEI(
        save.POUT.subarray(1),
        (3 * save.NP),
        save.PLATS2.as_slice_mut(),
    );

    save.PLATS2[[2, 1]] = 0;

    for I in 1..=save.NP {
        for J in 1..=3 {
            spicelib::MOVEI(
                save.POUT.subarray(1),
                (3 * save.NP),
                save.PLATS2.as_slice_mut(),
            );
            save.PLATS2[[J, I]] = 0;

            save.AREA = spicelib::PLTAR(
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATS2.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

            spicelib::MOVEI(
                save.POUT.subarray(1),
                (3 * save.NP),
                save.PLATS2.as_slice_mut(),
            );
            save.PLATS2[[J, I]] = (save.NV + 1);

            save.AREA = spicelib::PLTAR(
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATS2.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"PLTAR: Find area of a box centered at the origin.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::SSIZED((3 * MAXV), save.VOUT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEI((3 * MAXP), save.POUT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.A = 10.0;
    save.B = 20.0;
    save.C = 30.0;

    testutil::ZZPSBOX(
        save.A,
        save.B,
        save.C,
        save.VOUT.as_slice_mut(),
        save.POUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NV = (spicelib::CARDD(save.VOUT.as_slice(), ctx)? / 3);
    save.NP = (spicelib::CARDI(save.POUT.as_slice(), ctx)? / 3);

    save.AREA = spicelib::PLTAR(
        save.NV,
        save.VOUT.subarray(1),
        save.NP,
        save.POUT.subarray(1),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XAREA = ((2 as f64) * (((save.A * save.B) + (save.B * save.C)) + (save.A * save.C)));

    testutil::CHCKSD(b"AREA", save.AREA, b"~/", save.XAREA, VTIGHT, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"PLTAR: Find area of a box that excludes the origin.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::SSIZED((3 * MAXV), save.VOUT1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEI((3 * MAXP), save.POUT1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a vector by which to translate the vertices of the box.
    //
    spicelib::VPACK(100.0, 200.0, -400.0, save.OFFSET.as_slice_mut());

    //
    // Create a translated box. The plate set doesn't change.
    //
    testutil::ZZPSXLAT(
        save.VOUT.as_slice(),
        save.OFFSET.as_slice(),
        save.VOUT1.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compute the area of the translated box.
    //
    save.AREA = spicelib::PLTAR(
        save.NV,
        save.VOUT1.subarray(1),
        save.NP,
        save.POUT.subarray(1),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"AREA", save.AREA, b"~/", save.XAREA, VTIGHT, OK, ctx)?;

    //**********************************************************************
    //
    //     PLTVOL tests
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"PLTVOL Error: too few plates");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::CLEARD((3 * MAXV), save.VRTCES.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARI((3 * MAXP), save.PLATES.as_slice_mut());

    save.NV = 4;
    save.NP = 3;

    save.AREA = spicelib::PLTVOL(
        save.NV,
        save.VRTCES.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(TOOFEWPLATES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"PLTVOL Error: too few vertices");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NV = 3;
    save.NP = 4;

    save.AREA = spicelib::PLTVOL(
        save.NV,
        save.VRTCES.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(TOOFEWVERTICES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"PLVOL Error: bad vertex index in plate");

    testutil::TCASE(&save.TITLE, ctx)?;
    //
    // Initialize cells.
    //
    spicelib::SSIZED((3 * MAXV), save.VOUT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEI((3 * MAXP), save.POUT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a plate set for a box.
    //
    save.A = 10.0;
    save.B = 20.0;
    save.C = 30.0;

    testutil::ZZPSBOX(
        save.A,
        save.B,
        save.C,
        save.VOUT.as_slice_mut(),
        save.POUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NP = (spicelib::CARDI(save.POUT.as_slice(), ctx)? / 3);
    save.NV = (spicelib::CARDD(save.VOUT.as_slice(), ctx)? / 3);

    //
    // Create a bad plate: first vertex index is 0.
    //
    spicelib::MOVEI(
        save.POUT.subarray(1),
        (3 * save.NP),
        save.PLATS2.as_slice_mut(),
    );

    save.PLATS2[[1, 1]] = 0;

    save.VOL = spicelib::PLTVOL(
        save.NV,
        save.VRTCES.as_slice(),
        save.NP,
        save.PLATS2.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    //
    // Restore the original plate. It's simplest to copy the
    // plate set. Set another invalid vertex index.
    //
    spicelib::MOVEI(
        save.POUT.subarray(1),
        (3 * save.NP),
        save.PLATS2.as_slice_mut(),
    );

    save.PLATS2[[2, 1]] = 0;

    for I in 1..=save.NP {
        for J in 1..=3 {
            spicelib::MOVEI(
                save.POUT.subarray(1),
                (3 * save.NP),
                save.PLATS2.as_slice_mut(),
            );
            save.PLATS2[[J, I]] = 0;

            save.VOL = spicelib::PLTVOL(
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATS2.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

            spicelib::MOVEI(
                save.POUT.subarray(1),
                (3 * save.NP),
                save.PLATS2.as_slice_mut(),
            );
            save.PLATS2[[J, I]] = (save.NV + 1);

            save.VOL = spicelib::PLTVOL(
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATS2.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"PLTVOL: Find volume of a box centered at the origin.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::SSIZED((3 * MAXV), save.VOUT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEI((3 * MAXP), save.POUT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.A = 10.0;
    save.B = 20.0;
    save.C = 30.0;

    testutil::ZZPSBOX(
        save.A,
        save.B,
        save.C,
        save.VOUT.as_slice_mut(),
        save.POUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NV = (spicelib::CARDD(save.VOUT.as_slice(), ctx)? / 3);
    save.NP = (spicelib::CARDI(save.POUT.as_slice(), ctx)? / 3);

    save.VOL = spicelib::PLTVOL(
        save.NV,
        save.VOUT.subarray(1),
        save.NP,
        save.POUT.subarray(1),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XVOL = ((save.A * save.B) * save.C);

    testutil::CHCKSD(b"VOL", save.VOL, b"~/", save.XVOL, VTIGHT, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"PLTVOL: Find volume of a box that excludes the origin.",
    );

    spicelib::SSIZED((3 * MAXV), save.VOUT1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEI((3 * MAXP), save.POUT1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a vector by which to translate the vertices of the box.
    //
    spicelib::VPACK(100.0, 200.0, -400.0, save.OFFSET.as_slice_mut());

    //
    // Create a translated box. The plate set doesn't change.
    //
    testutil::ZZPSXLAT(
        save.VOUT.as_slice(),
        save.OFFSET.as_slice(),
        save.VOUT1.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compute the volume of the translated box.
    //
    save.VOL = spicelib::PLTVOL(
        save.NV,
        save.VOUT.subarray(1),
        save.NP,
        save.POUT.subarray(1),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XVOL = ((save.A * save.B) * save.C);

    testutil::CHCKSD(b"VOL", save.VOL, b"~/", save.XVOL, VTIGHT, OK, ctx)?;

    //**********************************************************************
    //
    //     PLTNRM tests
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"PLTNRM: Compute an upward normal of an equilateral triangle lying in the X-Y plane and centered at the origin.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.S = (f64::sqrt(3.0) / 2 as f64);

    spicelib::VPACK(save.S, -0.5, 0.0, save.V1.as_slice_mut());
    spicelib::VPACK(0.0, 1.0, 0.0, save.V2.as_slice_mut());
    spicelib::VPACK(-save.S, -0.5, 0.0, save.V3.as_slice_mut());

    spicelib::PLTNRM(
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
        save.NORMAL.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compute an expected normal vector.
    //
    spicelib::VSUB(
        save.V2.as_slice(),
        save.V1.as_slice(),
        save.EDGE1.as_slice_mut(),
    );
    spicelib::VSUB(
        save.V3.as_slice(),
        save.V2.as_slice(),
        save.EDGE2.as_slice_mut(),
    );

    spicelib::VCRSS(
        save.EDGE1.as_slice(),
        save.EDGE2.as_slice(),
        save.XNORML.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"NORMAL",
        save.NORMAL.as_slice(),
        b"~~/",
        save.XNORML.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Convert normal to unit length and compare to a known vector.
    //
    spicelib::VPACK(0.0, 0.0, 1.0, save.XNORML.as_slice_mut());

    spicelib::VHATIP(save.NORMAL.as_slice_mut());

    testutil::CHCKAD(
        b"NORMAL (unit)",
        save.NORMAL.as_slice(),
        b"~~/",
        save.XNORML.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //**********************************************************************
    //
    //     PLTEXP tests
    //
    //**********************************************************************

    fstr::assign(
        &mut save.TITLE,
        b"PLTEXP: expand a plate that is parallel to, but not contained in, the X-Y plane.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(-1.0, -1.0, 3.0, save.IVERTS.subarray_mut([1, 1]));
    spicelib::VPACK(2.0, -1.0, 3.0, save.IVERTS.subarray_mut([1, 2]));
    spicelib::VPACK(2.0, 4.0, 3.0, save.IVERTS.subarray_mut([1, 3]));

    save.DELTA = 1.0;

    spicelib::PLTEXP(
        save.IVERTS.as_slice(),
        save.DELTA,
        save.OVERTS.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Prepare the expected output.
    //
    save.S = (1.0 / 3.0);

    spicelib::VLCOM3(
        save.S,
        save.IVERTS.subarray([1, 1]),
        save.S,
        save.IVERTS.subarray([1, 2]),
        save.S,
        save.IVERTS.subarray([1, 3]),
        save.CENTER.as_slice_mut(),
    );

    for I in 1..=3 {
        spicelib::VSUB(
            save.IVERTS.subarray([1, I]),
            save.CENTER.as_slice(),
            save.OFFSET.as_slice_mut(),
        );

        spicelib::VLCOM(
            1.0,
            save.CENTER.as_slice(),
            2.0,
            save.OFFSET.as_slice(),
            save.XVERTS.subarray_mut([1, I]),
        );
    }

    testutil::CHCKAD(
        b"OVERTS",
        save.OVERTS.as_slice(),
        b"~~/",
        save.XVERTS.as_slice(),
        9,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
