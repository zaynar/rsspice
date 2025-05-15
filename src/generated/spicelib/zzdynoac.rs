//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const KVNMLN: i32 = 32;
const KVLEN: i32 = 80;
const FRNMLN: i32 = 32;
const BDNMLN: i32 = 36;
const MAXCOF: i32 = 20;
const MXNFAC: i32 = 10;
const LBSEP: f64 = 0.001;
const QEXP: i32 = -27;
const KWBFRM: &[u8] = b"RELATIVE";
const KWSTYL: &[u8] = b"DEF_STYLE";
const KVPARM: &[u8] = b"PARAMETERIZED";
const KWFREZ: &[u8] = b"FREEZE_EPOCH";
const KWRSTA: &[u8] = b"ROTATION_STATE";
const KVROTG: &[u8] = b"ROTATING";
const KVINRT: &[u8] = b"INERTIAL";
const KWFFAM: &[u8] = b"FAMILY";
const KVMEQT: &[u8] = b"MEAN_EQUATOR_AND_EQUINOX_OF_DATE";
const KVMECL: &[u8] = b"MEAN_ECLIPTIC_AND_EQUINOX_OF_DATE";
const KVTEQT: &[u8] = b"TRUE_EQUATOR_AND_EQUINOX_OF_DATE";
const KV2VEC: &[u8] = b"TWO-VECTOR";
const KVEULR: &[u8] = b"EULER";
const KVPROD: &[u8] = b"PRODUCT";
const KWPRCM: &[u8] = b"PREC_MODEL";
const KWNUTM: &[u8] = b"NUT_MODEL";
const KWOBQM: &[u8] = b"OBLIQ_MODEL";
const KVM001: &[u8] = b"EARTH_IAU_1976";
const KVM002: &[u8] = b"EARTH_IAU_1980";
const KVM003: &[u8] = b"EARTH_IAU_1980";
const KWVAXI: &[u8] = b"AXIS";
const KVX: &[u8] = b"X";
const KVY: &[u8] = b"Y";
const KVZ: &[u8] = b"Z";
const KWPRI: &[u8] = b"PRI_";
const KWSEC: &[u8] = b"SEC_";
const KWVCDF: &[u8] = b"VECTOR_DEF";
const KVPOSV: &[u8] = b"OBSERVER_TARGET_POSITION";
const KVVELV: &[u8] = b"OBSERVER_TARGET_VELOCITY";
const KVNEAR: &[u8] = b"TARGET_NEAR_POINT";
const KVCONS: &[u8] = b"CONSTANT";
const KWVOBS: &[u8] = b"OBSERVER";
const KWVTRG: &[u8] = b"TARGET";
const KWVFRM: &[u8] = b"FRAME";
const KWVABC: &[u8] = b"ABCORR";
const KWVSPC: &[u8] = b"SPEC";
const KVRECC: &[u8] = b"RECTANGULAR";
const KVLATC: &[u8] = b"LATITUDINAL";
const KVRADC: &[u8] = b"RA/DEC";
const KWVECT: &[u8] = b"VECTOR";
const KWLAT: &[u8] = b"LATITUDE";
const KWLON: &[u8] = b"LONGITUDE";
const KWRA: &[u8] = b"RA";
const KWDEC: &[u8] = b"DEC";
const KWATOL: &[u8] = b"ANGLE_SEP_TOL";
const KWEPOC: &[u8] = b"EPOCH";
const KWEUAX: &[u8] = b"AXES";
const KWEAC1: &[u8] = b"ANGLE_1_COEFFS";
const KWEAC2: &[u8] = b"ANGLE_2_COEFFS";
const KWEAC3: &[u8] = b"ANGLE_3_COEFFS";
const KWFFRM: &[u8] = b"FROM_FRAMES";
const KWTFRM: &[u8] = b"TO_FRAMES";
const KWUNIT: &[u8] = b"UNITS";
const KVRADN: &[u8] = b"RADIANS";
const KVDEGR: &[u8] = b"DEGREES";
const TEMPLT: &[u8] = b"FRAME_#_#";
const TEMPLN: i32 = 7;

//$Procedure ZZDYNOAC ( Fetch optional array, character frame variable )
pub fn ZZDYNOAC(
    FRNAME: &[u8],
    FRCODE: i32,
    ITEM: &[u8],
    MAXN: i32,
    N: &mut i32,
    VALUES: CharArrayMut,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut VALUES = DummyCharArrayMut::new(VALUES, None, 1..);
    let mut CDESTR = [b' '; KVNMLN as usize];
    let mut DTYPE = [b' '; 1 as usize];
    let mut KVNAME = [b' '; KVNMLN as usize];
    let mut CODELN: i32 = 0;
    let mut ITEMLN: i32 = 0;
    let mut NAMELN: i32 = 0;
    let mut REQNAM: i32 = 0;
    let mut REQNUM: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // TEMPLN is the length of the keyword template, minus
    // the sum of the lengths of the two substitution markers ('#').
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZDYNOAC", ctx)?;

    //
    // Nothing found yet.
    //
    *FOUND = false;

    //
    // Prepare to check the name of the kernel variable we're about
    // to look up.
    //
    // Convert the frame code to a string.
    //
    INTSTR(FRCODE, &mut CDESTR, ctx);

    if FAILED(ctx) {
        CHKOUT(b"ZZDYNOAC", ctx)?;
        return Ok(());
    }

    //
    // Get the lengths of the input frame code, name and item.
    // Compute the length of the ID-based kernel variable name;
    // check this length against the maximum allowed value.  If
    // the name is too long, proceed to look up the form of the
    // kernel variable name based on the frame name.
    //
    CODELN = RTRIM(&CDESTR);
    NAMELN = RTRIM(FRNAME);
    ITEMLN = RTRIM(ITEM);

    REQNUM = ((CODELN + ITEMLN) + TEMPLN);

    if (REQNUM <= KVNMLN) {
        //
        // First try looking for a kernel variable including the frame ID
        // code.
        //
        // Note the template is
        //
        //     'FRAME_#_#'
        //
        REPMI(TEMPLT, b"#", FRCODE, &mut KVNAME, ctx);
        REPMC(&KVNAME.clone(), b"#", ITEM, &mut KVNAME);
        DTPOOL(&KVNAME, FOUND, N, &mut DTYPE, ctx)?;
    } else {
        //
        // The ID-based name is too long. We can't find the variable if
        // we can't look it up.
        //
        *FOUND = false;
    }

    if !*FOUND {
        //
        // We need to look up the frame name-based kernel variable.
        // Determine the length of the name of this variable; make
        // sure it's not too long.
        //
        REQNAM = ((NAMELN + ITEMLN) + TEMPLN);

        if ((REQNAM > KVNMLN) && (REQNUM > KVNMLN)) {
            //
            // Both forms of the name are too long.
            //
            SETMSG(b"Kernel variable FRAME_#_# has length #; kernel variable FRAME_#_# has length #; maximum allowed length is #.  Neither variable could be searched for in the kernel pool due to these name length errors.", ctx);
            ERRINT(b"#", FRCODE, ctx);
            ERRCH(b"#", ITEM, ctx);
            ERRINT(b"#", REQNUM, ctx);
            ERRCH(b"#", FRNAME, ctx);
            ERRCH(b"#", ITEM, ctx);
            ERRINT(b"#", REQNAM, ctx);
            ERRINT(b"#", KVNMLN, ctx);
            SIGERR(b"SPICE(VARNAMETOOLONG)", ctx)?;
            CHKOUT(b"ZZDYNOAC", ctx)?;
            return Ok(());
        } else if (REQNAM > KVNMLN) {
            //
            // We couldn't find the variable having the ID-based name,
            // and the frame name-based variable name is too long to
            // look up.
            //
            CHKOUT(b"ZZDYNOAC", ctx)?;
            return Ok(());
        }

        //
        // Now try looking for a kernel variable including the frame
        // name.
        //
        REPMC(TEMPLT, b"#", FRNAME, &mut KVNAME);
        REPMC(&KVNAME.clone(), b"#", ITEM, &mut KVNAME);
        DTPOOL(&KVNAME, FOUND, N, &mut DTYPE, ctx)?;

        if !*FOUND {
            //
            // The FOUND flag is set appropriately.
            //
            CHKOUT(b"ZZDYNOAC", ctx)?;
            return Ok(());
        }
    }

    //
    // Getting to this point means we found a kernel variable. The name
    // of the variable is KVNAME.  The data type is DTYPE and the
    // cardinality is N.
    //
    // Rather than using BADKPV, we check the data type and cardinality
    // of the kernel variable in-line so we can create a more detailed
    // error message if need be.
    //
    if fstr::eq(&DTYPE, b"N") {
        SETMSG(b"The kernel variable # has used to define frame # was expected to have character data type but in fact has numeric data type.  Usually this type of problem is due to an error in a frame definition provided in a frame kernel.", ctx);
        ERRCH(b"#", &KVNAME, ctx);
        ERRCH(b"#", FRNAME, ctx);
        SIGERR(b"SPICE(BADVARIABLETYPE)", ctx)?;
        CHKOUT(b"ZZDYNOAC", ctx)?;
        return Ok(());
    }

    if (*N > MAXN) {
        SETMSG(b"The kernel variable # has used to define frame # was expected to have size not exceeding # but in fact has size #. Usually this type of problem is due to an error in a frame definition provided in a frame kernel.", ctx);
        ERRCH(b"#", &KVNAME, ctx);
        ERRCH(b"#", FRNAME, ctx);
        ERRINT(b"#", MAXN, ctx);
        ERRINT(b"#", *N, ctx);
        SIGERR(b"SPICE(BADVARIABLESIZE)", ctx)?;
        CHKOUT(b"ZZDYNOAC", ctx)?;
        return Ok(());
    }

    //
    // Look up the kernel variable.
    //
    GCPOOL(&KVNAME, 1, MAXN, N, VALUES.as_arg_mut(), FOUND, ctx)?;

    if !*FOUND {
        SETMSG(
            b"Variable # not found after DTPOOL indicated it was present in pool.",
            ctx,
        );
        ERRCH(b"#", &KVNAME, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZDYNOAC", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZDYNOAC", ctx)?;
    Ok(())
}
