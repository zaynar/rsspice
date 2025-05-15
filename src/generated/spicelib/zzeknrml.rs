//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

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
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const LBCELL: i32 = -5;
const NIL: i32 = 0;
const MAXREL: i32 = (10 * MAXTOK);
const MAXMET: i32 = (10 * MAXTOK);
const LBPOOL: i32 = -5;
const MAXSTK: i32 = 500;
const NLOGOP: i32 = 3;
const NRELOP: i32 = 8;
const RGROUP: i32 = -1;
const LGROUP: i32 = (RGROUP - 1);
const BTWEEN: i32 = (LGROUP - 1);
const NOTBTW: i32 = (BTWEEN - 1);
const BTWEXP: i32 = (NOTBTW - 1);
const NAME: i32 = (BTWEXP - 1);
const IDENT: i32 = (NAME - 1);
const VALUE: i32 = (IDENT - 1);
const PERIOD: i32 = (VALUE - 1);
const AND: i32 = (PERIOD - 1);
const OR: i32 = (AND - 1);
const NOT: i32 = (OR - 1);
const EXPR: i32 = (NOT - 1);
const NENDKW: i32 = 3;
const PUSH: i32 = 0;
const POP: i32 = (PUSH + 1);
const PARSE: i32 = (POP + 1);
const REDUCD: i32 = (PARSE + 1);
const TERM: i32 = (REDUCD + 1);
const NONE: i32 = 0;
const REDUCE: i32 = 1;
const ENDGRP: i32 = (REDUCE + 1);
const DSCSIZ: i32 = (EQVDSZ + 1);

struct SaveVars {
    RELS: ActualArray2D<i32>,
    RLPOOL: ActualArray2D<i32>,
    CJPOOL: ActualArray2D<i32>,
    CJPTRS: ActualArray<i32>,
    DJPOOL: ActualArray2D<i32>,
    DJPTRS: ActualArray<i32>,
    MTPOOL: ActualArray2D<i32>,
    MTCODE: ActualArray<i32>,
    MTEXPR: ActualArray<i32>,
    MSTART: ActualArray<i32>,
    POPCND: ActualArray<i32>,
    NMETA: i32,
    B: i32,
    CJ: StackArray<i32, 4>,
    CJNODE: i32,
    COLPTR: i32,
    DJ: StackArray<i32, 2>,
    DJNODE: i32,
    DJTAIL: i32,
    DSPOOL: ActualArray2D<i32>,
    DSCBUF: ActualArray2D<i32>,
    E: i32,
    ENDKW: StackArray<i32, 3>,
    ENDLOC: i32,
    EXPRHD: i32,
    FIRST: i32,
    FOURTH: i32,
    HEAD1: i32,
    HEAD2: i32,
    I: i32,
    J: i32,
    K: i32,
    LEVEL: i32,
    LXB: i32,
    LXE: i32,
    METAHD: i32,
    NCONJ: i32,
    NEWCJ: i32,
    NEWDJ: i32,
    NEWREL: i32,
    NEXT: i32,
    NODE: i32,
    NRELS: i32,
    OP: i32,
    PREV: i32,
    REL: StackArray<i32, 4>,
    RELPTR: i32,
    RELSET: ActualArray<i32>,
    RETCND: i32,
    RHSPTR: i32,
    SECOND: i32,
    SKIP: i32,
    SIZES: ActualArray<i32>,
    START: i32,
    STATE: i32,
    TABPTR: i32,
    TAIL: i32,
    THIRD: i32,
    TYPE: i32,
    WHRBEG: i32,
    WHREND: i32,
    WHRSIZ: i32,
    CMPCDE: StackArray<i32, 8>,
    CMPNEG: StackArray<i32, 8>,
    CMPOPS: StackArray<i32, 7>,
    LOGCDE: StackArray<i32, 3>,
    LOGOPS: StackArray<i32, 3>,
    DONOW: bool,
    FND: bool,
    QUAL: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut RELS = ActualArray2D::<i32>::new(1..=3, 1..=MAXREL);
        let mut RLPOOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=MAXREL);
        let mut CJPOOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=MAXREL);
        let mut CJPTRS = ActualArray::<i32>::new(1..=MAXREL);
        let mut DJPOOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=MAXREL);
        let mut DJPTRS = ActualArray::<i32>::new(1..=MAXREL);
        let mut MTPOOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=MAXTOK);
        let mut MTCODE = ActualArray::<i32>::new(1..=MAXTOK);
        let mut MTEXPR = ActualArray::<i32>::new(1..=MAXTOK);
        let mut MSTART = ActualArray::<i32>::new(1..=MAXSTK);
        let mut POPCND = ActualArray::<i32>::new(1..=MAXSTK);
        let mut NMETA: i32 = 0;
        let mut B: i32 = 0;
        let mut CJ = StackArray::<i32, 4>::new(1..=4);
        let mut CJNODE: i32 = 0;
        let mut COLPTR: i32 = 0;
        let mut DJ = StackArray::<i32, 2>::new(1..=2);
        let mut DJNODE: i32 = 0;
        let mut DJTAIL: i32 = 0;
        let mut DSPOOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=MAXREL);
        let mut DSCBUF = ActualArray2D::<i32>::new(1..=DSCSIZ, 1..=MAXREL);
        let mut E: i32 = 0;
        let mut ENDKW = StackArray::<i32, 3>::new(1..=NENDKW);
        let mut ENDLOC: i32 = 0;
        let mut EXPRHD: i32 = 0;
        let mut FIRST: i32 = 0;
        let mut FOURTH: i32 = 0;
        let mut HEAD1: i32 = 0;
        let mut HEAD2: i32 = 0;
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut LEVEL: i32 = 0;
        let mut LXB: i32 = 0;
        let mut LXE: i32 = 0;
        let mut METAHD: i32 = 0;
        let mut NCONJ: i32 = 0;
        let mut NEWCJ: i32 = 0;
        let mut NEWDJ: i32 = 0;
        let mut NEWREL: i32 = 0;
        let mut NEXT: i32 = 0;
        let mut NODE: i32 = 0;
        let mut NRELS: i32 = 0;
        let mut OP: i32 = 0;
        let mut PREV: i32 = 0;
        let mut REL = StackArray::<i32, 4>::new(1..=4);
        let mut RELPTR: i32 = 0;
        let mut RELSET = ActualArray::<i32>::new(LBCELL..=MAXREL);
        let mut RETCND: i32 = 0;
        let mut RHSPTR: i32 = 0;
        let mut SECOND: i32 = 0;
        let mut SKIP: i32 = 0;
        let mut SIZES = ActualArray::<i32>::new(1..=MAXCON);
        let mut START: i32 = 0;
        let mut STATE: i32 = 0;
        let mut TABPTR: i32 = 0;
        let mut TAIL: i32 = 0;
        let mut THIRD: i32 = 0;
        let mut TYPE: i32 = 0;
        let mut WHRBEG: i32 = 0;
        let mut WHREND: i32 = 0;
        let mut WHRSIZ: i32 = 0;
        let mut CMPCDE = StackArray::<i32, 8>::new(1..=NRELOP);
        let mut CMPNEG = StackArray::<i32, 8>::new(1..=NRELOP);
        let mut CMPOPS = StackArray::<i32, 7>::new(1..=(NRELOP - 1));
        let mut LOGCDE = StackArray::<i32, 3>::new(1..=NLOGOP);
        let mut LOGOPS = StackArray::<i32, 3>::new(1..=NLOGOP);
        let mut DONOW: bool = false;
        let mut FND: bool = false;
        let mut QUAL: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(KWAND),
                Val::I(AND),
                Val::I(KWOR),
                Val::I(OR),
                Val::I(KWNOT),
                Val::I(NOT),
            ]
            .into_iter();
            for I in intrinsics::range(1, NLOGOP, 1) {
                LOGOPS[I] = clist.next().unwrap().into_i32();
                LOGCDE[I] = clist.next().unwrap().into_i32();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(KWEQ),
                Val::I(EQ),
                Val::I(NE),
                Val::I(KWGE),
                Val::I(GE),
                Val::I(LT),
                Val::I(KWGT),
                Val::I(GT),
                Val::I(LE),
                Val::I(KWLE),
                Val::I(LE),
                Val::I(GT),
                Val::I(KWLT),
                Val::I(LT),
                Val::I(GE),
                Val::I(KWNE),
                Val::I(NE),
                Val::I(EQ),
                Val::I(KWLIKE),
                Val::I(LIKE),
                Val::I(UNLIKE),
            ]
            .into_iter();
            for I in intrinsics::range(1, (NRELOP - 1), 1) {
                CMPOPS[I] = clist.next().unwrap().into_i32();
                CMPCDE[I] = clist.next().unwrap().into_i32();
                CMPNEG[I] = clist.next().unwrap().into_i32();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(UNLIKE), Val::I(LIKE)].into_iter();
            CMPCDE[NRELOP] = clist.next().unwrap().into_i32();
            CMPNEG[NRELOP] = clist.next().unwrap().into_i32();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(KWFROM), Val::I(KWORDR), Val::I(KWSEL)].into_iter();
            ENDKW
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            RELS,
            RLPOOL,
            CJPOOL,
            CJPTRS,
            DJPOOL,
            DJPTRS,
            MTPOOL,
            MTCODE,
            MTEXPR,
            MSTART,
            POPCND,
            NMETA,
            B,
            CJ,
            CJNODE,
            COLPTR,
            DJ,
            DJNODE,
            DJTAIL,
            DSPOOL,
            DSCBUF,
            E,
            ENDKW,
            ENDLOC,
            EXPRHD,
            FIRST,
            FOURTH,
            HEAD1,
            HEAD2,
            I,
            J,
            K,
            LEVEL,
            LXB,
            LXE,
            METAHD,
            NCONJ,
            NEWCJ,
            NEWDJ,
            NEWREL,
            NEXT,
            NODE,
            NRELS,
            OP,
            PREV,
            REL,
            RELPTR,
            RELSET,
            RETCND,
            RHSPTR,
            SECOND,
            SKIP,
            SIZES,
            START,
            STATE,
            TABPTR,
            TAIL,
            THIRD,
            TYPE,
            WHRBEG,
            WHREND,
            WHRSIZ,
            CMPCDE,
            CMPNEG,
            CMPOPS,
            LOGCDE,
            LOGOPS,
            DONOW,
            FND,
            QUAL,
        }
    }
}

//$Procedure      ZZEKNRML ( EK, normalize WHERE clause )
pub fn ZZEKNRML(
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
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let LXBEGS = DummyArray::new(LXBEGS, 1..);
    let LXENDS = DummyArray::new(LXENDS, 1..);
    let TOKENS = DummyArray::new(TOKENS, 1..);
    let VALUES = DummyArray::new(VALUES, 1..);
    let NUMVLS = DummyArray::new(NUMVLS, 1..);
    let CHBEGS = DummyArray::new(CHBEGS, 1..);
    let CHENDS = DummyArray::new(CHENDS, 1..);
    let mut EQRYI = DummyArrayMut::new(EQRYI, LBCELL..);
    let mut EQRYD = DummyArrayMut::new(EQRYD, 1..);

    //
    //
    // SPICELIB functions
    //

    //
    //
    // Local parameters
    //

    //
    // Data structure bounds:
    //

    //
    // MAXREL is the maximum number of relations that can be handled
    // by this routine.
    //

    //
    // MAXMET is the maximum number of meta-tokens making up any
    // expression.
    //

    //
    // LBPOOL is the lower bound of the second index of a linked list
    // pool array.
    //

    //
    //
    // Stack parameters:
    //
    //

    //
    // Operator parameters:
    //
    //
    // NLOGOP is the number of recognized logical operators.  These
    // are AND, OR, and NOT.
    //

    //
    // NRELOP is the number of arithmetic and character comparison
    // operators.
    //

    //
    //
    // Meta-token codes, excluding codes for relational operators:
    //

    //
    // Number of keywords that can terminate a WHERE clause.
    //

    //
    //
    // State parameters:
    //

    //
    //
    // 'Pop condition' codes:
    //

    //
    // Token descriptor size:
    //

    //
    // Local variables
    //

    //
    // Each comparison relation is expressed by three tokens, so the
    // comparison relations are represented by a 3 x MAXREL array.  The
    // first and third elements of each row of RELS are array indices
    // that point into the input array TOKENS; the middle element
    // of each row is an operator code.  The set of triples representing
    // comparison relations is indexed by a doubly linked list pool.
    // Each conjunction of comparison relations is represented by a
    // linked list of pointers to entries in the RELS array.  These
    // pointers are contained in the CJPTRS array.  The pointers are
    // linked via entries in the double linked list pool CJPOOL.
    //

    //
    // Each normalized expression is a disjunction of conjunctions.  Each
    // such disjunction is represented by a linked list of nodes
    // associated with pointers to entries in the CJPOOL array.  DJPTRS
    // is the parallel array used to associate each node of a disjunction
    // with the head node of a conjunction list in CJPOOL.
    //

    //
    // Meta-tokens are groups of tokens that comprise syntactic units
    // in a query.  Each symbol that appears on the left hand side of
    // a production rule in the grammar corresponds to a type of
    // meta-token.
    //
    // Throughout the parsing process, the meta-tokens representing the
    // query are organized as a linked list.  Each meta-token is also
    // associated with a more detailed classification MTCODE.
    //
    // For each meta-token that represents an identifier, a value,
    // a name, or an expression, there is a corresponding element of
    // MTEXPR.  This element contains a pointer to a token or to a
    // normalized expression.  In the latter case, the pointer is the
    // head node of a list in the disjunction table.
    //

    //
    // Stack variables
    //
    // These variables have the following meanings:
    //
    //    MSTART is the node number of the first meta-token of
    //    the current expression being parsed.
    //
    //    NMETA is the number of meta-tokens in the query.
    //
    //    POPCND is the `pop condition'.  This is a code indicating
    //    what event must occur to trigger popping the current state.
    //    The two events that can cause the state to be popped are
    //    the execution of a reduction and encountering a right grouper.
    //

    //
    //
    // Other local variables
    //

    //
    // Saved variables
    //

    //
    //
    // Initial values
    //

    //
    // Note:  there is no "UNLIKE" keyword, but there is an UNLIKE
    // operator, which is the complement of the LIKE operator.
    //

    //
    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZEKNRML", ctx)?;
    }

    //
    // No error at this point.
    //
    *ERROR = false;
    fstr::assign(PRSERR, b" ");

    if (NTOKEN > MAXTOK) {
        *ERROR = true;
        fstr::assign(PRSERR, b"Too many tokens in query; max allowed is #.");
        REPMI(&PRSERR.to_vec(), b"#", MAXTOK, PRSERR, ctx);
        CHKOUT(b"ZZEKNRML", ctx)?;
        return Ok(());
    }

    //
    // Find out the start and end indices of the tokens comprising the
    // WHERE clause.  If there are no tokens in the WHERE clause, we may
    // as well go home.
    //
    ZZEKTLOC(
        TKKEY,
        KWWHER,
        NTOKEN,
        TOKENS.as_slice(),
        VALUES.as_slice(),
        &mut save.WHRBEG,
        &mut save.FND,
    );

    save.WHRBEG = (save.WHRBEG + 1);

    if !save.FND {
        *ERROR = true;
        fstr::assign(PRSERR, b"Missing WHERE keyword.");
        CHKOUT(b"ZZEKNRML", ctx)?;
        return Ok(());
    }

    //
    // The WHERE clause is terminated by the end of the query or by
    // the first keyword of the set {SELECT, FROM, ORDER} that follows
    // the WHERE keyword.
    //
    save.WHREND = NTOKEN;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            ZZEKTLOC(
                TKKEY,
                save.ENDKW[save.I],
                NTOKEN,
                TOKENS.as_slice(),
                VALUES.as_slice(),
                &mut save.ENDLOC,
                &mut save.FND,
            );

            if save.FND {
                if ((save.ENDLOC < save.WHREND) && (save.ENDLOC > save.WHRBEG)) {
                    save.WHREND = (save.ENDLOC - 1);
                }
            }

            save.I += m3__;
        }
    }

    save.WHRSIZ = ((save.WHREND - save.WHRBEG) + 1);

    if (save.WHRSIZ == 0) {
        *ERROR = true;
        fstr::assign(PRSERR, b"Empty WHERE clause.");
        CHKOUT(b"ZZEKNRML", ctx)?;
        return Ok(());
    }

    //
    // Initialize the pools.
    //
    LNKINI(MAXREL, save.RLPOOL.as_slice_mut(), ctx)?;
    LNKINI(MAXREL, save.CJPOOL.as_slice_mut(), ctx)?;
    LNKINI(MAXREL, save.DJPOOL.as_slice_mut(), ctx)?;
    LNKINI(MAXTOK, save.MTPOOL.as_slice_mut(), ctx)?;
    LNKINI(MAXREL, save.DSPOOL.as_slice_mut(), ctx)?;

    //
    // Loop through our token list and classify the tokens.  Initialize
    // the meta-token list.
    //
    save.NMETA = 0;
    save.TAIL = NIL;

    save.I = save.WHRBEG;

    while (save.I <= save.WHREND) {
        //
        // Allocate a node and link it in at the tail of the meta-token
        // list.
        //
        LNKAN(save.MTPOOL.as_slice_mut(), &mut save.NODE, ctx)?;
        LNKILA(save.TAIL, save.NODE, save.MTPOOL.as_slice_mut(), ctx)?;
        save.TAIL = save.NODE;

        //
        // Each meta-token's expression pointer points to its original
        // token index, by default.
        //
        save.MTEXPR[save.NODE] = save.I;

        if (TOKENS[save.I] == TKLPAR) {
            save.MTCODE[save.NODE] = LGROUP;
        } else if (TOKENS[save.I] == TKRPAR) {
            save.MTCODE[save.NODE] = RGROUP;
        } else if ((TOKENS[save.I] == TKINT) || (TOKENS[save.I] == TKDP)) {
            //
            // Numeric values must be added to the encoded query.  We
            // allocate a descriptor from the descriptor pool for
            // each identifier.  The expression pointer for the
            // identifier points to the descriptor.  Note:  the
            // allocation should be safe, since we've checked the total
            // number of tokens in the query.
            //
            save.MTCODE[save.NODE] = VALUE;

            LNKAN(save.DSPOOL.as_slice_mut(), &mut save.MTEXPR[save.NODE], ctx)?;

            if (TOKENS[save.I] == TKINT) {
                save.TYPE = INT;
            } else {
                save.TYPE = DP;
            }

            ZZEKINQN(
                NUMVLS[VALUES[save.I]],
                save.TYPE,
                LXBEGS[save.I],
                LXENDS[save.I],
                EQRYI.as_slice_mut(),
                EQRYD.as_slice_mut(),
                save.DSCBUF.subarray_mut([1, save.MTEXPR[save.NODE]]),
                ctx,
            )?;

            //
            // Set the descriptor to indicate that it represents a value.
            //
            save.DSCBUF[[DSCSIZ, save.MTEXPR[save.NODE]]] = VALUE;
        } else if (TOKENS[save.I] == TKQSTR) {
            //
            // The treatment of strings is analogous to that of numbers.
            //
            save.MTCODE[save.NODE] = VALUE;

            LNKAN(save.DSPOOL.as_slice_mut(), &mut save.MTEXPR[save.NODE], ctx)?;

            save.B = CHBEGS[VALUES[save.I]];
            save.E = CHENDS[VALUES[save.I]];

            ZZEKINQC(
                fstr::substr(CHRBUF, save.B..=save.E),
                ((save.E - save.B) + 1),
                LXBEGS[save.I],
                LXENDS[save.I],
                EQRYI.as_slice_mut(),
                EQRYC,
                save.DSCBUF.subarray_mut([1, save.MTEXPR[save.NODE]]),
                ctx,
            )?;

            //
            // Set the descriptor to indicate that it represents a value.
            //
            save.DSCBUF[[DSCSIZ, save.MTEXPR[save.NODE]]] = VALUE;
        } else if (TOKENS[save.I] == TKID) {
            //
            // Identifiers must be added to the encoded query.  We
            // allocate a descriptor from the descriptor pool for
            // each identifier.  The expression pointer for the
            // identifier points to the descriptor.
            //
            save.MTCODE[save.NODE] = IDENT;

            LNKAN(save.DSPOOL.as_slice_mut(), &mut save.MTEXPR[save.NODE], ctx)?;

            save.B = CHBEGS[VALUES[save.I]];
            save.E = CHENDS[VALUES[save.I]];

            ZZEKINQC(
                fstr::substr(CHRBUF, save.B..=save.E),
                ((save.E - save.B) + 1),
                LXBEGS[save.I],
                LXENDS[save.I],
                EQRYI.as_slice_mut(),
                EQRYC,
                save.DSCBUF.subarray_mut([1, save.MTEXPR[save.NODE]]),
                ctx,
            )?;

            //
            // Set the descriptor to indicate that it represents an
            // identifier.
            //
            save.DSCBUF[[DSCSIZ, save.MTEXPR[save.NODE]]] = IDENT;
        } else if (TOKENS[save.I] == TKDOT) {
            save.MTCODE[save.NODE] = PERIOD;
        } else if (TOKENS[save.I] == TKKEY) {
            //
            // We have a keyword.  Identify it and locate the corresponding
            // code.
            //
            save.J = ISRCHI(VALUES[save.I], NLOGOP, save.LOGOPS.as_slice());
            save.K = ISRCHI(VALUES[save.I], (NRELOP - 1), save.CMPOPS.as_slice());

            if (save.J > 0) {
                //
                // We have a logical operator, unless we have the NOT LIKE
                //  or NOT BETWEEN sequence.
                //
                if (save.LOGCDE[save.J] != NOT) {
                    save.MTCODE[save.NODE] = save.LOGCDE[save.J];
                } else {
                    if (save.I <= save.WHREND) {
                        if ((TOKENS[(save.I + 1)] == TKKEY) && (VALUES[(save.I + 1)] == KWLIKE)) {
                            //
                            // Replace the NOT LIKE sequence with the
                            // UNLIKE operator.  Skip over the LIKE token.
                            //
                            save.MTCODE[save.NODE] = UNLIKE;
                            save.I = (save.I + 1);
                        } else if ((TOKENS[(save.I + 1)] == TKKEY)
                            && (VALUES[(save.I + 1)] == KWBETW))
                        {
                            //
                            // Replace the NOT BETWEEN sequence with the
                            // NOTBTW operator.  Skip over the BETWEEN token.
                            //
                            save.MTCODE[save.NODE] = NOTBTW;
                            save.I = (save.I + 1);
                        } else {
                            save.MTCODE[save.NODE] = NOT;
                        }
                    } else {
                        save.MTCODE[save.NODE] = NOT;
                    }
                }
            } else if (save.K > 0) {
                save.MTCODE[save.NODE] = save.CMPCDE[save.K];
            } else if (VALUES[save.I] == KWBETW) {
                save.MTCODE[save.NODE] = BTWEEN;
            } else if (VALUES[save.I] == KWIS) {
                //
                // The token IS translates to EQ; the token sequence
                // IS NOT translates to NE.
                //
                if (save.I < save.WHREND) {
                    if ((TOKENS[(save.I + 1)] == TKKEY) && (VALUES[(save.I + 1)] == KWNOT)) {
                        //
                        // We have an IS NOT sequence.  Skip over the NOT
                        // token; indicate the sequence with a single NE
                        // meta-token.
                        //
                        save.MTCODE[save.NODE] = NE;
                        save.I = (save.I + 1);
                    } else {
                        save.MTCODE[save.NODE] = EQ;
                    }
                } else {
                    save.MTCODE[save.NODE] = EQ;
                }
            } else if (VALUES[save.I] == KWNULL) {
                //
                // The expression pointer for null values is NIL.
                //
                save.MTCODE[save.NODE] = VALUE;
                save.MTEXPR[save.NODE] = NIL;
            } else {
                //
                // Sorry, that was the last chance for valid keywords.
                //
                save.LXB = LXBEGS[save.I];
                save.LXE = LXENDS[save.I];
                *ERROR = true;
                fstr::assign(PRSERR, b"Unexpected keyword # found at location #.");
                REPMC(
                    &PRSERR.to_vec(),
                    b"#",
                    fstr::substr(QUERY, save.LXB..=save.LXE),
                    PRSERR,
                );
                REPMI(&PRSERR.to_vec(), b"#", save.LXB, PRSERR, ctx);
                CHKOUT(b"ZZEKNRML", ctx)?;
                return Ok(());
            }
        } else {
            //
            // Sorry, that was the last chance, period.
            //
            save.LXB = LXBEGS[save.I];
            save.LXE = LXENDS[save.I];
            *ERROR = true;
            fstr::assign(PRSERR, b"Unexpected token # found at location #.");
            REPMC(
                &PRSERR.to_vec(),
                b"#",
                fstr::substr(QUERY, save.LXB..=save.LXE),
                PRSERR,
            );
            REPMI(&PRSERR.to_vec(), b"#", save.LXB, PRSERR, ctx);
            CHKOUT(b"ZZEKNRML", ctx)?;
            return Ok(());
        }
        //
        // At this point, we've classified the Ith token.  MTCODE(NODE)
        // is the meta-token code for this token.
        //
        save.I = (save.I + 1);
        save.NMETA = (save.NMETA + 1);
    }

    //
    // Initialize the head of the meta-token list.
    //
    save.METAHD = LNKHL(save.TAIL, save.MTPOOL.as_slice(), ctx)?;

    //
    // Filter out extraneous parentheses around column names or
    // values.
    //
    save.NODE = save.METAHD;

    while (save.NODE > 0) {
        if ((save.MTCODE[save.NODE] == NAME) || (save.MTCODE[save.NODE] == VALUE)) {
            //
            // If the current metatoken is bracketed by parentheses,
            // remove them and update the metatoken count accordingly.
            //
            save.PREV = LNKPRV(save.NODE, save.MTPOOL.as_slice(), ctx)?;
            save.NEXT = LNKNXT(save.NODE, save.MTPOOL.as_slice(), ctx)?;

            if ((save.PREV > 0) && (save.NEXT > 0)) {
                if ((save.MTCODE[save.PREV] == LGROUP) && (save.MTCODE[save.NEXT] == RGROUP)) {
                    LNKFSL(save.PREV, save.PREV, save.MTPOOL.as_slice_mut(), ctx)?;
                    LNKFSL(save.NEXT, save.NEXT, save.MTPOOL.as_slice_mut(), ctx)?;
                    save.METAHD = LNKHL(save.NODE, save.MTPOOL.as_slice(), ctx)?;
                    save.NMETA = (save.NMETA - 2);
                //
                // We don't advance the current token in this case
                // because there may be more parentheses to remove.
                //
                } else {
                    //
                    // This token is not bracketed by parentheses; look at
                    // the next metatoken.
                    //
                    save.NODE = save.NEXT;
                }
            } else {
                //
                // This token is not bracketed by tokens on both sides; look
                // at the next metatoken.  It's ok for the next token to be
                // NIL.
                //
                save.NODE = save.NEXT;
            }
        } else {
            //
            // The current token is not a name or value; look at the next
            // token.
            //
            save.NODE = LNKNXT(save.NODE, save.MTPOOL.as_slice(), ctx)?;
        }
    }

    //
    //
    // Now it's time to parse our expression.  We will validate the
    // expression by using our grammar rules to condense groups of
    // meta-tokens that correspond to the right-hand sides of grammatical
    // rules into meta-tokens that correspond to the left-hand sides
    // of those same rules.  Each such application of a grammar rule
    // is called a `reduction.'  When we're left with a single
    // meta-token of type <relational expression>, we're done.
    //
    // If, before reaching the desired final state, we get to a point
    // where no reductions can be performed, we have a syntax error.
    //
    // As parsing advances, we'll start to get meta-tokens that are
    // logical expressions.  Each logical expression will be represented
    // by a data structure that organizes the expression in a way that
    // we'll refer to as `normalized':  the expression will be
    // represented as a disjunction of conjunctions, for example
    //
    //    ( A AND B AND C ) OR ( D AND E ) OR ( F ) OR ( G AND H AND I )
    //
    // Each metatoken that represents a logical expression will
    // refer to it through a pointer which is a member of the MTEXPR
    // array.
    //

    if (save.WHRSIZ < 3) {
        *ERROR = true;
        fstr::assign(PRSERR, b"Too few tokens in WHERE clause.");
        CHKOUT(b"ZZEKNRML", ctx)?;
        return Ok(());
    } else {
        save.LEVEL = 1;
        save.MSTART[save.LEVEL] = save.METAHD;
        save.POPCND[save.LEVEL] = NONE;
        save.STATE = PARSE;
    }

    while (save.STATE != TERM) {
        if (save.STATE == PARSE) {
            //
            // If the input query is valid, we're looking at the leftmost
            // meta-token of an expression that matches the right-hand
            // side of one of the grammar rules.  Referring back to the
            // rules, we see that there are only a few meta-tokens that are
            // valid as the first token of such an expression:
            //
            //    - A left grouper
            //    - An identifier
            //    - A name
            //    - An expression
            //    - A unary operator (`NOT' )
            //
            // We'll see if we can perform a reduction.  The reductions
            // that are possible depend on how many meta-tokens are
            // present in the expression we're looking at.
            //
            // FIRST is the node number of the first meta-token to look
            // at, in an attempt to perform a reduction.  SECOND, THIRD,
            // and FOURTH have the obvious meanings; some of these may
            // be 0.
            //
            save.FIRST = save.MSTART[save.LEVEL];

            if (save.FIRST > 0) {
                save.SECOND = LNKNXT(save.FIRST, save.MTPOOL.as_slice(), ctx)?;
            } else {
                save.SECOND = NIL;
            }

            if (save.SECOND > 0) {
                save.THIRD = LNKNXT(save.SECOND, save.MTPOOL.as_slice(), ctx)?;
            } else {
                save.THIRD = NIL;
            }

            if (save.THIRD > 0) {
                save.FOURTH = LNKNXT(save.THIRD, save.MTPOOL.as_slice(), ctx)?;
            } else {
                save.FOURTH = NIL;
            }

            if (save.FIRST <= 0) {
                //
                // This never happens to good commands.
                //
                *ERROR = true;
                fstr::assign(PRSERR, b"WHERE clause ran out of tokens unexpectedly.  This may be due to an extra left parenthesis.");
                CHKOUT(b"ZZEKNRML", ctx)?;
                return Ok(());
            }

            //
            // We have at least one meta-token to work with.  We'll
            // take different actions depending on its type.
            //

            if (save.MTCODE[save.FIRST] == IDENT) {
                //
                // This is a simple case to deal with:  in valid queries,
                // we have either the sequence
                //
                //    <identifier> . <identifier>
                //
                // or
                //
                //    <identifier>
                //
                // Both of these token sequences represent a column name;
                // in the former case, the name is qualified by a table
                // name, in the latter, the column name is unqualified.
                // If the table name is absent, we'll simply save a null
                // descriptor for it.  The descriptors will be linked, with
                // the table descriptor coming first, and the NAME token
                // resulting from reducing this token sequence will point to
                // the list of descriptors via the MTEXPR pointer.
                //
                //
                if (save.THIRD > 0) {
                    //
                    // We can look at the following two tokens.
                    //
                    if ((save.MTCODE[save.SECOND] == PERIOD) && (save.MTCODE[save.THIRD] == IDENT))
                    {
                        save.QUAL = true;
                    } else {
                        save.QUAL = false;
                    }
                } else {
                    //
                    // There aren't enough tokens for this name to be
                    // qualified.
                    //
                    save.QUAL = false;
                }

                if save.QUAL {
                    //
                    // We have a fully qualified column name.  Hook up the
                    // table and column name descriptors.
                    //
                    save.TABPTR = save.MTEXPR[save.FIRST];
                    save.COLPTR = save.MTEXPR[save.THIRD];

                    LNKILA(save.TABPTR, save.COLPTR, save.DSPOOL.as_slice_mut(), ctx)?;

                    //
                    // Reduce the expression to a <name> metatoken.
                    //
                    save.MTCODE[save.FIRST] = NAME;

                    LNKFSL(save.SECOND, save.THIRD, save.MTPOOL.as_slice_mut(), ctx)?;

                    save.NMETA = (save.NMETA - 2);
                } else {
                    //
                    // We have an unqualified column name.  Allocate a table
                    // descriptor.  Set the table descriptor to indicate a
                    // null character descriptor.  Link this descriptor in
                    // before the column descriptor.
                    //
                    LNKAN(save.DSPOOL.as_slice_mut(), &mut save.TABPTR, ctx)?;

                    CLEARI(DSCSIZ, save.DSCBUF.subarray_mut([1, save.TABPTR]));
                    save.DSCBUF[[EQCTYP, save.TABPTR]] = CHR;
                    save.DSCBUF[[DSCSIZ, save.TABPTR]] = IDENT;

                    save.COLPTR = save.MTEXPR[save.FIRST];

                    LNKILA(save.TABPTR, save.COLPTR, save.DSPOOL.as_slice_mut(), ctx)?;

                    //
                    // Reduce the expression to a <name> metatoken.
                    // The reduction doesn't change the number of metatokens.
                    //
                    save.MTEXPR[save.FIRST] = save.TABPTR;
                    save.MTCODE[save.FIRST] = NAME;
                }

                //
                // Decide the next state.
                //
                save.STATE = REDUCD;
            } else if (save.MTCODE[save.FIRST] == VALUE) {
                //
                // If the query is valid, the sequence of meta-tokens
                // should be one of
                //
                //    <value>  AND  <name>
                //    <value>  AND  <value>
                //
                // Both of these reduce to the symbol <BETWEEN expr>.
                //
                //
                if (save.THIRD <= 0) {
                    *ERROR = true;
                    fstr::assign(PRSERR, b"Tokens were missing from comparison relation.");
                    CHKOUT(b"ZZEKNRML", ctx)?;
                    return Ok(());
                }

                //
                // Null values are not allowed in BETWEEN expressions.
                //
                if ((save.MTEXPR[save.FIRST] == NIL) || (save.MTEXPR[save.THIRD] == NIL)) {
                    *ERROR = true;
                    fstr::assign(
                        PRSERR,
                        b"NULL values are not allowed in BETWEEN or NOT BETWEEN clauses.",
                    );
                    CHKOUT(b"ZZEKNRML", ctx)?;
                    return Ok(());
                }

                if (save.MTCODE[save.THIRD] == IDENT) {
                    //
                    // We'll need to reduce the IDENT before proceeding.
                    //
                    save.START = save.THIRD;
                    save.RETCND = REDUCE;
                    save.STATE = PUSH;
                } else if ((save.MTCODE[save.SECOND] == AND)
                    && ((save.MTCODE[save.THIRD] == NAME) || (save.MTCODE[save.THIRD] == VALUE)))
                {
                    //
                    // This sequence of tokens, when seen in the PARSE
                    // state, is a set of value bounds for a BETWEEN or
                    // NOT BETWEEN expression.  Note that this token sequence
                    // can occur elsewhere, but not in the PARSE state.
                    // This is because the meta-token sequences
                    //
                    //    <value>  AND  <name>
                    //    <value>  AND  <value>
                    //
                    // occur at the start of the RHS of only two
                    // productions, namely
                    //
                    //    <BETWEEN expr>  =>  <value>  AND  <name>
                    //    <BETWEEN expr>  =>  <value>  AND  <value>
                    //
                    //
                    // Hook up the name or value descriptors.
                    //
                    LNKILB(
                        save.MTEXPR[save.FIRST],
                        save.MTEXPR[save.THIRD],
                        save.DSPOOL.as_slice_mut(),
                        ctx,
                    )?;

                    save.MTCODE[save.FIRST] = BTWEXP;

                    LNKFSL(save.SECOND, save.THIRD, save.MTPOOL.as_slice_mut(), ctx)?;

                    save.NMETA = (save.NMETA - 2);

                    //
                    // Decide the next state.
                    //
                    save.STATE = REDUCD;
                } else if (save.MTCODE[save.SECOND] > 0) {
                    //
                    // The third meta-token is in the wrong place at the
                    // wrong time.
                    //
                    *ERROR = true;
                    fstr::assign(PRSERR, b"Token sequence must be followed by a value.");

                    CHKOUT(b"ZZEKNRML", ctx)?;
                    return Ok(());
                } else {
                    //
                    // The second meta-token is supposed to be the AND token,
                    // but it's actually something else.
                    //
                    *ERROR = true;
                    fstr::assign(PRSERR, b"Token must be followed by the AND operator.");

                    CHKOUT(b"ZZEKNRML", ctx)?;
                    return Ok(());
                }
            } else if (save.MTCODE[save.FIRST] == NAME) {
                //
                // If the query is valid, the sequence of meta-tokens
                // should be any of
                //
                //    <name>  <comparison operator> <value>
                //    <name>  <comparison operator> <name>
                //    <name>  <comparison operator> <ident>
                //
                // or
                //
                //    <name>  AND  <name>
                //    <name>  AND  <value>
                //    <name>  AND  <ident>
                //
                // or
                //
                //    <name>  BETWEEN  <BETWEEN expr>
                //    <name>  BETWEEN  <name>
                //    <name>  BETWEEN  <value>
                //    <name>  BETWEEN  <ident>
                //
                // or
                //
                //    <name>  <NOT BETWEEN>  <BETWEEN expr>
                //    <name>  <NOT BETWEEN>  <name>
                //    <name>  <NOT BETWEEN>  <value>
                //    <name>  <NOT BETWEEN>  <ident>
                //
                // There must be at least three meta-tokens here.
                //
                //
                if (save.THIRD <= 0) {
                    *ERROR = true;
                    fstr::assign(PRSERR, b"Tokens were missing from comparison relation.");
                    CHKOUT(b"ZZEKNRML", ctx)?;
                    return Ok(());
                }

                if (save.MTCODE[save.THIRD] == IDENT) {
                    //
                    // We'll need to reduce the IDENT before proceeding.
                    //
                    save.START = save.THIRD;
                    save.RETCND = REDUCE;
                    save.STATE = PUSH;
                } else if ((save.MTCODE[save.SECOND] == AND)
                    && ((save.MTCODE[save.THIRD] == NAME) || (save.MTCODE[save.THIRD] == VALUE)))
                {
                    //
                    // This sequence of tokens, when seen in the PARSE
                    // state, is a set of value bounds for a BETWEEN or
                    // NOT BETWEEN expression.  Note that this token sequence
                    // can occur elsewhere, but not in the PARSE state.
                    // This is because the meta-token sequences
                    //
                    //    <name>  AND  <name>
                    //    <name>  AND  <value>
                    //
                    // occur at the start of the RHS of only two
                    // productions, namely
                    //
                    //    <BETWEEN expr>  =>  <name>   AND  <name>
                    //    <BETWEEN expr>  =>  <name>   AND  <value>
                    //
                    //
                    // Null values are not allowed in BETWEEN expressions.
                    //
                    if ((save.MTEXPR[save.FIRST] == NIL) || (save.MTEXPR[save.THIRD] == NIL)) {
                        *ERROR = true;
                        fstr::assign(
                            PRSERR,
                            b"NULL values are not allowed in BETWEEN or NOT BETWEEN clauses.",
                        );
                        CHKOUT(b"ZZEKNRML", ctx)?;
                        return Ok(());
                    }

                    //
                    // Hook up the name or value descriptors.
                    //
                    LNKILB(
                        save.MTEXPR[save.FIRST],
                        save.MTEXPR[save.THIRD],
                        save.DSPOOL.as_slice_mut(),
                        ctx,
                    )?;

                    save.MTCODE[save.FIRST] = BTWEXP;

                    LNKFSL(save.SECOND, save.THIRD, save.MTPOOL.as_slice_mut(), ctx)?;

                    save.NMETA = (save.NMETA - 2);

                    //
                    // Decide the next state.
                    //
                    save.STATE = REDUCD;
                } else if ((save.MTCODE[save.SECOND] > 0)
                    && ((save.MTCODE[save.THIRD] == NAME) || (save.MTCODE[save.THIRD] == VALUE)))
                {
                    //
                    // Positive meta-token codes denote comparison
                    // operators.
                    //
                    // We have an arithmetic, string, or column comparison
                    // expression.  This is a trivial normalized
                    // relational expression.  All we have to do
                    // is store the expression in the relation table,
                    // and free the second and third meta-tokens.
                    //
                    if (LNKNFN(save.RLPOOL.as_slice()) < 1) {
                        *ERROR = true;
                        fstr::assign(PRSERR, b"Relation table is full.");
                        CHKOUT(b"ZZEKNRML", ctx)?;
                        return Ok(());
                    }

                    LNKAN(save.RLPOOL.as_slice_mut(), &mut save.NEWREL, ctx)?;

                    save.RELS[[1, save.NEWREL]] = save.MTEXPR[save.FIRST];
                    save.RELS[[2, save.NEWREL]] = save.MTCODE[save.SECOND];
                    save.RELS[[3, save.NEWREL]] = save.MTEXPR[save.THIRD];

                    LNKFSL(save.SECOND, save.THIRD, save.MTPOOL.as_slice_mut(), ctx)?;
                    save.NMETA = (save.NMETA - 2);

                    //
                    // Now allocate an entry in the conjunction pool
                    // and make this entry point to the relation table
                    // entry.
                    //
                    if (LNKNFN(save.CJPOOL.as_slice()) < 1) {
                        *ERROR = true;
                        fstr::assign(PRSERR, b"Conjunction table is full.");
                        CHKOUT(b"ZZEKNRML", ctx)?;
                        return Ok(());
                    }

                    LNKAN(save.CJPOOL.as_slice_mut(), &mut save.NEWCJ, ctx)?;

                    save.CJPTRS[save.NEWCJ] = save.NEWREL;

                    //
                    // Now allocate an entry in the disjunction pool
                    // and make this entry point to the conjunction pool
                    // entry.
                    //
                    if (LNKNFN(save.DJPOOL.as_slice()) < 1) {
                        *ERROR = true;
                        fstr::assign(PRSERR, b"Disjunction table is full.");
                        CHKOUT(b"ZZEKNRML", ctx)?;
                        return Ok(());
                    }

                    LNKAN(save.DJPOOL.as_slice_mut(), &mut save.NEWDJ, ctx)?;

                    save.DJPTRS[save.NEWDJ] = save.NEWCJ;

                    //
                    // Change the type of the first meta-token to EXPR and
                    // have that meta-token point to this table entry.  Bag
                    // the other two meta-tokens.
                    //
                    save.MTCODE[save.FIRST] = EXPR;
                    save.MTEXPR[save.FIRST] = save.NEWDJ;

                    //
                    // Decide the next state.
                    //
                    save.STATE = REDUCD;
                } else if ((save.MTCODE[save.SECOND] == BTWEEN)
                    || (save.MTCODE[save.SECOND] == NOTBTW))
                {
                    //
                    // If the command is syntactically correct, the
                    // meta-token sequence should be one of:
                    //
                    //    <name>  <BETWEEN>      <BETWEEN expr>
                    //    <name>  <BETWEEN>      <value>
                    //    <name>  <BETWEEN>      <name>
                    //    <name>  <NOT BETWEEN>  <BETWEEN expr>
                    //    <name>  <NOT BETWEEN>  <value>
                    //    <name>  <NOT BETWEEN>  <name>
                    //
                    //
                    if ((save.MTCODE[save.SECOND] == BTWEEN) && (save.MTCODE[save.THIRD] == BTWEXP))
                    {
                        //
                        // It's a BETWEEN comparison.  We treat this as a
                        // disjunction of conjunctions of comparison
                        // relations:
                        //                   <name>   >=   <item1>
                        //           AND     <name>   <=   <item2>
                        //
                        //      OR
                        //                   <name>   <=   <item1>
                        //           AND     <name>   >=   <item2>
                        //
                        // where item1 and item2 are specified by the
                        // descriptors belonging to the third meta-token.
                        //
                        {
                            let m1__: i32 = 1;
                            let m2__: i32 = 4;
                            let m3__: i32 = 1;
                            save.I = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                if ((save.I == 1) || (save.I == 3)) {
                                    save.K = save.MTEXPR[save.THIRD];
                                } else {
                                    //
                                    // We need the descriptor pointer for the RHS
                                    // item.  This descriptor is linked to the tail
                                    // of the descriptor for the LHS item.  The
                                    // number of nodes to skip over depends on
                                    // whether the LHS item is a name or value.
                                    //
                                    save.K = save.MTEXPR[save.THIRD];

                                    if (save.DSCBUF[[DSCSIZ, save.K]] == IDENT) {
                                        save.SKIP = 1;
                                    } else {
                                        save.SKIP = 0;
                                    }

                                    {
                                        let m1__: i32 = 1;
                                        let m2__: i32 = (save.SKIP + 1);
                                        let m3__: i32 = 1;
                                        save.J = m1__;
                                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                            save.K = LNKNXT(save.K, save.DSPOOL.as_slice(), ctx)?;
                                            save.J += m3__;
                                        }
                                    }
                                }

                                if (LNKNFN(save.RLPOOL.as_slice()) < 1) {
                                    *ERROR = true;
                                    fstr::assign(PRSERR, b"Relation table is full.");
                                    CHKOUT(b"ZZEKNRML", ctx)?;
                                    return Ok(());
                                }

                                LNKAN(save.RLPOOL.as_slice_mut(), &mut save.REL[save.I], ctx)?;

                                save.RELS[[1, save.REL[save.I]]] = save.MTEXPR[save.FIRST];

                                if ((save.I == 1) || (save.I == 4)) {
                                    save.RELS[[2, save.REL[save.I]]] = GE;
                                } else {
                                    save.RELS[[2, save.REL[save.I]]] = LE;
                                }

                                save.RELS[[3, save.REL[save.I]]] = save.K;

                                if (LNKNFN(save.CJPOOL.as_slice()) < 1) {
                                    *ERROR = true;
                                    fstr::assign(PRSERR, b"Conjunction table is full.");
                                    CHKOUT(b"ZZEKNRML", ctx)?;
                                    return Ok(());
                                }

                                LNKAN(save.CJPOOL.as_slice_mut(), &mut save.CJ[save.I], ctx)?;

                                save.CJPTRS[save.CJ[save.I]] = save.REL[save.I];

                                save.I += m3__;
                            }
                        }

                        //
                        // Link the conjunction nodes to form the two
                        // conjunctions shown above.
                        //
                        LNKILA(save.CJ[1], save.CJ[2], save.CJPOOL.as_slice_mut(), ctx)?;
                        LNKILA(save.CJ[3], save.CJ[4], save.CJPOOL.as_slice_mut(), ctx)?;

                        //
                        // Allocate disjunction pool entries and make them
                        // point to the two respective conjunctions.
                        //
                        {
                            let m1__: i32 = 1;
                            let m2__: i32 = 2;
                            let m3__: i32 = 1;
                            save.I = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                if (LNKNFN(save.DJPOOL.as_slice()) < 1) {
                                    *ERROR = true;
                                    fstr::assign(PRSERR, b"Disjunction table is full.");
                                    CHKOUT(b"ZZEKNRML", ctx)?;
                                    return Ok(());
                                }

                                LNKAN(save.DJPOOL.as_slice_mut(), &mut save.DJ[save.I], ctx)?;
                                save.DJPTRS[save.DJ[save.I]] = save.CJ[((2 * save.I) - 1)];

                                save.I += m3__;
                            }
                        }

                        //
                        // Finally, link the disjunction pool entries, and
                        // create an <expression> meta-token.  Free the unused
                        // meta-tokens.
                        //
                        LNKILA(save.DJ[1], save.DJ[2], save.DJPOOL.as_slice_mut(), ctx)?;

                        save.MTCODE[save.FIRST] = EXPR;
                        save.MTEXPR[save.FIRST] = save.DJ[1];

                        LNKFSL(save.SECOND, save.THIRD, save.MTPOOL.as_slice_mut(), ctx)?;
                        save.NMETA = (save.NMETA - 2);

                        //
                        // Decide the next state.
                        //
                        save.STATE = REDUCD;
                    } else if ((save.MTCODE[save.SECOND] == NOTBTW)
                        && (save.MTCODE[save.THIRD] == BTWEXP))
                    {
                        //
                        // It's a NOT BETWEEN comparison.  We treat
                        // this as a disjunction of conjunctions of comparison
                        // relations:
                        //
                        //                   <name>   <   <item1>
                        //           AND     <name>   <   <item2>
                        //
                        //      OR
                        //                   <name>   >   <item1>
                        //           AND     <name>   >   <item2>
                        //
                        // where item1 and item2 are specified by the
                        // descriptors belonging to the third meta-token.
                        //
                        // The actions here are closely analogous to those
                        // for the BETWEEN case.
                        //
                        {
                            let m1__: i32 = 1;
                            let m2__: i32 = 4;
                            let m3__: i32 = 1;
                            save.I = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                if ((save.I == 1) || (save.I == 3)) {
                                    save.K = save.MTEXPR[save.THIRD];
                                } else {
                                    //
                                    // We need the descriptor pointer for the RHS
                                    // item.  This descriptor is linked to the tail
                                    // of the descriptor for the LHS item.  The
                                    // number of nodes to skip over depends on
                                    // whether the LHS item is a name or value.
                                    //
                                    save.K = save.MTEXPR[save.THIRD];

                                    if (save.DSCBUF[[DSCSIZ, save.K]] == IDENT) {
                                        save.SKIP = 1;
                                    } else {
                                        save.SKIP = 0;
                                    }

                                    {
                                        let m1__: i32 = 1;
                                        let m2__: i32 = (save.SKIP + 1);
                                        let m3__: i32 = 1;
                                        save.J = m1__;
                                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                            save.K = LNKNXT(save.K, save.DSPOOL.as_slice(), ctx)?;
                                            save.J += m3__;
                                        }
                                    }
                                }

                                if (LNKNFN(save.RLPOOL.as_slice()) < 1) {
                                    *ERROR = true;
                                    fstr::assign(PRSERR, b"Relation table is full.");
                                    CHKOUT(b"ZZEKNRML", ctx)?;
                                    return Ok(());
                                }

                                LNKAN(save.RLPOOL.as_slice_mut(), &mut save.REL[save.I], ctx)?;

                                save.RELS[[1, save.REL[save.I]]] = save.MTEXPR[save.FIRST];

                                if (save.I <= 2) {
                                    save.RELS[[2, save.REL[save.I]]] = LT;
                                } else {
                                    save.RELS[[2, save.REL[save.I]]] = GT;
                                }

                                save.RELS[[3, save.REL[save.I]]] = save.K;

                                if (LNKNFN(save.CJPOOL.as_slice()) < 1) {
                                    *ERROR = true;
                                    fstr::assign(PRSERR, b"Conjunction table is full.");
                                    CHKOUT(b"ZZEKNRML", ctx)?;
                                    return Ok(());
                                }

                                LNKAN(save.CJPOOL.as_slice_mut(), &mut save.CJ[save.I], ctx)?;

                                save.CJPTRS[save.CJ[save.I]] = save.REL[save.I];

                                save.I += m3__;
                            }
                        }

                        //
                        // Link the conjunction nodes to form the two
                        // conjunctions shown above.
                        //
                        LNKILA(save.CJ[1], save.CJ[2], save.CJPOOL.as_slice_mut(), ctx)?;
                        LNKILA(save.CJ[3], save.CJ[4], save.CJPOOL.as_slice_mut(), ctx)?;

                        //
                        // Allocate disjunction pool entries and make them
                        // point to the two respective conjunctions.
                        //
                        {
                            let m1__: i32 = 1;
                            let m2__: i32 = 2;
                            let m3__: i32 = 1;
                            save.I = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                if (LNKNFN(save.DJPOOL.as_slice()) < 1) {
                                    *ERROR = true;
                                    fstr::assign(PRSERR, b"Disjunction table is full.");
                                    CHKOUT(b"ZZEKNRML", ctx)?;
                                    return Ok(());
                                }

                                LNKAN(save.DJPOOL.as_slice_mut(), &mut save.DJ[save.I], ctx)?;
                                save.DJPTRS[save.DJ[save.I]] = save.CJ[((2 * save.I) - 1)];

                                save.I += m3__;
                            }
                        }

                        //
                        // Finally, link the disjunction pool entries, and
                        // create an <expression> meta-token.  Free the unused
                        // meta-tokens.
                        //
                        LNKILA(save.DJ[1], save.DJ[2], save.DJPOOL.as_slice_mut(), ctx)?;

                        save.MTCODE[save.FIRST] = EXPR;
                        save.MTEXPR[save.FIRST] = save.DJ[1];

                        LNKFSL(save.SECOND, save.THIRD, save.MTPOOL.as_slice_mut(), ctx)?;
                        save.NMETA = (save.NMETA - 2);
                        //
                        // Decide the next state.
                        //
                        save.STATE = REDUCD;
                    } else if ((save.MTCODE[save.THIRD] == NAME)
                        || (save.MTCODE[save.THIRD] == VALUE))
                    {
                        //
                        // If the third meta-token is anything other than
                        // <BETWEEN expr>, we'll have to parse the portion of
                        // the query following the BETWEEN keyword before
                        // reducing the <BETWEEN> or <NOT BETWEEN> expression.
                        //
                        save.START = save.THIRD;
                        save.RETCND = REDUCE;
                        save.STATE = PUSH;
                    } else {
                        *ERROR = true;
                        fstr::assign(PRSERR, b"Token following BETWEEN operator is invalid.");
                        CHKOUT(b"ZZEKNRML", ctx)?;
                        return Ok(());
                    }
                } else if (save.MTCODE[save.SECOND] > 0) {
                    //
                    // The third meta-token is in the wrong place at the
                    // wrong time.
                    //
                    *ERROR = true;
                    fstr::assign(PRSERR, b"Token sequence must be followed by a value.");

                    CHKOUT(b"ZZEKNRML", ctx)?;
                    return Ok(());
                } else {
                    //
                    // The second meta-token is supposed to be a comparison
                    // operator, but it's actually something else.
                    //
                    *ERROR = true;
                    fstr::assign(PRSERR, b"Token must be followed by a comparison operator.");

                    CHKOUT(b"ZZEKNRML", ctx)?;
                    return Ok(());
                }
            } else if (save.MTCODE[save.FIRST] == EXPR) {
                //
                // If the query is valid, the sequence of meta-tokens
                // should be one of
                //
                //    <expression>
                //    <expression>   )
                //    <expression>   OR   <expression>
                //    <expression>   OR   NAME
                //    <expression>   OR   IDENT
                //    <expression>   OR   NOT
                //    <expression>   OR   (
                //    <expression>   AND  <expression>
                //    <expression>   AND  NAME
                //    <expression>   AND  IDENT
                //    <expression>   AND  NOT
                //    <expression>   AND  (
                //
                if (save.SECOND <= 0) {
                    //
                    // This is the last state we pass through
                    // before exiting the loop.  However, some syntax errors
                    // can get us here as well.
                    //
                    if ((save.LEVEL > 1) || (save.NMETA > 1)) {
                        *ERROR = true;
                        fstr::assign(PRSERR, b"More tokens expected.");
                        CHKOUT(b"ZZEKNRML", ctx)?;
                        return Ok(());
                    }

                    save.STATE = TERM;
                } else if (save.MTCODE[save.SECOND] == RGROUP) {
                    //
                    // We've reached the end of a `parenthesized'
                    // expression.
                    //
                    if ((save.LEVEL > 1) && (save.POPCND[save.LEVEL] == ENDGRP)) {
                        //
                        // Time to pop the state.
                        //
                        save.STATE = POP;
                    } else {
                        //
                        // There should not be a right grouper here.
                        //
                        *ERROR = true;
                        fstr::assign(PRSERR, b"Unexpected right parenthesis found.");
                        CHKOUT(b"ZZEKNRML", ctx)?;
                        return Ok(());
                    }

                //
                // In all other cases, there must be at least three
                // meta-tokens here.  Make sure there are.
                //
                } else if (save.THIRD <= 0) {
                    *ERROR = true;
                    fstr::assign(PRSERR, b"More tokens expected.");
                    CHKOUT(b"ZZEKNRML", ctx)?;
                    return Ok(());

                //
                // Take care of the cases that will require reducing a sub-
                // expression before reducing the current expression.
                //
                } else if (((save.MTCODE[save.THIRD] == IDENT)
                    || (save.MTCODE[save.THIRD] == NAME))
                    || (save.MTCODE[save.THIRD] == NOT))
                {
                    save.START = save.THIRD;
                    save.RETCND = REDUCE;
                    save.STATE = PUSH;
                } else if (save.MTCODE[save.THIRD] == LGROUP) {
                    //
                    // We'll have to push our state before continuing.
                    //
                    save.START = save.FOURTH;
                    save.RETCND = ENDGRP;
                    save.STATE = PUSH;

                //
                // Now continue with the interesting cases.
                //
                } else if (((save.MTCODE[save.FIRST] == EXPR) && (save.MTCODE[save.SECOND] == OR))
                    && (save.MTCODE[save.THIRD] == EXPR))
                {
                    //
                    // We have a disjunction of two normalized
                    // expressions.  We're not ready to perform a
                    // reduction yet; we need to see whether there's
                    // a higher priority operator, namely AND, on the
                    // right of the second expression.
                    //
                    save.DONOW = true;

                    if (save.FOURTH > 0) {
                        if (save.MTCODE[save.FOURTH] == AND) {
                            //
                            // The third token is already spoken for:
                            // the expression involving the operator
                            // to its right must be processed first.
                            //
                            save.DONOW = false;
                        }
                    }

                    if save.DONOW {
                        //
                        // This is an easy case to handle:
                        // we can form the resulting normalized
                        // expression by just linking together the two
                        // lists in the disjunction table.
                        //
                        save.DJ[1] = save.MTEXPR[save.FIRST];
                        save.DJ[2] = save.MTEXPR[save.THIRD];

                        LNKILB(save.DJ[1], save.DJ[2], save.DJPOOL.as_slice_mut(), ctx)?;
                        //
                        // The first meta-token will point to the resulting
                        // normalized expression; we'll discard the other
                        // two meta-tokens.
                        //
                        LNKFSL(save.SECOND, save.THIRD, save.MTPOOL.as_slice_mut(), ctx)?;
                        save.NMETA = (save.NMETA - 2);

                        //
                        // MTEXPR(FIRST) and MTCODE(FIRST) are already
                        // set correctly.  All we need to do is determine
                        // our next state.  The next state defaults to
                        // PARSE; the other possibility is POP.
                        //
                        save.STATE = REDUCD;
                    } else {
                        //
                        // We'll have to reduce the expression on the right
                        // of the third meta-token before coming back to
                        // this expression.  Get ready to push our state.
                        //
                        // The condition that must be met in order to pop our
                        // state will be that we've performed a reduction.
                        //
                        save.RETCND = REDUCE;
                        save.START = save.THIRD;
                        save.STATE = PUSH;
                    }
                //
                // Either we've reduced an OR expression, in which case
                // the state has been set to PARSE or POP, or we've
                // found a sub-expression that must be reduced before
                // we attack the current expression, in which case the
                // state has been set to PUSH.
                //
                } else if (((save.MTCODE[save.FIRST] == EXPR) && (save.MTCODE[save.SECOND] == AND))
                    && (save.MTCODE[save.THIRD] == EXPR))
                {
                    //
                    // We have the conjunction of two normalized
                    // expressions.  This case requires application of
                    // DeMorgan's laws to convert the expression to a
                    // normalized form.
                    //
                    // If we have two normalized expressions, say
                    //
                    //    EXPR1 =        ( A11 and A12 and ... )
                    //                or ( A21 and A22 and ... )
                    //                              .
                    //                              .
                    //                              .
                    //                or ( AM1 and AM2 and ... )
                    //
                    //
                    //    EXPR2 =        ( B11 and B12 and ... )
                    //                or ( B21 and B22 and ... )
                    //                              .
                    //                              .
                    //                              .
                    //                or ( BN1 and BN2 and ... )
                    //
                    //
                    //
                    // Then ( EXPR1 and EXPR2 ) =
                    //
                    //
                    //        or       {  (     ( AI1 and AI2 and ... )
                    //    I = 1,...,M       and ( BJ1 and BJ2 and ... ) )  }
                    //    J = 1,...,N
                    //
                    //
                    // We have the conjunction of two normalized
                    // So, to represent the normalized expression resulting
                    // from the conjuction of the expressions represented by
                    // the meta-tokens FIRST and THIRD, we will loop through
                    // each disjunction list and form the disjunction of all
                    // conjunctions of pairs of conjunctions, one of which is
                    // from the first expression and one of which is from the
                    // second.  After doing this, we'll clean up the
                    // conjunction and disjunction pools by freeing the
                    // elements in those pools used by the original two
                    // meta-tokens FIRST and THIRD.
                    //
                    //
                    save.DJ[1] = save.MTEXPR[save.FIRST];
                    save.DJTAIL = NIL;

                    while (save.DJ[1] > 0) {
                        save.DJ[2] = save.MTEXPR[save.THIRD];

                        while (save.DJ[2] > 0) {
                            //
                            // Allocate a new disjunction table entry,
                            // and create a new conjunction that represents
                            // the conjunction of the conjunction lists
                            // pointed to by DJ(1) and DJ(2).
                            //
                            if (LNKNFN(save.DJPOOL.as_slice()) < 1) {
                                *ERROR = true;
                                fstr::assign(PRSERR, b"Disjunction table is full.");
                                CHKOUT(b"ZZEKNRML", ctx)?;
                                return Ok(());
                            }

                            LNKAN(save.DJPOOL.as_slice_mut(), &mut save.NEWDJ, ctx)?;

                            //
                            // Make copies of the conjunction lists pointed
                            // to by DJ(1) and DJ(2).
                            //
                            save.CJ[1] = save.DJPTRS[save.DJ[1]];
                            save.TAIL = NIL;

                            while (save.CJ[1] > 0) {
                                if (LNKNFN(save.CJPOOL.as_slice()) < 1) {
                                    *ERROR = true;
                                    fstr::assign(PRSERR, b"Conjunction table is full.");
                                    CHKOUT(b"ZZEKNRML", ctx)?;
                                    return Ok(());
                                }

                                LNKAN(save.CJPOOL.as_slice_mut(), &mut save.NEWCJ, ctx)?;
                                LNKILA(save.TAIL, save.NEWCJ, save.CJPOOL.as_slice_mut(), ctx)?;
                                save.TAIL = save.NEWCJ;
                                save.CJPTRS[save.TAIL] = save.CJPTRS[save.CJ[1]];

                                save.CJ[1] = LNKNXT(save.CJ[1], save.CJPOOL.as_slice(), ctx)?;
                            }

                            save.HEAD1 = LNKHL(save.TAIL, save.CJPOOL.as_slice(), ctx)?;

                            save.CJ[2] = save.DJPTRS[save.DJ[2]];
                            save.TAIL = NIL;

                            while (save.CJ[2] > 0) {
                                if (LNKNFN(save.CJPOOL.as_slice()) < 1) {
                                    *ERROR = true;
                                    fstr::assign(PRSERR, b"Conjunction table is full.");
                                    CHKOUT(b"ZZEKNRML", ctx)?;
                                    return Ok(());
                                }

                                LNKAN(save.CJPOOL.as_slice_mut(), &mut save.NEWCJ, ctx)?;
                                LNKILA(save.TAIL, save.NEWCJ, save.CJPOOL.as_slice_mut(), ctx)?;
                                save.TAIL = save.NEWCJ;
                                save.CJPTRS[save.TAIL] = save.CJPTRS[save.CJ[2]];

                                save.CJ[2] = LNKNXT(save.CJ[2], save.CJPOOL.as_slice(), ctx)?;
                            }

                            save.HEAD2 = LNKHL(save.TAIL, save.CJPOOL.as_slice(), ctx)?;

                            //
                            // Now link these copies and make NEWDJ point to
                            // the resulting list.
                            //
                            LNKILB(save.HEAD1, save.HEAD2, save.CJPOOL.as_slice_mut(), ctx)?;

                            save.DJPTRS[save.NEWDJ] = save.HEAD1;

                            //
                            // Link NEWDJ in at the tail of the disjunction
                            // list.
                            //
                            LNKILA(save.DJTAIL, save.NEWDJ, save.DJPOOL.as_slice_mut(), ctx)?;
                            save.DJTAIL = save.NEWDJ;

                            save.DJ[2] = LNKNXT(save.DJ[2], save.DJPOOL.as_slice(), ctx)?;
                        }

                        save.DJ[1] = LNKNXT(save.DJ[1], save.DJPOOL.as_slice(), ctx)?;
                    }

                    //
                    // We've now created the new normalized expression that
                    // represents the conjunction of our original two
                    // expressions.
                    //
                    // Before continuing, we should clean up the entries in
                    // the disjunction and conjunction pools used by the
                    // original expressions.  We can save a little work
                    // by linking those entries before freeing them.
                    //
                    LNKILB(
                        save.MTEXPR[save.FIRST],
                        save.MTEXPR[save.THIRD],
                        save.DJPOOL.as_slice_mut(),
                        ctx,
                    )?;

                    save.DJNODE = save.MTEXPR[save.FIRST];

                    while (save.DJNODE > 0) {
                        //
                        // Free the conjunction list pointed to by DJNODE.
                        //
                        save.CJNODE = save.DJPTRS[save.DJNODE];

                        LNKFSL(
                            save.CJNODE,
                            LNKTL(save.CJNODE, save.CJPOOL.as_slice(), ctx)?,
                            save.CJPOOL.as_slice_mut(),
                            ctx,
                        )?;

                        save.DJNODE = LNKNXT(save.DJNODE, save.DJPOOL.as_slice(), ctx)?;
                    }
                    //
                    // Free the disjunction list that starts with
                    // MTEXPR(FIRST).
                    //
                    LNKFSL(
                        save.MTEXPR[save.FIRST],
                        LNKTL(save.MTEXPR[save.FIRST], save.DJPOOL.as_slice(), ctx)?,
                        save.DJPOOL.as_slice_mut(),
                        ctx,
                    )?;

                    //
                    // NEWDJ is the tail node of the list of disjunctions
                    // we've just finished.  The first meta-token should
                    // point to the head of this disjunction list.
                    //
                    save.MTEXPR[save.FIRST] = LNKHL(save.NEWDJ, save.DJPOOL.as_slice(), ctx)?;

                    //
                    // We no longer need the other two meta-tokens.
                    //
                    LNKFSL(save.SECOND, save.THIRD, save.MTPOOL.as_slice_mut(), ctx)?;
                    save.NMETA = (save.NMETA - 2);

                    //
                    // Decide the next state.
                    //
                    save.STATE = REDUCD;
                } else {
                    //
                    // There are no other valid cases in which the first
                    // meta-token is an expression.
                    //
                    *ERROR = true;
                    fstr::assign(
                        PRSERR,
                        b"Unexpected token found following valid expression.",
                    );
                    CHKOUT(b"ZZEKNRML", ctx)?;
                    return Ok(());
                }
            } else if (save.MTCODE[save.FIRST] == NOT) {
                //
                // There are four valid token sequences that we could
                // see here:
                //
                //    NOT  <expression>
                //    NOT  IDENT
                //    NOT  NAME
                //    NOT  NOT
                //    NOT  (
                //
                if (save.SECOND <= 0) {
                    *ERROR = true;
                    fstr::assign(PRSERR, b"Tokens were missing from logical expression.");
                    CHKOUT(b"ZZEKNRML", ctx)?;
                    return Ok(());
                } else if (save.MTCODE[save.SECOND] == LGROUP) {
                    //
                    // We'll have to push our state before continuing.
                    //
                    save.START = save.THIRD;
                    save.RETCND = ENDGRP;
                    save.STATE = PUSH;
                } else if (((save.MTCODE[save.SECOND] == NOT)
                    || (save.MTCODE[save.SECOND] == IDENT))
                    || (save.MTCODE[save.SECOND] == NAME))
                {
                    save.START = save.SECOND;
                    save.RETCND = REDUCE;
                    save.STATE = PUSH;
                } else if (save.MTCODE[save.SECOND] == EXPR) {
                    //
                    // We have the negation of a normalized expression. Since
                    // the NOT operator has higher precedence than any other,
                    // we need not concern ourselves with the token on the
                    // right of the expression.
                    //
                    // This case requires application of DeMorgan's laws to
                    // convert the expression to a normalized form.
                    //
                    //
                    // If we have a normalized expression, say
                    //
                    //    EXPR  =         ( A11 and A12 and ... )
                    //                 or ( A21 and A22 and ... )
                    //                              .
                    //                              .
                    //                              .
                    //                 or ( AM1 and AM2 and ... )
                    //
                    // Then (using the tilde to express negation):
                    //
                    //    ~EXPR =          ( ~A11 or ~A12 or ... )
                    //                 and ( ~A21 or ~A22 or ... )
                    //                              .
                    //                              .
                    //                              .
                    //                 and ( ~AM1 or ~AM2 or ... )
                    //
                    // Since each parenthesized expression above is a
                    // normalized expression, we can convert the conjunction
                    // of any of these expressions and a second normalized
                    // expression to normalized form using the method of the
                    // AND case above.
                    //
                    // We'll first build the expression
                    //
                    //    ( ~A11 or ~A12 or ... )
                    //
                    // and then combine the others with it, one by one.
                    // When we're all done, we'll negate the operators used
                    // in the comparison relations.
                    //
                    // The pointer EXPRHD will denote the head of the
                    // combined normalized expression.
                    //
                    save.DJNODE = save.MTEXPR[save.SECOND];

                    save.CJNODE = save.DJPTRS[save.DJNODE];
                    save.TAIL = NIL;

                    while (save.CJNODE > 0) {
                        //
                        // Create a new singleton disjunction list
                        // that points to the relation pointed to by
                        // CJNODE.
                        //
                        if (LNKNFN(save.DJPOOL.as_slice()) < 1) {
                            *ERROR = true;
                            fstr::assign(PRSERR, b"Disjunction table is full.");
                            CHKOUT(b"ZZEKNRML", ctx)?;
                            return Ok(());
                        }

                        LNKAN(save.DJPOOL.as_slice_mut(), &mut save.NEWDJ, ctx)?;

                        if (LNKNFN(save.CJPOOL.as_slice()) < 1) {
                            *ERROR = true;
                            fstr::assign(PRSERR, b"Conjunction table is full.");
                            CHKOUT(b"ZZEKNRML", ctx)?;
                            return Ok(());
                        }

                        LNKAN(save.CJPOOL.as_slice_mut(), &mut save.NEWCJ, ctx)?;

                        save.CJPTRS[save.NEWCJ] = save.CJPTRS[save.CJNODE];
                        save.DJPTRS[save.NEWDJ] = save.NEWCJ;
                        //
                        // Now link the new singleton disjunction list in
                        // at the tail of the disjunction list that
                        // parallels the conjunction list we're currently
                        // traversing.
                        //
                        LNKILA(save.TAIL, save.NEWDJ, save.DJPOOL.as_slice_mut(), ctx)?;
                        save.TAIL = save.NEWDJ;

                        save.CJNODE = LNKNXT(save.CJNODE, save.CJPOOL.as_slice(), ctx)?;
                    }

                    //
                    // Keep track of the head of the new normalized
                    // expression.
                    //
                    save.EXPRHD = LNKHL(save.TAIL, save.DJPOOL.as_slice(), ctx)?;

                    //
                    // Now, for every remaining conjunction in the original
                    // expression, we'll form the normalized expression
                    // resulting from the conjunction of its negation and
                    // of our cumulative normalized expression.  As mentioned
                    // before, we won't negate the comparison operators
                    // just yet.
                    //
                    //
                    save.DJNODE = LNKNXT(save.DJNODE, save.DJPOOL.as_slice(), ctx)?;

                    while (save.DJNODE > 0) {
                        //
                        // Loop through our existing cumulative
                        // expression and the latest conjunction, forming
                        // all pairwise conjunctions.
                        //
                        save.DJ[1] = save.EXPRHD;
                        save.DJTAIL = NIL;

                        while (save.DJ[1] > 0) {
                            save.CJ[2] = save.DJPTRS[save.DJNODE];

                            while (save.CJ[2] > 0) {
                                //
                                // Make a copy of the conjunction list pointed
                                // to by DJPTRS(DJ(1)).
                                //
                                save.CJNODE = save.DJPTRS[save.DJ[1]];
                                save.TAIL = NIL;

                                while (save.CJNODE > 0) {
                                    if (LNKNFN(save.CJPOOL.as_slice()) < 1) {
                                        *ERROR = true;
                                        fstr::assign(PRSERR, b"Conjunction table is full.");
                                        CHKOUT(b"ZZEKNRML", ctx)?;
                                        return Ok(());
                                    }

                                    LNKAN(save.CJPOOL.as_slice_mut(), &mut save.NEWCJ, ctx)?;
                                    LNKILA(save.TAIL, save.NEWCJ, save.CJPOOL.as_slice_mut(), ctx)?;

                                    save.CJPTRS[save.NEWCJ] = save.CJPTRS[save.CJNODE];
                                    save.TAIL = save.NEWCJ;
                                    save.CJNODE = LNKNXT(save.CJNODE, save.CJPOOL.as_slice(), ctx)?;
                                }

                                save.CJ[1] = LNKHL(save.TAIL, save.CJPOOL.as_slice(), ctx)?;

                                //
                                // Allocate a new conjunction table entry for
                                // the conjunction of the expressions
                                // pointed to by CJ(1) and CJ(2).  Allocate a
                                // new disjunction table entry to point to this
                                // new conjunction.
                                //
                                if (LNKNFN(save.CJPOOL.as_slice()) < 1) {
                                    *ERROR = true;
                                    fstr::assign(PRSERR, b"Conjunction table is full.");
                                    CHKOUT(b"ZZEKNRML", ctx)?;
                                    return Ok(());
                                }

                                LNKAN(save.CJPOOL.as_slice_mut(), &mut save.NEWCJ, ctx)?;
                                save.CJPTRS[save.NEWCJ] = save.CJPTRS[save.CJ[2]];

                                if (LNKNFN(save.DJPOOL.as_slice()) < 1) {
                                    *ERROR = true;
                                    fstr::assign(PRSERR, b"Disjunction table is full.");
                                    CHKOUT(b"ZZEKNRML", ctx)?;
                                    return Ok(());
                                }

                                LNKAN(save.DJPOOL.as_slice_mut(), &mut save.NEWDJ, ctx)?;

                                //
                                // Hook everything up.
                                //
                                LNKILB(save.CJ[1], save.NEWCJ, save.CJPOOL.as_slice_mut(), ctx)?;

                                save.DJPTRS[save.NEWDJ] = save.CJ[1];

                                LNKILA(save.DJTAIL, save.NEWDJ, save.DJPOOL.as_slice_mut(), ctx)?;
                                save.DJTAIL = save.NEWDJ;

                                save.CJ[2] = LNKNXT(save.CJ[2], save.CJPOOL.as_slice(), ctx)?;
                            }

                            save.DJ[1] = LNKNXT(save.DJ[1], save.DJPOOL.as_slice(), ctx)?;
                        }

                        //
                        // Before going on, clean up the conjunction and
                        // disjunction pool entries used by our last
                        // version of the cumulative expression.
                        //
                        save.DJ[1] = save.EXPRHD;

                        while (save.DJ[1] > 0) {
                            save.CJ[1] = save.DJPTRS[save.DJ[1]];
                            save.CJ[2] = LNKTL(save.CJ[1], save.CJPOOL.as_slice(), ctx)?;

                            LNKFSL(save.CJ[1], save.CJ[2], save.CJPOOL.as_slice_mut(), ctx)?;

                            save.DJ[1] = LNKNXT(save.DJ[1], save.DJPOOL.as_slice(), ctx)?;
                        }

                        LNKFSL(
                            save.EXPRHD,
                            LNKTL(save.EXPRHD, save.DJPOOL.as_slice(), ctx)?,
                            save.DJPOOL.as_slice_mut(),
                            ctx,
                        )?;

                        //
                        // Set EXPRHD to be the head of our updated,
                        // cumulative expression.  Start to work on the
                        // next conjunction.
                        //
                        save.EXPRHD = LNKHL(save.DJTAIL, save.DJPOOL.as_slice(), ctx)?;
                        save.DJNODE = LNKNXT(save.DJNODE, save.DJPOOL.as_slice(), ctx)?;
                    }

                    //
                    // EXPRHD now points to a new expression that will
                    // represent the negation of the expression pointed
                    // to by MTEXPR(SECOND), as soon as we negate the
                    // comparison operators referenced in the expression.
                    // Take care of this last step now.  To make sure that
                    // we negate each operator exactly once, we build a set
                    // of relations to be negated, then negate each relation
                    // in the set.
                    //
                    SSIZEI(MAXREL, save.RELSET.as_slice_mut(), ctx)?;

                    save.DJNODE = save.MTEXPR[save.SECOND];

                    while (save.DJNODE > 0) {
                        save.CJNODE = save.DJPTRS[save.DJNODE];

                        while (save.CJNODE > 0) {
                            save.RELPTR = save.CJPTRS[save.CJNODE];
                            INSRTI(save.RELPTR, save.RELSET.as_slice_mut(), ctx)?;

                            save.CJNODE = LNKNXT(save.CJNODE, save.CJPOOL.as_slice(), ctx)?;
                        }

                        save.DJNODE = LNKNXT(save.DJNODE, save.DJPOOL.as_slice(), ctx)?;
                    }

                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = CARDI(save.RELSET.as_slice(), ctx)?;
                        let m3__: i32 = 1;
                        save.I = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            save.RELPTR = save.RELSET[save.I];
                            save.J =
                                ISRCHI(save.RELS[[2, save.RELPTR]], NRELOP, save.CMPCDE.as_slice());
                            save.RELS[[2, save.RELPTR]] = save.CMPNEG[save.J];

                            save.I += m3__;
                        }
                    }

                    //
                    // Set the pointer of the first meta-token to point
                    // to our normalized expression, and change the
                    // meta-token's code to <expr>.
                    //
                    save.MTEXPR[save.FIRST] = save.EXPRHD;
                    save.MTCODE[save.FIRST] = EXPR;

                    //
                    // Get rid of the second meta-token, and determine the
                    // next state.
                    //
                    LNKFSL(save.SECOND, save.SECOND, save.MTPOOL.as_slice_mut(), ctx)?;
                    save.NMETA = (save.NMETA - 1);

                    save.STATE = REDUCD;
                } else {
                    //
                    // The second token is invalid in this context.
                    //
                    *ERROR = true;
                    fstr::assign(PRSERR, b"Token following NOT operator was invalid.");
                    CHKOUT(b"ZZEKNRML", ctx)?;
                    return Ok(());
                }
            //
            // This is the end of the NOT case.
            //
            } else if (save.MTCODE[save.FIRST] == LGROUP) {
                //
                // We're looking at the start of a `parenthesized'
                // sub-expression.
                //
                // Push our state, and start parsing at meta-token
                // SECOND.  The condition for popping our state will be
                // that we encounter a right grouper.
                //
                save.RETCND = ENDGRP;
                save.START = save.SECOND;
                save.STATE = PUSH;
            } else {
                //
                // Only a syntax error could get us here.
                //
                *ERROR = true;
                fstr::assign(PRSERR, b"Unexpected token found.");
                CHKOUT(b"ZZEKNRML", ctx)?;
                return Ok(());
            }

        //
        // This is the end of the code for the PARSE state.  We've
        // determined the next parsing state at this point.
        //
        } else if (save.STATE == REDUCD) {
            //
            // A reduction has been done.  Decide the next state.
            //
            save.STATE = REDUCD;

            if (save.POPCND[save.LEVEL] == REDUCE) {
                save.STATE = POP;
            } else {
                save.MSTART[save.LEVEL] = save.FIRST;
                save.STATE = PARSE;
            }
        } else if (save.STATE == PUSH) {
            //
            // Increment the stack level, and save the current
            // starting point and pop condition.
            //
            save.LEVEL = (save.LEVEL + 1);

            if (save.LEVEL > MAXSTK) {
                *ERROR = true;
                fstr::assign(PRSERR, b"Stack is full");
                save.STATE = TERM;
            } else {
                save.MSTART[save.LEVEL] = save.START;
                save.POPCND[save.LEVEL] = save.RETCND;
                save.STATE = PARSE;
            }
        } else if (save.STATE == POP) {
            //
            // If we can, pop the state.
            //
            if (save.LEVEL > 1) {
                if (save.POPCND[save.LEVEL] == ENDGRP) {
                    //
                    // If we're popping the state because we encountered a
                    // right grouper, we have a meta-token sequence that
                    // looks like this:
                    //
                    //    (  EXPR  )
                    //
                    //        ^    ^
                    //     FIRST  SECOND
                    //
                    // We need to remove the grouping tokens, taking care to
                    // update the starting token at the next lower level, if
                    // the left grouper was the starting token.
                    //
                    save.PREV = LNKPRV(save.FIRST, save.MTPOOL.as_slice(), ctx)?;

                    if (save.MSTART[(save.LEVEL - 1)] == save.PREV) {
                        save.MSTART[(save.LEVEL - 1)] = save.FIRST;
                    }

                    if (save.METAHD == save.PREV) {
                        save.METAHD = save.FIRST;
                    }

                    LNKFSL(save.PREV, save.PREV, save.MTPOOL.as_slice_mut(), ctx)?;
                    LNKFSL(save.SECOND, save.SECOND, save.MTPOOL.as_slice_mut(), ctx)?;

                    save.NMETA = (save.NMETA - 2);
                }

                save.LEVEL = (save.LEVEL - 1);
                save.STATE = PARSE;
            } else {
                *ERROR = true;
                fstr::assign(PRSERR, b"Syntax error:  badly formed WHERE clause.");
                CHKOUT(b"ZZEKNRML", ctx)?;
                return Ok(());
            }
        }

        //
        // We've considered all states.
        //
    }

    //
    // At this point, there should be a single meta-token of type EXPR.
    // This meta-token should point to a normalized expression.  We'll
    // set the encoded query to represent this expression.  For each
    // constraint, we'll add a constraint descriptor to the encoded
    // query.  We'll also update the count of constraints, the count of
    // conjunctions, and we'll add a list of conjunction sizes.
    //
    save.DJNODE = save.MTEXPR[save.FIRST];
    save.NCONJ = 0;
    save.NRELS = 0;

    while (save.DJNODE > 0) {
        save.NCONJ = (save.NCONJ + 1);
        save.SIZES[save.NCONJ] = 0;
        save.CJNODE = save.DJPTRS[save.DJNODE];

        while (save.CJNODE > 0) {
            save.NRELS = (save.NRELS + 1);
            save.SIZES[save.NCONJ] = (save.SIZES[save.NCONJ] + 1);
            save.RELPTR = save.CJPTRS[save.CJNODE];

            save.TABPTR = save.RELS[[1, save.RELPTR]];
            save.OP = save.RELS[[2, save.RELPTR]];
            save.RHSPTR = save.RELS[[3, save.RELPTR]];

            //
            // Add a constraint descriptor to the encoded query.  The
            // structure of these descriptors is documented in the include
            // file for encoded query parameters.
            //
            // First, save space for the constraint type.  We'll fill this
            // in after finding out what's on the right hand side.
            //
            APPNDI(0, EQRYI.as_slice_mut(), ctx)?;
            save.K = CARDI(EQRYI.as_slice(), ctx)?;

            //
            // Next, add name descriptors for the table and column on
            // the left-hand side.  These descriptors are linked and
            // pointed to by NAMPTR.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = EQVDSZ;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    APPNDI(
                        save.DSCBUF[[save.I, save.TABPTR]],
                        EQRYI.as_slice_mut(),
                        ctx,
                    )?;
                    save.I += m3__;
                }
            }

            save.COLPTR = LNKNXT(save.TABPTR, save.DSPOOL.as_slice(), ctx)?;

            {
                let m1__: i32 = 1;
                let m2__: i32 = EQVDSZ;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    APPNDI(
                        save.DSCBUF[[save.I, save.COLPTR]],
                        EQRYI.as_slice_mut(),
                        ctx,
                    )?;
                    save.I += m3__;
                }
            }

            //
            // What happens next depends on whether the query has a null
            // value on the right hand side.  This is indicated by the
            // relation's value pointer being NIL.
            //
            if (save.RHSPTR == NIL) {
                //
                // For constraints involving null values, we change the
                // operator to ISNULL or NOTNUL as appropriate.
                //
                if (save.OP == EQ) {
                    save.OP = ISNULL;
                } else if (save.OP == NE) {
                    save.OP = NOTNUL;
                } else {
                    *ERROR = true;
                    fstr::assign(PRSERR, b"NULL values can only be used with the operators \"IS NULL\", \"NOT NULL\", or equivalents.");
                    CHKOUT(b"ZZEKNRML", ctx)?;
                    return Ok(());
                }

                //
                // Set the operator code.
                //
                APPNDI(save.OP, EQRYI.as_slice_mut(), ctx)?;

                //
                // Pad the constraint descriptor up to the full length.
                //
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = (2 * EQVDSZ);
                    let m3__: i32 = 1;
                    save.I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        APPNDI(0, EQRYI.as_slice_mut(), ctx)?;
                        save.I += m3__;
                    }
                }

                //
                // Set the descriptor's type by updating the reserved
                // location.
                //
                EQRYI[save.K] = EQVAL;
            } else {
                //
                // For `normal' constraints, that is, constraints that don't
                // involve null values, we set the operator code, then
                // fill in the information describing the RHS of the
                // constraint.
                //
                APPNDI(save.OP, EQRYI.as_slice_mut(), ctx)?;

                if (save.DSCBUF[[DSCSIZ, save.RHSPTR]] == VALUE) {
                    //
                    // The RHS contains a value.  Append the descriptor
                    // for the value, then pad the constraint descriptor.
                    //
                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = EQVDSZ;
                        let m3__: i32 = 1;
                        save.I = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            APPNDI(
                                save.DSCBUF[[save.I, save.RHSPTR]],
                                EQRYI.as_slice_mut(),
                                ctx,
                            )?;
                            save.I += m3__;
                        }
                    }

                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = EQVDSZ;
                        let m3__: i32 = 1;
                        save.I = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            APPNDI(0, EQRYI.as_slice_mut(), ctx)?;
                            save.I += m3__;
                        }
                    }

                    //
                    // Set the descriptor's type by updating the reserved
                    // location.
                    //
                    EQRYI[save.K] = EQVAL;
                } else {
                    //
                    // The RHS contains a column name.  Append the
                    // descriptors for the table and column.
                    //
                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = EQVDSZ;
                        let m3__: i32 = 1;
                        save.I = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            APPNDI(
                                save.DSCBUF[[save.I, save.RHSPTR]],
                                EQRYI.as_slice_mut(),
                                ctx,
                            )?;
                            save.I += m3__;
                        }
                    }

                    save.COLPTR = LNKNXT(save.RHSPTR, save.DSPOOL.as_slice(), ctx)?;

                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = EQVDSZ;
                        let m3__: i32 = 1;
                        save.I = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            APPNDI(
                                save.DSCBUF[[save.I, save.COLPTR]],
                                EQRYI.as_slice_mut(),
                                ctx,
                            )?;
                            save.I += m3__;
                        }
                    }

                    //
                    // Set the descriptor's type by updating the reserved
                    // location.
                    //
                    EQRYI[save.K] = EQCOL;
                }
            }
            //
            // We've updated the encoded query to reflect the current
            // constraint relation.
            //
            save.CJNODE = LNKNXT(save.CJNODE, save.CJPOOL.as_slice(), ctx)?;
        }
        //
        // We've set the array element SIZES(NCONJ).
        //
        save.DJNODE = LNKNXT(save.DJNODE, save.DJPOOL.as_slice(), ctx)?;
    }

    //
    // Set the counts of constraints and conjunctions in the encoded
    // query.
    //
    ZZEKWEQI(b"NUM_CONSTRAINTS", save.NRELS, EQRYI.as_slice_mut(), ctx)?;
    ZZEKWEQI(b"NUM_CONJUNCTIONS", save.NCONJ, EQRYI.as_slice_mut(), ctx)?;

    //
    // Add the conjunction size list to the encoded query.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCONJ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            APPNDI(save.SIZES[save.I], EQRYI.as_slice_mut(), ctx)?;
            save.I += m3__;
        }
    }

    CHKOUT(b"ZZEKNRML", ctx)?;
    Ok(())
}
