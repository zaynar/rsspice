//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MINCOS: f64 = 0.000000000000001;
const VTIGHT: f64 = MINCOS;
const BUFLEN: i32 = 2;
const LINLEN: i32 = 80;
const WDSIZE: i32 = 32;

//$Procedure F_GETFOV (Family of tests for GETFOV, GETFVN )
pub fn F_GETFOV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BOREST = ActualCharArray::new(WDSIZE, 1..=3);
    let mut FOVFRM = [b' '; WDSIZE as usize];
    let mut FOVSHP = [b' '; WDSIZE as usize];
    let mut FRAME = [b' '; WDSIZE as usize];
    let mut INSTNM = [b' '; WDSIZE as usize];
    let mut SHAPE = [b' '; WDSIZE as usize];
    let mut SPEC = [b' '; WDSIZE as usize];
    let mut UNITS = [b' '; WDSIZE as usize];
    let mut ANGLE: f64 = 0.0;
    let mut BSIGHT = StackArray::<f64, 3>::new(1..=3);
    let mut BOUNDS = StackArray2D::<f64, 15>::new(1..=3, 1..=5);
    let mut CRSANG: f64 = 0.0;
    let mut FOVBS = StackArray::<f64, 3>::new(1..=3);
    let mut FOVBND = StackArray2D::<f64, 15>::new(1..=3, 1..=5);
    let mut REFVEC = StackArray::<f64, 3>::new(1..=3);
    let mut TMPVEC = StackArray::<f64, 3>::new(1..=3);
    let mut INSTID: i32 = 0;
    let mut N: i32 = 0;
    let mut ROOM: i32 = 0;
    let mut KWBORE = [b' '; WDSIZE as usize];
    let mut KWSHAP = [b' '; WDSIZE as usize];
    let mut KWFRAM = [b' '; WDSIZE as usize];
    let mut KWBOUN = [b' '; WDSIZE as usize];
    let mut KWSPEC = [b' '; WDSIZE as usize];
    let mut KWRVEC = [b' '; WDSIZE as usize];
    let mut KWRANG = [b' '; WDSIZE as usize];
    let mut KWAUNT = [b' '; WDSIZE as usize];
    let mut KWCANG = [b' '; WDSIZE as usize];
    let mut BUFFER = ActualCharArray::new(LINLEN, 1..=BUFLEN);

    //
    // SPICELIB Functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_GETFOV", ctx)?;

    fstr::assign(&mut KWBOUN, b"INS-1000_FOV_BOUNDARY");
    fstr::assign(&mut KWBORE, b"INS-1000_BORESIGHT");
    fstr::assign(&mut KWSHAP, b"INS-1000_FOV_SHAPE");
    fstr::assign(&mut KWFRAM, b"INS-1000_FOV_FRAME");
    fstr::assign(&mut KWSPEC, b"INS-1000_FOV_CLASS_SPEC");
    fstr::assign(&mut KWRVEC, b"INS-1000_FOV_REF_VECTOR");
    fstr::assign(&mut KWRANG, b"INS-1000_FOV_REF_ANGLE");
    fstr::assign(&mut KWAUNT, b"INS-1000_FOV_ANGLE_UNITS");
    fstr::assign(&mut KWCANG, b"INS-1000_FOV_CROSS_ANGLE");

    ROOM = 3;
    INSTID = -1000;

    fstr::assign(&mut SHAPE, b"POLYGON");
    fstr::assign(&mut FRAME, b"CKERNEL");
    fstr::assign(&mut INSTNM, b"CINSTR");
    fstr::assign(&mut UNITS, b"DEGREES");

    BOUNDS[[1, 1]] = 1.0;
    BOUNDS[[2, 1]] = 1.0;
    BOUNDS[[3, 1]] = 1.0;

    BOUNDS[[1, 2]] = -1.0;
    BOUNDS[[2, 2]] = 1.0;
    BOUNDS[[3, 2]] = 1.0;

    BOUNDS[[1, 3]] = -1.0;
    BOUNDS[[2, 3]] = -1.0;
    BOUNDS[[3, 3]] = 1.0;

    BOUNDS[[1, 4]] = 1.0;
    BOUNDS[[2, 4]] = -1.0;
    BOUNDS[[3, 4]] = 1.0;

    BOUNDS[[1, 5]] = 0.0;
    BOUNDS[[2, 5]] = 0.0;
    BOUNDS[[3, 5]] = 0.0;

    BSIGHT[1] = 0.0;
    BSIGHT[2] = 0.0;
    BSIGHT[3] = 1.0;

    REFVEC[1] = 0.0;
    REFVEC[2] = 1.0;
    REFVEC[3] = 0.0;

    ANGLE = 60.0;
    CRSANG = 60.0;

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 12, BOUNDS.as_slice(), ctx)?;
    spicelib::PCPOOL(b"INS-1001_FOV_FRAME", 1, CharArray::from_ref(&FRAME), ctx)?;

    //
    // *****************************************************************
    //
    // Error cases: GETFVN
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the name of the instrument can not be translated to its NAIF ID code -- check GETFVN.", ctx)?;

    spicelib::GETFVN(
        &INSTNM,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the frame of the instrument has not been stored in the kernel pool -- check GETFVN.", ctx)?;

    fstr::assign(BUFFER.get_mut(1), b"NAIF_BODY_CODE += -1000");
    fstr::assign(BUFFER.get_mut(2), b"NAIF_BODY_NAME += \'CINSTR\'");
    spicelib::LMPOOL(BUFFER.as_arg(), BUFLEN, ctx)?;

    spicelib::GETFVN(
        &INSTNM,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(FRAMEMISSING)", OK, ctx)?;

    //
    // *****************************************************************
    //
    // Error cases: GETFOV
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the frame of the instrument has not been stored in the kernel pool -- check GETFOV", ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(FRAMEMISSING)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the shape of the instrument field of view is not in the kernel pool.", ctx)?;

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(b"INS-1001_FOV_SHAPE", 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 12, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SHAPEMISSING)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the shape specified is not one of the known shapes.", ctx)?;

    fstr::assign(&mut SHAPE, b"SQUARE");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 12, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SHAPENOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the boresight information has not been stored in the kernel pool.", ctx)?;

    fstr::assign(&mut SHAPE, b"POLYGON");
    fstr::assign(&mut KWBORE, b"INS-1001_BORESIGHT");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 12, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BORESIGHTMISSING)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Check that an exception is signaled if the boresight is not a 3-vector.",
        ctx,
    )?;

    fstr::assign(&mut KWBORE, b"INS-1000_BORESIGHT");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 2, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 12, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADBORESIGHTSPEC)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Check that an exception is signaled if the boresight is not a numeric 3-vector.",
        ctx,
    )?;

    fstr::assign(&mut KWBORE, b"INS-1000_BORESIGHT");
    fstr::assign(BOREST.get_mut(1), b"A");
    fstr::assign(BOREST.get_mut(2), b"B");
    fstr::assign(BOREST.get_mut(3), b"C");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PCPOOL(&KWBORE, 3, BOREST.as_arg(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 12, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADBORESIGHTSPEC)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Check that an exception is signaled if the boresight is the zero vector.",
        ctx,
    )?;

    fstr::assign(&mut KWBORE, b"INS-1000_BORESIGHT");

    TMPVEC[1] = 0.0;
    TMPVEC[2] = 0.0;
    TMPVEC[3] = 0.0;

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, TMPVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 12, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        4,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROBORESIGHT)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Check that an exception is signaled if the FOV class specification is not supported.",
        ctx,
    )?;

    fstr::assign(&mut SPEC, b"UNKNOWN_SPEC");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 12, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(UNSUPPORTEDSPEC)", OK, ctx)?;

    //
    // The following test cases cover the 'CORNERS' specification of
    // FOV boundaries. Note that, the FOV class specification keyword
    // is not present, GETFOV defaults to 'CORNERS', therefore we will
    // not provide that keyword.
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the boundary vectors of the field of view have not been stored in the kernel pool ", ctx)?;

    fstr::assign(&mut KWBOUN, b"INS-1001_FOV_BOUNDARY");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 12, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BOUNDARYMISSING)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if there are too many vectors in the array of corner vectors.", ctx)?;

    fstr::assign(&mut KWBOUN, b"INS-1000_FOV_BOUNDARY");
    ROOM = 2;
    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 12, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BOUNDARYTOOBIG)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the size of the array of boundary numbers is not a multiple of 3.", ctx)?;

    ROOM = 4;
    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 11, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADBOUNDARY)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the number of boundary vectors for a circular field of view is not 1.", ctx)?;

    fstr::assign(&mut SHAPE, b"CIRCLE");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 9, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADBOUNDARY)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the number of boundary vectors for a elliptical field of view is not 2.", ctx)?;

    fstr::assign(&mut SHAPE, b"ELLIPSE");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 9, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADBOUNDARY)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the number of boundary vectors for a rectangular field of view is not 4.", ctx)?;

    fstr::assign(&mut SHAPE, b"RECTANGLE");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 9, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADBOUNDARY)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the number of boundary vectors for a polygonal field of view is not at least 3.", ctx)?;

    fstr::assign(&mut SHAPE, b"POLYGON");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 6, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADBOUNDARY)", OK, ctx)?;

    //
    // The following test cases cover the 'ANGLES' specification of
    // FOV boundaries.
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the shape specified is not one of the supported shapes for the ANGLES specification.", ctx)?;

    fstr::assign(&mut SPEC, b"ANGLES");
    fstr::assign(&mut SHAPE, b"POLYGON");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SHAPENOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the FOV reference vector information has not been stored in the kernel pool.", ctx)?;

    fstr::assign(&mut SHAPE, b"RECTANGLE");
    fstr::assign(&mut KWRVEC, b"INS-1001_FOV_REF_VECTOR");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(REFVECTORMISSING)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Check that an exception is signaled if the FOV reference vector is not a 3-vector.",
        ctx,
    )?;

    fstr::assign(&mut KWRVEC, b"INS-1000_FOV_REF_VECTOR");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 2, REFVEC.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADREFVECTORSPEC)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the FOV reference vector is not a numeric 3-vector.", ctx)?;
    fstr::assign(BOREST.get_mut(1), b"A");
    fstr::assign(BOREST.get_mut(2), b"B");
    fstr::assign(BOREST.get_mut(3), b"C");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PCPOOL(&KWRVEC, 3, BOREST.as_arg(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADREFVECTORSPEC)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the FOV reference vector is parallel to the boresight vector.", ctx)?;

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, BSIGHT.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADREFVECTORSPEC)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the FOV reference angle has not been stored in the kernel pool.", ctx)?;

    fstr::assign(&mut KWRANG, b"INS-1001_FOV_REF_ANGLE");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(REFANGLEMISSING)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the FOV reference angle units have not been stored in the kernel pool.", ctx)?;

    fstr::assign(&mut KWRANG, b"INS-1000_FOV_REF_ANGLE");
    fstr::assign(&mut KWAUNT, b"INS-1001_FOV_ANGLE_UNITS");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;
    spicelib::PCPOOL(&KWAUNT, 1, CharArray::from_ref(&UNITS), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(UNITSMISSING)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the FOV reference angle units are not recognized by CONVRT.", ctx)?;

    fstr::assign(&mut KWAUNT, b"INS-1000_FOV_ANGLE_UNITS");
    fstr::assign(&mut UNITS, b"UNKNOWN");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;
    spicelib::PCPOOL(&KWAUNT, 1, CharArray::from_ref(&UNITS), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(UNITSNOTREC)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the FOV reference angle units are not angular units.", ctx)?;

    fstr::assign(&mut UNITS, b"KM");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;
    spicelib::PCPOOL(&KWAUNT, 1, CharArray::from_ref(&UNITS), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INCOMPATIBLEUNITS)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the available room for the boundary corner vectors is not enough for ANGLES/CIRCLE specification.", ctx)?;

    fstr::assign(&mut SHAPE, b"CIRCLE");
    fstr::assign(&mut UNITS, b"DEGREES");
    ROOM = 0;

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;
    spicelib::PCPOOL(&KWAUNT, 1, CharArray::from_ref(&UNITS), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BOUNDARYTOOBIG)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the FOV cross reference angle has not been stored in the kernel pool for ANGLES/ELLIPSE specification.", ctx)?;

    fstr::assign(&mut SHAPE, b"ELLIPSE");
    fstr::assign(&mut KWCANG, b"INS-1001_FOV_CROSS_ANGLE");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;
    spicelib::PCPOOL(&KWAUNT, 1, CharArray::from_ref(&UNITS), ctx)?;
    spicelib::PDPOOL(&KWCANG, 1, &[CRSANG], ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(CROSSANGLEMISSING)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the available room for the boundary corner vectors is not enough for ANGLESS/ELLIPSE specification.", ctx)?;

    fstr::assign(&mut KWCANG, b"INS-1000_FOV_CROSS_ANGLE");
    ROOM = 1;

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;
    spicelib::PCPOOL(&KWAUNT, 1, CharArray::from_ref(&UNITS), ctx)?;
    spicelib::PDPOOL(&KWCANG, 1, &[CRSANG], ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BOUNDARYTOOBIG)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the FOV cross reference angle has not been stored in the kernel pool for ANGLE/RECTANGLE specification.", ctx)?;

    fstr::assign(&mut SHAPE, b"RECTANGLE");
    fstr::assign(&mut KWCANG, b"INS-1001_FOV_CROSS_ANGLE");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;
    spicelib::PCPOOL(&KWAUNT, 1, CharArray::from_ref(&UNITS), ctx)?;
    spicelib::PDPOOL(&KWCANG, 1, &[CRSANG], ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(CROSSANGLEMISSING)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the available room for the boundary corner vectors is not enough for ANGLES/RECTANGLE specification.", ctx)?;

    fstr::assign(&mut KWCANG, b"INS-1000_FOV_CROSS_ANGLE");
    ROOM = 3;

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;
    spicelib::PCPOOL(&KWAUNT, 1, CharArray::from_ref(&UNITS), ctx)?;
    spicelib::PDPOOL(&KWCANG, 1, &[CRSANG], ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BOUNDARYTOOBIG)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the FOV definitions for ANGLES/RECTANGLE specification result in a degenerate case due to reference angle being ~90.0 degrees.", ctx)?;

    ROOM = 4;
    ANGLE = f64::acos(0.0000000000000009);
    fstr::assign(&mut UNITS, b"RADIANS");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;
    spicelib::PCPOOL(&KWAUNT, 1, CharArray::from_ref(&UNITS), ctx)?;
    spicelib::PDPOOL(&KWCANG, 1, &[CRSANG], ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADBOUNDARY)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that an exception is signaled if the FOV definitions for ANGLES/RECTANGLE specification result in a degenerate case due to cross angle being ~90.0 degrees.", ctx)?;

    ROOM = 4;
    ANGLE = 1.0;
    CRSANG = f64::acos(0.0000000000000009);
    fstr::assign(&mut UNITS, b"RADIANS");

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;
    spicelib::PCPOOL(&KWAUNT, 1, CharArray::from_ref(&UNITS), ctx)?;
    spicelib::PDPOOL(&KWCANG, 1, &[CRSANG], ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADBOUNDARY)", OK, ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: GETFVN
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Check that we can get back a polygonal field of view -- check GETFVN.",
        ctx,
    )?;

    fstr::assign(&mut SHAPE, b"POLYGON");
    ROOM = 15;

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 9, BOUNDS.as_slice(), ctx)?;

    fstr::assign(BUFFER.get_mut(1), b"NAIF_BODY_CODE += -1000");
    fstr::assign(BUFFER.get_mut(2), b"NAIF_BODY_NAME += \'CINSTR\'");
    spicelib::LMPOOL(BUFFER.as_arg(), BUFLEN, ctx)?;

    spicelib::GETFVN(
        &INSTNM,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SHAPE", &FOVSHP, b"=", &SHAPE, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FOVFRM, b"=", &FRAME, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"FOVBS",
        FOVBS.as_slice(),
        b"=",
        BSIGHT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"FOVBND",
        FOVBND.as_slice(),
        b"=",
        BOUNDS.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Check that we can get back a elliptical field of view -- check GETFVN.",
        ctx,
    )?;

    fstr::assign(&mut SHAPE, b"ELLIPSE");
    ROOM = 2;

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 6, BOUNDS.as_slice(), ctx)?;

    fstr::assign(BUFFER.get_mut(1), b"NAIF_BODY_CODE += -1000");
    fstr::assign(BUFFER.get_mut(2), b"NAIF_BODY_NAME += \'CINSTR\'");
    spicelib::LMPOOL(BUFFER.as_arg(), BUFLEN, ctx)?;

    spicelib::GETFVN(
        &INSTNM,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SHAPE", &FOVSHP, b"=", &SHAPE, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FOVFRM, b"=", &FRAME, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 2, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"FOVBS",
        FOVBS.as_slice(),
        b"=",
        BSIGHT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"FOVBND",
        FOVBND.as_slice(),
        b"=",
        BOUNDS.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;

    //
    // *****************************************************************
    //
    // Normal cases: GETFOV
    //
    // *****************************************************************
    //
    // The following test cases cover the 'CORNERS' specification of
    // FOV boundaries. Note that, the FOV class specification keyword
    // is not present, GETFOV defaults to 'CORNERS', therefore we will
    // not provide that keyword.
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Check that we can get back a rectangular field of view.",
        ctx,
    )?;

    fstr::assign(&mut SHAPE, b"RECTANGLE");
    ROOM = 4;

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 12, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SHAPE", &FOVSHP, b"=", &SHAPE, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FOVFRM, b"=", &FRAME, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 4, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"FOVBS",
        FOVBS.as_slice(),
        b"=",
        BSIGHT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"FOVBND",
        FOVBND.as_slice(),
        b"=",
        BOUNDS.as_slice(),
        12,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that we can get back a circular field of view.", ctx)?;

    fstr::assign(&mut SHAPE, b"CIRCLE");
    ROOM = 1;

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 3, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SHAPE", &FOVSHP, b"=", &SHAPE, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FOVFRM, b"=", &FRAME, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"FOVBS",
        FOVBS.as_slice(),
        b"=",
        BSIGHT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"FOVBND",
        FOVBND.as_slice(),
        b"=",
        BOUNDS.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Check that we can get back a elliptical field of view.",
        ctx,
    )?;

    fstr::assign(&mut SHAPE, b"ELLIPSE");
    ROOM = 2;

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 6, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SHAPE", &FOVSHP, b"=", &SHAPE, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FOVFRM, b"=", &FRAME, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 2, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"FOVBS",
        FOVBS.as_slice(),
        b"=",
        BSIGHT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"FOVBND",
        FOVBND.as_slice(),
        b"=",
        BOUNDS.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Check that we can get back a polygonal field of view.",
        ctx,
    )?;

    fstr::assign(&mut SHAPE, b"POLYGON");
    ROOM = 15;

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWBOUN, 9, BOUNDS.as_slice(), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SHAPE", &FOVSHP, b"=", &SHAPE, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FOVFRM, b"=", &FRAME, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"FOVBS",
        FOVBS.as_slice(),
        b"=",
        BSIGHT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"FOVBND",
        FOVBND.as_slice(),
        b"=",
        BOUNDS.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    //
    // The following test cases cover the 'ANGLES' specification of
    // FOV boundaries.
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Check that we can get back a circular field of view from a CIRCLE/ANGLES specification.",
        ctx,
    )?;

    fstr::assign(&mut SHAPE, b"CIRCLE");
    ROOM = 1;
    fstr::assign(&mut UNITS, b"RADIANS");
    ANGLE = 1.0;

    BOUNDS[[1, 1]] = 0.0;
    BOUNDS[[2, 1]] = f64::sin(ANGLE);
    BOUNDS[[3, 1]] = f64::cos(ANGLE);

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;
    spicelib::PCPOOL(&KWAUNT, 1, CharArray::from_ref(&UNITS), ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SHAPE", &FOVSHP, b"=", &SHAPE, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FOVFRM, b"=", &FRAME, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"FOVBS",
        FOVBS.as_slice(),
        b"=",
        BSIGHT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"FOVBND",
        FOVBND.as_slice(),
        b"=",
        BOUNDS.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that we can get back a ellipsoidal field of view from a ELLIPSE/ANGLES specification.", ctx)?;

    fstr::assign(&mut SHAPE, b"ELLIPSE");
    ROOM = 2;
    fstr::assign(&mut UNITS, b"RADIANS");
    ANGLE = 1.0;
    CRSANG = 0.5;

    BOUNDS[[1, 1]] = 0.0;
    BOUNDS[[2, 1]] = f64::sin(ANGLE);
    BOUNDS[[3, 1]] = f64::cos(ANGLE);

    BOUNDS[[1, 2]] = -f64::sin(CRSANG);
    BOUNDS[[2, 2]] = 0.0;
    BOUNDS[[3, 2]] = f64::cos(CRSANG);

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;
    spicelib::PCPOOL(&KWAUNT, 1, CharArray::from_ref(&UNITS), ctx)?;
    spicelib::PDPOOL(&KWCANG, 1, &[CRSANG], ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SHAPE", &FOVSHP, b"=", &SHAPE, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FOVFRM, b"=", &FRAME, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 2, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"FOVBS",
        FOVBS.as_slice(),
        b"=",
        BSIGHT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"FOVBND",
        FOVBND.as_slice(),
        b"=",
        BOUNDS.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that we can get back a rectangular field of view from a RECTANGLE/ANGLES specification.", ctx)?;

    fstr::assign(&mut SHAPE, b"RECTANGLE");
    ROOM = 4;
    fstr::assign(&mut UNITS, b"RADIANS");
    ANGLE = 0.1;
    CRSANG = 0.05;

    TMPVEC[1] = -(f64::cos(ANGLE) * f64::sin(CRSANG));
    TMPVEC[2] = (f64::sin(ANGLE) * f64::cos(CRSANG));
    TMPVEC[3] = (f64::cos(ANGLE) * f64::cos(CRSANG));

    spicelib::VHAT(TMPVEC.as_slice(), BOUNDS.subarray_mut([1, 1]));

    TMPVEC[1] = -(f64::cos(ANGLE) * f64::sin(CRSANG));
    TMPVEC[2] = -(f64::sin(ANGLE) * f64::cos(CRSANG));
    TMPVEC[3] = (f64::cos(ANGLE) * f64::cos(CRSANG));

    spicelib::VHAT(TMPVEC.as_slice(), BOUNDS.subarray_mut([1, 2]));

    TMPVEC[1] = (f64::cos(ANGLE) * f64::sin(CRSANG));
    TMPVEC[2] = -(f64::sin(ANGLE) * f64::cos(CRSANG));
    TMPVEC[3] = (f64::cos(ANGLE) * f64::cos(CRSANG));

    spicelib::VHAT(TMPVEC.as_slice(), BOUNDS.subarray_mut([1, 3]));

    TMPVEC[1] = (f64::cos(ANGLE) * f64::sin(CRSANG));
    TMPVEC[2] = (f64::sin(ANGLE) * f64::cos(CRSANG));
    TMPVEC[3] = (f64::cos(ANGLE) * f64::cos(CRSANG));

    spicelib::VHAT(TMPVEC.as_slice(), BOUNDS.subarray_mut([1, 4]));

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;
    spicelib::PCPOOL(&KWAUNT, 1, CharArray::from_ref(&UNITS), ctx)?;
    spicelib::PDPOOL(&KWCANG, 1, &[CRSANG], ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SHAPE", &FOVSHP, b"=", &SHAPE, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FOVFRM, b"=", &FRAME, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 4, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"FOVBS",
        FOVBS.as_slice(),
        b"=",
        BSIGHT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"FOVBND",
        FOVBND.as_slice(),
        b"~",
        BOUNDS.as_slice(),
        12,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that we can get back a rectangular field of view from a RECTANGLE/ANGLES specification with reference angle with cosines close to MINCOS.", ctx)?;

    fstr::assign(&mut SHAPE, b"RECTANGLE");
    ROOM = 4;
    fstr::assign(&mut UNITS, b"RADIANS");
    ANGLE = (spicelib::HALFPI(ctx) - MINCOS);
    CRSANG = 0.1;

    TMPVEC[1] = -(f64::cos(ANGLE) * f64::sin(CRSANG));
    TMPVEC[2] = (f64::sin(ANGLE) * f64::cos(CRSANG));
    TMPVEC[3] = (f64::cos(ANGLE) * f64::cos(CRSANG));

    spicelib::VHAT(TMPVEC.as_slice(), BOUNDS.subarray_mut([1, 1]));

    TMPVEC[1] = -(f64::cos(ANGLE) * f64::sin(CRSANG));
    TMPVEC[2] = -(f64::sin(ANGLE) * f64::cos(CRSANG));
    TMPVEC[3] = (f64::cos(ANGLE) * f64::cos(CRSANG));

    spicelib::VHAT(TMPVEC.as_slice(), BOUNDS.subarray_mut([1, 2]));

    TMPVEC[1] = (f64::cos(ANGLE) * f64::sin(CRSANG));
    TMPVEC[2] = -(f64::sin(ANGLE) * f64::cos(CRSANG));
    TMPVEC[3] = (f64::cos(ANGLE) * f64::cos(CRSANG));

    spicelib::VHAT(TMPVEC.as_slice(), BOUNDS.subarray_mut([1, 3]));

    TMPVEC[1] = (f64::cos(ANGLE) * f64::sin(CRSANG));
    TMPVEC[2] = (f64::sin(ANGLE) * f64::cos(CRSANG));
    TMPVEC[3] = (f64::cos(ANGLE) * f64::cos(CRSANG));

    spicelib::VHAT(TMPVEC.as_slice(), BOUNDS.subarray_mut([1, 4]));

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;
    spicelib::PCPOOL(&KWAUNT, 1, CharArray::from_ref(&UNITS), ctx)?;
    spicelib::PDPOOL(&KWCANG, 1, &[CRSANG], ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SHAPE", &FOVSHP, b"=", &SHAPE, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FOVFRM, b"=", &FRAME, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 4, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"FOVBS",
        FOVBS.as_slice(),
        b"=",
        BSIGHT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"FOVBND",
        FOVBND.as_slice(),
        b"~",
        BOUNDS.as_slice(),
        12,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that we can get back a rectangular field of view from a RECTANGLE/ANGLES specification with cross angle with cosines close to MINCOS.", ctx)?;

    fstr::assign(&mut SHAPE, b"RECTANGLE");
    ROOM = 4;
    fstr::assign(&mut UNITS, b"RADIANS");
    ANGLE = 0.2;
    CRSANG = (spicelib::HALFPI(ctx) - MINCOS);

    TMPVEC[1] = -(f64::cos(ANGLE) * f64::sin(CRSANG));
    TMPVEC[2] = (f64::sin(ANGLE) * f64::cos(CRSANG));
    TMPVEC[3] = (f64::cos(ANGLE) * f64::cos(CRSANG));

    spicelib::VHAT(TMPVEC.as_slice(), BOUNDS.subarray_mut([1, 1]));

    TMPVEC[1] = -(f64::cos(ANGLE) * f64::sin(CRSANG));
    TMPVEC[2] = -(f64::sin(ANGLE) * f64::cos(CRSANG));
    TMPVEC[3] = (f64::cos(ANGLE) * f64::cos(CRSANG));

    spicelib::VHAT(TMPVEC.as_slice(), BOUNDS.subarray_mut([1, 2]));

    TMPVEC[1] = (f64::cos(ANGLE) * f64::sin(CRSANG));
    TMPVEC[2] = -(f64::sin(ANGLE) * f64::cos(CRSANG));
    TMPVEC[3] = (f64::cos(ANGLE) * f64::cos(CRSANG));

    spicelib::VHAT(TMPVEC.as_slice(), BOUNDS.subarray_mut([1, 3]));

    TMPVEC[1] = (f64::cos(ANGLE) * f64::sin(CRSANG));
    TMPVEC[2] = (f64::sin(ANGLE) * f64::cos(CRSANG));
    TMPVEC[3] = (f64::cos(ANGLE) * f64::cos(CRSANG));

    spicelib::VHAT(TMPVEC.as_slice(), BOUNDS.subarray_mut([1, 4]));

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;
    spicelib::PCPOOL(&KWAUNT, 1, CharArray::from_ref(&UNITS), ctx)?;
    spicelib::PDPOOL(&KWCANG, 1, &[CRSANG], ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SHAPE", &FOVSHP, b"=", &SHAPE, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FOVFRM, b"=", &FRAME, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 4, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"FOVBS",
        FOVBS.as_slice(),
        b"=",
        BSIGHT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"FOVBND",
        FOVBND.as_slice(),
        b"~",
        BOUNDS.as_slice(),
        12,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that we can get back a rectangular field of view from a RECTANGLE/ANGLES specification with both reference and cross angle with cosines close to  MINCOS.", ctx)?;

    fstr::assign(&mut SHAPE, b"RECTANGLE");
    ROOM = 4;
    fstr::assign(&mut UNITS, b"RADIANS");
    ANGLE = (spicelib::HALFPI(ctx) - MINCOS);
    CRSANG = (spicelib::HALFPI(ctx) - MINCOS);

    TMPVEC[1] = -(f64::cos(ANGLE) * f64::sin(CRSANG));
    TMPVEC[2] = (f64::sin(ANGLE) * f64::cos(CRSANG));
    TMPVEC[3] = (f64::cos(ANGLE) * f64::cos(CRSANG));

    spicelib::VHAT(TMPVEC.as_slice(), BOUNDS.subarray_mut([1, 1]));

    TMPVEC[1] = -(f64::cos(ANGLE) * f64::sin(CRSANG));
    TMPVEC[2] = -(f64::sin(ANGLE) * f64::cos(CRSANG));
    TMPVEC[3] = (f64::cos(ANGLE) * f64::cos(CRSANG));

    spicelib::VHAT(TMPVEC.as_slice(), BOUNDS.subarray_mut([1, 2]));

    TMPVEC[1] = (f64::cos(ANGLE) * f64::sin(CRSANG));
    TMPVEC[2] = -(f64::sin(ANGLE) * f64::cos(CRSANG));
    TMPVEC[3] = (f64::cos(ANGLE) * f64::cos(CRSANG));

    spicelib::VHAT(TMPVEC.as_slice(), BOUNDS.subarray_mut([1, 3]));

    TMPVEC[1] = (f64::cos(ANGLE) * f64::sin(CRSANG));
    TMPVEC[2] = (f64::sin(ANGLE) * f64::cos(CRSANG));
    TMPVEC[3] = (f64::cos(ANGLE) * f64::cos(CRSANG));

    spicelib::VHAT(TMPVEC.as_slice(), BOUNDS.subarray_mut([1, 4]));

    spicelib::CLPOOL(ctx)?;
    spicelib::PCPOOL(&KWFRAM, 1, CharArray::from_ref(&FRAME), ctx)?;
    spicelib::PCPOOL(&KWSPEC, 1, CharArray::from_ref(&SPEC), ctx)?;
    spicelib::PCPOOL(&KWSHAP, 1, CharArray::from_ref(&SHAPE), ctx)?;
    spicelib::PDPOOL(&KWBORE, 3, BSIGHT.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRVEC, 3, REFVEC.as_slice(), ctx)?;
    spicelib::PDPOOL(&KWRANG, 1, &[ANGLE], ctx)?;
    spicelib::PCPOOL(&KWAUNT, 1, CharArray::from_ref(&UNITS), ctx)?;
    spicelib::PDPOOL(&KWCANG, 1, &[CRSANG], ctx)?;

    spicelib::GETFOV(
        INSTID,
        ROOM,
        &mut FOVSHP,
        &mut FOVFRM,
        FOVBS.as_slice_mut(),
        &mut N,
        FOVBND.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SHAPE", &FOVSHP, b"=", &SHAPE, OK, ctx)?;
    testutil::CHCKSC(b"FRAME", &FOVFRM, b"=", &FRAME, OK, ctx)?;
    testutil::CHCKSI(b"N", N, b"=", 4, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"FOVBS",
        FOVBS.as_slice(),
        b"=",
        BSIGHT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"FOVBND",
        FOVBND.as_slice(),
        b"~",
        BOUNDS.as_slice(),
        12,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Leave the kernel pool clean.
    //
    spicelib::CLPOOL(ctx)?;

    //
    // Retrieve the current test status.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
