//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VTIGHT: f64 = 0.00000000000001;
const LBLSIZ: i32 = 50;
const LNSIZE: i32 = 320;

struct SaveVars {
    LABEL: Vec<u8>,
    TITLE: Vec<u8>,
    BIGRAD: f64,
    BOXORI: StackArray<f64, 3>,
    CENTER: StackArray<f64, 3>,
    DELTA: StackArray<f64, 3>,
    EXTENT: StackArray<f64, 3>,
    MEDRAD: f64,
    RAYDIR: StackArray<f64, 3>,
    S: f64,
    SMLRAD: f64,
    TARG: StackArray<f64, 3>,
    TOL: f64,
    UDIR: StackArray<f64, 3>,
    VERTEX: StackArray<f64, 3>,
    XPT: StackArray<f64, 3>,
    XXPT: StackArray<f64, 3>,
    HCIDX1: i32,
    HCIDX2: i32,
    NEXT: StackArray<i32, 3>,
    NSTEP: i32,
    FOUND: bool,
    OUT1: bool,
    OUT2: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LABEL = vec![b' '; LBLSIZ as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut BIGRAD: f64 = 0.0;
        let mut BOXORI = StackArray::<f64, 3>::new(1..=3);
        let mut CENTER = StackArray::<f64, 3>::new(1..=3);
        let mut DELTA = StackArray::<f64, 3>::new(1..=3);
        let mut EXTENT = StackArray::<f64, 3>::new(1..=3);
        let mut MEDRAD: f64 = 0.0;
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut S: f64 = 0.0;
        let mut SMLRAD: f64 = 0.0;
        let mut TARG = StackArray::<f64, 3>::new(1..=3);
        let mut TOL: f64 = 0.0;
        let mut UDIR = StackArray::<f64, 3>::new(1..=3);
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut XXPT = StackArray::<f64, 3>::new(1..=3);
        let mut HCIDX1: i32 = 0;
        let mut HCIDX2: i32 = 0;
        let mut NEXT = StackArray::<i32, 3>::new(1..=3);
        let mut NSTEP: i32 = 0;
        let mut FOUND: bool = false;
        let mut OUT1: bool = false;
        let mut OUT2: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(2), Val::I(3), Val::I(1)].into_iter();
            NEXT.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            LABEL,
            TITLE,
            BIGRAD,
            BOXORI,
            CENTER,
            DELTA,
            EXTENT,
            MEDRAD,
            RAYDIR,
            S,
            SMLRAD,
            TARG,
            TOL,
            UDIR,
            VERTEX,
            XPT,
            XXPT,
            HCIDX1,
            HCIDX2,
            NEXT,
            NSTEP,
            FOUND,
            OUT1,
            OUT2,
        }
    }
}

//$Procedure F_ZZRAYBOX ( ray-box intercept tests )
pub fn F_ZZRAYBOX(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //
    //  DOUBLE PRECISION      VDIST

    //
    // Local Parameters
    //

    // DOUBLE PRECISION      TIGHT
    // PARAMETER           ( TIGHT  = 1.D-12 )

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
    testutil::TOPEN(b"F_ZZRAYBOX", ctx)?;

    //**********************************************************************
    //
    //     Error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Ray direction is zero vector.", ctx)?;

    spicelib::VPACK(1.0, 2.0, 3.0, save.BOXORI.as_slice_mut());
    spicelib::VPACK(30.0, 20.0, 10.0, save.EXTENT.as_slice_mut());

    spicelib::VPACK(1000.0, 1000.0, 1000.0, save.VERTEX.as_slice_mut());

    spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());

    spicelib::ZZRAYBOX(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOXORI.as_slice(),
        save.EXTENT.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-positive extents", ctx)?;

    spicelib::VPACK(1.0, 2.0, 3.0, save.BOXORI.as_slice_mut());
    spicelib::VPACK(1000.0, 1000.0, 1000.0, save.VERTEX.as_slice_mut());

    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::VPACK(0.0, 20.0, 10.0, save.EXTENT.as_slice_mut());

    spicelib::ZZRAYBOX(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOXORI.as_slice(),
        save.EXTENT.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::VPACK(-1.0, 20.0, 10.0, save.EXTENT.as_slice_mut());

    spicelib::ZZRAYBOX(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOXORI.as_slice(),
        save.EXTENT.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::VPACK(10.0, 0.0, 10.0, save.EXTENT.as_slice_mut());

    spicelib::ZZRAYBOX(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOXORI.as_slice(),
        save.EXTENT.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::VPACK(10.0, -1.0, 10.0, save.EXTENT.as_slice_mut());

    spicelib::ZZRAYBOX(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOXORI.as_slice(),
        save.EXTENT.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::VPACK(10.0, 20.0, 0.0, save.EXTENT.as_slice_mut());

    spicelib::ZZRAYBOX(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOXORI.as_slice(),
        save.EXTENT.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::VPACK(10.0, 20.0, -1.0, save.EXTENT.as_slice_mut());

    spicelib::ZZRAYBOX(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOXORI.as_slice(),
        save.EXTENT.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Vertex is inside box.", ctx)?;

    spicelib::VPACK(1.0, 2.0, 3.0, save.BOXORI.as_slice_mut());
    spicelib::VPACK(30.0, 20.0, 10.0, save.EXTENT.as_slice_mut());

    spicelib::VPACK(15.0, 10.0, 5.0, save.VERTEX.as_slice_mut());
    spicelib::VEQU(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRAYBOX(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOXORI.as_slice(),
        save.EXTENT.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    if save.FOUND {
        testutil::CHCKAD(
            b"XPT",
            save.XPT.as_slice(),
            b"=",
            save.VERTEX.as_slice(),
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
        b"Simple non-intersection case. Vertices are outside the bounding sphere.",
        ctx,
    )?;

    //
    // The ray points directly away from the box in each case.
    //
    spicelib::VPACK(1.0, 2.0, 3.0, save.BOXORI.as_slice_mut());
    spicelib::VPACK(30.0, 20.0, 10.0, save.EXTENT.as_slice_mut());

    spicelib::VLCOM(
        1.0,
        save.BOXORI.as_slice(),
        0.5,
        save.EXTENT.as_slice(),
        save.CENTER.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(15.0, 10.0, 5.0, save.VERTEX.as_slice_mut());

    for I in 1..=3 {
        for J in 1..=2 {
            //
            // S is 1 for J = 1; -1 for J = 2.
            //
            save.S = ((2 * J) - 3) as f64;

            spicelib::VEQU(save.CENTER.as_slice(), save.VERTEX.as_slice_mut());
            save.VERTEX[I] = (save.CENTER[I] + ((save.S * save.EXTENT[I]) * 3 as f64));

            spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());
            save.RAYDIR[I] = save.S;

            spicelib::ZZRAYBOX(
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.BOXORI.as_slice(),
                save.EXTENT.as_slice(),
                save.XPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"FOUND # #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKSL(&save.LABEL, save.FOUND, false, OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Simple non-intersection case. Vertices are inside the bounding sphere.",
        ctx,
    )?;

    //
    // The ray points directly away from the box in each case.
    //
    spicelib::VPACK(1.0, 2.0, 3.0, save.BOXORI.as_slice_mut());
    spicelib::VPACK(30.0, 20.0, 10.0, save.EXTENT.as_slice_mut());

    spicelib::VLCOM(
        1.0,
        save.BOXORI.as_slice(),
        0.5,
        save.EXTENT.as_slice(),
        save.CENTER.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(15.0, 10.0, 5.0, save.VERTEX.as_slice_mut());
    spicelib::VEQU(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    for I in 1..=3 {
        for J in 1..=2 {
            //
            // S is 1 for J = 1; -1 for J = 2.
            //
            save.S = ((2 * J) - 3) as f64;

            spicelib::VEQU(save.CENTER.as_slice(), save.VERTEX.as_slice_mut());
            save.VERTEX[I] = (save.CENTER[I] + ((save.S * save.EXTENT[I]) * 0.500001));

            spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());
            save.RAYDIR[I] = save.S;

            spicelib::ZZRAYBOX(
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.BOXORI.as_slice(),
                save.EXTENT.as_slice(),
                save.XPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"FOUND # #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKSL(&save.LABEL, save.FOUND, false, OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Simple intersection case", ctx)?;

    //
    // The ray points directly toward from the box in each case.
    //
    spicelib::VPACK(1.0, 2.0, 3.0, save.BOXORI.as_slice_mut());
    spicelib::VPACK(30.0, 20.0, 10.0, save.EXTENT.as_slice_mut());

    spicelib::VLCOM(
        1.0,
        save.BOXORI.as_slice(),
        0.5,
        save.EXTENT.as_slice(),
        save.CENTER.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=3 {
        for J in 1..=2 {
            //
            // S is -1 for J = 1; 1 for J = 2.
            //
            save.S = ((2 * J) - 3) as f64;

            //
            // Select a vertex outside the bounding sphere.
            //
            spicelib::VEQU(save.CENTER.as_slice(), save.VERTEX.as_slice_mut());
            save.VERTEX[I] = (save.CENTER[I] + ((save.S * save.EXTENT[I]) * 3 as f64));

            spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());
            save.RAYDIR[I] = -save.S;

            spicelib::ZZRAYBOX(
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.BOXORI.as_slice(),
                save.EXTENT.as_slice(),
                save.XPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"FOUND # #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                spicelib::VEQU(save.CENTER.as_slice(), save.XXPT.as_slice_mut());
                save.XXPT[I] = (save.CENTER[I] + ((save.S * save.EXTENT[I]) / 2 as f64));

                fstr::assign(&mut save.LABEL, b"XPT # #");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);

                save.TOL = VTIGHT;

                testutil::CHCKAD(
                    &save.LABEL,
                    save.XPT.as_slice(),
                    b"~~/",
                    save.XXPT.as_slice(),
                    3,
                    save.TOL,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Simple intersection case; vertex is inside the bounding sphere.",
        ctx,
    )?;

    //
    // The ray points directly toward from the box in each case.
    //
    spicelib::VPACK(1.0, 2.0, 3.0, save.BOXORI.as_slice_mut());
    spicelib::VPACK(30.0, 20.0, 10.0, save.EXTENT.as_slice_mut());

    spicelib::VLCOM(
        1.0,
        save.BOXORI.as_slice(),
        0.5,
        save.EXTENT.as_slice(),
        save.CENTER.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=3 {
        for J in 1..=2 {
            //
            // S is -1 for J = 1; 1 for J = 2.
            //
            save.S = ((2 * J) - 3) as f64;

            //
            // Select a vertex outside the bounding sphere.
            //
            spicelib::VEQU(save.CENTER.as_slice(), save.VERTEX.as_slice_mut());
            save.VERTEX[I] = (save.CENTER[I] + ((save.S * save.EXTENT[I]) * 0.50001));

            spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());
            save.RAYDIR[I] = -save.S;

            spicelib::ZZRAYBOX(
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.BOXORI.as_slice(),
                save.EXTENT.as_slice(),
                save.XPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"FOUND # #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                spicelib::VEQU(save.CENTER.as_slice(), save.XXPT.as_slice_mut());
                save.XXPT[I] = (save.CENTER[I] + ((save.S * save.EXTENT[I]) / 2 as f64));

                fstr::assign(&mut save.LABEL, b"XPT # #");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);

                save.TOL = VTIGHT;

                testutil::CHCKAD(
                    &save.LABEL,
                    save.XPT.as_slice(),
                    b"~~/",
                    save.XXPT.as_slice(),
                    3,
                    save.TOL,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     A series of detailed intersection tests follows. Vertices
    //     will be generated for a set of directions spanning the
    //     unit sphere centered at the box center. All rays are
    //     pointed at specific points on the box's surface. The
    //     vertices are outside the bounding sphere but not at
    //     a very great distance from it.
    //
    //     MEDRAD is the vertex distance from the center.
    //
    save.MEDRAD = 100.0;

    spicelib::VPACK(1.0, 2.0, 3.0, save.BOXORI.as_slice_mut());

    spicelib::VPACK(30.0, 20.0, 10.0, save.EXTENT.as_slice_mut());

    spicelib::VLCOM(
        1.0,
        save.BOXORI.as_slice(),
        0.5,
        save.EXTENT.as_slice(),
        save.CENTER.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compute steps separating target points.
    //
    save.NSTEP = 20;

    for I in 1..=3 {
        save.DELTA[I] = (save.EXTENT[I] / save.NSTEP as f64);
    }

    //
    // Loop over all coordinates and both faces for each
    // coordinate.
    //
    for I in 1..=3 {
        //
        // Set the indices of the "horizontal" coordinates;
        // these are the coordinate on the axes orthogonal
        // to the Ith axis.
        //
        save.HCIDX1 = save.NEXT[I];
        save.HCIDX2 = save.NEXT[save.HCIDX1];

        for J in 1..=2 {
            //
            // S is -1 for J = 1; 1 for J = 2.
            //
            save.S = ((2 * J) - 3) as f64;
            //
            // Loop over the horizontal coordinates; generate
            // target points.
            //
            save.XXPT[I] = (save.CENTER[I] + ((save.S * save.EXTENT[I]) / 2 as f64));

            for H1 in 0..=save.NSTEP {
                save.XXPT[save.HCIDX1] = ((save.CENTER[save.HCIDX1]
                    - (save.EXTENT[save.HCIDX1] / 2 as f64))
                    + (save.DELTA[save.HCIDX1] * H1 as f64));

                for H2 in 0..=save.NSTEP {
                    save.XXPT[save.HCIDX2] = ((save.CENTER[save.HCIDX2]
                        - (save.EXTENT[save.HCIDX2] / 2 as f64))
                        + (save.DELTA[save.HCIDX2] * H2 as f64));

                    //
                    // --- Case: ------------------------------------------------------
                    //

                    fstr::assign(
                        &mut save.TITLE,
                        b"Medium range intercept: I =  #; J = #; H1 =  #; H2 = #",
                    );
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", H1, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", H2, &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::TCASE(&save.TITLE, ctx)?;
                    //
                    // Generate the ray's vertex for this case.
                    // The vertex lies along a ray emanating from
                    // the box's center. The distance of the vertex
                    // from the center is MEDRAD.
                    //
                    spicelib::VSUB(
                        save.CENTER.as_slice(),
                        save.XXPT.as_slice(),
                        save.RAYDIR.as_slice_mut(),
                    );
                    spicelib::VHAT(save.RAYDIR.as_slice(), save.UDIR.as_slice_mut());
                    spicelib::VLCOM(
                        1.0,
                        save.CENTER.as_slice(),
                        -save.MEDRAD,
                        save.UDIR.as_slice(),
                        save.VERTEX.as_slice_mut(),
                    );

                    spicelib::ZZRAYBOX(
                        save.VERTEX.as_slice(),
                        save.RAYDIR.as_slice(),
                        save.BOXORI.as_slice(),
                        save.EXTENT.as_slice(),
                        save.XPT.as_slice_mut(),
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.LABEL, b"FOUND # # # #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", H1, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", H2, &mut save.LABEL, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;

                    if save.FOUND {
                        fstr::assign(&mut save.LABEL, b"XPT # #");
                        spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                        spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
                        spicelib::REPMI(&save.LABEL.to_vec(), b"#", H1, &mut save.LABEL, ctx);
                        spicelib::REPMI(&save.LABEL.to_vec(), b"#", H2, &mut save.LABEL, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        save.TOL = VTIGHT;

                        testutil::CHCKAD(
                            &save.LABEL,
                            save.XPT.as_slice(),
                            b"~~/",
                            save.XXPT.as_slice(),
                            3,
                            save.TOL,
                            OK,
                            ctx,
                        )?;
                    }
                }
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    //     Repeat the previous tests but use a very large vertex
    //     distance. We must reduce the relative error tolerance
    //     to a value suitable for the large expected errors in the
    //     ray-bounding sphere intercept.
    //
    save.BIGRAD = 100000000000000.0;

    save.TOL = ((VTIGHT * save.BIGRAD) / 10 as f64);

    //
    // Compute steps separating target points.
    //
    save.NSTEP = 20;

    for I in 1..=3 {
        save.DELTA[I] = (save.EXTENT[I] / save.NSTEP as f64);
    }

    //
    // Loop over all coordinates and both faces for each
    // coordinate.
    //
    for I in 1..=3 {
        //
        // Set the indices of the "horizontal" coordinates;
        // these are the coordinate on the axes orthogonal
        // to the Ith axis.
        //
        save.HCIDX1 = save.NEXT[I];
        save.HCIDX2 = save.NEXT[save.HCIDX1];

        for J in 1..=2 {
            //
            // S is -1 for J = 1; 1 for J = 2.
            //
            save.S = ((2 * J) - 3) as f64;
            //
            // Loop over the horizontal coordinates; generate
            // target points.
            //
            save.XXPT[I] = (save.CENTER[I] + ((save.S * save.EXTENT[I]) / 2 as f64));

            for H1 in 0..=save.NSTEP {
                save.XXPT[save.HCIDX1] = ((save.CENTER[save.HCIDX1]
                    - (save.EXTENT[save.HCIDX1] / 2 as f64))
                    + (save.DELTA[save.HCIDX1] * H1 as f64));

                for H2 in 0..=save.NSTEP {
                    save.XXPT[save.HCIDX2] = ((save.CENTER[save.HCIDX2]
                        - (save.EXTENT[save.HCIDX2] / 2 as f64))
                        + (save.DELTA[save.HCIDX2] * H2 as f64));

                    //
                    // --- Case: ------------------------------------------------------
                    //

                    fstr::assign(
                        &mut save.TITLE,
                        b"Long range intercept: I =  #; J = #; H1 =  #; H2 = #",
                    );
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", H1, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", H2, &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::TCASE(&save.TITLE, ctx)?;
                    //
                    // Generate the ray's vertex for this case.
                    // The vertex lies along a ray emanating from
                    // the box's center. The distance of the vertex
                    // from the center is BIGRAD.
                    //
                    spicelib::VSUB(
                        save.CENTER.as_slice(),
                        save.XXPT.as_slice(),
                        save.RAYDIR.as_slice_mut(),
                    );
                    spicelib::VHAT(save.RAYDIR.as_slice(), save.UDIR.as_slice_mut());
                    spicelib::VLCOM(
                        1.0,
                        save.CENTER.as_slice(),
                        -save.BIGRAD,
                        save.UDIR.as_slice(),
                        save.VERTEX.as_slice_mut(),
                    );

                    spicelib::ZZRAYBOX(
                        save.VERTEX.as_slice(),
                        save.RAYDIR.as_slice(),
                        save.BOXORI.as_slice(),
                        save.EXTENT.as_slice(),
                        save.XPT.as_slice_mut(),
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.LABEL, b"FOUND # # # #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", H1, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", H2, &mut save.LABEL, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;

                    if save.FOUND {
                        fstr::assign(&mut save.LABEL, b"XPT # #");
                        spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                        spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
                        spicelib::REPMI(&save.LABEL.to_vec(), b"#", H1, &mut save.LABEL, ctx);
                        spicelib::REPMI(&save.LABEL.to_vec(), b"#", H2, &mut save.LABEL, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKAD(
                            &save.LABEL,
                            save.XPT.as_slice(),
                            b"~~/",
                            save.XXPT.as_slice(),
                            3,
                            save.TOL,
                            OK,
                            ctx,
                        )?;
                    }
                }
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    //     Repeat the previous tests but use vertices slightly outside
    //     the box.
    //
    save.SMLRAD = 0.001;

    save.TOL = VTIGHT;

    //
    // Compute steps separating target points.
    //
    save.NSTEP = 20;

    for I in 1..=3 {
        save.DELTA[I] = (save.EXTENT[I] / save.NSTEP as f64);
    }

    //
    // Loop over all coordinates and both faces for each
    // coordinate.
    //
    for I in 1..=3 {
        //
        // Set the indices of the "horizontal" coordinates;
        // these are the coordinate on the axes orthogonal
        // to the Ith axis.
        //
        save.HCIDX1 = save.NEXT[I];
        save.HCIDX2 = save.NEXT[save.HCIDX1];

        for J in 1..=2 {
            //
            // S is -1 for J = 1; 1 for J = 2.
            //
            save.S = ((2 * J) - 3) as f64;
            //
            // Loop over the horizontal coordinates; generate
            // target points.
            //
            save.XXPT[I] = (save.CENTER[I] + ((save.S * save.EXTENT[I]) / 2 as f64));

            for H1 in 0..=save.NSTEP {
                save.XXPT[save.HCIDX1] = ((save.CENTER[save.HCIDX1]
                    - (save.EXTENT[save.HCIDX1] / 2 as f64))
                    + (save.DELTA[save.HCIDX1] * H1 as f64));

                for H2 in 0..=save.NSTEP {
                    save.XXPT[save.HCIDX2] = ((save.CENTER[save.HCIDX2]
                        - (save.EXTENT[save.HCIDX2] / 2 as f64))
                        + (save.DELTA[save.HCIDX2] * H2 as f64));

                    //
                    // --- Case: ------------------------------------------------------
                    //

                    fstr::assign(
                        &mut save.TITLE,
                        b"short range intercept: I =  #; J = #; H1 =  #; H2 = #",
                    );
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", H1, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", H2, &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::TCASE(&save.TITLE, ctx)?;
                    //
                    // Generate the ray's vertex for this case.
                    // The vertex lies along a ray emanating from
                    // the box's center. The distance of the vertex
                    // from the expected intercept (NOT the center)
                    // is SMLRAD.
                    //
                    spicelib::VSUB(
                        save.CENTER.as_slice(),
                        save.XXPT.as_slice(),
                        save.RAYDIR.as_slice_mut(),
                    );
                    spicelib::VHAT(save.RAYDIR.as_slice(), save.UDIR.as_slice_mut());
                    spicelib::VLCOM(
                        1.0,
                        save.XXPT.as_slice(),
                        -save.SMLRAD,
                        save.UDIR.as_slice(),
                        save.VERTEX.as_slice_mut(),
                    );

                    spicelib::ZZRAYBOX(
                        save.VERTEX.as_slice(),
                        save.RAYDIR.as_slice(),
                        save.BOXORI.as_slice(),
                        save.EXTENT.as_slice(),
                        save.XPT.as_slice_mut(),
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.LABEL, b"FOUND # # # #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", H1, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", H2, &mut save.LABEL, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;

                    if save.FOUND {
                        fstr::assign(&mut save.LABEL, b"XPT # #");
                        spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                        spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
                        spicelib::REPMI(&save.LABEL.to_vec(), b"#", H1, &mut save.LABEL, ctx);
                        spicelib::REPMI(&save.LABEL.to_vec(), b"#", H2, &mut save.LABEL, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKAD(
                            &save.LABEL,
                            save.XPT.as_slice(),
                            b"~~/",
                            save.XXPT.as_slice(),
                            3,
                            save.TOL,
                            OK,
                            ctx,
                        )?;
                    }
                }
            }
        }
    }

    //**********************************************************************
    //
    //     Normal non-intersection cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     A series of detailed non-intersection tests follows.
    //

    //
    // Compute steps separating target points.
    //
    save.NSTEP = 8;

    for I in 1..=3 {
        //
        // Note that the increments are based on twice the
        // box extents.
        //
        save.DELTA[I] = (((2 as f64) * save.EXTENT[I]) / save.NSTEP as f64);
    }

    //
    // Generate target points that the rays will pass through.
    //
    // Loop over all coordinates and both faces for each
    // coordinate.
    //
    for I in 1..=3 {
        //
        // Set the indices of the "horizontal" coordinates;
        // these are the coordinate on the axes orthogonal
        // to the Ith axis.
        //
        save.HCIDX1 = save.NEXT[I];
        save.HCIDX2 = save.NEXT[save.HCIDX1];

        for J in 1..=2 {
            //
            // S is -1 for J = 1; 1 for J = 2.
            //
            save.S = ((2 * J) - 3) as f64;
            //
            // Loop over the horizontal coordinates; generate
            // target points. The target plane is on the far side
            // of the box, relative to the Ith coordinate of the
            // vertex.
            //
            save.TARG[I] = (save.CENTER[I] - ((save.S * save.EXTENT[I]) / 2 as f64));

            for H1 in 0..=save.NSTEP {
                save.TARG[save.HCIDX1] = ((save.CENTER[save.HCIDX1] - save.EXTENT[save.HCIDX1])
                    + (save.DELTA[save.HCIDX1] * H1 as f64));

                for H2 in 0..=save.NSTEP {
                    save.TARG[save.HCIDX2] = ((save.CENTER[save.HCIDX2]
                        - save.EXTENT[save.HCIDX2])
                        + (save.DELTA[save.HCIDX2] * H2 as f64));

                    //
                    // Use only those target points that lie outside
                    // of the horizontal bounds of the box.
                    //
                    save.OUT1 = (f64::abs((save.TARG[save.HCIDX1] - save.CENTER[save.HCIDX1]))
                        > (save.EXTENT[save.HCIDX1] / 2 as f64));

                    save.OUT2 = (f64::abs((save.TARG[save.HCIDX2] - save.CENTER[save.HCIDX2]))
                        > (save.EXTENT[save.HCIDX2] / 2 as f64));

                    if (save.OUT1 || save.OUT2) {
                        //
                        // Generate vertices. We'll place the vertices
                        // far enough from the box that no "outside"
                        // target point is obscured by the box.
                        //
                        spicelib::CLEARD(3, save.VERTEX.as_slice_mut());

                        save.VERTEX[I] = (1000.0 * save.S);

                        for V1 in 0..=save.NSTEP {
                            save.VERTEX[save.HCIDX1] = (save.CENTER[save.HCIDX1]
                                + ((V1 as f64) * save.DELTA[save.HCIDX1]));

                            for V2 in 0..=save.NSTEP {
                                save.VERTEX[save.HCIDX2] = (save.CENTER[save.HCIDX2]
                                    + ((V2 as f64) * save.DELTA[save.HCIDX2]));

                                //
                                // --- Case: ------------------------------------------------------
                                //

                                fstr::assign(&mut save.TITLE, b"non-intercept: I =  #; J = #; H1 =  #; H2 = #; V1 = #; V2 = #");

                                spicelib::REPMI(
                                    &save.TITLE.to_vec(),
                                    b"#",
                                    I,
                                    &mut save.TITLE,
                                    ctx,
                                );
                                spicelib::REPMI(
                                    &save.TITLE.to_vec(),
                                    b"#",
                                    J,
                                    &mut save.TITLE,
                                    ctx,
                                );
                                spicelib::REPMI(
                                    &save.TITLE.to_vec(),
                                    b"#",
                                    H1,
                                    &mut save.TITLE,
                                    ctx,
                                );
                                spicelib::REPMI(
                                    &save.TITLE.to_vec(),
                                    b"#",
                                    H2,
                                    &mut save.TITLE,
                                    ctx,
                                );
                                spicelib::REPMI(
                                    &save.TITLE.to_vec(),
                                    b"#",
                                    V1,
                                    &mut save.TITLE,
                                    ctx,
                                );
                                spicelib::REPMI(
                                    &save.TITLE.to_vec(),
                                    b"#",
                                    V2,
                                    &mut save.TITLE,
                                    ctx,
                                );
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                testutil::TCASE(&save.TITLE, ctx)?;

                                //
                                // The ray points from the vertex toward
                                // the target on the far boundary plane.
                                //
                                spicelib::VSUB(
                                    save.TARG.as_slice(),
                                    save.VERTEX.as_slice(),
                                    save.RAYDIR.as_slice_mut(),
                                );

                                spicelib::ZZRAYBOX(
                                    save.VERTEX.as_slice(),
                                    save.RAYDIR.as_slice(),
                                    save.BOXORI.as_slice(),
                                    save.EXTENT.as_slice(),
                                    save.XPT.as_slice_mut(),
                                    &mut save.FOUND,
                                    ctx,
                                )?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                fstr::assign(&mut save.LABEL, b"FOUND # # # # # #");
                                spicelib::REPMI(
                                    &save.LABEL.to_vec(),
                                    b"#",
                                    I,
                                    &mut save.LABEL,
                                    ctx,
                                );
                                spicelib::REPMI(
                                    &save.LABEL.to_vec(),
                                    b"#",
                                    J,
                                    &mut save.LABEL,
                                    ctx,
                                );
                                spicelib::REPMI(
                                    &save.LABEL.to_vec(),
                                    b"#",
                                    H1,
                                    &mut save.LABEL,
                                    ctx,
                                );
                                spicelib::REPMI(
                                    &save.LABEL.to_vec(),
                                    b"#",
                                    H2,
                                    &mut save.LABEL,
                                    ctx,
                                );
                                spicelib::REPMI(
                                    &save.LABEL.to_vec(),
                                    b"#",
                                    V1,
                                    &mut save.LABEL,
                                    ctx,
                                );
                                spicelib::REPMI(
                                    &save.LABEL.to_vec(),
                                    b"#",
                                    V2,
                                    &mut save.LABEL,
                                    ctx,
                                );
                                testutil::CHCKXC(false, b" ", OK, ctx)?;
                                //
                                // No intercept should be found.
                                //
                                testutil::CHCKSL(&save.LABEL, save.FOUND, false, OK, ctx)?;
                            }
                        }
                    }
                }
            }
        }
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
