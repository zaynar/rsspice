//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CDOFF: i32 = 24;
const CDSCSZ: i32 = 11;
const CLSIDX: i32 = 1;
const TYPIDX: i32 = (CLSIDX + 1);
const LENIDX: i32 = (TYPIDX + 1);
const SIZIDX: i32 = (LENIDX + 1);
const NAMIDX: i32 = (SIZIDX + 1);
const IXTIDX: i32 = (NAMIDX + 1);
const IXPIDX: i32 = (IXTIDX + 1);
const NFLIDX: i32 = (IXPIDX + 1);
const ORDIDX: i32 = (NFLIDX + 1);
const METIDX: i32 = (ORDIDX + 1);
const MXCLSG: i32 = 100;

//$Procedure      ZZEKSTYP ( EK, determine segment type )
pub fn ZZEKSTYP(NCOLS: i32, CDSCRS: &[i32], ctx: &mut Context) -> f2rust_std::Result<i32> {
    let CDSCRS = DummyArray2D::new(CDSCRS, 1..=CDSCSZ, 1..);
    let mut ZZEKSTYP: i32 = 0;
    let mut FIXED: bool = false;
    let mut VAR: bool = false;

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
        ZZEKSTYP = 0;
        return Ok(ZZEKSTYP);
    } else {
        CHKIN(b"ZZEKSTYP", ctx)?;
    }

    //
    // FIXED and VAR indicate whether we've seen any fixed or variable
    // column classes so far.
    //
    FIXED = false;
    VAR = false;

    for I in 1..=NCOLS {
        if ((CDSCRS[[CLSIDX, I]] >= 1) && (CDSCRS[[CLSIDX, I]] <= 6)) {
            VAR = true;
        } else if ((CDSCRS[[CLSIDX, I]] >= 7) && (CDSCRS[[CLSIDX, I]] <= 9)) {
            FIXED = true;
        }
    }

    if (VAR && !FIXED) {
        ZZEKSTYP = 1;
    } else if (FIXED && !VAR) {
        ZZEKSTYP = 2;
    } else {
        ZZEKSTYP = 0;

        SETMSG(b"Column set contains a mixture of variable and fixed-count columns.  Segments must contain all variable or all fixed count columns.", ctx);
        SIGERR(b"SPICE(BADATTRIBUTES)", ctx)?;
        CHKOUT(b"ZZEKSTYP", ctx)?;
        return Ok(ZZEKSTYP);
    }

    CHKOUT(b"ZZEKSTYP", ctx)?;
    Ok(ZZEKSTYP)
}
