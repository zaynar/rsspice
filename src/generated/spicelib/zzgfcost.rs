//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CNVTOL: f64 = 0.000001;
const NWMAX: i32 = 15;
const NWDIST: i32 = 5;
const NWSEP: i32 = 5;
const NWRR: i32 = 5;
const NWUDS: i32 = 5;
const NWPA: i32 = 5;
const NWILUM: i32 = 5;
const ADDWIN: f64 = 0.5;
const FRMNLN: i32 = 32;
const FOVTLN: i32 = 40;
const FTCIRC: &[u8] = b"CIRCLE";
const FTELLI: &[u8] = b"ELLIPSE";
const FTPOLY: &[u8] = b"POLYGON";
const FTRECT: &[u8] = b"RECTANGLE";
const ANNULR: &[u8] = b"ANNULAR";
const ANY: &[u8] = b"ANY";
const PARTL: &[u8] = b"PARTIAL";
const FULL: &[u8] = b"FULL";
const DSSHAP: &[u8] = b"DSK";
const EDSHAP: &[u8] = b"ELLIPSOID";
const PTSHAP: &[u8] = b"POINT";
const RYSHAP: &[u8] = b"RAY";
const SPSHAP: &[u8] = b"SPHERE";
const NOCTYP: i32 = 4;
const OCLLN: i32 = 7;
const SHPLEN: i32 = 9;
const MAXVRT: i32 = 10000;
const CIRFOV: &[u8] = b"CIRCLE";
const ELLFOV: &[u8] = b"ELLIPSE";
const POLFOV: &[u8] = b"POLYGON";
const RECFOV: &[u8] = b"RECTANGLE";
const RECSYS: &[u8] = b"RECTANGULAR";
const LATSYS: &[u8] = b"LATITUDINAL";
const SPHSYS: &[u8] = b"SPHERICAL";
const RADSYS: &[u8] = b"RA/DEC";
const CYLSYS: &[u8] = b"CYLINDRICAL";
const GEOSYS: &[u8] = b"GEODETIC";
const PGRSYS: &[u8] = b"PLANETOGRAPHIC";
const XCRD: &[u8] = b"X";
const YCRD: &[u8] = b"Y";
const ZCRD: &[u8] = b"Z";
const RADCRD: &[u8] = b"RADIUS";
const LONCRD: &[u8] = b"LONGITUDE";
const LATCRD: &[u8] = b"LATITUDE";
const RACRD: &[u8] = b"RIGHT ASCENSION";
const DECCRD: &[u8] = b"DECLINATION";
const RNGCRD: &[u8] = b"RANGE";
const CLTCRD: &[u8] = b"COLATITUDE";
const ALTCRD: &[u8] = b"ALTITUDE";
const POSDEF: &[u8] = b"POSITION";
const SOBDEF: &[u8] = b"SUB-OBSERVER POINT";
const SINDEF: &[u8] = b"SURFACE INTERCEPT POINT";
const NWREL: i32 = 5;
const NWLONG: i32 = 7;
const EXWIDX: i32 = ((NWREL + NWLONG) + 1);
const MXBEGM: i32 = 55;
const MXENDM: i32 = 13;
const MXMSG: i32 = ((MXBEGM + MXENDM) + 10);

//$Procedure      ZZGFCOST ( GF, coordinate definition state )
pub fn ZZGFCOST(
    VECDEF: &[u8],
    METHOD: &[u8],
    TRGID: i32,
    ET: f64,
    REF: &[u8],
    ABCORR: &[u8],
    OBSID: i32,
    DREF: &[u8],
    DCTR: i32,
    DVEC: &[f64],
    RADII: &[f64],
    STATE: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DVEC = DummyArray::new(DVEC, 1..=3);
    let RADII = DummyArray::new(RADII, 1..=3);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut LT: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFCOST", ctx)?;

    //
    // No result was found yet.
    //
    *FOUND = false;

    if fstr::eq(VECDEF, POSDEF) {
        //
        // Find the observer-target state vector.
        //
        SPKEZ(
            TRGID,
            ET,
            REF,
            ABCORR,
            OBSID,
            STATE.as_slice_mut(),
            &mut LT,
            ctx,
        )?;

        *FOUND = true;
    } else if fstr::eq(VECDEF, SOBDEF) {
        //
        // The caller has requested the state of a sub-observer point.
        //
        ZZGFSSOB(
            METHOD,
            TRGID,
            ET,
            REF,
            ABCORR,
            OBSID,
            RADII.as_slice(),
            STATE.as_slice_mut(),
            ctx,
        )?;

        *FOUND = true;
    } else if fstr::eq(VECDEF, SINDEF) {
        //
        // The caller has requested the state of a surface intercept
        // point.
        //
        ZZGFSSIN(
            METHOD,
            TRGID,
            ET,
            REF,
            ABCORR,
            OBSID,
            DREF,
            DCTR,
            DVEC.as_slice(),
            RADII.as_slice(),
            STATE.as_slice_mut(),
            FOUND,
            ctx,
        )?;
    } else {
        SETMSG(b"The coordinate quantity # is not recognized.", ctx);
        ERRCH(b"#", VECDEF, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZGFCOST", ctx)?;
        return Ok(());
    }

    //
    // At this point, one of the following is true:
    //
    //    - the state vector was found and
    //      FOUND is .TRUE.
    //
    //    - FOUND is .FALSE.
    //
    //    - a SPICE error occurred
    //

    CHKOUT(b"ZZGFCOST", ctx)?;
    Ok(())
}
