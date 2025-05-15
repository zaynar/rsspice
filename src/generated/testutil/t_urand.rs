//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const EPS: f64 = 0.000000000000001;
const RNMX: f64 = (1.0 - EPS);
const IM1: i32 = 2147483563;
const IM2: i32 = 2147483399;
const AM: f64 = (1.0 / IM1 as f64);
const IMM1: i32 = (IM1 - 1);
const IA1: i32 = 40014;
const IA2: i32 = 40692;
const IQ1: i32 = 53668;
const IQ2: i32 = 52774;
const IR1: i32 = 12211;
const IR2: i32 = 3791;
const NTAB: i32 = 32;
const NDIV: i32 = (1 + (IMM1 / NTAB));

struct SaveVars {
    IDUM2: i32,
    IV: StackArray<i32, 32>,
    IY: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut IDUM2: i32 = 0;
        let mut IV = StackArray::<i32, 32>::new(1..=NTAB);
        let mut IY: i32 = 0;

        IDUM2 = 123456789;
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), NTAB as usize))
                .chain([]);

            IV.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        IY = 0;

        Self { IDUM2, IV, IY }
    }
}

//$Procedure      T_URAND ( Random double precision number )
pub fn T_URAND(SEED: &mut i32, ctx: &mut Context) -> f64 {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut T_URAND: f64 = 0.0;
    let mut IDUM: i32 = 0;
    let mut J: i32 = 0;
    let mut K: i32 = 0;

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
    // Copy SEED to local variable IDUM; we do this to maintain
    // consistency between local variable names here and in the
    // reference [1].
    //
    IDUM = *SEED;

    //
    // The following algorithm is from [1].
    //
    if (IDUM <= 0) {
        //
        // Initialize.
        //
        // Be sure to prevent IDUM = 0.
        //
        IDUM = intrinsics::MAX0(&[-IDUM, 1]);
        save.IDUM2 = IDUM;

        {
            let m1__: i32 = (NTAB + 8);
            let m2__: i32 = 1;
            let m3__: i32 = -1;
            J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Load the shuffle table (after 8 warm-ups).
                //
                K = (IDUM / IQ1);
                //
                // Compute IDUM = MOD ( IA1*IDUM, IM1 ) without overflows
                // by Schrage's method.
                //
                IDUM = ((IA1 * (IDUM - (K * IQ1))) - (K * IR1));

                if (IDUM < 0) {
                    IDUM = (IDUM + IM1);
                }

                if (J <= NTAB) {
                    save.IV[J] = IDUM;
                }

                J += m3__;
            }
        }

        save.IY = save.IV[1];
    }

    //
    // Start here when not initializing.
    //
    K = (IDUM / IQ1);

    //
    // Compute IDUM = MOD ( IA1*IDUM, IM1 ) without overflows
    // by Schrage's method.
    //
    IDUM = ((IA1 * (IDUM - (K * IQ1))) - (K * IR1));

    if (IDUM < 0) {
        IDUM = (IDUM + IM1);
    }

    //
    // Compute IDUM2 = MOD ( IA2*IDUM2, IM2 ) likewise.
    //
    K = (save.IDUM2 / IQ2);

    save.IDUM2 = ((IA2 * (save.IDUM2 - (K * IQ2))) - (K * IR2));

    if (save.IDUM2 < 0) {
        save.IDUM2 = (save.IDUM2 + IM2);
    }

    //
    // Compute table index J (in the range 1:NTAB).
    //
    J = (1 + (save.IY / NDIV));

    //
    // Here IDUM is shuffled, IDUM and IDUM2 are combined
    // to generate output.
    //
    save.IY = (save.IV[J] - save.IDUM2);
    save.IV[J] = IDUM;

    if (save.IY < 1) {
        save.IY = (save.IY + IMM1);
    }

    //
    // Adjust the output to exclude endpoint values.
    //
    T_URAND = intrinsics::DMIN1(&[(AM * save.IY as f64), RNMX]);

    //
    // Update the seed.
    //
    *SEED = IDUM;

    T_URAND
}
