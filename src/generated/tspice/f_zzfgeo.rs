//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.00000000001;
const VTIGHT: f64 = 0.0000000000001;
const UBPL: i32 = 4;
const TITLEN: i32 = 240;

struct SaveVars {
    TITLE: Vec<u8>,
    CONST: f64,
    DIR: StackArray<f64, 3>,
    PLANE: StackArray<f64, 4>,
    MAXD: f64,
    NORMAL: StackArray<f64, 3>,
    R: f64,
    SCALE: f64,
    UDIR: StackArray<f64, 3>,
    UNORML: StackArray<f64, 3>,
    VERTEX: StackArray<f64, 3>,
    XPT: StackArray<f64, 3>,
    XXPT: StackArray<f64, 3>,
    NCASE: i32,
    NXPTS: i32,
    SEED: i32,
    XNXPTS: i32,
    FOUND: bool,
    XFOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; TITLEN as usize];
        let mut CONST: f64 = 0.0;
        let mut DIR = StackArray::<f64, 3>::new(1..=3);
        let mut PLANE = StackArray::<f64, 4>::new(1..=UBPL);
        let mut MAXD: f64 = 0.0;
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut R: f64 = 0.0;
        let mut SCALE: f64 = 0.0;
        let mut UDIR = StackArray::<f64, 3>::new(1..=3);
        let mut UNORML = StackArray::<f64, 3>::new(1..=3);
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut XXPT = StackArray::<f64, 3>::new(1..=3);
        let mut NCASE: i32 = 0;
        let mut NXPTS: i32 = 0;
        let mut SEED: i32 = 0;
        let mut XNXPTS: i32 = 0;
        let mut FOUND: bool = false;
        let mut XFOUND: bool = false;

        Self {
            TITLE,
            CONST,
            DIR,
            PLANE,
            MAXD,
            NORMAL,
            R,
            SCALE,
            UDIR,
            UNORML,
            VERTEX,
            XPT,
            XXPT,
            NCASE,
            NXPTS,
            SEED,
            XNXPTS,
            FOUND,
            XFOUND,
        }
    }
}

//$Procedure      F_ZZFGEO ( Test private "fast" geometry routines )
pub fn F_ZZFGEO(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

    //
    // SPICELIB functions
    //
    // DOUBLE PRECISION      VNORM

    //
    // Other functions
    //

    //
    // Local parameters
    //

    // INTEGER               LNSIZE
    // PARAMETER           ( LNSIZE = 80 )

    //
    // Local Variables
    //
    //  CHARACTER*(LNSIZE)    LABEL

    // INTEGER               I

    //
    // Save all variables to avoid stack problems on some
    // platforms.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZFGEO", ctx)?;

    //**********************************************************************
    //
    //     ZZINRYPL tests
    //
    //**********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZINRYPL: basic test with plane parallel to X-Y axes.",
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.NORMAL.as_slice_mut());

    save.CONST = 1.0;

    spicelib::VPACK(2.0, 4.0, 2.0, save.VERTEX.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, -1.0, save.DIR.as_slice_mut());

    spicelib::VHAT(save.DIR.as_slice(), save.UDIR.as_slice_mut());

    //
    // Compute expected result.
    //
    spicelib::NVC2PL(
        save.NORMAL.as_slice(),
        save.CONST,
        save.PLANE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::INRYPL(
        save.VERTEX.as_slice(),
        save.UDIR.as_slice(),
        save.PLANE.as_slice(),
        &mut save.XNXPTS,
        save.XXPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"ZNXPTS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute result using ZZINRYPL.
    //
    save.MAXD = spicelib::DPMAX();

    spicelib::ZZINRYPL(
        save.VERTEX.as_slice(),
        save.UDIR.as_slice(),
        save.NORMAL.as_slice(),
        save.CONST,
        save.MAXD,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    if *OK {
        testutil::CHCKAD(
            b"XPT",
            save.XPT.as_slice(),
            b"~~/",
            save.XXPT.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"ZZINRYPL: ray lies in plane.", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.NORMAL.as_slice_mut());

    save.CONST = 0.0;

    spicelib::VPACK(2.0, 4.0, 0.0, save.VERTEX.as_slice_mut());

    spicelib::VPACK(-1.0, 0.0, 0.0, save.DIR.as_slice_mut());

    spicelib::VHAT(save.DIR.as_slice(), save.UDIR.as_slice_mut());

    //
    // Compute result using ZZINRYPL.
    //
    save.MAXD = spicelib::DPMAX();

    spicelib::ZZINRYPL(
        save.VERTEX.as_slice(),
        save.UDIR.as_slice(),
        save.NORMAL.as_slice(),
        save.CONST,
        save.MAXD,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // This is a non-intersection case for ZZINRYPL.
    //
    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"ZZINRYPL: ray vertex only lies in plane.", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.NORMAL.as_slice_mut());

    save.CONST = 0.0;

    spicelib::VPACK(2.0, 4.0, 0.0, save.VERTEX.as_slice_mut());

    spicelib::VPACK(-1.0, 0.0, -1.0, save.DIR.as_slice_mut());

    spicelib::VHAT(save.DIR.as_slice(), save.UDIR.as_slice_mut());

    //
    // Compute result using ZZINRYPL.
    //
    save.MAXD = spicelib::DPMAX();

    spicelib::ZZINRYPL(
        save.VERTEX.as_slice(),
        save.UDIR.as_slice(),
        save.NORMAL.as_slice(),
        save.CONST,
        save.MAXD,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // This is an intersection case for ZZINRYPL.
    //
    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    if *OK {
        testutil::CHCKAD(
            b"XPT",
            save.XPT.as_slice(),
            b"~~/",
            save.VERTEX.as_slice(),
            3,
            TIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //

    //
    // ZZINRYPL random tests
    //

    save.SEED = -1;

    save.NCASE = 10000;

    for CASENO in 1..=save.NCASE {
        save.SCALE = f64::powf(10.0, testutil::T_RANDD(-10.0, 10.0, &mut save.SEED, ctx)?);

        save.NORMAL[1] = testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)?;
        save.NORMAL[2] = testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)?;
        save.NORMAL[3] = testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)?;

        spicelib::VHAT(save.NORMAL.as_slice(), save.UNORML.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CONST = save.SCALE;

        spicelib::NVC2PL(
            save.UNORML.as_slice(),
            save.CONST,
            save.PLANE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.DIR[1] = testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)?;
        save.DIR[2] = testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)?;
        save.DIR[3] = testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)?;

        spicelib::VHAT(save.DIR.as_slice(), save.UDIR.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.VERTEX[1] = (testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)? * save.SCALE);
        save.VERTEX[2] = (testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)? * save.SCALE);
        save.VERTEX[3] = (testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)? * save.SCALE);

        //
        // --- Case --------------------------------------------------------
        //
        fstr::assign(
            &mut save.TITLE,
            b"Scale = #; unit normal = (#,#,#); const = #; UDIR = (#,#,#), VERTEX = (#,#,#).",
        );

        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.SCALE,
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.UNORML[1],
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.UNORML[2],
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.UNORML[3],
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.CONST,
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.UDIR[1],
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.UDIR[2],
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.UDIR[3],
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.VERTEX[1],
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.VERTEX[2],
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.VERTEX[3],
            7,
            &mut save.TITLE,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::INRYPL(
            save.VERTEX.as_slice(),
            save.UDIR.as_slice(),
            save.PLANE.as_slice(),
            &mut save.XNXPTS,
            save.XXPT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Compute result using ZZINRYPL.
        //
        save.MAXD = (10000000000.0 * save.SCALE);

        spicelib::ZZINRYPL(
            save.VERTEX.as_slice(),
            save.UDIR.as_slice(),
            save.UNORML.as_slice(),
            save.CONST,
            save.MAXD,
            &mut save.NXPTS,
            save.XPT.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if ((save.XNXPTS == 1) || (save.XNXPTS == 0)) {
            testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

            if (*OK && (save.NXPTS == 1)) {
                testutil::CHCKAD(
                    b"XPT",
                    save.XPT.as_slice(),
                    b"~~/",
                    save.XXPT.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    //**********************************************************************
    //
    //     ZZRYXSPH tests
    //
    //**********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Unit sphere, vertex at center", ctx)?;

    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());

    save.DIR[1] = 1.0;
    save.DIR[2] = 0.0;
    save.DIR[3] = 0.0;

    save.R = 1.0;

    spicelib::ZZRYXSPH(
        save.VERTEX.as_slice(),
        save.DIR.as_slice(),
        save.R,
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::VPACK(1.0, 0.0, 0.0, save.XXPT.as_slice_mut());

    if (*OK && save.FOUND) {
        testutil::CHCKAD(
            b"XPT",
            save.XPT.as_slice(),
            b"~~/",
            save.XXPT.as_slice(),
            3,
            TIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Unit sphere, vertex on Z axis at Z=2; DIR points in -Z direction.",
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, 2.0, save.VERTEX.as_slice_mut());

    save.DIR[1] = 0.0;
    save.DIR[2] = 0.0;
    save.DIR[3] = -1.0;

    save.R = 1.0;

    spicelib::ZZRYXSPH(
        save.VERTEX.as_slice(),
        save.DIR.as_slice(),
        save.R,
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.XXPT.as_slice_mut());

    if (*OK && save.FOUND) {
        testutil::CHCKAD(
            b"XPT",
            save.XPT.as_slice(),
            b"~~/",
            save.XXPT.as_slice(),
            3,
            TIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Unit sphere, vertex on line X=1,Y=2 at Z=2; DIR points in -Z direction. Miss expected.",
        ctx,
    )?;

    spicelib::VPACK(1.0, 2.0, 2.0, save.VERTEX.as_slice_mut());

    save.DIR[1] = 0.0;
    save.DIR[2] = 0.0;
    save.DIR[3] = -1.0;

    save.R = 1.0;

    spicelib::ZZRYXSPH(
        save.VERTEX.as_slice(),
        save.DIR.as_slice(),
        save.R,
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //

    //
    // ZZRYXSPH random tests
    //

    save.SEED = -1;

    save.NCASE = 10000;

    for CASENO in 1..=save.NCASE {
        save.SCALE = f64::powf(10.0, testutil::T_RANDD(-10.0, 10.0, &mut save.SEED, ctx)?);

        save.R = ((testutil::T_RANDD(1.0, 2.0, &mut save.SEED, ctx)? * save.SCALE) / 2 as f64);

        save.VERTEX[1] = (testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)? * save.SCALE);
        save.VERTEX[2] = (testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)? * save.SCALE);
        save.VERTEX[3] = (testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)? * save.SCALE);

        save.DIR[1] =
            (-save.VERTEX[1] + (testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)? / 2 as f64));
        save.DIR[2] =
            (-save.VERTEX[2] + (testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)? / 2 as f64));
        save.DIR[3] =
            (-save.VERTEX[3] + (testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)? / 2 as f64));

        spicelib::VHAT(save.DIR.as_slice(), save.UDIR.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // --- Case --------------------------------------------------------
        //
        fstr::assign(
            &mut save.TITLE,
            b"Scale = #; unit normal = (#,#,#); const = #; UDIR = (#,#,#), VERTEX = (#,#,#).",
        );

        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.SCALE,
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.UNORML[1],
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.UNORML[2],
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.UNORML[3],
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.CONST,
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.UDIR[1],
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.UDIR[2],
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.UDIR[3],
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.VERTEX[1],
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.VERTEX[2],
            7,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.VERTEX[3],
            7,
            &mut save.TITLE,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::INRYPL(
            save.VERTEX.as_slice(),
            save.UDIR.as_slice(),
            save.PLANE.as_slice(),
            &mut save.XNXPTS,
            save.XXPT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Compute result using ZZINRYPL.
        //
        spicelib::SURFPT(
            save.VERTEX.as_slice(),
            save.UDIR.as_slice(),
            save.R,
            save.R,
            save.R,
            save.XXPT.as_slice_mut(),
            &mut save.XFOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::ZZRYXSPH(
            save.VERTEX.as_slice(),
            save.UDIR.as_slice(),
            save.R,
            save.XPT.as_slice_mut(),
            &mut save.FOUND,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        // WRITE (*,*) 'FOUND, ||V||-R = ', FOUND, VNORM(VERTEX)-R

        testutil::CHCKSL(b"FOUND", save.FOUND, save.XFOUND, OK, ctx)?;

        if (*OK && save.FOUND) {
            testutil::CHCKAD(
                b"XPT",
                save.XPT.as_slice(),
                b"~~/",
                save.XXPT.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;
        }
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
