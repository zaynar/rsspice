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
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
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
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const LBCELL: i32 = -5;

//$Procedure  ZZEKQCON ( Private: EK, read constraints from query )
pub fn ZZEKQCON(
    EQRYI: &[i32],
    EQRYC: &[u8],
    EQRYD: &[f64],
    N: i32,
    CNSTYP: &mut i32,
    LTNAME: &mut [u8],
    LTIDX: &mut i32,
    LCNAME: &mut [u8],
    LCIDX: &mut i32,
    OPCODE: &mut i32,
    RTNAME: &mut [u8],
    RTIDX: &mut i32,
    RCNAME: &mut [u8],
    RCIDX: &mut i32,
    DTYPE: &mut i32,
    CBEG: &mut i32,
    CEND: &mut i32,
    DVAL: &mut f64,
    IVAL: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let EQRYI = DummyArray::new(EQRYI, LBCELL..);
    let EQRYD = DummyArray::new(EQRYD, 1..);
    let mut BASE: i32 = 0;
    let mut CB: i32 = 0;
    let mut CE: i32 = 0;
    let mut ICHECK: i32 = 0;
    let mut NCNS: i32 = 0;
    let mut NTAB: i32 = 0;
    let mut PTR: i32 = 0;
    let mut TB: i32 = 0;
    let mut TE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //

    ZZEKREQI(EQRYI.as_slice(), b"SEM_CHECKED", &mut ICHECK, ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    if (ICHECK == IFALSE) {
        CHKIN(b"ZZEKQCON", ctx)?;
        SETMSG(b"Encoded query has not been semantically checked.", ctx);
        SIGERR(b"SPICE(NOTSEMCHECKED)", ctx)?;
        CHKOUT(b"ZZEKQCON", ctx)?;
        return Ok(());
    }

    ZZEKREQI(EQRYI.as_slice(), b"NUM_CONSTRAINTS", &mut NCNS, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"NUM_TABLES", &mut NTAB, ctx)?;

    if ((N < 1) || (N > NCNS)) {
        CHKIN(b"ZZEKQCON", ctx)?;
        SETMSG(b"Constraint index # is out of valid range 1:#.", ctx);
        ERRINT(b"#", N, ctx);
        ERRINT(b"#", NCNS, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKQCON", ctx)?;
        return Ok(());
    }

    //
    // Compute the base address of the Nth constraint.
    //
    BASE = ((EQVBAS + ((NTAB * 2) * EQVDSZ)) + ((N - 1) * EQCDSZ));

    //
    // Get the constraint type.
    //
    *CNSTYP = EQRYI[(BASE + EQCTYP)];

    //
    // Get the LHS items.
    //
    *LTIDX = EQRYI[(((BASE + EQLTAB) - 1) + EQTORD)];
    TB = EQRYI[(((BASE + EQLTAB) - 1) + EQBSTR)];
    TE = EQRYI[(((BASE + EQLTAB) - 1) + EQESTR)];

    if (TB != 0) {
        fstr::assign(LTNAME, fstr::substr(EQRYC, TB..=TE));
    } else {
        fstr::assign(LTNAME, b" ");
    }

    *LCIDX = EQRYI[(((BASE + EQLCOL) - 1) + EQCIDX)];
    CB = EQRYI[(((BASE + EQLCOL) - 1) + EQBSTR)];
    CE = EQRYI[(((BASE + EQLCOL) - 1) + EQESTR)];
    fstr::assign(LCNAME, fstr::substr(EQRYC, CB..=CE));

    //
    // Next, the opcode.
    //
    *OPCODE = EQRYI[(BASE + EQOPCD)];

    //
    // If the constraint compares two columns, get the RHS table and
    // column info.
    //
    if (*CNSTYP == EQCOL) {
        *RTIDX = EQRYI[(((BASE + EQRTAB) - 1) + EQTORD)];
        TB = EQRYI[(((BASE + EQRTAB) - 1) + EQBSTR)];
        TE = EQRYI[(((BASE + EQRTAB) - 1) + EQESTR)];

        if (TB != 0) {
            fstr::assign(RTNAME, fstr::substr(EQRYC, TB..=TE));
        } else {
            fstr::assign(RTNAME, b" ");
        }

        *RCIDX = EQRYI[(((BASE + EQRCOL) - 1) + EQCIDX)];
        CB = EQRYI[(((BASE + EQRCOL) - 1) + EQBSTR)];
        CE = EQRYI[(((BASE + EQRCOL) - 1) + EQESTR)];
        fstr::assign(RCNAME, fstr::substr(EQRYC, CB..=CE));

        //
        // ...and clear out the scalar outputs.
        //
        *CBEG = 1;
        *CEND = 1;
        *DVAL = 0.0;
        *IVAL = 0;
    } else {
        //
        // The constraint compares a column and a value.  Set the
        // appropriate scalar output, and clear out the other outputs.
        //
        if ((*OPCODE == ISNULL) || (*OPCODE == NOTNUL)) {
            //
            // There's no output value; the opcode implies the value NULL.
            // Set the outputs to innocuous defaults.
            //
            *CBEG = 1;
            *CEND = 1;
            *DVAL = 0.0;
            *IVAL = 0;
        } else {
            //
            // This is the normal case; set the scalar output values
            // according to the RHS data type.
            //
            *DTYPE = EQRYI[(((BASE + EQBVAL) - 1) + EQDTYP)];

            if (*DTYPE == CHR) {
                *CBEG = EQRYI[(((BASE + EQBVAL) - 1) + EQBSTR)];
                *CEND = EQRYI[(((BASE + EQBVAL) - 1) + EQESTR)];
                *DVAL = 0.0;
                *IVAL = 0;
            } else if (*DTYPE == INT) {
                PTR = EQRYI[(((BASE + EQBVAL) - 1) + EQVPTR)];
                *IVAL = intrinsics::IDNINT(EQRYD[PTR]);
                *DVAL = 0.0;
                *CBEG = 1;
                *CEND = 1;
            } else {
                //
                // The data type is DP or TIME.
                //
                PTR = EQRYI[(((BASE + EQBVAL) - 1) + EQVPTR)];
                *DVAL = EQRYD[PTR];
                *IVAL = 0;
                *CBEG = 1;
                *CEND = 1;
            }
        }

        //
        // Set the RHS table and column outputs.
        //
        *RTIDX = 0;
        fstr::assign(RTNAME, b" ");
        *RCIDX = 0;
        fstr::assign(RTNAME, b" ");
    }

    Ok(())
}
