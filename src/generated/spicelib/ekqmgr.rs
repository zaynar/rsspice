//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
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
const MXJRS: i32 = 200;
const JSZIDX: i32 = 1;
const JRCIDX: i32 = 2;
const JTCIDX: i32 = 3;
const JSCIDX: i32 = 4;
const JSVBAS: i32 = 4;
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
const TNAMSZ: i32 = 64;
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const FTSIZE: i32 = 20;
const STSIZE: i32 = 200;
const MXTBLD: i32 = 100;
const MXCLLD: i32 = 500;
const LBCELL: i32 = -5;
const MAXSEG: i32 = STSIZE;
const CTSIZE: i32 = MXCLLD;
const DTSIZE: i32 = ((MAXSEG * MXCLLD) / 10);
const LBPOOL: i32 = -5;
const NIL: i32 = 0;
const NTYPES: i32 = 4;
const TYPLEN: i32 = 4;
const SHORT: i32 = 80;

struct SaveVars {
    FTPOOL: StackArray2D<i32, 52>,
    FTHAN: StackArray<i32, 20>,
    FTHEAD: i32,
    TBPOOL: StackArray2D<i32, 212>,
    TBSTPT: StackArray<i32, 100>,
    TBNCOL: StackArray<i32, 100>,
    TBNAMS: ActualCharArray,
    TBCTPT: StackArray<i32, 100>,
    TBFILS: ActualArray2D<i32>,
    TBFLSZ: StackArray<i32, 100>,
    TBHEAD: i32,
    STPOOL: ActualArray2D<i32>,
    STHAN: StackArray<i32, 200>,
    STSIDX: StackArray<i32, 200>,
    STDSCS: ActualArray2D<i32>,
    STNROW: StackArray<i32, 200>,
    STNCOL: StackArray<i32, 200>,
    STDTPT: StackArray<i32, 200>,
    DTPOOL: ActualArray2D<i32>,
    DTDSCS: ActualArray2D<i32>,
    CTPOOL: ActualArray2D<i32>,
    CTNAMS: ActualCharArray,
    CTCLAS: ActualArray<i32>,
    CTTYPS: ActualArray<i32>,
    CTLENS: ActualArray<i32>,
    CTFIXD: ActualArray<bool>,
    CTSIZS: ActualArray<i32>,
    CTINDX: ActualArray<bool>,
    CTNULL: ActualArray<bool>,
    CHTYPE: ActualCharArray,
    CNAMS: ActualCharArray,
    CNMSET: ActualCharArray,
    COLNAM: Vec<u8>,
    FRMALS: ActualCharArray,
    FRMTAB: ActualCharArray,
    LCNAME: Vec<u8>,
    LTNAME: Vec<u8>,
    PROBLM: Vec<u8>,
    RCNAME: Vec<u8>,
    RTNAME: Vec<u8>,
    STATE: Vec<u8>,
    TABNAM: Vec<u8>,
    TABVEC: ActualCharArray,
    DVALS: ActualArray<f64>,
    BEGIDX: i32,
    CBEGS: ActualArray<i32>,
    CDSCRS: ActualArray2D<i32>,
    CENDS: ActualArray<i32>,
    CJBEG: i32,
    CJEND: i32,
    CJROWS: i32,
    CJSIZE: i32,
    CNSTYP: ActualArray<i32>,
    COL: i32,
    COLPTR: i32,
    CVLEN: i32,
    CTNEW: i32,
    DELSEG: i32,
    DTNEW: i32,
    DTYPE: ActualArray<i32>,
    ENDIDX: i32,
    I: i32,
    IVALS: ActualArray<i32>,
    J: i32,
    JBASE1: i32,
    JBASE2: i32,
    JSIZE: i32,
    K: i32,
    KEY: i32,
    KEYDSC: StackArray<i32, 11>,
    L: i32,
    LCIDX: ActualArray<i32>,
    LELTS: ActualArray<i32>,
    LDSCRS: ActualArray2D<i32>,
    LTBIDX: ActualArray<i32>,
    LXBEG: i32,
    LXEND: i32,
    NACT: i32,
    NCOLS: i32,
    NCONJ: i32,
    NEW: i32,
    NEXT: i32,
    NMATCH: i32,
    NORDER: i32,
    NPCOL: i32,
    NSEG: i32,
    NSEL: i32,
    NSV: i32,
    NTAB: i32,
    OCOLS: StackArray<i32, 10>,
    OELTS: StackArray<i32, 10>,
    OPS: ActualArray<i32>,
    ORDBAS: i32,
    OTABS: StackArray<i32, 10>,
    PTROFF: i32,
    R: i32,
    RBAS: StackArray<i32, 10>,
    RCIDX: ActualArray<i32>,
    RDSCRS: ActualArray2D<i32>,
    RELTS: ActualArray<i32>,
    RESBAS: i32,
    ROWIDX: i32,
    ROWVEC: StackArray<i32, 10>,
    RSIZE: StackArray<i32, 200>,
    RTBIDX: ActualArray<i32>,
    RTOTAL: i32,
    RWVBAS: i32,
    SELCOL: StackArray<i32, 50>,
    SELCTP: StackArray<i32, 50>,
    SELTAB: StackArray<i32, 50>,
    SEG: i32,
    SEGDSC: StackArray<i32, 24>,
    SEGVEC: StackArray<i32, 10>,
    SENSE: StackArray<i32, 10>,
    SGVBAS: i32,
    SIZES: ActualArray<i32>,
    STNEW: i32,
    TAB: i32,
    TABIDX: i32,
    TBCURR: i32,
    TOP: i32,
    TPTVEC: StackArray<i32, 16>,
    UBASE: StackArray<i32, 200>,
    UNROWS: i32,
    USIZE: i32,
    ACTIVC: ActualArray<bool>,
    ACTIVV: ActualArray<bool>,
    ATTMCH: bool,
    CMTCH: bool,
    DOSORT: bool,
    FIRST: bool,
    FND: bool,
    INDEXD: bool,
    KEYFND: bool,
    NULSOK: bool,
    PRESNT: bool,
    SORTED: bool,
    VMTCH: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FTPOOL = StackArray2D::<i32, 52>::new(1..=2, LBPOOL..=FTSIZE);
        let mut FTHAN = StackArray::<i32, 20>::new(1..=FTSIZE);
        let mut FTHEAD: i32 = 0;
        let mut TBPOOL = StackArray2D::<i32, 212>::new(1..=2, LBPOOL..=MXTBLD);
        let mut TBSTPT = StackArray::<i32, 100>::new(1..=MXTBLD);
        let mut TBNCOL = StackArray::<i32, 100>::new(1..=MXTBLD);
        let mut TBNAMS = ActualCharArray::new(TNAMSZ, 1..=MXTBLD);
        let mut TBCTPT = StackArray::<i32, 100>::new(1..=MXTBLD);
        let mut TBFILS = ActualArray2D::<i32>::new(1..=FTSIZE, 1..=MXTBLD);
        let mut TBFLSZ = StackArray::<i32, 100>::new(1..=MXTBLD);
        let mut TBHEAD: i32 = 0;
        let mut STPOOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=MAXSEG);
        let mut STHAN = StackArray::<i32, 200>::new(1..=MAXSEG);
        let mut STSIDX = StackArray::<i32, 200>::new(1..=MAXSEG);
        let mut STDSCS = ActualArray2D::<i32>::new(1..=SDSCSZ, 1..=MAXSEG);
        let mut STNROW = StackArray::<i32, 200>::new(1..=MAXSEG);
        let mut STNCOL = StackArray::<i32, 200>::new(1..=MAXSEG);
        let mut STDTPT = StackArray::<i32, 200>::new(1..=MAXSEG);
        let mut DTPOOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=DTSIZE);
        let mut DTDSCS = ActualArray2D::<i32>::new(1..=CDSCSZ, 1..=DTSIZE);
        let mut CTPOOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=CTSIZE);
        let mut CTNAMS = ActualCharArray::new(CNAMSZ, 1..=CTSIZE);
        let mut CTCLAS = ActualArray::<i32>::new(1..=CTSIZE);
        let mut CTTYPS = ActualArray::<i32>::new(1..=CTSIZE);
        let mut CTLENS = ActualArray::<i32>::new(1..=CTSIZE);
        let mut CTFIXD = ActualArray::<bool>::new(1..=CTSIZE);
        let mut CTSIZS = ActualArray::<i32>::new(1..=CTSIZE);
        let mut CTINDX = ActualArray::<bool>::new(1..=CTSIZE);
        let mut CTNULL = ActualArray::<bool>::new(1..=CTSIZE);
        let mut CHTYPE = ActualCharArray::new(TYPLEN, 1..=NTYPES);
        let mut CNAMS = ActualCharArray::new(CNAMSZ, 1..=MXCLLD);
        let mut CNMSET = ActualCharArray::new(CNAMSZ, LBCELL..=MXCLLD);
        let mut COLNAM = vec![b' '; CNAMSZ as usize];
        let mut FRMALS = ActualCharArray::new(TNAMSZ, 1..=MAXTAB);
        let mut FRMTAB = ActualCharArray::new(TNAMSZ, 1..=MAXTAB);
        let mut LCNAME = vec![b' '; CNAMSZ as usize];
        let mut LTNAME = vec![b' '; TNAMSZ as usize];
        let mut PROBLM = vec![b' '; SHORT as usize];
        let mut RCNAME = vec![b' '; CNAMSZ as usize];
        let mut RTNAME = vec![b' '; TNAMSZ as usize];
        let mut STATE = vec![b' '; SHORT as usize];
        let mut TABNAM = vec![b' '; TNAMSZ as usize];
        let mut TABVEC = ActualCharArray::new(TNAMSZ, LBCELL..=MAXTAB);
        let mut DVALS = ActualArray::<f64>::new(1..=MAXCON);
        let mut BEGIDX: i32 = 0;
        let mut CBEGS = ActualArray::<i32>::new(1..=MAXCON);
        let mut CDSCRS = ActualArray2D::<i32>::new(1..=CDSCSZ, 1..=MXCLLD);
        let mut CENDS = ActualArray::<i32>::new(1..=MAXCON);
        let mut CJBEG: i32 = 0;
        let mut CJEND: i32 = 0;
        let mut CJROWS: i32 = 0;
        let mut CJSIZE: i32 = 0;
        let mut CNSTYP = ActualArray::<i32>::new(1..=MAXCON);
        let mut COL: i32 = 0;
        let mut COLPTR: i32 = 0;
        let mut CVLEN: i32 = 0;
        let mut CTNEW: i32 = 0;
        let mut DELSEG: i32 = 0;
        let mut DTNEW: i32 = 0;
        let mut DTYPE = ActualArray::<i32>::new(1..=MAXCON);
        let mut ENDIDX: i32 = 0;
        let mut I: i32 = 0;
        let mut IVALS = ActualArray::<i32>::new(1..=MAXCON);
        let mut J: i32 = 0;
        let mut JBASE1: i32 = 0;
        let mut JBASE2: i32 = 0;
        let mut JSIZE: i32 = 0;
        let mut K: i32 = 0;
        let mut KEY: i32 = 0;
        let mut KEYDSC = StackArray::<i32, 11>::new(1..=CDSCSZ);
        let mut L: i32 = 0;
        let mut LCIDX = ActualArray::<i32>::new(1..=MAXCON);
        let mut LELTS = ActualArray::<i32>::new(1..=MAXCON);
        let mut LDSCRS = ActualArray2D::<i32>::new(1..=CDSCSZ, 1..=MAXCON);
        let mut LTBIDX = ActualArray::<i32>::new(1..=MAXCON);
        let mut LXBEG: i32 = 0;
        let mut LXEND: i32 = 0;
        let mut NACT: i32 = 0;
        let mut NCOLS: i32 = 0;
        let mut NCONJ: i32 = 0;
        let mut NEW: i32 = 0;
        let mut NEXT: i32 = 0;
        let mut NMATCH: i32 = 0;
        let mut NORDER: i32 = 0;
        let mut NPCOL: i32 = 0;
        let mut NSEG: i32 = 0;
        let mut NSEL: i32 = 0;
        let mut NSV: i32 = 0;
        let mut NTAB: i32 = 0;
        let mut OCOLS = StackArray::<i32, 10>::new(1..=MAXORD);
        let mut OELTS = StackArray::<i32, 10>::new(1..=MAXORD);
        let mut OPS = ActualArray::<i32>::new(1..=MAXCON);
        let mut ORDBAS: i32 = 0;
        let mut OTABS = StackArray::<i32, 10>::new(1..=MAXORD);
        let mut PTROFF: i32 = 0;
        let mut R: i32 = 0;
        let mut RBAS = StackArray::<i32, 10>::new(1..=MXJOIN);
        let mut RCIDX = ActualArray::<i32>::new(1..=MAXCON);
        let mut RDSCRS = ActualArray2D::<i32>::new(1..=CDSCSZ, 1..=MAXCON);
        let mut RELTS = ActualArray::<i32>::new(1..=MAXCON);
        let mut RESBAS: i32 = 0;
        let mut ROWIDX: i32 = 0;
        let mut ROWVEC = StackArray::<i32, 10>::new(1..=MAXTAB);
        let mut RSIZE = StackArray::<i32, 200>::new(1..=MXJRS);
        let mut RTBIDX = ActualArray::<i32>::new(1..=MAXCON);
        let mut RTOTAL: i32 = 0;
        let mut RWVBAS: i32 = 0;
        let mut SELCOL = StackArray::<i32, 50>::new(1..=MAXSEL);
        let mut SELCTP = StackArray::<i32, 50>::new(1..=MAXSEL);
        let mut SELTAB = StackArray::<i32, 50>::new(1..=MAXSEL);
        let mut SEG: i32 = 0;
        let mut SEGDSC = StackArray::<i32, 24>::new(1..=SDSCSZ);
        let mut SEGVEC = StackArray::<i32, 10>::new(1..=MAXTAB);
        let mut SENSE = StackArray::<i32, 10>::new(1..=MAXORD);
        let mut SGVBAS: i32 = 0;
        let mut SIZES = ActualArray::<i32>::new(1..=MAXCON);
        let mut STNEW: i32 = 0;
        let mut TAB: i32 = 0;
        let mut TABIDX: i32 = 0;
        let mut TBCURR: i32 = 0;
        let mut TOP: i32 = 0;
        let mut TPTVEC = StackArray::<i32, 16>::new(LBCELL..=MAXTAB);
        let mut UBASE = StackArray::<i32, 200>::new(1..=MXJRS);
        let mut UNROWS: i32 = 0;
        let mut USIZE: i32 = 0;
        let mut ACTIVC = ActualArray::<bool>::new(1..=MAXCON);
        let mut ACTIVV = ActualArray::<bool>::new(1..=MAXCON);
        let mut ATTMCH: bool = false;
        let mut CMTCH: bool = false;
        let mut DOSORT: bool = false;
        let mut FIRST: bool = false;
        let mut FND: bool = false;
        let mut INDEXD: bool = false;
        let mut KEYFND: bool = false;
        let mut NULSOK: bool = false;
        let mut PRESNT: bool = false;
        let mut SORTED: bool = false;
        let mut VMTCH: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(1), MAXCON as usize))
                .chain([]);

            LELTS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(1), MAXORD as usize))
                .chain([]);

            OELTS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(1), MAXCON as usize))
                .chain([]);

            RELTS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"CHR"),
                Val::C(b"DP"),
                Val::C(b"INT"),
                Val::C(b"TIME"),
            ]
            .into_iter();
            CHTYPE
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        FTHEAD = 0;
        TBHEAD = 0;
        FIRST = true;

        Self {
            FTPOOL,
            FTHAN,
            FTHEAD,
            TBPOOL,
            TBSTPT,
            TBNCOL,
            TBNAMS,
            TBCTPT,
            TBFILS,
            TBFLSZ,
            TBHEAD,
            STPOOL,
            STHAN,
            STSIDX,
            STDSCS,
            STNROW,
            STNCOL,
            STDTPT,
            DTPOOL,
            DTDSCS,
            CTPOOL,
            CTNAMS,
            CTCLAS,
            CTTYPS,
            CTLENS,
            CTFIXD,
            CTSIZS,
            CTINDX,
            CTNULL,
            CHTYPE,
            CNAMS,
            CNMSET,
            COLNAM,
            FRMALS,
            FRMTAB,
            LCNAME,
            LTNAME,
            PROBLM,
            RCNAME,
            RTNAME,
            STATE,
            TABNAM,
            TABVEC,
            DVALS,
            BEGIDX,
            CBEGS,
            CDSCRS,
            CENDS,
            CJBEG,
            CJEND,
            CJROWS,
            CJSIZE,
            CNSTYP,
            COL,
            COLPTR,
            CVLEN,
            CTNEW,
            DELSEG,
            DTNEW,
            DTYPE,
            ENDIDX,
            I,
            IVALS,
            J,
            JBASE1,
            JBASE2,
            JSIZE,
            K,
            KEY,
            KEYDSC,
            L,
            LCIDX,
            LELTS,
            LDSCRS,
            LTBIDX,
            LXBEG,
            LXEND,
            NACT,
            NCOLS,
            NCONJ,
            NEW,
            NEXT,
            NMATCH,
            NORDER,
            NPCOL,
            NSEG,
            NSEL,
            NSV,
            NTAB,
            OCOLS,
            OELTS,
            OPS,
            ORDBAS,
            OTABS,
            PTROFF,
            R,
            RBAS,
            RCIDX,
            RDSCRS,
            RELTS,
            RESBAS,
            ROWIDX,
            ROWVEC,
            RSIZE,
            RTBIDX,
            RTOTAL,
            RWVBAS,
            SELCOL,
            SELCTP,
            SELTAB,
            SEG,
            SEGDSC,
            SEGVEC,
            SENSE,
            SGVBAS,
            SIZES,
            STNEW,
            TAB,
            TABIDX,
            TBCURR,
            TOP,
            TPTVEC,
            UBASE,
            UNROWS,
            USIZE,
            ACTIVC,
            ACTIVV,
            ATTMCH,
            CMTCH,
            DOSORT,
            FIRST,
            FND,
            INDEXD,
            KEYFND,
            NULSOK,
            PRESNT,
            SORTED,
            VMTCH,
        }
    }
}

/// EK, query manager
///
/// Manage query operations on EK files.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  ENTRY POINTS
///  --------  ---  --------------------------------------------------
///  CINDEX     I   EKCII
///  ELMENT     I   EKGC, EKGD, EKGI
///  EQRYC      I   EKSRCH
///  EQRYD      I   EKSRCH
///  EQRYI      I   EKSRCH
///  FNAME      I   EKLEF
///  ROW        I   EKGC, EKGD, EKGI, EKNELT
///  SELIDX     I   EKGC, EKGD, EKGI, EKNELT
///  COLUMN    I-O  EKCIN, EKGC, EKGD, EKGI, EKNELT, EKCII
///  HANDLE    I-O  EKLEF, EKUEF
///  N         I-O  EKTNAM, EKNTAB
///  TABLE     I-O  EKCCNT, EKCII, EKTNAM
///  ATTDSC     O   EKCII, EKCIN
///  CCOUNT     O   EKCCNT
///  FOUND      O   EKCIN, EKGC, EKGD, EKGI
///  NELT       O   EKNELT
///  NMROWS     O   EKSRCH
///  SEMERR     O   EKSRCH
///  ERRMSG     O   EKSRCH
///  CDATA      O   EKGC
///  DDATA      O   EKGD
///  IDATA      O   EKGI
///  NULL       O   EKGC, EKGD, EKGI
///  FTSIZE     P   All
///  MAXCON     P   All
///  MXCLLD     P   All
///  STSIZE     P   All
///  MAXORD     P   All
///  CNAMSZ     P   All
///  ITSIZE     P   All
/// ```
///
/// # Detailed Input
///
/// ```text
///  See the entry points for descriptions of their inputs.
/// ```
///
/// # Detailed Output
///
/// ```text
///  See the entry points for descriptions of their outputs.
/// ```
///
/// # Parameters
///
/// ```text
///  FTSIZE   is the maximum number of EK files that may be
///           loaded. Any other DAS files loaded by the calling
///           program count against this limit.
///
///  STSIZE   is the size of the segment table; this is the
///           maximum number of segments that can be loaded at
///           one time.
///
///  MXTBLD   is the maximum number of tables that can be loaded
///           at any time. A table can consist of multiple
///           segments.
///
///  MXCLLD   is the maximum number of columns that can be loaded
///           at any time. A column may be spread across
///           multiple segments; in this case, the portions of
///           the column contained in each segment count against
///           this limit.
///
///  ADSCSZ   is the size of column attribute descriptor.
///           (Defined in ekattdsc.inc.)
///
///  LBCELL   is the SPICE cell lower bound.
///
///  Many other parameters are defined in the include files referenced
///  above. See those files for details.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If this routine is called directly, the error
///      SPICE(BOGUSENTRY) is signaled.
///
///  2)  See the headers of the entry points for descriptions of
///      exceptions specific to those routines.
/// ```
///
/// # Files
///
/// ```text
///  This suite of routines reads binary `sequence component' EK files.
///  In order for a binary EK file to be accessible to this routine,
///  the file must be `loaded' via a call to the entry point EKLEF.
///
///  Text format EK files cannot be used by this routine; they must
///  first be converted by binary format by the NAIF Toolkit utility
///  SPACIT.
/// ```
///
/// # Particulars
///
/// ```text
///  EKQMGR is an umbrella routine for its entry points: all variables
///  used by the entry points are declared here.
///
///  EKQMGR supports loading and unloading EK files, executing queries,
///  and fetching the results of executed queries. The entry points
///  and their functions are:
///
///     File loading and unloading:
///
///        EKLEF  ( EK, load event file   )
///        EKUEF  ( EK, unload event file )
///
///     Query execution:
///
///        EKSRCH ( EK, search for events )
///
///     Fetching query results:
///
///        EKGC   ( EK, get event data, character        )
///        EKGD   ( EK, get event data, double precision )
///        EKGI   ( EK, get event data, integer          )
///
///     Utilities:
///
///        EKNTAB ( EK, return the number of loaded tables        )
///        EKTNAM ( EK, return the names of loaded tables         )
///        EKCCNT ( EK, return the column count of a table        )
///        EKCII  ( EK, look up column info by index              )
///        EKNELT ( EK, return number of elements in column entry )
///
///
///  To issue queries to the EK system, users would normally call the
///  high-level interface routine EKFIND. EKFIND parses queries and
///  converts them to the encoded form expected by EKSRCH. It is
///  possible to call EKSRCH directly, but this should not be attempted
///  by others than EK masters. EKFIND is not an entry point of
///  EKQMGR, but instead is a separate subroutine.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Query the EK system and fetch data matching that query.
///
///     The program shown here does not rely on advance
///     knowledge of the input query or the contents of any loaded EK
///     files.
///
///     To simplify the example, we assume that all data are scalar.
///     This assumption relieves us of the need to test the size of
///     column entries before fetching them. In the event that a
///     column contains variable-size array entries, the entry point
///     EKNELT may be called to obtain the size of column entries to
///     be fetched. See EKNELT for an example.
///
///
///     Use the EK kernel below to load the information from the
///     original Supplementary Engineering Data Record (SEDR) data
///     set generated by the Viking Project.
///
///        vo_sedr.bdb
///
///     Use the LSK kernel below to load the leap seconds and time
///     constants required for the conversions.
///
///        naif0012.tls
///
///
///     Example code begins here.
///
///
///           PROGRAM EKQMGR_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include EK Query Limit Parameters
///     C
///           INCLUDE 'ekqlimit.inc'
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               RTRIM
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         EKNAME
///           PARAMETER           ( EKNAME = 'vo_sedr.bdb' )
///
///           CHARACTER*(*)         LSKNAM
///           PARAMETER           ( LSKNAM = 'naif0012.tls' )
///
///           INTEGER               DESCSZ
///           PARAMETER           ( DESCSZ = 31   )
///
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN = 1840 )
///
///           INTEGER               ITEMSZ
///           PARAMETER           ( ITEMSZ = DESCSZ + 4 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 27   )
///
///           INTEGER               TYPLEN
///           PARAMETER           ( TYPLEN = 4    )
///
///           INTEGER               XCLSLN
///           PARAMETER           ( XCLSLN = 4    )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(MAXSTR)    CDATA
///           CHARACTER*(MAXCLN)    COLS   ( MAXSEL )
///           CHARACTER*(ERRLEN)    ERRMSG
///           CHARACTER*(ITEMSZ)    ITEM
///           CHARACTER*(DESCSZ)    OUTSTR
///           CHARACTER*(MAXQRY)    QUERY
///           CHARACTER*(TIMLEN)    UTCSTR
///           CHARACTER*(MAXCLN)    TABS   ( MAXTAB )
///           CHARACTER*(XCLSLN)    XCLASS ( MAXSEL )
///           CHARACTER*(TYPLEN)    XTYPES ( MAXSEL )
///
///           DOUBLE PRECISION      DDATA
///           DOUBLE PRECISION      TDATA
///
///           INTEGER               B
///           INTEGER               COLNO
///           INTEGER               E
///           INTEGER               HANDLE
///           INTEGER               IDATA
///           INTEGER               N
///           INTEGER               NMROWS
///           INTEGER               ROW
///           INTEGER               XBEGS  ( MAXSEL )
///           INTEGER               XENDS  ( MAXSEL )
///
///           LOGICAL               ERROR
///           LOGICAL               FOUND
///           LOGICAL               NULL
///
///     C
///     C     Load leapseconds file for time conversion.
///     C
///           CALL FURNSH ( LSKNAM )
///
///     C
///     C     Load EK.
///     C
///           CALL EKLEF  ( EKNAME, HANDLE )
///
///     C
///     C     Setup the query.  Parse the SELECT clause using
///     C     EKPSEL.
///     C
///           QUERY = 'Select IMAGE_NUMBER, IMAGE_ID, '
///          .   //          'PLATFORM_CLOCK, IMAGE_TIME '
///          .   //   'from VIKING_SEDR_DATA '
///          .   //   'where IMAGE_NUMBER < 25850000 '
///          .   //   'order by IMAGE_NUMBER'
///
///           CALL EKPSEL ( QUERY,  N,    XBEGS, XENDS, XTYPES,
///          .              XCLASS, TABS, COLS,  ERROR, ERRMSG )
///
///           IF ( ERROR ) THEN
///
///              WRITE(*,*) ERRMSG
///
///           ELSE
///
///     C
///     C        Submit query to the EK query system.
///     C
///              CALL EKFIND ( QUERY, NMROWS, ERROR, ERRMSG )
///
///              IF ( ERROR ) THEN
///
///                 WRITE(*,*) ERRMSG
///
///              ELSE
///
///     C
///     C           Fetch the rows that matched the query.
///     C
///                 DO ROW = 1, NMROWS
///
///     C
///     C              Fetch data from the Ith row.
///     C
///                    WRITE (*,*) ' '
///                    WRITE (*,*) 'ROW = ', ROW
///
///                    DO COLNO = 1, N
///
///     C
///     C                 Fetch the data from the Jth selected
///     C                 column.
///     C
///                       IF ( XCLASS(COLNO) .EQ. 'COL' ) THEN
///
///                          OUTSTR  =  COLS(COLNO)
///                          CALL PREFIX ( '.',         0, OUTSTR )
///                          CALL PREFIX ( TABS(COLNO), 0, OUTSTR )
///                          ITEM = '  ' // OUTSTR // ':'
///
///                       ELSE
///
///                          B  =  XBEGS(COLNO)
///                          E  =  XENDS(COLNO)
///                          ITEM = '  ITEM = ' // QUERY(B:E)
///
///                       END IF
///
///                       IF ( XTYPES(COLNO) .EQ. 'CHR' ) THEN
///
///                          CALL EKGC ( COLNO,  ROW,  1,
///          .                           CDATA, NULL, FOUND )
///
///                          IF ( NULL ) THEN
///                             WRITE(*,*) ITEM, '<Null>'
///                          ELSE
///                             WRITE(*,*) ITEM, CDATA(:RTRIM(CDATA))
///                          END IF
///
///
///                       ELSE IF ( XTYPES(COLNO) .EQ. 'DP' ) THEN
///
///                          CALL EKGD ( COLNO,  ROW,  1,
///          .                           DDATA, NULL, FOUND )
///
///                          IF ( NULL ) THEN
///                             WRITE(*,*) ITEM, '<Null>'
///                          ELSE
///                             WRITE(*,*) ITEM, DDATA
///                          END IF
///
///
///                       ELSE IF ( XTYPES(COLNO) .EQ. 'INT' ) THEN
///
///                          CALL EKGI ( COLNO,  ROW,  1,
///          .                           IDATA, NULL, FOUND )
///
///                          IF ( NULL ) THEN
///                             WRITE(*,*) ITEM, '<Null>'
///                          ELSE
///                             WRITE(*,*) ITEM, IDATA
///                          END IF
///
///
///                       ELSE
///     C
///     C                    The item is a time value.  Convert it
///     C                    to UTC for output.
///     C
///                          CALL EKGD ( COLNO,  ROW,  1,
///          .                           TDATA, NULL, FOUND )
///
///                          IF ( NULL ) THEN
///                             WRITE(*,*) ITEM, '<Null>'
///                          ELSE
///                             CALL ET2UTC ( TDATA, 'C', 3, UTCSTR )
///                             WRITE(*,*) ITEM, UTCSTR
///                          END IF
///
///                       END IF
///
///     C
///     C              We're done with the column having index COLNO.
///     C
///                    END DO
///
///     C
///     C           We're done with the row having index ROW.
///     C
///                 END DO
///
///     C
///     C        We either processed the query or had an error.
///     C
///              END IF
///
///     C
///     C     We either parsed the SELECT clause or had an error.
///     C
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      ROW =            1
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25837050
///        VIKING_SEDR_DATA.IMAGE_ID      : 168C09
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    119.88000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 16 16:50:55.925
///
///      ROW =            2
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25837051
///        VIKING_SEDR_DATA.IMAGE_ID      : 168C10
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    119.27000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 16 16:51:00.269
///
///      ROW =            3
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25840344
///        VIKING_SEDR_DATA.IMAGE_ID      : 168C11
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    119.88000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 16 20:56:53.051
///
///      ROW =            4
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25840345
///        VIKING_SEDR_DATA.IMAGE_ID      : 168C12
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    119.27000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 16 20:56:57.395
///
///      ROW =            5
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25843638
///        VIKING_SEDR_DATA.IMAGE_ID      : 169C01
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    119.88000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 17 01:02:50.177
///
///      ROW =            6
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25843639
///        VIKING_SEDR_DATA.IMAGE_ID      : 169C02
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    119.27000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 17 01:02:54.521
///
///      ROW =            7
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25846934
///        VIKING_SEDR_DATA.IMAGE_ID      : 169C03
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    120.14000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 17 05:08:56.263
///
///      ROW =            8
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25846935
///        VIKING_SEDR_DATA.IMAGE_ID      : 169C04
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    119.52000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 17 05:09:00.607
///
///      ROW =            9
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25848026
///        VIKING_SEDR_DATA.IMAGE_ID      : 169C05
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    120.14000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 17 06:30:28.424
///
///      ROW =           10
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25848030
///        VIKING_SEDR_DATA.IMAGE_ID      : 169C09
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    120.14000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 17 06:30:46.174
///
///      ROW =           11
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25848032
///        VIKING_SEDR_DATA.IMAGE_ID      : 169C11
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    120.14000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 17 06:30:55.168
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.2.0, 27-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header of EKQMGR umbrella routine and all its entry
///         points. Removed unnecessary $Revisions section.
///
/// -    SPICELIB Version 2.1.0, 09-FEB-2015 (NJB)
///
///         Now uses ERRHAN to insert DAS file name into
///         long error messages.
///
/// -    SPICELIB Version 2.0.3, 10-FEB-2014 (BVS)
///
///         Added descriptions of ADSCSZ and LBCELL to the $Parameters
///         section of the header.
///
/// -    SPICELIB Version 2.0.2, 22-AUG-2006 (EDW)
///
///         Replaced references to LDPOOL with references
///         to FURNSH.
///
/// -    SPICELIB Version 2.0.1, 22-SEP-2004 (EDW)
///
///         Removed from the header descriptions, all occurrences of the
///         token used to mark the $Procedure section.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (NJB)
///
///         Bug fix: When an already loaded kernel is opened with EKOPR,
///         it now has its link count reset to 1 via a call to EKCLS.
///
/// -    SPICELIB Version 1.3.0, 12-FEB-1999 (NJB)
///
///         Bug fix: in entry point EKNELT, there was a error handling
///         branch that called CHKOUT where CHKIN should have been called.
///         This has been fixed.
///
/// -    SPICELIB Version 1.2.0, 21-JUL-1998 (NJB)
///
///         In the entry point EKSRCH, a ZZEKJSQZ call was added after
///         the ZZEKJOIN call. This change reduces the scratch area usage
///         for intermediate results of joins. It also prevents ZZEKJOIN
///         from being handed a join row set containing a segment vector
///         having no corresponding row vectors.
///
/// -    SPICELIB Version 1.1.0, 07-JUL-1996 (NJB)
///
///         Code fixes were made in routines
///
///            EKNELT, EKGC, EKGD, EKGI
///
///         Version lines were fixed in all routines: versions were
///         changed from "Beta" to "SPICELIB."
///
/// -    SPICELIB Version 1.0.0, 23-OCT-1995 (NJB)
/// ```
pub fn ekqmgr(
    ctx: &mut SpiceContext,
    cindex: i32,
    elment: i32,
    eqryc: &str,
    eqryd: &[f64],
    eqryi: &[i32],
    fname: &str,
    row: i32,
    selidx: i32,
    column: &str,
    handle: i32,
    n: i32,
    table: &str,
    attdsc: &[i32; 6],
    ccount: i32,
    found: bool,
    nelt: i32,
    nmrows: i32,
    semerr: bool,
    errmsg: &str,
    cdata: &str,
    ddata: f64,
    idata: i32,
    null: bool,
) -> crate::Result<()> {
    EKQMGR(
        cindex,
        elment,
        eqryc.as_bytes(),
        eqryd,
        eqryi,
        fname.as_bytes(),
        row,
        selidx,
        column.as_bytes(),
        handle,
        n,
        table.as_bytes(),
        attdsc,
        ccount,
        found,
        nelt,
        nmrows,
        semerr,
        errmsg.as_bytes(),
        cdata.as_bytes(),
        ddata,
        idata,
        null,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKQMGR  ( EK, query manager )
pub fn EKQMGR(
    CINDEX: i32,
    ELMENT: i32,
    EQRYC: &[u8],
    EQRYD: &[f64],
    EQRYI: &[i32],
    FNAME: &[u8],
    ROW: i32,
    SELIDX: i32,
    COLUMN: &[u8],
    HANDLE: i32,
    N: i32,
    TABLE: &[u8],
    ATTDSC: &[i32],
    CCOUNT: i32,
    FOUND: bool,
    NELT: i32,
    NMROWS: i32,
    SEMERR: bool,
    ERRMSG: &[u8],
    CDATA: &[u8],
    DDATA: f64,
    IDATA: i32,
    NULL: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Non-SPICELIB functions
    //

    //
    // Linked list functions:
    //
    //    Find next node
    //    Find tail of list
    //    Return number of free nodes
    //
    //

    //
    // Local parameters
    //

    //
    // Maximum number of constraints allowed in a single query:
    //

    //
    // Miscellaneous parameters
    //

    //
    // Number of data types
    //

    //
    // Length of strings used for data type names.
    //

    //
    // Length of status strings.
    //

    //
    // Local variables
    //

    //
    // As do the CK and SPK `BSR' entry points, the EKQMGR entry points
    // make use of an amusing panoply of tables, linked lists, and
    // pointers.  Here's where they're declared and described.
    //
    //
    // The file table contains a list of handles of loaded EK files.
    // Entries in the table are organized as a doubly linked list.
    // Names of file table variables begin with the string 'FT'.
    //
    //    The maximum number of EK files that can be loaded is FTSIZE.
    //
    //    The linked list pool used to index table entries is called
    //    FTPOOL.
    //
    //    FTHAN is an array containing file handles of loaded EKs.
    //
    //    FTHEAD is the head node of the file list.
    //

    //
    // The table list contains table names, segment table pointers,
    // and column table pointers for every table associated with a
    // loaded segment.  The segment table pointers indicate the head node
    // of the segment list for each table.  The column table pointers
    // indicate the column names and attributes associated with each
    // table.
    //
    // The entries of the table list are organized as a doubly linked
    // list.  All variables in the table list have names starting with
    // the string 'TB'.
    //
    //    MXTBLD is the maximum number of tables that can be
    //    accommodated by the table list.
    //
    //    TBPOOL is the doubly linked list pool used to index the
    //    table list.
    //
    //    TBNAMS is an array of table names.
    //
    //    TBSTPT is an array containing pointers to the heads of segment
    //    lists corresponding to segments belonging to the table.
    //
    //    TBNCOL is the number of columns in each table.
    //
    //    TBCTPT is an array of pointers to lists of column table
    //    entries giving the names and attributes of the columns in each
    //    table.
    //
    //    TBFILS is an array containing, for each table, handles of the
    //    files that contain segments belonging to that table.
    //
    //    TBFLSZ is an array of sizes of handle lists for each table
    //    entry.
    //
    //    TBHEAD is the head node of the table list.
    //
    //

    //
    //
    // The segment table contains descriptive information for each
    // loaded segment.  Entries in the table are indexed by a linked
    // list pool containing a doubly linked list for each system (or
    // instrument) for which segments are loaded.
    //
    // Names of segment table variables begin with the string 'ST'.
    //
    //    The maximum number of segments that can be loaded is MAXSEG.
    //    Currently, the value of MAXSEG is just the size of the segment
    //    table, STSIZE.
    //
    //    The linked list pool used to index segment table entries is
    //    called STPOOL.
    //
    //    For each loaded segment, the following information is stored:
    //
    //       -- The file handle of the EK containing the segment.
    //
    //       -- The index of the segment within the EK that contains it.
    //          Indices start at 1 and end with the segment count for the
    //          EK file.
    //
    //       -- The segment descriptor.
    //
    //       -- The number of rows in the segment.
    //
    //       -- The number of columns in the segment.
    //
    //       -- A pointer to a list of column descriptors.  The
    //          column descriptor table contains a complete descriptor
    //          for every loaded column.
    //
    //
    //

    //
    // The column descriptor table contains a column descriptor for
    // every loaded column.  This table allows segments to share the
    // area used for buffering descriptors, making it reasonable for
    // the buffer space to have room for fewer than
    //
    //    MXCLLD * MAXSEG
    //
    // column descriptors.
    //
    // The space in the table is organized as a doubly linked list.
    //

    //
    // The column attribute table contains attribute information for
    // every column in every loaded segment.  There is one entry per
    // column name; columns with the same names and different data
    // types may not be loaded simultaneously.
    //
    // The entries of the column table are organized as a doubly linked
    // list.  All variables in the column table have names starting with
    // the string 'CT'.
    //
    //    CTSIZE is the maximum number of distinct column declarations
    //    that can be accommodated by the column table.
    //
    //    CTPOOL is the doubly linked list pool used to index the column
    //    table.
    //
    //    CTNAMS is an array containing column names.
    //
    //    CTCLAS is an array containing column class specifiers.
    //
    //    CTTYPS is an array containing column data types.
    //
    //    CTLENS is an array containing column string length specifiers.
    //
    //    CTFIXD is an array of logical flags indicating whether the
    //    columns they correspond to have fixed size.
    //
    //    CTSIZS is an array of integers indicating the number of array
    //    elements per column entry, for fixed-size columns.
    //
    //    CTINDX is an array of logical flags that indicate whether the
    //    columns they correspond to are indexed.
    //
    //    CTNULL is an array of logical flags that indicate whether the
    //    columns they correspond to may contain null values.
    //
    //

    //
    //
    // Other local variables
    //

    //
    //
    // Saved variables
    //

    //
    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"EKQMGR", ctx)?;
    }

    //
    // Never come here.
    //
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    CHKOUT(b"EKQMGR", ctx)?;
    Ok(())
}

/// EK, load event file
///
/// Load an EK file, making it accessible to the EK readers.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of EK file to load.
///  HANDLE     O   File handle of loaded EK file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of a binary EK file to be loaded.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the handle of the EK file. The file is
///           accessible by the EK reader routines once it
///           has been loaded.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the EK file indicated by FNAME contains a column whose
///      name matches that of a column in an already loaded EK, but
///      whose declared attributes don't match those of the loaded
///      column of the same name, the error SPICE(BADATTRIBUTES) is
///      signaled. HANDLE is is undefined in this case.
///
///  2)  Loading an EK file that is already loaded does not cause side
///      effects. The handle already associated with the file will be
///      returned.
///
///  3)  If a file open error occurs, the error is signaled by a
///      routine in the call tree of this routine. HANDLE is undefined
///      in this case.
///
///  4)  If loading the input file would cause the maximum number of
///      loaded EK files to be exceeded, the error
///      SPICE(EKFILETABLEFULL) is signaled. HANDLE is undefined in
///      this case. This routine will attempt to unload the file
///      from the DAS system.
///
///  5)  If loading the input file would cause the maximum number of
///      loaded DAS files to be exceeded, an error is signaled by a
///      routine in the call tree of this routine. HANDLE is undefined
///      in this case. This routine will attempt to unload the file
///      from the DAS system.
///
///  6)  If loading the input file would cause the maximum number of
///      segments allowed in loaded EK files to be exceeded, the error
///      SPICE(EKSEGMENTTABLEFULL) is signaled. HANDLE is undefined in
///      this case. This routine will attempt to unload the file from
///      the DAS system.
///
///  7)  If loading the input file would cause the maximum number of
///      columns allowed in loaded EK files to be exceeded, the error
///      SPICE(EKCOLDESCTABLEFULL) is signaled. HANDLE is undefined in
///      this case. This routine will attempt to unload the file from
///      the DAS system.
///
///  8)  If loading the input file would cause the maximum allowed
///      number of columns having distinct attributes in loaded EK
///      files to be exceeded, the error SPICE(EKCOLATTRTABLEFULL) is
///      signaled. HANDLE is undefined in this case. This routine will
///      attempt to unload the file from the DAS system.
///
///  9)  If loading the input file would cause the maximum number of
///      instrument codes allowed in loaded EK files to be exceeded,
///      the error SPICE(EKIDTABLEFULL) is signaled. HANDLE is
///      undefined in this case. This routine will attempt to unload
///      the file from the DAS system.
///
///  10) If the input file does not contain at least one segment, the
///      error SPICE(EKNOSEGMENTS) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See description of FNAME in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine makes EK files known to the EK system. It is
///  necessary to load EK files using this routine in order to
///  query the files using the EK readers.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Load two EK files and perform a query on them. During query
///     execution, all files will be searched.
///
///     Use the EK kernel below to load the Cassini Science Plan
///     SPICE E-Kernel File based upon the integrated science
///     plan #78.
///
///        S78_CIMSSSUPa.bep
///
///     Use the EK kernel below to load the data based upon the
///     integrated science plan #79.
///
///        S79_CIMSSSUPa.bep
///
///
///     Example code begins here.
///
///
///           PROGRAM EKLEF_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Maximum length of an input query,
///     C     MAXQRY.
///     C
///           INCLUDE 'ekqlimit.inc'
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               EKNMLN
///           PARAMETER           ( EKNMLN = 17 )
///
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN = 1840 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(EKNMLN)    EKNAMS ( 2 )
///           CHARACTER*(ERRLEN)    ERRMSG
///           CHARACTER*(MAXQRY)    QUERY
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               N
///           INTEGER               NMROWS
///
///           LOGICAL               ERROR
///
///     C
///     C     Set up the array holding the EK file names.
///     C
///           DATA                  EKNAMS / 'S78_CIMSSSUPa.bep',
///          .                               'S79_CIMSSSUPa.bep'  /
///
///     C
///     C     Load the EK files. This call could be replaced by a call
///     C     to FURNSH (in this case, a meta-kernel listing the EKs
///     C     to be loaded could also be used).
///     C
///           DO I = 1, 2
///
///              CALL EKLEF ( EKNAMS(I), HANDLE )
///              WRITE(*,'(2A)') 'Loading EK: ', EKNAMS(I)
///
///           END DO
///
///     C
///     C     The EK files contain a table 'CASSINI_SP_OBSERVATION',
///     C     that contains columns named:
///     C
///     C        NOTES, OBSERVATION_ID, OBSERVATION_TITLE,
///     C        OBS_DESCRIPTION, SCIENCE_OBJECTIVE, SEQUENCE,
///     C        SUBSYSTEM
///     C
///     C     Define a set of constraints to perform a query on all
///     C     loaded EK files (the SELECT clause).
///     C
///           QUERY = 'Select SUBSYSTEM, SCIENCE_OBJECTIVE, '
///          .   //   'OBSERVATION_ID from CASSINI_SP_OBSERVATION '
///          .   //   'order by SUBSYSTEM'
///
///     C
///     C     Query the EK system for data rows matching the
///     C     SELECT constraints.
///     C
///           CALL EKFIND ( QUERY, NMROWS, ERROR, ERRMSG )
///
///     C
///     C     Check whether an error occurred while processing the
///     C     SELECT clause. If so, output the error message.
///     C
///           WRITE(*,*)
///           IF ( ERROR ) THEN
///
///              WRITE(*,'(2A)') 'SELECT clause error: ', ERRMSG
///
///           ELSE
///
///     C
///     C        If no error, NMROWS contains the number of rows
///     C        matching the constraints specified in the query
///     C        string.
///     C
///              WRITE(*,'(A,I3)') 'Number of matching rows: ', NMROWS
///
///           END IF
///
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Loading EK: S78_CIMSSSUPa.bep
///     Loading EK: S79_CIMSSSUPa.bep
///
///     Number of matching rows:   9
///
///
///  2) Repeat the previous exercise, using the same input kernels,
///     but this time unloading the previous file before each new
///     file is loaded. Unloading files prevents them from being
///     searched during query execution.
///
///
///     Example code begins here.
///
///
///           PROGRAM EKLEF_EX2
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Maximum length of an input query,
///     C     MAXQRY.
///     C
///           INCLUDE 'ekqlimit.inc'
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               EKNMLN
///           PARAMETER           ( EKNMLN = 17 )
///
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN = 1840 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(EKNMLN)    EKNAMS ( 2 )
///           CHARACTER*(ERRLEN)    ERRMSG
///           CHARACTER*(MAXQRY)    QUERY
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               N
///           INTEGER               NMROWS
///
///           LOGICAL               ERROR
///
///     C
///     C     Set up the array holding the EK file names.
///     C
///           DATA                  EKNAMS / 'S78_CIMSSSUPa.bep',
///          .                               'S79_CIMSSSUPa.bep'  /
///
///     C
///     C     The EK files contain a table 'CASSINI_SP_OBSERVATION',
///     C     that contains columns named:
///     C
///     C        NOTES, OBSERVATION_ID, OBSERVATION_TITLE,
///     C        OBS_DESCRIPTION, SCIENCE_OBJECTIVE, SEQUENCE,
///     C        SUBSYSTEM
///     C
///     C     Define a set of constraints to perform a query on all
///     C     loaded EK files (the SELECT clause).
///     C
///           QUERY = 'Select SUBSYSTEM, SCIENCE_OBJECTIVE, '
///          .   //   'OBSERVATION_ID from CASSINI_SP_OBSERVATION '
///          .   //   'order by SUBSYSTEM'
///
///     C
///     C     Load the EK files. This call could be replaced by a call
///     C     to FURNSH.
///     C
///           DO I = 1, 2
///
///              CALL EKLEF ( EKNAMS(I), HANDLE )
///              WRITE(*,'(2A)') 'Loading EK: ', EKNAMS(I)
///
///     C
///     C        Query the EK system for data rows matching the
///     C        SELECT constraints.
///     C
///              CALL EKFIND ( QUERY, NMROWS, ERROR, ERRMSG )
///
///     C
///     C        Check whether an error occurred while processing the
///     C        SELECT clause. If so, output the error message.
///     C
///              IF ( ERROR ) THEN
///
///                 WRITE(*,'(2A)') 'SELECT clause error: ', ERRMSG
///
///              ELSE
///
///     C
///     C           If no error, NMROWS contains the number of rows
///     C           matching the constraints specified in the query
///     C           string.
///     C
///                 WRITE(*,'(A,I3)') 'Number of matching rows: ',
///          .                        NMROWS
///
///              END IF
///
///     C
///     C        Unload the current file. Unloading files prevents
///     C        them from being searched during query execution.
///     C
///              CALL EKUEF ( HANDLE )
///              WRITE(*,'(2A)') 'Unloading EK: ', EKNAMS(I)
///              WRITE(*,*)
///
///           END DO
///
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Loading EK: S78_CIMSSSUPa.bep
///     Number of matching rows:   4
///     Unloading EK: S78_CIMSSSUPa.bep
///
///     Loading EK: S79_CIMSSSUPa.bep
///     Number of matching rows:   5
///     Unloading EK: S79_CIMSSSUPa.bep
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  EK files containing columns having the same name but
///      inconsistent declarations are not diagnosed. Such kernels
///      are invalid in any case.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.2.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code examples based on existing fragments.
///
/// -    SPICELIB Version 2.1.0, 09-FEB-2015 (NJB)
///
///         Now uses ERRHAN to insert DAS file name into
///         long error messages.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (NJB)
///
///         Bug fix: When an already loaded kernel is opened with EKOPR,
///         it now has its link count reset to 1 via a call to EKCLS.
///
/// -    SPICELIB Version 1.0.1, 07-JUL-1996 (NJB)
///
///         Previous version line was changed from "Beta" to "SPICELIB."
///
/// -    SPICELIB Version 1.0.0, 23-OCT-1995 (NJB)
/// ```
pub fn eklef(ctx: &mut SpiceContext, fname: &str, handle: &mut i32) -> crate::Result<()> {
    EKLEF(fname.as_bytes(), handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKLEF ( EK, load event file )
pub fn EKLEF(FNAME: &[u8], HANDLE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"EKLEF", ctx)?;
    }

    //
    // Here's a brief overview of what follows:
    //
    //    -- We do some once-per-program run initializations.
    //
    //    -- We do some simple error checks.  We need to make sure
    //       that DAS can load the file, and that the EK architecture is
    //       the right kind.
    //
    //    -- We need to make sure that there's enough space in our
    //       data structures to hold the information about the new
    //       EK.  Some of these checks are simple; we do these first.
    //       However, checking that we have enough room in the column
    //       table is best done by simply loading the column data into
    //       the table.  If we run out of room, we abort the load.
    //
    //    -- We also need to make sure that the column attributes for
    //       any two columns with the same name in the same table are
    //       identical.  This is easy to do if the attributes for every
    //       column we've encountered have been loaded into the column
    //       table.
    //
    //    -- We save the table name and column names and attributes for
    //       each new table we encounter.  For each table, we maintain a
    //       list of handles of files that contain segments in that
    //       table.
    //
    //    -- We make a segment table entry for each segment we find.
    //
    //    -- We save the column descriptor for each column we find,
    //       associating it with the segment table entry for the segment
    //       containing the column.  The column descriptor entries are
    //       linked together in the same order that the corresponding
    //       column names appear in the parent table's column name list;
    //       this order is not necessarily the order that the columns
    //       have within the segment.
    //
    //    -- We maintain a list of handles of loaded EKs.
    //
    //    If we run out of room in the column table, we clean up our
    //    mess.  This means removing the current file's contributions
    //    to the column table, segment table, file table, and if
    //    necessary, the table list.
    //
    //
    // On the first pass through this routine, initialize the tables,
    // if it hasn't been done yet.
    //
    if save.FIRST {
        //
        // Initialize the file table pool, segment table pool, column
        // descriptor pool, column table pool, and table list pool.
        //
        LNKINI(FTSIZE, save.FTPOOL.as_slice_mut(), ctx)?;
        LNKINI(STSIZE, save.STPOOL.as_slice_mut(), ctx)?;
        LNKINI(DTSIZE, save.DTPOOL.as_slice_mut(), ctx)?;
        LNKINI(CTSIZE, save.CTPOOL.as_slice_mut(), ctx)?;
        LNKINI(MXTBLD, save.TBPOOL.as_slice_mut(), ctx)?;

        save.FTHEAD = 0;
        save.TBHEAD = 0;

        save.FIRST = false;
    }

    //
    // Open the EK file for read access.  Bail out now if this doesn't
    // work.  This retreat will protect the various tables from
    // corruption.
    //
    EKOPR(FNAME, HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"EKLEF", ctx)?;
        return Ok(());
    }

    //
    // Check to see whether the named EK has already been loaded.
    // If so, we've added another link to the EK, which must be
    // removed.
    //
    save.I = save.FTHEAD;

    while (save.I > 0) {
        if (*HANDLE == save.FTHAN[save.I]) {
            //
            // The last call we made to EKOPR added another link to
            // the EK file.  Remove this link.
            //
            DASCLS(*HANDLE, ctx)?;

            CHKOUT(b"EKLEF", ctx)?;
            return Ok(());
        }

        save.I = LNKNXT(save.I, save.FTPOOL.as_slice(), ctx)?;
    }

    //
    // Nothing doing unless the architecture is correct.  This file
    // should be a paged DAS EK.
    //
    ZZEKPGCH(*HANDLE, b"READ", ctx)?;

    //
    // Before getting too involved with this new EK file, let's check it
    // out.  We must have enough room to accommodate it in the file
    // table, segment table, table list, and column table.
    //
    // Make sure there's enough room in the file table.
    //

    if (LNKNFN(save.FTPOOL.as_slice()) == 0) {
        //
        // Sorry, there are no free file table entries left.
        //
        // We close the EK AFTER setting the long error message.
        //
        SETMSG(b"The EK file # could not be loaded; the maximum number of loaded EKs has already been reached.", ctx);
        ERRHAN(b"#", *HANDLE, ctx)?;
        EKCLS(*HANDLE, ctx)?;
        SIGERR(b"SPICE(EKFILETABLEFULL)", ctx)?;
        CHKOUT(b"EKLEF", ctx)?;
        return Ok(());
    }

    //
    // Find out how many segments are in the new kernel, and make
    // sure there's enough room in the segment table.
    //
    save.NSEG = EKNSEG(*HANDLE, ctx)?;

    if (save.NSEG > LNKNFN(save.STPOOL.as_slice())) {
        //
        // There are too many segments for the amount of space we've got
        // left.
        //
        // We close the EK AFTER setting the long error message.
        //
        SETMSG(b"The EK file # could not be loaded; the maximum number of loaded segments has already been reached.", ctx);
        ERRHAN(b"#", *HANDLE, ctx)?;
        EKCLS(*HANDLE, ctx)?;
        SIGERR(b"SPICE(EKSEGTABLEFULL)", ctx)?;
        CHKOUT(b"EKLEF", ctx)?;
        return Ok(());
    } else if (save.NSEG < 1) {
        SETMSG(b"The EK file # contains no segments.", ctx);
        ERRHAN(b"#", *HANDLE, ctx)?;
        EKCLS(*HANDLE, ctx)?;
        SIGERR(b"SPICE(EKNOSEGMENTS)", ctx)?;
        CHKOUT(b"EKLEF", ctx)?;
        return Ok(());
    }

    //
    // At this point, the file has insinuated itself into our confidence,
    // justified or not.  We'll attempt to load the segment and column
    // tables, and we'll update the table list if new tables are
    // introduced in this file.
    //
    save.SEG = 1;
    fstr::assign(&mut save.STATE, b"LOAD_FILE_TABLE");

    while fstr::ne(&save.STATE, b"DONE") {
        if fstr::eq(&save.STATE, b"LOAD_FILE_TABLE") {
            //
            // Allocate a file table entry and link the new entry in before
            // the current head of the list.  Update the list head pointer.
            // Record the file handle in the new file table entry.
            //
            LNKAN(save.FTPOOL.as_slice_mut(), &mut save.NEW, ctx)?;
            LNKILB(save.NEW, save.FTHEAD, save.FTPOOL.as_slice_mut(), ctx)?;

            save.FTHEAD = save.NEW;
            save.FTHAN[save.NEW] = *HANDLE;

            fstr::assign(&mut save.STATE, b"SUMMARIZE_SEGMENT");
        } else if fstr::eq(&save.STATE, b"SUMMARIZE_SEGMENT") {
            //
            // Get the summary information for this segment.
            //
            ZZEKSINF(
                *HANDLE,
                save.SEG,
                &mut save.TABNAM,
                save.SEGDSC.as_slice_mut(),
                save.CNAMS.as_arg_mut(),
                save.CDSCRS.as_slice_mut(),
                ctx,
            )?;

            save.NCOLS = save.SEGDSC[NCIDX];

            //
            // Before going further, check the segment for duplicate
            // column names.  Bail out if we find any.
            //
            SSIZEC(MXCLLD, save.CNMSET.as_arg_mut(), ctx)?;
            MOVEC(save.CNAMS.as_arg(), save.NCOLS, save.CNMSET.subarray_mut(1));
            VALIDC(MXCLLD, save.NCOLS, save.CNMSET.as_arg_mut(), ctx)?;

            if (CARDC(save.CNMSET.as_arg(), ctx)? < save.NCOLS) {
                fstr::assign(&mut save.STATE, b"ABORT");
                fstr::assign(&mut save.PROBLM, b"DUPLICATE_COLUMN_NAMES");
            } else {
                fstr::assign(&mut save.STATE, b"FIND_TABLE");
            }
        } else if fstr::eq(&save.STATE, b"FIND_TABLE") {
            //
            // Traverse the table list, checking for a match.
            //
            save.TBCURR = save.TBHEAD;
            save.PRESNT = false;

            while ((save.TBCURR > 0) && !save.PRESNT) {
                if fstr::eq(&save.TABNAM, save.TBNAMS.get(save.TBCURR)) {
                    save.PRESNT = true;
                } else {
                    save.TBCURR = LNKNXT(save.TBCURR, save.TBPOOL.as_slice(), ctx)?;
                }
            }

            //
            // If TABNAM is the name of a table we know about, go on to
            // fill out the segment list entry for the current segment.
            // If we didn't find TABNAM, we have a new table.  Make a table
            // list entry for it.
            //
            if save.PRESNT {
                //
                // Before going further, make sure the number of columns
                // in the segment matches the number of columns in the
                // parent table.
                //
                if (save.NCOLS != save.TBNCOL[save.TBCURR]) {
                    save.NPCOL = save.TBNCOL[save.TBCURR];
                    fstr::assign(&mut save.STATE, b"ABORT");
                    fstr::assign(&mut save.PROBLM, b"COLUMN_NUMBER_MISMATCH");
                } else {
                    //
                    // Add the current file to the list of files containing
                    // the current table.
                    //
                    save.TBFILS[[1, save.TBCURR]] = *HANDLE;
                    save.TBFLSZ[save.TBCURR] = (save.TBFLSZ[save.TBCURR] + 1);

                    fstr::assign(&mut save.STATE, b"MAKE_SEGMENT_TABLE_ENTRY");
                }
            } else {
                //
                // This segment belongs to a new table.
                //
                fstr::assign(&mut save.STATE, b"MAKE_TABLE_LIST_ENTRY");
            }
        } else if fstr::eq(&save.STATE, b"MAKE_TABLE_LIST_ENTRY") {
            //
            // Allocate a table list entry, if we can.
            //

            if (LNKNFN(save.TBPOOL.as_slice()) == 0) {
                //
                // Oops, we're out of room.
                //
                fstr::assign(&mut save.STATE, b"ABORT");
                fstr::assign(&mut save.PROBLM, b"TABLE_LIST_FULL");
            } else {
                //
                // We have an entry; link it to the tail of the table list.
                // For consistency with the case in which the table entry
                // already exists, we'll call the table list node TBCURR.
                //
                // If this is the first table in the table list, set the
                // table head pointer.
                //
                LNKAN(save.TBPOOL.as_slice_mut(), &mut save.TBCURR, ctx)?;

                if (save.TBHEAD <= 0) {
                    save.TBHEAD = save.TBCURR;
                } else {
                    LNKILB(save.TBHEAD, save.TBCURR, save.TBPOOL.as_slice_mut(), ctx)?;
                }

                //
                // Fill in the table name.
                //
                fstr::assign(save.TBNAMS.get_mut(save.TBCURR), &save.TABNAM);
                //
                // Since this table is new, the file list for this table
                // contains only the handle of the current EK.
                //
                save.TBFILS[[1, save.TBCURR]] = *HANDLE;
                save.TBFLSZ[save.TBCURR] = 1;

                //
                // Initialize the column count, column table pointer, and
                // segment list pointer for this table.
                //
                save.TBNCOL[save.TBCURR] = save.NCOLS;
                save.TBCTPT[save.TBCURR] = 0;
                save.TBSTPT[save.TBCURR] = 0;

                //
                // Go on to add a segment table entry for the current
                // segment.
                //
                fstr::assign(&mut save.STATE, b"MAKE_SEGMENT_TABLE_ENTRY");
            }
        } else if fstr::eq(&save.STATE, b"MAKE_SEGMENT_TABLE_ENTRY") {
            //
            // Add the data for the current segment to the segment
            // table.
            //
            // Allocate a segment table entry.  We've already verified
            // that there's enough room.
            //
            LNKAN(save.STPOOL.as_slice_mut(), &mut save.STNEW, ctx)?;

            //
            // Link this segment table entry to the tail of the segment
            // list for the parent table, or, if the tail is NIL, just set
            // the segment list pointer to the current segment node.
            //
            if (save.TBSTPT[save.TBCURR] <= 0) {
                save.TBSTPT[save.TBCURR] = save.STNEW;
            } else {
                LNKILB(
                    save.TBSTPT[save.TBCURR],
                    save.STNEW,
                    save.STPOOL.as_slice_mut(),
                    ctx,
                )?;
            }

            //
            // At this point, we can fill in all elements of the segment
            // table entry except for the pointers into the column table
            // and the column base addresses.
            //
            save.STHAN[save.STNEW] = *HANDLE;
            save.STSIDX[save.STNEW] = save.SEG;
            save.STNROW[save.STNEW] = save.SEGDSC[NRIDX];
            save.STNCOL[save.STNEW] = save.SEGDSC[NCIDX];
            save.STDTPT[save.STNEW] = 0;

            MOVEI(
                save.SEGDSC.as_slice(),
                SDSCSZ,
                save.STDSCS.subarray_mut([1, save.STNEW]),
            );

            //
            // The next step is to set up the column attributes and
            // descriptors.
            //
            fstr::assign(&mut save.STATE, b"MAKE_COLUMN_TABLE_ENTRIES");
        } else if fstr::eq(&save.STATE, b"MAKE_COLUMN_TABLE_ENTRIES") {
            if save.PRESNT {
                //
                // If the current table was present before loading the
                // current segment, we must make sure that the attributes
                // of the columns in this segment match those of the table
                // to which the segment belongs.
                //
                // We must load the column descriptors for this segment
                // in the *same order* as those for every other segment
                // in the table.  This order matches that of the columns
                // in the column attribute list for the table.
                //
                // For each column in the column list of the current table,
                // check the list of column names for the current segment,
                // looking for a match.
                //
                save.J = save.TBCTPT[save.TBCURR];

                while ((save.J > 0) && fstr::ne(&save.STATE, b"ABORT")) {
                    save.K = ISRCHC(&save.CTNAMS[save.J], save.NCOLS, save.CNAMS.as_arg());

                    if (save.K > 0) {
                        //
                        // We have a name match.  At this point, we must
                        // check that the column's other attributes---data
                        // type, size, and whether the column is
                        // indexed---match as well.  It's an error if they
                        // don't.
                        //
                        save.INDEXD = (save.CDSCRS[[IXTIDX, save.K]] != IFALSE);
                        save.NULSOK = (save.CDSCRS[[NFLIDX, save.K]] != IFALSE);
                        save.ATTMCH = ((((((save.CDSCRS[[CLSIDX, save.K]]
                            == save.CTCLAS[save.J])
                            && (save.CDSCRS[[TYPIDX, save.K]] == save.CTTYPS[save.J]))
                            && (save.CDSCRS[[LENIDX, save.K]] == save.CTLENS[save.J]))
                            && (save.CDSCRS[[SIZIDX, save.K]] == save.CTSIZS[save.J]))
                            && (save.INDEXD == save.CTINDX[save.J]))
                            && (save.NULSOK == save.CTNULL[save.J]));

                        if save.ATTMCH {
                            //
                            // Great, the attributes match.  Actually, the
                            // addition of the current segment can *change*
                            // one attribute of the current table:  the
                            // maximum non-blank width associated with the
                            // current column, if the column has character
                            // type.  We'll make this change after we're
                            // sure we won't have to undo it.
                            //
                            // Store the column descriptor for this column
                            // in the descriptor table.  We'll need to
                            // allocate a descriptor table entry first.
                            //
                            if (LNKNFN(save.DTPOOL.as_slice()) == 0) {
                                //
                                // No free nodes left in the descriptor table.
                                //
                                fstr::assign(&mut save.STATE, b"ABORT");
                                fstr::assign(&mut save.PROBLM, b"DESCRIPTOR_TABLE_FULL");
                            } else {
                                //
                                // A free node is available.  Link it in
                                // at the tail of the descriptor list for
                                // the current segment.
                                //
                                LNKAN(save.DTPOOL.as_slice_mut(), &mut save.DTNEW, ctx)?;

                                if (save.STDTPT[save.STNEW] <= 0) {
                                    save.STDTPT[save.STNEW] = save.DTNEW;
                                } else {
                                    LNKILB(
                                        save.STDTPT[save.STNEW],
                                        save.DTNEW,
                                        save.DTPOOL.as_slice_mut(),
                                        ctx,
                                    )?;
                                }

                                //
                                // Fill in the descriptor.
                                //
                                MOVEI(
                                    save.CDSCRS.subarray([1, save.K]),
                                    CDSCSZ,
                                    save.DTDSCS.subarray_mut([1, save.DTNEW]),
                                );
                            }
                        //
                        // We filled in a descriptor table entry, or
                        // else we ran out of room.
                        //
                        } else {
                            //
                            // Seriously bad news.  Someone's tried to
                            // load an EK containing a column with
                            // attributes that conflict with those of a
                            // loaded column of the same name in the
                            // current table.
                            //
                            fstr::assign(&mut save.COLNAM, save.CTNAMS.get(save.J));
                            fstr::assign(&mut save.STATE, b"ABORT");
                            fstr::assign(&mut save.PROBLM, b"MISMATCHED_COLUMN_ATTRIBUTES");
                        }
                    } else {
                        //
                        // No name match; the current column from the current
                        // table is not present in the segment we're looking
                        // at.
                        //
                        fstr::assign(&mut save.COLNAM, save.CTNAMS.get(save.J));
                        fstr::assign(&mut save.STATE, b"ABORT");
                        fstr::assign(&mut save.PROBLM, b"MISSING_COLUMN");
                    }
                    //
                    // The current column matched one in the column list
                    // for the current table, or else we have a problem.
                    //
                    // Advance to the next column in the table's column list.
                    //
                    if fstr::ne(&save.STATE, b"ABORT") {
                        save.J = LNKNXT(save.J, save.CTPOOL.as_slice(), ctx)?;
                    }
                }
            //
            // We've made descriptor table entries for each column in
            // the current segment, or else we have an error.
            //
            } else {
                //
                // We need to set up the column attribute entries for
                // the new table introduced by loading this segment.  We
                // also need to set up descriptor table entries for the
                // segment.  We *don't* have to check the consistency of
                // the attributes of the columns.
                //
                save.K = 1;

                while ((save.K <= save.NCOLS) && fstr::ne(&save.STATE, b"ABORT")) {
                    //
                    // Allocate a new entry in the column attribute table and
                    // link it to the tail of the column list for the
                    // current table.  If the column list is empty, update
                    // the list head.
                    //
                    if (LNKNFN(save.CTPOOL.as_slice()) == 0) {
                        //
                        // There's no more space to store attribute
                        // descriptors.
                        //
                        fstr::assign(&mut save.STATE, b"ABORT");
                        fstr::assign(&mut save.PROBLM, b"ATTRIBUTE_TABLE_FULL");
                    } else {
                        LNKAN(save.CTPOOL.as_slice_mut(), &mut save.CTNEW, ctx)?;

                        if (save.TBCTPT[save.TBCURR] <= 0) {
                            save.TBCTPT[save.TBCURR] = save.CTNEW;
                        } else {
                            LNKILB(
                                save.TBCTPT[save.TBCURR],
                                save.CTNEW,
                                save.CTPOOL.as_slice_mut(),
                                ctx,
                            )?;
                        }

                        //
                        // Fill in the new column attribute entry with the
                        // attributes for this column.
                        //
                        fstr::assign(save.CTNAMS.get_mut(save.CTNEW), save.CNAMS.get(save.K));
                        save.CTCLAS[save.CTNEW] = save.CDSCRS[[CLSIDX, save.K]];
                        save.CTTYPS[save.CTNEW] = save.CDSCRS[[TYPIDX, save.K]];
                        save.CTLENS[save.CTNEW] = save.CDSCRS[[LENIDX, save.K]];
                        save.CTSIZS[save.CTNEW] = save.CDSCRS[[SIZIDX, save.K]];
                        save.CTINDX[save.CTNEW] = (save.CDSCRS[[IXTIDX, save.K]] != IFALSE);
                        save.CTFIXD[save.CTNEW] = (save.CDSCRS[[SIZIDX, save.K]] != IFALSE);
                        save.CTNULL[save.CTNEW] = (save.CDSCRS[[NFLIDX, save.K]] != IFALSE);

                        //
                        // Store the column descriptor for this column
                        // in the descriptor table.  We'll need to
                        // allocate a descriptor table entry first.
                        //
                        if (LNKNFN(save.DTPOOL.as_slice()) == 0) {
                            //
                            // No free nodes left in the descriptor table.
                            //
                            fstr::assign(&mut save.STATE, b"ABORT");
                            fstr::assign(&mut save.PROBLM, b"DESCRIPTOR_TABLE_FULL");
                        } else {
                            //
                            // A free node is available.  Link it in at the
                            // tail of the descriptor list for the current
                            // segment.
                            //
                            LNKAN(save.DTPOOL.as_slice_mut(), &mut save.DTNEW, ctx)?;

                            if (save.STDTPT[save.STNEW] <= 0) {
                                save.STDTPT[save.STNEW] = save.DTNEW;
                            } else {
                                LNKILB(
                                    save.STDTPT[save.STNEW],
                                    save.DTNEW,
                                    save.DTPOOL.as_slice_mut(),
                                    ctx,
                                )?;
                            }

                            //
                            // Fill in the descriptor.
                            //
                            MOVEI(
                                save.CDSCRS.subarray([1, save.K]),
                                CDSCSZ,
                                save.DTDSCS.subarray_mut([1, save.DTNEW]),
                            );
                        }
                    }
                    //
                    // We created attribute and descriptor entries for the
                    // current column, or we encountered an error.
                    //
                    if fstr::ne(&save.STATE, b"ABORT") {
                        //
                        // Consider the next column.
                        //
                        save.K = (save.K + 1);
                    }
                }
                //
                // We created attribute and descriptor entries for every
                // column in the current segment, or we encountered an
                // error.
                //
            }
            //
            // We've processed the current segment in the new file, or
            // else we have an error condition.
            //

            if fstr::ne(&save.STATE, b"ABORT") {
                //
                // We're ready to look at the next segment in the new file.
                //
                fstr::assign(&mut save.STATE, b"NEXT_SEGMENT");
            }
        } else if fstr::eq(&save.STATE, b"NEXT_SEGMENT") {
            if (save.SEG < save.NSEG) {
                save.SEG = (save.SEG + 1);
                fstr::assign(&mut save.STATE, b"SUMMARIZE_SEGMENT");
            } else {
                //
                // We're done with all of the segments.
                //
                fstr::assign(&mut save.STATE, b"DONE");
            }
        } else if fstr::eq(&save.STATE, b"ABORT") {
            //
            // We must clean up all the data structure additions we made to
            // accommodate the new file.
            //
            // Basically, we unload the new file.  We defer the call to
            // EKCLS until after we've reported the error.
            //
            // The file table is first.  The file is at the head of the
            // list.  If the file has a successor, that file is now at the
            // head of the list.
            //

            save.FTHEAD = LNKNXT(save.NEW, save.FTPOOL.as_slice(), ctx)?;

            if (save.FTHEAD < 0) {
                //
                // There are no files left.  Clean up the whole shebang.
                //
                LNKINI(FTSIZE, save.FTPOOL.as_slice_mut(), ctx)?;
                LNKINI(STSIZE, save.STPOOL.as_slice_mut(), ctx)?;
                LNKINI(DTSIZE, save.DTPOOL.as_slice_mut(), ctx)?;
                LNKINI(CTSIZE, save.CTPOOL.as_slice_mut(), ctx)?;
                LNKINI(MXTBLD, save.TBPOOL.as_slice_mut(), ctx)?;

                save.FTHEAD = 0;
                save.TBHEAD = 0;
            } else {
                //
                // If we arrived here, the file we're unloading is not the
                // only loaded file.
                //
                // Free the file table entry for the file.  The entry can be
                // regarded as a sublist that starts and ends with the Ith
                // node, so we can call the `free sublist' routine to get
                // rid of it.
                //
                LNKFSL(save.NEW, save.NEW, save.FTPOOL.as_slice_mut(), ctx)?;

                //
                // It's time to clean up the table list, segment table,
                // column attribute table, and column descriptor table.  The
                // plan is to traverse the table list, and for each member
                // of the list, traverse the corresponding segment list,
                // removing from the list information about segments and
                // columns in the file we're unloading.  If the segment list
                // for any table becomes empty, we remove the entry for that
                // table from the table list.
                //
                save.TBCURR = save.TBHEAD;

                while (save.TBCURR > 0) {
                    //
                    // See whether the current table is in the file we're
                    // unloading.
                    //
                    save.I = 1;

                    while ((save.I <= save.TBFLSZ[save.TBCURR]) && !save.FND) {
                        if (save.TBFILS[[save.I, save.TBCURR]] == *HANDLE) {
                            //
                            // This table is affected by unloading the file.
                            //
                            save.FND = true;
                        } else {
                            //
                            // Look at the next file handle.
                            //
                            save.I = (save.I + 1);
                        }
                    }

                    if save.FND {
                        //
                        // Update the information for the current table to
                        // reflect the unloading of the specified EK.
                        //
                        // Unloading the specified EK removes one handle from
                        // the list of file handles associated with this
                        // table.  Compress this handle out of the list.
                        //
                        {
                            let m1__: i32 = save.I;
                            let m2__: i32 = (save.TBFLSZ[save.TBCURR] - 1);
                            let m3__: i32 = 1;
                            save.J = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                save.TBFILS[[save.J, save.TBCURR]] =
                                    save.TBFILS[[(save.J + 1), save.TBCURR]];

                                save.J += m3__;
                            }
                        }

                        save.TBFLSZ[save.TBCURR] = (save.TBFLSZ[save.TBCURR] - 1);

                        //
                        // Traverse the segment list for this table, looking
                        // for segments in the specified EK.
                        //
                        save.DELSEG = save.TBSTPT[save.TBCURR];

                        while (save.DELSEG > 0) {
                            if (save.STHAN[save.DELSEG] == *HANDLE) {
                                //
                                // This segment is aboard the sinking ship.  Put
                                // it out of its misery.
                                //
                                // First, euthanize its column descriptors.
                                // These descriptors are linked together, so we
                                // can free all of them in one shot.
                                //
                                save.J = save.STDTPT[save.DELSEG];

                                if (save.J > 0) {
                                    save.K = LNKTL(save.J, save.DTPOOL.as_slice(), ctx)?;
                                    LNKFSL(save.J, save.K, save.DTPOOL.as_slice_mut(), ctx)?;
                                }

                                //
                                // Now we can delete the segment table entry
                                // itself.  This deletion may necessitate
                                // updating the segment list pointer in the
                                // parent table's table list entry.
                                //
                                if (save.DELSEG == save.TBSTPT[save.TBCURR]) {
                                    save.TBSTPT[save.TBCURR] =
                                        LNKNXT(save.DELSEG, save.STPOOL.as_slice(), ctx)?;
                                }

                                save.NEXT = LNKNXT(save.DELSEG, save.STPOOL.as_slice(), ctx)?;
                                LNKFSL(save.DELSEG, save.DELSEG, save.STPOOL.as_slice_mut(), ctx)?;

                                //
                                // The segment we just freed may have been the
                                // last one belonging to this table.  We deal
                                // with this possibility later, below the end of
                                // the current loop.
                                //
                                save.DELSEG = save.NEXT;
                            } else {
                                save.DELSEG = LNKNXT(save.DELSEG, save.STPOOL.as_slice(), ctx)?;
                            }
                        }

                        //
                        // We've examined all of the segments in the current
                        // table.
                        //
                        // If the segment list for the current table became
                        // empty as a result of our having plundered the
                        // segment table, delete the entry for this table from
                        // the table list. We do *not* need to concern
                        // ourselves with the possibility that this deletion
                        // will empty the table list, since we know we're
                        // not unloading the last loaded file.  However, we
                        // may need to update the head-of-list pointer for the
                        // table list.
                        //
                        if (save.TBSTPT[save.TBCURR] <= 0) {
                            //
                            // There are no loaded segments left for this
                            // table.
                            //
                            // Delete the list of column attribute entries for
                            // the columns in this table, then delete the
                            // table's entry from the table list.
                            //
                            // The column attribute entries are linked, so we
                            // can free them in one shot.  Don't crash if the
                            // column attribute list is empty.
                            //
                            save.J = save.TBCTPT[save.TBCURR];

                            if (save.J > 0) {
                                save.K = LNKTL(save.J, save.CTPOOL.as_slice(), ctx)?;
                                LNKFSL(save.J, save.K, save.CTPOOL.as_slice_mut(), ctx)?;
                            }

                            if (save.TBCURR == save.TBHEAD) {
                                //
                                // The entry for this table is at the head of
                                // the table list.  Update the head of the list.
                                //
                                save.TBHEAD = LNKNXT(save.TBCURR, save.TBPOOL.as_slice(), ctx)?;
                                save.NEXT = save.TBHEAD;
                            } else {
                                save.NEXT = LNKNXT(save.TBCURR, save.TBPOOL.as_slice(), ctx)?;
                            }

                            //
                            // Make the entry for this table go away.
                            //
                            LNKFSL(save.TBCURR, save.TBCURR, save.TBPOOL.as_slice_mut(), ctx)?;

                            //
                            // Look at the next table.
                            //
                            save.TBCURR = save.NEXT;
                        } else {
                            //
                            // We're done with the current table.  Look at the
                            // next one.
                            //
                            save.TBCURR = LNKNXT(save.TBCURR, save.TBPOOL.as_slice(), ctx)?;
                        }
                    //
                    // We've cleaned up the table entry for the current
                    // table, if it was necessary to do so.
                    //
                    } else {
                        //
                        // The current table is not affected by unloading this
                        // file.  Examine the next table.
                        //
                        save.TBCURR = LNKNXT(save.TBCURR, save.TBPOOL.as_slice(), ctx)?;
                    }

                    //
                    // We've processed the current table.
                    //
                }
            }

            //
            // We've cleaned up after the aborted partial load.
            //
            // Now that the mess has been arranged, tell the user what the
            // problem was.
            //
            if fstr::eq(&save.PROBLM, b"TABLE_LIST_FULL") {
                SETMSG(b"The EK file # could not be loaded; the maximum number of distinct tables has already been reached.", ctx);
                ERRHAN(b"#", *HANDLE, ctx)?;
                SIGERR(b"SPICE(EKTABLELISTFULL)", ctx)?;
            } else if fstr::eq(&save.PROBLM, b"DUPLICATE_COLUMN_NAMES") {
                SETMSG(b"The EK file # could not be loaded; the segment # contains duplicate column names in table #.", ctx);
                ERRHAN(b"#", *HANDLE, ctx)?;
                ERRINT(b"#", save.SEG, ctx);
                ERRCH(b"#", &save.TABNAM, ctx);
                SIGERR(b"SPICE(EKCOLNUMMISMATCH)", ctx)?;
            } else if fstr::eq(&save.PROBLM, b"COLUMN_NUMBER_MISMATCH") {
                SETMSG(b"The EK file # could not be loaded; the number of columns (#) in segment # does not match the number of columns (#) in the parent table #.", ctx);
                ERRHAN(b"#", *HANDLE, ctx)?;
                ERRINT(b"#", save.NCOLS, ctx);
                ERRINT(b"#", save.SEG, ctx);
                ERRINT(b"#", save.NPCOL, ctx);
                ERRCH(b"#", &save.TABNAM, ctx);
                SIGERR(b"SPICE(EKCOLNUMMISMATCH)", ctx)?;
            } else if fstr::eq(&save.PROBLM, b"MISMATCHED_COLUMN_ATTRIBUTES") {
                SETMSG(b"EK file # contains a column whose attributes conflict with a loaded column.  The offending column name is #; the column is in segment #* of the file.", ctx);
                ERRHAN(b"#", *HANDLE, ctx)?;
                ERRCH(b"#", &save.COLNAM, ctx);
                ERRINT(b"*", save.SEG, ctx);
                SIGERR(b"SPICE(BADATTRIBUTES)", ctx)?;
            } else if fstr::eq(&save.PROBLM, b"DESCRIPTOR_TABLE_FULL") {
                SETMSG(b"The EK file # could not be loaded; themaximum allowed number of loaded columns already been reached.", ctx);
                ERRHAN(b"#", *HANDLE, ctx)?;
                SIGERR(b"SPICE(COLDESCTABLEFULL)", ctx)?;
            } else if fstr::eq(&save.PROBLM, b"ATTRIBUTE_TABLE_FULL") {
                SETMSG(b"The EK file # could not be loaded; the maximum number of columns havingdistinct attributes has already been reached.", ctx);
                ERRHAN(b"#", *HANDLE, ctx)?;
                SIGERR(b"SPICE(EKCOLATTRTABLEFULL)", ctx)?;
            } else if fstr::eq(&save.PROBLM, b"MISSING_COLUMN") {
                SETMSG(b"The EK file # could not be loaded; the column # in already loaded table # is not present in segment # in the EK file.", ctx);
                ERRHAN(b"#", *HANDLE, ctx)?;
                ERRCH(b"#", &save.COLNAM, ctx);
                ERRCH(b"#", &save.TABNAM, ctx);
                ERRINT(b"#", save.SEG, ctx);
                SIGERR(b"SPICE(EKMISSINGCOLUMN)", ctx)?;
            } else {
                SETMSG(b"The EK file # could not be loaded; the problem \"#\" occurred while attempting to load the file.  By way, there is a bug in EKLEF if you see this message.", ctx);
                ERRHAN(b"#", *HANDLE, ctx)?;
                ERRCH(b"#", &save.PROBLM, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
            }

            EKCLS(*HANDLE, ctx)?;

            CHKOUT(b"EKLEF", ctx)?;
            return Ok(());
        }
    }
    //
    // At this point, we've made the file table, table list, segment
    // table, column descriptor table, and column attribute table updates
    // necessary to reflect the presence of the new file.
    //

    CHKOUT(b"EKLEF", ctx)?;
    Ok(())
}

/// EK, unload event file
///
/// Unload an EK file, making its contents inaccessible to the
/// EK reader routines, and clearing space in order to allow other
/// EK files to be loaded.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of EK file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is a file handle returned by EKLEF.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None.
///
///  See $Particulars for a description of the effect of this routine.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  Unloading a file that is not loaded has no effect.
/// ```
///
/// # Files
///
/// ```text
///  This routine unloads a binary EK file from the EK query system.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine removes information about an EK file from the
///  EK system, freeing space to increase the number of other EK
///  files that can be loaded. The file is also unloaded from
///  the DAS system and closed.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Load two EK files and perform a query on them. During query
///     execution, all files will be searched. Unload the previous
///     file before each new file is loaded. Unloading files prevents
///     them from being searched during query execution.
///
///     Use the EK kernel below to load the Cassini Science Plan
///     SPICE E-Kernel File based upon the integrated science
///     plan #78.
///
///        S78_CIMSSSUPa.bep
///
///     Use the EK kernel below to load the data based upon the
///     integrated science plan #79.
///
///        S79_CIMSSSUPa.bep
///
///
///     Example code begins here.
///
///
///           PROGRAM EKUEF_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Maximum length of an input query,
///     C     MAXQRY.
///     C
///           INCLUDE 'ekqlimit.inc'
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               EKNMLN
///           PARAMETER           ( EKNMLN = 17 )
///
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN = 1840 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(EKNMLN)    EKNAMS ( 2 )
///           CHARACTER*(ERRLEN)    ERRMSG
///           CHARACTER*(MAXQRY)    QUERY
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               N
///           INTEGER               NMROWS
///
///           LOGICAL               ERROR
///
///     C
///     C     Set up the array holding the EK file names.
///     C
///           DATA                  EKNAMS / 'S78_CIMSSSUPa.bep',
///          .                               'S79_CIMSSSUPa.bep'  /
///
///     C
///     C     The EK files contain a table 'CASSINI_SP_OBSERVATION',
///     C     that contains columns named:
///     C
///     C        NOTES, OBSERVATION_ID, OBSERVATION_TITLE,
///     C        OBS_DESCRIPTION, SCIENCE_OBJECTIVE, SEQUENCE,
///     C        SUBSYSTEM
///     C
///     C     Define a set of constraints to perform a query on all
///     C     loaded EK files (the SELECT clause).
///     C
///           QUERY = 'Select SUBSYSTEM, SCIENCE_OBJECTIVE, '
///          .   //   'OBSERVATION_ID from CASSINI_SP_OBSERVATION '
///          .   //   'order by SUBSYSTEM'
///
///     C
///     C     Load the EK files. This call could be replaced by a call
///     C     to FURNSH.
///     C
///           DO I = 1, 2
///
///              CALL EKLEF ( EKNAMS(I), HANDLE )
///              WRITE(*,'(2A)') 'Loading EK: ', EKNAMS(I)
///
///     C
///     C        Query the EK system for data rows matching the
///     C        SELECT constraints.
///     C
///              CALL EKFIND ( QUERY, NMROWS, ERROR, ERRMSG )
///
///     C
///     C        Check whether an error occurred while processing the
///     C        SELECT clause. If so, output the error message.
///     C
///              IF ( ERROR ) THEN
///
///                 WRITE(*,'(2A)') 'SELECT clause error: ', ERRMSG
///
///              ELSE
///
///     C
///     C           If no error, NMROWS contains the number of rows
///     C           matching the constraints specified in the query
///     C           string.
///     C
///                 WRITE(*,'(A,I3)') 'Number of matching rows: ',
///          .                        NMROWS
///
///              END IF
///
///     C
///     C        Unload the current file. Unloading files prevents
///     C        them from being searched during query execution.
///     C
///              CALL EKUEF ( HANDLE )
///              WRITE(*,'(2A)') 'Unloading EK: ', EKNAMS(I)
///              WRITE(*,*)
///
///           END DO
///
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Loading EK: S78_CIMSSSUPa.bep
///     Number of matching rows:   4
///     Unloading EK: S78_CIMSSSUPa.bep
///
///     Loading EK: S79_CIMSSSUPa.bep
///     Number of matching rows:   5
///     Unloading EK: S79_CIMSSSUPa.bep
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (NJB)
///
///         Bug fix: When an already loaded kernel is opened with EKOPR,
///         it now has its link count reset to 1 via a call to EKCLS.
///
/// -    SPICELIB Version 1.0.1, 07-JUL-1996 (NJB)
///
///         Previous version line was changed from "Beta" to "SPICELIB."
///
/// -    SPICELIB Version 1.0.0, 23-OCT-1995 (NJB)
/// ```
pub fn ekuef(ctx: &mut SpiceContext, handle: i32) -> crate::Result<()> {
    EKUEF(handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKUEF  ( EK, unload event file )
pub fn EKUEF(HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"EKUEF", ctx)?;
    }

    //
    // On the first pass through this routine, initialize the tables,
    // if it hasn't been done yet.
    //
    if save.FIRST {
        //
        // Initialize the file table pool, segment table pool, column
        // descriptor pool, column table pool, and table list pool.
        //
        LNKINI(FTSIZE, save.FTPOOL.as_slice_mut(), ctx)?;
        LNKINI(STSIZE, save.STPOOL.as_slice_mut(), ctx)?;
        LNKINI(DTSIZE, save.DTPOOL.as_slice_mut(), ctx)?;
        LNKINI(CTSIZE, save.CTPOOL.as_slice_mut(), ctx)?;
        LNKINI(MXTBLD, save.TBPOOL.as_slice_mut(), ctx)?;

        save.FTHEAD = 0;
        save.TBHEAD = 0;

        save.FIRST = false;
    }

    //
    // Check to see whether the named EK has been loaded.  Do nothing
    // if not.
    //
    save.I = save.FTHEAD;
    save.FND = false;

    while ((save.I > 0) && !save.FND) {
        if (HANDLE == save.FTHAN[save.I]) {
            save.FND = true;
        } else {
            save.I = LNKNXT(save.I, save.FTPOOL.as_slice(), ctx)?;
        }
    }

    if !save.FND {
        CHKOUT(b"EKUEF", ctx)?;
        return Ok(());
    }

    //
    // If we got to here, HANDLE points to a loaded EK file.  It's
    // time to wipe from the EK tables all trivial fond records
    // pertaining to the file in question.
    //
    // The file table is first.
    //
    if (save.I == save.FTHEAD) {
        //
        // The file is at the head of the list.  If the file has a
        // successor, that file is now at the head of the list.
        //
        save.FTHEAD = LNKNXT(save.I, save.FTPOOL.as_slice(), ctx)?;

        if (save.FTHEAD < 0) {
            //
            // There are no files left.  Clean up the whole shebang.
            //
            LNKINI(FTSIZE, save.FTPOOL.as_slice_mut(), ctx)?;
            LNKINI(STSIZE, save.STPOOL.as_slice_mut(), ctx)?;
            LNKINI(DTSIZE, save.DTPOOL.as_slice_mut(), ctx)?;
            LNKINI(CTSIZE, save.CTPOOL.as_slice_mut(), ctx)?;
            LNKINI(MXTBLD, save.TBPOOL.as_slice_mut(), ctx)?;

            save.FTHEAD = 0;
            save.TBHEAD = 0;

            //
            // Close the EK file, to keep the DAS system's bookkeeping
            // up to date.
            //
            EKCLS(HANDLE, ctx)?;

            CHKOUT(b"EKUEF", ctx)?;
            return Ok(());
        }
    }

    //
    // If we arrived here, the file we're unloading is not the only
    // loaded file.
    //
    // Free the file table entry for the file.  The entry can be
    // regarded as a sublist that starts and ends with the Ith node,
    // so we can call the `free sublist' routine to get rid of it.
    //
    LNKFSL(save.I, save.I, save.FTPOOL.as_slice_mut(), ctx)?;

    //
    // It's time to clean up the table list, segment table, column
    // attribute table, and column descriptor table.  The plan is
    // to traverse the table list, and for each member of the list,
    // traverse the corresponding segment list, removing from the list
    // information about segments and columns in the file we're
    // unloading.  If the segment list for any table becomes empty, we
    // remove the entry for that table from the table list.
    //
    save.TBCURR = save.TBHEAD;

    while (save.TBCURR > 0) {
        //
        // See whether the current table is in the file we're unloading.
        //
        save.I = 1;

        while ((save.I <= save.TBFLSZ[save.TBCURR]) && !save.FND) {
            if (save.TBFILS[[save.I, save.TBCURR]] == HANDLE) {
                //
                // This table is affected by unloading the file.
                //
                save.FND = true;
            } else {
                //
                // Look at the next file handle.
                //
                save.I = (save.I + 1);
            }
        }

        if save.FND {
            //
            // Update the information for the current table to reflect
            // the unloading of the specified EK.
            //
            // Unloading the specified EK removes one handle from the
            // list of file handles associated with this table.  Compress
            // this handle out of the list.
            //
            {
                let m1__: i32 = save.I;
                let m2__: i32 = (save.TBFLSZ[save.TBCURR] - 1);
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.TBFILS[[save.J, save.TBCURR]] = save.TBFILS[[(save.J + 1), save.TBCURR]];

                    save.J += m3__;
                }
            }

            save.TBFLSZ[save.TBCURR] = (save.TBFLSZ[save.TBCURR] - 1);

            //
            // Traverse the segment list for this table, looking
            // for segments in the specified EK.
            //
            save.SEG = save.TBSTPT[save.TBCURR];

            while (save.SEG > 0) {
                if (save.STHAN[save.SEG] == HANDLE) {
                    //
                    // This segment is aboard the sinking ship.  Put it
                    // out of its misery.
                    //
                    // First, euthanize the segment's column descriptors.
                    // These descriptors are linked together, so we can free
                    // all of them in one shot.  Don't crash if the column
                    // descriptor list is empty.
                    //
                    save.J = save.STDTPT[save.SEG];

                    if (save.J > 0) {
                        save.K = LNKTL(save.J, save.DTPOOL.as_slice(), ctx)?;
                        LNKFSL(save.J, save.K, save.DTPOOL.as_slice_mut(), ctx)?;
                    }

                    //
                    // Now we can delete the segment table entry itself.
                    // This deletion may necessitate updating the segment
                    // list pointer in the parent table's table list entry.
                    //
                    if (save.SEG == save.TBSTPT[save.TBCURR]) {
                        save.TBSTPT[save.TBCURR] = LNKNXT(save.SEG, save.STPOOL.as_slice(), ctx)?;
                    }

                    save.NEXT = LNKNXT(save.SEG, save.STPOOL.as_slice(), ctx)?;
                    LNKFSL(save.SEG, save.SEG, save.STPOOL.as_slice_mut(), ctx)?;

                    save.SEG = save.NEXT;
                } else {
                    save.SEG = LNKNXT(save.SEG, save.STPOOL.as_slice(), ctx)?;
                }
            }

            //
            // We've examined all of the segments in the current table.
            //
            // If the segment list for the current table became empty
            // as a result of our having plundered the segment table,
            // delete the entry for this table from the table list.  We do
            // *not* need to concern ourselves with the possibility that
            // this deletion will empty the table list, since we know we're
            // not unloading the last loaded file.  However, we may need to
            // update the head-of-list pointer for the table list.
            //
            if (save.TBSTPT[save.TBCURR] <= 0) {
                //
                // There are no loaded segments left for this table.
                //
                // Delete the list of column attribute entries for the
                // columns in this table, then delete the table's entry from
                // the table list.
                //
                // The column attribute entries are linked, so we can free
                // them in one shot.
                //
                save.J = save.TBCTPT[save.TBCURR];

                if (save.J > 0) {
                    save.K = LNKTL(save.J, save.CTPOOL.as_slice(), ctx)?;
                    LNKFSL(save.J, save.K, save.CTPOOL.as_slice_mut(), ctx)?;
                }

                if (save.TBCURR == save.TBHEAD) {
                    //
                    // The entry for this table is at the head of the
                    // table list.  Update the head of the list.
                    //
                    save.TBHEAD = LNKNXT(save.TBCURR, save.TBPOOL.as_slice(), ctx)?;
                    save.NEXT = save.TBHEAD;
                } else {
                    save.NEXT = LNKNXT(save.TBCURR, save.TBPOOL.as_slice(), ctx)?;
                }

                //
                // Make the entry for this table go away.
                //
                LNKFSL(save.TBCURR, save.TBCURR, save.TBPOOL.as_slice_mut(), ctx)?;

                //
                // The successor of the current node is the next node to
                // examine.
                //
                save.TBCURR = save.NEXT;
            } else {
                //
                // We're done with the current table.  Look at the next one.
                //
                save.TBCURR = LNKNXT(save.TBCURR, save.TBPOOL.as_slice(), ctx)?;
            }
        //
        // We've cleaned up the table entry for the current table,
        // if it was necessary to do so.
        //
        } else {
            //
            // The current table is not affected by unloading this file.
            // Examine the next table.
            //
            save.TBCURR = LNKNXT(save.TBCURR, save.TBPOOL.as_slice(), ctx)?;
        }
        //
        // We've processed the current table.
        //
    }

    //
    // Don't forget to unload the EK file from the DAS system.
    //
    EKCLS(HANDLE, ctx)?;

    CHKOUT(b"EKUEF", ctx)?;
    Ok(())
}

/// EK, return number of loaded tables
///
/// Return the number of loaded EK tables.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  N          O   Number of loaded tables.
/// ```
///
/// # Detailed Output
///
/// ```text
///  N        is the number of loaded tables. The count refers
///           to the number of logical tables; if multiple
///           segments contain data for the same table, these
///           segments collectively contribute only one table
///           to the count.
/// ```
///
/// # Files
///
/// ```text
///  The returned count is based on the currently loaded EK files.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is a utility that provides the caller with the
///  number of loaded tables. Callers of EKTNAM can use this count
///  as the upper bound on set of table indices when looking up table
///  names.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Suppose we have several EK files. Load one at a time and
///     display, right after, the number of loaded EK tables in the
///     system.
///
///     Use the EK kernel below to load the Cassini Science Plan
///     SPICE E-Kernel File based upon integrated science plan. This
///     kernel contains 3 tables.
///
///        S79_CIMSSSUPa.bep
///
///     Use the EK kernel below to load the Cassini Spacecraft
///     Sequence Status SPICE E-Kernel File based upon integrated
///     Predicted Events File. This kernel contains 1 table.
///
///        S79_status_pf.bes
///
///
///     Example code begins here.
///
///
///           PROGRAM EKNTAB_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables
///     C
///           INTEGER               N
///
///     C
///     C     Load the first EK.
///     C
///           CALL FURNSH ( 'S79_CIMSSSUPa.bep' )
///
///     C
///     C     Display the number of EK tables in the system after
///     C     the first EK file is loaded.
///     C
///           CALL EKNTAB ( N )
///           WRITE(*,'(A,I3)') 'EK tables in the system (1 EK):', N
///
///     C
///     C     Load the second EK.
///     C
///           CALL FURNSH ( 'S79_status_pf.bes' )
///
///     C
///     C     Display the number of EK tables in the system after
///     C     the second EK file is loaded.
///     C
///           CALL EKNTAB ( N )
///           WRITE(*,'(A,I3)') 'EK tables in the system (2 EK):', N
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     EK tables in the system (1 EK):  3
///     EK tables in the system (2 EK):  4
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing fragments.
///
///         Removed the requirement of loading the files via EKELF from the
///         $Files section.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (NJB)
///
///         Bug fix: When an already loaded kernel is opened with EKOPR,
///         it now has its link count reset to 1 via a call to EKCLS.
///
/// -    SPICELIB Version 1.0.1, 07-JUL-1996 (NJB)
///
///         Previous version line was changed from "Beta" to "SPICELIB."
///
/// -    SPICELIB Version 1.0.0, 23-OCT-1995 (NJB)
/// ```
pub fn ekntab(ctx: &mut SpiceContext, n: &mut i32) -> crate::Result<()> {
    EKNTAB(n, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKNTAB  ( EK, return number of loaded tables )
pub fn EKNTAB(N: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if save.FIRST {
        //
        // Initialize the file table pool, segment table pool, column
        // descriptor pool, column table pool, and table list pool.
        //
        LNKINI(FTSIZE, save.FTPOOL.as_slice_mut(), ctx)?;
        LNKINI(STSIZE, save.STPOOL.as_slice_mut(), ctx)?;
        LNKINI(DTSIZE, save.DTPOOL.as_slice_mut(), ctx)?;
        LNKINI(CTSIZE, save.CTPOOL.as_slice_mut(), ctx)?;
        LNKINI(MXTBLD, save.TBPOOL.as_slice_mut(), ctx)?;

        save.FTHEAD = 0;
        save.TBHEAD = 0;

        save.FIRST = false;
    }
    //
    // Return the number of loaded tables.
    //
    *N = (MXTBLD - LNKNFN(save.TBPOOL.as_slice()));
    Ok(())
}

/// EK, return name of loaded table
///
/// Return the name of a specified, loaded table.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  N          I   Index of table.
///  TABLE      O   Name of table.
/// ```
///
/// # Detailed Input
///
/// ```text
///  N        is the index of the table whose name is desired.
///           The value of N ranges from 1 to the number of
///           loaded tables, which count may be obtained from
///           EKNTAB.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TABLE    is the name of the N'th loaded table. If TABLE
///           is too small to accommodate the name, the name will
///           be truncated on the right.
/// ```
///
/// # Parameters
///
/// ```text
///  TNAMSZ   is the maximum allowed table name length. See the
///           include file ektnamsz.inc for the actual value of
///           this parameter.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If this routine is called when no files are loaded, the
///      error SPICE(NOLOADEDFILES) is signaled.
///
///  2)  If the input N is out of range, the error SPICE(INVALDINDEX)
///      is signaled.
/// ```
///
/// # Files
///
/// ```text
///  The returned name is based on the currently loaded EK files.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is a utility that provides the caller with the
///  name of a specified loaded table. The index of a table with
///  a given name depends on the kernels loaded and possibly on
///  the order in which the files have been loaded.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Dump the names of all the loaded tables.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: ektnam_ex1.tm
///
///        This meta-kernel is intended to support operation of SPICE
///        example programs. The kernels shown here should not be
///        assumed to contain adequate or correct versions of data
///        required by SPICE-based user applications.
///
///        In order for an application to use this meta-kernel, the
///        kernels referenced here must be present in the user's
///        current working directory.
///
///        The names and contents of the kernels referenced
///        by this meta-kernel are as follows:
///
///           File name                 Contents
///           ---------                 --------
///           S78_CIMSSSUPa.bep         Cassini Science Plan #78
///           S79_CIMSSSUPa.bep         Cassini Science Plan #79
///           S79_status_pf.bes         Cassini Spacecraft Sequence
///                                     Status #79
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'S78_CIMSSSUPa.bep',
///                               'S79_CIMSSSUPa.bep',
///                               'S79_status_pf.bes'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM EKTNAM_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include EK parameter declarations:
///     C
///     C        EK Table Name Size
///     C
///           INCLUDE 'ektnamsz.inc'
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'ektnam_ex1.tm' )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(TNAMSZ)      TABNAM
///
///           INTEGER                 NTAB
///           INTEGER                 TAB
///
///     C
///     C     Load the EK files. Use a meta-kernel for convenience.
///     C
///           CALL FURNSH ( META )
///
///     C
///     C     Get the number of loaded tables. The count refers to the
///     C     number of logical tables; if multiple EKs contain data
///     C     for the same table, these EKs collectively contribute
///     C     only one table to the count.
///     C
///           CALL EKNTAB ( NTAB )
///
///           WRITE(*,'(A,I3)') 'Number of tables in EK subsystem:',
///          .                   NTAB
///
///           DO TAB = 1, NTAB
///
///     C
///     C        Get the name of the current table, and display it.
///     C
///              CALL EKTNAM ( TAB, TABNAM )
///              WRITE(*,'(2A)') '   TABLE = ', TABNAM
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Number of tables in EK subsystem:  4
///        TABLE = CASSINI_SP_REQUEST
///        TABLE = CASSINI_SP_OBSERVATION
///        TABLE = CASSINI_SP_REQ_OBS
///        TABLE = CASSINI_STATUS
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
///         Added description of TNAMSZ parameter.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (NJB)
///
///         Bug fix: When an already loaded kernel is opened with EKOPR,
///         it now has its link count reset to 1 via a call to EKCLS.
///
/// -    SPICELIB Version 1.0.1, 07-JUL-1996 (NJB)
///
///         Previous version line was changed from "Beta" to "SPICELIB."
///
/// -    SPICELIB Version 1.0.0, 23-OCT-1995 (NJB)
/// ```
pub fn ektnam(ctx: &mut SpiceContext, n: i32, table: &mut str) -> crate::Result<()> {
    EKTNAM(n, fstr::StrBytes::new(table).as_mut(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKTNAM  ( EK, return name of loaded table )
pub fn EKTNAM(N: i32, TABLE: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"EKTNAM", ctx)?;
    }

    if save.FIRST {
        //
        // Initialize the file table pool, segment table pool, column
        // descriptor pool, column table pool, and table list pool.
        //
        LNKINI(FTSIZE, save.FTPOOL.as_slice_mut(), ctx)?;
        LNKINI(STSIZE, save.STPOOL.as_slice_mut(), ctx)?;
        LNKINI(DTSIZE, save.DTPOOL.as_slice_mut(), ctx)?;
        LNKINI(CTSIZE, save.CTPOOL.as_slice_mut(), ctx)?;
        LNKINI(MXTBLD, save.TBPOOL.as_slice_mut(), ctx)?;

        save.FTHEAD = 0;
        save.TBHEAD = 0;

        save.FIRST = false;
    }

    //
    // There nothing to fetch if no files are loaded.  A sure
    // symptom of this problem is that the file list is empty.
    //
    if (save.FTHEAD <= 0) {
        SETMSG(b"No E-kernels are currently loaded.", ctx);
        SIGERR(b"SPICE(NOLOADEDFILES)", ctx)?;
        CHKOUT(b"EKTNAM", ctx)?;
        return Ok(());
    }

    save.TBCURR = save.TBHEAD;
    save.FND = false;
    save.I = 0;

    while ((save.TBCURR > 0) && !save.FND) {
        save.I = (save.I + 1);

        if (save.I == N) {
            save.FND = true;
            fstr::assign(TABLE, save.TBNAMS.get(save.TBCURR));
        } else {
            save.TBCURR = LNKNXT(save.TBCURR, save.TBPOOL.as_slice(), ctx)?;
        }
    }

    if !save.FND {
        SETMSG(b"The index # does not correspond to a loaded table.", ctx);
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
    }

    CHKOUT(b"EKTNAM", ctx)?;
    Ok(())
}

/// EK, column count
///
/// Return the number of distinct columns in a specified, currently
/// loaded table.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  TABLE      I   Name of table.
///  CCOUNT     O   Count of distinct, currently loaded columns.
/// ```
///
/// # Detailed Input
///
/// ```text
///  TABLE    is the name of a currently loaded table. Case
///           is not significant in the table name.
/// ```
///
/// # Detailed Output
///
/// ```text
///  CCOUNT   is the number of distinct columns in TABLE.
///           Columns that have the same name but belong to
///           different segments that are considered to be
///           portions of the same column, if the segments
///           containing those columns belong to TABLE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified table is not loaded, the error
///      SPICE(TABLENOTLOADED) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the header of EKQMGR for a description of files used
///  by this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is a utility intended for use in conjunction with
///  the entry point EKCII. These routines can be used to find the
///  names and attributes of the columns that are currently loaded.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Examine an EK. Dump the names and attributes of the columns in
///     each loaded table. EKCCNT is used to obtain column counts.
///
///
///     Example code begins here.
///
///
///           PROGRAM EKCCNT_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include EK parameter declarations:
///     C
///     C        ekattdsc.inc: EK Column Attribute Descriptor
///     C                      Parameters
///     C        ekcnamsz.inc: EK Column Name Size
///     C        ektnamsz.inc: EK Table Name Size
///     C        ektype.inc:   EK Data Types
///     C
///           INCLUDE 'ekattdsc.inc'
///           INCLUDE 'ekcnamsz.inc'
///           INCLUDE 'ektnamsz.inc'
///           INCLUDE 'ektype.inc'
///
///     C
///     C     Local parameters.
///     C
///           INTEGER                 FILEN
///           PARAMETER             ( FILEN = 255 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(CNAMSZ)      COLNAM
///           CHARACTER*(FILEN)       EKFILE
///           CHARACTER*(TNAMSZ)      TABNAM
///
///           INTEGER                 ATTDSC ( ADSCSZ )
///           INTEGER                 I
///           INTEGER                 NCOLS
///           INTEGER                 NTAB
///           INTEGER                 TAB
///
///     C
///     C     Prompt for the EK file name.
///     C
///           CALL PROMPT ( 'Enter name of EK to examine > ', EKFILE )
///
///           CALL FURNSH ( EKFILE )
///
///     C
///     C     Get the number of loaded tables.
///     C
///           CALL EKNTAB ( NTAB )
///
///           WRITE(*,*) 'Number of tables in EK:', NTAB
///
///           DO TAB = 1, NTAB
///
///     C
///     C        Get the name of the current table, and look up
///     C        the column count for this table.
///     C
///              CALL EKTNAM ( TAB,    TABNAM )
///              CALL EKCCNT ( TABNAM, NCOLS  )
///
///              WRITE(*,*) '------------------------------'
///          .           // '------------------------------'
///              WRITE(*,*) 'TABLE = ', TABNAM
///              WRITE(*,*) ' '
///
///     C
///     C        For each column in the current table, look up the
///     C        column's attributes.  The attribute block
///     C        index parameters are defined in the include file
///     C        ekattdsc.inc.
///     C
///              DO I = 1, NCOLS
///
///                 CALL EKCII ( TABNAM, I, COLNAM, ATTDSC )
///
///                 WRITE (*,*) 'COLUMN = ', COLNAM
///
///     C
///     C           Write out the current column's data type.
///     C
///                 IF ( ATTDSC(ATTTYP) .EQ. CHR ) THEN
///
///                    WRITE (*,*) '   TYPE   =  CHR'
///
///                    IF ( ATTDSC(ATTLEN) .EQ. -1 ) THEN
///                       WRITE (*,*) '   STRING LENGTH = VARIABLE.'
///                    ELSE
///                       WRITE (*,'(A,I2)') '    STRING LENGTH = ',
///          .                         ATTDSC(ATTLEN)
///                    END IF
///
///                 ELSE IF ( ATTDSC(ATTTYP) .EQ. DP ) THEN
///                    WRITE (*,*) '   TYPE   =  DP'
///
///                 ELSE IF ( ATTDSC(ATTTYP) .EQ. INT ) THEN
///                    WRITE (*,*) '   TYPE   =  INT'
///
///                 ELSE
///                    WRITE (*,*) '   TYPE   =  TIME'
///                 END IF
///
///     C
///     C           Write out the current column's entry size.
///     C
///                 WRITE (*,'(A,I2)') '    SIZE   = ', ATTDSC(ATTSIZ)
///
///     C
///     C           Indicate whether the current column is indexed.
///     C
///                 IF ( ATTDSC(ATTIDX) .EQ. -1 ) THEN
///                    WRITE (*,*) '   NOT INDEXED'
///                 ELSE
///                    WRITE (*,*) '   INDEXED'
///                 END IF
///
///     C
///     C           Indicate whether the current column allows
///     C           null values.
///     C
///                 IF ( ATTDSC(ATTNFL) .EQ. -1 ) THEN
///                    WRITE (*,*) '   NULL VALUES NOT ALLOWED'
///                 ELSE
///                    WRITE (*,*) '   NULL VALUES ALLOWED'
///                 END IF
///
///              END DO
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the EK named S79_CIMSSSUPa.bep to load the
///     Cassini Science Plan SPICE E-Kernel File based upon the
///     integrated science plan, the output was:
///
///
///     Enter name of EK to examine > S79_CIMSSSUPa.bep
///      Number of tables in EK:           3
///      ------------------------------------------------------------
///      TABLE = CASSINI_SP_REQUEST
///
///      COLUMN = SUBSYSTEM
///         TYPE   =  CHR
///         STRING LENGTH = 32
///         SIZE   =  1
///         INDEXED
///         NULL VALUES NOT ALLOWED
///      COLUMN = REQUEST_ID
///         TYPE   =  CHR
///         STRING LENGTH = VARIABLE.
///         SIZE   =  1
///         INDEXED
///         NULL VALUES NOT ALLOWED
///      COLUMN = REQUEST_TITLE
///         TYPE   =  CHR
///         STRING LENGTH = VARIABLE.
///         SIZE   =  1
///         INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = BEGIN_TIME
///         TYPE   =  TIME
///         SIZE   =  1
///         INDEXED
///         NULL VALUES NOT ALLOWED
///      COLUMN = END_TIME
///         TYPE   =  TIME
///         SIZE   =  1
///         INDEXED
///         NULL VALUES NOT ALLOWED
///      COLUMN = SEQUENCE
///         TYPE   =  CHR
///         STRING LENGTH = 32
///         SIZE   =  1
///         INDEXED
///         NULL VALUES NOT ALLOWED
///      COLUMN = POINTING_AGREEMENT
///         TYPE   =  CHR
///         STRING LENGTH = 80
///         SIZE   = -1
///         NOT INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = PRIMARY_POINTING
///         TYPE   =  CHR
///         STRING LENGTH = VARIABLE.
///         SIZE   =  1
///         INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = SECONDARY_POINTING
///         TYPE   =  CHR
///         STRING LENGTH = VARIABLE.
///         SIZE   =  1
///         INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = REQ_DESCRIPTION
///         TYPE   =  CHR
///         STRING LENGTH = 80
///         SIZE   = -1
///         NOT INDEXED
///         NULL VALUES ALLOWED
///      ------------------------------------------------------------
///      TABLE = CASSINI_SP_OBSERVATION
///
///      COLUMN = SUBSYSTEM
///         TYPE   =  CHR
///         STRING LENGTH = 32
///         SIZE   =  1
///         INDEXED
///         NULL VALUES NOT ALLOWED
///      COLUMN = OBSERVATION_ID
///         TYPE   =  CHR
///         STRING LENGTH = VARIABLE.
///         SIZE   =  1
///         INDEXED
///         NULL VALUES NOT ALLOWED
///      COLUMN = OBSERVATION_TITLE
///         TYPE   =  CHR
///         STRING LENGTH = VARIABLE.
///         SIZE   =  1
///         INDEXED
///         NULL VALUES NOT ALLOWED
///      COLUMN = SEQUENCE
///         TYPE   =  CHR
///         STRING LENGTH = 32
///         SIZE   =  1
///         INDEXED
///         NULL VALUES NOT ALLOWED
///      COLUMN = SCIENCE_OBJECTIVE
///         TYPE   =  CHR
///         STRING LENGTH = 80
///         SIZE   = -1
///         NOT INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = OBS_DESCRIPTION
///         TYPE   =  CHR
///         STRING LENGTH = 80
///         SIZE   = -1
///
///     [...]
///
///
///     Warning: incomplete output. Only 100 out of 129 lines have been
///     provided.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing code fragment.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (NJB)
///
///         Bug fix: When an already loaded kernel is opened with EKOPR,
///         it now has its link count reset to 1 via a call to EKCLS.
///
/// -    SPICELIB Version 1.0.1, 07-JUL-1996 (NJB)
///
///         Misspelling of "conjunction" was fixed.
///
/// -    SPICELIB Version 1.0.0, 23-OCT-1995 (NJB)
/// ```
pub fn ekccnt(ctx: &mut SpiceContext, table: &str, ccount: &mut i32) -> crate::Result<()> {
    EKCCNT(table.as_bytes(), ccount, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKCCNT ( EK, column count )
pub fn EKCCNT(TABLE: &[u8], CCOUNT: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"EKCCNT", ctx)?;
    }

    //
    // On the first pass through this routine, initialize the tables,
    // if it hasn't been done yet.
    //
    if save.FIRST {
        //
        // Initialize the file table pool, segment table pool, column
        // descriptor pool, column table pool, and table list pool.
        //
        LNKINI(FTSIZE, save.FTPOOL.as_slice_mut(), ctx)?;
        LNKINI(STSIZE, save.STPOOL.as_slice_mut(), ctx)?;
        LNKINI(DTSIZE, save.DTPOOL.as_slice_mut(), ctx)?;
        LNKINI(CTSIZE, save.CTPOOL.as_slice_mut(), ctx)?;
        LNKINI(MXTBLD, save.TBPOOL.as_slice_mut(), ctx)?;

        save.FTHEAD = 0;
        save.TBHEAD = 0;

        save.FIRST = false;
    }

    //
    // Find the table.  If there's no match, the number of loaded columns
    // is zero.
    //
    save.TBCURR = save.TBHEAD;
    save.FND = false;

    while ((save.TBCURR > 0) && !save.FND) {
        if EQSTR(TABLE, &save.TBNAMS[save.TBCURR]) {
            save.FND = true;
        } else {
            save.TBCURR = LNKNXT(save.TBCURR, save.TBPOOL.as_slice(), ctx)?;
        }
    }

    if !save.FND {
        *CCOUNT = 0;
        SETMSG(b"The table # is not currently loaded.", ctx);
        ERRCH(b"#", TABLE, ctx);
        SIGERR(b"SPICE(TABLENOTLOADED)", ctx)?;
        CHKOUT(b"EKCCNT", ctx)?;
        return Ok(());
    } else {
        //
        // Count the columns in the attribute table for the current table.
        //
        *CCOUNT = 0;
        save.COL = save.TBCTPT[save.TBCURR];

        while (save.COL > 0) {
            *CCOUNT = (*CCOUNT + 1);
            save.COL = LNKNXT(save.COL, save.CTPOOL.as_slice(), ctx)?;
        }
    }

    CHKOUT(b"EKCCNT", ctx)?;
    Ok(())
}

/// EK, column info by index
///
/// Return attribute information about a column belonging to a loaded
/// EK table, specifying the column by table and index.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  TABLE      I   Name of table containing column.
///  CINDEX     I   Index of column whose attributes are to be found.
///  COLUMN     O   Name of column.
///  ATTDSC     O   Column attribute descriptor.
/// ```
///
/// # Detailed Input
///
/// ```text
///  TABLE    is the name of a loaded EK table. Case is not
///           significant.
///
///  CINDEX   is the index, within TABLE's column attribute
///           table, of the column whose attributes are to be
///           found. The indices of the column table entries
///           range from 1 to CCOUNT, where CCOUNT is the value
///           returned by the entry point EKCCNT.
/// ```
///
/// # Detailed Output
///
/// ```text
///  COLUMN   is the name of the specified column.
///
///  ATTDSC   is a column attribute descriptor. ATTDSC is an
///           integer array containing descriptive information
///           that applies uniformly to all loaded columns
///           having the name COLUMN. The following parameter
///           values occur in ATTDSC:
///
///              IFALSE:  -1
///              ITRUE:    1
///              CHR:      1
///              DP:       2
///              INT:      3
///              TIME:     4
///
///           The meanings of the elements of ATTDSC are given
///           below. The indices of the elements are
///           parameterized; the parameter values are defined
///           in the include file ekattdsc.inc.
///
///              ATTDSC(ATTCLS):   Column class code
///
///              ATTDSC(ATTTYP):   Data type code---CHR, DP, INT,
///                                or TIME
///
///              ATTDSC(ATTLEN):   String length; applies to CHR
///                                type. Value is IFALSE for
///                                variable-length strings.
///
///              ATTDSC(ATTSIZ):   Column entry size; value is
///                                IFALSE for variable-size
///                                columns. Here `size' refers
///                                to the number of array
///                                elements in a column entry.
///
///              ATTDSC(ATTIDX):   Index flag; value is ITRUE if
///                                column is indexed, IFALSE
///                                otherwise.
///
///              ATTDSC(ATTNFL):   Null flag; value is ITRUE if
///                                column may contain null
///                                values, IFALSE otherwise.
/// ```
///
/// # Parameters
///
/// ```text
///  ADSCSZ   is the size of column attribute descriptor.
///           (Defined in ekattdsc.inc.)
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified table is not loaded, the error
///      SPICE(TABLENOTLOADED) is signaled.
///
///  2)  If the input argument CINDEX is out of range, the error
///      SPICE(INVALIDINDEX) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the header of EKQMGR for a description of files used
///  by this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is a utility that allows a calling routine to
///  determine the attributes of the currently loaded columns.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Dump the names and attributes of the columns in each loaded
///     table.
///
///
///     Example code begins here.
///
///
///           PROGRAM EKCII_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include EK parameter declarations:
///     C
///     C        ekattdsc.inc: EK Column Attribute Descriptor
///     C                      Parameters
///     C        ekcnamsz.inc: EK Column Name Size
///     C        ektnamsz.inc: EK Table Name Size
///     C        ektype.inc:   EK Data Types
///     C
///           INCLUDE 'ekattdsc.inc'
///           INCLUDE 'ekcnamsz.inc'
///           INCLUDE 'ektnamsz.inc'
///           INCLUDE 'ektype.inc'
///
///     C
///     C     Local constants.
///     C
///           INTEGER                 FILEN
///           PARAMETER             ( FILEN = 255 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(CNAMSZ)      COLNAM
///           CHARACTER*(FILEN)       EKFILE
///           CHARACTER*(TNAMSZ)      TABNAM
///
///           INTEGER                 ATTDSC ( ADSCSZ )
///           INTEGER                 I
///           INTEGER                 NCOLS
///           INTEGER                 NTAB
///           INTEGER                 TAB
///
///     C
///     C     Prompt for the EK file name.
///     C
///           CALL PROMPT ( 'Enter name of EK to examine > ', EKFILE )
///
///           CALL FURNSH ( EKFILE )
///
///     C
///     C     Get the number of loaded tables.
///     C
///           CALL EKNTAB ( NTAB )
///
///           WRITE(*,*) 'Number of tables in EK:', NTAB
///
///           DO TAB = 1, NTAB
///
///     C
///     C        Get the name of the current table, and look up
///     C        the column count for this table.
///     C
///              CALL EKTNAM ( TAB,    TABNAM )
///              CALL EKCCNT ( TABNAM, NCOLS  )
///
///              WRITE(*,*) '------------------------------'
///          .           // '------------------------------'
///              WRITE(*,*) 'TABLE = ', TABNAM
///              WRITE(*,*) ' '
///
///     C
///     C        For each column in the current table, look up the
///     C        column's attributes.  The attribute block
///     C        index parameters are defined in the include file
///     C        ekattdsc.inc.
///     C
///              DO I = 1, NCOLS
///
///                 CALL EKCII ( TABNAM, I, COLNAM, ATTDSC )
///
///                 WRITE (*,*) 'COLUMN = ', COLNAM
///
///     C
///     C           Write out the current column's data type.
///     C
///                 IF ( ATTDSC(ATTTYP) .EQ. CHR ) THEN
///
///                    WRITE (*,*) '   TYPE   =  CHR'
///
///                    IF ( ATTDSC(ATTLEN) .EQ. -1 ) THEN
///                       WRITE (*,*) '   STRING LENGTH = VARIABLE.'
///                    ELSE
///                       WRITE (*,'(A,I2)') '    STRING LENGTH = ',
///          .                         ATTDSC(ATTLEN)
///                    END IF
///
///                 ELSE IF ( ATTDSC(ATTTYP) .EQ. DP ) THEN
///                    WRITE (*,*) '   TYPE   =  DP'
///
///                 ELSE IF ( ATTDSC(ATTTYP) .EQ. INT ) THEN
///                    WRITE (*,*) '   TYPE   =  INT'
///
///                 ELSE
///                    WRITE (*,*) '   TYPE   =  TIME'
///                 END IF
///
///     C
///     C           Write out the current column's entry size.
///     C
///                 WRITE (*,'(A,I2)') '    SIZE   = ', ATTDSC(ATTSIZ)
///
///     C
///     C           Indicate whether the current column is indexed.
///     C
///                 IF ( ATTDSC(ATTIDX) .EQ. -1 ) THEN
///                    WRITE (*,*) '   NOT INDEXED'
///                 ELSE
///                    WRITE (*,*) '   INDEXED'
///                 END IF
///
///     C
///     C           Indicate whether the current column allows
///     C           null values.
///     C
///                 IF ( ATTDSC(ATTNFL) .EQ. -1 ) THEN
///                    WRITE (*,*) '   NULL VALUES NOT ALLOWED'
///                 ELSE
///                    WRITE (*,*) '   NULL VALUES ALLOWED'
///                 END IF
///
///              END DO
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the EK file named vo_sedr.bdb to load the
///     Viking Orbiter Image SEDR Data, the output was:
///
///
///     Enter name of EK to examine > vo_sedr.bdb
///      Number of tables in EK:           1
///      ------------------------------------------------------------
///      TABLE = VIKING_SEDR_DATA
///
///      COLUMN = IMAGE_ID
///         TYPE   =  CHR
///         STRING LENGTH =  6
///         SIZE   =  1
///         INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = IMAGE_NUMBER
///         TYPE   =  INT
///         SIZE   =  1
///         INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = SPACECRAFT_ID
///         TYPE   =  CHR
///         STRING LENGTH =  3
///         SIZE   =  1
///         INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = IMAGE_TIME
///         TYPE   =  TIME
///         SIZE   =  1
///         INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = INSTRUMENT_ID
///         TYPE   =  CHR
///         STRING LENGTH =  4
///         SIZE   =  1
///         INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = GAIN_MODE_ID
///         TYPE   =  CHR
///         STRING LENGTH =  4
///         SIZE   =  1
///         NOT INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = FLOOD_MODE_ID
///         TYPE   =  CHR
///         STRING LENGTH =  3
///         SIZE   =  1
///         NOT INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = OFFSET_MODE_ID
///         TYPE   =  CHR
///         STRING LENGTH =  3
///         SIZE   =  1
///         NOT INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = FILTER_ID
///         TYPE   =  INT
///         SIZE   =  1
///         INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = EXPOSURE_DURATION
///         TYPE   =  DP
///         SIZE   =  1
///         INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = PLATFORM_IN_MOTION
///         TYPE   =  CHR
///         STRING LENGTH =  3
///         SIZE   =  1
///         NOT INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = PLATFORM_CONE
///         TYPE   =  DP
///         SIZE   =  1
///         NOT INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = PLATFORM_CLOCK
///         TYPE   =  DP
///         SIZE   =  1
///         NOT INDEXED
///         NULL VALUES ALLOWED
///      COLUMN = PLATFORM_TWIST
///         TYPE   =  DP
///         SIZE   =  1
///         NOT INDEXED
///         NULL VALUES ALLOWED
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited header to comply with NAIF standard.
///         Added complete code example from existing code fragment.
///
/// -    SPICELIB Version 2.0.1, 10-FEB-2014 (BVS)
///
///         Added description of ADSCSZ to the $Parameters section of the
///         header.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (NJB)
///
///         Bug fix: When an already loaded kernel is opened with EKOPR,
///         it now has its link count reset to 1 via a call to EKCLS.
///
/// -    SPICELIB Version 1.0.1, 07-JUL-1996 (NJB)
///
///         Previous version line was changed from "Beta" to "SPICELIB."
///
/// -    SPICELIB Version 1.0.0, 23-OCT-1995 (NJB)
/// ```
pub fn ekcii(
    ctx: &mut SpiceContext,
    table: &str,
    cindex: i32,
    column: &mut str,
    attdsc: &mut [i32; 6],
) -> crate::Result<()> {
    EKCII(
        table.as_bytes(),
        cindex,
        fstr::StrBytes::new(column).as_mut(),
        attdsc,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKCII  ( EK, column info by index )
pub fn EKCII(
    TABLE: &[u8],
    CINDEX: i32,
    COLUMN: &mut [u8],
    ATTDSC: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ATTDSC = DummyArrayMut::new(ATTDSC, 1..=ADSCSZ);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"EKCII", ctx)?;
    }

    //
    // On the first pass through this routine, initialize the tables,
    // if it hasn't been done yet.
    //
    if save.FIRST {
        //
        // Initialize the file table pool, segment table pool, column
        // descriptor pool, column table pool, and table list pool.
        //
        LNKINI(FTSIZE, save.FTPOOL.as_slice_mut(), ctx)?;
        LNKINI(STSIZE, save.STPOOL.as_slice_mut(), ctx)?;
        LNKINI(DTSIZE, save.DTPOOL.as_slice_mut(), ctx)?;
        LNKINI(CTSIZE, save.CTPOOL.as_slice_mut(), ctx)?;
        LNKINI(MXTBLD, save.TBPOOL.as_slice_mut(), ctx)?;

        save.FTHEAD = 0;
        save.TBHEAD = 0;

        save.FIRST = false;
    }

    //
    // Find the table.  If there's no match, the number of loaded columns
    // is zero.
    //
    save.TBCURR = save.TBHEAD;
    save.FND = false;

    while ((save.TBCURR > 0) && !save.FND) {
        if EQSTR(TABLE, &save.TBNAMS[save.TBCURR]) {
            save.FND = true;
        } else {
            save.TBCURR = LNKNXT(save.TBCURR, save.TBPOOL.as_slice(), ctx)?;
        }
    }

    if !save.FND {
        SETMSG(b"The table # is not currently loaded.", ctx);
        ERRCH(b"#", TABLE, ctx);
        SIGERR(b"SPICE(TABLENOTLOADED)", ctx)?;
        CHKOUT(b"EKCII", ctx)?;
        return Ok(());
    }

    //
    // Locate the named column in the column attribute table.
    //
    save.I = 0;
    save.COL = save.TBCTPT[save.TBCURR];

    while ((save.COL > 0) && (save.I < CINDEX)) {
        save.I = (save.I + 1);

        if (save.I == CINDEX) {
            //
            // We've found the column.  Set the output arguments using
            // its attributes.
            //
            fstr::assign(COLUMN, save.CTNAMS.get(save.COL));

            ATTDSC[1] = save.CTCLAS[save.COL];
            ATTDSC[2] = save.CTTYPS[save.COL];
            ATTDSC[3] = save.CTLENS[save.COL];
            ATTDSC[4] = save.CTSIZS[save.COL];

            if save.CTINDX[save.COL] {
                ATTDSC[5] = ITRUE;
            } else {
                ATTDSC[5] = IFALSE;
            }

            if save.CTNULL[save.COL] {
                ATTDSC[6] = ITRUE;
            } else {
                ATTDSC[6] = IFALSE;
            }

            CHKOUT(b"EKCII", ctx)?;
            return Ok(());
        } else {
            save.COL = LNKNXT(save.COL, save.CTPOOL.as_slice(), ctx)?;
        }
    }

    //
    // We end up here if we ran out of columns before finding the
    // CINDEXth one, or if CINDEX was non-positive.
    //
    SETMSG(
        b"Column indices for table # range from # to #; requested index was #.",
        ctx,
    );
    ERRCH(b"#", &save.TABNAM, ctx);
    ERRINT(b"#", intrinsics::MAX0(&[1, save.I]), ctx);
    ERRINT(b"#", save.I, ctx);
    ERRINT(b"#", CINDEX, ctx);
    SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;

    CHKOUT(b"EKCII", ctx)?;
    Ok(())
}

/// EK, search for events
///
/// Search for EK events matching a specified set of constraints.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  EQRYI      I   Integer component of encoded query.
///  EQRYC      I   Character component of encoded query.
///  EQRYD      I   D.p. component of encoded query.
///  NMROWS     O   Number of rows matching query constraints.
///  SEMERR     O   Flag indicating whether semantic error occurred.
///  ERRMSG     O   Message describing semantic error, if any.
/// ```
///
/// # Detailed Input
///
/// ```text
///  EQRYI,
///  EQRYC,
///  EQRYD    are, respectively, the integer, character, and
///           double precision portions of an encoded query.
///           The query must have been parsed and must have
///           its table and column names resolved. Time values
///           must have been resolved. The query is expected
///           to be semantically correct.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NMROWS   is the number of rows matching the input query
///           constraints.
///
///  SEMERR   is a logical flag indicating whether a semantic
///           error was detected while attempting to respond to
///           the input query.
///
///  ERRMSG   is a descriptive error message that is set if a
///           semantic error is detected. Otherwise, ERRMSG
///           is returned blank.
/// ```
///
/// # Parameters
///
/// ```text
///  LBCELL   is the SPICE cell lower bound.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If this routine is called when no files are loaded, the
///      error SPICE(NOLOADEDFILES) is signaled.
///
///  2)  If the structure of the input query is invalid, this routine
///      may fail in mysterious ways.
/// ```
///
/// # Files
///
/// ```text
///  See the header of EKQMGR for a description of files used
///  by this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  NAIF Toolkit-based applications will rarely need to call this
///  routine directly; the high-level routine EKFIND should normally
///  be used to query the EK system.
///
///  Because the structure of encoded queries is not part of the
///  SPICELIB user interface, we strongly recommend that users'
///  applications not call this routine directly.
/// ```
///
/// # Examples
///
/// ```text
///  See the header of the umbrella subroutine EKQMGR for a
///  comprehensive example of the use of EKQMGR's entry points.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine should normally not be called directly from
///      users' applications.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 27-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.1, 10-FEB-2014 (BVS)
///
///         Added description of LBCELL to the $Parameters section of the
///         header.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (NJB)
///
///         Bug fix: When an already loaded kernel is opened with EKOPR,
///         it now has its link count reset to 1 via a call to EKCLS.
///
/// -    SPICELIB Version 1.2.0, 21-JUL-1998 (NJB)
///
///         ZZEKJSQZ call was added after the ZZEKJOIN call. This change
///         reduces the scratch area usage for intermediate results of
///         joins. It also prevents ZZEKJOIN from being handed a join
///         row set containing a segment vector having no corresponding
///         row vectors.
///
///         Removed a comment in the join loop indicating that non-join
///         constraints involving comparisons of column entries in the
///         table were being activated. This comment was incorrect; the
///         constraints in question were applied earlier.
///
/// -    SPICELIB Version 1.0.1, 07-JUL-1996 (NJB)
///
///         Previous version line was changed from "Beta" to "SPICELIB."
///
/// -    SPICELIB Version 1.0.0, 23-OCT-1995 (NJB)
/// ```
pub fn eksrch(
    ctx: &mut SpiceContext,
    eqryi: &[i32],
    eqryc: &str,
    eqryd: &[f64],
    nmrows: &mut i32,
    semerr: &mut bool,
    errmsg: &mut str,
) -> crate::Result<()> {
    EKSRCH(
        eqryi,
        eqryc.as_bytes(),
        eqryd,
        nmrows,
        semerr,
        fstr::StrBytes::new(errmsg).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKSRCH ( EK, search for events )
pub fn EKSRCH(
    EQRYI: &[i32],
    EQRYC: &[u8],
    EQRYD: &[f64],
    NMROWS: &mut i32,
    SEMERR: &mut bool,
    ERRMSG: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let EQRYD = DummyArray::new(EQRYD, 1..);
    let EQRYI = DummyArray::new(EQRYI, LBCELL..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"EKSRCH", ctx)?;
    }

    //
    // There nothing to search if no files are loaded.  A sure
    // symptom of this problem is that the file list is empty.
    //
    if (save.FTHEAD <= 0) {
        SETMSG(b"No E-kernels are currently loaded.", ctx);
        SIGERR(b"SPICE(NOLOADEDFILES)", ctx)?;
        CHKOUT(b"EKSRCH", ctx)?;
        return Ok(());
    }

    //
    // No error to begin with.
    //
    *SEMERR = false;
    fstr::assign(ERRMSG, b" ");
    *NMROWS = 0;

    if save.FIRST {
        //
        // Initialize the file table pool, segment table pool, column
        // descriptor pool, column table pool, and table list pool.
        //
        LNKINI(FTSIZE, save.FTPOOL.as_slice_mut(), ctx)?;
        LNKINI(STSIZE, save.STPOOL.as_slice_mut(), ctx)?;
        LNKINI(DTSIZE, save.DTPOOL.as_slice_mut(), ctx)?;
        LNKINI(CTSIZE, save.CTPOOL.as_slice_mut(), ctx)?;
        LNKINI(MXTBLD, save.TBPOOL.as_slice_mut(), ctx)?;

        save.FTHEAD = 0;
        save.TBHEAD = 0;

        save.FIRST = false;
    }

    //
    // Read some of our favorite things from the query.  We need:
    //
    //    - the table count
    //    - the SELECT clause column count
    //    - the order-by column count
    //    - the table and alias list
    //
    ZZEKREQI(EQRYI.as_slice(), b"NUM_TABLES", &mut save.NTAB, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"NUM_SELECT_COLS", &mut save.NSEL, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"NUM_ORDERBY_COLS", &mut save.NORDER, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            ZZEKQTAB(
                EQRYI.as_slice(),
                EQRYC,
                save.I,
                &mut save.FRMTAB[save.I],
                &mut save.FRMALS[save.I],
                ctx,
            )?;
            save.I += m3__;
        }
    }

    //
    // Initialize the table vectors.  Also initialize a vector of column
    // list pointers.
    //
    SSIZEC(MAXTAB, save.TABVEC.as_arg_mut(), ctx)?;
    SSIZEI(MAXTAB, save.TPTVEC.as_slice_mut(), ctx)?;

    //
    // Fill in the FROM table vector and corresponding column pointer
    // vector.  It's an error if a table referenced in the FROM clause
    // can't be found.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Find the table list entry for this table name.
            //
            save.TBCURR = save.TBHEAD;
            save.FND = false;

            while ((save.TBCURR > 0) && !save.FND) {
                if fstr::eq(save.TBNAMS.get(save.TBCURR), save.FRMTAB.get(save.I)) {
                    //
                    // We've found the table list entry for the Ith table.
                    //
                    APPNDC(&save.FRMTAB[save.I], save.TABVEC.as_arg_mut(), ctx)?;
                    APPNDI(save.TBCURR, save.TPTVEC.as_slice_mut(), ctx)?;
                    save.FND = true;
                } else {
                    save.TBCURR = LNKNXT(save.TBCURR, save.TBPOOL.as_slice(), ctx)?;
                }
            }

            if !save.FND {
                SETMSG(b"The table # is not currently loaded.", ctx);
                ERRCH(b"#", &save.FRMTAB[save.I], ctx);
                SIGERR(b"SPICE(INVALIDTABLENAME)", ctx)?;
                CHKOUT(b"EKSRCH", ctx)?;
                return Ok(());
            }

            save.I += m3__;
        }
    }

    //
    // Since this is a new search, re-initialize the stack in the EK
    // scratch area.  Also initialize our total segment list count.
    //
    ZZEKSTOP(&mut save.TOP, ctx);
    ZZEKSDEC(save.TOP, ctx)?;

    //
    // Initialize the size of the join row set union for the current
    // query.  At this point, no matching rows have been found.
    //
    save.USIZE = 0;
    save.UNROWS = 0;

    //
    // Get the number of conjunctions and the sizes of the conjunctions.
    //
    ZZEKREQI(EQRYI.as_slice(), b"NUM_CONJUNCTIONS", &mut save.NCONJ, ctx)?;
    CLEARI(MAXCON, save.SIZES.as_slice_mut());

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCONJ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            ZZEKQCNJ(EQRYI.as_slice(), save.I, &mut save.SIZES[save.I], ctx)?;
            save.I += m3__;
        }
    }

    //
    // For each conjunction of constraints, we'll build a join row
    // set representing the row vectors matching those constraints.
    // The final result will be a join row set union representing the
    // row vectors satisfying at least one conjunction.
    //
    // We want to build a join row set even if there are *no*
    // constraints.  Therefore, we always make at least one pass
    // through the loop below.
    //
    save.CJEND = 0;

    for CONJ in 1..=intrinsics::MAX0(&[1, save.NCONJ]) {
        //
        // Our objective is to build a join row set representing the table
        // defined by the FROM columns and the input constraints.  To do
        // this, we'll first build a trivial join row set for each table;
        // this join row set represents the rows that satisfy constraints
        // on columns in that table.  Having done this, we'll produce a
        // final (for this conjunction) join row set that represents the
        // constrained join of the FROM tables.  The base address of this
        // join row set will be stored in the array UBASE.
        //
        // We'll start out by recording the FROM table indices and column
        // list indices of columns listed in the constraints.
        //
        if (save.NCONJ == 0) {
            save.CJSIZE = 0;
        } else {
            save.CJSIZE = save.SIZES[CONJ];
        }

        save.CJBEG = (save.CJEND + 1);
        save.CJEND = (save.CJEND + save.CJSIZE);

        {
            let m1__: i32 = 1;
            let m2__: i32 = save.CJSIZE;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                ZZEKQCON(
                    EQRYI.as_slice(),
                    EQRYC,
                    EQRYD.as_slice(),
                    ((save.CJBEG + save.I) - 1),
                    &mut save.CNSTYP[save.I],
                    &mut save.LTNAME,
                    &mut save.LTBIDX[save.I],
                    &mut save.LCNAME,
                    &mut save.LCIDX[save.I],
                    &mut save.OPS[save.I],
                    &mut save.RTNAME,
                    &mut save.RTBIDX[save.I],
                    &mut save.RCNAME,
                    &mut save.RCIDX[save.I],
                    &mut save.DTYPE[save.I],
                    &mut save.CBEGS[save.I],
                    &mut save.CENDS[save.I],
                    &mut save.DVALS[save.I],
                    &mut save.IVALS[save.I],
                    ctx,
                )?;
                save.I += m3__;
            }
        }

        for T in 1..=save.NTAB {
            //
            // We will build a trivial (one-table) join row set for the
            // current table.
            //
            // Initialize the join row set.  Retain the base address.  We
            // can fill in the table count right away; the count is 1.
            //
            ZZEKSTOP(&mut save.RBAS[T], ctx);

            {
                let m1__: i32 = 1;
                let m2__: i32 = JSCIDX;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    ZZEKSPSH(1, &[0], ctx)?;
                    save.I += m3__;
                }
            }

            ZZEKSUPD((save.RBAS[T] + JTCIDX), (save.RBAS[T] + JTCIDX), &[1], ctx)?;

            //
            // Count the loaded segments for the current table.  We'll
            // leave enough room in the join row set for each segment.
            //
            save.TAB = save.TPTVEC[T];
            save.I = save.TBSTPT[save.TAB];
            save.NSV = 0;

            while (save.I > 0) {
                ZZEKSPSH(1, &[0], ctx)?;

                save.NSV = (save.NSV + 1);
                save.I = LNKNXT(save.I, save.STPOOL.as_slice(), ctx)?;
            }

            //
            // Save room for the row vector base addresses and counts.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = (2 * save.NSV);
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    ZZEKSPSH(1, &[0], ctx)?;
                    save.I += m3__;
                }
            }

            //
            // At this point, we can set the segment vector count in the
            // join row set.
            //
            ZZEKSUPD(
                (save.RBAS[T] + JSCIDX),
                (save.RBAS[T] + JSCIDX),
                &[save.NSV],
                ctx,
            )?;

            //
            // Find the matching rows in the segments belonging to the
            // current table.
            //
            save.SEG = save.TBSTPT[save.TAB];
            save.NSEG = 0;
            save.RTOTAL = 0;

            while (save.SEG > 0) {
                save.NSEG = (save.NSEG + 1);
                //
                // The segment vector for this segment is trivial:  it's
                // just the segment's index in the segment table.
                //
                save.SGVBAS = ((save.RBAS[T] + JSVBAS) + (save.NSEG - 1));

                ZZEKSUPD((save.SGVBAS + 1), (save.SGVBAS + 1), &[save.SEG], ctx)?;

                //
                // Label as `inactive' any constraints that don't apply to
                // this table.  Join constraints are inactive at this stage
                // of the game.  Label all other constraints `active'.
                // We'll keep track of column and value constraints
                // separately.
                //
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = save.CJSIZE;
                    let m3__: i32 = 1;
                    save.I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        //
                        // Each constraint is active to start with.
                        //
                        save.ACTIVC[save.I] = (save.CNSTYP[save.I] == EQCOL);
                        save.ACTIVV[save.I] = (save.CNSTYP[save.I] == EQVAL);

                        //
                        // The parent table of the LHS column must be the Tth
                        // table, or this constraint does not apply.
                        //
                        // We'll also exclude join constraints.  Note that
                        // constraints comparing values from two columns need not
                        // be join constraints:  it's possible that the column on
                        // the right belongs to the same FROM table as the
                        // column on the left.
                        //
                        if (save.LTBIDX[save.I] != T) {
                            save.ACTIVC[save.I] = false;
                            save.ACTIVV[save.I] = false;
                        } else if (save.CNSTYP[save.I] == EQCOL) {
                            if (save.LTBIDX[save.I] != save.RTBIDX[save.I]) {
                                //
                                // This is a join constraint; disable it.
                                //
                                save.ACTIVC[save.I] = false;
                            }
                        }

                        save.I += m3__;
                    }
                }

                //
                // At this point, we'll have to search the segment for
                // matching rows.  Pick a key column for the segment.  To
                // do this, we'll need to pack an array with column
                // descriptors for each active constraint.  The
                // descriptor for the column on the left side of the Ith
                // constraint will be placed in elements LDSCRS(*,I), if
                // the Ith constraint is active.
                //
                CLEARI((CDSCSZ * MAXCON), save.LDSCRS.as_slice_mut());

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = save.CJSIZE;
                    let m3__: i32 = 1;
                    save.I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        if save.ACTIVV[save.I] {
                            //
                            // Look up the column descriptor for this
                            // constraint.
                            //
                            save.J = save.STDTPT[save.SEG];

                            {
                                let m1__: i32 = 2;
                                let m2__: i32 = save.LCIDX[save.I];
                                let m3__: i32 = 1;
                                save.K = m1__;
                                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                    save.J = LNKNXT(save.J, save.DTPOOL.as_slice(), ctx)?;
                                    save.K += m3__;
                                }
                            }

                            MOVEI(
                                save.DTDSCS.subarray([1, save.J]),
                                CDSCSZ,
                                save.LDSCRS.subarray_mut([1, save.I]),
                            );
                        }

                        save.I += m3__;
                    }
                }

                ZZEKKEY(
                    save.STHAN[save.SEG],
                    save.STDSCS.subarray([1, save.SEG]),
                    save.STNROW[save.SEG],
                    save.CJSIZE,
                    save.LCIDX.as_slice(),
                    save.LDSCRS.as_slice(),
                    save.OPS.as_slice(),
                    save.DTYPE.as_slice(),
                    EQRYC,
                    save.CBEGS.as_slice(),
                    save.CENDS.as_slice(),
                    save.DVALS.as_slice(),
                    save.IVALS.as_slice(),
                    save.ACTIVV.as_slice_mut(),
                    &mut save.KEY,
                    save.KEYDSC.as_slice_mut(),
                    &mut save.BEGIDX,
                    &mut save.ENDIDX,
                    &mut save.KEYFND,
                    ctx,
                )?;

                //
                // ZZEKKEY has updated ACTIVV to reflect the application
                // of constraints that were used to determine BEGIDX and
                // ENDIDX.
                //
                if save.KEYFND {
                    save.INDEXD = true;
                } else {
                    //
                    // A key column could not be determined from the
                    // active constraints.  We'll use the first column of
                    // the segment as the key column.
                    //
                    save.INDEXD = false;
                    save.BEGIDX = 1;
                    save.ENDIDX = save.STNROW[save.SEG];
                }

                //
                // Whether or not we have any matching rows, we'll need
                // to record how many we have.  Save the offset from the
                // join row set base for the pointer to the row vectors.
                // The row vector count follows this pointer.
                //
                save.PTROFF = (((JSVBAS + save.NSV) + ((save.NSEG - 1) * 2)) + 1);

                if (save.ENDIDX >= save.BEGIDX) {
                    //
                    // Initialize the count of matching rows for this
                    // segment.  The current stack top is the base address
                    // for the row vectors; save the offset of this
                    // address from the join row set's base.
                    // Also compute the base address of the segment vector
                    // for the current segment.
                    //
                    save.NMATCH = 0;
                    ZZEKSTOP(&mut save.RWVBAS, ctx);

                    ZZEKSUPD(
                        (save.RBAS[T] + save.PTROFF),
                        (save.RBAS[T] + save.PTROFF),
                        &[(save.RWVBAS - save.RBAS[T])],
                        ctx,
                    )?;

                    //
                    // Count the active constraints.  While we're at it,
                    // fill in the descriptor lists LDSCRS and RDSCRS
                    // with, respectively, the descriptors for the columns
                    // on the left hand sides and right hand sides of
                    // these constraints.
                    //
                    CLEARI((CDSCSZ * MAXCON), save.LDSCRS.as_slice_mut());
                    CLEARI((CDSCSZ * MAXCON), save.RDSCRS.as_slice_mut());
                    save.NACT = 0;

                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = save.CJSIZE;
                        let m3__: i32 = 1;
                        save.I = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            if (save.ACTIVC[save.I] || save.ACTIVV[save.I]) {
                                save.NACT = (save.NACT + 1);
                                //
                                // Look up the column descriptor for this
                                // constraint.

                                save.J = save.STDTPT[save.SEG];

                                {
                                    let m1__: i32 = 2;
                                    let m2__: i32 = save.LCIDX[save.I];
                                    let m3__: i32 = 1;
                                    save.K = m1__;
                                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                        save.J = LNKNXT(save.J, save.DTPOOL.as_slice(), ctx)?;
                                        save.K += m3__;
                                    }
                                }

                                MOVEI(
                                    save.DTDSCS.subarray([1, save.J]),
                                    CDSCSZ,
                                    save.LDSCRS.subarray_mut([1, save.I]),
                                );

                                save.J = save.STDTPT[save.SEG];

                                {
                                    let m1__: i32 = 2;
                                    let m2__: i32 = save.RCIDX[save.I];
                                    let m3__: i32 = 1;
                                    save.K = m1__;
                                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                        save.J = LNKNXT(save.J, save.DTPOOL.as_slice(), ctx)?;
                                        save.K += m3__;
                                    }
                                }

                                MOVEI(
                                    save.DTDSCS.subarray([1, save.J]),
                                    CDSCSZ,
                                    save.RDSCRS.subarray_mut([1, save.I]),
                                );
                            }

                            save.I += m3__;
                        }
                    }

                    if (save.NACT > 0) {
                        //
                        // There are still active constraints left, so
                        // proceed linearly through the remaining rows,
                        // testing each one against these constraints. Add
                        // matching rows to the current join row set.
                        //
                        {
                            let m1__: i32 = save.BEGIDX;
                            let m2__: i32 = save.ENDIDX;
                            let m3__: i32 = 1;
                            save.R = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                if save.INDEXD {
                                    ZZEKIXLK(
                                        save.STHAN[save.SEG],
                                        save.KEYDSC.as_slice(),
                                        save.R,
                                        &mut save.ROWIDX,
                                        ctx,
                                    )?;
                                } else {
                                    //
                                    // Look up the record pointer for row R.
                                    //
                                    ZZEKRPLK(
                                        save.STHAN[save.SEG],
                                        save.STDSCS.subarray([1, save.SEG]),
                                        save.R,
                                        &mut save.ROWIDX,
                                        ctx,
                                    )?;
                                }

                                //
                                // Test the row against both value and column
                                // constraints.  For now, we supply an array
                                // of default column entry element indices.
                                //
                                save.VMTCH = ZZEKRMCH(
                                    save.CJSIZE,
                                    save.ACTIVV.as_slice(),
                                    save.STHAN[save.SEG],
                                    save.STDSCS.subarray([1, save.SEG]),
                                    save.LDSCRS.as_slice(),
                                    save.ROWIDX,
                                    save.LELTS.as_slice(),
                                    save.OPS.as_slice(),
                                    save.DTYPE.as_slice(),
                                    EQRYC,
                                    save.CBEGS.as_slice(),
                                    save.CENDS.as_slice(),
                                    save.DVALS.as_slice(),
                                    save.IVALS.as_slice(),
                                    ctx,
                                )?;

                                save.CMTCH = true;

                                //
                                // Note that ZZEKVMCH expects a set of inputs
                                // that are not really parallel to those
                                // expected by ZZEKRMCH.  We feed the
                                // column comparison constraints to ZZEKVMCH
                                // one at a time.
                                //
                                {
                                    let m1__: i32 = 1;
                                    let m2__: i32 = save.CJSIZE;
                                    let m3__: i32 = 1;
                                    save.J = m1__;
                                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                        save.CMTCH = (save.CMTCH
                                            && ZZEKVMCH(
                                                1,
                                                save.ACTIVC.subarray(save.J),
                                                save.STHAN.subarray(save.SEG),
                                                save.STDSCS.subarray([1, save.SEG]),
                                                save.LDSCRS.subarray([1, save.J]),
                                                &[save.ROWIDX],
                                                &[1],
                                                save.OPS.subarray(save.J),
                                                save.STHAN.subarray(save.SEG),
                                                save.STDSCS.subarray([1, save.SEG]),
                                                save.RDSCRS.subarray([1, save.J]),
                                                &[save.ROWIDX],
                                                &[1],
                                                ctx,
                                            )?);
                                        save.J += m3__;
                                    }
                                }

                                if (save.CMTCH && save.VMTCH) {
                                    //
                                    // Push the `augmented row vector' for the
                                    // current row onto the stack.  In this case,
                                    // of course, the augmented row vector is
                                    // trivial:  it consists of the row number,
                                    // followed by the base address of the parent
                                    // segment vector.
                                    //
                                    save.NMATCH = (save.NMATCH + 1);

                                    ZZEKSPSH(1, &[save.ROWIDX], ctx)?;
                                    ZZEKSPSH(1, &[(save.SGVBAS - save.RBAS[T])], ctx)?;
                                }

                                save.R += m3__;
                            }
                        }
                    } else {
                        //
                        // All the rows indicated by BEGIDX and ENDIDX
                        // match the constraints.  This code section should
                        // be upgraded to transfer the row numbers in
                        // chunks.
                        //
                        save.NMATCH = ((save.ENDIDX - save.BEGIDX) + 1);

                        {
                            let m1__: i32 = save.BEGIDX;
                            let m2__: i32 = save.ENDIDX;
                            let m3__: i32 = 1;
                            save.R = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                if save.INDEXD {
                                    //
                                    // Look up the record pointer for row R
                                    // from the column index.
                                    //
                                    ZZEKIXLK(
                                        save.STHAN[save.SEG],
                                        save.KEYDSC.as_slice(),
                                        save.R,
                                        &mut save.ROWIDX,
                                        ctx,
                                    )?;
                                } else {
                                    //
                                    // Look up the record pointer for row R.
                                    //
                                    ZZEKRPLK(
                                        save.STHAN[save.SEG],
                                        save.STDSCS.subarray([1, save.SEG]),
                                        save.R,
                                        &mut save.ROWIDX,
                                        ctx,
                                    )?;
                                }

                                ZZEKSPSH(1, &[save.ROWIDX], ctx)?;
                                ZZEKSPSH(1, &[(save.SGVBAS - save.RBAS[T])], ctx)?;

                                save.R += m3__;
                            }
                        }
                    }

                    //
                    // Fill in the row count for this segment in the join row
                    // set.
                    //
                    ZZEKSUPD(
                        ((save.RBAS[T] + save.PTROFF) + 1),
                        ((save.RBAS[T] + save.PTROFF) + 1),
                        &[save.NMATCH],
                        ctx,
                    )?;
                }

                //
                // Take a look at the next segment.  Update the total count
                // of matching rows for this table.
                //
                save.SEG = LNKNXT(save.SEG, save.STPOOL.as_slice(), ctx)?;
                save.RTOTAL = (save.RTOTAL + save.NMATCH);
            }

            //
            // Fill in the size and count information for the join row set.
            //
            ZZEKSTOP(&mut save.TOP, ctx);
            save.RSIZE[T] = (save.TOP - save.RBAS[T]);

            ZZEKSUPD(
                (save.RBAS[T] + JSZIDX),
                (save.RBAS[T] + JSZIDX),
                save.RSIZE.subarray(T),
                ctx,
            )?;
            ZZEKSUPD(
                (save.RBAS[T] + JRCIDX),
                (save.RBAS[T] + JRCIDX),
                &[save.RTOTAL],
                ctx,
            )?;

            //
            // Compress out any empty segment vectors from the join row
            // set.
            //
            ZZEKJSQZ(save.RBAS[T], ctx)?;

            //
            // At this point, we've filled in the entire join row set for
            // table T.
            //
        }

        //
        // Join the trivial join row sets, producing a final join row set
        // for the current conjunction.  Retain the base address of this
        // join row set, if it is non-empty.  Update the size of the join
        // row set union.
        //
        //
        save.RESBAS = save.RBAS[1];

        for T in 2..=save.NTAB {
            //
            // Arm the join constraints!  Turn on the constraints that
            // have the Tth table on the one side, and tables
            // 1, 2, ... , T on the other.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.CJSIZE;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.ACTIVC[save.I] = false;

                    if (save.CNSTYP[save.I] == EQCOL) {
                        save.L = save.LTBIDX[save.I];
                        save.R = save.RTBIDX[save.I];

                        if ((((((save.L >= 1) && (save.L <= T)) && (save.R >= 1))
                            && (save.R <= T))
                            && (save.L != save.R))
                            && ((save.R == T) || (save.L == T)))
                        {
                            save.ACTIVC[save.I] = true;
                        }
                    }

                    save.I += m3__;
                }
            }

            //
            // The base address of the first join row set is the base
            // address of the result of the previous join.  The first time
            // through, the base of the join row set for table 1 is used.
            //
            if (T == 2) {
                save.JBASE1 = save.RBAS[1];
            } else {
                save.JBASE1 = save.RESBAS;
            }

            save.JBASE2 = save.RBAS[T];

            ZZEKJOIN(
                save.JBASE1,
                save.JBASE2,
                save.CJSIZE,
                save.ACTIVC.as_slice(),
                save.LTBIDX.as_slice(),
                save.LCIDX.as_slice(),
                save.LELTS.as_slice(),
                save.OPS.as_slice(),
                save.RTBIDX.as_slice(),
                save.RCIDX.as_slice(),
                save.RELTS.as_slice(),
                save.STHAN.as_slice(),
                save.STDSCS.as_slice(),
                save.STDTPT.as_slice(),
                save.DTPOOL.as_slice(),
                save.DTDSCS.as_slice(),
                &mut save.RESBAS,
                &mut save.JSIZE,
                ctx,
            )?;

            ZZEKJSQZ(save.RESBAS, ctx)?;
        }

        //
        // At this point, we've found the matching rows for the current
        // query conjunction.  Update the size of the join row set union
        // corresponding to the current query.  Save the base address of
        // the final join row set.  Update the total number of matching
        // rows in the join row set union.
        //
        save.USIZE = (save.USIZE + 1);
        save.UBASE[save.USIZE] = save.RESBAS;

        ZZEKSRD(
            (save.RESBAS + JRCIDX),
            (save.RESBAS + JRCIDX),
            std::slice::from_mut(&mut save.CJROWS),
            ctx,
        )?;

        save.UNROWS = (save.UNROWS + save.CJROWS);

        //
        // Remove redundant row vectors from the join row set union.
        // These row vectors may arise in the execution of queries whose
        // WHERE clauses contain multiple conjunctions.
        //
        ZZEKWEED(
            &mut save.USIZE,
            save.UBASE.as_slice_mut(),
            &mut save.UNROWS,
            ctx,
        )?;

        //
        // Initialize the addressing function for the current join row
        // set union.
        //
        if (save.USIZE > 0) {
            ZZEKVSET(save.USIZE, save.UBASE.as_slice(), ctx)?;
        }
    }

    //
    // At this point, we've formed the join row set union that
    // represents the set of row vectors matching the entire query.
    //
    *NMROWS = save.UNROWS;

    //
    // Get the tables and columns of from the SELECT clause.  For
    // each qualifying table, we need the index in the FROM clause
    // of that table.  For each column, we need the column table
    // index.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEL;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            ZZEKQSEL(
                EQRYI.as_slice(),
                EQRYC,
                save.I,
                &mut save.LXBEG,
                &mut save.LXEND,
                &mut save.TABNAM,
                &mut save.TABIDX,
                &mut save.COLNAM,
                &mut save.K,
                ctx,
            )?;

            //
            // Locate the column's attribute information.  Retain the column's
            // index within the parent table's column list.
            //
            save.TAB = save.TPTVEC[save.TABIDX];
            save.J = save.TBCTPT[save.TAB];
            save.COL = 0;
            save.FND = false;

            while ((save.J > 0) && !save.FND) {
                save.COL = (save.COL + 1);

                if fstr::eq(save.CTNAMS.get(save.J), &save.COLNAM) {
                    save.FND = true;
                } else {
                    save.J = LNKNXT(save.J, save.CTPOOL.as_slice(), ctx)?;
                }
            }

            if !save.FND {
                SETMSG(b"# is not name of a column in FROM table #.", ctx);
                ERRCH(b"#", &save.COLNAM, ctx);
                ERRINT(b"#", save.TABIDX, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"EKSRCH", ctx)?;
                return Ok(());
            }

            save.SELCTP[save.I] = save.J;
            save.SELCOL[save.I] = save.COL;
            save.SELTAB[save.I] = save.TABIDX;

            save.I += m3__;
        }
    }

    //
    // Enable sorting of the matching row vectors, if necessary.  The
    // first fetch request will invoke the sort.
    //
    save.DOSORT = ((save.NORDER > 0) && (*NMROWS > 0));
    save.SORTED = false;

    if save.DOSORT {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NORDER;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                ZZEKQORD(
                    EQRYI.as_slice(),
                    EQRYC,
                    save.I,
                    &mut save.TABNAM,
                    &mut save.OTABS[save.I],
                    &mut save.COLNAM,
                    &mut save.OCOLS[save.I],
                    &mut save.SENSE[save.I],
                    ctx,
                )?;
                save.I += m3__;
            }
        }
    }

    CHKOUT(b"EKSRCH", ctx)?;
    Ok(())
}

/// EK, get number of elements in column entry
///
/// Return the number of elements in a specified column entry in
/// the current row.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  SELIDX     I   Index of parent column in SELECT clause.
///  ROW        I   Row containing element.
///  NELT       O   Number of elements in entry in current row.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SELIDX   is the SELECT clause index of the column to
///           fetch from.
///
///  ROW      is the index of the row containing the element.
///           This number refers to a member of the set of rows
///           matching a query. ROW must be in the range
///
///              1 : NMROWS
///
///           where NMROWS is the matching row count returned
///           by EKSRCH.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NELT     is the number of elements in the column entry
///           belonging to the specified column in the current
///           row.
///
///           Null entries in variable-size columns are
///           considered to have size 1.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If this routine is called when no E-kernels have been loaded,
///      the error SPICE(NOLOADEDFILES) is signaled.
///
///  2)  If SELIDX is outside of the range established by the
///      last query passed to EKSRCH, the error SPICE(INVALIDINDEX)
///      is signaled.
///
///  3)  If ROW is outside of the range established by the
///      last query passed to EKSRCH, the error SPICE(INVALIDINDEX)
///      is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the header of EKQMGR for a description of files used
///  by this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is meant to be used in conjunction with the EKQMGR
///  fetch entry points EKGC, EKGD, and EKGI. This routine
///  allows the caller of those routines to determine appropriate
///  loop bounds to use to fetch each column entry in the current row.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) This example demonstrates how to fetch integer, double
///     precision and character string values from a column when such
///     column corresponds to either a variable-size array or to a
///     static-size array.
///
///     Create an EK that contains a table TAB that has the following
///     columns:
///
///         Column name   Data Type   Size
///         -----------   ---------   ----
///         IARRAY        INT         3
///         DARRAY        DP          VARIABLE
///         CARRAY        CHR         VARIABLE
///
///     Issue the following query
///
///         QUERY = 'SELECT IARRAY, DARRAY, CARRAY FROM TAB'
///
///     to fetch and dump column values from the rows that satisfy the
///     query.
///
///
///     Example code begins here.
///
///
///           PROGRAM EKNELT_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Column Name Size (CNAMSZ)
///     C     and EK Query Limit Parameters (MAXQRY)
///     C
///           INCLUDE 'ekcnamsz.inc'
///           INCLUDE 'ekqlimit.inc'
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               RTRIM
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         EKNAME
///           PARAMETER           ( EKNAME  = 'eknelt_ex1.bdb' )
///
///           CHARACTER*(*)         TABLE
///           PARAMETER           ( TABLE   = 'TAB' )
///
///           INTEGER               CHRSLN
///           PARAMETER           ( CHRSLN = 5   )
///
///           INTEGER               COL1SZ
///           PARAMETER           ( COL1SZ = 3   )
///
///           INTEGER               DECLEN
///           PARAMETER           ( DECLEN = 200 )
///
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN = 1840 )
///
///           INTEGER               MXC2SZ
///           PARAMETER           ( MXC2SZ = 4   )
///
///           INTEGER               MXC3SZ
///           PARAMETER           ( MXC3SZ = 7   )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 40  )
///
///           INTEGER               NCOLS
///           PARAMETER           ( NCOLS  = 3   )
///
///           INTEGER               NROWS
///           PARAMETER           ( NROWS  = 4   )
///
///           INTEGER               STRSIZ
///           PARAMETER           ( STRSIZ  = 30 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(DECLEN)    CDECLS ( NCOLS  )
///           CHARACTER*(CNAMSZ)    CNAMES ( NCOLS  )
///           CHARACTER*(CHRSLN)    COL3   ( MXC3SZ )
///           CHARACTER*(STRSIZ)    CVALS  ( MXC3SZ )
///           CHARACTER*(ERRLEN)    ERRMSG
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(MAXQRY)    QUERY
///
///           DOUBLE PRECISION      COL2   ( MXC2SZ )
///           DOUBLE PRECISION      DVALS  ( MXC2SZ )
///
///           INTEGER               COL1   ( COL1SZ )
///           INTEGER               ELTIDX
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               IVALS ( COL1SZ )
///           INTEGER               J
///           INTEGER               NELT
///           INTEGER               NMROWS
///           INTEGER               NRESVC
///           INTEGER               RECNO
///           INTEGER               ROW
///           INTEGER               SEGNO
///           INTEGER               SELIDX
///
///           LOGICAL               ERROR
///           LOGICAL               FOUND
///           LOGICAL               ISNULL
///
///     C
///     C     Open a new EK file.  For simplicity, we will not
///     C     reserve any space for the comment area, so the
///     C     number of reserved comment characters is zero.
///     C     The variable IFNAME is the internal file name.
///     C
///           NRESVC  =  0
///           IFNAME  =  'Test EK/Created 13-JUN-2019'
///
///           CALL EKOPN ( EKNAME, IFNAME, NRESVC, HANDLE )
///
///     C
///     C     Set up the column names and declarations
///     C     for the TAB segment.  We'll index all of
///     C     the columns.
///     C
///           CNAMES(1) = 'IARRAY'
///           CDECLS(1) = 'DATATYPE = INTEGER, SIZE = 3'
///
///           CNAMES(2) = 'DARRAY'
///           CDECLS(2) = 'DATATYPE = DOUBLE PRECISION, ' //
///          .            'SIZE = VARIABLE'
///
///           CNAMES(3) = 'CARRAY'
///           CDECLS(3) = 'DATATYPE = CHARACTER*(5), ' //
///          .            'SIZE = VARIABLE'
///
///     C
///     C     Start the segment.
///     C
///           CALL EKBSEG ( HANDLE, TABLE,  NCOLS,
///          .              CNAMES, CDECLS, SEGNO )
///
///     C
///     C     At the records to the table.
///     C
///           DO I = 1, NROWS
///
///     C
///     C        Append a new record to the EK.
///     C
///              CALL EKAPPR ( HANDLE, SEGNO, RECNO )
///
///     C
///     C        Add 3 items to IARRAY
///     C
///              DO J = 1, COL1SZ
///                 COL1(J) =  I + J*100.D0
///              END DO
///
///              CALL EKACEI ( HANDLE,    SEGNO,  RECNO,
///          .                 CNAMES(1), COL1SZ, COL1,  .FALSE. )
///
///     C
///     C        Add I items to DARRAY
///     C
///              DO J = 1, I
///                 COL2(J) = J + I*200.D0
///              END DO
///
///              CALL EKACED ( HANDLE,    SEGNO, RECNO,
///          .                 CNAMES(2), I,     COL2,  .FALSE. )
///
///     C
///     C        Add 3+I items to CARRAY
///     C
///              DO J = 1, 3+I
///                 CALL REPMI ( 'ST#', '#', J + I*100, COL3(J) )
///              END DO
///
///              CALL EKACEC ( HANDLE,    SEGNO, RECNO,
///          .                 CNAMES(3), I+3,   COL3,  .FALSE. )
///
///           END DO
///
///     C
///     C     Close the file.
///     C
///           CALL EKCLS ( HANDLE )
///
///     C
///     C     Open the created file. Perform the query and show the
///     C     results.
///     C
///           CALL FURNSH ( EKNAME )
///
///           QUERY = 'SELECT IARRAY, DARRAY, CARRAY FROM TAB'
///
///     C
///     C     Query the EK system for data rows matching the
///     C     SELECT constraints.
///     C
///           CALL EKFIND ( QUERY, NMROWS, ERROR, ERRMSG )
///
///     C
///     C     Check whether an error occurred while processing the
///     C     SELECT clause. If so, output the error message.
///     C
///           IF ( ERROR ) THEN
///
///              WRITE(*,*) 'SELECT clause error: ', ERRMSG
///
///           ELSE
///
///              DO ROW = 1, NMROWS
///
///                 WRITE(*,*) ' '
///                 WRITE(*,'(A,I3)') 'ROW  = ', ROW
///
///     C
///     C           Fetch values from column IARRAY in the current
///     C           row.  Since IARRAY was the first column selected,
///     C           the selection index SELIDX is set to 1.
///     C
///                 SELIDX = 1
///                 ELTIDX = 1
///                 ISNULL = .FALSE.
///
///                 DO WHILE (       ( ELTIDX .LE.  COL1SZ )
///          .                 .AND. (        .NOT. ISNULL )  )
///     C
///     C              If the column entry is null, we'll be kicked
///     C              out of this loop after the first iteration.
///     C
///                    CALL EKGI ( SELIDX,         ROW,     ELTIDX,
///          .                     IVALS(ELTIDX),  ISNULL,  FOUND   )
///
///                    ELTIDX = ELTIDX + 1
///
///                 END DO
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE(*,'(A)') '   COLUMN = IARRAY: <Null>'
///
///                 ELSE
///
///                    WRITE(*,'(A,3I6)') '   COLUMN = IARRAY:',
///          .                            ( IVALS(I), I = 1, COL1SZ )
///
///                 END IF
///
///     C
///     C           Fetch values from column DARRAY in the current
///     C           row.  Since DARRAY contains variable-size array
///     C           elements, we call EKNELT to determine how many
///     C           elements to fetch.
///     C
///                 SELIDX = 2
///                 CALL EKNELT ( SELIDX, ROW, NELT )
///
///                 ELTIDX = 1
///                 ISNULL = .FALSE.
///
///                 DO WHILE (       ( ELTIDX .LE.  NELT   )
///          .                 .AND. (        .NOT. ISNULL )  )
///
///                    CALL EKGD ( SELIDX,         ROW,     ELTIDX,
///          .                     DVALS(ELTIDX),  ISNULL,  FOUND   )
///
///                    ELTIDX = ELTIDX + 1
///
///                 END DO
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE(*,'(A)') '   COLUMN = DARRAY: <Null>'
///
///                 ELSE
///
///                    WRITE(*,'(A,4F6.1)') '   COLUMN = DARRAY:',
///          .                              ( DVALS(I), I = 1, NELT )
///
///                 END IF
///
///     C
///     C           Fetch values from column CARRAY in the current
///     C           row.
///     C
///                 SELIDX = 3
///                 CALL EKNELT ( SELIDX, ROW, NELT )
///
///                 ELTIDX = 1
///                 ISNULL = .FALSE.
///
///                 DO WHILE (       ( ELTIDX .LE.  NELT   )
///          .                 .AND. (        .NOT. ISNULL )  )
///
///                    CALL EKGC ( SELIDX,         ROW,     ELTIDX,
///          .                     CVALS(ELTIDX),  ISNULL,  FOUND   )
///
///                    ELTIDX = ELTIDX + 1
///
///                 END DO
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE(*,'(A)') '   COLUMN = CARRAY: <Null>'
///
///                 ELSE
///
///                    WRITE(*,'(8A)') '   COLUMN = CARRAY: ',
///          .                     ( CVALS(I)(:RTRIM(CVALS(I))) //' ',
///          .                                         I = 1, NELT   )
///
///                 END IF
///
///              END DO
///
///     C
///     C     We either parsed the SELECT clause or had an error.
///     C
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     ROW  =   1
///        COLUMN = IARRAY:   101   201   301
///        COLUMN = DARRAY: 201.0
///        COLUMN = CARRAY: ST101 ST102 ST103 ST104
///
///     ROW  =   2
///        COLUMN = IARRAY:   102   202   302
///        COLUMN = DARRAY: 401.0 402.0
///        COLUMN = CARRAY: ST201 ST202 ST203 ST204 ST205
///
///     ROW  =   3
///        COLUMN = IARRAY:   103   203   303
///        COLUMN = DARRAY: 601.0 602.0 603.0
///        COLUMN = CARRAY: ST301 ST302 ST303 ST304 ST305 ST306
///
///     ROW  =   4
///        COLUMN = IARRAY:   104   204   304
///        COLUMN = DARRAY: 801.0 802.0 803.0 804.0
///        COLUMN = CARRAY: ST401 ST402 ST403 ST404 ST405 ST406 ST407
///
///
///     Note that after run completion, a new EK file exists in the
///     output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example from existing fragment.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (NJB)
///
///         Bug fix: When an already loaded kernel is opened with EKOPR,
///         it now has its link count reset to 1 via a call to EKCLS.
///
/// -    SPICELIB Version 1.2.0, 12-FEB-1999 (NJB)
///
///         Bug fix: There was a error handling branch that called CHKOUT
///         where CHKIN should have been called. This has been fixed.
///
/// -    SPICELIB Version 1.1.0, 09-JUL-1996 (NJB)
///
///         Bug fix: EKNELT now initiates a sort operation if sorted
///         outputs are required and EKNELT is called after query
///         resolution but before the fetch routines. Also, addressing
///         for sorted query results has been fixed.
///
///         Misspelling of "issued" was fixed. Previous version line was
///         changed from "Beta" to "SPICELIB."
///
///
/// -    SPICELIB Version 1.0.0, 23-OCT-1995 (NJB)
/// ```
pub fn eknelt(ctx: &mut SpiceContext, selidx: i32, row: i32, nelt: &mut i32) -> crate::Result<()> {
    EKNELT(selidx, row, nelt, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKNELT  ( EK, get number of elements in column entry )
pub fn EKNELT(SELIDX: i32, ROW: i32, NELT: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Use discovery check-in for speed.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // The request doesn't make sense if no files are loaded.  A sure
    // symptom of this problem is that the file list is empty.
    //
    if (save.FTHEAD <= 0) {
        CHKIN(b"EKNELT", ctx)?;
        SETMSG(b"No E-kernels are currently loaded.", ctx);
        SIGERR(b"SPICE(NOLOADEDFILES)", ctx)?;
        CHKOUT(b"EKNELT", ctx)?;
        return Ok(());
    }

    //
    // The row number must be valid, or we can't proceed.
    //
    if ((ROW < 1) || (ROW > save.UNROWS)) {
        CHKIN(b"EKNELT", ctx)?;
        SETMSG(
            b"Row indices for query result range from 1 to #; requested row index was #.",
            ctx,
        );
        ERRINT(b"#", save.UNROWS, ctx);
        ERRINT(b"#", ROW, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"EKNELT", ctx)?;
        return Ok(());
    }

    //
    // Make sure the SELECT clause column index is valid.
    //
    if ((SELIDX < 1) || (SELIDX > save.NSEL)) {
        CHKIN(b"EKNELT", ctx)?;
        SETMSG(
            b"The SELECT column index # is out of the valid range 1:#",
            ctx,
        );
        ERRINT(b"#", SELIDX, ctx);
        ERRINT(b"#", save.NTAB, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"EKNELT", ctx)?;
        return Ok(());
    }

    //
    // If it hasn't been done yet, and if it needs to be done, sort the
    // matching row vectors.
    //
    if save.DOSORT {
        ZZEKJSRT(
            save.USIZE,
            save.UBASE.as_slice(),
            save.NORDER,
            save.OTABS.as_slice(),
            save.OCOLS.as_slice(),
            save.OELTS.as_slice(),
            save.SENSE.as_slice(),
            save.STHAN.as_slice(),
            save.STDSCS.as_slice(),
            save.STDTPT.as_slice(),
            save.DTPOOL.as_slice(),
            save.DTDSCS.as_slice(),
            &mut save.ORDBAS,
            ctx,
        )?;

        save.DOSORT = false;
        save.SORTED = true;
    }

    //
    // Look up the segment vector and row vector for the current row.
    //
    if save.SORTED {
        ZZEKSRD(
            (save.ORDBAS + ROW),
            (save.ORDBAS + ROW),
            std::slice::from_mut(&mut save.I),
            ctx,
        )?;
        ZZEKVCAL(save.I, &mut save.RWVBAS, &mut save.SGVBAS, ctx)?;
    } else {
        ZZEKVCAL(ROW, &mut save.RWVBAS, &mut save.SGVBAS, ctx)?;
    }

    ZZEKSRD(
        (save.RWVBAS + 1),
        (save.RWVBAS + save.NTAB),
        save.ROWVEC.as_slice_mut(),
        ctx,
    )?;
    ZZEKSRD(
        (save.SGVBAS + 1),
        (save.SGVBAS + save.NTAB),
        save.SEGVEC.as_slice_mut(),
        ctx,
    )?;

    save.TABIDX = save.SELTAB[SELIDX];
    save.ROWIDX = save.ROWVEC[save.TABIDX];
    save.SEG = save.SEGVEC[save.TABIDX];
    save.COL = save.SELCOL[SELIDX];

    save.COLPTR = save.STDTPT[save.SEG];

    {
        let m1__: i32 = 2;
        let m2__: i32 = save.COL;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.COLPTR = LNKNXT(save.COLPTR, save.DTPOOL.as_slice(), ctx)?;
            save.I += m3__;
        }
    }

    *NELT = ZZEKESIZ(
        save.STHAN[save.SEG],
        save.STDSCS.subarray([1, save.SEG]),
        save.DTDSCS.subarray([1, save.COLPTR]),
        save.ROWIDX,
        ctx,
    )?;

    Ok(())
}

/// EK, get event data, character
///
/// Return an element of an entry in a column of character
/// type in a specified row.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  SELIDX     I   Index of parent column in SELECT clause.
///  ROW        I   Row to fetch from.
///  ELMENT     I   Index of element, within column entry, to fetch.
///  CDATA      O   Character string element of column entry.
///  NULL       O   Flag indicating whether column entry was null.
///  FOUND      O   Flag indicating whether column was present in row.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SELIDX   is the SELECT clause index of the column to
///           fetch from.
///
///  ROW      is the output row containing the entry to fetch
///           from.
///
///  ELMENT   is the index of the element of the column entry
///           to fetch. The normal range of ELMENT is from 1 to
///           the size of the column's entry, but ELMENT is
///           allowed to exceed the number of elements in the
///           column entry; if it does, FOUND is returned .FALSE.
///           This allows the caller to read data from the column
///           entry in a loop without checking the number of
///           available elements first.
///
///           Null values in variable-sized columns are
///           considered to have size 1.
/// ```
///
/// # Detailed Output
///
/// ```text
///  CDATA    is the requested element of the specified column
///           entry. If the entry is null, CDATA is undefined.
///
///           If CDATA is too short to accommodate the requested
///           column entry element, the element is truncated on
///           the right to fit CDATA. If CDATA is longer than
///           the element, CDATA is returned blank-padded on
///           the right.
///
///  NULL     is a logical flag indicating whether the entry
///           belonging to the specified column in the specified
///           row is null.
///
///  FOUND    is a logical flag indicating whether the specified
///           element was found. If the element does not exist,
///           FOUND is returned .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input argument ELMENT is less than 1, the error
///      SPICE(INVALIDINDEX) is signaled and FOUND is returned .FALSE.
///      However, ELMENT is allowed to be greater than the number of
///      elements in the specified column entry; this allows the caller
///      to read data from the column entry in a loop without checking
///      the number of available elements first. If ELMENT is greater
///      than the number of available elements, FOUND is returned
///      .FALSE.
///
///  2)  If SELIDX is outside of the range established by the
///      last query passed to EKSRCH, the error SPICE(INVALIDINDEX)
///      is signaled and FOUND is returned .FALSE.
///
///  3)  If the input argument ROW is less than 1 or greater than the
///      number of rows matching the query, the error
///      SPICE(INVALIDINDEX) is signaled and FOUND is returned .FALSE.
///
///  4)  If the specified column does not have character type, the
///      error SPICE(INVALIDTYPE) is signaled.
///
///  5)  If this routine is called when no E-kernels have been loaded,
///      the error SPICE(NOLOADEDFILES) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the header of EKQMGR for a description of files used
///  by this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows retrieval of data from character columns.
///
///  This routine returns one element at a time in order to save the
///  caller from imposing a limit on the size of the column entries
///  that can be handled.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Perform a query on an EK file that contains a database with
///     the Supplementary Engineering Data Records of the Viking
///     Project in order to retrieve the IMAGE_ID values (character
///     strings) that correspond to the images with IMAGE_NUMBER
///     smaller than a given value, ordered by IMAGE_NUMBER.
///
///
///     Use the EK kernel below to load the information from the
///     original Supplementary Engineering Data Record (SEDR) data
///     set generated by the Viking Project.
///
///        vo_sedr.bdb
///
///
///     Example code begins here.
///
///
///           PROGRAM EKGC_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Maximum length of an input query,
///     C     MAXQRY, and the maximum length of literal string
///     C     values, MAXSTR, from eklimit.inc.
///     C
///           INCLUDE 'ekqlimit.inc'
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         EKNAME
///           PARAMETER           ( EKNAME = 'vo_sedr.bdb' )
///
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN = 1840 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(MAXSTR)    CDATA
///           CHARACTER*(ERRLEN)    ERRMSG
///           CHARACTER*(MAXQRY)    QUERY
///
///           INTEGER               ELTIDX
///           INTEGER               NMROWS
///           INTEGER               ROWNO
///           INTEGER               SELIDX
///
///           LOGICAL               ERROR
///           LOGICAL               FOUND
///           LOGICAL               ISNULL
///
///     C
///     C     Open an EK file.
///     C
///           CALL FURNSH ( EKNAME )
///
///     C
///     C     The table 'VIKING_SEDR_DATA' has a column 'IMAGE_ID'
///     C     of scalar strings.
///     C
///     C     Define a set of constraints to perform a query on
///     C     all loaded EK files (the SELECT clause). In this
///     C     case select the column 'IMAGE_ID' from table
///     C     'VIKING_SEDR_DATA' sorted by 'IMAGE_NUMBER'.
///     C
///           QUERY = 'Select IMAGE_ID from VIKING_SEDR_DATA '
///          .   //   'where IMAGE_NUMBER < 25860000 '
///          .   //   'order by IMAGE_NUMBER'
///
///     C
///     C     Query the EK system for data rows matching the
///     C     SELECT constraints.
///     C
///           CALL EKFIND ( QUERY, NMROWS, ERROR, ERRMSG )
///
///     C
///     C     Check whether an error occurred while processing the
///     C     SELECT clause. If so, output the error message.
///     C
///           IF ( ERROR ) THEN
///
///              WRITE(*,*) 'SELECT clause error: ', ERRMSG
///
///           ELSE
///
///     C
///     C        Fetch the character data. We know the query returned
///     C        one column and the column contains only scalar data,
///     C        so the index of all elements is 1.
///     C
///              SELIDX = 1
///              ELTIDX = 1
///
///     C
///     C        Loop over each row found matching the query.
///     C
///              DO ROWNO = 1, NMROWS
///
///     C
///     C           Use EKGC to retrieve the string from
///     C
///                 CALL EKGC ( SELIDX,  ROWNO,  ELTIDX,
///          .                  CDATA,   ISNULL, FOUND   )
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE (*,'(A,I3,A)') 'Row ', ROWNO,
///          .                  ': Character data: <Null>'
///
///                 ELSE
///
///                    WRITE (*,'(A,I3,2A)') 'Row ', ROWNO,
///          .                  ': Character data: ', CDATA
///                 END IF
///
///              END DO
///
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Row   1: Character data: 168C09
///     Row   2: Character data: 168C10
///     Row   3: Character data: 168C11
///     Row   4: Character data: 168C12
///     Row   5: Character data: 169C01
///     Row   6: Character data: 169C02
///     Row   7: Character data: 169C03
///     Row   8: Character data: 169C04
///     Row   9: Character data: 169C05
///     Row  10: Character data: 169C09
///     Row  11: Character data: 169C11
///     Row  12: Character data: 169C19
///     Row  13: Character data: 169C23
///     Row  14: Character data: 169C25
///     Row  15: Character data: 169C26
///     Row  16: Character data: 169C30
///     Row  17: Character data: 169C32
///     Row  18: Character data: 169C33
///     Row  19: Character data: 169C37
///     Row  20: Character data: 169C39
///     Row  21: Character data: 169C40
///     Row  22: Character data: 169C44
///     Row  23: Character data: 169C46
///     Row  24: Character data: 169C47
///     Row  25: Character data: 169C51
///     Row  26: Character data: 169C53
///
///
///  2) This example demonstrates how to fetch string values from a
///     column in three different cases: single values, variable-size
///     arrays and static-size arrays.
///
///     Create an EK that contains a table TAB that has the following
///     columns:
///
///        Column name   Data Type   Size
///        -----------   ---------   ----
///        CHR_COL_1     CHR         1
///        CHR_COL_2     CHR         VARIABLE
///        CHR_COL_3     CHR         3
///
///     Issue the following query
///
///         QUERY = 'SELECT CHR_COL_1, CHR_COL_2, CHR_COL_3 FROM TAB'
///
///     to fetch and dump column values from the rows that satisfy the
///     query.
///
///
///     Example code begins here.
///
///
///           PROGRAM EKGC_EX2
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Column Name Size (CNAMSZ)
///     C     and EK Query Limit Parameters (MAXQRY)
///     C
///           INCLUDE 'ekcnamsz.inc'
///           INCLUDE 'ekqlimit.inc'
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               RTRIM
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         EKNAME
///           PARAMETER           ( EKNAME  = 'ekgc_ex2.bdb' )
///
///           CHARACTER*(*)         TABLE
///           PARAMETER           ( TABLE   = 'TAB' )
///
///           INTEGER               CHRSLN
///           PARAMETER           ( CHRSLN = 3   )
///
///           INTEGER               COL3SZ
///           PARAMETER           ( COL3SZ = 3   )
///
///           INTEGER               DECLEN
///           PARAMETER           ( DECLEN = 200 )
///
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN = 1840 )
///
///           INTEGER               MXC2SZ
///           PARAMETER           ( MXC2SZ = 8   )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 40  )
///
///           INTEGER               NCOLS
///           PARAMETER           ( NCOLS  = 3   )
///
///           INTEGER               NROWS
///           PARAMETER           ( NROWS  = 4   )
///
///           INTEGER               STRSIZ
///           PARAMETER           ( STRSIZ  = 30 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(DECLEN)    CDECLS ( NCOLS  )
///           CHARACTER*(CNAMSZ)    CNAMES ( NCOLS  )
///           CHARACTER*(STRSIZ)    COL1
///           CHARACTER*(CHRSLN)    COL2   ( MXC2SZ )
///           CHARACTER*(CHRSLN)    COL3   ( COL3SZ )
///           CHARACTER*(STRSIZ)    CVALS  ( MXC2SZ )
///           CHARACTER*(ERRLEN)    ERRMSG
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(MAXQRY)    QUERY
///
///           INTEGER               ELTIDX
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               J
///           INTEGER               NELT
///           INTEGER               NMROWS
///           INTEGER               NRESVC
///           INTEGER               RECNO
///           INTEGER               ROW
///           INTEGER               SEGNO
///           INTEGER               SELIDX
///
///           LOGICAL               ERROR
///           LOGICAL               FOUND
///           LOGICAL               ISNULL
///
///     C
///     C     Open a new EK file.  For simplicity, we will not
///     C     reserve any space for the comment area, so the
///     C     number of reserved comment characters is zero.
///     C     The variable IFNAME is the internal file name.
///     C
///           NRESVC  =  0
///           IFNAME  =  'Test EK/Created 13-JUN-2019'
///
///           CALL EKOPN ( EKNAME, IFNAME, NRESVC, HANDLE )
///
///     C
///     C     Set up the column names and declarations
///     C     for the TAB segment.  We'll index all of
///     C     the columns.
///     C
///           CNAMES(1) = 'CHR_COL_1'
///           CDECLS(1) = 'DATATYPE = CHARACTER*(*), ' //
///          .            'INDEXED  = TRUE'
///
///           CNAMES(2) = 'CHR_COL_2'
///           CDECLS(2) = 'DATATYPE = CHARACTER*(3), ' //
///          .            'SIZE = VARIABLE'
///
///           CNAMES(3) = 'CHR_COL_3'
///           CDECLS(3) = 'DATATYPE = CHARACTER*(3), ' //
///          .            'SIZE = 3'
///
///     C
///     C     Start the segment.
///     C
///           CALL EKBSEG ( HANDLE, TABLE,  NCOLS,
///          .              CNAMES, CDECLS, SEGNO )
///
///     C
///     C     At the records to the table.
///     C
///           DO I = 1, NROWS
///
///     C
///     C        Append a new record to the EK.
///     C
///              CALL EKAPPR ( HANDLE, SEGNO, RECNO )
///
///     C
///     C        Add CHR_COL_1
///     C
///              CALL REPMI ( 'Column #2 has $ elements.',
///          .                '$', I*2, COL1              )
///
///              CALL EKACEC ( HANDLE,    SEGNO, RECNO,
///          .                 CNAMES(1), 1,     COL1, .FALSE. )
///
///     C
///     C        Add I*2 items to CHR_COL_2
///     C
///              DO J = 1, I*2
///                 CALL INTSTR( J + I*100, COL2(J) )
///              END DO
///
///              CALL EKACEC ( HANDLE,    SEGNO, RECNO,
///          .                 CNAMES(2), I*2,   COL2, .FALSE. )
///
///     C
///     C        Add 3 items to CHR_COL_3
///     C
///              DO J = 1, 3
///                 CALL INTSTR( I + J*100, COL3(J) )
///              END DO
///
///              CALL EKACEC ( HANDLE,    SEGNO, RECNO,
///          .                 CNAMES(3), 3,     COL3, .FALSE. )
///
///           END DO
///
///     C
///     C     Close the file.
///     C
///           CALL EKCLS ( HANDLE )
///
///     C
///     C     Open the created file. Perform the query and show the
///     C     results.
///     C
///           CALL FURNSH ( EKNAME )
///
///           QUERY = 'SELECT CHR_COL_1, CHR_COL_2, CHR_COL_3 FROM TAB'
///
///     C
///     C     Query the EK system for data rows matching the
///     C     SELECT constraints.
///     C
///           CALL EKFIND ( QUERY, NMROWS, ERROR, ERRMSG )
///
///     C
///     C     Check whether an error occurred while processing the
///     C     SELECT clause. If so, output the error message.
///     C
///           IF ( ERROR ) THEN
///
///              WRITE(*,*) 'SELECT clause error: ', ERRMSG
///
///           ELSE
///
///              DO ROW = 1, NMROWS
///
///                 WRITE(*,*) ' '
///                 WRITE(*,'(A,I3)') 'ROW  = ', ROW
///
///     C
///     C           Fetch values from column CHR_COL_1.  Since
///     C           CHR_COL_1 was the first column selected, the
///     C           selection index SELIDX is set to 1.
///     C
///                 SELIDX = 1
///                 ELTIDX = 1
///                 CALL EKGC ( SELIDX,    ROW,     ELTIDX,
///          .                  CVALS(1),  ISNULL,  FOUND   )
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE(*,*) '  COLUMN = CHR_COL_1: <Null>'
///
///                 ELSE
///
///                    WRITE(*,*) '  COLUMN = CHR_COL_1: ', CVALS(1)
///
///                 END IF
///
///     C
///     C           Fetch values from column CHR_COL_2 in the current
///     C           row.  Since CHR_COL_2 contains variable-size array
///     C           elements, we call EKNELT to determine how many
///     C           elements to fetch.
///     C
///                 SELIDX = 2
///                 CALL EKNELT ( SELIDX, ROW, NELT )
///
///                 ELTIDX = 1
///                 ISNULL = .FALSE.
///
///                 DO WHILE (       ( ELTIDX .LE.  NELT   )
///          .                 .AND. (        .NOT. ISNULL )  )
///
///                    CALL EKGC ( SELIDX,         ROW,     ELTIDX,
///          .                     CVALS(ELTIDX),  ISNULL,  FOUND   )
///
///                    ELTIDX = ELTIDX + 1
///
///     C
///     C           If the column entry is null, we'll be kicked
///     C           out of this loop after the first iteration.
///     C
///                 END DO
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE(*,*) '  COLUMN = CHR_COL_2: <Null>'
///
///                 ELSE
///
///                     WRITE(*,*) '  COLUMN = CHR_COL_2: ',
///          .                     ( CVALS(I)(:RTRIM(CVALS(I))) //' ',
///          .                                           I = 1, NELT )
///
///                 END IF
///
///     C
///     C           Fetch values from column CHR_COL_3 in the current
///     C           row.  We need not call EKNELT since we know how
///     C           many elements are in each column entry.
///     C
///                 SELIDX = 3
///                 ELTIDX = 1
///                 ISNULL = .FALSE.
///
///                 DO WHILE (       ( ELTIDX .LE.  COL3SZ )
///          .                 .AND. (        .NOT. ISNULL )  )
///
///                    CALL EKGC ( SELIDX,         ROW,     ELTIDX,
///          .                     CVALS(ELTIDX),  ISNULL,  FOUND   )
///
///                    ELTIDX = ELTIDX + 1
///
///                 END DO
///
///                 IF ( ISNULL ) THEN
///                    WRITE(*,*) '  COLUMN = CHR_COL_3: <Null>'
///                 ELSE
///                     WRITE(*,*) '  COLUMN = CHR_COL_3: ',
///          .                     ( CVALS(I)(:RTRIM(CVALS(I))) //' ',
///          .                                         I = 1, COL3SZ )
///                 END IF
///
///              END DO
///
///     C
///     C     We either parsed the SELECT clause or had an error.
///     C
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     ROW  =   1
///        COLUMN = CHR_COL_1: Column #2 has 2 elements.
///        COLUMN = CHR_COL_2: 101 102
///        COLUMN = CHR_COL_3: 101 201 301
///
///     ROW  =   2
///        COLUMN = CHR_COL_1: Column #2 has 4 elements.
///        COLUMN = CHR_COL_2: 201 202 203 204
///        COLUMN = CHR_COL_3: 102 202 302
///
///     ROW  =   3
///        COLUMN = CHR_COL_1: Column #2 has 6 elements.
///        COLUMN = CHR_COL_2: 301 302 303 304 305 306
///        COLUMN = CHR_COL_3: 103 203 303
///
///     ROW  =   4
///        COLUMN = CHR_COL_1: Column #2 has 8 elements.
///        COLUMN = CHR_COL_2: 401 402 403 404 405 406 407 408
///        COLUMN = CHR_COL_3: 104 204 304
///
///
///     Note that after run completion, a new EK file exists in the
///     output directory.
///
///
///  3) See $Examples in EKQMGR.
///
///     In this example, the names and data types of the columns from
///     which to fetch data are not known in advance.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code examples from existing fragments.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (NJB)
///
///         Bug fix: When an already loaded kernel is opened with EKOPR,
///         it now has its link count reset to 1 via a call to EKCLS.
///
/// -    SPICELIB Version 1.1.0, 07-JUL-1996 (NJB)
///
///         Redundant CHKIN call removed from SELIDX error check.
///         Misspelling of "issued" was fixed. Previous version line
///         was changed from "Beta" to "SPICELIB."
///
/// -    SPICELIB Version 1.0.0, 23-OCT-1995 (NJB)
/// ```
pub fn ekgc(
    ctx: &mut SpiceContext,
    selidx: i32,
    row: i32,
    elment: i32,
    cdata: &mut str,
    null: &mut bool,
    found: &mut bool,
) -> crate::Result<()> {
    EKGC(
        selidx,
        row,
        elment,
        fstr::StrBytes::new(cdata).as_mut(),
        null,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKGC  ( EK, get event data, character )
pub fn EKGC(
    SELIDX: i32,
    ROW: i32,
    ELMENT: i32,
    CDATA: &mut [u8],
    NULL: &mut bool,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"EKGC", ctx)?;
    }

    //
    // Nothing found yet.
    //
    *FOUND = false;

    //
    // There nothing to fetch if no files are loaded.  A sure
    // symptom of this problem is that the file list is empty.
    //
    if (save.FTHEAD <= 0) {
        SETMSG(b"No E-kernels are currently loaded.", ctx);
        SIGERR(b"SPICE(NOLOADEDFILES)", ctx)?;
        CHKOUT(b"EKGC", ctx)?;
        return Ok(());
    }

    //
    // The row number must be valid, or we can't proceed.
    //
    if ((ROW < 1) || (ROW > save.UNROWS)) {
        SETMSG(
            b"Row indices for query result range from 1 to #; requested row index was #.",
            ctx,
        );
        ERRINT(b"#", save.UNROWS, ctx);
        ERRINT(b"#", ROW, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"EKGC", ctx)?;
        return Ok(());
    }

    //
    // The element index must be positive.
    //
    if (ELMENT < 1) {
        SETMSG(b"ELMENT must be positive but was #.", ctx);
        ERRINT(b"#", ELMENT, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"EKGC", ctx)?;
        return Ok(());
    }

    //
    // Make sure the SELECT clause column index is valid.
    //
    if ((SELIDX < 1) || (SELIDX > save.NSEL)) {
        SETMSG(
            b"The SELECT column index # is out of the valid range 1:#",
            ctx,
        );
        ERRINT(b"#", SELIDX, ctx);
        ERRINT(b"#", save.NTAB, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"EKGC", ctx)?;
        return Ok(());
    }

    //
    // COL is the column's index within the parent
    // table's column list.
    //
    save.TABIDX = save.SELTAB[SELIDX];
    save.COL = save.SELCOL[SELIDX];
    save.COLPTR = save.SELCTP[SELIDX];
    save.TAB = save.TPTVEC[save.TABIDX];

    //
    // Make sure the column has character type.
    //
    if (save.CTTYPS[save.COLPTR] != CHR) {
        SETMSG(b"Column # has data type #.", ctx);
        ERRCH(b"#", &save.CTNAMS[save.COLPTR], ctx);
        ERRCH(b"#", &save.CHTYPE[save.CTTYPS[save.COLPTR]], ctx);
        SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
        CHKOUT(b"EKGC", ctx)?;
        return Ok(());
    }

    //
    // If it hasn't been done yet, and if it needs to be done, sort the
    // matching row vectors.
    //
    if save.DOSORT {
        ZZEKJSRT(
            save.USIZE,
            save.UBASE.as_slice(),
            save.NORDER,
            save.OTABS.as_slice(),
            save.OCOLS.as_slice(),
            save.OELTS.as_slice(),
            save.SENSE.as_slice(),
            save.STHAN.as_slice(),
            save.STDSCS.as_slice(),
            save.STDTPT.as_slice(),
            save.DTPOOL.as_slice(),
            save.DTDSCS.as_slice(),
            &mut save.ORDBAS,
            ctx,
        )?;

        save.DOSORT = false;
        save.SORTED = true;
    }

    //
    // Look up the segment vector and row vector for the current row.
    //
    if save.SORTED {
        ZZEKSRD(
            (save.ORDBAS + ROW),
            (save.ORDBAS + ROW),
            std::slice::from_mut(&mut save.I),
            ctx,
        )?;
        ZZEKVCAL(save.I, &mut save.RWVBAS, &mut save.SGVBAS, ctx)?;
    } else {
        ZZEKVCAL(ROW, &mut save.RWVBAS, &mut save.SGVBAS, ctx)?;
    }

    ZZEKSRD(
        (save.RWVBAS + 1),
        (save.RWVBAS + save.NTAB),
        save.ROWVEC.as_slice_mut(),
        ctx,
    )?;
    ZZEKSRD(
        (save.SGVBAS + 1),
        (save.SGVBAS + save.NTAB),
        save.SEGVEC.as_slice_mut(),
        ctx,
    )?;

    //
    // Identify the segment containing the column entry of interest.
    // Obtain the column descriptor for the column.
    //
    save.ROWIDX = save.ROWVEC[save.TABIDX];
    save.SEG = save.SEGVEC[save.TABIDX];

    save.J = save.STDTPT[save.SEG];

    {
        let m1__: i32 = 2;
        let m2__: i32 = save.COL;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = LNKNXT(save.J, save.DTPOOL.as_slice(), ctx)?;
            save.I += m3__;
        }
    }

    //
    // Look up the element.
    //
    ZZEKRSC(
        save.STHAN[save.SEG],
        save.STDSCS.subarray([1, save.SEG]),
        save.DTDSCS.subarray([1, save.J]),
        save.ROWIDX,
        ELMENT,
        &mut save.CVLEN,
        CDATA,
        NULL,
        FOUND,
        ctx,
    )?;

    CHKOUT(b"EKGC", ctx)?;
    Ok(())
}

/// EK, get event data, double precision
///
/// Return an element of an entry in a column of double precision
/// or `time' type in a specified row.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  SELIDX     I   Index of parent column in SELECT clause.
///  ROW        I   Row to fetch from.
///  ELMENT     I   Index of element, within column entry, to fetch.
///  DDATA      O   D.p. element of column entry.
///  NULL       O   Flag indicating whether column entry was null.
///  FOUND      O   Flag indicating whether column was present in row.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SELIDX   is the SELECT clause index of the column to
///           fetch from.
///
///  ROW      is the output row containing the entry to fetch
///           from.
///
///  ELMENT   is the index of the element of the column entry
///           to fetch. The normal range of ELMENT is from 1 to
///           the size of the column's entry, but ELMENT is
///           allowed to exceed the number of elements in the
///           column entry; if it does, FOUND is returned .FALSE.
///           This allows the caller to read data from the column
///           entry in a loop without checking the number of
///           available elements first.
///
///           Null values in variable-sized columns are
///           considered to have size 1.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DDATA    is the requested element of the specified column
///           entry. If the entry is null, DDATA is undefined.
///
///  NULL     is a logical flag indicating whether the entry
///           belonging to the specified column in the specified
///           row is null.
///
///  FOUND    is a logical flag indicating whether the specified
///           element was found. If the element does not exist,
///           FOUND is returned .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input argument ELMENT is less than 1, the error
///      SPICE(INVALIDINDEX) is signaled and FOUND is returned .FALSE.
///      However, ELMENT is allowed to be greater than the number of
///      elements in the specified column entry; this allows the caller
///      to read data from the column entry in a loop without checking
///      the number of available elements first. If ELMENT is greater
///      than the number of available elements, FOUND is returned
///      .FALSE.
///
///  2)  If SELIDX is outside of the range established by the
///      last query passed to EKSRCH, the error SPICE(INVALIDINDEX)
///      is signaled and FOUND is returned .FALSE.
///
///  3)  If the input argument ROW is less than 1 or greater than the
///      number of rows matching the query, the error
///      SPICE(INVALIDINDEX) is signaled and FOUND is returned .FALSE.
///
///  4)  If the specified column does not have DP or TIME type, the
///      error SPICE(INVALIDTYPE) is signaled.
///
///  5)  If this routine is called when no E-kernels have been loaded,
///      the error SPICE(NOLOADEDFILES) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the header of EKQMGR for a description of files used
///  by this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows retrieval of data from double precision or
///  `time' columns.
///
///  This routine returns one element at a time in order to save the
///  caller from imposing a limit on the size of the column entries
///  that can be handled.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Perform a query on an EK file that contains a database with
///     the Supplementary Engineering Data Records of the Viking
///     Project in order to retrieve the PLATFORM_CLOCK values (double
///     precision) that correspond to the images with IMAGE_NUMBER
///     smaller than a given value, ordered by IMAGE_NUMBER.
///
///
///     Use the EK kernel below to load the information from the
///     original Supplementary Engineering Data Record (SEDR) data
///     set generated by the Viking Project.
///
///        vo_sedr.bdb
///
///
///     Example code begins here.
///
///
///           PROGRAM EKGD_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Maximum length of an input query,
///     C     MAXQRY, and the maximum length of literal string
///     C     values, MAXSTR, from eklimit.inc.
///     C
///           INCLUDE 'ekqlimit.inc'
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         EKNAME
///           PARAMETER           ( EKNAME = 'vo_sedr.bdb' )
///
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN = 1840 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(ERRLEN)    ERRMSG
///           CHARACTER*(MAXQRY)    QUERY
///
///           DOUBLE PRECISION      DDATA
///
///           INTEGER               ELTIDX
///           INTEGER               NMROWS
///           INTEGER               ROWNO
///           INTEGER               SELIDX
///
///           LOGICAL               ERROR
///           LOGICAL               FOUND
///           LOGICAL               ISNULL
///
///     C
///     C     Open an EK file.
///     C
///           CALL FURNSH ( EKNAME )
///
///     C
///     C     The table 'VIKING_SEDR_DATA' has a column
///     C     'PLATFORM_CLOCK' of double precision values.
///     C
///     C     Define a set of constraints to perform a query on
///     C     all loaded EK files (the SELECT clause). In this
///     C     case select the column 'PLATFORM_CLOCK' from table
///     C     'VIKING_SEDR_DATA' sorted by 'IMAGE_NUMBER'.
///     C
///           QUERY = 'Select PLATFORM_CLOCK from VIKING_SEDR_DATA '
///          .   //   'where IMAGE_NUMBER < 25860000 '
///          .   //   'order by IMAGE_NUMBER'
///
///     C
///     C     Query the EK system for data rows matching the
///     C     SELECT constraints.
///     C
///           CALL EKFIND ( QUERY, NMROWS, ERROR, ERRMSG )
///
///     C
///     C     Check whether an error occurred while processing the
///     C     SELECT clause. If so, output the error message.
///     C
///           IF ( ERROR ) THEN
///
///              WRITE(*,*) 'SELECT clause error: ', ERRMSG
///
///           ELSE
///
///     C
///     C        Fetch the character data. We know the query returned
///     C        one column and the column contains only scalar data,
///     C        so the index of all elements is 1.
///     C
///              SELIDX = 1
///              ELTIDX = 1
///
///     C
///     C        Loop over each row found matching the query.
///     C
///              DO ROWNO = 1, NMROWS
///
///
///
///     C
///     C           Use EKGD to retrieve the string from
///     C
///                 CALL EKGD ( SELIDX, ROWNO,  ELTIDX,
///          .                  DDATA,  ISNULL, FOUND   )
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE (*,'(A,I3,A)') 'Row ', ROWNO,
///          .                  ': Double precision data: <Null>'
///
///                 ELSE
///
///                    WRITE (*,'(A,I3,A,F10.6)') 'Row ', ROWNO,
///          .                  ': Double precision data: ', DDATA
///
///                 END IF
///
///              END DO
///
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Row   1: Double precision data: 119.880000
///     Row   2: Double precision data: 119.270000
///     Row   3: Double precision data: 119.880000
///     Row   4: Double precision data: 119.270000
///     Row   5: Double precision data: 119.880000
///     Row   6: Double precision data: 119.270000
///     Row   7: Double precision data: 120.140000
///     Row   8: Double precision data: 119.520000
///     Row   9: Double precision data: 120.140000
///     Row  10: Double precision data: 120.140000
///     Row  11: Double precision data: 120.140000
///     Row  12: Double precision data: 221.920000
///     Row  13: Double precision data: 221.920000
///     Row  14: Double precision data: 221.920000
///     Row  15: Double precision data: 120.140000
///     Row  16: Double precision data: 120.140000
///     Row  17: Double precision data: 120.140000
///     Row  18: Double precision data: 120.220000
///     Row  19: Double precision data: 120.220000
///     Row  20: Double precision data: 120.220000
///     Row  21: Double precision data: 120.370000
///     Row  22: Double precision data: 120.370000
///     Row  23: Double precision data: 120.370000
///     Row  24: Double precision data: 120.290000
///     Row  25: Double precision data: 120.290000
///     Row  26: Double precision data: 120.290000
///
///
///  2) Perform a query on an EK file that contains a database with
///     the Supplementary Engineering Data Records of the Viking
///     Project in order to retrieve the IMAGE_TIME values (double
///     precision time) that correspond to the images with
///     IMAGE_NUMBER smaller than a given value, ordered by
///     IMAGE_NUMBER.
///
///
///     Use the EK kernel below to load the information from the
///     original Supplementary Engineering Data Record (SEDR) data
///     set generated by the Viking Project.
///
///        vo_sedr.bdb
///
///     Use the LSK kernel below to load the leap seconds and time
///     constants required for the conversions.
///
///        naif0012.tls
///
///
///     Example code begins here.
///
///
///           PROGRAM EKGD_EX2
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Maximum length of an input query,
///     C     MAXQRY, and the maximum length of literal string
///     C     values, MAXSTR, from eklimit.inc.
///     C
///           INCLUDE 'ekqlimit.inc'
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         EKNAME
///           PARAMETER           ( EKNAME = 'vo_sedr.bdb' )
///
///           CHARACTER*(*)         LSKNAM
///           PARAMETER           ( LSKNAM = 'naif0012.tls' )
///
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN = 1840 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 24   )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(ERRLEN)    ERRMSG
///           CHARACTER*(MAXQRY)    QUERY
///           CHARACTER*(TIMLEN)    UTCSTR
///
///           DOUBLE PRECISION      DDATA
///
///           INTEGER               ELTIDX
///           INTEGER               NMROWS
///           INTEGER               ROWNO
///           INTEGER               SELIDX
///
///           LOGICAL               ERROR
///           LOGICAL               FOUND
///           LOGICAL               ISNULL
///
///     C
///     C     Load leapseconds file for time conversion.
///     C
///           CALL FURNSH ( LSKNAM )
///
///     C
///     C     Open an EK file.
///     C
///           CALL FURNSH ( EKNAME )
///
///     C
///     C     The table 'VIKING_SEDR_DATA' has a column
///     C     'IMAGE_TIME' of time values (stored as double
///     C     precision items).
///     C
///     C     Define a set of constraints to perform a query on
///     C     all loaded EK files (the SELECT clause). In this
///     C     case select the column 'IMAGE_TIME' from table
///     C     'VIKING_SEDR_DATA' sorted by 'IMAGE_NUMBER'.
///     C
///           QUERY = 'Select IMAGE_TIME from VIKING_SEDR_DATA '
///          .   //   'where IMAGE_NUMBER < 25860000 '
///          .   //   'order by IMAGE_NUMBER'
///
///     C
///     C     Query the EK system for data rows matching the
///     C     SELECT constraints.
///     C
///           CALL EKFIND ( QUERY, NMROWS, ERROR, ERRMSG )
///
///     C
///     C     Check whether an error occurred while processing the
///     C     SELECT clause. If so, output the error message.
///     C
///           IF ( ERROR ) THEN
///
///              WRITE(*,*) 'SELECT clause error: ', ERRMSG
///
///           ELSE
///
///     C
///     C        Fetch the character data. We know the query returned
///     C        one column and the column contains only scalar data,
///     C        so the index of all elements is 1.
///     C
///              SELIDX = 1
///              ELTIDX = 1
///
///     C
///     C        Loop over each row found matching the query.
///     C
///              DO ROWNO = 1, NMROWS
///
///     C
///     C           Use EKGD to retrieve the string from
///     C
///                 CALL EKGD ( SELIDX, ROWNO,  ELTIDX,
///          .                  DDATA,  ISNULL, FOUND   )
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE (*,'(A,I3,A)') 'Row ', ROWNO,
///          .                              ': Time data: <Null>'
///
///                 ELSE
///
///                    CALL ET2UTC ( DDATA, 'C', 3, UTCSTR )
///
///                    WRITE (*,'(A,I3,2A)') 'Row ', ROWNO,
///          .                               ': Time data:  ', UTCSTR
///
///                 END IF
///
///              END DO
///
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Row   1: Time data:  1976 JUN 16 16:50:55.925
///     Row   2: Time data:  1976 JUN 16 16:51:00.269
///     Row   3: Time data:  1976 JUN 16 20:56:53.051
///     Row   4: Time data:  1976 JUN 16 20:56:57.395
///     Row   5: Time data:  1976 JUN 17 01:02:50.177
///     Row   6: Time data:  1976 JUN 17 01:02:54.521
///     Row   7: Time data:  1976 JUN 17 05:08:56.263
///     Row   8: Time data:  1976 JUN 17 05:09:00.607
///     Row   9: Time data:  1976 JUN 17 06:30:28.424
///     Row  10: Time data:  1976 JUN 17 06:30:46.174
///     Row  11: Time data:  1976 JUN 17 06:30:55.168
///     Row  12: Time data:  1976 JUN 17 11:17:47.471
///     Row  13: Time data:  1976 JUN 17 11:18:05.221
///     Row  14: Time data:  1976 JUN 17 11:18:14.215
///     Row  15: Time data:  1976 JUN 17 13:20:23.634
///     Row  16: Time data:  1976 JUN 17 13:20:41.384
///     Row  17: Time data:  1976 JUN 17 13:20:50.378
///     Row  18: Time data:  1976 JUN 17 15:23:17.717
///     Row  19: Time data:  1976 JUN 17 15:23:35.467
///     Row  20: Time data:  1976 JUN 17 15:23:44.461
///     Row  21: Time data:  1976 JUN 17 17:26:20.760
///     Row  22: Time data:  1976 JUN 17 17:26:38.510
///     Row  23: Time data:  1976 JUN 17 17:26:47.504
///     Row  24: Time data:  1976 JUN 17 19:29:23.803
///     Row  25: Time data:  1976 JUN 17 19:29:41.553
///     Row  26: Time data:  1976 JUN 17 19:29:50.547
///
///
///  3) This example demonstrates how to fetch double precision values
///     from a column in three different cases: single values,
///     variable-size arrays and static-size arrays.
///
///     Create an EK that contains a table TAB that has the following
///     columns:
///
///        Column name   Data Type   Size
///        -----------   ---------   ----
///        DP_COL_1      DP          1
///        DP_COL_2      DP          VARIABLE
///        DP_COL_3      DP          3
///
///     Issue the following query
///
///         QUERY = 'SELECT DP_COL_1, DP_COL_2, DP_COL_3 FROM TAB'
///
///     to fetch and dump column values from the rows that satisfy the
///     query.
///
///
///     Example code begins here.
///
///
///           PROGRAM EKGD_EX3
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Column Name Size (CNAMSZ)
///     C     and EK Query Limit Parameters (MAXQRY)
///     C
///           INCLUDE 'ekcnamsz.inc'
///           INCLUDE 'ekqlimit.inc'
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         EKNAME
///           PARAMETER           ( EKNAME  = 'ekgd_ex3.bdb' )
///
///           CHARACTER*(*)         TABLE
///           PARAMETER           ( TABLE   = 'TAB' )
///
///           INTEGER               COL3SZ
///           PARAMETER           ( COL3SZ = 3   )
///
///           INTEGER               DECLEN
///           PARAMETER           ( DECLEN = 200 )
///
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN = 1840 )
///
///           INTEGER               MXC2SZ
///           PARAMETER           ( MXC2SZ = 4   )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 40  )
///
///           INTEGER               NCOLS
///           PARAMETER           ( NCOLS  = 3   )
///
///           INTEGER               NROWS
///           PARAMETER           ( NROWS  = 4   )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(DECLEN)    CDECLS ( NCOLS  )
///           CHARACTER*(CNAMSZ)    CNAMES ( NCOLS  )
///           CHARACTER*(ERRLEN)    ERRMSG
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(MAXQRY)    QUERY
///
///           DOUBLE PRECISION      COL1
///           DOUBLE PRECISION      COL2   ( MXC2SZ )
///           DOUBLE PRECISION      COL3   ( COL3SZ )
///           DOUBLE PRECISION      DVALS  ( MXC2SZ )
///
///           INTEGER               ELTIDX
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               J
///           INTEGER               NELT
///           INTEGER               NMROWS
///           INTEGER               NRESVC
///           INTEGER               RECNO
///           INTEGER               ROW
///           INTEGER               SEGNO
///           INTEGER               SELIDX
///
///           LOGICAL               ERROR
///           LOGICAL               FOUND
///           LOGICAL               ISNULL
///
///     C
///     C     Open a new EK file.  For simplicity, we will not
///     C     reserve any space for the comment area, so the
///     C     number of reserved comment characters is zero.
///     C     The variable IFNAME is the internal file name.
///     C
///           NRESVC  =  0
///           IFNAME  =  'Test EK/Created 13-JUN-2019'
///
///           CALL EKOPN ( EKNAME, IFNAME, NRESVC, HANDLE )
///
///     C
///     C     Set up the column names and declarations
///     C     for the TAB segment.  We'll index all of
///     C     the columns.
///     C
///           CNAMES(1) = 'DP_COL_1'
///           CDECLS(1) = 'DATATYPE = DOUBLE PRECISION, ' //
///          .            'INDEXED  = TRUE'
///
///           CNAMES(2) = 'DP_COL_2'
///           CDECLS(2) = 'DATATYPE = DOUBLE PRECISION, ' //
///          .            'SIZE = VARIABLE'
///
///           CNAMES(3) = 'DP_COL_3'
///           CDECLS(3) = 'DATATYPE = DOUBLE PRECISION, ' //
///          .            'SIZE = 3'
///
///     C
///     C     Start the segment.
///     C
///           CALL EKBSEG ( HANDLE, TABLE,  NCOLS,
///          .              CNAMES, CDECLS, SEGNO )
///
///     C
///     C     At the records to the table.
///     C
///           DO I = 1, NROWS
///
///     C
///     C        Append a new record to the EK.
///     C
///              CALL EKAPPR ( HANDLE, SEGNO, RECNO )
///
///     C
///     C        Add DP_COL_1
///     C
///              COL1 = I * 100.D0
///
///              CALL EKACED ( HANDLE,    SEGNO, RECNO,
///          .                 CNAMES(1), 1,     COL1,  .FALSE. )
///
///     C
///     C        Add I items to DP_COL_2
///     C
///              DO J = 1, I
///                 COL2(J) = J + I*200.D0
///              END DO
///
///              CALL EKACED ( HANDLE,    SEGNO, RECNO,
///          .                 CNAMES(2), I,     COL2,  .FALSE. )
///
///     C
///     C        Add 3 items to DP_COL_3
///     C
///              DO J = 1, 3
///                 COL3(J) =  I + J*100.D0
///              END DO
///
///              CALL EKACED ( HANDLE,    SEGNO, RECNO,
///          .                 CNAMES(3), 3,     COL3, .FALSE. )
///
///           END DO
///
///     C
///     C     Close the file.
///     C
///           CALL EKCLS ( HANDLE )
///
///     C
///     C     Open the created file. Perform the query and show the
///     C     results.
///     C
///           CALL FURNSH ( EKNAME )
///
///           QUERY = 'SELECT DP_COL_1, DP_COL_2, DP_COL_3 FROM TAB'
///
///     C
///     C     Query the EK system for data rows matching the
///     C     SELECT constraints.
///     C
///           CALL EKFIND ( QUERY, NMROWS, ERROR, ERRMSG )
///
///     C
///     C     Check whether an error occurred while processing the
///     C     SELECT clause. If so, output the error message.
///     C
///           IF ( ERROR ) THEN
///
///              WRITE(*,*) 'SELECT clause error: ', ERRMSG
///
///           ELSE
///
///              DO ROW = 1, NMROWS
///
///                 WRITE(*,*) ' '
///                 WRITE(*,'(A,I3)') 'ROW  = ', ROW
///
///     C
///     C           Fetch values from column DP_COL_1.  Since
///     C           DP_COL_1 was the first column selected, the
///     C           selection index SELIDX is set to 1.
///     C
///                 SELIDX = 1
///                 ELTIDX = 1
///                 CALL EKGD ( SELIDX,    ROW,     ELTIDX,
///          .                  DVALS(1),  ISNULL,  FOUND   )
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE(*,*) '  COLUMN = DP_COL_1: <Null>'
///
///                 ELSE
///
///                    WRITE(*,*) '  COLUMN = DP_COL_1:', DVALS(1)
///
///                 END IF
///
///     C
///     C           Fetch values from column DP_COL_2 in the current
///     C           row.  Since DP_COL_2 contains variable-size array
///     C           elements, we call EKNELT to determine how many
///     C           elements to fetch.
///     C
///                 SELIDX = 2
///                 CALL EKNELT ( SELIDX, ROW, NELT )
///
///                 ELTIDX = 1
///                 ISNULL = .FALSE.
///
///                 DO WHILE (       ( ELTIDX .LE.  NELT   )
///          .                 .AND. (        .NOT. ISNULL )  )
///
///                    CALL EKGD ( SELIDX,         ROW,     ELTIDX,
///          .                     DVALS(ELTIDX),  ISNULL,  FOUND   )
///
///                    ELTIDX = ELTIDX + 1
///
///     C
///     C           If the column entry is null, we'll be kicked
///     C           out of this loop after the first iteration.
///     C
///                 END DO
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE(*,*) '  COLUMN = DP_COL_2: <Null>'
///
///                 ELSE
///
///                    WRITE(*,'(A,4F6.1)') '   COLUMN = DP_COL_2:',
///          .                              ( DVALS(I), I = 1, NELT )
///
///                 END IF
///
///     C
///     C           Fetch values from column DP_COL_3 in the current
///     C           row.  We need not call EKNELT since we know how
///     C           many elements are in each column entry.
///     C
///                 SELIDX = 3
///                 ELTIDX = 1
///                 ISNULL = .FALSE.
///
///                 DO WHILE (       ( ELTIDX .LE.  COL3SZ )
///          .                 .AND. (        .NOT. ISNULL )  )
///
///                    CALL EKGD ( SELIDX,         ROW,     ELTIDX,
///          .                     DVALS(ELTIDX),  ISNULL,  FOUND   )
///
///                    ELTIDX = ELTIDX + 1
///
///                 END DO
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE(*,*) '  COLUMN = DP_COL_3: <Null>'
///
///                 ELSE
///
///                    WRITE(*,'(A,3F6.1)') '   COLUMN = DP_COL_3:',
///          .                              ( DVALS(I), I = 1, COL3SZ )
///
///                 END IF
///
///              END DO
///
///     C
///     C     We either parsed the SELECT clause or had an error.
///     C
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     ROW  =   1
///        COLUMN = DP_COL_1:   100.00000000000000
///        COLUMN = DP_COL_2: 201.0
///        COLUMN = DP_COL_3: 101.0 201.0 301.0
///
///     ROW  =   2
///        COLUMN = DP_COL_1:   200.00000000000000
///        COLUMN = DP_COL_2: 401.0 402.0
///        COLUMN = DP_COL_3: 102.0 202.0 302.0
///
///     ROW  =   3
///        COLUMN = DP_COL_1:   300.00000000000000
///        COLUMN = DP_COL_2: 601.0 602.0 603.0
///        COLUMN = DP_COL_3: 103.0 203.0 303.0
///
///     ROW  =   4
///        COLUMN = DP_COL_1:   400.00000000000000
///        COLUMN = DP_COL_2: 801.0 802.0 803.0 804.0
///        COLUMN = DP_COL_3: 104.0 204.0 304.0
///
///
///     Note that after run completion, a new EK file exists in the
///     output directory.
///
///  4) See $Examples in EKQMGR.
///
///     In this example, the names and data types of the columns from
///     which to fetch data are not known in advance.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code examples from existing fragments.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (NJB)
///
///         Bug fix: When an already loaded kernel is opened with EKOPR,
///         it now has its link count reset to 1 via a call to EKCLS.
///
/// -    SPICELIB Version 1.1.0, 07-JUL-1996 (NJB)
///
///         Redundant CHKIN call removed from SELIDX error check.
///         Misspelling of "issued" was fixed. Previous version line
///         was changed from "Beta" to "SPICELIB."
///
/// -    SPICELIB Version 1.0.0, 23-OCT-1995 (NJB)
/// ```
pub fn ekgd(
    ctx: &mut SpiceContext,
    selidx: i32,
    row: i32,
    elment: i32,
    ddata: &mut f64,
    null: &mut bool,
    found: &mut bool,
) -> crate::Result<()> {
    EKGD(selidx, row, elment, ddata, null, found, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKGD  ( EK, get event data, double precision )
pub fn EKGD(
    SELIDX: i32,
    ROW: i32,
    ELMENT: i32,
    DDATA: &mut f64,
    NULL: &mut bool,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"EKGD", ctx)?;
    }

    //
    // Nothing found yet.
    //
    *FOUND = false;

    //
    // There nothing to fetch if no files are loaded.  A sure
    // symptom of this problem is that the file list is empty.
    //
    if (save.FTHEAD <= 0) {
        SETMSG(b"No E-kernels are currently loaded.", ctx);
        SIGERR(b"SPICE(NOLOADEDFILES)", ctx)?;
        CHKOUT(b"EKGD", ctx)?;
        return Ok(());
    }

    //
    // The row number must be valid, or we can't proceed.
    //
    if ((ROW < 1) || (ROW > save.UNROWS)) {
        SETMSG(
            b"Row indices for query result range from 1 to #; requested row index was #.",
            ctx,
        );
        ERRINT(b"#", save.UNROWS, ctx);
        ERRINT(b"#", ROW, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"EKGD", ctx)?;
        return Ok(());
    }

    //
    // The element index must be positive.
    //
    if (ELMENT < 1) {
        SETMSG(b"ELMENT must be positive but was #.", ctx);
        ERRINT(b"#", ELMENT, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"EKGD", ctx)?;
        return Ok(());
    }

    //
    // Make sure the SELECT clause column index is valid.
    //
    if ((SELIDX < 1) || (SELIDX > save.NSEL)) {
        SETMSG(
            b"The SELECT column index # is out of the valid range 1:#",
            ctx,
        );
        ERRINT(b"#", SELIDX, ctx);
        ERRINT(b"#", save.NTAB, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"EKGD", ctx)?;
        return Ok(());
    }

    //
    // COL is the column's index within the parent
    // table's column list.
    //
    save.TABIDX = save.SELTAB[SELIDX];
    save.COL = save.SELCOL[SELIDX];
    save.COLPTR = save.SELCTP[SELIDX];
    save.TAB = save.TPTVEC[save.TABIDX];

    //
    // Make sure the column has double precision or `time' type.
    //
    if ((save.CTTYPS[save.COLPTR] != DP) && (save.CTTYPS[save.COLPTR] != TIME)) {
        SETMSG(b"Column # has data type #.", ctx);
        ERRCH(b"#", &save.CTNAMS[save.COLPTR], ctx);
        ERRCH(b"#", &save.CHTYPE[save.CTTYPS[save.COLPTR]], ctx);
        SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
        CHKOUT(b"EKGD", ctx)?;
        return Ok(());
    }

    //
    // If it hasn't been done yet, and if it needs to be done, sort the
    // matching row vectors.
    //
    if save.DOSORT {
        ZZEKJSRT(
            save.USIZE,
            save.UBASE.as_slice(),
            save.NORDER,
            save.OTABS.as_slice(),
            save.OCOLS.as_slice(),
            save.OELTS.as_slice(),
            save.SENSE.as_slice(),
            save.STHAN.as_slice(),
            save.STDSCS.as_slice(),
            save.STDTPT.as_slice(),
            save.DTPOOL.as_slice(),
            save.DTDSCS.as_slice(),
            &mut save.ORDBAS,
            ctx,
        )?;

        save.DOSORT = false;
        save.SORTED = true;
    }

    //
    // Look up the segment vector and row vector for the current row.
    //
    if save.SORTED {
        ZZEKSRD(
            (save.ORDBAS + ROW),
            (save.ORDBAS + ROW),
            std::slice::from_mut(&mut save.I),
            ctx,
        )?;
        ZZEKVCAL(save.I, &mut save.RWVBAS, &mut save.SGVBAS, ctx)?;
    } else {
        ZZEKVCAL(ROW, &mut save.RWVBAS, &mut save.SGVBAS, ctx)?;
    }

    ZZEKSRD(
        (save.RWVBAS + 1),
        (save.RWVBAS + save.NTAB),
        save.ROWVEC.as_slice_mut(),
        ctx,
    )?;
    ZZEKSRD(
        (save.SGVBAS + 1),
        (save.SGVBAS + save.NTAB),
        save.SEGVEC.as_slice_mut(),
        ctx,
    )?;

    //
    // Identify the segment containing the column entry of interest.
    // Obtain the column descriptor for the column.
    //
    save.ROWIDX = save.ROWVEC[save.TABIDX];
    save.SEG = save.SEGVEC[save.TABIDX];

    save.J = save.STDTPT[save.SEG];

    {
        let m1__: i32 = 2;
        let m2__: i32 = save.COL;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = LNKNXT(save.J, save.DTPOOL.as_slice(), ctx)?;
            save.I += m3__;
        }
    }

    //
    // Look up the element.
    //
    ZZEKRSD(
        save.STHAN[save.SEG],
        save.STDSCS.subarray([1, save.SEG]),
        save.DTDSCS.subarray([1, save.J]),
        save.ROWIDX,
        ELMENT,
        DDATA,
        NULL,
        FOUND,
        ctx,
    )?;

    CHKOUT(b"EKGD", ctx)?;
    Ok(())
}

/// EK, get event data, integer
///
/// Return an element of an entry in a column of integer
/// type in a specified row.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  SELIDX     I   Index of parent column in SELECT clause.
///  ROW        I   Row to fetch from.
///  ELMENT     I   Index of element, within column entry, to fetch.
///  IDATA      O   Integer element of column entry.
///  NULL       O   Flag indicating whether column entry was null.
///  FOUND      O   Flag indicating whether column was present in row.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SELIDX   is the SELECT clause index of the column to
///           fetch from.
///
///  ROW      is the output row containing the entry to fetch
///           from.
///
///  ELMENT   is the index of the element of the column entry
///           to fetch. The normal range of ELMENT is from 1 to
///           the size of the column's entry, but ELMENT is
///           allowed to exceed the number of elements in the
///           column entry; if it does, FOUND is returned .FALSE.
///           This allows the caller to read data from the column
///           entry in a loop without checking the number of
///           available elements first.
///
///           Null values in variable-sized columns are
///           considered to have size 1.
/// ```
///
/// # Detailed Output
///
/// ```text
///  IDATA    is the requested element of the specified column
///           entry. If the entry is null, IDATA is undefined.
///
///  NULL     is a logical flag indicating whether the entry
///           belonging to the specified column in the specified
///           row is null.
///
///  FOUND    is a logical flag indicating whether the specified
///           element was found. If the element does not exist,
///           FOUND is returned .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input argument ELMENT is less than 1, the error
///      SPICE(INVALIDINDEX) is signaled and FOUND is returned .FALSE.
///      However, ELMENT is allowed to be greater than the number of
///      elements in the specified column entry; this allows the caller
///      to read data from the column entry in a loop without checking
///      the number of available elements first. If ELMENT is greater
///      than the number of available elements, FOUND is returned
///      .FALSE.
///
///  2)  If SELIDX is outside of the range established by the
///      last query passed to EKSRCH, the error SPICE(INVALIDINDEX)
///      is signaled and FOUND is returned .FALSE.
///
///  3)  If the input argument ROW is less than 1 or greater than the
///      number of rows matching the query, the error
///      SPICE(INVALIDINDEX) is signaled and FOUND is returned .FALSE.
///
///  4)  If the specified column does not have integer type, the
///      error SPICE(INVALIDTYPE) is signaled.
///
///  5)  If this routine is called when no E-kernels have been loaded,
///      the error SPICE(NOLOADEDFILES) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the header of EKQMGR for a description of files used
///  by this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows retrieval of data from integer columns.
///
///  This routine returns one element at a time in order to save the
///  caller from imposing a limit on the size of the column entries
///  that can be handled.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Perform a query on an EK file that contains a database with
///     the Supplementary Engineering Data Records of the Viking
///     Project in order to retrieve the IMAGE_NUMBER values (integer
///     numbers) that correspond to the images with IMAGE_NUMBER
///     smaller than a given value, ordered by IMAGE_NUMBER.
///
///
///     Use the EK kernel below to load the information from the
///     original Supplementary Engineering Data Record (SEDR) data
///     set generated by the Viking Project.
///
///        vo_sedr.bdb
///
///
///     Example code begins here.
///
///
///           PROGRAM EKGI_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Maximum length of an input query,
///     C     MAXQRY, and the maximum length of literal string
///     C     values, MAXSTR, from eklimit.inc.
///     C
///           INCLUDE 'ekqlimit.inc'
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         EKNAME
///           PARAMETER           ( EKNAME = 'vo_sedr.bdb' )
///
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN = 1840 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(ERRLEN)    ERRMSG
///           CHARACTER*(MAXQRY)    QUERY
///
///           INTEGER               ELTIDX
///           INTEGER               IDATA
///           INTEGER               NMROWS
///           INTEGER               ROWNO
///           INTEGER               SELIDX
///
///           LOGICAL               ERROR
///           LOGICAL               FOUND
///           LOGICAL               ISNULL
///
///     C
///     C     Open an EK file.
///     C
///           CALL FURNSH ( EKNAME )
///
///     C
///     C     The table 'VIKING_SEDR_DATA' has a column
///     C     'IMAGE_NUMBER' of double precision values.
///     C
///     C     Define a set of constraints to perform a query on
///     C     all loaded EK files (the SELECT clause). In this
///     C     case select the column 'IMAGE_NUMBER' from table
///     C     'VIKING_SEDR_DATA' sorted by 'IMAGE_NUMBER'.
///     C
///           QUERY = 'Select IMAGE_NUMBER from VIKING_SEDR_DATA '
///          .   //   'where IMAGE_NUMBER < 25860000 '
///          .   //   'order by IMAGE_NUMBER'
///
///     C
///     C     Query the EK system for data rows matching the
///     C     SELECT constraints.
///     C
///           CALL EKFIND ( QUERY, NMROWS, ERROR, ERRMSG )
///
///     C
///     C     Check whether an error occurred while processing the
///     C     SELECT clause. If so, output the error message.
///     C
///           IF ( ERROR ) THEN
///
///              WRITE(*,*) 'SELECT clause error: ', ERRMSG
///
///           ELSE
///
///     C
///     C        Fetch the character data. We know the query returned
///     C        one column and the column contains only scalar data,
///     C        so the index of all elements is 1.
///     C
///              SELIDX = 1
///              ELTIDX = 1
///
///     C
///     C        Loop over each row found matching the query.
///     C
///              DO ROWNO = 1, NMROWS
///
///     C
///     C           Use EKGD to retrieve the string from
///     C
///                 CALL EKGI ( SELIDX, ROWNO,  ELTIDX,
///          .                  IDATA,  ISNULL, FOUND   )
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE (*,'(A,I3,A)') 'Row ', ROWNO,
///          .                  ': Integer data: <Null>'
///
///                 ELSE
///
///                    WRITE (*,'(A,I3,A,I10)') 'Row ', ROWNO,
///          .                  ': Integer data: ', IDATA
///
///                 END IF
///
///              END DO
///
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Row   1: Integer data:   25837050
///     Row   2: Integer data:   25837051
///     Row   3: Integer data:   25840344
///     Row   4: Integer data:   25840345
///     Row   5: Integer data:   25843638
///     Row   6: Integer data:   25843639
///     Row   7: Integer data:   25846934
///     Row   8: Integer data:   25846935
///     Row   9: Integer data:   25848026
///     Row  10: Integer data:   25848030
///     Row  11: Integer data:   25848032
///     Row  12: Integer data:   25851874
///     Row  13: Integer data:   25851878
///     Row  14: Integer data:   25851880
///     Row  15: Integer data:   25853516
///     Row  16: Integer data:   25853520
///     Row  17: Integer data:   25853522
///     Row  18: Integer data:   25855162
///     Row  19: Integer data:   25855166
///     Row  20: Integer data:   25855168
///     Row  21: Integer data:   25856810
///     Row  22: Integer data:   25856814
///     Row  23: Integer data:   25856816
///     Row  24: Integer data:   25858458
///     Row  25: Integer data:   25858462
///     Row  26: Integer data:   25858464
///
///
///  2) This example demonstrates how to fetch integer values
///     from a column in three different cases: single values,
///     variable-size arrays and static-size arrays.
///
///     Create an EK that contains a table TAB that has the following
///     columns:
///
///        Column name   Data Type   Size
///        -----------   ---------   ----
///        INT_COL_1     INT         1
///        INT_COL_2     INT         VARIABLE
///        INT_COL_3     INT         3
///
///
///     Issue the following query
///
///         QUERY = 'SELECT INT_COL_1, INT_COL2, INT_COL3 FROM TAB'
///
///     to fetch and dump column values from the rows that satisfy the
///     query.
///
///
///     Example code begins here.
///
///
///           PROGRAM EKGI_EX2
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Column Name Size (CNAMSZ)
///     C     and EK Query Limit Parameters (MAXQRY)
///     C
///           INCLUDE 'ekcnamsz.inc'
///           INCLUDE 'ekqlimit.inc'
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         EKNAME
///           PARAMETER           ( EKNAME  = 'ekgi_ex2.bdb' )
///
///           CHARACTER*(*)         TABLE
///           PARAMETER           ( TABLE   = 'TAB' )
///
///           INTEGER               COL3SZ
///           PARAMETER           ( COL3SZ = 3   )
///
///           INTEGER               DECLEN
///           PARAMETER           ( DECLEN = 200 )
///
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN = 1840 )
///
///           INTEGER               MXC2SZ
///           PARAMETER           ( MXC2SZ = 4   )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 40  )
///
///           INTEGER               NCOLS
///           PARAMETER           ( NCOLS  = 3   )
///
///           INTEGER               NROWS
///           PARAMETER           ( NROWS  = 4   )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(DECLEN)    CDECLS ( NCOLS  )
///           CHARACTER*(CNAMSZ)    CNAMES ( NCOLS  )
///           CHARACTER*(ERRLEN)    ERRMSG
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(MAXQRY)    QUERY
///
///           INTEGER               COL1
///           INTEGER               COL2   ( MXC2SZ )
///           INTEGER               COL3   ( COL3SZ )
///           INTEGER               ELTIDX
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               IVALS  ( MXC2SZ )
///           INTEGER               J
///           INTEGER               NELT
///           INTEGER               NMROWS
///           INTEGER               NRESVC
///           INTEGER               RECNO
///           INTEGER               ROW
///           INTEGER               SEGNO
///           INTEGER               SELIDX
///
///           LOGICAL               ERROR
///           LOGICAL               FOUND
///           LOGICAL               ISNULL
///
///     C
///     C     Open a new EK file.  For simplicity, we will not
///     C     reserve any space for the comment area, so the
///     C     number of reserved comment characters is zero.
///     C     The variable IFNAME is the internal file name.
///     C
///           NRESVC  =  0
///           IFNAME  =  'Test EK/Created 13-JUN-2019'
///
///           CALL EKOPN ( EKNAME, IFNAME, NRESVC, HANDLE )
///
///     C
///     C     Set up the column names and declarations
///     C     for the TAB segment.  We'll index all of
///     C     the columns.
///     C
///           CNAMES(1) = 'INT_COL_1'
///           CDECLS(1) = 'DATATYPE = INTEGER, INDEXED  = TRUE'
///
///           CNAMES(2) = 'INT_COL_2'
///           CDECLS(2) = 'DATATYPE = INTEGER, SIZE = VARIABLE'
///
///           CNAMES(3) = 'INT_COL_3'
///           CDECLS(3) = 'DATATYPE = INTEGER, SIZE = 3'
///
///     C
///     C     Start the segment.
///     C
///           CALL EKBSEG ( HANDLE, TABLE,  NCOLS,
///          .              CNAMES, CDECLS, SEGNO )
///
///     C
///     C     At the records to the table.
///     C
///           DO I = 1, NROWS
///
///     C
///     C        Append a new record to the EK.
///     C
///              CALL EKAPPR ( HANDLE, SEGNO, RECNO )
///
///     C
///     C        Add INT_COL_1
///     C
///              COL1 = I * 100
///
///              CALL EKACEI ( HANDLE,    SEGNO, RECNO,
///          .                 CNAMES(1), 1,     COL1,  .FALSE. )
///
///     C
///     C        Add I items to INT_COL_2
///     C
///              DO J = 1, I
///                 COL2(J) = J + I*200
///              END DO
///
///              CALL EKACEI ( HANDLE,    SEGNO, RECNO,
///          .                 CNAMES(2), I,     COL2,  .FALSE. )
///
///     C
///     C        Add 3 items to INT_COL_3
///     C
///              DO J = 1, 3
///                 COL3(J) =  I + J*100.D0
///              END DO
///
///              CALL EKACEI ( HANDLE,    SEGNO, RECNO,
///          .                 CNAMES(3), 3,     COL3, .FALSE. )
///
///           END DO
///
///     C
///     C     Close the file.
///     C
///           CALL EKCLS ( HANDLE )
///
///     C
///     C     Open the created file. Perform the query and show the
///     C     results.
///     C
///           CALL FURNSH ( EKNAME )
///
///           QUERY = 'SELECT INT_COL_1, INT_COL_2, INT_COL_3 '
///          .   //   'FROM TAB'
///
///     C
///     C     Query the EK system for data rows matching the
///     C     SELECT constraints.
///     C
///           CALL EKFIND ( QUERY, NMROWS, ERROR, ERRMSG )
///
///     C
///     C     Check whether an error occurred while processing the
///     C     SELECT clause. If so, output the error message.
///     C
///           IF ( ERROR ) THEN
///
///              WRITE(*,*) 'SELECT clause error: ', ERRMSG
///
///           ELSE
///
///              DO ROW = 1, NMROWS
///
///                 WRITE(*,*) ' '
///                 WRITE(*,'(A,I3)') 'ROW  = ', ROW
///
///     C
///     C           Fetch values from column INT_COL_1.  Since
///     C           INT_COL_1 was the first column selected, the
///     C           selection index SELIDX is set to 1.
///     C
///                 SELIDX = 1
///                 ELTIDX = 1
///                 CALL EKGI ( SELIDX,    ROW,     ELTIDX,
///          .                  IVALS(1),  ISNULL,  FOUND   )
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE(*,*) '  COLUMN = INT_COL_1: <Null>'
///
///                 ELSE
///
///                    WRITE(*,'(A,I6)') '   COLUMN = INT_COL_1:',
///          .                           IVALS(1)
///
///                 END IF
///
///     C
///     C           Fetch values from column INT_COL_2 in the current
///     C           row.  Since INT_COL_2 contains variable-size array
///     C           elements, we call EKNELT to determine how many
///     C           elements to fetch.
///     C
///                 SELIDX = 2
///                 CALL EKNELT ( SELIDX, ROW, NELT )
///
///                 ELTIDX = 1
///                 ISNULL = .FALSE.
///
///                 DO WHILE (       ( ELTIDX .LE.  NELT   )
///          .                 .AND. (        .NOT. ISNULL )  )
///
///                    CALL EKGI ( SELIDX,         ROW,     ELTIDX,
///          .                     IVALS(ELTIDX),  ISNULL,  FOUND   )
///
///                    ELTIDX = ELTIDX + 1
///
///     C
///     C           If the column entry is null, we'll be kicked
///     C           out of this loop after the first iteration.
///     C
///                 END DO
///
///                 IF ( ISNULL ) THEN
///                    WRITE(*,*) '  COLUMN = INT_COL_2: <Null>'
///                 ELSE
///
///                    WRITE(*,'(A,4I6)') '   COLUMN = INT_COL_2:',
///          .                            ( IVALS(I), I = 1, NELT )
///
///                 END IF
///
///     C
///     C           Fetch values from column INT_COL_3 in the current
///     C           row.  We need not call EKNELT since we know how
///     C           many elements are in each column entry.
///     C
///                 SELIDX = 3
///                 ELTIDX = 1
///                 ISNULL = .FALSE.
///
///                 DO WHILE (       ( ELTIDX .LE.  COL3SZ )
///          .                 .AND. (        .NOT. ISNULL )  )
///
///                    CALL EKGI ( SELIDX,         ROW,     ELTIDX,
///          .                     IVALS(ELTIDX),  ISNULL,  FOUND   )
///
///                    ELTIDX = ELTIDX + 1
///
///                 END DO
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE(*,*) '  COLUMN = INT_COL_3: <Null>'
///
///                 ELSE
///
///                    WRITE(*,'(A,3I6)') '   COLUMN = INT_COL_3:',
///          .                            ( IVALS(I), I = 1, COL3SZ )
///
///                 END IF
///
///              END DO
///
///     C
///     C     We either parsed the SELECT clause or had an error.
///     C
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     ROW  =   1
///        COLUMN = INT_COL_1:   100
///        COLUMN = INT_COL_2:   201
///        COLUMN = INT_COL_3:   101   201   301
///
///     ROW  =   2
///        COLUMN = INT_COL_1:   200
///        COLUMN = INT_COL_2:   401   402
///        COLUMN = INT_COL_3:   102   202   302
///
///     ROW  =   3
///        COLUMN = INT_COL_1:   300
///        COLUMN = INT_COL_2:   601   602   603
///        COLUMN = INT_COL_3:   103   203   303
///
///     ROW  =   4
///        COLUMN = INT_COL_1:   400
///        COLUMN = INT_COL_2:   801   802   803   804
///        COLUMN = INT_COL_3:   104   204   304
///
///
///     Note that after run completion, a new EK file exists in the
///     output directory.
///
///
///  3) See $Examples in EKQMGR.
///
///     In this example, the names and data types of the columns from
///     which to fetch data are not known in advance.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code examples from existing fragments.
///
/// -    SPICELIB Version 2.0.1, 22-SEP-2004 (EDW)
///
///         Edited version 1.1.0 entry to not include
///         the token used to mark the $Procedure section.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (NJB)
///
///         Bug fix: When an already loaded kernel is opened with EKOPR,
///         it now has its link count reset to 1 via a call to EKCLS.
///
/// -    SPICELIB Version 1.1.0, 07-JUL-1996 (NJB)
///
///         Redundant CHKIN call removed from SELIDX error check.
///         Misspelling of "issued" was fixed. Previous version line
///         was changed from "Beta" to "SPICELIB." Header $Procedure
///         line was corrected to indicate integer data type.
///
/// -    SPICELIB Version 1.0.0, 23-OCT-1995 (NJB)
/// ```
pub fn ekgi(
    ctx: &mut SpiceContext,
    selidx: i32,
    row: i32,
    elment: i32,
    idata: &mut i32,
    null: &mut bool,
    found: &mut bool,
) -> crate::Result<()> {
    EKGI(selidx, row, elment, idata, null, found, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKGI  ( EK, get event data, integer )
pub fn EKGI(
    SELIDX: i32,
    ROW: i32,
    ELMENT: i32,
    IDATA: &mut i32,
    NULL: &mut bool,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"EKGI", ctx)?;
    }

    //
    // Nothing found yet.
    //
    *FOUND = false;

    //
    // There nothing to fetch if no files are loaded.  A sure
    // symptom of this problem is that the file list is empty.
    //
    if (save.FTHEAD <= 0) {
        SETMSG(b"No E-kernels are currently loaded.", ctx);
        SIGERR(b"SPICE(NOLOADEDFILES)", ctx)?;
        CHKOUT(b"EKGI", ctx)?;
        return Ok(());
    }

    //
    // The row number must be valid, or we can't proceed.
    //
    if ((ROW < 1) || (ROW > save.UNROWS)) {
        SETMSG(
            b"Row indices for query result range from 1 to #; requested row index was #.",
            ctx,
        );
        ERRINT(b"#", save.UNROWS, ctx);
        ERRINT(b"#", ROW, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"EKGI", ctx)?;
        return Ok(());
    }

    //
    // The element index must be positive.
    //
    if (ELMENT < 1) {
        SETMSG(b"ELMENT must be positive but was #.", ctx);
        ERRINT(b"#", ELMENT, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"EKGI", ctx)?;
        return Ok(());
    }

    //
    // Make sure the SELECT clause column index is valid.
    //
    if ((SELIDX < 1) || (SELIDX > save.NSEL)) {
        SETMSG(
            b"The SELECT column index # is out of the valid range 1:#",
            ctx,
        );
        ERRINT(b"#", SELIDX, ctx);
        ERRINT(b"#", save.NTAB, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"EKGI", ctx)?;
        return Ok(());
    }

    //
    // COL is the column's index within the parent
    // table's column list.
    //
    save.TABIDX = save.SELTAB[SELIDX];
    save.COL = save.SELCOL[SELIDX];
    save.COLPTR = save.SELCTP[SELIDX];
    save.TAB = save.TPTVEC[save.TABIDX];

    //
    // Make sure the column has integer type.
    //
    if (save.CTTYPS[save.COLPTR] != INT) {
        SETMSG(b"Column # has data type #.", ctx);
        ERRCH(b"#", &save.CTNAMS[save.COLPTR], ctx);
        ERRCH(b"#", &save.CHTYPE[save.CTTYPS[save.COLPTR]], ctx);
        SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
        CHKOUT(b"EKGI", ctx)?;
        return Ok(());
    }

    //
    // If it hasn't been done yet, and if it needs to be done, sort the
    // matching row vectors.
    //
    if save.DOSORT {
        ZZEKJSRT(
            save.USIZE,
            save.UBASE.as_slice(),
            save.NORDER,
            save.OTABS.as_slice(),
            save.OCOLS.as_slice(),
            save.OELTS.as_slice(),
            save.SENSE.as_slice(),
            save.STHAN.as_slice(),
            save.STDSCS.as_slice(),
            save.STDTPT.as_slice(),
            save.DTPOOL.as_slice(),
            save.DTDSCS.as_slice(),
            &mut save.ORDBAS,
            ctx,
        )?;

        save.DOSORT = false;
        save.SORTED = true;
    }

    //
    // Look up the segment vector and row vector for the current row.
    //
    if save.SORTED {
        ZZEKSRD(
            (save.ORDBAS + ROW),
            (save.ORDBAS + ROW),
            std::slice::from_mut(&mut save.I),
            ctx,
        )?;
        ZZEKVCAL(save.I, &mut save.RWVBAS, &mut save.SGVBAS, ctx)?;
    } else {
        ZZEKVCAL(ROW, &mut save.RWVBAS, &mut save.SGVBAS, ctx)?;
    }

    ZZEKSRD(
        (save.RWVBAS + 1),
        (save.RWVBAS + save.NTAB),
        save.ROWVEC.as_slice_mut(),
        ctx,
    )?;
    ZZEKSRD(
        (save.SGVBAS + 1),
        (save.SGVBAS + save.NTAB),
        save.SEGVEC.as_slice_mut(),
        ctx,
    )?;

    //
    // Identify the segment containing the column entry of interest.
    // Obtain the column descriptor for the column.
    //
    save.ROWIDX = save.ROWVEC[save.TABIDX];
    save.SEG = save.SEGVEC[save.TABIDX];

    save.J = save.STDTPT[save.SEG];

    {
        let m1__: i32 = 2;
        let m2__: i32 = save.COL;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = LNKNXT(save.J, save.DTPOOL.as_slice(), ctx)?;
            save.I += m3__;
        }
    }

    //
    // Look up the element.
    //
    ZZEKRSI(
        save.STHAN[save.SEG],
        save.STDSCS.subarray([1, save.SEG]),
        save.DTDSCS.subarray([1, save.J]),
        save.ROWIDX,
        ELMENT,
        IDATA,
        NULL,
        FOUND,
        ctx,
    )?;

    CHKOUT(b"EKGI", ctx)?;
    Ok(())
}
