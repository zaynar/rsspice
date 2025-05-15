//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const REF: &[u8] = b"J2000";
const BASELT: f64 = 1.0;
const IDA: i32 = 1000;
const IDB: i32 = 2000;
const IDG: i32 = 1001;
const IDC: i32 = 10;
const N1: i32 = 600;
const N2: i32 = 4;
const N3: i32 = 8;
const N4: i32 = 86400;
const SIDLEN: i32 = 40;

struct SaveVars {
    XVEC: StackArray<f64, 3>,
    YVEC: StackArray<f64, 3>,
    ZVEC: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut XVEC = StackArray::<f64, 3>::new(1..=3);
        let mut YVEC = StackArray::<f64, 3>::new(1..=3);
        let mut ZVEC = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1.0), Val::D(0.0), Val::D(0.0)].into_iter();
            XVEC.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(1.0), Val::D(0.0)].into_iter();
            YVEC.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            ZVEC.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { XVEC, YVEC, ZVEC }
    }
}

//$Procedure ZZNATSPK ( Create an SPK file for Nat's solar system )
pub fn ZZNATSPK(FILE: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut SEGID = [b' '; SIDLEN as usize];
    let mut APROG: f64 = 0.0;
    let mut ARADA: f64 = 0.0;
    let mut ARADB: f64 = 0.0;
    let mut DELTAO: f64 = 0.0;
    let mut FIRST: f64 = 0.0;
    let mut LAST: f64 = 0.0;
    let mut MU: f64 = 0.0;
    let mut N323: f64 = 0.0;
    let mut OMEGAA: f64 = 0.0;
    let mut OMEGAB: f64 = 0.0;
    let mut OMEGAG: f64 = 0.0;
    let mut RALPHA: f64 = 0.0;
    let mut SUNST = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
    let mut RBETA: f64 = 0.0;
    let mut RGAMMA: f64 = 0.0;
    let mut SPEEDA: f64 = 0.0;
    let mut SPEEDB: f64 = 0.0;
    let mut SPEEDG: f64 = 0.0;
    let mut STATEA = StackArray::<f64, 6>::new(1..=6);
    let mut STATEB = StackArray::<f64, 6>::new(1..=6);
    let mut STATEG = StackArray::<f64, 6>::new(1..=6);
    let mut TAU: f64 = 0.0;
    let mut TSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut MYHAND: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // The current (2004/09/29) values of the parameters imply
    //
    //    Radius of ALPHA = 0.36624698766937712D+05 km
    //    Radius of BETA  = 0.22891526271046937D+04 km
    //

    //
    // Local variables
    //

    //
    // Declarations for RADA and RADB are needed if the lines of code
    // computing these items are un-commented.
    //
    //  DOUBLE PRECISION      RADA
    //  DOUBLE PRECISION      RADB
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    spicelib::CHKIN(b"ZZNATSPK", ctx)?;

    //
    // Wipe out any existing file with the target name.  Then open
    // a new SPC file for writing.
    //
    KILFIL(FILE, ctx)?;
    spicelib::SPCOPN(FILE, b"TestUtilitySPK", &mut MYHAND, ctx)?;

    //
    // Now just construct the state information needed to create
    // segments for objects ALPHA and BETA.
    //
    // Define the distances RBETA and RALPHA.  RBETA is the distance
    // light travels in
    //
    //
    //    BASELT * ( 1 - N3 ) / ( N3**(2/3) - N3 )
    //
    // seconds.
    //
    // RALPHA is
    //
    //      (2/3)
    //    N       * RBETA
    //     3
    //
    //
    // This power of
    //
    //    N
    //     3
    //
    // comes up a lot, so we allocate a variable for it.
    //
    //
    N323 = f64::powf((N3 as f64), (2.0 / 3.0));

    RBETA = (((spicelib::CLIGHT() * BASELT) * (1 - N3) as f64) / (N323 - N3 as f64));

    RALPHA = (N323 * RBETA);

    //
    // Set the angular rates of bodies alpha and beta.
    //
    OMEGAA = (spicelib::TWOPI(ctx) / (((N3 as f64) - 1.0) * N4 as f64));

    OMEGAB = ((N3 as f64) * OMEGAA);

    //
    // Get the differential angular velocity.
    //
    DELTAO = (OMEGAB - OMEGAA);

    //
    // Set the central GM value.
    //
    MU = (f64::powf(OMEGAA, 2.0) * f64::powf(RALPHA, 3.0));

    //
    // Set the angular radii of bodies ALPHA and BETA.  We want
    // the occultation to last N1 seconds.  If the angular
    // size of ALPHA is N2 times the angular size of BETA, then
    // BETA must make angular progress of (N2+1) * the angular
    // size of beta, relative to ALPHA, in N1 seconds.
    //
    APROG = ((N1 as f64) * DELTAO);

    ARADB = ((APROG / (N2 + 1) as f64) / 2 as f64);

    ARADA = ((N2 as f64) * ARADB);

    //
    // Set the radii of ALPHA and BETA.  These lines of code can be
    // used to reconstitute the radius values; however they're not
    // needed in this routine.
    //
    //  RADA  = RALPHA * SIN(ARADA)
    //  RADB  = RBETA  * SIN(ARADB)

    //
    // Set the initial positions.  We start by making both objects
    // line up on the +X axis at J2000 TDB.  Later, we'll rotate
    // body BETA to make an occultation start at this epoch.
    //
    spicelib::VSCL(RALPHA, save.XVEC.as_slice(), STATEA.as_slice_mut());
    spicelib::VSCL(RBETA, save.XVEC.as_slice(), STATEB.as_slice_mut());

    //
    // Set the initial velocities.  Both objects are traveling in the
    // +y direction at J2000 TDB.
    //
    SPEEDA = (RALPHA * OMEGAA);
    SPEEDB = (RBETA * OMEGAB);

    spicelib::VSCL(SPEEDA, save.YVEC.as_slice(), STATEA.subarray_mut(4));
    spicelib::VSCL(SPEEDB, save.YVEC.as_slice(), STATEB.subarray_mut(4));

    //
    // Now rotate the state of BETA by -(ARADA+ARADB) about +Z to make
    // objects ALPHA and BETA appear to be tangent at the current
    // epoch, as seen from the origin.
    //
    spicelib::VROTV(
        STATEB.as_slice(),
        save.ZVEC.as_slice(),
        -(ARADA + ARADB),
        TSTATE.as_slice_mut(),
    );
    spicelib::VROTV(
        STATEB.subarray(4),
        save.ZVEC.as_slice(),
        -(ARADA + ARADB),
        TSTATE.subarray_mut(4),
    );
    spicelib::MOVED(TSTATE.as_slice(), 6, STATEB.as_slice_mut());

    FIRST = -(100.0 * spicelib::JYEAR());
    LAST = (100.0 * spicelib::JYEAR());

    fstr::assign(&mut SEGID, b"Outer object alpha");

    spicelib::SPKW05(
        MYHAND,
        IDA,
        IDC,
        REF,
        FIRST,
        LAST,
        &SEGID,
        MU,
        1,
        STATEA.as_slice(),
        &[0.0],
        ctx,
    )?;

    fstr::assign(&mut SEGID, b"Inner object beta");

    spicelib::SPKW05(
        MYHAND,
        IDB,
        IDC,
        REF,
        FIRST,
        LAST,
        &SEGID,
        MU,
        1,
        STATEB.as_slice(),
        &[0.0],
        ctx,
    )?;

    //
    // Create a segment for GAMMA. This object orbits
    // ALPHA with a 24 hour period. The orbit is circular
    // and lies in the J2000 X-Y plane. The orbital radius
    // is 1.E5 km.
    //
    RGAMMA = 100000.0;
    TAU = spicelib::SPD();

    OMEGAG = (spicelib::TWOPI(ctx) / TAU);

    MU = (f64::powi(OMEGAG, 2) * f64::powi(RGAMMA, 3));

    SPEEDG = (RGAMMA * OMEGAG);

    spicelib::VSCL(-RGAMMA, save.YVEC.as_slice(), STATEG.as_slice_mut());
    spicelib::VSCL(SPEEDG, save.ZVEC.as_slice(), STATEG.subarray_mut(4));

    fstr::assign(&mut SEGID, b"Alpha\'s moon Gamma");

    spicelib::SPKW05(
        MYHAND,
        IDG,
        IDA,
        REF,
        FIRST,
        LAST,
        &SEGID,
        MU,
        1,
        STATEG.as_slice(),
        &[0.0],
        ctx,
    )?;

    //
    // Create a type 8 segment for the Sun.
    //
    fstr::assign(&mut SEGID, b"Motionless Sun at SSB");

    spicelib::CLEARD(12, SUNST.as_slice_mut());

    spicelib::SPKW08(
        MYHAND,
        10,
        0,
        b"J2000",
        FIRST,
        LAST,
        &SEGID,
        1,
        2,
        SUNST.as_slice(),
        FIRST,
        (LAST - FIRST),
        ctx,
    )?;

    //
    // Close the SPK file.
    //
    spicelib::DAFCLS(MYHAND, ctx)?;

    spicelib::CHKOUT(b"ZZNATSPK", ctx)?;
    Ok(())
}
