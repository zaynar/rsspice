//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXSRF: i32 = 100;
const SRFIDX: i32 = 1;
const CTRIDX: i32 = (SRFIDX + 1);
const CLSIDX: i32 = (CTRIDX + 1);
const TYPIDX: i32 = (CLSIDX + 1);
const FRMIDX: i32 = (TYPIDX + 1);
const SYSIDX: i32 = (FRMIDX + 1);
const PARIDX: i32 = (SYSIDX + 1);
const NSYPAR: i32 = 10;
const MN1IDX: i32 = (PARIDX + NSYPAR);
const MX1IDX: i32 = (MN1IDX + 1);
const MN2IDX: i32 = (MX1IDX + 1);
const MX2IDX: i32 = (MN2IDX + 1);
const MN3IDX: i32 = (MX2IDX + 1);
const MX3IDX: i32 = (MN3IDX + 1);
const BTMIDX: i32 = (MX3IDX + 1);
const ETMIDX: i32 = (BTMIDX + 1);
const DSKDSZ: i32 = ETMIDX;
const SVFCLS: i32 = 1;
const GENCLS: i32 = 2;
const LATSYS: i32 = 1;
const CYLSYS: i32 = 2;
const RECSYS: i32 = 3;
const PDTSYS: i32 = 4;
const MARGIN: f64 = 0.000000000001;

struct SaveVars {
    PI2: f64,
    SAVCO1: f64,
    SAVCO2: f64,
    SAVET: f64,
    SAVPAR: StackArray<f64, 10>,
    SAVPOS: StackArray<f64, 3>,
    SAVBID: i32,
    SAVCLS: i32,
    SAVFID: i32,
    SAVNSF: i32,
    SAVSRF: StackArray<i32, 100>,
    SAVSYS: i32,
    SAVTRG: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PI2: f64 = 0.0;
        let mut SAVCO1: f64 = 0.0;
        let mut SAVCO2: f64 = 0.0;
        let mut SAVET: f64 = 0.0;
        let mut SAVPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
        let mut SAVPOS = StackArray::<f64, 3>::new(1..=3);
        let mut SAVBID: i32 = 0;
        let mut SAVCLS: i32 = 0;
        let mut SAVFID: i32 = 0;
        let mut SAVNSF: i32 = 0;
        let mut SAVSRF = StackArray::<i32, 100>::new(1..=MAXSRF);
        let mut SAVSYS: i32 = 0;
        let mut SAVTRG: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            PI2,
            SAVCO1,
            SAVCO2,
            SAVET,
            SAVPAR,
            SAVPOS,
            SAVBID,
            SAVCLS,
            SAVFID,
            SAVNSF,
            SAVSRF,
            SAVSYS,
            SAVTRG,
            FIRST,
        }
    }
}

//$Procedure ZZDSKSEL ( DSK, segment selection callback umbrella )
pub fn ZZDSKSEL(
    SURFID: i32,
    NSURF: i32,
    SRFLST: &[i32],
    BODYID: i32,
    DCLASS: i32,
    CORSYS: i32,
    CORPAR: &[f64],
    COR1: f64,
    COR2: f64,
    FRAMID: i32,
    POS: &[f64],
    ET: f64,
    HANDLE: i32,
    DLADSC: &[i32],
    DSKDSC: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let DLADSC = DummyArray::new(DLADSC, 1..);
    let mut ZZDSKSEL: bool = false;
    let mut I: i32 = 0;

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

    //
    // Initial values
    //

    //
    // Set return value.
    //
    ZZDSKSEL = false;

    //
    // The following no-op calls are used to suppress compiler
    // warnings.
    //
    I = TOUCHI(HANDLE);
    I = TOUCHI(CORSYS);
    I = TOUCHI(*DLADSC.first());
    I = TOUCHI(FRAMID);

    CHKIN(b"ZZDSKSEL", ctx)?;
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZDSKSEL", ctx)?;
    Ok(ZZDSKSEL)
}

//$Procedure ZZDSKSBD ( DSK, set body ID )
pub fn ZZDSKSBD(BODYID: i32, ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ZZDSKSBD: bool = false;

    //
    // Set the function's return value. This value has no
    // particular meaning and is not meant to be used by
    // the caller. This setting suppresses compiler warnings.
    //
    ZZDSKSBD = false;
    //
    // Save input value.
    //
    save.SAVBID = BODYID;

    ZZDSKSBD
}

//$Procedure ZZDSKBDC ( DSK, check segment's body ID )
pub fn ZZDSKBDC(HANDLE: i32, DLADSC: &[i32], DSKDSC: &[f64], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DSKDSC = DummyArray::new(DSKDSC, 1..);
    let mut ZZDSKBDC: bool = false;

    //
    // The body ID is the only DSK segment attribute that must match.
    //
    ZZDSKBDC = (intrinsics::IDNINT(DSKDSC[CTRIDX]) == save.SAVBID);

    ZZDSKBDC
}

//$Procedure ZZDSKNOT ( DSK, match nothing )
pub fn ZZDSKNOT(HANDLE: i32, DLADSC: &[i32], DSKDSC: &[f64], ctx: &mut Context) -> bool {
    let mut ZZDSKNOT: bool = false;

    //
    // Whatever's in this segment, it doesn't match.
    //
    ZZDSKNOT = false;

    ZZDSKNOT
}

//$Procedure ZZDSKSIT ( DSK, set body ID, surfaces, and time )
pub fn ZZDSKSIT(
    BODYID: i32,
    NSURF: i32,
    SRFLST: &[i32],
    ET: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SRFLST = DummyArray::new(SRFLST, 1..);
    let mut ZZDSKSIT: bool = false;
    let mut I: i32 = 0;

    //
    // Set return value.
    //
    ZZDSKSIT = false;
    //
    // Save input values.
    //
    save.SAVBID = BODYID;
    save.SAVET = ET;

    if (NSURF > MAXSRF) {
        CHKIN(b"ZZDSKSIT", ctx)?;

        SETMSG(
            b"Maximum allowed surface ID count is #; input count was #.",
            ctx,
        );
        ERRINT(b"#", MAXSRF, ctx);
        ERRINT(b"#", NSURF, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZDSKSIT", ctx)?;
        return Ok(ZZDSKSIT);
    }

    save.SAVNSF = NSURF;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NSURF;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SAVSRF[I] = SRFLST[I];
            I += m3__;
        }
    }

    SHELLI(NSURF, save.SAVSRF.as_slice_mut());

    Ok(ZZDSKSIT)
}

//$Procedure ZZDSKCIT ( DSK, check body ID, surface, and time )
pub fn ZZDSKCIT(HANDLE: i32, DLADSC: &[i32], DSKDSC: &[f64], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DSKDSC = DummyArray::new(DSKDSC, 1..);
    let mut ZZDSKCIT: bool = false;
    let mut SURF: i32 = 0;

    //
    // Currently, we don't need to access the DSK file in order
    // to make this test, but the required inputs are available.
    //
    ZZDSKCIT = false;

    if (save.SAVBID == intrinsics::IDNINT(DSKDSC[CTRIDX])) {
        if ((save.SAVET >= DSKDSC[BTMIDX]) && (save.SAVET <= DSKDSC[ETMIDX])) {
            if (save.SAVNSF < 1) {
                //
                // There are no surface ID constraints; we have
                // a match.
                //
                ZZDSKCIT = true;
            } else {
                //
                // We have a match if and only if the surface ID of this
                // segment is on the list of allowed surface IDs.
                //
                SURF = intrinsics::IDNINT(DSKDSC[SRFIDX]);

                ZZDSKCIT = (BSRCHI(SURF, save.SAVNSF, save.SAVSRF.as_slice()) > 0);
            }
        }
    }

    ZZDSKCIT
}

//$Procedure ZZDSKUSC ( DSK, set body ID, coordinates, and time )
pub fn ZZDSKUSC(BODYID: i32, ET: f64, COR1: f64, COR2: f64, ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ZZDSKUSC: bool = false;

    ZZDSKUSC = false;

    save.SAVBID = BODYID;
    save.SAVET = ET;
    save.SAVCO1 = COR1;
    save.SAVCO2 = COR2;

    ZZDSKUSC
}

//$Procedure ZZDSKUMC ( DSK, check body ID, coordinates, and time )
pub fn ZZDSKUMC(HANDLE: i32, DLADSC: &[i32], DSKDSC: &[f64], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DSKDSC = DummyArray::new(DSKDSC, 1..);
    let mut ZZDSKUMC: bool = false;
    let mut CO1MAX: f64 = 0.0;
    let mut CO1MIN: f64 = 0.0;
    let mut CO2MAX: f64 = 0.0;
    let mut CO2MIN: f64 = 0.0;
    let mut LOCCOR = StackArray::<f64, 1>::new(1..=1);
    let mut SCALE: f64 = 0.0;
    let mut SEGSYS: i32 = 0;

    //
    // We don't have a match to begin with.
    //
    ZZDSKUMC = false;

    if save.FIRST {
        save.PI2 = TWOPI(ctx);
        save.FIRST = false;
    }

    //
    // Check the body ID first.
    //
    if (save.SAVBID == intrinsics::IDNINT(DSKDSC[CTRIDX])) {
        //
        // The body ID matches; check the time coverage.
        //
        if ((save.SAVET >= DSKDSC[BTMIDX]) && (save.SAVET <= DSKDSC[ETMIDX])) {
            //
            // Check the coordinates. Note that we don't
            // know whether the frame and coordinates are
            // reasonable. It's up to the user to ensure this.
            //
            // We do need to know whether the first coordinate
            // is a longitude, since we may need to adjust it to
            // get it into the range of the segment.
            //
            SEGSYS = intrinsics::IDNINT(DSKDSC[SYSIDX]);
            //
            // Make a local copy of the first coordinate.
            //
            LOCCOR[1] = save.SAVCO1;

            if ((SEGSYS == LATSYS) || (SEGSYS == PDTSYS)) {
                //
                // Adjust segment bounds using a small margin.
                //
                CO1MIN = (DSKDSC[MN1IDX] - MARGIN);
                CO1MAX = (DSKDSC[MX1IDX] + MARGIN);
                CO2MIN = (DSKDSC[MN2IDX] - MARGIN);
                CO2MAX = (DSKDSC[MX2IDX] + MARGIN);
                //
                // Move longitude into range.
                //
                if (LOCCOR[1] < CO1MIN) {
                    LOCCOR[1] = (LOCCOR[1] + save.PI2);
                } else if (LOCCOR[1] > CO1MAX) {
                    LOCCOR[1] = (LOCCOR[1] - save.PI2);
                }
            } else {
                SCALE = (1.0 + MARGIN);

                CO1MIN = (DSKDSC[MN1IDX] - (SCALE * f64::abs(DSKDSC[MN1IDX])));
                CO1MAX = (DSKDSC[MX1IDX] + (SCALE * f64::abs(DSKDSC[MX1IDX])));
                CO2MIN = (DSKDSC[MN2IDX] - (SCALE * f64::abs(DSKDSC[MN2IDX])));
                CO2MAX = (DSKDSC[MX2IDX] + (SCALE * f64::abs(DSKDSC[MX2IDX])));
            }

            //
            // Check the first coordinate against the segment's
            // corresponding coverage range.
            //
            if ((LOCCOR[1] < CO1MIN) || (LOCCOR[1] > CO1MAX)) {
                //
                // The first input coordinate is not covered by this
                // segment.
                //
                return ZZDSKUMC;
            }

            //
            // Compare the second coordinate against the segment's
            // corresponding coverage range.
            //
            if ((save.SAVCO2 < CO2MIN) || (save.SAVCO2 > CO2MAX)) {
                //
                // The second input coordinate is not covered by this
                // segment.
                //
                return ZZDSKUMC;
            }

            //
            // At this point we have a match.
            //
            ZZDSKUMC = true;
        }
        //
        // This is the end of the time check block.
        //
    }
    //
    // This is the end of the surface ID check block.
    //
    ZZDSKUMC
}

//$Procedure ZZDSKMSC ( DSK, setup matched attribute search )
pub fn ZZDSKMSC(
    BODYID: i32,
    SURFID: i32,
    FRAMID: i32,
    CORSYS: i32,
    CORPAR: &[f64],
    ET: f64,
    COR1: f64,
    COR2: f64,
    ctx: &mut Context,
) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let CORPAR = DummyArray::new(CORPAR, 1..);
    let mut ZZDSKMSC: bool = false;

    ZZDSKMSC = false;

    save.SAVTRG = BODYID;
    save.SAVBID = SURFID;
    save.SAVFID = FRAMID;
    save.SAVSYS = CORSYS;

    MOVED(CORPAR.as_slice(), NSYPAR, save.SAVPAR.as_slice_mut());

    save.SAVET = ET;
    save.SAVCO1 = COR1;
    save.SAVCO2 = COR2;

    ZZDSKMSC
}

//$Procedure ZZDSKMMC ( DSK, matched segment attribute comparison )
pub fn ZZDSKMMC(HANDLE: i32, DLADSC: &[i32], DSKDSC: &[f64], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DSKDSC = DummyArray::new(DSKDSC, 1..);
    let mut ZZDSKMMC: bool = false;
    let mut CO1MAX: f64 = 0.0;
    let mut CO1MIN: f64 = 0.0;
    let mut CO2MAX: f64 = 0.0;
    let mut CO2MIN: f64 = 0.0;
    let mut LOCCOR = StackArray::<f64, 1>::new(1..=1);
    let mut SCALE: f64 = 0.0;
    let mut SEGSYS: i32 = 0;

    //
    // We don't have a match to begin with.
    //
    ZZDSKMMC = false;

    if save.FIRST {
        save.PI2 = TWOPI(ctx);
        save.FIRST = false;
    }

    //
    // Check the target ID.
    //
    if (save.SAVTRG != intrinsics::IDNINT(DSKDSC[CTRIDX])) {
        return ZZDSKMMC;
    }

    //
    // Check the surface ID.
    //
    if (save.SAVBID != intrinsics::IDNINT(DSKDSC[SRFIDX])) {
        return ZZDSKMMC;
    }

    //
    // Check the frame ID.
    //
    if (save.SAVFID != intrinsics::IDNINT(DSKDSC[FRMIDX])) {
        return ZZDSKMMC;
    }

    //
    // Check the coordinate system.
    //
    if (save.SAVSYS != intrinsics::IDNINT(DSKDSC[SYSIDX])) {
        return ZZDSKMMC;
    }

    //
    // If the system is planetodetic, check the reference
    // ellipsoid parameters.
    //
    if (save.SAVSYS == PDTSYS) {
        if (f64::abs((save.SAVPAR[1] - DSKDSC[PARIDX])) > MARGIN) {
            return ZZDSKMMC;
        }

        if (f64::abs((save.SAVPAR[2] - DSKDSC[(PARIDX + 1)])) > MARGIN) {
            return ZZDSKMMC;
        }
    }

    //
    // The segment attributes match; check the time coverage.
    //
    if ((save.SAVET >= DSKDSC[BTMIDX]) && (save.SAVET <= DSKDSC[ETMIDX])) {
        //
        // Check the coordinates.
        //
        SEGSYS = intrinsics::IDNINT(DSKDSC[SYSIDX]);
        //
        // Make a local copy of the first coordinate.
        //
        LOCCOR[1] = save.SAVCO1;

        if ((SEGSYS == LATSYS) || (SEGSYS == PDTSYS)) {
            //
            // Adjust segment bounds using a small margin.
            //
            CO1MIN = (DSKDSC[MN1IDX] - MARGIN);
            CO1MAX = (DSKDSC[MX1IDX] + MARGIN);
            CO2MIN = (DSKDSC[MN2IDX] - MARGIN);
            CO2MAX = (DSKDSC[MX2IDX] + MARGIN);

            //
            // Move longitude into range.
            //
            if (LOCCOR[1] < CO1MIN) {
                LOCCOR[1] = (LOCCOR[1] + save.PI2);
            } else if (LOCCOR[1] > CO1MAX) {
                LOCCOR[1] = (LOCCOR[1] - save.PI2);
            }
        } else {
            SCALE = (1.0 + MARGIN);

            CO1MIN = (DSKDSC[MN1IDX] - (SCALE * f64::abs(DSKDSC[MN1IDX])));
            CO1MAX = (DSKDSC[MX1IDX] + (SCALE * f64::abs(DSKDSC[MX1IDX])));
            CO2MIN = (DSKDSC[MN2IDX] - (SCALE * f64::abs(DSKDSC[MN2IDX])));
            CO2MAX = (DSKDSC[MX2IDX] + (SCALE * f64::abs(DSKDSC[MX2IDX])));
        }

        //
        // Check the first coordinate against the segment's
        // corresponding coverage range.
        //
        if ((LOCCOR[1] < CO1MIN) || (LOCCOR[1] > CO1MAX)) {
            //
            // The first input coordinate is not covered by this
            // segment.
            //
            return ZZDSKMMC;
        }

        //
        // Compare the second coordinate against the segment's
        // corresponding coverage range.
        //
        if ((save.SAVCO2 < CO2MIN) || (save.SAVCO2 > CO2MAX)) {
            //
            // The second input coordinate is not covered by this
            // segment.
            //
            return ZZDSKMMC;
        }

        //
        // At this point we have a match.
        //
        ZZDSKMMC = true;
    }

    ZZDSKMMC
}

//$Procedure ZZDSKSRC ( DSK, setup rectangular coordinate search )
pub fn ZZDSKSRC(
    SURFID: i32,
    BODYID: i32,
    DCLASS: i32,
    ET: f64,
    FRAMID: i32,
    POS: &[f64],
    ctx: &mut Context,
) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let POS = DummyArray::new(POS, 1..=3);
    let mut ZZDSKSRC: bool = false;

    ZZDSKSRC = false;

    save.SAVBID = SURFID;
    save.SAVTRG = BODYID;
    save.SAVCLS = DCLASS;
    save.SAVET = ET;
    save.SAVFID = FRAMID;
    VEQU(POS.as_slice(), save.SAVPOS.as_slice_mut());

    ZZDSKSRC
}

//$Procedure ZZDSKMRC ( DSK, rectangular coordinate comparison )
pub fn ZZDSKMRC(
    HANDLE: i32,
    DLADSC: &[i32],
    DSKDSC: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DSKDSC = DummyArray::new(DSKDSC, 1..);
    let mut ZZDSKMRC: bool = false;
    let mut ALT: f64 = 0.0;
    let mut CO1MAX: f64 = 0.0;
    let mut CO1MIN: f64 = 0.0;
    let mut CO2MAX: f64 = 0.0;
    let mut CO2MIN: f64 = 0.0;
    let mut F: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LOCPOS = StackArray::<f64, 3>::new(1..=3);
    let mut LON: f64 = 0.0;
    let mut R: f64 = 0.0;
    let mut RE: f64 = 0.0;
    let mut RMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SEGFID: i32 = 0;
    let mut SEGSYS: i32 = 0;

    //
    // We don't have a match to begin with.
    //
    ZZDSKMRC = false;

    if save.FIRST {
        save.PI2 = TWOPI(ctx);
        save.FIRST = false;
    }

    // Check the surface ID.
    //
    if (save.SAVBID != intrinsics::IDNINT(DSKDSC[SRFIDX])) {
        return Ok(ZZDSKMRC);
    }

    //
    // Reject any segment whose target (center) ID doesn't match
    // the request ID.
    //
    if (save.SAVTRG != intrinsics::IDNINT(DSKDSC[CTRIDX])) {
        return Ok(ZZDSKMRC);
    }

    //
    // Check the time coverage.
    //
    if ((save.SAVET < DSKDSC[BTMIDX]) || (save.SAVET > DSKDSC[ETMIDX])) {
        return Ok(ZZDSKMRC);
    }

    //
    // Check the data class.
    //
    if (save.SAVCLS != intrinsics::IDNINT(DSKDSC[CLSIDX])) {
        return Ok(ZZDSKMRC);
    }

    //
    // Check whether the position vector is covered by the segment.
    // In order to determine this, we need to transform the vector
    // into the frame of the segment, if the frames differ.
    //
    SEGFID = intrinsics::IDNINT(DSKDSC[FRMIDX]);

    if (save.SAVFID == SEGFID) {
        //
        // The request frame and segment frame match. Just copy
        // the saved vector.
        //
        VEQU(save.SAVPOS.as_slice(), LOCPOS.as_slice_mut());
    } else {
        //
        // Transform the saved vector to the frame of the
        // segment. The transformation epoch is the saved
        // value of ET.
        //
        REFCHG(save.SAVFID, SEGFID, save.SAVET, RMAT.as_slice_mut(), ctx)?;
        MXV(
            RMAT.as_slice(),
            save.SAVPOS.as_slice(),
            LOCPOS.as_slice_mut(),
        );
    }

    // We do need to know whether the first coordinate is a longitude,
    // since we may need to adjust it to get it into the range of the
    // segment.
    //
    SEGSYS = intrinsics::IDNINT(DSKDSC[SYSIDX]);

    if ((SEGSYS == LATSYS) || (SEGSYS == PDTSYS)) {
        //
        // Find the latitude and longitude of the input point, expressed
        // in the coordinate system of this segment.
        //
        if (SEGSYS == LATSYS) {
            RECLAT(LOCPOS.as_slice(), &mut R, &mut LON, &mut LAT);
        } else if (SEGSYS == PDTSYS) {
            RE = DSKDSC[PARIDX];
            F = DSKDSC[(PARIDX + 1)];

            RECGEO(LOCPOS.as_slice(), RE, F, &mut LON, &mut LAT, &mut ALT, ctx)?;
        } else {
            CHKIN(b"ZZDSKMRC", ctx)?;
            SETMSG(b"Backstop error (0): this code should be unreachable.", ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZDSKMRC", ctx)?;
            return Ok(ZZDSKMRC);
        }

        //
        // Adjust segment bounds using a small margin.
        //
        CO1MIN = (DSKDSC[MN1IDX] - MARGIN);
        CO1MAX = (DSKDSC[MX1IDX] + MARGIN);
        CO2MIN = (DSKDSC[MN2IDX] - MARGIN);
        CO2MAX = (DSKDSC[MX2IDX] + MARGIN);

        // Move longitude into range.
        //
        if (LON < CO1MIN) {
            LON = (LON + save.PI2);
        } else if (LON > CO1MAX) {
            LON = (LON - save.PI2);
        }
    } else {
        CHKIN(b"ZZDSKMRC", ctx)?;
        SETMSG(b"Only planetocentric and planetodetic coordinates are supported by this entry point. Segment coordinate system was #.", ctx);
        ERRINT(b"#", SEGSYS, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZDSKMRC", ctx)?;
        return Ok(ZZDSKMRC);
    }

    //
    // Check the first coordinate against the segment's
    // corresponding coverage range.
    //
    if ((LON < CO1MIN) || (LON > CO1MAX)) {
        //
        // The first input coordinate is not covered by this
        // segment.
        //
        return Ok(ZZDSKMRC);
    }

    //
    // Compare the second coordinate against the segment's
    // corresponding coverage range.
    //
    if ((LAT < CO2MIN) || (LAT > CO2MAX)) {
        //
        // The second input coordinate is not covered by this
        // segment.
        //
        return Ok(ZZDSKMRC);
    }

    //
    // At this point we have a match.
    //
    ZZDSKMRC = true;

    Ok(ZZDSKMRC)
}
