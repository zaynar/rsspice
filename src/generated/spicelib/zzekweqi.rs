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
const LBCELL: i32 = -5;
const NAMLEN: i32 = 32;
const MAXNAM: i32 = 15;

struct SaveVars {
    NAMLST: ActualCharArray,
    TMPNAM: Vec<u8>,
    I: i32,
    NAMIDX: StackArray<i32, 15>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NAMLST = ActualCharArray::new(NAMLEN, 1..=MAXNAM);
        let mut TMPNAM = vec![b' '; NAMLEN as usize];
        let mut I: i32 = 0;
        let mut NAMIDX = StackArray::<i32, 15>::new(1..=MAXNAM);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"ARCHITECTURE"),
                Val::I(EQARCH),
                Val::C(b"INITIALIZED"),
                Val::I(EQINIT),
                Val::C(b"PARSED"),
                Val::I(EQPARS),
                Val::C(b"NAMES_RESOLVED"),
                Val::I(EQNRES),
                Val::C(b"TIMES_RESOLVED"),
                Val::I(EQTRES),
                Val::C(b"SEM_CHECKED"),
                Val::I(EQSCHK),
                Val::C(b"NUM_TABLES"),
                Val::I(EQNTAB),
                Val::C(b"NUM_CONJUNCTIONS"),
                Val::I(EQNCNJ),
                Val::C(b"NUM_CONSTRAINTS"),
                Val::I(EQNCNS),
                Val::C(b"NUM_SELECT_COLS"),
                Val::I(EQNSEL),
                Val::C(b"NUM_ORDERBY_COLS"),
                Val::I(EQNORD),
                Val::C(b"NUM_BUF_SIZE"),
                Val::I(EQNSIZ),
                Val::C(b"FREE_NUM"),
                Val::I(EQNPTR),
                Val::C(b"CHR_BUF_SIZE"),
                Val::I(EQCSIZ),
                Val::C(b"FREE_CHR"),
                Val::I(EQCPTR),
            ]
            .into_iter();
            for I in intrinsics::range(1, MAXNAM, 1) {
                fstr::assign(NAMLST.get_mut(I), clist.next().unwrap().into_str());
                NAMIDX[I] = clist.next().unwrap().into_i32();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            NAMLST,
            TMPNAM,
            I,
            NAMIDX,
        }
    }
}

//$Procedure   ZZEKWEQI ( Private: EK, write to encoded query, integer )
pub fn ZZEKWEQI(
    NAME: &[u8],
    VALUE: i32,
    EQRYI: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut EQRYI = DummyArrayMut::new(EQRYI, LBCELL..);

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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Use discovery check-in.
    //
    //
    // Find the location of the named item.
    //
    LJUST(NAME, &mut save.TMPNAM);
    UCASE(&save.TMPNAM.to_vec(), &mut save.TMPNAM, ctx);

    save.I = ISRCHC(&save.TMPNAM, MAXNAM, save.NAMLST.as_arg());

    if (save.I == 0) {
        CHKIN(b"ZZEKWEQI", ctx)?;
        SETMSG(b"Item # not found.", ctx);
        ERRCH(b"#", NAME, ctx);
        SIGERR(b"SPICE(INVALIDNAME)", ctx)?;
        CHKOUT(b"ZZEKWEQI", ctx)?;
        return Ok(());
    }

    //
    // Do the deed.
    //
    EQRYI[save.NAMIDX[save.I]] = VALUE;

    Ok(())
}
