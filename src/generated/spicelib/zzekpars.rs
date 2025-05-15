//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TKKEY: i32 = 1;
const TKID: i32 = (TKKEY + 1);
const TKINT: i32 = (TKID + 1);
const TKDP: i32 = (TKINT + 1);
const TKQSTR: i32 = (TKDP + 1);
const TKLPAR: i32 = (TKQSTR + 1);
const TKRPAR: i32 = (TKLPAR + 1);
const TKCOMA: i32 = (TKRPAR + 1);
const TKDOT: i32 = (TKCOMA + 1);
const TKSTAR: i32 = (TKDOT + 1);
const TKEOQ: i32 = (TKSTAR + 1);
const KWLEN: i32 = 32;
const NKEYWD: i32 = 29;
const KWALL: i32 = 1;
const KWAND: i32 = (KWALL + 1);
const KWASND: i32 = (KWAND + 1);
const KWAVG: i32 = (KWASND + 1);
const KWBETW: i32 = (KWAVG + 1);
const KWBY: i32 = (KWBETW + 1);
const KWCNT: i32 = (KWBY + 1);
const KWDSND: i32 = (KWCNT + 1);
const KWDSTN: i32 = (KWDSND + 1);
const KWEQ: i32 = (KWDSTN + 1);
const KWFROM: i32 = (KWEQ + 1);
const KWGE: i32 = (KWFROM + 1);
const KWGRP: i32 = (KWGE + 1);
const KWGT: i32 = (KWGRP + 1);
const KWHAV: i32 = (KWGT + 1);
const KWIS: i32 = (KWHAV + 1);
const KWLE: i32 = (KWIS + 1);
const KWLIKE: i32 = (KWLE + 1);
const KWLT: i32 = (KWLIKE + 1);
const KWMAX: i32 = (KWLT + 1);
const KWMIN: i32 = (KWMAX + 1);
const KWNE: i32 = (KWMIN + 1);
const KWNOT: i32 = (KWNE + 1);
const KWNULL: i32 = (KWNOT + 1);
const KWOR: i32 = (KWNULL + 1);
const KWORDR: i32 = (KWOR + 1);
const KWSEL: i32 = (KWORDR + 1);
const KWSUM: i32 = (KWSEL + 1);
const KWWHER: i32 = (KWSUM + 1);
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
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
const LBCELL: i32 = -5;
const FROM: i32 = 0;
const FRMTAB: i32 = (FROM + 1);
const FRMCNT: i32 = (FRMTAB + 1);
const FRMALS: i32 = (FRMCNT + 1);
const WHERE: i32 = (FRMALS + 1);
const ORDER: i32 = (WHERE + 1);
const ORDRBY: i32 = (ORDER + 1);
const ORDTAB: i32 = (ORDRBY + 1);
const ORDNAM: i32 = (ORDTAB + 1);
const ORDCOL: i32 = (ORDNAM + 1);
const ORDSNS: i32 = (ORDCOL + 1);
const SELKEY: i32 = (ORDSNS + 1);
const SELECT: i32 = (SELKEY + 1);
const SELTAB: i32 = (SELECT + 1);
const SELNAM: i32 = (SELTAB + 1);
const SELCOL: i32 = (SELNAM + 1);
const TERM: i32 = (SELCOL + 1);
const MAXREL: i32 = 200;
const TYPLEN: i32 = 32;
const KEYLEN: i32 = 32;

//$Procedure      ZZEKPARS ( EK, parse tokenized EK query )
pub fn ZZEKPARS(
    QUERY: &[u8],
    NTOKEN: i32,
    LXBEGS: &[i32],
    LXENDS: &[i32],
    TOKENS: &[i32],
    VALUES: &[i32],
    NUMVLS: &[f64],
    CHRBUF: &[u8],
    CHBEGS: &[i32],
    CHENDS: &[i32],
    EQRYI: &mut [i32],
    EQRYC: &mut [u8],
    EQRYD: &mut [f64],
    ERROR: &mut bool,
    PRSERR: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let LXBEGS = DummyArray::new(LXBEGS, 1..);
    let LXENDS = DummyArray::new(LXENDS, 1..);
    let TOKENS = DummyArray::new(TOKENS, 1..);
    let VALUES = DummyArray::new(VALUES, 1..);
    let NUMVLS = DummyArray::new(NUMVLS, 1..);
    let CHBEGS = DummyArray::new(CHBEGS, 1..);
    let CHENDS = DummyArray::new(CHENDS, 1..);
    let mut EQRYI = DummyArrayMut::new(EQRYI, LBCELL..);
    let mut EQRYD = DummyArrayMut::new(EQRYD, 1..);
    let mut ERRTYP = [b' '; TYPLEN as usize];
    let EXPKEY = [b' '; KEYLEN as usize];
    let mut ALSDSC = StackArray::<i32, 6>::new(1..=EQVDSZ);
    let mut B: i32 = 0;
    let mut COLDSC = StackArray::<i32, 6>::new(1..=EQVDSZ);
    let mut E: i32 = 0;
    let mut I: i32 = 0;
    let mut L: i32 = 0;
    let mut LXB: i32 = 0;
    let mut LXE: i32 = 0;
    let mut NAMDSC = StackArray::<i32, 6>::new(1..=EQVDSZ);
    let mut NCNSTR: i32 = 0;
    let mut NORDER: i32 = 0;
    let mut NSEL: i32 = 0;
    let mut NTABS: i32 = 0;
    let mut STATE: i32 = 0;
    let mut TABDSC = StackArray::<i32, 6>::new(1..=EQVDSZ);
    let mut TOKEN: i32 = 0;
    let mut TOKNUM: i32 = 0;
    let mut VALDSC = StackArray::<i32, 6>::new(1..=EQVDSZ);
    let mut FND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // State parameters
    //

    //
    // Other local parameters
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
        CHKIN(b"ZZEKPARS", ctx)?;
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

    if FAILED(ctx) {
        *ERROR = true;
        fstr::assign(PRSERR, b"SPICE(BUG):  encoded query init failed.");
        CHKOUT(b"ZZEKPARS", ctx)?;
        return Ok(());
    }

    //
    // The structure of a query is
    //
    //    <QUERY>                 =>    <SELECT clause> <FROM clause>
    //                                  <WHERE clause> <ORDER BY clause>
    //
    //    <SELECT clause>         =>    SELECT <select list>
    //
    //    <select list>           =>    <column entry>
    //                                | <select list>, <column entry>
    //
    //    <column entry>          =>    <table name>.<column name>
    //                                | <column name>
    //
    //    <FROM clause>           =>    <table name list>
    //
    //    <table name list>       =>    <table entry>
    //                                | <table name list>, <table entry>
    //
    //    <table entry>           =>    <table name>
    //                                | <table name> <table alias>
    //
    //    <WHERE clause>          =>    WHERE <constraint expression>
    //                                | <NIL>
    //
    //    <ORDER BY clause>       =>    ORDER BY <order-by list>
    //                                | <NIL>
    //
    //    <order-by list>         =>    <order-by column entry>
    //                                | <order-by list>,
    //                                  <order-by column entry>
    //
    //    <order-by column entry> =>    <column entry> <order>
    //
    //    <order>                 =>    ASC
    //                                | DESC
    //                                | <NIL>
    //
    //
    //
    // We'll parse the clauses of the query in the following order:
    //
    //    FROM
    //    WHERE     (if present)
    //    ORDER BY  (if present)
    //    SELECT
    //
    //
    ZZEKTLOC(
        TKKEY,
        KWFROM,
        NTOKEN,
        TOKENS.as_slice(),
        VALUES.as_slice(),
        &mut TOKNUM,
        &mut FND,
    );

    if !FND {
        *ERROR = true;
        fstr::assign(&mut ERRTYP, b"FROM_NOT_FOUND");
        STATE = TERM;
    } else {
        STATE = FROM;
        NTABS = 0;
        NSEL = 0;
        NCNSTR = 0;
        NORDER = 0;
        *ERROR = false;
        fstr::assign(PRSERR, b" ");
        fstr::assign(&mut ERRTYP, b" ");
    }

    while (STATE != TERM) {
        //
        // Advance to the next token, if there is one.
        //
        TOKNUM = (TOKNUM + 1);

        if (TOKNUM > NTOKEN) {
            //
            // We're out of tokens.  Set the token value to indicate
            // `end of query'.
            //
            TOKEN = TKEOQ;
        } else {
            TOKEN = TOKENS[TOKNUM];
        }

        //
        // Perform semantic actions based on the state and current token.
        //
        if (STATE == FROM) {
            //
            // We expect to see an identifier representing a table name.
            // No other tokens are allowed.
            //
            if (TOKEN == TKID) {
                //
                // We've found a table name (as far as we can tell at
                // this point).  Make sure we haven't exceeded the limit
                // for table names; if not, add the appropriate information
                // to the encoded query.
                //
                NTABS = (NTABS + 1);

                if (NTABS > MAXTAB) {
                    *ERROR = true;
                    fstr::assign(&mut ERRTYP, b"TOO_MANY_TABLES");
                    STATE = TERM;
                } else {
                    I = VALUES[TOKNUM];
                    B = CHBEGS[I];
                    E = CHENDS[I];
                    L = ((E - B) + 1);
                    LXB = LXBEGS[TOKNUM];
                    LXE = LXENDS[TOKNUM];

                    ZZEKINQC(
                        fstr::substr(CHRBUF, B..=E),
                        L,
                        LXB,
                        LXE,
                        EQRYI.as_slice_mut(),
                        EQRYC,
                        TABDSC.as_slice_mut(),
                        ctx,
                    )?;

                    //
                    // Append the table descriptor to the integer part of the
                    // query.
                    //
                    for J in 1..=EQVDSZ {
                        APPNDI(TABDSC[J], EQRYI.as_slice_mut(), ctx)?;
                    }

                    //
                    // Add a place-holder value descriptor to reserve
                    // space for an alias descriptor for this table.  If an
                    // actual alias is supplied, we'll update this
                    // descriptor.
                    //
                    CLEARI(EQVDSZ, ALSDSC.as_slice_mut());
                    ALSDSC[EQDTYP] = CHR;

                    for J in 1..=EQVDSZ {
                        APPNDI(ALSDSC[J], EQRYI.as_slice_mut(), ctx)?;
                    }

                    //
                    // Update the table count in the encoded query.
                    //
                    ZZEKWEQI(b"NUM_TABLES", NTABS, EQRYI.as_slice_mut(), ctx)?;

                    STATE = FRMTAB;
                }
            } else if (TOKEN == TKEOQ) {
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"MORE_TOKENS_EXP");
                STATE = TERM;
            } else {
                //
                // We've got the wrong kind of token here.
                //
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"TABLE_EXP");
                STATE = TERM;
            }
        //
        // State is a member of {FRMTAB, TERM}.
        //
        } else if (STATE == FRMTAB) {
            //
            // We should see a comma, an alias, one of the SELECT,
            // WHERE or ORDER keywords, or the end of the query.
            //
            if (TOKEN == TKEOQ) {
                //
                // We're out of tokens.  It's time to parse the
                // WHERE clause.
                //
                STATE = WHERE;
            } else if (TOKEN == TKCOMA) {
                //
                // It's time to look for another table name.
                //
                STATE = FROM;
            } else if (TOKEN == TKID) {
                //
                // We've got an alias.  Add this string to the encoded
                // query.
                //
                I = VALUES[TOKNUM];
                B = CHBEGS[I];
                E = CHENDS[I];
                L = ((E - B) + 1);
                LXB = LXBEGS[TOKNUM];
                LXE = LXENDS[TOKNUM];

                ZZEKINQC(
                    fstr::substr(CHRBUF, B..=E),
                    L,
                    LXB,
                    LXE,
                    EQRYI.as_slice_mut(),
                    EQRYC,
                    ALSDSC.as_slice_mut(),
                    ctx,
                )?;

                //
                // Update the place-holder alias descriptor in the integer
                // part of the query.
                //
                MOVEI(
                    ALSDSC.as_slice(),
                    EQVDSZ,
                    EQRYI.subarray_mut(((CARDI(EQRYI.as_slice(), ctx)? - EQVDSZ) + 1)),
                );

                STATE = FRMALS;
            } else if (TOKEN == TKKEY) {
                //
                // The last table name in the FROM clause is followed by
                // a keyword.  SELECT, WHERE and ORDER are the only valid
                // possibilities.
                //
                if (((VALUES[TOKNUM] != KWWHER) && (VALUES[TOKNUM] != KWSEL))
                    && (VALUES[TOKNUM] != KWORDR))
                {
                    //
                    // We've got a keyword we don't want here.
                    //
                    *ERROR = true;
                    fstr::assign(&mut ERRTYP, b"BAD_KEYWORD");
                    STATE = TERM;
                } else {
                    //
                    // Parse the WHERE clause.
                    //
                    STATE = WHERE;
                }
            } else {
                //
                // We've got the wrong kind of token altogether.
                //
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"ALIAS_EXP");
                STATE = TERM;
            }

        //
        // STATE is a member of {FROM, FRMALS, WHERE, TERM}.
        //
        } else if (STATE == FRMALS) {
            //
            // We should see a comma, the SELECT, WHERE or ORDER
            // keywords, or the end of the query.
            //
            if (TOKEN == TKEOQ) {
                //
                // We're out of tokens.  It's time to parse the
                // WHERE clause.
                //
                STATE = WHERE;
            } else if (TOKEN == TKCOMA) {
                //
                // It's time to look for another table name.
                //
                STATE = FROM;
            } else if (TOKEN == TKKEY) {
                //
                // The last table name in the FROM clause is followed by
                // a keyword.  SELECT, WHERE and ORDER are the only valid
                // possibilities.
                //
                if (((VALUES[TOKNUM] != KWWHER) && (VALUES[TOKNUM] != KWSEL))
                    && (VALUES[TOKNUM] != KWORDR))
                {
                    //
                    // We've got a keyword we don't want here.
                    //
                    *ERROR = true;
                    fstr::assign(&mut ERRTYP, b"BAD_KEYWORD");
                    STATE = TERM;
                } else {
                    //
                    // Parse the WHERE clause.
                    //
                    STATE = WHERE;
                }
            } else {
                //
                // We've got the wrong kind of token altogether.
                //
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"COMMA_OR_KEY_EXP");
                STATE = TERM;
            }
        //
        // STATE is a member of {FROM, WHERE, TERM}.
        //
        } else if (STATE == SELKEY) {
            //
            // It's time to parse the SELECT clause.  We'll need to
            // locate the SELECT keyword.
            //
            ZZEKTLOC(
                TKKEY,
                KWSEL,
                NTOKEN,
                TOKENS.as_slice(),
                VALUES.as_slice(),
                &mut TOKNUM,
                &mut FND,
            );

            if !FND {
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"SELECT_NOT_FOUND");
                STATE = TERM;
            } else {
                STATE = SELECT;
            }
        } else if (STATE == SELECT) {
            //
            // We must see either the * token, the ALL keyword,
            // or an identifier here.  The identifier may be a lone
            // column name, or it may be a column name qualified by a
            // table name or alias.
            //
            // For the moment, we don't support the * or ALL options.
            //
            if (TOKEN == TKID) {
                //
                // We've found a name (as far as we can tell at this point).
                // Make sure we haven't exceeded the limit for SELECT
                // column names; if not, store the name string in the
                // encoded query, and save the descriptor until we've
                // figured out whether we're looking at a column name or
                // table name.
                //
                NSEL = (NSEL + 1);

                if (NSEL > MAXSEL) {
                    *ERROR = true;
                    fstr::assign(&mut ERRTYP, b"TOO_MANY_SEL_COLS");
                    STATE = TERM;
                } else {
                    I = VALUES[TOKNUM];
                    B = CHBEGS[I];
                    E = CHENDS[I];
                    L = ((E - B) + 1);
                    LXB = LXBEGS[TOKNUM];
                    LXE = LXENDS[TOKNUM];

                    ZZEKINQC(
                        fstr::substr(CHRBUF, B..=E),
                        L,
                        LXB,
                        LXE,
                        EQRYI.as_slice_mut(),
                        EQRYC,
                        NAMDSC.as_slice_mut(),
                        ctx,
                    )?;

                    //
                    // Add a place-holder value descriptor to reserve
                    // space for a table descriptor for this name.  If it
                    // turns out that the current name is a table name, we'll
                    // update this descriptor.
                    //
                    CLEARI(EQVDSZ, VALDSC.as_slice_mut());
                    VALDSC[EQDTYP] = CHR;

                    for J in 1..=EQVDSZ {
                        APPNDI(VALDSC[J], EQRYI.as_slice_mut(), ctx)?;
                    }

                    //
                    // Update the SELECT column count in the encoded query.
                    //
                    ZZEKWEQI(b"NUM_SELECT_COLS", NSEL, EQRYI.as_slice_mut(), ctx)?;

                    STATE = SELNAM;
                }
            } else if (TOKEN == TKEOQ) {
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"MORE_TOKENS_EXP");
                STATE = TERM;
            } else {
                //
                // We've got the wrong kind of token here.
                //
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"TABLE_OR_COLUMN_EXP");
                STATE = TERM;
            }
        //
        // State is a member of {SELNAM, TERM}.
        //
        } else if (STATE == SELNAM) {
            //
            // We've seen a SELECT column name, or else the name
            // of a table qualifying a SELECT column name.
            //
            if (TOKEN == TKEOQ) {
                //
                // The name we picked up was an unqualified column
                // name.  Append the saved name descriptor to the encoded
                // query.
                //
                for J in 1..=EQVDSZ {
                    APPNDI(NAMDSC[J], EQRYI.as_slice_mut(), ctx)?;
                }

                STATE = TERM;
            } else if (TOKEN == TKCOMA) {
                //
                // The name we picked up was an unqualified column
                // name.  Append the saved name descriptor to the encoded
                // query.  Another name should follow.
                //
                for J in 1..=EQVDSZ {
                    APPNDI(NAMDSC[J], EQRYI.as_slice_mut(), ctx)?;
                }

                STATE = SELECT;
            } else if (TOKEN == TKDOT) {
                //
                // The name we picked up was a table name or alias.  A
                // column name should follow.
                //
                STATE = SELTAB;
            } else if (TOKEN == TKKEY) {
                //
                // We have the last column name in the SELECT clause.
                //
                // Append the saved name descriptor to the encoded
                // query.
                //
                for J in 1..=EQVDSZ {
                    APPNDI(NAMDSC[J], EQRYI.as_slice_mut(), ctx)?;
                }

                //
                // The last column name in the SELECT clause is followed by
                // a keyword.  FROM, WHERE and ORDER are the only valid
                // possibilities.
                //
                if (((VALUES[TOKNUM] == KWWHER) || (VALUES[TOKNUM] == KWFROM))
                    || (VALUES[TOKNUM] == KWORDR))
                {
                    //
                    // We're done with the SELECT clause.
                    //
                    STATE = TERM;
                } else {
                    //
                    // We've got a keyword we don't want here.
                    //
                    *ERROR = true;
                    fstr::assign(&mut ERRTYP, b"BAD_KEYWORD");
                    STATE = TERM;
                }
            } else {
                //
                // We've got the wrong kind of token here.
                //
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"BAD_TOKEN");
                STATE = TERM;
            }
        //
        // STATE is a member of {SELECT, SELTAB, TERM}.
        //
        } else if (STATE == SELTAB) {
            //
            // We've picked up a qualifying table name for a SELECT
            // column.  We must see a column name here.
            //
            if (TOKEN == TKID) {
                //
                // Update the place-holder table name descriptor in the
                // encoded query.
                //
                MOVEI(
                    NAMDSC.as_slice(),
                    EQVDSZ,
                    EQRYI.subarray_mut(((CARDI(EQRYI.as_slice(), ctx)? - EQVDSZ) + 1)),
                );

                //
                // Add the column name to the character part of the
                // encoded query.
                //
                I = VALUES[TOKNUM];
                B = CHBEGS[I];
                E = CHENDS[I];
                L = ((E - B) + 1);
                LXB = LXBEGS[TOKNUM];
                LXE = LXENDS[TOKNUM];

                ZZEKINQC(
                    fstr::substr(CHRBUF, B..=E),
                    L,
                    LXB,
                    LXE,
                    EQRYI.as_slice_mut(),
                    EQRYC,
                    COLDSC.as_slice_mut(),
                    ctx,
                )?;

                //
                // Add the descriptor for the column name to the encoded
                // query.
                //
                for J in 1..=EQVDSZ {
                    APPNDI(COLDSC[J], EQRYI.as_slice_mut(), ctx)?;
                }

                STATE = SELCOL;
            } else if (TOKEN == TKEOQ) {
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"MORE_TOKENS_EXP");
                STATE = TERM;
            } else {
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"COLUMN_EXP");
                STATE = TERM;
            }
        //
        // STATE is a member of {SELCOL, TERM}.
        //
        } else if (STATE == SELCOL) {
            //
            // We've picked up a qualified column name.  At this point,
            // we should see a keyword, a comma, or the end of the
            // query.
            //
            if (TOKEN == TKKEY) {
                //
                // The last column name in the SELECT clause is followed by
                // a keyword.  FROM, WHERE and ORDER are the only valid
                // possibilities.
                //
                if (((VALUES[TOKNUM] == KWWHER) || (VALUES[TOKNUM] == KWFROM))
                    || (VALUES[TOKNUM] == KWORDR))
                {
                    //
                    // We're done with the SELECT clause.
                    //
                    STATE = TERM;
                } else {
                    //
                    // We've got a keyword we don't want here.
                    //
                    *ERROR = true;
                    fstr::assign(&mut ERRTYP, b"BAD_KEYWORD");
                    STATE = TERM;
                }
            } else if (TOKEN == TKCOMA) {
                //
                // We expect another SELECT column.
                //
                STATE = SELECT;
            } else if (TOKEN == TKEOQ) {
                //
                // We're done with the SELECT clause.
                //
                STATE = TERM;
            } else {
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"COMMA_OR_KEY_EXP");
                STATE = TERM;
            }
        //
        // STATE is a member of {SELECT, TERM}.
        //
        } else if (STATE == WHERE) {
            //
            // The WHERE clause is optional.  See whether we have one.  The
            // clause is started by a WHERE keyword.
            //
            ZZEKTLOC(
                TKKEY,
                KWWHER,
                NTOKEN,
                TOKENS.as_slice(),
                VALUES.as_slice(),
                &mut TOKNUM,
                &mut FND,
            );

            if FND {
                //
                // We're going to hand off the list of tokens that comprise
                // the WHERE clause of the query to a routine that will
                // parse the tokens and form a list of relational
                // constraints.  Once this is done, all we have to do here
                // is check the validity of the column names and the values
                // used in the constraints.
                //
                ZZEKNRML(
                    QUERY,
                    NTOKEN,
                    LXBEGS.as_slice(),
                    LXENDS.as_slice(),
                    TOKENS.as_slice(),
                    VALUES.as_slice(),
                    NUMVLS.as_slice(),
                    CHRBUF,
                    CHBEGS.as_slice(),
                    CHENDS.as_slice(),
                    EQRYI.as_slice_mut(),
                    EQRYC,
                    EQRYD.as_slice_mut(),
                    ERROR,
                    PRSERR,
                    ctx,
                )?;

                if *ERROR {
                    fstr::assign(&mut ERRTYP, b"WHERE_ERROR");
                    STATE = TERM;
                } else {
                    //
                    // Parse the ORDER BY clause, if one is present.
                    //
                    STATE = ORDER;
                }
            } else {
                //
                // Parse the ORDER BY clause, if one is present.
                //
                STATE = ORDER;
            }
        //
        // STATE is a member of {ORDER, TERM}.
        //
        } else if (STATE == ORDER) {
            //
            // The ORDER BY clause is optional.  See whether we have one.
            // The clause is started by an ORDER keyword.
            //
            ZZEKTLOC(
                TKKEY,
                KWORDR,
                NTOKEN,
                TOKENS.as_slice(),
                VALUES.as_slice(),
                &mut TOKNUM,
                &mut FND,
            );

            if FND {
                //
                // The BY keyword should follow the ORDER keyword.
                //
                if (TOKNUM < NTOKEN) {
                    TOKNUM = (TOKNUM + 1);

                    if ((TOKENS[TOKNUM] == TKKEY) && (VALUES[TOKNUM] == KWBY)) {
                        //
                        // We're ready to parse the ORDER BY clause.
                        //
                        STATE = ORDRBY;
                    } else {
                        //
                        // No BY keyword followed the ORDER keyword.
                        //
                        *ERROR = true;
                        fstr::assign(&mut ERRTYP, b"BY_EXPECTED");
                        STATE = TERM;
                    }
                } else {
                    //
                    // We're out of tokens where we shouldn't be.
                    //
                    *ERROR = true;
                    fstr::assign(&mut ERRTYP, b"BY_EXPECTED");
                    STATE = TERM;
                }
            } else {
                //
                // We're ready to go on to the SELECT clause.
                //
                STATE = SELKEY;
            }
        //
        // STATE is a member of {ORDRBY, SELKEY, TERM}.
        //
        } else if (STATE == ORDRBY) {
            //
            // We must see a name in the order column list here.
            // The name may be a lone column name, or it may be a column
            // name qualified by a table name or alias.
            //
            if (TOKEN == TKID) {
                //
                // We've found a name (as far as we can tell at this point).
                // Make sure we haven't exceeded the limit for order-by
                // column names; if not, store the name string in the
                // encoded query, and save the descriptor until we've
                // figured out whether we're looking at a column name or
                // table name.
                //
                NORDER = (NORDER + 1);

                if (NORDER > MAXORD) {
                    *ERROR = true;
                    fstr::assign(&mut ERRTYP, b"TOO_MANY_ORD_COLS");
                    STATE = TERM;
                } else {
                    I = VALUES[TOKNUM];
                    B = CHBEGS[I];
                    E = CHENDS[I];
                    L = ((E - B) + 1);
                    LXB = LXBEGS[TOKNUM];
                    LXE = LXENDS[TOKNUM];

                    ZZEKINQC(
                        fstr::substr(CHRBUF, B..=E),
                        L,
                        LXB,
                        LXE,
                        EQRYI.as_slice_mut(),
                        EQRYC,
                        NAMDSC.as_slice_mut(),
                        ctx,
                    )?;

                    //
                    // Add a place-holder value descriptor to reserve
                    // space for a table descriptor for this name.  If it
                    // turns out that the current name is a table name, we'll
                    // update this descriptor.
                    //
                    CLEARI(EQVDSZ, VALDSC.as_slice_mut());
                    VALDSC[EQDTYP] = CHR;

                    for J in 1..=EQVDSZ {
                        APPNDI(VALDSC[J], EQRYI.as_slice_mut(), ctx)?;
                    }

                    //
                    // Update the order-by column count in the encoded query.
                    //
                    ZZEKWEQI(b"NUM_ORDERBY_COLS", NORDER, EQRYI.as_slice_mut(), ctx)?;

                    STATE = ORDNAM;
                }
            } else if (TOKEN == TKEOQ) {
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"MORE_TOKENS_EXP");
                STATE = TERM;
            } else {
                //
                // We've got the wrong kind of token here.
                //
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"TABLE_OR_COLUMN_EXP");
                STATE = TERM;
            }
        //
        // State is a member of {ORDNAM, TERM}.
        //
        } else if (STATE == ORDNAM) {
            //
            // We've seen an order-by column name, or else the name
            // of a table qualifying an order-by column name.
            //
            if (TOKEN == TKEOQ) {
                //
                // The name we picked up was an unqualified column
                // name.  Append the saved name descriptor to the encoded
                // query.
                //
                for J in 1..=EQVDSZ {
                    APPNDI(NAMDSC[J], EQRYI.as_slice_mut(), ctx)?;
                }

                //
                // Since no ASCENDING or DESCENDING sense keyword was
                // supplied, append the default value ASCENDING to the
                // order-by column descriptor in the encoded query.
                //
                APPNDI(EQASND, EQRYI.as_slice_mut(), ctx)?;

                //
                // We're done with the ORDER BY clause; go on to parse the
                // SELECT clause.
                //
                STATE = SELKEY;
            } else if (TOKEN == TKCOMA) {
                //
                // The name we picked up was an unqualified column
                // name.  Append the saved name descriptor to the encoded
                // query.  Another name should follow.
                //
                for J in 1..=EQVDSZ {
                    APPNDI(NAMDSC[J], EQRYI.as_slice_mut(), ctx)?;
                }

                //
                // Since no ASCENDING or DESCENDING sense keyword was
                // supplied, append the default value ASCENDING to the
                // order-by column descriptor in the encoded query.
                //
                APPNDI(EQASND, EQRYI.as_slice_mut(), ctx)?;

                STATE = ORDRBY;
            } else if (TOKEN == TKDOT) {
                //
                // The name we picked up was a table name or alias.  A
                // column name should follow.
                //
                STATE = ORDTAB;
            } else if (TOKEN == TKKEY) {
                //
                // We have a column name, which may be followed by a
                // keyword indicating the sense of the ordering, or may
                // be followed by a keyword starting a new clause.
                //
                // Append the saved name descriptor to the encoded
                // query.
                //
                for J in 1..=EQVDSZ {
                    APPNDI(NAMDSC[J], EQRYI.as_slice_mut(), ctx)?;
                }

                //
                // Set the sense descriptor according to the keyword we've
                // picked up.  After this, we're ready to look for another
                // order-by column.
                //
                if (VALUES[TOKNUM] == KWASND) {
                    APPNDI(EQASND, EQRYI.as_slice_mut(), ctx)?;

                    STATE = ORDSNS;
                } else if (VALUES[TOKNUM] == KWDSND) {
                    APPNDI(EQDSND, EQRYI.as_slice_mut(), ctx)?;

                    STATE = ORDSNS;
                } else if (((VALUES[TOKNUM] == KWWHER) || (VALUES[TOKNUM] == KWFROM))
                    || (VALUES[TOKNUM] == KWSEL))
                {
                    //
                    // Since no ASCENDING or DESCENDING sense keyword was
                    // supplied, append the default value ASCENDING to the
                    // order-by column descriptor in the encoded query.
                    //
                    APPNDI(EQASND, EQRYI.as_slice_mut(), ctx)?;

                    //
                    // We're done with the ORDER BY clause.  Go on to
                    // parse the SELECT clause.
                    //
                    STATE = SELKEY;
                } else {
                    //
                    // We've got a keyword we don't want here.
                    //
                    *ERROR = true;
                    fstr::assign(&mut ERRTYP, b"BAD_KEYWORD");
                    STATE = TERM;
                }
            } else {
                //
                // We've got the wrong kind of token here.
                //
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"BAD_TOKEN");
                STATE = TERM;
            }
        //
        // STATE is a member of {ORDRBY, ORDTAB, ORDSNS, SELKEY, TERM}.
        //
        } else if (STATE == ORDTAB) {
            //
            // We've picked up a qualifying table name for an order-by
            // column.  We must see a column name here.
            //
            if (TOKEN == TKID) {
                //
                // Update the place-holder table name descriptor in the
                // encoded query.
                //
                MOVEI(
                    NAMDSC.as_slice(),
                    EQVDSZ,
                    EQRYI.subarray_mut(((CARDI(EQRYI.as_slice(), ctx)? - EQVDSZ) + 1)),
                );

                //
                // Add the column name to the character part of the
                // encoded query.
                //
                I = VALUES[TOKNUM];
                B = CHBEGS[I];
                E = CHENDS[I];
                L = ((E - B) + 1);
                LXB = LXBEGS[TOKNUM];
                LXE = LXENDS[TOKNUM];

                ZZEKINQC(
                    fstr::substr(CHRBUF, B..=E),
                    L,
                    LXB,
                    LXE,
                    EQRYI.as_slice_mut(),
                    EQRYC,
                    COLDSC.as_slice_mut(),
                    ctx,
                )?;

                //
                // Add the descriptor for the column name to the encoded
                // query.
                //
                for J in 1..=EQVDSZ {
                    APPNDI(COLDSC[J], EQRYI.as_slice_mut(), ctx)?;
                }

                STATE = ORDCOL;
            } else if (TOKEN == TKEOQ) {
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"MORE_TOKENS_EXP");
                STATE = TERM;
            } else {
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"COLUMN_EXP");
                STATE = TERM;
            }
        //
        // STATE is a member of {ORDCOL, TERM}.
        //
        } else if (STATE == ORDCOL) {
            //
            // We've picked up a qualified column name.  At this point,
            // we should see a sense keyword, a comma, the end of the
            // query, or one of the FROM, SELECT, or WHERE keywords.
            //
            if (TOKEN == TKKEY) {
                if (VALUES[TOKNUM] == KWASND) {
                    //
                    // The ASCENDING keyword has been supplied.  After this,
                    // look for another column.
                    //
                    APPNDI(EQASND, EQRYI.as_slice_mut(), ctx)?;

                    STATE = ORDSNS;
                } else if (VALUES[TOKNUM] == KWDSND) {
                    //
                    // The DESCENDING keyword has been supplied.  After this,
                    // look for another column.
                    //
                    APPNDI(EQDSND, EQRYI.as_slice_mut(), ctx)?;

                    STATE = ORDSNS;
                } else if (((VALUES[TOKNUM] == KWWHER) || (VALUES[TOKNUM] == KWFROM))
                    || (VALUES[TOKNUM] == KWSEL))
                {
                    //
                    // We're done with the ORDER BY clause.  Go on to
                    // parse the SELECT clause.
                    //
                    STATE = SELKEY;
                } else {
                    *ERROR = true;
                    fstr::assign(&mut ERRTYP, b"BAD_KEYWORD");
                    STATE = TERM;
                }
            } else if (TOKEN == TKCOMA) {
                //
                // The ASCENDING keyword is implied.
                //
                APPNDI(EQASND, EQRYI.as_slice_mut(), ctx)?;

                STATE = ORDRBY;
            } else if (TOKEN == TKEOQ) {
                //
                // The ASCENDING keyword is implied.
                //
                APPNDI(EQASND, EQRYI.as_slice_mut(), ctx)?;

                //
                // We're done with the ORDER BY clause.  Parse the SELECT
                // clause.
                //
                STATE = SELKEY;
            } else {
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"COMMA_OR_KEY_EXP");
                STATE = TERM;
            }
        //
        // STATE is a member of {ORDRBY, ORDSNS, SELKEY, TERM}.
        //
        } else if (STATE == ORDSNS) {
            //
            // We've picked up an order sense keyword.  At this point,
            // we should see comma or the end of the query, or one of the
            // FROM, SELECT, or WHERE keywords.
            //
            if (TOKEN == TKCOMA) {
                //
                // We're ready to look for another column.
                //
                STATE = ORDRBY;
            } else if (TOKEN == TKEOQ) {
                //
                // We're done with the ORDER BY clause.  Parse the SELECT
                // clause.
                //
                STATE = SELKEY;
            } else if (TOKEN == TKKEY) {
                if (((VALUES[TOKNUM] == KWWHER) || (VALUES[TOKNUM] == KWFROM))
                    || (VALUES[TOKNUM] == KWSEL))
                {
                    //
                    // We're done with the ORDER BY clause.  Go on to
                    // parse the SELECT clause.
                    //
                    STATE = SELKEY;
                } else {
                    *ERROR = true;
                    fstr::assign(&mut ERRTYP, b"BAD_KEYWORD");
                    STATE = TERM;
                }
            } else {
                *ERROR = true;
                fstr::assign(&mut ERRTYP, b"COMMA_EXP");
                STATE = TERM;
            }
        //
        // STATE is a member of {ORDRBY, SELKEY, TERM}.
        //
        } else {
            //
            // Somehow, we've reached an invalid state.
            //
            *ERROR = true;
            fstr::assign(PRSERR, b"SPICE(BUG) -- Invalid state reached in EK parser.");
            STATE = TERM;
        }
        //
        // STATE is a member of {ORDRBY, TERM}.
        //
    }
    //
    // At this point, either an error has been detected, or the query
    // has been parsed, and the query is represented in encoded form
    // in the outputs EQRYI, EQRYC, and EQRYD.
    //

    //
    // We centralize construction of error messages in the following
    // section.
    //
    if *ERROR {
        if fstr::eq(&ERRTYP, b"FROM_NOT_FOUND") {
            fstr::assign(
                PRSERR,
                b"Every query must contain a FROM clause. The FROM keyword was not found.",
            );
        } else if fstr::eq(&ERRTYP, b"SELECT_NOT_FOUND") {
            fstr::assign(
                PRSERR,
                b"Every query must contain a SELECT clause. The SELECT keyword was not found.",
            );
        } else if fstr::eq(&ERRTYP, b"BY_EXPECTED") {
            fstr::assign(
                PRSERR,
                b"The BY keyword was not found following the ORDER keyword.",
            );
        } else if fstr::eq(&ERRTYP, b"BAD_KEYWORD") {
            LXB = LXBEGS[TOKNUM];
            LXE = LXENDS[TOKNUM];
            fstr::assign(
                PRSERR,
                b"Invalid keyword at location #. Actual token was: #",
            );
            REPMI(&PRSERR.to_vec(), b"#", LXB, PRSERR, ctx);
            REPMC(
                &PRSERR.to_vec(),
                b"#",
                fstr::substr(QUERY, LXB..=LXE),
                PRSERR,
            );
        } else if fstr::eq(&ERRTYP, b"TABLE_OR_COLUMN_EXP") {
            LXB = LXBEGS[TOKNUM];
            LXE = LXENDS[TOKNUM];
            fstr::assign(
                PRSERR,
                b"Table or column name expected at location #. Actual token was: #",
            );
            REPMI(&PRSERR.to_vec(), b"#", LXB, PRSERR, ctx);
            REPMC(
                &PRSERR.to_vec(),
                b"#",
                fstr::substr(QUERY, LXB..=LXE),
                PRSERR,
            );
        } else if fstr::eq(&ERRTYP, b"TABLE_EXP") {
            LXB = LXBEGS[TOKNUM];
            LXE = LXENDS[TOKNUM];
            fstr::assign(
                PRSERR,
                b"Table name expected at location #. Actual token was: #",
            );
            REPMI(&PRSERR.to_vec(), b"#", LXB, PRSERR, ctx);
            REPMC(
                &PRSERR.to_vec(),
                b"#",
                fstr::substr(QUERY, LXB..=LXE),
                PRSERR,
            );
        } else if fstr::eq(&ERRTYP, b"COLUMN_EXP") {
            LXB = LXBEGS[TOKNUM];
            LXE = LXENDS[TOKNUM];
            fstr::assign(
                PRSERR,
                b"Column name expected at location #. Actual token was: #",
            );
            REPMI(&PRSERR.to_vec(), b"#", LXB, PRSERR, ctx);
            REPMC(
                &PRSERR.to_vec(),
                b"#",
                fstr::substr(QUERY, LXB..=LXE),
                PRSERR,
            );
        } else if fstr::eq(&ERRTYP, b"ALIAS_EXP") {
            LXB = LXBEGS[TOKNUM];
            LXE = LXENDS[TOKNUM];
            fstr::assign(
                PRSERR,
                b"Table alias, comma, or keyword expected at location #. Actual token was: #",
            );
            REPMI(&PRSERR.to_vec(), b"#", LXB, PRSERR, ctx);
            REPMC(
                &PRSERR.to_vec(),
                b"#",
                fstr::substr(QUERY, LXB..=LXE),
                PRSERR,
            );
        } else if fstr::eq(&ERRTYP, b"COMMA_OR_KEY_EXP") {
            LXB = LXBEGS[TOKNUM];
            LXE = LXENDS[TOKNUM];
            fstr::assign(
                PRSERR,
                b"Comma or keyword expected at location #. Actual token was: #",
            );
            REPMI(&PRSERR.to_vec(), b"#", LXB, PRSERR, ctx);
            REPMC(
                &PRSERR.to_vec(),
                b"#",
                fstr::substr(QUERY, LXB..=LXE),
                PRSERR,
            );
        } else if fstr::eq(&ERRTYP, b"COMMA_EXP") {
            LXB = LXBEGS[TOKNUM];
            LXE = LXENDS[TOKNUM];
            fstr::assign(PRSERR, b"Comma expected at location #. Actual token was: #");
            REPMI(&PRSERR.to_vec(), b"#", LXB, PRSERR, ctx);
            REPMC(
                &PRSERR.to_vec(),
                b"#",
                fstr::substr(QUERY, LXB..=LXE),
                PRSERR,
            );
        } else if fstr::eq(&ERRTYP, b"MORE_TOKENS_EXP") {
            fstr::assign(PRSERR, b"More tokens were expected in query.");
        } else if fstr::eq(&ERRTYP, b"KEYWORD_EXP") {
            fstr::assign(
                PRSERR,
                b"The keyword # was expected at location #. Actual token was: #",
            );
            REPMC(&PRSERR.to_vec(), b"#", &EXPKEY, PRSERR);
            REPMI(&PRSERR.to_vec(), b"#", LXB, PRSERR, ctx);
            REPMC(
                &PRSERR.to_vec(),
                b"#",
                fstr::substr(QUERY, LXB..=LXE),
                PRSERR,
            );
        } else if fstr::eq(&ERRTYP, b"BAD_TOKEN") {
            LXB = LXBEGS[TOKNUM];
            LXE = LXENDS[TOKNUM];
            fstr::assign(PRSERR, b"Invalid token at location #. Token was: #");
            REPMI(&PRSERR.to_vec(), b"#", LXB, PRSERR, ctx);
            REPMC(
                &PRSERR.to_vec(),
                b"#",
                fstr::substr(QUERY, LXB..=LXE),
                PRSERR,
            );
        } else if fstr::eq(&ERRTYP, b"TOO_MANY_TABLES") {
            fstr::assign(
                PRSERR,
                b"Number of tables in \"FROM\" clause exceeds allowed maximum of #.",
            );
            REPMI(&PRSERR.to_vec(), b"#", MAXTAB, PRSERR, ctx);
        } else if fstr::eq(&ERRTYP, b"TOO_MANY_ORD_COLS") {
            fstr::assign(
                PRSERR,
                b"Number of order-by columns exceeds allowed maximum of #.",
            );
            REPMI(&PRSERR.to_vec(), b"#", MAXTAB, PRSERR, ctx);
        } else if fstr::eq(&ERRTYP, b"TOO_MANY_SEL_COLS") {
            fstr::assign(
                PRSERR,
                b"Number of SELECT columns exceeds allowed maximum of #.",
            );
            REPMI(&PRSERR.to_vec(), b"#", MAXSEL, PRSERR, ctx);
        } else if fstr::ne(&ERRTYP, b"WHERE_ERROR") {
            fstr::assign(PRSERR, b"SPICE(BUG)--Unrecognized error type.  Type was #.");
            REPMC(&PRSERR.to_vec(), b"#", &ERRTYP, PRSERR);
        }
    } else {
        //
        // Indicate that parsing is complete.
        //
        ZZEKWEQI(b"PARSED", ITRUE, EQRYI.as_slice_mut(), ctx)?;
    }

    CHKOUT(b"ZZEKPARS", ctx)?;
    Ok(())
}
