//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const EQ: i32 = 1;
const GE: i32 = (EQ + 1);
const GT: i32 = (GE + 1);
const LE: i32 = (GT + 1);
const LT: i32 = (LE + 1);
const NE: i32 = (LT + 1);
const LIKE: i32 = (NE + 1);
const UNLIKE: i32 = (LIKE + 1);
const ISNULL: i32 = (UNLIKE + 1);
const NOTNUL: i32 = (ISNULL + 1);
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
const EQARCH: i32 = 2;
const EQINIT: i32 = (EQARCH + 1);
const EQPARS: i32 = (EQINIT + 1);
const EQNRES: i32 = (EQPARS + 1);
const EQTRES: i32 = (EQNRES + 1);
const EQSCHK: i32 = (EQTRES + 1);
const EQNTAB: i32 = (EQSCHK + 1);
const EQNCNS: i32 = (EQNTAB + 1);
const EQMXML: i32 = -1;
const EQNCNJ: i32 = (EQNCNS + 1);
const EQNORD: i32 = (EQNCNJ + 1);
const EQNSEL: i32 = (EQNORD + 1);
const EQNSIZ: i32 = (EQNSEL + 1);
const EQNPTR: i32 = (EQNSIZ + 1);
const EQCSIZ: i32 = (EQNPTR + 1);
const EQCPTR: i32 = (EQCSIZ + 1);
const EQBSEL: i32 = (EQCPTR + 1);
const EQBCON: i32 = (EQBSEL + 1);
const EQBCNJ: i32 = (EQBCON + 1);
const EQBORD: i32 = (EQBCON + 1);
const EQVBAS: i32 = EQBORD;
const EQDTYP: i32 = 1;
const EQBLEX: i32 = (EQDTYP + 1);
const EQELEX: i32 = (EQBLEX + 1);
const EQBSTR: i32 = (EQELEX + 1);
const EQESTR: i32 = (EQBSTR + 1);
const EQVPTR: i32 = (EQELEX + 1);
const EQVDSZ: i32 = 6;
const EQBCOL: i32 = 1;
const EQCIDX: i32 = EQVDSZ;
const EQBTAB: i32 = 1;
const EQTORD: i32 = EQVDSZ;
const EQCTYP: i32 = 1;
const EQCOL: i32 = 1;
const EQVAL: i32 = 2;
const EQLTAB: i32 = (EQCTYP + 1);
const EQLCOL: i32 = (EQLTAB + EQVDSZ);
const EQOPCD: i32 = (EQLCOL + EQVDSZ);
const EQRTAB: i32 = (EQOPCD + 1);
const EQRCOL: i32 = (EQRTAB + EQVDSZ);
const EQBVAL: i32 = (EQOPCD + 1);
const EQCDSZ: i32 = (2 + (4 * EQVDSZ));
const EQOTAB: i32 = 1;
const EQOCOL: i32 = (EQOTAB + EQVDSZ);
const EQODIR: i32 = (EQOCOL + EQVDSZ);
const EQODSZ: i32 = (1 + (2 * EQVDSZ));
const EQASND: i32 = 0;
const EQDSND: i32 = 1;
const EQSTAB: i32 = 1;
const EQSCOL: i32 = (EQSTAB + EQVDSZ);
const EQSDSZ: i32 = (2 * EQVDSZ);
const EQIMIN: i32 =
    (((((EQVBAS + ((10 * EQVDSZ) * 2)) + (1000 * EQCDSZ)) + 1000) + (10 * EQODSZ)) + (50 * EQSDSZ));
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const LBCELL: i32 = -5;
const MAXCOL: i32 = 100;

//$Procedure      ZZEKENCD ( EK, encode query )
pub fn ZZEKENCD(
    QUERY: &[u8],
    EQRYI: &mut [i32],
    EQRYC: &mut [u8],
    EQRYD: &mut [f64],
    ERROR: &mut bool,
    ERRMSG: &mut [u8],
    ERRPTR: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut EQRYI = DummyArrayMut::new(EQRYI, LBCELL..);
    let mut EQRYD = DummyArrayMut::new(EQRYD, 1..);
    let mut CHRBUF = [b' '; MAXCLN as usize];
    let mut NUMVLS = StackArray::<f64, 100>::new(1..=MAXQNM);
    let mut CHBEGS = ActualArray::<i32>::new(1..=MAXTOK);
    let mut CHENDS = ActualArray::<i32>::new(1..=MAXTOK);
    let mut LXBEGS = ActualArray::<i32>::new(1..=MAXTOK);
    let mut LXENDS = ActualArray::<i32>::new(1..=MAXTOK);
    let mut NTOKEN: i32 = 0;
    let mut TOKENS = ActualArray::<i32>::new(1..=MAXTOK);
    let mut VALUES = ActualArray::<i32>::new(1..=MAXTOK);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // Storage limits:
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZEKENCD", ctx)?;
    }

    //
    // Initialize the encoded query each time, for safety.
    //
    ZZEKQINI(
        EQIMIN,
        MAXQNM,
        EQRYI.as_slice_mut(),
        EQRYC,
        EQRYD.as_slice_mut(),
        ctx,
    )?;

    //
    // Find the tokens in the input query.
    //
    ZZEKSCAN(
        QUERY,
        MAXTOK,
        MAXQNM,
        &mut NTOKEN,
        TOKENS.as_slice_mut(),
        LXBEGS.as_slice_mut(),
        LXENDS.as_slice_mut(),
        VALUES.as_slice_mut(),
        NUMVLS.as_slice_mut(),
        &mut CHRBUF,
        CHBEGS.as_slice_mut(),
        CHENDS.as_slice_mut(),
        ERROR,
        ERRMSG,
        ctx,
    )?;

    if *ERROR {
        *ERRPTR = 1;
        CHKOUT(b"ZZEKENCD", ctx)?;
        return Ok(());
    }

    //
    // Now parse the query.
    //
    ZZEKPARS(
        QUERY,
        NTOKEN,
        LXBEGS.as_slice(),
        LXENDS.as_slice(),
        TOKENS.as_slice(),
        VALUES.as_slice(),
        NUMVLS.as_slice(),
        &CHRBUF,
        CHBEGS.as_slice(),
        CHENDS.as_slice(),
        EQRYI.as_slice_mut(),
        EQRYC,
        EQRYD.as_slice_mut(),
        ERROR,
        ERRMSG,
        ctx,
    )?;

    if *ERROR {
        *ERRPTR = 1;
        CHKOUT(b"ZZEKENCD", ctx)?;
        return Ok(());
    }

    //
    // Resolve names.
    //
    ZZEKNRES(
        QUERY,
        EQRYI.as_slice_mut(),
        EQRYC,
        ERROR,
        ERRMSG,
        ERRPTR,
        ctx,
    )?;

    if *ERROR {
        CHKOUT(b"ZZEKENCD", ctx)?;
        return Ok(());
    }

    //
    // Resolve time values, if necessary.
    //
    ZZEKTRES(
        QUERY,
        EQRYI.as_slice_mut(),
        EQRYC,
        EQRYD.as_slice_mut(),
        ERROR,
        ERRMSG,
        ERRPTR,
        ctx,
    )?;

    if *ERROR {
        CHKOUT(b"ZZEKENCD", ctx)?;
        return Ok(());
    }

    //
    // Perform semantic checks.
    //
    ZZEKSEMC(
        QUERY,
        EQRYI.as_slice_mut(),
        EQRYC,
        ERROR,
        ERRMSG,
        ERRPTR,
        ctx,
    )?;

    if *ERROR {
        CHKOUT(b"ZZEKENCD", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZEKENCD", ctx)?;
    Ok(())
}
