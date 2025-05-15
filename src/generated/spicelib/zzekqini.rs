//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

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
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
const LBCELL: i32 = -5;
const MNISIZ: i32 = (((((EQVBAS + ((MAXTAB * EQVDSZ) * 2)) + (MAXCON * EQCDSZ)) + MAXCON)
    + (MAXORD * EQODSZ))
    + (MAXSEL * EQSDSZ));

//$Procedure   ZZEKQINI ( Private: EK, initialize encoded query )
pub fn ZZEKQINI(
    ISIZE: i32,
    DSIZE: i32,
    EQRYI: &mut [i32],
    EQRYC: &mut [u8],
    EQRYD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut EQRYI = DummyArrayMut::new(EQRYI, LBCELL..);
    let mut EQRYD = DummyArrayMut::new(EQRYD, 1..);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // Minimum upper bound for the integer cell of an encoded query:
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
        CHKIN(b"ZZEKQINI", ctx)?;
    }

    //
    // Check sizes:
    //
    if (ISIZE < MNISIZ) {
        SETMSG(
            b"Size of integer component of encoded query is #; at least # elements are required.",
            ctx,
        );
        ERRINT(b"#", ISIZE, ctx);
        ERRINT(b"#", MNISIZ, ctx);
        SIGERR(b"SPICE(CELLTOOSMALL)", ctx)?;
        CHKOUT(b"ZZEKQINI", ctx)?;
        return Ok(());
    }

    if (DSIZE < MAXQNM) {
        SETMSG(
            b"Size of d.p. component of encoded query is #; at least # elements are required.",
            ctx,
        );
        ERRINT(b"#", DSIZE, ctx);
        ERRINT(b"#", MAXQNM, ctx);
        SIGERR(b"SPICE(CELLTOOSMALL)", ctx)?;
        CHKOUT(b"ZZEKQINI", ctx)?;
        return Ok(());
    }

    if (intrinsics::LEN(EQRYC) < MAXCLN) {
        SETMSG(b"Size of character component of encoded query is #; a length of at least # characters is required.", ctx);
        ERRINT(b"#", intrinsics::LEN(EQRYC), ctx);
        ERRINT(b"#", MAXCLN, ctx);
        SIGERR(b"SPICE(STRINGTOOSHORT)", ctx)?;
        CHKOUT(b"ZZEKQINI", ctx)?;
        return Ok(());
    }

    //
    // Initialize the integer cell, the d.p. array, and the string.
    //
    SSIZEI(ISIZE, EQRYI.as_slice_mut(), ctx)?;
    CLEARD(DSIZE, EQRYD.as_slice_mut());
    fstr::assign(EQRYC, b" ");

    //
    // Append enough elements to the integer cell to contain the
    // fixed-size portion of the encoded query:
    //
    for I in 1..=EQVBAS {
        APPNDI(0, EQRYI.as_slice_mut(), ctx)?;
    }

    //
    // Clear out the fixed-size portion of the integer cell.
    //
    CLEARI(EQVBAS, EQRYI.subarray_mut(1));

    //
    // Fill in the architecture version.
    //
    EQRYI[EQARCH] = 1;

    //
    // Set the parse completion and name and time resolution flags to
    // indicate `not done':
    //
    EQRYI[EQPARS] = IFALSE;
    EQRYI[EQNRES] = IFALSE;
    EQRYI[EQTRES] = IFALSE;

    //
    // Set the buffer sizes:
    //
    EQRYI[EQCSIZ] = intrinsics::LEN(EQRYC);
    EQRYI[EQNSIZ] = DSIZE;

    //
    // Set the free pointers:
    //
    EQRYI[EQNPTR] = 1;
    EQRYI[EQCPTR] = 1;

    //
    // Indicate that initialization has been done:
    //
    EQRYI[EQINIT] = ITRUE;

    CHKOUT(b"ZZEKQINI", ctx)?;
    Ok(())
}
