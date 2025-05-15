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
const MAXQRY: i32 = 2000;
const MAXSEL: i32 = 50;
const MAXTAB: i32 = 10;
const MAXCON: i32 = 1000;
const MXJOIN: i32 = 10;
const MXJCON: i32 = 100;
const MAXORD: i32 = 10;
const MAXTOK: i32 = 500;
const MAXQNM: i32 = 100;
const MAXCLN: i32 = MAXQRY;
const MAXSTR: i32 = 1024;
const SDSCSZ: i32 = 24;
const EKTIDX: i32 = 1;
const SNOIDX: i32 = (EKTIDX + 1);
const IMDIDX: i32 = (SNOIDX + 1);
const TNMIDX: i32 = (IMDIDX + 1);
const NCIDX: i32 = (TNMIDX + 1);
const NRIDX: i32 = (NCIDX + 1);
const RTIDX: i32 = (NRIDX + 1);
const CPTIDX: i32 = (RTIDX + 1);
const DPTIDX: i32 = (CPTIDX + 1);
const IPTIDX: i32 = (DPTIDX + 1);
const MFLIDX: i32 = (IPTIDX + 1);
const IFLIDX: i32 = (MFLIDX + 1);
const SHDIDX: i32 = (IFLIDX + 1);
const CFHIDX: i32 = (SHDIDX + 1);
const CSNIDX: i32 = (CFHIDX + 1);
const LCPIDX: i32 = (CSNIDX + 1);
const LDPIDX: i32 = (LCPIDX + 1);
const LIPIDX: i32 = (LDPIDX + 1);
const LCWIDX: i32 = (LIPIDX + 1);
const LDWIDX: i32 = (LCWIDX + 1);
const LIWIDX: i32 = (LDWIDX + 1);
const NMLIDX: i32 = (LIWIDX + 1);
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;

//$Procedure     ZZEKFRX ( EK, find record in index )
pub fn ZZEKFRX(
    HANDLE: i32,
    SEGDSC: &[i32],
    COLDSC: &[i32],
    RECPTR: i32,
    POS: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..=SDSCSZ);
    let COLDSC = DummyArray::new(COLDSC, 1..=CDSCSZ);
    let mut CVAL = [b' '; MAXSTR as usize];
    let mut DVAL: f64 = 0.0;
    let mut CMPLEN: i32 = 0;
    let mut CVLEN: i32 = 0;
    let mut DTYPE: i32 = 0;
    let mut IVAL: i32 = 0;
    let mut PRVPTR: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut FOUND: bool = false;
    let mut ISNULL: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Non-SPICELIB functions
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

    CHKIN(b"ZZEKFRX", ctx)?;

    //
    // Determine the data type of the column, and look up the value
    // associated with RECPTR.
    //
    DTYPE = COLDSC[TYPIDX];

    if (DTYPE == CHR) {
        ZZEKRSC(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            RECPTR,
            1,
            &mut CVLEN,
            &mut CVAL,
            &mut ISNULL,
            &mut FOUND,
            ctx,
        )?;

        if (FOUND && !ISNULL) {
            CMPLEN = intrinsics::MIN0(&[CVLEN, MAXSTR]);
        } else {
            CMPLEN = 0;
        }
    } else if ((DTYPE == DP) || (DTYPE == TIME)) {
        ZZEKRSD(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            RECPTR,
            1,
            &mut DVAL,
            &mut ISNULL,
            &mut FOUND,
            ctx,
        )?;
    } else if (DTYPE == INT) {
        ZZEKRSI(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            RECPTR,
            1,
            &mut IVAL,
            &mut ISNULL,
            &mut FOUND,
            ctx,
        )?;
    } else {
        SETMSG(
            b"File = #; COLIDX = #. Unrecognized data type code # found in descriptor.",
            ctx,
        );
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", COLDSC[ORDIDX], ctx);
        ERRINT(b"#", DTYPE, ctx);
        SIGERR(b"SPICE(ITEMNOTFOUND)", ctx)?;
        CHKOUT(b"ZZEKFRX", ctx)?;
        return Ok(());
    }

    if !FOUND {
        //
        // We have a most heinous situation.  We should always be able
        // to find the value associated with a record.
        //
        RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;

        SETMSG(b"File = #; RECNO = #; COLIDX = #. Column entry was not found.  This probably indicates a corrupted file or a bug in the EK code.", ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", COLDSC[ORDIDX], ctx);
        SIGERR(b"SPICE(ITEMNOTFOUND)", ctx)?;
        CHKOUT(b"ZZEKFRX", ctx)?;
        return Ok(());
    }

    //
    // Find the last column entry less than or equal to the one
    // associated with the input record, where the order relation is
    // dictionary ordering on (<column value>, <record number>) pairs.
    // These ordered pairs are distinct, even if the column entries
    // are not.  Therefore, the ordinal position POS will actually be
    // the ordinal position of our record.
    //
    if (DTYPE == CHR) {
        ZZEKLERC(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            fstr::substr(&CVAL, 1..=CMPLEN),
            RECPTR,
            ISNULL,
            POS,
            &mut PRVPTR,
            ctx,
        )?;
    } else if ((DTYPE == DP) || (DTYPE == TIME)) {
        ZZEKLERD(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            DVAL,
            RECPTR,
            ISNULL,
            POS,
            &mut PRVPTR,
            ctx,
        )?;
    } else {
        //
        // The data type is INT.  (We've already checked for invalid
        // types.)
        //

        ZZEKLERI(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            IVAL,
            RECPTR,
            ISNULL,
            POS,
            &mut PRVPTR,
            ctx,
        )?;
    }

    if (PRVPTR != RECPTR) {
        //
        // Big problem.  This should never happen.
        //
        RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;

        SETMSG(b"File = #; RECNO = #; COLIDX = #.  Record that was last less than or equal to RECNO was not equal to RECNO.  This probably indicates  a corrupted file or a bug in the EK code.", ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", COLDSC[ORDIDX], ctx);
        SIGERR(b"SPICE(ITEMNOTFOUND)", ctx)?;
        CHKOUT(b"ZZEKFRX", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZEKFRX", ctx)?;
    Ok(())
}
