//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const BUFSIZ: i32 = 3;
const MAXBIT: i32 = 64;

struct SaveVars {
    B: f64,
    B2RV: f64,
    BQ: f64,
    BR: f64,
    BR0: f64,
    C0: f64,
    C1: f64,
    C2: f64,
    C3: f64,
    E: f64,
    EQVEC: StackArray<f64, 3>,
    F: f64,
    FX2: f64,
    H2: f64,
    HVEC: StackArray<f64, 3>,
    KFUN: f64,
    KFUNL: f64,
    KFUNU: f64,
    LOGBND: f64,
    LOWER: f64,
    OLDX: f64,
    PC: f64,
    PCDOT: f64,
    POS: StackArray<f64, 3>,
    Q: f64,
    QOVR0: f64,
    R0: f64,
    RV: f64,
    TMPVEC: StackArray<f64, 3>,
    UPPER: f64,
    VC: f64,
    VCDOT: f64,
    VEL: StackArray<f64, 3>,
    X: f64,
    X2: f64,
    X3: f64,
    SAVEPV: StackArray2D<f64, 18>,
    SAVEGM: StackArray<f64, 3>,
    SBOUND: StackArray<f64, 3>,
    SB2RV: StackArray<f64, 3>,
    SBQ: StackArray<f64, 3>,
    SBR0: StackArray<f64, 3>,
    SF: StackArray<f64, 3>,
    SQOVR0: StackArray<f64, 3>,
    NEWEST: StackArray<i32, 3>,
    NSAVED: i32,
    BOUND: f64,
    FIXED: f64,
    LOGDPM: f64,
    LOGF: f64,
    LOGMXC: f64,
    MAXC: f64,
    ROOTF: f64,
    BUMPED: i32,
    I: i32,
    K: i32,
    MOSTC: i32,
    LCOUNT: i32,
    NEW: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut B: f64 = 0.0;
        let mut B2RV: f64 = 0.0;
        let mut BQ: f64 = 0.0;
        let mut BR: f64 = 0.0;
        let mut BR0: f64 = 0.0;
        let mut C0: f64 = 0.0;
        let mut C1: f64 = 0.0;
        let mut C2: f64 = 0.0;
        let mut C3: f64 = 0.0;
        let mut E: f64 = 0.0;
        let mut EQVEC = StackArray::<f64, 3>::new(1..=3);
        let mut F: f64 = 0.0;
        let mut FX2: f64 = 0.0;
        let mut H2: f64 = 0.0;
        let mut HVEC = StackArray::<f64, 3>::new(1..=3);
        let mut KFUN: f64 = 0.0;
        let mut KFUNL: f64 = 0.0;
        let mut KFUNU: f64 = 0.0;
        let mut LOGBND: f64 = 0.0;
        let mut LOWER: f64 = 0.0;
        let mut OLDX: f64 = 0.0;
        let mut PC: f64 = 0.0;
        let mut PCDOT: f64 = 0.0;
        let mut POS = StackArray::<f64, 3>::new(1..=3);
        let mut Q: f64 = 0.0;
        let mut QOVR0: f64 = 0.0;
        let mut R0: f64 = 0.0;
        let mut RV: f64 = 0.0;
        let mut TMPVEC = StackArray::<f64, 3>::new(1..=3);
        let mut UPPER: f64 = 0.0;
        let mut VC: f64 = 0.0;
        let mut VCDOT: f64 = 0.0;
        let mut VEL = StackArray::<f64, 3>::new(1..=3);
        let mut X: f64 = 0.0;
        let mut X2: f64 = 0.0;
        let mut X3: f64 = 0.0;
        let mut SAVEPV = StackArray2D::<f64, 18>::new(1..=6, 1..=BUFSIZ);
        let mut SAVEGM = StackArray::<f64, 3>::new(1..=BUFSIZ);
        let mut SBOUND = StackArray::<f64, 3>::new(1..=BUFSIZ);
        let mut SB2RV = StackArray::<f64, 3>::new(1..=BUFSIZ);
        let mut SBQ = StackArray::<f64, 3>::new(1..=BUFSIZ);
        let mut SBR0 = StackArray::<f64, 3>::new(1..=BUFSIZ);
        let mut SF = StackArray::<f64, 3>::new(1..=BUFSIZ);
        let mut SQOVR0 = StackArray::<f64, 3>::new(1..=BUFSIZ);
        let mut NEWEST = StackArray::<i32, 3>::new(1..=BUFSIZ);
        let mut NSAVED: i32 = 0;
        let mut BOUND: f64 = 0.0;
        let mut FIXED: f64 = 0.0;
        let mut LOGDPM: f64 = 0.0;
        let mut LOGF: f64 = 0.0;
        let mut LOGMXC: f64 = 0.0;
        let mut MAXC: f64 = 0.0;
        let mut ROOTF: f64 = 0.0;
        let mut BUMPED: i32 = 0;
        let mut I: i32 = 0;
        let mut K: i32 = 0;
        let mut MOSTC: i32 = 0;
        let mut LCOUNT: i32 = 0;
        let mut NEW: bool = false;

        NSAVED = 0;
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(1), Val::I(2), Val::I(3)].into_iter();
            NEWEST
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            B,
            B2RV,
            BQ,
            BR,
            BR0,
            C0,
            C1,
            C2,
            C3,
            E,
            EQVEC,
            F,
            FX2,
            H2,
            HVEC,
            KFUN,
            KFUNL,
            KFUNU,
            LOGBND,
            LOWER,
            OLDX,
            PC,
            PCDOT,
            POS,
            Q,
            QOVR0,
            R0,
            RV,
            TMPVEC,
            UPPER,
            VC,
            VCDOT,
            VEL,
            X,
            X2,
            X3,
            SAVEPV,
            SAVEGM,
            SBOUND,
            SB2RV,
            SBQ,
            SBR0,
            SF,
            SQOVR0,
            NEWEST,
            NSAVED,
            BOUND,
            FIXED,
            LOGDPM,
            LOGF,
            LOGMXC,
            MAXC,
            ROOTF,
            BUMPED,
            I,
            K,
            MOSTC,
            LCOUNT,
            NEW,
        }
    }
}

/// Propagate a two-body solution
///
/// Compute the state of a massless body at time t_0 + DT by applying
/// the two-body force model to a given central mass and a given body
/// state at time t_0.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  GM         I   Gravity of the central mass.
///  PVINIT     I   Initial state from which to propagate a state.
///  DT         I   Time offset from initial state to propagate to.
///  PVPROP     O   The propagated state.
/// ```
///
/// # Detailed Input
///
/// ```text
///  GM       is the gravitational constant G times the mass M of the
///           central body.
///
///  PVINIT   is the state at some specified time relative to the
///           central mass. The mass of the object is assumed to
///           be negligible when compared to the central mass.
///
///  DT       is a offset in time from the time of the initial
///           state to which the two-body state should be
///           propagated. (The units of time and distance must be
///           the same in GM, PVINIT, and DT).
/// ```
///
/// # Detailed Output
///
/// ```text
///  PVPROP   is the two-body propagation of the initial state
///           DT units of time past the epoch of the initial state.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If GM is not positive, the error SPICE(NONPOSITIVEMASS) is
///      signaled.
///
///  2)  If the position of the initial state is the zero vector, the
///      error SPICE(ZEROPOSITION) is signaled.
///
///  3)  If the velocity of the initial state is the zero vector, the
///      error SPICE(ZEROVELOCITY) is signaled.
///
///  4)  If the cross product of the position and velocity of PVINIT
///      has squared length of zero, the error SPICE(NONCONICMOTION)
///      is signaled.
///
///  5)  If DT is so large that there is a danger of floating point
///      overflow during computation, the error SPICE(DTOUTOFRANGE) is
///      signaled and a message is generated describing the problem.
///      The value of DT must be "reasonable". In other words, DT
///      should be less than 10**20 seconds for realistic solar system
///      orbits specified in the MKS system. (The actual bounds on DT
///      are much greater but require substantial computation.) The
///      "reasonableness" of DT is checked at run-time.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine uses a universal variables formulation for the
///  two-body motion of an object in orbit about a central mass. It
///  propagates an initial state to an epoch offset from the
///  epoch of the initial state by time DT.
///
///  This routine does not suffer from the finite precision
///  problems of the machine that are inherent to classical
///  formulations based on the solutions to Kepler's equation:
///
///        n( t - T ) = E - e Sin(E)         elliptic case
///        n( t - T ) = e sinh(F) - F        hyperbolic case
///
///  The derivation used to determine the propagated state is a
///  slight variation of the derivation in Danby's book
///  "Fundamentals of Celestial Mechanics" [1].
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
///  1) When the eccentricity of an orbit is near 1, and the epoch
///     of classical elements is near the epoch of periapse, classical
///     formulations that propagate a state from elements tend to
///     lack robustness due to the finite precision of floating point
///     machines. In those situations it is better to use a universal
///     variables formulation to propagate the state.
///
///     By using this routine, you need not go from a state to
///     elements and back to a state. Instead, you can get the state
///     from an initial state.
///
///     If PVINIT is your initial state and you want the state 3600
///     seconds later, the following call will suffice.
///
///        Look up GM somewhere
///
///        DT = 3600.0D0
///
///        CALL PROP2B ( GM, PVINIT, DT, PVPROP )
///
///     After the call, PVPROP will contain the state of the
///     object 3600 seconds after the time it had state PVINIT.
///
///  2) Use the two-body force model to propagate the state of a
///     massless body orbiting the Earth at 100,000,000 km after half
///     a period.
///
///     In circular two-body motion, the orbital speed is
///
///        s     = sqrt(mu/r)
///
///     where mu is the central mass. After tau/2 = pi*r/s seconds
///     (half period), the state should equal the negative of the
///     original state.
///
///     Example code begins here.
///
///
///           PROGRAM PROP2B_EX2
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      PI
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      MU
///           DOUBLE PRECISION      PVINIT ( 6 )
///           DOUBLE PRECISION      R
///           DOUBLE PRECISION      SPEED
///           DOUBLE PRECISION      STATE  ( 6 )
///           DOUBLE PRECISION      T
///
///     C
///     C     Initial values.
///     C
///           MU    =  3.9860043543609598D+05
///           R     =  1.0D+08
///           SPEED =  SQRT( MU / R )
///           T     =  PI( )*R/SPEED
///
///           PVINIT(1) =  0.0D0
///           PVINIT(2) =  R/SQRT(2.0D0)
///           PVINIT(3) =  R/SQRT(2.0D0)
///           PVINIT(4) =  0.0D0
///           PVINIT(5) = -SPEED/SQRT(2.0D0)
///           PVINIT(6) =  SPEED/SQRT(2.0D0)
///
///     C
///     C     Calculate the state of the body at 0.5 period
///     C     after the epoch.
///     C
///           CALL PROP2B ( MU, PVINIT, T, STATE )
///
///     C
///     C     The `state' vector should equal -pvinit
///     C
///           WRITE(*,*) 'State at t0:'
///           WRITE(*,'(A,3F17.5)') '   R   (km):',
///          .                 PVINIT(1), PVINIT(2), PVINIT(3)
///           WRITE(*,'(A,3F17.5)') '   V (km/s):',
///          .                 PVINIT(4), PVINIT(5), PVINIT(6)
///
///           WRITE(*,*) ' '
///           WRITE(*,*) 'State at tau/2:'
///           WRITE(*,'(A,3F17.5)') '   R   (km):',
///          .                    STATE(1), STATE(2), STATE(3)
///           WRITE(*,'(A,3F17.5)') '   V (km/s):',
///          .                    STATE(4), STATE(5), STATE(6)
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      State at t0:
///        R   (km):          0.00000   70710678.11865   70710678.11865
///        V (km/s):          0.00000         -0.04464          0.04464
///
///      State at tau/2:
///        R   (km):         -0.00000  -70710678.11865  -70710678.11865
///        V (km/s):          0.00000          0.04464         -0.04464
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Users should be sure that GM, PVINIT and DT are all in the
///      same system of units ( for example MKS ).
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  J. Danby, "Fundamentals of Celestial Mechanics," 2nd Edition,
///       pp 168-180, Willman-Bell, 1988.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.2.0, 26-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Removed unnecessary $Revisions section.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
/// -    SPICELIB Version 2.1.0, 31-AUG-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VSCL call.
///
/// -    SPICELIB Version 2.0.1, 22-AUG-2001 (EDW)
///
///         Corrected ENDIF to END IF.
///
/// -    SPICELIB Version 2.0.0, 16-MAY-1995 (WLT)
///
///         The initial guess at a solution to Kepler's equation was
///         modified slightly and a loop counter was added to the
///         bisection loop together with logic that will force termination
///         of the bisection loop.
///
/// -    SPICELIB Version 1.0.0, 10-MAR-1992 (WLT)
/// ```
pub fn prop2b(
    ctx: &mut SpiceContext,
    gm: f64,
    pvinit: &[f64; 6],
    dt: f64,
    pvprop: &mut [f64; 6],
) -> crate::Result<()> {
    PROP2B(gm, pvinit, dt, pvprop, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PROP2B ( Propagate a two-body solution )
pub fn PROP2B(
    GM: f64,
    PVINIT: &[f64],
    DT: f64,
    PVPROP: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let PVINIT = DummyArray::new(PVINIT, 1..=6);
    let mut PVPROP = DummyArrayMut::new(PVPROP, 1..=6);

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Local variables
    //
    //
    // The following quantities are needed in the solution of Kepler's
    // equation and in the propagation of the input state.  They are
    // described as they are introduced in the code below.
    //

    //
    // The variables below store intermediate results that can be
    // reused if PVINIT is supplied more than once to this routine.
    // In this way, the number of redundant computations can be reduced.
    //

    //
    // Variables used to bracket X in our solution of Kepler's equation.
    //

    //
    // Save everything.
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
        CHKIN(b"PROP2B", ctx)?;
    }

    //
    // Life will be easier if we use POS and VEL to hold the state.
    //
    save.POS[1] = PVINIT[1];
    save.POS[2] = PVINIT[2];
    save.POS[3] = PVINIT[3];

    save.VEL[1] = PVINIT[4];
    save.VEL[2] = PVINIT[5];
    save.VEL[3] = PVINIT[6];

    //
    // If we propagate many states from the same initial state,
    // most of the variables used to propagate the state will
    // not change in value.
    //
    // To save time needed to compute these variables, we recompute
    // variables that depend upon the initial state only when the
    // initial state is not one of those already buffered by this
    // routine.
    //
    // Determine whether or not this GM and state are the same as the
    // one of those already buffered.  Note that we look through the
    // saved states and GM from the most recently input values of PVINIT
    // and GM to the oldest saved state and GM.
    //
    // NEWEST(1)  contains the most recently input initial conditions
    // NEWEST(2)  contains the next most recently input initial
    //            conditions etc.
    //
    // Also note that when this routine starts up there will be no
    // buffered states or GMs.  Every time we encounter a new state, we
    // will increment the number of saved states NSAVED until we have
    // BUFSIZ states buffered.  From that point on, when a new state is
    // encountered we will overwrite the oldest buffered state.
    //
    save.I = 0;
    save.NEW = true;

    while ((save.I < save.NSAVED) && save.NEW) {
        save.I = (save.I + 1);
        save.K = save.NEWEST[save.I];

        save.NEW = (((((((PVINIT[1] != save.SAVEPV[[1, save.K]])
            || (PVINIT[2] != save.SAVEPV[[2, save.K]]))
            || (PVINIT[3] != save.SAVEPV[[3, save.K]]))
            || (PVINIT[4] != save.SAVEPV[[4, save.K]]))
            || (PVINIT[5] != save.SAVEPV[[5, save.K]]))
            || (PVINIT[6] != save.SAVEPV[[6, save.K]]))
            || (GM != save.SAVEGM[save.K]));
    }

    if !save.NEW {
        //
        // We update the order vector NEWEST so that the state being
        // used this time becomes the "youngest" state.
        //
        save.K = save.I;
        save.BUMPED = save.NEWEST[save.K];

        {
            let m1__: i32 = save.K;
            let m2__: i32 = 2;
            let m3__: i32 = -1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.NEWEST[save.I] = save.NEWEST[(save.I - 1)];
                save.I += m3__;
            }
        }

        save.NEWEST[1] = save.BUMPED;
        save.K = save.BUMPED;

        //
        // Now look up all of the other saved quantities.
        //
        save.B2RV = save.SB2RV[save.K];
        save.BOUND = save.SBOUND[save.K];
        save.BQ = save.SBQ[save.K];
        save.BR0 = save.SBR0[save.K];
        save.F = save.SF[save.K];
        save.QOVR0 = save.SQOVR0[save.K];
    } else {
        //
        // We have a new state, new GM or both.  First let's make sure
        // there is nothing obviously wrong with them.  (We buffer
        // only states, GMs and intermediate values that are "good.")
        // First check for nonpositive mass.
        //
        if (GM <= 0.0) {
            SIGERR(b"SPICE(NONPOSITIVEMASS)", ctx)?;
            CHKOUT(b"PROP2B", ctx)?;
            return Ok(());
        }

        //
        // Next for a zero position vector
        //
        if VZERO(save.POS.as_slice()) {
            SIGERR(b"SPICE(ZEROPOSITION)", ctx)?;
            CHKOUT(b"PROP2B", ctx)?;
            return Ok(());
        }

        //
        // Finally for a zero velocity vector
        //
        if VZERO(save.VEL.as_slice()) {
            SIGERR(b"SPICE(ZEROVELOCITY)", ctx)?;
            CHKOUT(b"PROP2B", ctx)?;
            return Ok(());
        }

        //
        // Obvious problems have been checked. Here are the relevant
        // equations. Let ...
        //
        //    GM        be the gravitational attraction of the central
        //              mass.
        //
        //    POS and   be the initial position and velocity respectively
        //    VEL       of the orbiting object.
        //
        //    R0       be the magnitude of the position vector POS
        //
        //    RV       be the value of the dot product  POS * VEL
        //
        save.R0 = VNORM(save.POS.as_slice());
        save.RV = VDOT(save.POS.as_slice(), save.VEL.as_slice());

        //
        // Let HVEC be the specific angular momentum vector and let Q be
        // the distance at periapse.
        //
        //            1)    HVEC  =   POS  x  VEL
        //
        //                                2
        //            2)    H2    = |HVEC|  =  GM*(1+E)*Q
        //
        //
        VCRSS(
            save.POS.as_slice(),
            save.VEL.as_slice(),
            save.HVEC.as_slice_mut(),
        );
        save.H2 = VDOT(save.HVEC.as_slice(), save.HVEC.as_slice());

        //
        // Let's make sure we are not in the pathological case of
        // rectilinear motion.
        //
        if (save.H2 == 0 as f64) {
            SIGERR(b"SPICE(NONCONICMOTION)", ctx)?;
            CHKOUT(b"PROP2B", ctx)?;
            return Ok(());
        }

        //
        // Let E be the eccentricity of the orbit.
        //
        // Let QVEC be the unit vector that points toward perihelion, and
        // let EQVEC be QVEC scaled by E.
        //
        //                            VEL X HVEC      POS
        //             1)  E*QVEC  =  ----------  -   ---
        //                                GM           R0
        //
        //
        //                                  VEL X HVEC      POS
        //             2)  E       = NORM ( ----------  -   --- )
        //                                     GM            R0
        //
        //
        VCRSS(
            save.VEL.as_slice(),
            save.HVEC.as_slice(),
            save.TMPVEC.as_slice_mut(),
        );
        VLCOM(
            (1.0 / GM),
            save.TMPVEC.as_slice(),
            -(1.0 / save.R0),
            save.POS.as_slice(),
            save.EQVEC.as_slice_mut(),
        );

        save.E = VNORM(save.EQVEC.as_slice());

        //
        // Solve the equation H2 = GM*Q*(1+E) for Q.
        //
        save.Q = (save.H2 / (GM * ((1 as f64) + save.E)));

        //
        // From the discussion of the universal variables formulation in
        // Danby's book on pages 174 and 175 (see the reference listed
        // above) you can show that by making the substitutions
        //
        //       F  =  1 - E
        //
        // and
        //
        //                _____
        //               /  Q
        //       S  =   / -----    X   = B * X
        //            \/   GM
        //
        //
        // that DT satisfies the universal variables Kepler's equation:
        //
        //                            2     2     2        2
        //       DT =  B*R0*X*C_1( F*X ) + B *RV*X C_2( F*X )
        //
        //                                        3        2
        //                               +   B*Q*X C_3( F*X )
        //
        //          =  KFUN( X )
        //
        // (where C_k is used to denote the Stumpff functions. This is
        // the universal variables formulation of Kepler's equation.
        // KFUN is our abbreviation for "Kepler function.")
        //
        // (One might wonder, "Why make such a change of variables?"
        // By making this substitution early in the derivation supplied
        // in Danby's book, you can always deal with physically
        // meaningful quantities --- the pure numeric value of F and the
        // distance of periapse.  Thus one does not need to be concerned
        // about infinite or negative semi-major axes or with discussing
        // how to interpret these somewhat artificial artifacts of the
        // classical derivations for two body motion.)
        //
        // Given the unique X for which this Kepler's equation is
        // satisfied, we can compute the state of the orbiting object
        // at a time DT past the epoch of the state POS and VEL.
        // Evidently we will need the constants:
        //
        save.F = (1.0 - save.E);
        save.B = f64::sqrt((save.Q / GM));
        save.BR0 = (save.B * save.R0);
        save.B2RV = ((save.B * save.B) * save.RV);
        save.BQ = (save.B * save.Q);

        //
        // The state corresponding to the value of X that solves this
        // equation is given by
        //
        //       PC * POS + VC * VEL              ( position )
        //
        // and
        //
        //       PCDOT * POS + VCDOT * VEL        ( velocity )
        //
        // where
        //                                     2        2
        //    ( 1 )    PC    =  1  -  ( Q/R0 )X C_2( F*X )
        //
        //                                     3        2
        //    ( 2 )    VC    =  DT -  ( B*Q  )X C_3( F*X )
        //
        //
        //                                Q               2
        //    ( 3 )    PCDOT =     -  ( ------ ) X C_1( F*X )
        //                              B*R*R0
        //
        //                               B*Q     2        2
        //    ( 4 )    VCDOT =  1  -  (  ---  ) X C_2( F*X )
        //                               B*R
        //
        // Here R denotes the distance from the center of CP*POS + CV*VEL
        // It turns out that R can be computed as:
        //
        //                                 2     2             2
        //    ( 5 )   B*R    = B*R0 C_0(F*X ) + B *RV X C_1(F*X )
        //
        //                                          2       2
        //                                 +   B*Q X C_2(F*X )
        //
        //
        // Therefore we will also need the constant
        //
        save.QOVR0 = (save.Q / save.R0);

        //
        // We will have to find the unique value of X such that
        //
        //      DT = KFUN ( X )
        //
        // where KFUN stands for the "Kepler function" defined by the
        // equation below:
        //
        //                            2
        // KFUN(X) =   B*R0*X * C_1(FX )
        //
        //            2     2        2
        //         + B *RV*X * C_2(FX )
        //
        //                  3        2
        //         +   B*Q*X * C_3(FX )
        //
        //
        // (There is a unique solution to this equation. KFUN(X) is
        // unbounded above and below and is an increasing function
        // over all real X for all non-rectilinear orbits. To see this
        // we note that the variable X is a function of DT and is given
        // by the integral from 0 to DT of the differential:
        //
        //            dt
        //          ------
        //          B*R(t)
        //
        // where R(t) is the range of the body as a function of time.
        // Therefore X is an increasing function of DT, and DT must
        // also be an increasing function of X.
        //
        // Thus, there is a unique value of X  that solves this
        // equation).
        //
        // If F is less than zero, we can have the computation of C0,...
        // overflow.  This is because for X < 0
        //
        //
        //        C_0(X) = COSH( DSQRT(-X) )
        //
        //        C_1(X) = SINH( DSQRT(-X) )
        //                 -----------------
        //                       DSQRT(-X)
        //
        //
        //
        // and from the recursion relationship we know that
        //
        //
        //        C_2(X) =  ( 1/0! - C_0(X) ) / X
        //
        //        C_3(X) =  ( 1/1! - C_1(X) ) / X
        //
        //
        //                  1 - COSH( DSQRT(-X) )
        //        C_2(X) = ------------------------
        //                           X
        //
        //                  1  - SINH( DSQRT(-X) ) / DSQRT(-X)
        //        C_3(X) = -----------------------------------
        //                             X
        //
        // Clearly for negative values of F*X*X having large magnitude,
        // it is easy to get an overflow.
        //
        // In the case when F is less than 0 we choose X so that we can
        // compute all of the following:
        //
        //        | COEF_0 * X**0 * C_0(FX**2) |
        //
        //        | COEF_1 * X**1 * C_1(FX**2) |
        //
        //        | COEF_2 * X**2 * C_2(FX**2) |
        //
        //        | COEF_3 * X**3 * C_3(FX**2) |
        //
        //
        //  where COEF_n are coefficients that will be used in forming
        //  linear combinations of X**n C_n(FX**2) terms.
        //
        //  The variable portion of the last 3 terms above can be
        //  rewritten as:
        //
        //
        //                            SINH ( DSQRT(-F)*|X| )
        // | X**1 * C_1(FX**2) |  =   ----------------------
        //                                   DSQRT(-F)
        //
        //
        //
        //                            1 - COSH( DSQRT(-F)*|X| )
        // | X**2 * C_2(FX**2) |  =  ----------------------------
        //                                      -F
        //
        //
        //                           DSQRT(-F)*|X|   - SINH(DSQRT(-F)*|X|)
        // | X**3 * C_3(FX**2) |  =  -------------------------------------
        //                                       F*DSQRT(-F)
        //
        //
        // For large |X| the absolute values of these expressions are well
        // approximated by
        //
        //                                  0.0
        //        COSH( DSQRT(-F)|X| ) * |F|
        //
        //                                  -0.5
        //        SINH( DSQRT(-F)|X| ) * |F|
        //
        //                                  -1.0
        //        COSH( DSQRT(-F)|X| ) * |F|
        //
        //                                  -1.5
        //        SINH( DSQRT(-F)|X| ) * |F|
        //
        //
        // For large |X| the logarithms of these expressions are well
        // approximated by:
        //
        //
        //        DSQRT(-F)|X| - LOG(2) - 0.0*LOG(-F)
        //
        //        DSQRT(-F)|X| - LOG(2) - 0.5*LOG(-F)
        //
        //        DSQRT(-F)|X| - LOG(2) - 1.0*LOG(-F)
        //
        //        DSQRT(-F)|X| - LOG(2) - 1.5*LOG(-F)
        //
        // respectively.
        //
        //
        // To ensure that we can form a linear combination of these terms
        // we will require that:
        //
        //
        //    |COEF_N*X**N * C_N(FX**2)| < DPMAX / 4
        //
        //
        //
        // for N=0,1,2,3.  This is equivalent to
        //
        //       LOG ( X**N * C_N(FX**2) )   <      LOG ( DPMAX )
        //     + LOG (|COEF_N|)                   - 2 LOG ( 2     )
        //
        //
        //
        // or
        //
        //       LOG ( X**N * C_N(FX**2) )   <      LOG ( DPMAX    )
        //                                      -   LOG ( |COEF_N| )
        //                                      - 2*LOG ( 2        ).
        //
        //
        // Replacing the left hand side with the magnitude expressions
        // computed above we have:
        //
        //     DSQRT(-F)|X| - LOG(2) - N*0.5*LOG( -F )  <   LOG ( DPMAX  )
        //                                               -  LOG (|COEF_N|)
        //                                               -2*LOG ( 2      )
        //
        //  So that:
        //
        //
        //     |X|  <    {   LOG ( DPMAX  )
        //                 - LOG (|COEF_N|)
        //                 - LOG (  2     )
        //                 + LOG ( -F     )*N*0.5 } / DSQRT(-F)
        //
        //  Let MAXC be the maximum of 1.0D0 and the various coefficients
        //  of the Stumpff functions.  We can then set our absolute value
        //  bound on X to be:
        //
        //
        //      MIN        LOG(DPMAX/2) - LOG(MAXC) + (n/2)LOG(-F)
        //     n = 0,3  {  -----------------------------------------  }
        //                        DSQRT(-F)
        //
        // (Actually we know that the minimum must occur for n = 0 or
        // for n = 3).
        //
        //
        save.MAXC = intrinsics::DMAX1(&[
            1.0,
            f64::abs(save.BR0),
            f64::abs(save.B2RV),
            f64::abs(save.BQ),
            f64::abs((save.QOVR0 / save.BQ)),
        ]);

        if (save.F < 0 as f64) {
            save.LOGMXC = f64::ln(save.MAXC);
            save.LOGDPM = f64::ln((DPMAX() / 2.0));

            save.FIXED = (save.LOGDPM - save.LOGMXC);

            save.ROOTF = f64::sqrt(-save.F);
            save.LOGF = f64::ln(-save.F);

            save.BOUND = intrinsics::DMIN1(&[
                (save.FIXED / save.ROOTF),
                ((save.FIXED + (1.5 * save.LOGF)) / save.ROOTF),
            ]);

        //
        // Note that in the above, we can always perform the division
        // by ROOTF.  To see this we note that -F is at least the
        // machine precision (we got it by subtracting E from 1.)
        // Thus its square root is a reasonably large number (if F is
        // 10**-N then ROOTF is 10**(-N/2) )  The value of FIXED is
        // about 3*M where M is the largest exponent such that 2**M
        // is representable on the host machine.  Thus BOUND is at
        // worst M*10**(N/2)  This will always be computable.
        //
        } else {
            //
            //
            // In the case when F is non-negative we must be sure we
            // can compute all of the following.
            //
            //     | COEF_0 * X**0 * C_0(FX**2) | < | COEF_0          |
            //
            //     | COEF_1 * X**1 * C_1(FX**2) | < | COEF_1*|X|      |
            //
            //     | COEF_2 * X**2 * C_2(FX**2) | < | COEF_2*X**2 / 2 |
            //
            //     | COEF_3 * X**3 * C_3(FX**2) | < | COEF_3*X**3 / 6 |
            //
            // If we assume that COEF_0 is computable, all of these are
            // bounded above by:
            //
            //             | MAX(COEF_1,...COEF_3) * X**3 / 6 |
            //
            // We want to make sure we can add these terms so we need to
            // make sure that
            //
            //    | MAX(COEF_1,...,COEF_3) * X**3 / 6 | < DPMAX() / 4.
            //
            // Thus we need:
            //
            //    |X**3| <          1.5*DPMAX / MAX(COEF_1,...,COEF_3)
            //    |X|    <  DCBRT ( 1.5*DPMAX / MAX(COEF_1,...,COEF_3) )
            //
            // (We'll use logarithms to compute the upper bound for |X|.)
            //
            save.LOGBND = (((f64::ln(1.5) + f64::ln(DPMAX())) - f64::ln(save.MAXC)) / 3.0);

            save.BOUND = f64::exp(save.LOGBND);
        }
        //
        // All the obvious problems have been checked, move everybody
        // on the list down and put the new guy on top of the list.
        //

        save.NSAVED = BRCKTI((save.NSAVED + 1), 1, BUFSIZ);
        save.BUMPED = save.NEWEST[save.NSAVED];

        {
            let m1__: i32 = save.NSAVED;
            let m2__: i32 = 2;
            let m3__: i32 = -1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.NEWEST[save.I] = save.NEWEST[(save.I - 1)];
                save.I += m3__;
            }
        }

        save.NEWEST[1] = save.BUMPED;
        save.K = save.BUMPED;

        save.SAVEPV[[1, save.K]] = PVINIT[1];
        save.SAVEPV[[2, save.K]] = PVINIT[2];
        save.SAVEPV[[3, save.K]] = PVINIT[3];
        save.SAVEPV[[4, save.K]] = PVINIT[4];
        save.SAVEPV[[5, save.K]] = PVINIT[5];
        save.SAVEPV[[6, save.K]] = PVINIT[6];
        save.SAVEGM[save.K] = GM;

        //
        // Finally we save the results of all of the above
        // computations so that we won't have to do them again,
        // if this initial state and GM are entered again.
        //
        save.SB2RV[save.K] = save.B2RV;
        save.SBOUND[save.K] = save.BOUND;
        save.SBQ[save.K] = save.BQ;
        save.SBR0[save.K] = save.BR0;
        save.SF[save.K] = save.F;
        save.SQOVR0[save.K] = save.QOVR0;
    }
    //
    //
    // We are now ready to find the unique value of X such that
    //
    //         DT = KFUN ( X )
    //
    // First we must bracket the root. The basic idea is this:
    //
    // 1) KFUN(0) = 0 so we will let one endpoint of our initial
    //    guess of a bracketing interval be 0.
    //
    // 2) We get our initial guess at the other endpoint of the
    //    bracketing interval by recalling that
    //
    //               dt
    //     dX  =   ------
    //             B*R(t)
    //
    //    From this observation it follows that
    //
    //               DT
    //      X  <  -------
    //               B*Q
    //
    //    Thus the solution to
    //
    //         DT = KFUN ( X )
    //
    //    Satisfies
    //
    //                 DT
    //     0 < X  <  -------
    //                B*Q
    //
    //
    //    We now have a guess at a bracketing interval. In the case
    //    DT is positive it looks like
    //
    //            0        X
    //     -------[--------]-----------------------------
    //
    //    This is ok mathematically, but due to rounding etc it is
    //    conceivable that we might not have bracketed the root.
    //    We check and if not we will double the
    //    endpoint farthest from zero and call this X, and make
    //    the other endpoint the old value of X.
    //
    //
    //            0
    //     -------+--------[--------]--------------------
    //
    //
    //    We continue this process ...
    //
    //            0
    //     -------+-----------------[-----------------]--
    //
    //    ...until the root is bracketed. (One shift is certain
    //    to do the job).
    //
    //    If we perform this interval shift, we will have to take
    //    care that X does not run out of the domain for which
    //    we can safely compute KFUN.  Thus we will make sure that
    //    the endpoints of these shifted intervals always stay safely
    //    inside the domain for which KFUN can be computed.
    //
    save.X = (DT / save.BQ);
    save.X = BRCKTD(save.X, -save.BOUND, save.BOUND);
    save.FX2 = ((save.F * save.X) * save.X);

    STMP03(
        save.FX2,
        &mut save.C0,
        &mut save.C1,
        &mut save.C2,
        &mut save.C3,
        ctx,
    )?;

    save.KFUN = (save.X
        * ((save.BR0 * save.C1)
            + (save.X * ((save.B2RV * save.C2) + (save.X * (save.BQ * save.C3))))));

    if (DT < 0 as f64) {
        save.UPPER = 0 as f64;
        save.LOWER = save.X;

        while (save.KFUN > DT) {
            save.UPPER = save.LOWER;
            save.LOWER = (save.LOWER * 2.0);
            save.OLDX = save.X;
            save.X = BRCKTD(save.LOWER, -save.BOUND, save.BOUND);

            //
            // Make sure we are making progress. (In other words make sure
            // we don't run into the boundary of values that X can assume.
            // If we do run into the boundary, X will be unchanged and
            // there's nothing further we can do.  We'll have to call it
            // quits and tell the user what happened.)
            //
            if (save.X == save.OLDX) {
                save.FX2 = ((save.F * save.BOUND) * save.BOUND);

                STMP03(
                    save.FX2,
                    &mut save.C0,
                    &mut save.C1,
                    &mut save.C2,
                    &mut save.C3,
                    ctx,
                )?;

                save.KFUNL = -(save.BOUND
                    * ((save.BR0 * save.C1)
                        - (save.BOUND
                            * ((save.B2RV * save.C2) - ((save.BOUND * save.BQ) * save.C3)))));
                save.KFUNU = (save.BOUND
                    * ((save.BR0 * save.C1)
                        + (save.BOUND
                            * ((save.B2RV * save.C2) + ((save.BOUND * save.BQ) * save.C3)))));

                SETMSG(b"The input delta time (DT) has a value of #.  This is beyond the range of DT for which we can reliably propagate states. The limits for this GM and initial state are from # to #. ", ctx);
                ERRDP(b"#", DT, ctx);
                ERRDP(b"#", save.KFUNL, ctx);
                ERRDP(b"#", save.KFUNU, ctx);
                SIGERR(b"SPICE(DTOUTOFRANGE)", ctx)?;
                CHKOUT(b"PROP2B", ctx)?;
                return Ok(());
            }

            save.FX2 = ((save.F * save.X) * save.X);

            STMP03(
                save.FX2,
                &mut save.C0,
                &mut save.C1,
                &mut save.C2,
                &mut save.C3,
                ctx,
            )?;

            save.KFUN = (save.X
                * ((save.BR0 * save.C1)
                    + (save.X * ((save.B2RV * save.C2) + (save.X * (save.BQ * save.C3))))));
        }
    } else if (DT > 0 as f64) {
        save.LOWER = 0 as f64;
        save.UPPER = save.X;

        while (save.KFUN < DT) {
            save.LOWER = save.UPPER;
            save.UPPER = (save.UPPER * 2.0);
            save.OLDX = save.X;
            save.X = BRCKTD(save.UPPER, -save.BOUND, save.BOUND);
            //
            // Make sure we are making progress.
            //
            if (save.X == save.OLDX) {
                save.FX2 = ((save.F * save.BOUND) * save.BOUND);

                STMP03(
                    save.FX2,
                    &mut save.C0,
                    &mut save.C1,
                    &mut save.C2,
                    &mut save.C3,
                    ctx,
                )?;

                save.KFUNL = -(save.BOUND
                    * ((save.BR0 * save.C1)
                        - (save.BOUND
                            * ((save.B2RV * save.C2) - ((save.BOUND * save.BQ) * save.C3)))));
                save.KFUNU = (save.BOUND
                    * ((save.BR0 * save.C1)
                        + (save.BOUND
                            * ((save.B2RV * save.C2) + ((save.BOUND * save.BQ) * save.C3)))));

                SETMSG(b"The input delta time (DT) has a value of #.  This is beyond the range of DT for which we can reliably propagate states. The limits for this GM and initial state are from # to #. ", ctx);
                ERRDP(b"#", DT, ctx);
                ERRDP(b"#", save.KFUNL, ctx);
                ERRDP(b"#", save.KFUNU, ctx);
                SIGERR(b"SPICE(DTOUTOFRANGE)", ctx)?;
                CHKOUT(b"PROP2B", ctx)?;
                return Ok(());
            }

            save.FX2 = ((save.F * save.X) * save.X);

            STMP03(
                save.FX2,
                &mut save.C0,
                &mut save.C1,
                &mut save.C2,
                &mut save.C3,
                ctx,
            )?;

            save.KFUN = (save.X
                * ((save.BR0 * save.C1)
                    + (save.X * ((save.B2RV * save.C2) + ((save.X * save.BQ) * save.C3)))));
        }
    } else {
        VEQUG(PVINIT.as_slice(), 6, PVPROP.as_slice_mut());
        CHKOUT(b"PROP2B", ctx)?;
        return Ok(());
    }

    //
    // Ok. We've bracketed the root.  Now for lack of anything more
    // clever, we just bisect to find the solution.
    //
    // We add a loop counter so that we can ensure termination of the
    // loop below.
    //
    // On some systems the computed midpoint is stored in an extended
    // precision register.  Thus the midpoint is always different from
    // UPPER and LOWER.  Yet when the new value of LOWER and UPPER
    // are assigned UPPER and LOWER do not change and hence the
    // loop fails to terminate.  With the loop counter we force
    // termination of the loop.
    //
    save.X = intrinsics::DMIN1(&[
        save.UPPER,
        intrinsics::DMAX1(&[save.LOWER, ((save.LOWER + save.UPPER) / 2.0)]),
    ]);
    save.FX2 = ((save.F * save.X) * save.X);

    STMP03(
        save.FX2,
        &mut save.C0,
        &mut save.C1,
        &mut save.C2,
        &mut save.C3,
        ctx,
    )?;
    save.LCOUNT = 0;
    save.MOSTC = 1000;

    while (((save.X > save.LOWER) && (save.X < save.UPPER)) && (save.LCOUNT < save.MOSTC)) {
        save.KFUN = (save.X
            * ((save.BR0 * save.C1)
                + (save.X * ((save.B2RV * save.C2) + ((save.X * save.BQ) * save.C3)))));

        if (save.KFUN > DT) {
            save.UPPER = save.X;
        } else if (save.KFUN < DT) {
            save.LOWER = save.X;
        } else {
            save.UPPER = save.X;
            save.LOWER = save.X;
        }
        //
        // As soon as the bracketing values move away from
        // zero we can modify the count limit.
        //
        if (save.MOSTC > MAXBIT) {
            if ((save.UPPER != 0.0) && (save.LOWER != 0.0)) {
                save.MOSTC = MAXBIT;
                save.LCOUNT = 0;
            }
        }

        save.X = intrinsics::DMIN1(&[
            save.UPPER,
            intrinsics::DMAX1(&[save.LOWER, ((save.LOWER + save.UPPER) / 2.0)]),
        ]);
        save.FX2 = ((save.F * save.X) * save.X);

        STMP03(
            save.FX2,
            &mut save.C0,
            &mut save.C1,
            &mut save.C2,
            &mut save.C3,
            ctx,
        )?;

        save.LCOUNT = (save.LCOUNT + 1);
    }

    //
    // With X in hand we simply compute BR, PC, VC, PCDOT and VCDOT
    // described in equations (1) --- (5) above. (Note, by our choice
    // of BOUND above, one can show that none of the computations
    // below can cause an overflow).
    //
    save.X2 = (save.X * save.X);
    save.X3 = (save.X2 * save.X);
    save.BR = ((save.BR0 * save.C0)
        + (save.X * ((save.B2RV * save.C1) + (save.X * (save.BQ * save.C2)))));

    save.PC = (1.0 - ((save.QOVR0 * save.X2) * save.C2));
    save.VC = (DT - ((save.BQ * save.X3) * save.C3));
    save.PCDOT = -(((save.QOVR0 / save.BR) * save.X) * save.C1);
    save.VCDOT = (1.0 - (((save.BQ / save.BR) * save.X2) * save.C2));

    //
    // ... and compute the linear combinations needed to get PVPROP
    //
    VLCOM(
        save.PC,
        save.POS.as_slice(),
        save.VC,
        save.VEL.as_slice(),
        PVPROP.subarray_mut(1),
    );
    VLCOM(
        save.PCDOT,
        save.POS.as_slice(),
        save.VCDOT,
        save.VEL.as_slice(),
        PVPROP.subarray_mut(4),
    );

    CHKOUT(b"PROP2B", ctx)?;
    Ok(())
}
