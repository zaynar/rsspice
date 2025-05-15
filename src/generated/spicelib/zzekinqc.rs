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
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
const LBCELL: i32 = -5;

//$Procedure   ZZEKINQC ( Private: EK, insert into query, character )
pub fn ZZEKINQC(
    VALUE: &[u8],
    LENGTH: i32,
    LEXBEG: i32,
    LEXEND: i32,
    EQRYI: &mut [i32],
    EQRYC: &mut [u8],
    DESCR: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut EQRYI = DummyArrayMut::new(EQRYI, LBCELL..);
    let mut DESCR = DummyArrayMut::new(DESCR, 1..);
    let mut FREE: i32 = 0;
    let mut INIT: i32 = 0;
    let mut L: i32 = 0;
    let mut ROOM: i32 = 0;
    let mut SIZE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    ZZEKREQI(EQRYI.as_slice(), b"INITIALIZED", &mut INIT, ctx)?;

    if (INIT != ITRUE) {
        CHKIN(b"ZZEKINQC", ctx)?;
        SETMSG(
            b"Encoded query must be initialized before it may be written.",
            ctx,
        );
        SIGERR(b"SPICE(NOTINITIALIZED)", ctx)?;
        CHKOUT(b"ZZEKINQC", ctx)?;
        return Ok(());
    }

    //
    // Check the input length value.
    //
    if (LENGTH < 1) {
        CHKIN(b"ZZEKINQC", ctx)?;
        SETMSG(b"Length of string value was #; must be > 0.", ctx);
        ERRINT(b"#", LENGTH, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZEKINQC", ctx)?;
        return Ok(());
    }

    //
    // Get the character free pointer; make sure there's enough room.
    //
    ZZEKREQI(EQRYI.as_slice(), b"FREE_CHR", &mut FREE, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"CHR_BUF_SIZE", &mut SIZE, ctx)?;

    ROOM = ((SIZE - FREE) + 1);

    if (LENGTH > ROOM) {
        CHKIN(b"ZZEKINQC", ctx)?;
        SETMSG(b"Out of room in character portion of encoded query; only # elements were available; # are needed.", ctx);
        ERRINT(b"#", ROOM, ctx);
        ERRINT(b"#", LENGTH, ctx);
        SIGERR(b"SPICE(BUFFERTOOSMALL)", ctx)?;
        CHKOUT(b"ZZEKINQC", ctx)?;
        return Ok(());
    }

    //
    // Insert the value into the character portion of the encoded query.
    //
    L = intrinsics::MIN0(&[LENGTH, intrinsics::LEN(VALUE)]);
    fstr::assign(fstr::substr_mut(EQRYC, FREE..), fstr::substr(VALUE, 1..=L));

    //
    // Fill in the descriptor.
    //
    CLEARI(EQVDSZ, DESCR.as_slice_mut());

    DESCR[EQDTYP] = CHR;
    DESCR[EQBLEX] = LEXBEG;
    DESCR[EQELEX] = LEXEND;
    DESCR[EQBSTR] = FREE;
    DESCR[EQESTR] = ((FREE + LENGTH) - 1);

    //
    // Update the character free pointer.
    //
    ZZEKWEQI(b"FREE_CHR", (FREE + LENGTH), EQRYI.as_slice_mut(), ctx)?;

    Ok(())
}
