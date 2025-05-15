//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FMT1: &[u8] = b"(F18.13)";
const IDWORD: &[u8] = b"KPL/FK";
const SQUOTE: &[u8] = b"\'";
const SIDLEN: i32 = 40;
const BSLASH: i32 = 92;
const OLSIZE: i32 = 255;
const MARKLN: i32 = 10;
const KVNMLN: i32 = 32;
const LMARG: i32 = 4;
const EQIDX: i32 = 40;
const VALIDX: i32 = (EQIDX + 3);
const NUMIDX: i32 = (VALIDX + 2);
const BDNMLN: i32 = 36;
const NUMLEN: i32 = 25;

//$Procedure T_TOPKER ( Create topocentric FK and SPK )
pub fn T_TOPKER(
    FK: &[u8],
    SPK: &[u8],
    TARGET: &[u8],
    TRGREF: &[u8],
    NSITES: i32,
    SITIDS: &[i32],
    SITNMS: CharArray,
    SITPOS: &[f64],
    SITFNM: CharArray,
    SITFID: &[i32],
    FIRST: f64,
    LAST: f64,
    AXES: &[i32],
    ANGLES: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SITIDS = DummyArray::new(SITIDS, 1..=NSITES);
    let SITNMS = DummyCharArray::new(SITNMS, None, 1..=NSITES);
    let SITPOS = DummyArray2D::new(SITPOS, 1..=3, 1..=NSITES);
    let SITFNM = DummyCharArray::new(SITFNM, None, 1..=NSITES);
    let SITFID = DummyArray::new(SITFID, 1..=NSITES);
    let AXES = DummyArray2D::new(AXES, 1..=3, 1..=NSITES);
    let ANGLES = DummyArray2D::new(ANGLES, 1..=3, 1..=NSITES);
    let mut ANGSTR = ActualCharArray::new(NUMLEN, 1..=3);
    let mut BEGDAT = [b' '; MARKLN as usize];
    let mut BEGTXT = [b' '; MARKLN as usize];
    let mut BNAME = [b' '; BDNMLN as usize];
    let mut FSTEM = [b' '; KVNMLN as usize];
    let mut OUTLIN = [b' '; OLSIZE as usize];
    let mut SEGID = [b' '; SIDLEN as usize];
    let mut ANGLE: f64 = 0.0;
    let mut STATES = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
    let mut STEP: f64 = 0.0;
    let mut DEGREE: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut NSTATE: i32 = 0;
    let mut TRGCDE: i32 = 0;
    let mut UNIT: i32 = 0;
    let mut FOUND: bool = false;

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

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_TOPKER", ctx)?;

    if (NSITES < 1) {
        spicelib::SETMSG(b"Site count must be positive but was #.", ctx);
        spicelib::ERRINT(b"#", NSITES, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        spicelib::CHKOUT(b"T_TOPKER", ctx)?;
        return Ok(());
    }

    //
    // Get the body ID for the target.
    //
    spicelib::BODS2C(TARGET, &mut TRGCDE, &mut FOUND, ctx)?;

    if !FOUND {
        spicelib::SETMSG(b"Could not translate body name #.", ctx);
        spicelib::ERRCH(b"#", TARGET, ctx);
        spicelib::SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        spicelib::CHKOUT(b"T_TOPKER", ctx)?;
        return Ok(());
    }

    //
    // Create an SPK file containing data for the input sites.
    //
    spicelib::SPKOPN(SPK, SPK, 0, &mut HANDLE, ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_TOPKER", ctx)?;
        return Ok(());
    }

    spicelib::CLEARD(12, STATES.as_slice_mut());

    fstr::assign(&mut SEGID, b" ");
    NSTATE = 2;
    DEGREE = 1;
    STEP = (LAST - FIRST);

    for I in 1..=NSITES {
        fstr::assign(&mut SEGID, b"Body #");
        spicelib::REPMI(&SEGID.clone(), b"#", I, &mut SEGID, ctx);

        spicelib::VEQU(SITPOS.subarray([1, I]), STATES.subarray_mut([1, 1]));
        spicelib::VEQU(SITPOS.subarray([1, I]), STATES.subarray_mut([1, 2]));

        spicelib::SPKW08(
            HANDLE,
            SITIDS[I],
            TRGCDE,
            TRGREF,
            FIRST,
            LAST,
            &SEGID,
            DEGREE,
            NSTATE,
            STATES.as_slice(),
            FIRST,
            STEP,
            ctx,
        )?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"T_TOPKER", ctx)?;
            return Ok(());
        }
    }

    spicelib::SPKCLS(HANDLE, ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_TOPKER", ctx)?;
        return Ok(());
    }

    //
    // Create frame kernel.
    //
    if spicelib::EXISTS(FK, ctx)? {
        spicelib::SETMSG(
            b"Frame kernel # already exists; this program wont\'t overwrite it.",
            ctx,
        );
        spicelib::ERRCH(b"#", FK, ctx);
        spicelib::SIGERR(b"SPICE(FILEEXISTS)", ctx)?;
        spicelib::CHKOUT(b"T_TOPKER", ctx)?;
        return Ok(());
    }

    //
    // Open output file.
    //
    spicelib::TXTOPN(FK, &mut UNIT, ctx)?;

    //
    // Write ID word.
    //
    spicelib::WRITLN(IDWORD, UNIT, ctx)?;

    spicelib::WRITLN(b" ", UNIT, ctx)?;

    fstr::assign(
        &mut OUTLIN,
        &fstr::concat(b"   FILE: ", fstr::substr(FK, 1..=spicelib::RTRIM(FK))),
    );
    spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;
    spicelib::WRITLN(b" ", UNIT, ctx)?;

    fstr::assign(&mut OUTLIN, b"   This file was created by T_TOPKER.");
    spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;
    spicelib::WRITLN(b" ", UNIT, ctx)?;

    //
    // Initialize data and text markers.
    //
    fstr::assign(
        &mut BEGDAT,
        &fstr::concat(&intrinsics::CHAR(BSLASH), b"begindata"),
    );
    fstr::assign(
        &mut BEGTXT,
        &fstr::concat(&intrinsics::CHAR(BSLASH), b"begintext"),
    );

    //
    // Create a body-name mapping for each central body, in case the
    // mapping is not already known to SPICE.
    //
    spicelib::WRITLN(b" ", UNIT, ctx)?;

    fstr::assign(&mut OUTLIN, b"   Body-name mapping follows:");
    spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;

    //
    // Start data area.
    //
    spicelib::WRITLN(b" ", UNIT, ctx)?;
    spicelib::WRITLN(&BEGDAT, UNIT, ctx)?;

    for I in 1..=NSITES {
        spicelib::UCASE(&SITNMS[I], &mut BNAME, ctx);

        spicelib::PREFIX(SQUOTE, 0, &mut BNAME);
        spicelib::SUFFIX(SQUOTE, 0, &mut BNAME);

        spicelib::WRITLN(b" ", UNIT, ctx)?;

        fstr::assign(&mut OUTLIN, b" ");
        fstr::assign(fstr::substr_mut(&mut OUTLIN, LMARG..), b"NAIF_BODY_NAME");
        fstr::assign(fstr::substr_mut(&mut OUTLIN, EQIDX..), b"+=");
        fstr::assign(fstr::substr_mut(&mut OUTLIN, VALIDX..), &BNAME);

        spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;

        fstr::assign(&mut OUTLIN, b" ");
        fstr::assign(fstr::substr_mut(&mut OUTLIN, LMARG..), b"NAIF_BODY_CODE");
        fstr::assign(fstr::substr_mut(&mut OUTLIN, EQIDX..), b"+=");

        spicelib::INTSTR(SITIDS[I], fstr::substr_mut(&mut OUTLIN, VALIDX..), ctx);

        spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;
    }

    //
    // End data area.
    //
    spicelib::WRITLN(b" ", UNIT, ctx)?;
    spicelib::WRITLN(&BEGTXT, UNIT, ctx)?;
    spicelib::WRITLN(b" ", UNIT, ctx)?;

    //
    // Introduce the frame definitions.
    //
    spicelib::WRITLN(b" ", UNIT, ctx)?;

    fstr::assign(&mut OUTLIN, b"   Reference frame specifications follow:");
    spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;
    spicelib::WRITLN(b" ", UNIT, ctx)?;
    spicelib::WRITLN(b" ", UNIT, ctx)?;

    //
    // Start data area.
    //
    spicelib::WRITLN(b" ", UNIT, ctx)?;
    spicelib::WRITLN(&BEGDAT, UNIT, ctx)?;
    spicelib::WRITLN(b" ", UNIT, ctx)?;

    for I in 1..=NSITES {
        //
        // Create the frame ID assignment.
        //
        fstr::assign(&mut OUTLIN, b" ");

        spicelib::LJUCRS(0, &SITFNM[I], fstr::substr_mut(&mut OUTLIN, LMARG..), ctx);

        spicelib::PREFIX(b"FRAME_", 0, fstr::substr_mut(&mut OUTLIN, LMARG..));

        fstr::assign(fstr::substr_mut(&mut OUTLIN, EQIDX..), b"=");

        spicelib::INTSTR(SITFID[I], fstr::substr_mut(&mut OUTLIN, VALIDX..), ctx);

        spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;

        //
        // Create the frame name assignment.
        //
        fstr::assign(&mut OUTLIN, b" ");

        spicelib::INTSTR(SITFID[I], &mut FSTEM, ctx);
        spicelib::PREFIX(b"FRAME_", 0, &mut FSTEM);

        fstr::assign(fstr::substr_mut(&mut OUTLIN, LMARG..), &FSTEM);
        spicelib::SUFFIX(b"_NAME", 0, fstr::substr_mut(&mut OUTLIN, LMARG..));

        fstr::assign(fstr::substr_mut(&mut OUTLIN, EQIDX..), b"=");

        spicelib::LJUCRS(0, &SITFNM[I], fstr::substr_mut(&mut OUTLIN, VALIDX..), ctx);

        spicelib::PREFIX(SQUOTE, 0, fstr::substr_mut(&mut OUTLIN, VALIDX..));
        spicelib::SUFFIX(SQUOTE, 0, fstr::substr_mut(&mut OUTLIN, VALIDX..));

        spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;

        //
        // Create the frame class and class ID assignments.
        //
        fstr::assign(&mut OUTLIN, b" ");
        fstr::assign(fstr::substr_mut(&mut OUTLIN, LMARG..), &FSTEM);
        spicelib::SUFFIX(b"_CLASS", 0, fstr::substr_mut(&mut OUTLIN, LMARG..));

        fstr::assign(fstr::substr_mut(&mut OUTLIN, EQIDX..), b"=  4");

        spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;

        fstr::assign(&mut OUTLIN, b" ");
        fstr::assign(fstr::substr_mut(&mut OUTLIN, LMARG..), &FSTEM);
        spicelib::SUFFIX(b"_CLASS_ID", 0, fstr::substr_mut(&mut OUTLIN, LMARG..));

        fstr::assign(fstr::substr_mut(&mut OUTLIN, EQIDX..), b"=");

        spicelib::INTSTR(SITFID[I], fstr::substr_mut(&mut OUTLIN, VALIDX..), ctx);

        spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;

        //
        // Create the center assignment.
        //
        fstr::assign(&mut OUTLIN, b" ");
        fstr::assign(fstr::substr_mut(&mut OUTLIN, LMARG..), &FSTEM);
        spicelib::SUFFIX(b"_CENTER", 0, fstr::substr_mut(&mut OUTLIN, LMARG..));

        fstr::assign(fstr::substr_mut(&mut OUTLIN, EQIDX..), b"=");

        spicelib::INTSTR(SITIDS[I], fstr::substr_mut(&mut OUTLIN, VALIDX..), ctx);

        spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;

        //
        // Create the object-frame association assignment.
        //
        fstr::assign(&mut OUTLIN, b" ");

        spicelib::INTSTR(SITIDS[I], fstr::substr_mut(&mut OUTLIN, LMARG..), ctx);
        spicelib::PREFIX(b"OBJECT_", 0, fstr::substr_mut(&mut OUTLIN, LMARG..));
        spicelib::SUFFIX(b"_FRAME", 0, fstr::substr_mut(&mut OUTLIN, LMARG..));

        fstr::assign(fstr::substr_mut(&mut OUTLIN, EQIDX..), b"=");

        spicelib::LJUCRS(0, &SITFNM[I], fstr::substr_mut(&mut OUTLIN, VALIDX..), ctx);

        spicelib::PREFIX(SQUOTE, 0, fstr::substr_mut(&mut OUTLIN, VALIDX..));
        spicelib::SUFFIX(SQUOTE, 0, fstr::substr_mut(&mut OUTLIN, VALIDX..));

        spicelib::WRITLN(b" ", UNIT, ctx)?;
        spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;
        spicelib::WRITLN(b" ", UNIT, ctx)?;

        //
        // Create the relative frame assignment.
        //
        spicelib::PREFIX(b"TK", 0, &mut FSTEM);

        fstr::assign(&mut OUTLIN, b" ");
        fstr::assign(fstr::substr_mut(&mut OUTLIN, LMARG..), &FSTEM);
        spicelib::SUFFIX(b"_RELATIVE", 0, fstr::substr_mut(&mut OUTLIN, LMARG..));

        fstr::assign(fstr::substr_mut(&mut OUTLIN, EQIDX..), b"=");

        fstr::assign(fstr::substr_mut(&mut OUTLIN, VALIDX..), TRGREF);

        spicelib::PREFIX(SQUOTE, 0, fstr::substr_mut(&mut OUTLIN, VALIDX..));
        spicelib::SUFFIX(SQUOTE, 0, fstr::substr_mut(&mut OUTLIN, VALIDX..));

        spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;

        //
        // Create the frame and units assignments.
        //
        fstr::assign(&mut OUTLIN, b" ");

        fstr::assign(fstr::substr_mut(&mut OUTLIN, LMARG..), &FSTEM);

        spicelib::SUFFIX(b"_SPEC", 0, fstr::substr_mut(&mut OUTLIN, LMARG..));

        fstr::assign(fstr::substr_mut(&mut OUTLIN, EQIDX..), b"=  \'ANGLES\'");

        spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;

        fstr::assign(&mut OUTLIN, b" ");

        fstr::assign(fstr::substr_mut(&mut OUTLIN, LMARG..), &FSTEM);

        spicelib::SUFFIX(b"_UNITS", 0, fstr::substr_mut(&mut OUTLIN, LMARG..));

        fstr::assign(fstr::substr_mut(&mut OUTLIN, EQIDX..), b"=  \'DEGREES\'");

        spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;

        //
        // Create the axis sequence assignment.
        //
        fstr::assign(&mut OUTLIN, b" ");

        fstr::assign(fstr::substr_mut(&mut OUTLIN, LMARG..), &FSTEM);

        spicelib::SUFFIX(b"_AXES", 0, fstr::substr_mut(&mut OUTLIN, LMARG..));

        fstr::assign(fstr::substr_mut(&mut OUTLIN, EQIDX..), b"=");

        fstr::assign(fstr::substr_mut(&mut OUTLIN, VALIDX..), b"( #, #, # )");

        for J in 1..=3 {
            spicelib::REPMI(
                &fstr::substr(&OUTLIN, VALIDX..).to_vec(),
                b"#",
                AXES[[J, I]],
                fstr::substr_mut(&mut OUTLIN, VALIDX..),
                ctx,
            );
        }

        spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;

        //
        // Create the angle sequence assignments.
        //
        //
        // First convert the angles to strings. Scale to degrees
        // in the process.
        //
        for J in 1..=3 {
            ANGLE = (ANGLES[[J, I]] * spicelib::DPR(ctx));

            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let internal_file = io::InternalFile::open(ANGSTR.get_mut(J));
                let mut writer = io::FormattedWriter::new(internal_file, None, FMT1)?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_f64(ANGLE)?;
                    writer.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                spicelib::SETMSG(
                    b"Attempt to write angle # for frame # failed; IOSTAT was #.",
                    ctx,
                );
                spicelib::ERRINT(b"#", J, ctx);
                spicelib::ERRCH(b"#", &SITFNM[I], ctx);
                spicelib::ERRINT(b"#", IOSTAT, ctx);
                spicelib::SIGERR(b"SPICE(WRITEFAILED)", ctx)?;
                spicelib::CHKOUT(b"T_TOPKER", ctx)?;
                return Ok(());
            }
        }

        //
        // First angle:
        //
        fstr::assign(&mut OUTLIN, b" ");
        fstr::assign(fstr::substr_mut(&mut OUTLIN, LMARG..), &FSTEM);

        spicelib::SUFFIX(b"_ANGLES", 0, fstr::substr_mut(&mut OUTLIN, LMARG..));

        fstr::assign(fstr::substr_mut(&mut OUTLIN, EQIDX..), b"=  (");

        fstr::assign(fstr::substr_mut(&mut OUTLIN, NUMIDX..), ANGSTR.get(1));

        spicelib::SUFFIX(b",", 0, fstr::substr_mut(&mut OUTLIN, NUMIDX..));

        spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;

        //
        // Second angle:
        //
        fstr::assign(&mut OUTLIN, b" ");

        fstr::assign(fstr::substr_mut(&mut OUTLIN, NUMIDX..), ANGSTR.get(2));

        spicelib::SUFFIX(b",", 0, fstr::substr_mut(&mut OUTLIN, NUMIDX..));

        spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;

        //
        // Third angle:
        //
        fstr::assign(&mut OUTLIN, b" ");

        fstr::assign(fstr::substr_mut(&mut OUTLIN, NUMIDX..), ANGSTR.get(3));

        spicelib::SUFFIX(b")", 1, fstr::substr_mut(&mut OUTLIN, NUMIDX..));

        spicelib::WRITLN(&OUTLIN, UNIT, ctx)?;

        //
        // Add a little vertical white space between frame
        // specifications.
        //
        spicelib::WRITLN(b" ", UNIT, ctx)?;
    }

    //
    // End data area.
    //
    spicelib::WRITLN(&BEGTXT, UNIT, ctx)?;
    spicelib::WRITLN(b" ", UNIT, ctx)?;

    spicelib::WRITLN(b" ", UNIT, ctx)?;
    spicelib::WRITLN(b"[End of definitions file]", UNIT, ctx)?;
    spicelib::WRITLN(b" ", UNIT, ctx)?;

    //
    // Close output file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    spicelib::CHKOUT(b"T_TOPKER", ctx)?;
    Ok(())
}
