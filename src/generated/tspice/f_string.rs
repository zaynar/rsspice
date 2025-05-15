//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SMLSIZ: i32 = 8;
const LRGSIZ: i32 = 16;
const NSTR: i32 = 13;

struct SaveVars {
    SSTR: Vec<u8>,
    SSTR1: Vec<u8>,
    SARRAY: ActualCharArray,
    SLJUST: ActualCharArray,
    SUCASE: ActualCharArray,
    SLCASE: ActualCharArray,
    SLJSTU: ActualCharArray,
    SCMPR0: ActualCharArray,
    SCMPR1: ActualCharArray,
    SCMPR2: ActualCharArray,
    SLUCR0: ActualCharArray,
    SLUCR1: ActualCharArray,
    SLUCR2: ActualCharArray,
    LSTR: Vec<u8>,
    LSTR1: Vec<u8>,
    LARRAY: ActualCharArray,
    LLJUST: ActualCharArray,
    LUCASE: ActualCharArray,
    LLCASE: ActualCharArray,
    LLJSTU: ActualCharArray,
    LCMPR0: ActualCharArray,
    LCMPR1: ActualCharArray,
    LCMPR2: ActualCharArray,
    LLUCR0: ActualCharArray,
    LLUCR1: ActualCharArray,
    LLUCR2: ActualCharArray,
    R: i32,
    E: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SSTR = vec![b' '; SMLSIZ as usize];
        let mut SSTR1 = vec![b' '; SMLSIZ as usize];
        let mut SARRAY = ActualCharArray::new(SMLSIZ, 1..=NSTR);
        let mut SLJUST = ActualCharArray::new(SMLSIZ, 1..=NSTR);
        let mut SUCASE = ActualCharArray::new(SMLSIZ, 1..=NSTR);
        let mut SLCASE = ActualCharArray::new(SMLSIZ, 1..=NSTR);
        let mut SLJSTU = ActualCharArray::new(SMLSIZ, 1..=NSTR);
        let mut SCMPR0 = ActualCharArray::new(SMLSIZ, 1..=NSTR);
        let mut SCMPR1 = ActualCharArray::new(SMLSIZ, 1..=NSTR);
        let mut SCMPR2 = ActualCharArray::new(SMLSIZ, 1..=NSTR);
        let mut SLUCR0 = ActualCharArray::new(SMLSIZ, 1..=NSTR);
        let mut SLUCR1 = ActualCharArray::new(SMLSIZ, 1..=NSTR);
        let mut SLUCR2 = ActualCharArray::new(SMLSIZ, 1..=NSTR);
        let mut LSTR = vec![b' '; LRGSIZ as usize];
        let mut LSTR1 = vec![b' '; LRGSIZ as usize];
        let mut LARRAY = ActualCharArray::new(LRGSIZ, 1..=NSTR);
        let mut LLJUST = ActualCharArray::new(LRGSIZ, 1..=NSTR);
        let mut LUCASE = ActualCharArray::new(LRGSIZ, 1..=NSTR);
        let mut LLCASE = ActualCharArray::new(LRGSIZ, 1..=NSTR);
        let mut LLJSTU = ActualCharArray::new(LRGSIZ, 1..=NSTR);
        let mut LCMPR0 = ActualCharArray::new(LRGSIZ, 1..=NSTR);
        let mut LCMPR1 = ActualCharArray::new(LRGSIZ, 1..=NSTR);
        let mut LCMPR2 = ActualCharArray::new(LRGSIZ, 1..=NSTR);
        let mut LLUCR0 = ActualCharArray::new(LRGSIZ, 1..=NSTR);
        let mut LLUCR1 = ActualCharArray::new(LRGSIZ, 1..=NSTR);
        let mut LLUCR2 = ActualCharArray::new(LRGSIZ, 1..=NSTR);
        let mut R: i32 = 0;
        let mut E: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"a A a A "),
                Val::C(b" b   B  "),
                Val::C(b"  c C c "),
                Val::C(b"   d   D"),
                Val::C(b"    e E "),
                Val::C(b"     f  "),
                Val::C(b"      g "),
                Val::C(b"       h"),
                Val::C(b"i-I-i-I "),
                Val::C(b" j---J  "),
                Val::C(b"  k-K-k "),
                Val::C(b"   l---L"),
                Val::C(b"    m-M "),
            ]
            .into_iter();
            SARRAY
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"a A a A "),
                Val::C(b"b   B   "),
                Val::C(b"c C c   "),
                Val::C(b"d   D   "),
                Val::C(b"e E     "),
                Val::C(b"f       "),
                Val::C(b"g       "),
                Val::C(b"h       "),
                Val::C(b"i-I-i-I "),
                Val::C(b"j---J   "),
                Val::C(b"k-K-k   "),
                Val::C(b"l---L   "),
                Val::C(b"m-M     "),
            ]
            .into_iter();
            SLJUST
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"A A A A "),
                Val::C(b" B   B  "),
                Val::C(b"  C C C "),
                Val::C(b"   D   D"),
                Val::C(b"    E E "),
                Val::C(b"     F  "),
                Val::C(b"      G "),
                Val::C(b"       H"),
                Val::C(b"I-I-I-I "),
                Val::C(b" J---J  "),
                Val::C(b"  K-K-K "),
                Val::C(b"   L---L"),
                Val::C(b"    M-M "),
            ]
            .into_iter();
            SUCASE
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"a a a a "),
                Val::C(b" b   b  "),
                Val::C(b"  c c c "),
                Val::C(b"   d   d"),
                Val::C(b"    e e "),
                Val::C(b"     f  "),
                Val::C(b"      g "),
                Val::C(b"       h"),
                Val::C(b"i-i-i-i "),
                Val::C(b" j---j  "),
                Val::C(b"  k-k-k "),
                Val::C(b"   l---l"),
                Val::C(b"    m-m "),
            ]
            .into_iter();
            SLCASE
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"A A A A "),
                Val::C(b"B   B   "),
                Val::C(b"C C C   "),
                Val::C(b"D   D   "),
                Val::C(b"E E     "),
                Val::C(b"F       "),
                Val::C(b"G       "),
                Val::C(b"H       "),
                Val::C(b"I-I-I-I "),
                Val::C(b"J---J   "),
                Val::C(b"K-K-K   "),
                Val::C(b"L---L   "),
                Val::C(b"M-M     "),
            ]
            .into_iter();
            SLJSTU
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"aAaA    "),
                Val::C(b"bB      "),
                Val::C(b"cCc     "),
                Val::C(b"dD      "),
                Val::C(b"eE      "),
                Val::C(b"f       "),
                Val::C(b"g       "),
                Val::C(b"h       "),
                Val::C(b"i-I-i-I "),
                Val::C(b"j---J   "),
                Val::C(b"k-K-k   "),
                Val::C(b"l---L   "),
                Val::C(b"m-M     "),
            ]
            .into_iter();
            SCMPR0
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"a A a A "),
                Val::C(b" b B    "),
                Val::C(b" c C c  "),
                Val::C(b" d D    "),
                Val::C(b" e E    "),
                Val::C(b" f      "),
                Val::C(b" g      "),
                Val::C(b" h      "),
                Val::C(b"i-I-i-I "),
                Val::C(b" j---J  "),
                Val::C(b" k-K-k  "),
                Val::C(b" l---L  "),
                Val::C(b" m-M    "),
            ]
            .into_iter();
            SCMPR1
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"a A a A "),
                Val::C(b" b  B   "),
                Val::C(b"  c C c "),
                Val::C(b"  d  D  "),
                Val::C(b"  e E   "),
                Val::C(b"  f     "),
                Val::C(b"  g     "),
                Val::C(b"  h     "),
                Val::C(b"i-I-i-I "),
                Val::C(b" j---J  "),
                Val::C(b"  k-K-k "),
                Val::C(b"  l---L "),
                Val::C(b"  m-M   "),
            ]
            .into_iter();
            SCMPR2
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"AAAA    "),
                Val::C(b"BB      "),
                Val::C(b"CCC     "),
                Val::C(b"DD      "),
                Val::C(b"EE      "),
                Val::C(b"F       "),
                Val::C(b"G       "),
                Val::C(b"H       "),
                Val::C(b"I-I-I-I "),
                Val::C(b"J---J   "),
                Val::C(b"K-K-K   "),
                Val::C(b"L---L   "),
                Val::C(b"M-M     "),
            ]
            .into_iter();
            SLUCR0
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"A A A A "),
                Val::C(b"B B     "),
                Val::C(b"C C C   "),
                Val::C(b"D D     "),
                Val::C(b"E E     "),
                Val::C(b"F       "),
                Val::C(b"G       "),
                Val::C(b"H       "),
                Val::C(b"I-I-I-I "),
                Val::C(b"J---J   "),
                Val::C(b"K-K-K   "),
                Val::C(b"L---L   "),
                Val::C(b"M-M     "),
            ]
            .into_iter();
            SLUCR1
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"A A A A "),
                Val::C(b"B  B    "),
                Val::C(b"C C C   "),
                Val::C(b"D  D    "),
                Val::C(b"E E     "),
                Val::C(b"F       "),
                Val::C(b"G       "),
                Val::C(b"H       "),
                Val::C(b"I-I-I-I "),
                Val::C(b"J---J   "),
                Val::C(b"K-K-K   "),
                Val::C(b"L---L   "),
                Val::C(b"M-M     "),
            ]
            .into_iter();
            SLUCR2
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"N n N n N n N n "),
                Val::C(b" O+++o+++O+++o  "),
                Val::C(b"  P p P p P p P "),
                Val::C(b"   Q   q   Q   q"),
                Val::C(b"    R r R r R r "),
                Val::C(b"     S   s   S  "),
                Val::C(b"      T t T t T "),
                Val::C(b"       U   u   U"),
                Val::C(b"        V/v/V/v "),
                Val::C(b"         W   w  "),
                Val::C(b"          X x X "),
                Val::C(b"           Y   y"),
                Val::C(b"            Z z "),
            ]
            .into_iter();
            LARRAY
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"N n N n N n N n "),
                Val::C(b"O+++o+++O+++o   "),
                Val::C(b"P p P p P p P   "),
                Val::C(b"Q   q   Q   q   "),
                Val::C(b"R r R r R r     "),
                Val::C(b"S   s   S       "),
                Val::C(b"T t T t T       "),
                Val::C(b"U   u   U       "),
                Val::C(b"V/v/V/v         "),
                Val::C(b"W   w           "),
                Val::C(b"X x X           "),
                Val::C(b"Y   y           "),
                Val::C(b"Z z             "),
            ]
            .into_iter();
            LLJUST
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"N N N N N N N N "),
                Val::C(b" O+++O+++O+++O  "),
                Val::C(b"  P P P P P P P "),
                Val::C(b"   Q   Q   Q   Q"),
                Val::C(b"    R R R R R R "),
                Val::C(b"     S   S   S  "),
                Val::C(b"      T T T T T "),
                Val::C(b"       U   U   U"),
                Val::C(b"        V/V/V/V "),
                Val::C(b"         W   W  "),
                Val::C(b"          X X X "),
                Val::C(b"           Y   Y"),
                Val::C(b"            Z Z "),
            ]
            .into_iter();
            LUCASE
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"n n n n n n n n "),
                Val::C(b" o+++o+++o+++o  "),
                Val::C(b"  p p p p p p p "),
                Val::C(b"   q   q   q   q"),
                Val::C(b"    r r r r r r "),
                Val::C(b"     s   s   s  "),
                Val::C(b"      t t t t t "),
                Val::C(b"       u   u   u"),
                Val::C(b"        v/v/v/v "),
                Val::C(b"         w   w  "),
                Val::C(b"          x x x "),
                Val::C(b"           y   y"),
                Val::C(b"            z z "),
            ]
            .into_iter();
            LLCASE
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"N N N N N N N N "),
                Val::C(b"O+++O+++O+++O   "),
                Val::C(b"P P P P P P P   "),
                Val::C(b"Q   Q   Q   Q   "),
                Val::C(b"R R R R R R     "),
                Val::C(b"S   S   S       "),
                Val::C(b"T T T T T       "),
                Val::C(b"U   U   U       "),
                Val::C(b"V/V/V/V         "),
                Val::C(b"W   W           "),
                Val::C(b"X X X           "),
                Val::C(b"Y   Y           "),
                Val::C(b"Z Z             "),
            ]
            .into_iter();
            LLJSTU
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"NnNnNnNn        "),
                Val::C(b"O+++o+++O+++o   "),
                Val::C(b"PpPpPpP         "),
                Val::C(b"QqQq            "),
                Val::C(b"RrRrRr          "),
                Val::C(b"SsS             "),
                Val::C(b"TtTtT           "),
                Val::C(b"UuU             "),
                Val::C(b"V/v/V/v         "),
                Val::C(b"Ww              "),
                Val::C(b"XxX             "),
                Val::C(b"Yy              "),
                Val::C(b"Zz              "),
            ]
            .into_iter();
            LCMPR0
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"N n N n N n N n "),
                Val::C(b" O+++o+++O+++o  "),
                Val::C(b" P p P p P p P  "),
                Val::C(b" Q q Q q        "),
                Val::C(b" R r R r R r    "),
                Val::C(b" S s S          "),
                Val::C(b" T t T t T      "),
                Val::C(b" U u U          "),
                Val::C(b" V/v/V/v        "),
                Val::C(b" W w            "),
                Val::C(b" X x X          "),
                Val::C(b" Y y            "),
                Val::C(b" Z z            "),
            ]
            .into_iter();
            LCMPR1
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"N n N n N n N n "),
                Val::C(b" O+++o+++O+++o  "),
                Val::C(b"  P p P p P p P "),
                Val::C(b"  Q  q  Q  q    "),
                Val::C(b"  R r R r R r   "),
                Val::C(b"  S  s  S       "),
                Val::C(b"  T t T t T     "),
                Val::C(b"  U  u  U       "),
                Val::C(b"  V/v/V/v       "),
                Val::C(b"  W  w          "),
                Val::C(b"  X x X         "),
                Val::C(b"  Y  y          "),
                Val::C(b"  Z z           "),
            ]
            .into_iter();
            LCMPR2
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"NNNNNNNN        "),
                Val::C(b"O+++O+++O+++O   "),
                Val::C(b"PPPPPPP         "),
                Val::C(b"QQQQ            "),
                Val::C(b"RRRRRR          "),
                Val::C(b"SSS             "),
                Val::C(b"TTTTT           "),
                Val::C(b"UUU             "),
                Val::C(b"V/V/V/V         "),
                Val::C(b"WW              "),
                Val::C(b"XXX             "),
                Val::C(b"YY              "),
                Val::C(b"ZZ              "),
            ]
            .into_iter();
            LLUCR0
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"N N N N N N N N "),
                Val::C(b"O+++O+++O+++O   "),
                Val::C(b"P P P P P P P   "),
                Val::C(b"Q Q Q Q         "),
                Val::C(b"R R R R R R     "),
                Val::C(b"S S S           "),
                Val::C(b"T T T T T       "),
                Val::C(b"U U U           "),
                Val::C(b"V/V/V/V         "),
                Val::C(b"W W             "),
                Val::C(b"X X X           "),
                Val::C(b"Y Y             "),
                Val::C(b"Z Z             "),
            ]
            .into_iter();
            LLUCR1
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"N N N N N N N N "),
                Val::C(b"O+++O+++O+++O   "),
                Val::C(b"P P P P P P P   "),
                Val::C(b"Q  Q  Q  Q      "),
                Val::C(b"R R R R R R     "),
                Val::C(b"S  S  S         "),
                Val::C(b"T T T T T       "),
                Val::C(b"U  U  U         "),
                Val::C(b"V/V/V/V         "),
                Val::C(b"W  W            "),
                Val::C(b"X X X           "),
                Val::C(b"Y  Y            "),
                Val::C(b"Z Z             "),
            ]
            .into_iter();
            LLUCR2
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            SSTR,
            SSTR1,
            SARRAY,
            SLJUST,
            SUCASE,
            SLCASE,
            SLJSTU,
            SCMPR0,
            SCMPR1,
            SCMPR2,
            SLUCR0,
            SLUCR1,
            SLUCR2,
            LSTR,
            LSTR1,
            LARRAY,
            LLJUST,
            LUCASE,
            LLCASE,
            LLJSTU,
            LCMPR0,
            LCMPR1,
            LCMPR2,
            LLUCR0,
            LLUCR1,
            LLUCR2,
            R,
            E,
        }
    }
}

//$Procedure F_STRING ( Family of tests for string routines )
pub fn F_STRING(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Save all
    //

    //
    // Initial values
    //

    //
    // Short string array.
    //

    //
    // Short string array, left-justified.
    //

    //
    // Short string array, upper-cased.
    //

    //
    // Short string array, lower-cased.
    //

    //
    // Short string array, left-justified and upper-cased.
    //

    //
    // Short string array, compressed, 0 spaces.
    //

    //
    // Short string array, compressed, 1 space.
    //

    //
    // Short string array, compressed, 2 spaces.
    //

    //
    // Short string array, left-justified, upper-cased, compressed, 0
    // spaces.
    //

    //
    // Short string array, left-justified, upper-cased, compressed, 1
    // space.
    //

    //
    // Short string array, left-justified, upper-cased, compressed, 2
    // spaces.
    //

    //
    // Long string array.
    //

    //
    // Long string array, left-justified.
    //

    //
    // Long string array, upper-cased,
    //

    //
    // Long string array, lower-cased.
    //

    //
    // Long string array, left-justified, upper-cased.
    //

    //
    // Long string array, compressed, 0 spaces.
    //

    //
    // Long string array, compressed, 1 space.
    //

    //
    // Long string array, compressed, 2 spaces.
    //

    //
    // Long string array, left-justified, upper-cased, compressed, 0
    // spaces.
    //

    //
    // Long string array, left-justified, upper-cased, compressed, 1
    // space.
    //

    //
    // Long string array, left-justified, upper-cased, compressed, 2
    // spaces.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_string", ctx)?;

    //
    // LJUST tests.
    //
    testutil::TCASE(b"LJUST: short to short", ctx)?;
    for I in 1..=NSTR {
        spicelib::LJUST(&save.SARRAY[I], &mut save.SSTR);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SLJUST[I], OK, ctx)?;
    }

    testutil::TCASE(b"LJUST: short to short, same I/O", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR, save.SARRAY.get(I));
        spicelib::LJUST(&save.SSTR.to_vec(), &mut save.SSTR);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SLJUST[I], OK, ctx)?;
    }

    testutil::TCASE(b"LJUST: long to long", ctx)?;
    for I in 1..=NSTR {
        spicelib::LJUST(&save.LARRAY[I], &mut save.LSTR);
        testutil::CHCKSC(b"LARRAY(I)", &save.LSTR, b"=", &save.LLJUST[I], OK, ctx)?;
    }

    testutil::TCASE(b"LJUST: short to long", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.LSTR1, b" ");
        save.R = spicelib::LASTNB(&save.SLJUST[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.LSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.LSTR1, 1..=save.E),
            fstr::substr(save.SLJUST.get(I), 1..=save.E),
        );
        spicelib::LJUST(&save.SARRAY[I], &mut save.LSTR);
        testutil::CHCKSC(b"SARRAY(I)", &save.LSTR, b"=", &save.LSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"LJUST: long to short", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR1, b" ");
        save.R = spicelib::LASTNB(&save.LLJUST[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.SSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.SSTR1, 1..=save.E),
            fstr::substr(save.LLJUST.get(I), 1..=save.E),
        );
        spicelib::LJUST(&save.LARRAY[I], &mut save.SSTR);
        testutil::CHCKSC(b"LARRAY(I)", &save.SSTR, b"=", &save.SSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"LJUST: blank", ctx)?;
    fstr::assign(&mut save.SSTR, b" ");
    fstr::assign(&mut save.SSTR1, b"boo");

    spicelib::LJUST(&save.SSTR, &mut save.SSTR1);
    testutil::CHCKSC(b"SLMST1", &save.SSTR1, b"=", b" ", OK, ctx)?;

    //
    // UCASE tests.
    //
    testutil::TCASE(b"UCASE: short to short", ctx)?;
    for I in 1..=NSTR {
        spicelib::UCASE(&save.SARRAY[I], &mut save.SSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SUCASE[I], OK, ctx)?;
    }

    testutil::TCASE(b"UCASE: short to short, same I/O", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR, save.SARRAY.get(I));
        spicelib::UCASE(&save.SSTR.to_vec(), &mut save.SSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SUCASE[I], OK, ctx)?;
    }

    testutil::TCASE(b"UCASE: long to long", ctx)?;
    for I in 1..=NSTR {
        spicelib::UCASE(&save.LARRAY[I], &mut save.LSTR, ctx);
        testutil::CHCKSC(b"LARRAY(I)", &save.LSTR, b"=", &save.LUCASE[I], OK, ctx)?;
    }

    testutil::TCASE(b"UCASE: short to long", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.LSTR1, b" ");
        save.R = spicelib::LASTNB(&save.SUCASE[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.LSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.LSTR1, 1..=save.E),
            fstr::substr(save.SUCASE.get(I), 1..=save.E),
        );
        spicelib::UCASE(&save.SARRAY[I], &mut save.LSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.LSTR, b"=", &save.LSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"UCASE: long to short", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR1, b" ");
        save.R = spicelib::LASTNB(&save.LUCASE[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.SSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.SSTR1, 1..=save.E),
            fstr::substr(save.LUCASE.get(I), 1..=save.E),
        );
        spicelib::UCASE(&save.LARRAY[I], &mut save.SSTR, ctx);
        testutil::CHCKSC(b"LARRAY(I)", &save.SSTR, b"=", &save.SSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"UCASE: blank", ctx)?;
    fstr::assign(&mut save.SSTR, b" ");
    fstr::assign(&mut save.SSTR1, b"boo");

    spicelib::UCASE(&save.SSTR, &mut save.SSTR1, ctx);
    testutil::CHCKSC(b"SLMST1", &save.SSTR1, b"=", b" ", OK, ctx)?;

    //
    // LCASE tests.
    //
    testutil::TCASE(b"LCASE: short to short", ctx)?;
    for I in 1..=NSTR {
        spicelib::LCASE(&save.SARRAY[I], &mut save.SSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SLCASE[I], OK, ctx)?;
    }

    testutil::TCASE(b"LCASE: short to short, same I/O", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR, save.SARRAY.get(I));
        spicelib::LCASE(&save.SSTR.to_vec(), &mut save.SSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SLCASE[I], OK, ctx)?;
    }

    testutil::TCASE(b"LCASE: long to long", ctx)?;
    for I in 1..=NSTR {
        spicelib::LCASE(&save.LARRAY[I], &mut save.LSTR, ctx);
        testutil::CHCKSC(b"LARRAY(I)", &save.LSTR, b"=", &save.LLCASE[I], OK, ctx)?;
    }

    testutil::TCASE(b"LCASE: short to long", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.LSTR1, b" ");
        save.R = spicelib::LASTNB(&save.SLCASE[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.LSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.LSTR1, 1..=save.E),
            fstr::substr(save.SLCASE.get(I), 1..=save.E),
        );
        spicelib::LCASE(&save.SARRAY[I], &mut save.LSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.LSTR, b"=", &save.LSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"LCASE: long to short", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR1, b" ");
        save.R = spicelib::LASTNB(&save.LLCASE[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.SSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.SSTR1, 1..=save.E),
            fstr::substr(save.LLCASE.get(I), 1..=save.E),
        );
        spicelib::LCASE(&save.LARRAY[I], &mut save.SSTR, ctx);
        testutil::CHCKSC(b"LARRAY(I)", &save.SSTR, b"=", &save.SSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"LCASE: blank", ctx)?;
    fstr::assign(&mut save.SSTR, b" ");
    fstr::assign(&mut save.SSTR1, b"boo");

    spicelib::LCASE(&save.SSTR, &mut save.SSTR1, ctx);
    testutil::CHCKSC(b"SLMST1", &save.SSTR1, b"=", b" ", OK, ctx)?;

    //
    // CMPRSS tests.
    //
    testutil::TCASE(b"CMPRSS: short to short, 0", ctx)?;
    for I in 1..=NSTR {
        spicelib::CMPRSS(b" ", 0, &save.SARRAY[I], &mut save.SSTR);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SCMPR0[I], OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: short to short, 1", ctx)?;
    for I in 1..=NSTR {
        spicelib::CMPRSS(b" ", 1, &save.SARRAY[I], &mut save.SSTR);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SCMPR1[I], OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: short to short, 2", ctx)?;
    for I in 1..=NSTR {
        spicelib::CMPRSS(b" ", 2, &save.SARRAY[I], &mut save.SSTR);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SCMPR2[I], OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: short to short, too many", ctx)?;
    for I in 1..=NSTR {
        spicelib::CMPRSS(b" ", (SMLSIZ + 1), &save.SARRAY[I], &mut save.SSTR);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SARRAY[I], OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: short to short, same I/O, 0", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR, save.SARRAY.get(I));
        spicelib::CMPRSS(b" ", 0, &save.SSTR.to_vec(), &mut save.SSTR);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SCMPR0[I], OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: short to short, same I/O, 1", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR, save.SARRAY.get(I));
        spicelib::CMPRSS(b" ", 1, &save.SSTR.to_vec(), &mut save.SSTR);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SCMPR1[I], OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: short to short, same I/O, 2", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR, save.SARRAY.get(I));
        spicelib::CMPRSS(b" ", 2, &save.SSTR.to_vec(), &mut save.SSTR);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SCMPR2[I], OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: short to short, same I/O, too many", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR, save.SARRAY.get(I));
        spicelib::CMPRSS(b" ", (SMLSIZ + 1), &save.SSTR.to_vec(), &mut save.SSTR);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SARRAY[I], OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: long to long, 0", ctx)?;
    for I in 1..=NSTR {
        spicelib::CMPRSS(b" ", 0, &save.LARRAY[I], &mut save.LSTR);
        testutil::CHCKSC(b"LARRAY(I)", &save.LSTR, b"=", &save.LCMPR0[I], OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: long to long, 1", ctx)?;
    for I in 1..=NSTR {
        spicelib::CMPRSS(b" ", 1, &save.LARRAY[I], &mut save.LSTR);
        testutil::CHCKSC(b"LARRAY(I)", &save.LSTR, b"=", &save.LCMPR1[I], OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: long to long, 2", ctx)?;
    for I in 1..=NSTR {
        spicelib::CMPRSS(b" ", 2, &save.LARRAY[I], &mut save.LSTR);
        testutil::CHCKSC(b"LARRAY(I)", &save.LSTR, b"=", &save.LCMPR2[I], OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: long to long, too many", ctx)?;
    for I in 1..=NSTR {
        spicelib::CMPRSS(b" ", (LRGSIZ + 1), &save.LARRAY[I], &mut save.LSTR);
        testutil::CHCKSC(b"LARRAY(I)", &save.LSTR, b"=", &save.LARRAY[I], OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: short to long, 0", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.LSTR1, b" ");
        save.R = spicelib::LASTNB(&save.SCMPR0[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.LSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.LSTR1, 1..=save.E),
            fstr::substr(save.SCMPR0.get(I), 1..=save.E),
        );
        spicelib::CMPRSS(b" ", 0, &save.SARRAY[I], &mut save.LSTR);
        testutil::CHCKSC(b"SARRAY(I)", &save.LSTR, b"=", &save.LSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: short to long, 1", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.LSTR1, b" ");
        save.R = spicelib::LASTNB(&save.SCMPR1[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.LSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.LSTR1, 1..=save.E),
            fstr::substr(save.SCMPR1.get(I), 1..=save.E),
        );
        spicelib::CMPRSS(b" ", 1, &save.SARRAY[I], &mut save.LSTR);
        testutil::CHCKSC(b"SARRAY(I)", &save.LSTR, b"=", &save.LSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: short to long, 2", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.LSTR1, b" ");
        save.R = spicelib::LASTNB(&save.SCMPR2[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.LSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.LSTR1, 1..=save.E),
            fstr::substr(save.SCMPR2.get(I), 1..=save.E),
        );
        spicelib::CMPRSS(b" ", 2, &save.SARRAY[I], &mut save.LSTR);
        testutil::CHCKSC(b"SARRAY(I)", &save.LSTR, b"=", &save.LSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: short to long, too many", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.LSTR1, b" ");
        save.R = spicelib::LASTNB(&save.SARRAY[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.LSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.LSTR1, 1..=save.E),
            fstr::substr(save.SARRAY.get(I), 1..=save.E),
        );
        spicelib::CMPRSS(b" ", (SMLSIZ + 1), &save.SARRAY[I], &mut save.LSTR);
        testutil::CHCKSC(b"SARRAY(I)", &save.LSTR, b"=", &save.LSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: long to short, 0", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR1, b" ");
        save.R = spicelib::LASTNB(&save.LCMPR0[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.SSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.SSTR1, 1..=save.E),
            fstr::substr(save.LCMPR0.get(I), 1..=save.E),
        );
        spicelib::CMPRSS(b" ", 0, &save.LARRAY[I], &mut save.SSTR);
        testutil::CHCKSC(b"LARRAY(I)", &save.SSTR, b"=", &save.SSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: long to short, 1", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR1, b" ");
        save.R = spicelib::LASTNB(&save.LCMPR1[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.SSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.SSTR1, 1..=save.E),
            fstr::substr(save.LCMPR1.get(I), 1..=save.E),
        );
        spicelib::CMPRSS(b" ", 1, &save.LARRAY[I], &mut save.SSTR);
        testutil::CHCKSC(b"LARRAY(I)", &save.SSTR, b"=", &save.SSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: long to short, 2", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR1, b" ");
        save.R = spicelib::LASTNB(&save.LCMPR2[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.SSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.SSTR1, 1..=save.E),
            fstr::substr(save.LCMPR2.get(I), 1..=save.E),
        );
        spicelib::CMPRSS(b" ", 2, &save.LARRAY[I], &mut save.SSTR);
        testutil::CHCKSC(b"LARRAY(I)", &save.SSTR, b"=", &save.SSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: long to short, too many", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR1, b" ");
        save.R = spicelib::LASTNB(&save.LARRAY[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.SSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.SSTR1, 1..=save.E),
            fstr::substr(save.LARRAY.get(I), 1..=save.E),
        );
        spicelib::CMPRSS(b" ", (LRGSIZ + 1), &save.LARRAY[I], &mut save.SSTR);
        testutil::CHCKSC(b"LARRAY(I)", &save.SSTR, b"=", &save.SSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"CMPRSS: blank, 0", ctx)?;
    fstr::assign(&mut save.SSTR, b" ");
    fstr::assign(&mut save.SSTR1, b"boo");

    spicelib::CMPRSS(b" ", 0, &save.SSTR, &mut save.SSTR1);
    testutil::CHCKSC(b"SLMST1", &save.SSTR1, b"=", b" ", OK, ctx)?;

    testutil::TCASE(b"CMPRSS: blank, 1", ctx)?;
    fstr::assign(&mut save.SSTR, b" ");
    fstr::assign(&mut save.SSTR1, b"boo");

    spicelib::CMPRSS(b" ", 1, &save.SSTR, &mut save.SSTR1);
    testutil::CHCKSC(b"SLMST1", &save.SSTR1, b"=", b" ", OK, ctx)?;

    testutil::TCASE(b"CMPRSS: blank, too many", ctx)?;
    fstr::assign(&mut save.SSTR, b" ");
    fstr::assign(&mut save.SSTR1, b"boo");

    spicelib::CMPRSS(b" ", (SMLSIZ + 1), &save.SSTR, &mut save.SSTR1);
    testutil::CHCKSC(b"SLMST1", &save.SSTR1, b"=", b" ", OK, ctx)?;

    //
    // LJUCRS tests.
    //
    testutil::TCASE(b"LJUCRS: short to short, 0", ctx)?;
    for I in 1..=NSTR {
        spicelib::LJUCRS(0, &save.SARRAY[I], &mut save.SSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SLUCR0[I], OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: short to short, 1", ctx)?;
    for I in 1..=NSTR {
        spicelib::LJUCRS(1, &save.SARRAY[I], &mut save.SSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SLUCR1[I], OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: short to short, 2", ctx)?;
    for I in 1..=NSTR {
        spicelib::LJUCRS(2, &save.SARRAY[I], &mut save.SSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SLUCR2[I], OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: short to short, too many", ctx)?;
    for I in 1..=NSTR {
        spicelib::LJUCRS((SMLSIZ + 1), &save.SARRAY[I], &mut save.SSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SLJSTU[I], OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: short to short, same I/O, 0", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR, save.SARRAY.get(I));
        spicelib::LJUCRS(0, &save.SSTR.to_vec(), &mut save.SSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SLUCR0[I], OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: short to short, same I/O, 1", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR, save.SARRAY.get(I));
        spicelib::LJUCRS(1, &save.SSTR.to_vec(), &mut save.SSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SLUCR1[I], OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: short to short, same I/O, 2", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR, save.SARRAY.get(I));
        spicelib::LJUCRS(2, &save.SSTR.to_vec(), &mut save.SSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SLUCR2[I], OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: short to short, same I/O, too many", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR, save.SARRAY.get(I));
        spicelib::LJUCRS((SMLSIZ + 1), &save.SSTR.to_vec(), &mut save.SSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.SSTR, b"=", &save.SLJSTU[I], OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: long to long, 0", ctx)?;
    for I in 1..=NSTR {
        spicelib::LJUCRS(0, &save.LARRAY[I], &mut save.LSTR, ctx);
        testutil::CHCKSC(b"LARRAY(I)", &save.LSTR, b"=", &save.LLUCR0[I], OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: long to long, 1", ctx)?;
    for I in 1..=NSTR {
        spicelib::LJUCRS(1, &save.LARRAY[I], &mut save.LSTR, ctx);
        testutil::CHCKSC(b"LARRAY(I)", &save.LSTR, b"=", &save.LLUCR1[I], OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: long to long, 2", ctx)?;
    for I in 1..=NSTR {
        spicelib::LJUCRS(2, &save.LARRAY[I], &mut save.LSTR, ctx);
        testutil::CHCKSC(b"LARRAY(I)", &save.LSTR, b"=", &save.LLUCR2[I], OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: long to long, too many", ctx)?;
    for I in 1..=NSTR {
        spicelib::LJUCRS((LRGSIZ + 1), &save.LARRAY[I], &mut save.LSTR, ctx);
        testutil::CHCKSC(b"LARRAY(I)", &save.LSTR, b"=", &save.LLJSTU[I], OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: short to long, 0", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.LSTR1, b" ");
        save.R = spicelib::LASTNB(&save.SLUCR0[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.LSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.LSTR1, 1..=save.E),
            fstr::substr(save.SLUCR0.get(I), 1..=save.E),
        );
        spicelib::LJUCRS(0, &save.SARRAY[I], &mut save.LSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.LSTR, b"=", &save.LSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: short to long, 1", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.LSTR1, b" ");
        save.R = spicelib::LASTNB(&save.SLUCR1[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.LSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.LSTR1, 1..=save.E),
            fstr::substr(save.SLUCR1.get(I), 1..=save.E),
        );
        spicelib::LJUCRS(1, &save.SARRAY[I], &mut save.LSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.LSTR, b"=", &save.LSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: short to long, 2", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.LSTR1, b" ");
        save.R = spicelib::LASTNB(&save.SLUCR2[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.LSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.LSTR1, 1..=save.E),
            fstr::substr(save.SLUCR2.get(I), 1..=save.E),
        );
        spicelib::LJUCRS(2, &save.SARRAY[I], &mut save.LSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.LSTR, b"=", &save.LSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: short to long, too many", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.LSTR1, b" ");
        save.R = spicelib::LASTNB(&save.SLJSTU[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.LSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.LSTR1, 1..=save.E),
            fstr::substr(save.SLJSTU.get(I), 1..=save.E),
        );
        spicelib::LJUCRS((SMLSIZ + 1), &save.SARRAY[I], &mut save.LSTR, ctx);
        testutil::CHCKSC(b"SARRAY(I)", &save.LSTR, b"=", &save.LSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: long to short, 0", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR1, b" ");
        save.R = spicelib::LASTNB(&save.LLUCR0[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.SSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.SSTR1, 1..=save.E),
            fstr::substr(save.LLUCR0.get(I), 1..=save.E),
        );
        spicelib::LJUCRS(0, &save.LARRAY[I], &mut save.SSTR, ctx);
        testutil::CHCKSC(b"LARRAY(I)", &save.SSTR, b"=", &save.SSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: long to short, 1", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR1, b" ");
        save.R = spicelib::LASTNB(&save.LLUCR1[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.SSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.SSTR1, 1..=save.E),
            fstr::substr(save.LLUCR1.get(I), 1..=save.E),
        );
        spicelib::LJUCRS(1, &save.LARRAY[I], &mut save.SSTR, ctx);
        testutil::CHCKSC(b"LARRAY(I)", &save.SSTR, b"=", &save.SSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: long to short, 2", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR1, b" ");
        save.R = spicelib::LASTNB(&save.LLUCR2[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.SSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.SSTR1, 1..=save.E),
            fstr::substr(save.LLUCR2.get(I), 1..=save.E),
        );
        spicelib::LJUCRS(2, &save.LARRAY[I], &mut save.SSTR, ctx);
        testutil::CHCKSC(b"LARRAY(I)", &save.SSTR, b"=", &save.SSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: long to short, too many", ctx)?;
    for I in 1..=NSTR {
        fstr::assign(&mut save.SSTR1, b" ");
        save.R = spicelib::LASTNB(&save.LLJSTU[I]);
        save.E = intrinsics::MIN0(&[save.R, intrinsics::LEN(&save.SSTR1)]);
        fstr::assign(
            fstr::substr_mut(&mut save.SSTR1, 1..=save.E),
            fstr::substr(save.LLJSTU.get(I), 1..=save.E),
        );
        spicelib::LJUCRS((LRGSIZ + 1), &save.LARRAY[I], &mut save.SSTR, ctx);
        testutil::CHCKSC(b"LARRAY(I)", &save.SSTR, b"=", &save.SSTR1, OK, ctx)?;
    }

    testutil::TCASE(b"LJUCRS: blank, 0", ctx)?;
    fstr::assign(&mut save.SSTR, b" ");
    fstr::assign(&mut save.SSTR1, b"boo");

    spicelib::LJUCRS(0, &save.SSTR, &mut save.SSTR1, ctx);
    testutil::CHCKSC(b"SLMST1", &save.SSTR1, b"=", b" ", OK, ctx)?;

    testutil::TCASE(b"LJUCRS: blank, 1", ctx)?;
    fstr::assign(&mut save.SSTR, b" ");
    fstr::assign(&mut save.SSTR1, b"boo");

    spicelib::LJUCRS(1, &save.SSTR, &mut save.SSTR1, ctx);
    testutil::CHCKSC(b"SLMST1", &save.SSTR1, b"=", b" ", OK, ctx)?;

    testutil::TCASE(b"LJUCRS: blank, too many", ctx)?;
    fstr::assign(&mut save.SSTR, b" ");
    fstr::assign(&mut save.SSTR1, b"boo");

    spicelib::LJUCRS((SMLSIZ + 1), &save.SSTR, &mut save.SSTR1, ctx);
    testutil::CHCKSC(b"SLMST1", &save.SSTR1, b"=", b" ", OK, ctx)?;

    //
    // This is good enough for now.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
