//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ADSCSZ: i32 = 6;
const ATTCLS: i32 = 1;
const ATTTYP: i32 = (ATTCLS + 1);
const ATTLEN: i32 = (ATTTYP + 1);
const ATTSIZ: i32 = (ATTLEN + 1);
const ATTIDX: i32 = (ATTSIZ + 1);
const ATTNFL: i32 = (ATTIDX + 1);
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
const CNAMSZ: i32 = 32;
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
const TNAMSZ: i32 = 64;
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const LBCELL: i32 = -5;
const SHORT: i32 = 32;

//$Procedure  ZZEKTRES ( Private: EK, resolve times in encoded query )
pub fn ZZEKTRES(
    QUERY: &[u8],
    EQRYI: &mut [i32],
    EQRYC: &[u8],
    EQRYD: &mut [f64],
    ERROR: &mut bool,
    ERRMSG: &mut [u8],
    ERRPTR: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut EQRYI = DummyArrayMut::new(EQRYI, LBCELL..);
    let mut EQRYD = DummyArrayMut::new(EQRYD, 1..);
    let mut ALIAS = ActualCharArray::new(TNAMSZ, 1..=MAXTAB);
    let mut COLNAM = [b' '; CNAMSZ as usize];
    let mut TABLE = ActualCharArray::new(TNAMSZ, 1..=MAXTAB);
    let mut TIMSTR = [b' '; SHORT as usize];
    let mut TOUCHC = [b' '; 1 as usize];
    let mut ET: f64 = 0.0;
    let mut ATTDSC = StackArray::<i32, 6>::new(1..=ADSCSZ);
    let mut BASE: i32 = 0;
    let mut CNSTYP: i32 = 0;
    let mut COLIDX: i32 = 0;
    let mut DESCR = StackArray::<i32, 6>::new(1..=EQVDSZ);
    let mut DTYPE: i32 = 0;
    let mut IRSOLV: i32 = 0;
    let mut LXB: i32 = 0;
    let mut LXE: i32 = 0;
    let mut NCNS: i32 = 0;
    let mut NTAB: i32 = 0;
    let mut OPCODE: i32 = 0;
    let mut SB: i32 = 0;
    let mut SE: i32 = 0;
    let mut TABIDX: i32 = 0;

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
    // No error to start with.
    //
    *ERROR = false;
    fstr::assign(ERRMSG, b" ");
    *ERRPTR = 0;

    fstr::assign(&mut TOUCHC, fstr::substr(QUERY, 1..=1));

    //
    // The query must have had names resolved at this point, or it's no
    // go.
    //
    ZZEKREQI(EQRYI.as_slice(), b"NAMES_RESOLVED", &mut IRSOLV, ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    if (IRSOLV == IFALSE) {
        CHKIN(b"ZZEKTRES", ctx)?;
        SETMSG(b"Names are not resolved in encoded query.", ctx);
        SIGERR(b"SPICE(NAMESNOTRESOLVED)", ctx)?;
        CHKOUT(b"ZZEKTRES", ctx)?;
        return Ok(());
    }

    //
    // Time strings occur only on the right sides of constraints.
    // Examine each constraint that compares a column and a value.
    //
    ZZEKREQI(EQRYI.as_slice(), b"NUM_TABLES", &mut NTAB, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"NUM_CONSTRAINTS", &mut NCNS, ctx)?;

    for I in 1..=NCNS {
        //
        // Calculate the base address of the constraint.
        //
        BASE = ((EQVBAS + ((NTAB * 2) * EQVDSZ)) + ((I - 1) * EQCDSZ));

        //
        // Obtain the constraint type.  If the RHS is not a value or if
        // the RHS is null (as indicated by the opcode), we can skip it.
        //
        CNSTYP = EQRYI[(BASE + EQCTYP)];
        OPCODE = EQRYI[(BASE + EQOPCD)];

        if (((CNSTYP == EQVAL) && (OPCODE != ISNULL)) && (OPCODE != NOTNUL)) {
            //
            // Get the index of the table containing the LHS column, and
            // get the index of this column within that table.  Get the
            // table name, then get the column's attributes.
            //
            TABIDX = EQRYI[(((BASE + EQLTAB) - 1) + EQTORD)];
            COLIDX = EQRYI[(((BASE + EQLCOL) - 1) + EQCIDX)];

            ZZEKQTAB(
                EQRYI.as_slice(),
                EQRYC,
                TABIDX,
                TABLE.first_mut(),
                ALIAS.first_mut(),
                ctx,
            )?;

            EKCII(
                TABLE.first(),
                COLIDX,
                &mut COLNAM,
                ATTDSC.as_slice_mut(),
                ctx,
            )?;

            DTYPE = ATTDSC[2];

            if (DTYPE == TIME) {
                //
                // The RHS points to a string representing a time
                // value.
                //
                LXB = EQRYI[(((BASE + EQBVAL) - 1) + EQBLEX)];
                LXE = EQRYI[(((BASE + EQBVAL) - 1) + EQBLEX)];
                SB = EQRYI[(((BASE + EQBVAL) - 1) + EQBSTR)];
                SE = EQRYI[(((BASE + EQBVAL) - 1) + EQESTR)];
                fstr::assign(&mut TIMSTR, fstr::substr(EQRYC, SB..=SE));

                //
                // Convert the time to ET, if possible.
                //
                ZZEKTCNV(&TIMSTR, &mut ET, ERROR, ERRMSG, ctx)?;

                if *ERROR {
                    *ERRPTR = SB;
                    return Ok(());
                }
                //
                // Insert the ET value into the query, and replace the
                // value descriptor for the time string.
                //
                ZZEKINQN(
                    ET,
                    TIME,
                    LXB,
                    LXE,
                    EQRYI.as_slice_mut(),
                    EQRYD.as_slice_mut(),
                    DESCR.as_slice_mut(),
                    ctx,
                )?;

                MOVEI(
                    DESCR.as_slice(),
                    EQVDSZ,
                    EQRYI.subarray_mut((BASE + EQBVAL)),
                );
            }
            //
            // We've parsed a time string, if the current column's type
            // was TIME.
            //
        }
        //
        // We've examined the current constraint, if it compares a
        // column with a value.
        //
    }

    //
    // Indicate completion of time resolution.
    //
    ZZEKWEQI(b"TIMES_RESOLVED", ITRUE, EQRYI.as_slice_mut(), ctx)?;

    Ok(())
}
