//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const BDNMLN: i32 = 36;
const LNSIZE: i32 = 80;
const NLINES: i32 = 100;
const UBEL: i32 = 9;
const MAXVRT: i32 = 10;

//$Procedure   NATIK ( Create an IK for Nat's solar system )
pub fn NATIK(
    IK: &[u8],
    SPK: &[u8],
    PCK: &[u8],
    LOADIK: bool,
    KEEPIK: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut TX = ActualCharArray::new(LNSIZE, 1..=NLINES);
    let mut INSNAM = [b' '; BDNMLN as usize];
    let mut BSITE = StackArray::<f64, 3>::new(1..=3);
    let mut CENTER = StackArray::<f64, 3>::new(1..=3);
    let mut BNDS = StackArray2D::<f64, 30>::new(1..=3, 1..=MAXVRT);
    let mut BVEC = StackArray2D::<f64, 30>::new(1..=3, 1..=MAXVRT);
    let mut ET0: f64 = 0.0;
    let mut LIMB = StackArray::<f64, 9>::new(1..=UBEL);
    let mut LT: f64 = 0.0;
    let mut POS = StackArray::<f64, 3>::new(1..=3);
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
    let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
    let mut VIEWPT = StackArray::<f64, 3>::new(1..=3);
    let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
    let mut HANDLE: i32 = 0;
    let mut INST: i32 = 0;
    let mut IXBCB: i32 = 0;
    let mut IXBSB: i32 = 0;
    let mut IXINB: i32 = 0;
    let mut IXINE: i32 = 0;
    let mut J: i32 = 0;
    let mut L: i32 = 0;
    let mut N: i32 = 0;
    let mut NL: i32 = 0;
    let mut NV: i32 = 0;
    let mut UNIT: i32 = 0;

    //
    // Test Utility Functions
    //

    //
    // Local Parameters
    //

    //
    // Local variables
    //

    spicelib::CHKIN(b"NATIK", ctx)?;

    //
    // Delete any existing file of the same name.
    //
    KILFIL(IK, ctx)?;

    //
    // Before creating the IK, we derive the parameters
    // needed to create the instrument FOVs we'll specify there.
    //
    // Load the Nat's solar system SPK and PCK.
    //
    spicelib::SPKLEF(SPK, &mut HANDLE, ctx)?;
    spicelib::LDPOOL(PCK, ctx)?;

    spicelib::BODVRD(b"ALPHA", b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;

    //
    // Compute the limb of body Alpha as seen from the Sun
    // at 0 seconds past J2000 TDB. Do not use aberration
    // corrections.
    //
    ET0 = 0.0;

    spicelib::SPKPOS(
        b"ALPHA",
        ET0,
        b"ALPHAFIXED",
        b"NONE",
        b"SUN",
        POS.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    spicelib::VMINUS(POS.as_slice(), VIEWPT.as_slice_mut());

    spicelib::EDLIMB(
        RADII[1],
        RADII[2],
        RADII[3],
        VIEWPT.as_slice(),
        LIMB.as_slice_mut(),
        ctx,
    )?;

    //
    // Create boundary vectors for an elliptical FOV in the
    // ALPHAFIXED frame.
    //
    spicelib::EL2CGV(
        LIMB.as_slice(),
        CENTER.as_slice_mut(),
        SMAJOR.as_slice_mut(),
        SMINOR.as_slice_mut(),
    );

    //**********************************************************************
    //
    //     Create FOV specification and instrument name-ID mapping
    //     for the instrument "Alpha_Ellipse_NONE."
    //
    //**********************************************************************

    //
    // This FOV is fixed to the dynamic frame ALPHA_VIEW_XY.
    // Thus the instrument "tracks" body Alpha. As seen from the
    // Sun, with aberration corrections turned off,
    // the FOV coincides with the limb of Alpha.
    //

    fstr::assign(&mut INSNAM, b"ALPHA_ELLIPSE_NONE");
    INST = -1500000;

    //
    // Look up the transformation from the ALPHAFIXED frame to the
    // ALPHA_VIEW_XY frame. The latter rotates with the orbital motion
    // of planet Alpha.
    //
    spicelib::PXFORM(
        b"ALPHAFIXED",
        b"ALPHA_VIEW_XY",
        ET0,
        XFORM.as_slice_mut(),
        ctx,
    )?;

    //
    // Compute the boresight vector.
    //
    spicelib::MXV(XFORM.as_slice(), POS.as_slice(), VTEMP.as_slice_mut());
    spicelib::VHAT(VTEMP.as_slice(), BSITE.as_slice_mut());

    //
    // New!
    //
    spicelib::VSCLIP(2.0, BSITE.as_slice_mut());

    spicelib::VLCOM3(
        1.0,
        CENTER.as_slice(),
        1.0,
        SMAJOR.as_slice(),
        -1.0,
        VIEWPT.as_slice(),
        BVEC.subarray_mut([1, 1]),
    );

    spicelib::VLCOM3(
        1.0,
        CENTER.as_slice(),
        1.0,
        SMINOR.as_slice(),
        -1.0,
        VIEWPT.as_slice(),
        BVEC.subarray_mut([1, 2]),
    );

    for I in 1..=2 {
        spicelib::MXV(
            XFORM.as_slice(),
            BVEC.subarray([1, I]),
            BNDS.subarray_mut([1, I]),
        );
    }

    //
    // As obnoxious as the practice may seem, we'll use the variable
    // L instead of hard-coded constants to index buffer elements.
    // Hard-coded indices are a big problem when changes are needed.
    //
    L = 1;
    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b" KPL/IK");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);
    BEGDAT(&mut TX[L]);
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);

    fstr::assign(TX.get_mut(L), b"NAIF_BODY_NAME += \'#\' ");
    spicelib::REPMC(&TX[L].to_vec(), b"#", &INSNAM, &mut TX[L]);
    L = (L + 1);

    fstr::assign(TX.get_mut(L), b"NAIF_BODY_CODE += #");
    spicelib::REPMI(&TX[L].to_vec(), b"#", INST, &mut TX[L], ctx);
    L = (L + 1);

    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);

    IXINB = L;
    fstr::assign(TX.get_mut(L), b"INS#_FOV_SHAPE            = \'ELLIPSE\' ");
    L = (L + 1);
    fstr::assign(
        TX.get_mut(L),
        b"INS#_FOV_FRAME            = \'ALPHA_VIEW_XY\' ",
    );
    L = (L + 1);
    IXBSB = L;
    fstr::assign(TX.get_mut(L), b"INS#_BORESIGHT            = ( *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     * )");
    L = (L + 1);
    IXBCB = L;
    fstr::assign(TX.get_mut(L), b"INS#_FOV_BOUNDARY         = ( *");
    IXINE = L;
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     * )");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);
    BEGTXT(&mut TX[L]);
    L = (L + 1);

    //
    // Set the instrument ID in the keywords above.
    //
    for I in IXINB..=IXINE {
        spicelib::REPMI(&TX[I].to_vec(), b"#", INST, &mut TX[I], ctx);
    }

    //
    // Set the boresight values.
    //
    J = (IXBSB - 1);

    for I in 1..=3 {
        spicelib::REPMD(
            &TX[(J + I)].to_vec(),
            b"*",
            BSITE[I],
            14,
            &mut TX[(J + I)],
            ctx,
        );
    }

    //
    // Set the boundary corner values.
    //
    J = (IXBCB - 1);

    for I in 1..=3 {
        spicelib::REPMD(
            &TX[(J + I)].to_vec(),
            b"*",
            BNDS[[I, 1]],
            14,
            &mut TX[(J + I)],
            ctx,
        );
        spicelib::REPMD(
            &TX[((J + I) + 3)].to_vec(),
            b"*",
            BNDS[[I, 2]],
            14,
            &mut TX[((J + I) + 3)],
            ctx,
        );
    }

    //**********************************************************************
    //
    //     Create FOV specification and instrument name-ID mapping
    //     for the instrument "Alpha_Circle_NONE."
    //
    //**********************************************************************

    //
    // This FOV is fixed to the dynamic frame ALPHA_VIEW_XY.
    // Thus the instrument "tracks" body Alpha. As seen from the
    // Sun, with aberration corrections turned off,
    // the FOV is inscribed in the limb of Alpha.
    //

    fstr::assign(&mut INSNAM, b"ALPHA_CIRCLE_NONE");
    INST = -1500001;

    //
    // Look up the transformation from the ALPHAFIXED frame to the
    // ALPHA_VIEW_XY frame. The latter rotates with the orbital motion
    // of planet Alpha.
    //
    spicelib::PXFORM(
        b"ALPHAFIXED",
        b"ALPHA_VIEW_XY",
        ET0,
        XFORM.as_slice_mut(),
        ctx,
    )?;

    //
    // Compute the boresight vector.
    //
    spicelib::MXV(XFORM.as_slice(), POS.as_slice(), VTEMP.as_slice_mut());
    spicelib::VHAT(VTEMP.as_slice(), BSITE.as_slice_mut());

    //
    // We use the semi-minor axis to define the boundary vector,
    // since we want our circle to be inscribed in Alpha's limb.
    //
    spicelib::VLCOM3(
        1.0,
        CENTER.as_slice(),
        1.0,
        SMINOR.as_slice(),
        -1.0,
        VIEWPT.as_slice(),
        BVEC.subarray_mut([1, 1]),
    );

    spicelib::MXV(
        XFORM.as_slice(),
        BVEC.subarray([1, 1]),
        BNDS.subarray_mut([1, 1]),
    );

    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b" KPL/TX");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);
    BEGDAT(&mut TX[L]);
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);

    fstr::assign(TX.get_mut(L), b"NAIF_BODY_NAME += \'#\' ");
    spicelib::REPMC(&TX[L].to_vec(), b"#", &INSNAM, &mut TX[L]);
    L = (L + 1);

    fstr::assign(TX.get_mut(L), b"NAIF_BODY_CODE += #");
    spicelib::REPMI(&TX[L].to_vec(), b"#", INST, &mut TX[L], ctx);
    L = (L + 1);

    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);

    IXINB = L;
    fstr::assign(TX.get_mut(L), b"INS#_FOV_SHAPE            = \'CIRCLE\' ");
    L = (L + 1);
    fstr::assign(
        TX.get_mut(L),
        b"INS#_FOV_FRAME            = \'ALPHA_VIEW_XY\' ",
    );
    L = (L + 1);
    IXBSB = L;
    fstr::assign(TX.get_mut(L), b"INS#_BORESIGHT            = ( *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     * )");
    L = (L + 1);
    IXBCB = L;
    fstr::assign(TX.get_mut(L), b"INS#_FOV_BOUNDARY         = ( *");
    IXINE = L;
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     * )");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);
    BEGTXT(&mut TX[L]);
    L = (L + 1);

    //
    // Set the instrument ID in the keywords above.
    //
    for I in IXINB..=IXINE {
        spicelib::REPMI(&TX[I].to_vec(), b"#", INST, &mut TX[I], ctx);
    }

    //
    // Set the boresight values.
    //
    J = (IXBSB - 1);

    for I in 1..=3 {
        spicelib::REPMD(
            &TX[(J + I)].to_vec(),
            b"*",
            BSITE[I],
            14,
            &mut TX[(J + I)],
            ctx,
        );
    }

    //
    // Set the boundary corner values.
    //
    J = (IXBCB - 1);

    for I in 1..=3 {
        spicelib::REPMD(
            &TX[(J + I)].to_vec(),
            b"*",
            BNDS[[I, 1]],
            14,
            &mut TX[(J + I)],
            ctx,
        );
    }

    //**********************************************************************
    //
    //     Create FOV specification and instrument name-ID mapping
    //     for the instrument "Alpha_Rectangle_NONE."
    //
    //**********************************************************************

    //
    // This FOV is fixed to the dynamic frame ALPHA_VIEW_XY. Thus the
    // instrument "tracks" body Alpha. As seen from the Sun, with
    // aberration corrections turned off, the FOV circumscribes the limb
    // of Alpha.
    //

    fstr::assign(&mut INSNAM, b"ALPHA_RECTANGLE_NONE");
    INST = -1500003;
    NV = 4;

    //
    // Look up the transformation from the ALPHAFIXED frame to the
    // ALPHA_VIEW_XY frame. The latter rotates with the orbital motion
    // of planet Alpha.
    //
    spicelib::PXFORM(
        b"ALPHAFIXED",
        b"ALPHA_VIEW_XY",
        ET0,
        XFORM.as_slice_mut(),
        ctx,
    )?;

    //
    // Compute the boresight vector.
    //
    spicelib::MXV(XFORM.as_slice(), POS.as_slice(), VTEMP.as_slice_mut());
    spicelib::VHAT(VTEMP.as_slice(), BSITE.as_slice_mut());

    //
    // Compute the boundary vectors.
    //
    spicelib::VSUB(CENTER.as_slice(), VIEWPT.as_slice(), VTEMP.as_slice_mut());

    spicelib::VLCOM3(
        1.0,
        SMINOR.as_slice(),
        1.0,
        SMAJOR.as_slice(),
        1.0,
        VTEMP.as_slice(),
        BVEC.subarray_mut([1, 1]),
    );

    spicelib::VLCOM3(
        -1.0,
        SMINOR.as_slice(),
        1.0,
        SMAJOR.as_slice(),
        1.0,
        VTEMP.as_slice(),
        BVEC.subarray_mut([1, 2]),
    );

    spicelib::VLCOM3(
        -1.0,
        SMINOR.as_slice(),
        -1.0,
        SMAJOR.as_slice(),
        1.0,
        VTEMP.as_slice(),
        BVEC.subarray_mut([1, 3]),
    );

    spicelib::VLCOM3(
        1.0,
        SMINOR.as_slice(),
        -1.0,
        SMAJOR.as_slice(),
        1.0,
        VTEMP.as_slice(),
        BVEC.subarray_mut([1, 4]),
    );

    //
    // Apply the transformation from the ALPHAFIXED frame to the
    // ALPHA_VIEW_XY frame. The latter rotates with the orbital motion
    // of planet Alpha.
    //
    spicelib::MXV(XFORM.as_slice(), POS.as_slice(), VTEMP.as_slice_mut());
    spicelib::VHAT(VTEMP.as_slice(), BSITE.as_slice_mut());

    for I in 1..=NV {
        spicelib::MXV(
            XFORM.as_slice(),
            BVEC.subarray([1, I]),
            BNDS.subarray_mut([1, I]),
        );

        spicelib::VHAT(BNDS.subarray([1, I]), VTEMP.as_slice_mut());
        spicelib::VEQU(VTEMP.as_slice(), BNDS.subarray_mut([1, I]));
    }

    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);
    BEGDAT(&mut TX[L]);
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);

    fstr::assign(TX.get_mut(L), b"NAIF_BODY_NAME += \'#\' ");
    spicelib::REPMC(&TX[L].to_vec(), b"#", &INSNAM, &mut TX[L]);
    L = (L + 1);

    fstr::assign(TX.get_mut(L), b"NAIF_BODY_CODE += #");
    spicelib::REPMI(&TX[L].to_vec(), b"#", INST, &mut TX[L], ctx);
    L = (L + 1);

    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);
    IXINB = L;

    fstr::assign(TX.get_mut(L), b"INS#_FOV_SHAPE            = \'RECTANGLE\' ");
    L = (L + 1);
    fstr::assign(
        TX.get_mut(L),
        b"INS#_FOV_FRAME            = \'ALPHA_VIEW_XY\' ",
    );
    L = (L + 1);
    IXBSB = L;
    fstr::assign(TX.get_mut(L), b"INS#_BORESIGHT            = ( *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     * )");
    L = (L + 1);
    IXBCB = L;
    fstr::assign(TX.get_mut(L), b"INS#_FOV_BOUNDARY         = ( *");
    IXINE = L;
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     * )");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);
    BEGTXT(&mut TX[L]);
    L = (L + 1);

    //
    // Set the instrument ID in the keywords above.
    //
    for I in IXINB..=IXINE {
        spicelib::REPMI(&TX[I].to_vec(), b"#", INST, &mut TX[I], ctx);
    }

    //
    // Set the boresight values.
    //
    J = (IXBSB - 1);

    for I in 1..=3 {
        spicelib::REPMD(
            &TX[(J + I)].to_vec(),
            b"*",
            BSITE[I],
            14,
            &mut TX[(J + I)],
            ctx,
        );
    }

    //
    // Set the boundary corner values.
    //
    J = (IXBCB - 1);

    for I in 1..=3 {
        spicelib::REPMD(
            &TX[(J + I)].to_vec(),
            b"*",
            BNDS[[I, 1]],
            14,
            &mut TX[(J + I)],
            ctx,
        );
        spicelib::REPMD(
            &TX[((J + I) + 3)].to_vec(),
            b"*",
            BNDS[[I, 2]],
            14,
            &mut TX[((J + I) + 3)],
            ctx,
        );
        spicelib::REPMD(
            &TX[((J + I) + 6)].to_vec(),
            b"*",
            BNDS[[I, 3]],
            14,
            &mut TX[((J + I) + 6)],
            ctx,
        );
        spicelib::REPMD(
            &TX[((J + I) + 9)].to_vec(),
            b"*",
            BNDS[[I, 4]],
            14,
            &mut TX[((J + I) + 9)],
            ctx,
        );
    }

    //**********************************************************************
    //
    //     Create FOV specification and instrument name-ID mapping
    //     for the instrument "Alpha_Diamond_NONE."
    //
    //**********************************************************************

    //
    // This FOV is fixed to the dynamic frame ALPHA_VIEW_XZ (note: NOT
    // "_XY"). Thus the instrument "tracks" body Alpha. As seen from the
    // Sun, with aberration corrections turned off, the FOV inscribes
    // the limb of Alpha.
    //

    fstr::assign(&mut INSNAM, b"ALPHA_DIAMOND_NONE");
    INST = -1500004;
    NV = 4;

    //
    // Look up the transformation from the ALPHAFIXED frame to the
    // ALPHA_VIEW_XY frame. The latter rotates with the orbital motion
    // of planet Alpha.
    //
    spicelib::PXFORM(
        b"ALPHAFIXED",
        b"ALPHA_VIEW_XZ",
        ET0,
        XFORM.as_slice_mut(),
        ctx,
    )?;

    //
    // Compute the boresight vector.
    //
    spicelib::MXV(XFORM.as_slice(), POS.as_slice(), VTEMP.as_slice_mut());
    spicelib::VHAT(VTEMP.as_slice(), BSITE.as_slice_mut());

    //
    // Compute the boundary vectors.
    //
    spicelib::VSUB(CENTER.as_slice(), VIEWPT.as_slice(), VTEMP.as_slice_mut());

    spicelib::VLCOM(
        1.0,
        SMINOR.as_slice(),
        1.0,
        VTEMP.as_slice(),
        BVEC.subarray_mut([1, 1]),
    );

    spicelib::VLCOM(
        1.0,
        SMAJOR.as_slice(),
        1.0,
        VTEMP.as_slice(),
        BVEC.subarray_mut([1, 2]),
    );

    spicelib::VLCOM(
        -1.0,
        SMINOR.as_slice(),
        1.0,
        VTEMP.as_slice(),
        BVEC.subarray_mut([1, 3]),
    );

    spicelib::VLCOM(
        -1.0,
        SMAJOR.as_slice(),
        1.0,
        VTEMP.as_slice(),
        BVEC.subarray_mut([1, 4]),
    );

    //
    // Apply the transformation from the ALPHAFIXED frame to the
    // ALPHA_VIEW_XZ frame. The latter rotates with the orbital motion
    // of planet Alpha.
    //
    spicelib::MXV(XFORM.as_slice(), POS.as_slice(), VTEMP.as_slice_mut());
    spicelib::VHAT(VTEMP.as_slice(), BSITE.as_slice_mut());

    for I in 1..=NV {
        spicelib::MXV(
            XFORM.as_slice(),
            BVEC.subarray([1, I]),
            BNDS.subarray_mut([1, I]),
        );

        spicelib::VHAT(BNDS.subarray([1, I]), VTEMP.as_slice_mut());
        spicelib::VEQU(VTEMP.as_slice(), BNDS.subarray_mut([1, I]));
    }

    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);
    BEGDAT(&mut TX[L]);
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);

    fstr::assign(TX.get_mut(L), b"NAIF_BODY_NAME += \'#\' ");
    spicelib::REPMC(&TX[L].to_vec(), b"#", &INSNAM, &mut TX[L]);
    L = (L + 1);

    fstr::assign(TX.get_mut(L), b"NAIF_BODY_CODE += #");
    spicelib::REPMI(&TX[L].to_vec(), b"#", INST, &mut TX[L], ctx);
    L = (L + 1);

    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);
    IXINB = L;

    fstr::assign(TX.get_mut(L), b"INS#_FOV_SHAPE            = \'POLYGON\' ");
    L = (L + 1);
    fstr::assign(
        TX.get_mut(L),
        b"INS#_FOV_FRAME            = \'ALPHA_VIEW_XZ\' ",
    );
    L = (L + 1);
    IXBSB = L;
    fstr::assign(TX.get_mut(L), b"INS#_BORESIGHT            = ( *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     * )");
    L = (L + 1);
    IXBCB = L;
    fstr::assign(TX.get_mut(L), b"INS#_FOV_BOUNDARY         = ( *");
    IXINE = L;
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     *");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b"                                     * )");
    L = (L + 1);
    fstr::assign(TX.get_mut(L), b" ");
    L = (L + 1);
    BEGTXT(&mut TX[L]);
    L = (L + 1);

    //
    // Set the instrument ID in the keywords above.
    //
    for I in IXINB..=IXINE {
        spicelib::REPMI(&TX[I].to_vec(), b"#", INST, &mut TX[I], ctx);
    }

    //
    // Set the boresight values.
    //
    J = (IXBSB - 1);

    for I in 1..=3 {
        spicelib::REPMD(
            &TX[(J + I)].to_vec(),
            b"*",
            BSITE[I],
            14,
            &mut TX[(J + I)],
            ctx,
        );
    }

    //
    // Set the boundary corner values.
    //
    J = (IXBCB - 1);

    for I in 1..=3 {
        spicelib::REPMD(
            &TX[(J + I)].to_vec(),
            b"*",
            BNDS[[I, 1]],
            14,
            &mut TX[(J + I)],
            ctx,
        );
        spicelib::REPMD(
            &TX[((J + I) + 3)].to_vec(),
            b"*",
            BNDS[[I, 2]],
            14,
            &mut TX[((J + I) + 3)],
            ctx,
        );
        spicelib::REPMD(
            &TX[((J + I) + 6)].to_vec(),
            b"*",
            BNDS[[I, 3]],
            14,
            &mut TX[((J + I) + 6)],
            ctx,
        );
        spicelib::REPMD(
            &TX[((J + I) + 9)].to_vec(),
            b"*",
            BNDS[[I, 4]],
            14,
            &mut TX[((J + I) + 9)],
            ctx,
        );
    }

    //**********************************************************************
    //
    //     Create the IK.
    //
    //**********************************************************************

    //
    // L is incremented after each assignment; decrement it here.
    //
    NL = (L - 1);

    spicelib::TXTOPN(IK, &mut UNIT, ctx)?;

    for I in 1..=NL {
        spicelib::WRITLN(&TX[I], UNIT, ctx)?;
    }

    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    //
    // If this file needs to be loaded, do it.
    //
    if LOADIK {
        spicelib::LDPOOL(IK, ctx)?;
    }

    if KEEPIK {
        //
        // If we are keeping this file, we need to register it
        // with FILREG.
        //
        TFILES(IK, ctx);
    } else {
        //
        // Lose the file.
        //
        KILFIL(IK, ctx)?;
    }

    if !LOADIK {
        //
        // Unload the SPK.
        //
        spicelib::SPKUEF(HANDLE, ctx)?;
    }

    spicelib::CHKOUT(b"NATIK", ctx)?;
    Ok(())
}
