//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const NINERT: i32 = 21;
const NNINRT: i32 = 124;
const LBPOOL: i32 = -5;
const NON: i32 = NNINRT;
const NNAMES: i32 = (NINERT + NON);

//$Procedure ZZFDAT ( Initialize built-in frame names and ID codes )
pub fn ZZFDAT(
    NCOUNT: i32,
    MAXBFR: i32,
    NAME: CharArrayMut,
    IDCODE: &mut [i32],
    CENTER: &mut [i32],
    TYPE: &mut [i32],
    TYPID: &mut [i32],
    CENTRD: &mut [i32],
    BNMLST: &mut [i32],
    BNMPOL: &mut [i32],
    BNMNMS: CharArrayMut,
    BNMIDX: &mut [i32],
    BIDLST: &mut [i32],
    BIDPOL: &mut [i32],
    BIDIDS: &mut [i32],
    BIDIDX: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut NAME = DummyCharArrayMut::new(NAME, None, 1..);
    let mut IDCODE = DummyArrayMut::new(IDCODE, 1..);
    let mut CENTER = DummyArrayMut::new(CENTER, 1..);
    let mut TYPE = DummyArrayMut::new(TYPE, 1..);
    let mut TYPID = DummyArrayMut::new(TYPID, 1..);
    let mut CENTRD = DummyArrayMut::new(CENTRD, 1..);
    let mut BNMLST = DummyArrayMut::new(BNMLST, 1..);
    let mut BNMPOL = DummyArrayMut::new(BNMPOL, LBPOOL..);
    let mut BNMNMS = DummyCharArrayMut::new(BNMNMS, None, 1..);
    let mut BNMIDX = DummyArrayMut::new(BNMIDX, 1..);
    let mut BIDLST = DummyArrayMut::new(BIDLST, 1..);
    let mut BIDPOL = DummyArrayMut::new(BIDPOL, LBPOOL..);
    let mut BIDIDS = DummyArrayMut::new(BIDIDS, 1..);
    let mut BIDIDX = DummyArrayMut::new(BIDIDX, 1..);
    let mut ITEM: i32 = 0;
    let mut NEW: bool = false;

    //
    // To add to the list of recognized frames,
    //
    // 1. Determine whether or not the frame is inertial.
    //
    //    Inertial Case.
    //
    //    A. Be sure that the routine CHGIRF has been modified to
    //       reflect the new frame and set NINERT (above) equal to
    //       the number of recognized inertial frames give by CHGIRF.
    //
    //    Non Inertial Case.
    //
    //    A. Locate the last non-inertial frame in the lengthy list
    //       below.
    //
    //    B. Add the frame name to the array NAME.  Add the IDCODE
    //       to the array IDCODE. (Unless there is a compelling reason
    //       to do otherwise this should just be the next integer in
    //       the sequence of ID codes.  The mixture of old and new code
    //       should look something like this:
    //
    //          Last bit of old assignments
    //
    //             NAME   ( NINERT + NON ) = last name in the old routine
    //             IDCODE ( NINERT + NON ) = 10000 + NON
    //
    //          Your new assignment
    //
    //             NAME   ( NINERT + NEXT ) = your name
    //             IDCODE ( NINERT + NEXT ) = 10000 + NEXT
    //
    //       where
    //
    //          NON  = the value of the parameter above
    //          NEXT = NON + 1
    //
    //    C. Modify the value of the parameter NON above to reflect the
    //       new number of non-inertial frames.
    //
    // 2. Update the version and date routine.
    //
    // 3. Update the routines that call this routine so that they
    //    will be expecting the correct number of names and ID codes
    //    to be returned.
    //

    //
    // Local variables.
    //

    //
    // Perform the consistency checks first.
    //
    if (NCOUNT != NNAMES) {
        CHKIN(b"ZZFDAT", ctx)?;
        SETMSG(b"There is an inconsistency between the version of the routine calling ZZFDAT and the current version of ZZFDAT. Check to make sure that you have the most current versions of ZZFDAT and the routines that make use of it.", ctx);
        SIGERR(b"SPICE(VERSIONMISMATCH1)", ctx)?;
        CHKOUT(b"ZZFDAT", ctx)?;
        return Ok(());
    }

    if (MAXBFR < NNAMES) {
        CHKIN(b"ZZFDAT", ctx)?;
        SETMSG(b"There is an inconsistency between the version of the routine calling ZZFDAT and the current version of ZZFDAT. Check to make sure that you have the most current versions of ZZFDAT and the routines that make use of it.", ctx);
        SIGERR(b"SPICE(VERSIONMISMATCH2)", ctx)?;
        CHKOUT(b"ZZFDAT", ctx)?;
        return Ok(());
    }

    //
    // Inertial Frames Section
    //
    // Fetch the names of the inertial frames from CHGIRF.
    //
    for I in 1..=NINERT {
        IDCODE[I] = I;
        CENTER[I] = 0;
        TYPE[I] = INERTL;
        TYPID[I] = I;
        IRFNAM(I, &mut NAME[I], ctx)?;
    }

    //
    // Non-Inertial Frames Section.
    //
    fstr::assign(NAME.get_mut((NINERT + 1)), b"IAU_MERCURY_BARYCENTER");
    IDCODE[(NINERT + 1)] = 10001;
    CENTER[(NINERT + 1)] = 1;
    TYPID[(NINERT + 1)] = 1;
    TYPE[(NINERT + 1)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 2)), b"IAU_VENUS_BARYCENTER");
    IDCODE[(NINERT + 2)] = 10002;
    CENTER[(NINERT + 2)] = 2;
    TYPID[(NINERT + 2)] = 2;
    TYPE[(NINERT + 2)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 3)), b"IAU_EARTH_BARYCENTER");
    IDCODE[(NINERT + 3)] = 10003;
    CENTER[(NINERT + 3)] = 3;
    TYPID[(NINERT + 3)] = 3;
    TYPE[(NINERT + 3)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 4)), b"IAU_MARS_BARYCENTER");
    IDCODE[(NINERT + 4)] = 10004;
    CENTER[(NINERT + 4)] = 4;
    TYPID[(NINERT + 4)] = 4;
    TYPE[(NINERT + 4)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 5)), b"IAU_JUPITER_BARYCENTER");
    IDCODE[(NINERT + 5)] = 10005;
    CENTER[(NINERT + 5)] = 5;
    TYPID[(NINERT + 5)] = 5;
    TYPE[(NINERT + 5)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 6)), b"IAU_SATURN_BARYCENTER");
    IDCODE[(NINERT + 6)] = 10006;
    CENTER[(NINERT + 6)] = 6;
    TYPID[(NINERT + 6)] = 6;
    TYPE[(NINERT + 6)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 7)), b"IAU_URANUS_BARYCENTER");
    IDCODE[(NINERT + 7)] = 10007;
    CENTER[(NINERT + 7)] = 7;
    TYPID[(NINERT + 7)] = 7;
    TYPE[(NINERT + 7)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 8)), b"IAU_NEPTUNE_BARYCENTER");
    IDCODE[(NINERT + 8)] = 10008;
    CENTER[(NINERT + 8)] = 8;
    TYPID[(NINERT + 8)] = 8;
    TYPE[(NINERT + 8)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 9)), b"IAU_PLUTO_BARYCENTER");
    IDCODE[(NINERT + 9)] = 10009;
    CENTER[(NINERT + 9)] = 9;
    TYPID[(NINERT + 9)] = 9;
    TYPE[(NINERT + 9)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 10)), b"IAU_SUN");
    IDCODE[(NINERT + 10)] = 10010;
    CENTER[(NINERT + 10)] = 10;
    TYPID[(NINERT + 10)] = 10;
    TYPE[(NINERT + 10)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 11)), b"IAU_MERCURY");
    IDCODE[(NINERT + 11)] = 10011;
    CENTER[(NINERT + 11)] = 199;
    TYPID[(NINERT + 11)] = 199;
    TYPE[(NINERT + 11)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 12)), b"IAU_VENUS");
    IDCODE[(NINERT + 12)] = 10012;
    CENTER[(NINERT + 12)] = 299;
    TYPID[(NINERT + 12)] = 299;
    TYPE[(NINERT + 12)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 13)), b"IAU_EARTH");
    IDCODE[(NINERT + 13)] = 10013;
    CENTER[(NINERT + 13)] = 399;
    TYPID[(NINERT + 13)] = 399;
    TYPE[(NINERT + 13)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 14)), b"IAU_MARS");
    IDCODE[(NINERT + 14)] = 10014;
    CENTER[(NINERT + 14)] = 499;
    TYPID[(NINERT + 14)] = 499;
    TYPE[(NINERT + 14)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 15)), b"IAU_JUPITER");
    IDCODE[(NINERT + 15)] = 10015;
    CENTER[(NINERT + 15)] = 599;
    TYPID[(NINERT + 15)] = 599;
    TYPE[(NINERT + 15)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 16)), b"IAU_SATURN");
    IDCODE[(NINERT + 16)] = 10016;
    CENTER[(NINERT + 16)] = 699;
    TYPID[(NINERT + 16)] = 699;
    TYPE[(NINERT + 16)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 17)), b"IAU_URANUS");
    IDCODE[(NINERT + 17)] = 10017;
    CENTER[(NINERT + 17)] = 799;
    TYPID[(NINERT + 17)] = 799;
    TYPE[(NINERT + 17)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 18)), b"IAU_NEPTUNE");
    IDCODE[(NINERT + 18)] = 10018;
    CENTER[(NINERT + 18)] = 899;
    TYPID[(NINERT + 18)] = 899;
    TYPE[(NINERT + 18)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 19)), b"IAU_PLUTO");
    IDCODE[(NINERT + 19)] = 10019;
    CENTER[(NINERT + 19)] = 999;
    TYPID[(NINERT + 19)] = 999;
    TYPE[(NINERT + 19)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 20)), b"IAU_MOON");
    IDCODE[(NINERT + 20)] = 10020;
    CENTER[(NINERT + 20)] = 301;
    TYPID[(NINERT + 20)] = 301;
    TYPE[(NINERT + 20)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 21)), b"IAU_PHOBOS");
    IDCODE[(NINERT + 21)] = 10021;
    CENTER[(NINERT + 21)] = 401;
    TYPID[(NINERT + 21)] = 401;
    TYPE[(NINERT + 21)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 22)), b"IAU_DEIMOS");
    IDCODE[(NINERT + 22)] = 10022;
    CENTER[(NINERT + 22)] = 402;
    TYPID[(NINERT + 22)] = 402;
    TYPE[(NINERT + 22)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 23)), b"IAU_IO");
    IDCODE[(NINERT + 23)] = 10023;
    CENTER[(NINERT + 23)] = 501;
    TYPID[(NINERT + 23)] = 501;
    TYPE[(NINERT + 23)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 24)), b"IAU_EUROPA");
    IDCODE[(NINERT + 24)] = 10024;
    CENTER[(NINERT + 24)] = 502;
    TYPID[(NINERT + 24)] = 502;
    TYPE[(NINERT + 24)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 25)), b"IAU_GANYMEDE");
    IDCODE[(NINERT + 25)] = 10025;
    CENTER[(NINERT + 25)] = 503;
    TYPID[(NINERT + 25)] = 503;
    TYPE[(NINERT + 25)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 26)), b"IAU_CALLISTO");
    IDCODE[(NINERT + 26)] = 10026;
    CENTER[(NINERT + 26)] = 504;
    TYPID[(NINERT + 26)] = 504;
    TYPE[(NINERT + 26)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 27)), b"IAU_AMALTHEA");
    IDCODE[(NINERT + 27)] = 10027;
    CENTER[(NINERT + 27)] = 505;
    TYPID[(NINERT + 27)] = 505;
    TYPE[(NINERT + 27)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 28)), b"IAU_HIMALIA");
    IDCODE[(NINERT + 28)] = 10028;
    CENTER[(NINERT + 28)] = 506;
    TYPID[(NINERT + 28)] = 506;
    TYPE[(NINERT + 28)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 29)), b"IAU_ELARA");
    IDCODE[(NINERT + 29)] = 10029;
    CENTER[(NINERT + 29)] = 507;
    TYPID[(NINERT + 29)] = 507;
    TYPE[(NINERT + 29)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 30)), b"IAU_PASIPHAE");
    IDCODE[(NINERT + 30)] = 10030;
    CENTER[(NINERT + 30)] = 508;
    TYPID[(NINERT + 30)] = 508;
    TYPE[(NINERT + 30)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 31)), b"IAU_SINOPE");
    IDCODE[(NINERT + 31)] = 10031;
    CENTER[(NINERT + 31)] = 509;
    TYPID[(NINERT + 31)] = 509;
    TYPE[(NINERT + 31)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 32)), b"IAU_LYSITHEA");
    IDCODE[(NINERT + 32)] = 10032;
    CENTER[(NINERT + 32)] = 510;
    TYPID[(NINERT + 32)] = 510;
    TYPE[(NINERT + 32)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 33)), b"IAU_CARME");
    IDCODE[(NINERT + 33)] = 10033;
    CENTER[(NINERT + 33)] = 511;
    TYPID[(NINERT + 33)] = 511;
    TYPE[(NINERT + 33)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 34)), b"IAU_ANANKE");
    IDCODE[(NINERT + 34)] = 10034;
    CENTER[(NINERT + 34)] = 512;
    TYPID[(NINERT + 34)] = 512;
    TYPE[(NINERT + 34)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 35)), b"IAU_LEDA");
    IDCODE[(NINERT + 35)] = 10035;
    CENTER[(NINERT + 35)] = 513;
    TYPID[(NINERT + 35)] = 513;
    TYPE[(NINERT + 35)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 36)), b"IAU_THEBE");
    IDCODE[(NINERT + 36)] = 10036;
    CENTER[(NINERT + 36)] = 514;
    TYPID[(NINERT + 36)] = 514;
    TYPE[(NINERT + 36)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 37)), b"IAU_ADRASTEA");
    IDCODE[(NINERT + 37)] = 10037;
    CENTER[(NINERT + 37)] = 515;
    TYPID[(NINERT + 37)] = 515;
    TYPE[(NINERT + 37)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 38)), b"IAU_METIS");
    IDCODE[(NINERT + 38)] = 10038;
    CENTER[(NINERT + 38)] = 516;
    TYPID[(NINERT + 38)] = 516;
    TYPE[(NINERT + 38)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 39)), b"IAU_MIMAS");
    IDCODE[(NINERT + 39)] = 10039;
    CENTER[(NINERT + 39)] = 601;
    TYPID[(NINERT + 39)] = 601;
    TYPE[(NINERT + 39)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 40)), b"IAU_ENCELADUS");
    IDCODE[(NINERT + 40)] = 10040;
    CENTER[(NINERT + 40)] = 602;
    TYPID[(NINERT + 40)] = 602;
    TYPE[(NINERT + 40)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 41)), b"IAU_TETHYS");
    IDCODE[(NINERT + 41)] = 10041;
    CENTER[(NINERT + 41)] = 603;
    TYPID[(NINERT + 41)] = 603;
    TYPE[(NINERT + 41)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 42)), b"IAU_DIONE");
    IDCODE[(NINERT + 42)] = 10042;
    CENTER[(NINERT + 42)] = 604;
    TYPID[(NINERT + 42)] = 604;
    TYPE[(NINERT + 42)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 43)), b"IAU_RHEA");
    IDCODE[(NINERT + 43)] = 10043;
    CENTER[(NINERT + 43)] = 605;
    TYPID[(NINERT + 43)] = 605;
    TYPE[(NINERT + 43)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 44)), b"IAU_TITAN");
    IDCODE[(NINERT + 44)] = 10044;
    CENTER[(NINERT + 44)] = 606;
    TYPID[(NINERT + 44)] = 606;
    TYPE[(NINERT + 44)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 45)), b"IAU_HYPERION");
    IDCODE[(NINERT + 45)] = 10045;
    CENTER[(NINERT + 45)] = 607;
    TYPID[(NINERT + 45)] = 607;
    TYPE[(NINERT + 45)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 46)), b"IAU_IAPETUS");
    IDCODE[(NINERT + 46)] = 10046;
    CENTER[(NINERT + 46)] = 608;
    TYPID[(NINERT + 46)] = 608;
    TYPE[(NINERT + 46)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 47)), b"IAU_PHOEBE");
    IDCODE[(NINERT + 47)] = 10047;
    CENTER[(NINERT + 47)] = 609;
    TYPID[(NINERT + 47)] = 609;
    TYPE[(NINERT + 47)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 48)), b"IAU_JANUS");
    IDCODE[(NINERT + 48)] = 10048;
    CENTER[(NINERT + 48)] = 610;
    TYPID[(NINERT + 48)] = 610;
    TYPE[(NINERT + 48)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 49)), b"IAU_EPIMETHEUS");
    IDCODE[(NINERT + 49)] = 10049;
    CENTER[(NINERT + 49)] = 611;
    TYPID[(NINERT + 49)] = 611;
    TYPE[(NINERT + 49)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 50)), b"IAU_HELENE");
    IDCODE[(NINERT + 50)] = 10050;
    CENTER[(NINERT + 50)] = 612;
    TYPID[(NINERT + 50)] = 612;
    TYPE[(NINERT + 50)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 51)), b"IAU_TELESTO");
    IDCODE[(NINERT + 51)] = 10051;
    CENTER[(NINERT + 51)] = 613;
    TYPID[(NINERT + 51)] = 613;
    TYPE[(NINERT + 51)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 52)), b"IAU_CALYPSO");
    IDCODE[(NINERT + 52)] = 10052;
    CENTER[(NINERT + 52)] = 614;
    TYPID[(NINERT + 52)] = 614;
    TYPE[(NINERT + 52)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 53)), b"IAU_ATLAS");
    IDCODE[(NINERT + 53)] = 10053;
    CENTER[(NINERT + 53)] = 615;
    TYPID[(NINERT + 53)] = 615;
    TYPE[(NINERT + 53)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 54)), b"IAU_PROMETHEUS");
    IDCODE[(NINERT + 54)] = 10054;
    CENTER[(NINERT + 54)] = 616;
    TYPID[(NINERT + 54)] = 616;
    TYPE[(NINERT + 54)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 55)), b"IAU_PANDORA");
    IDCODE[(NINERT + 55)] = 10055;
    CENTER[(NINERT + 55)] = 617;
    TYPID[(NINERT + 55)] = 617;
    TYPE[(NINERT + 55)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 56)), b"IAU_ARIEL");
    IDCODE[(NINERT + 56)] = 10056;
    CENTER[(NINERT + 56)] = 701;
    TYPID[(NINERT + 56)] = 701;
    TYPE[(NINERT + 56)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 57)), b"IAU_UMBRIEL");
    IDCODE[(NINERT + 57)] = 10057;
    CENTER[(NINERT + 57)] = 702;
    TYPID[(NINERT + 57)] = 702;
    TYPE[(NINERT + 57)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 58)), b"IAU_TITANIA");
    IDCODE[(NINERT + 58)] = 10058;
    CENTER[(NINERT + 58)] = 703;
    TYPID[(NINERT + 58)] = 703;
    TYPE[(NINERT + 58)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 59)), b"IAU_OBERON");
    IDCODE[(NINERT + 59)] = 10059;
    CENTER[(NINERT + 59)] = 704;
    TYPID[(NINERT + 59)] = 704;
    TYPE[(NINERT + 59)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 60)), b"IAU_MIRANDA");
    IDCODE[(NINERT + 60)] = 10060;
    CENTER[(NINERT + 60)] = 705;
    TYPID[(NINERT + 60)] = 705;
    TYPE[(NINERT + 60)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 61)), b"IAU_CORDELIA");
    IDCODE[(NINERT + 61)] = 10061;
    CENTER[(NINERT + 61)] = 706;
    TYPID[(NINERT + 61)] = 706;
    TYPE[(NINERT + 61)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 62)), b"IAU_OPHELIA");
    IDCODE[(NINERT + 62)] = 10062;
    CENTER[(NINERT + 62)] = 707;
    TYPID[(NINERT + 62)] = 707;
    TYPE[(NINERT + 62)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 63)), b"IAU_BIANCA");
    IDCODE[(NINERT + 63)] = 10063;
    CENTER[(NINERT + 63)] = 708;
    TYPID[(NINERT + 63)] = 708;
    TYPE[(NINERT + 63)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 64)), b"IAU_CRESSIDA");
    IDCODE[(NINERT + 64)] = 10064;
    CENTER[(NINERT + 64)] = 709;
    TYPID[(NINERT + 64)] = 709;
    TYPE[(NINERT + 64)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 65)), b"IAU_DESDEMONA");
    IDCODE[(NINERT + 65)] = 10065;
    CENTER[(NINERT + 65)] = 710;
    TYPID[(NINERT + 65)] = 710;
    TYPE[(NINERT + 65)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 66)), b"IAU_JULIET");
    IDCODE[(NINERT + 66)] = 10066;
    CENTER[(NINERT + 66)] = 711;
    TYPID[(NINERT + 66)] = 711;
    TYPE[(NINERT + 66)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 67)), b"IAU_PORTIA");
    IDCODE[(NINERT + 67)] = 10067;
    CENTER[(NINERT + 67)] = 712;
    TYPID[(NINERT + 67)] = 712;
    TYPE[(NINERT + 67)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 68)), b"IAU_ROSALIND");
    IDCODE[(NINERT + 68)] = 10068;
    CENTER[(NINERT + 68)] = 713;
    TYPID[(NINERT + 68)] = 713;
    TYPE[(NINERT + 68)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 69)), b"IAU_BELINDA");
    IDCODE[(NINERT + 69)] = 10069;
    CENTER[(NINERT + 69)] = 714;
    TYPID[(NINERT + 69)] = 714;
    TYPE[(NINERT + 69)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 70)), b"IAU_PUCK");
    IDCODE[(NINERT + 70)] = 10070;
    CENTER[(NINERT + 70)] = 715;
    TYPID[(NINERT + 70)] = 715;
    TYPE[(NINERT + 70)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 71)), b"IAU_TRITON");
    IDCODE[(NINERT + 71)] = 10071;
    CENTER[(NINERT + 71)] = 801;
    TYPID[(NINERT + 71)] = 801;
    TYPE[(NINERT + 71)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 72)), b"IAU_NEREID");
    IDCODE[(NINERT + 72)] = 10072;
    CENTER[(NINERT + 72)] = 802;
    TYPID[(NINERT + 72)] = 802;
    TYPE[(NINERT + 72)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 73)), b"IAU_NAIAD");
    IDCODE[(NINERT + 73)] = 10073;
    CENTER[(NINERT + 73)] = 803;
    TYPID[(NINERT + 73)] = 803;
    TYPE[(NINERT + 73)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 74)), b"IAU_THALASSA");
    IDCODE[(NINERT + 74)] = 10074;
    CENTER[(NINERT + 74)] = 804;
    TYPID[(NINERT + 74)] = 804;
    TYPE[(NINERT + 74)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 75)), b"IAU_DESPINA");
    IDCODE[(NINERT + 75)] = 10075;
    CENTER[(NINERT + 75)] = 805;
    TYPID[(NINERT + 75)] = 805;
    TYPE[(NINERT + 75)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 76)), b"IAU_GALATEA");
    IDCODE[(NINERT + 76)] = 10076;
    CENTER[(NINERT + 76)] = 806;
    TYPID[(NINERT + 76)] = 806;
    TYPE[(NINERT + 76)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 77)), b"IAU_LARISSA");
    IDCODE[(NINERT + 77)] = 10077;
    CENTER[(NINERT + 77)] = 807;
    TYPID[(NINERT + 77)] = 807;
    TYPE[(NINERT + 77)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 78)), b"IAU_PROTEUS");
    IDCODE[(NINERT + 78)] = 10078;
    CENTER[(NINERT + 78)] = 808;
    TYPID[(NINERT + 78)] = 808;
    TYPE[(NINERT + 78)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 79)), b"IAU_CHARON");
    IDCODE[(NINERT + 79)] = 10079;
    CENTER[(NINERT + 79)] = 901;
    TYPID[(NINERT + 79)] = 901;
    TYPE[(NINERT + 79)] = PCK;

    //
    // This is for the first new PCK frame---the high precision earth
    // frame ITRF93.
    //
    fstr::assign(NAME.get_mut((NINERT + 80)), b"ITRF93");
    IDCODE[(NINERT + 80)] = 13000;
    CENTER[(NINERT + 80)] = 399;
    TYPID[(NINERT + 80)] = 3000;
    TYPE[(NINERT + 80)] = PCK;

    //
    // This if for the alias frame EARTH BODYFIXED.  This is a TK
    // class frame.  To use it a FRAME kernel must be loaded via
    // FURNSH.
    //
    fstr::assign(NAME.get_mut((NINERT + 81)), b"EARTH_FIXED");
    IDCODE[(NINERT + 81)] = 10081;
    CENTER[(NINERT + 81)] = 399;
    TYPID[(NINERT + 81)] = 10081;
    TYPE[(NINERT + 81)] = TK;

    //
    // Frames introduced into the generic NAIF PCK
    // system as referenced from the 1997 IAU report.
    //
    fstr::assign(NAME.get_mut((NINERT + 82)), b"IAU_PAN");
    IDCODE[(NINERT + 82)] = 10082;
    CENTER[(NINERT + 82)] = 618;
    TYPID[(NINERT + 82)] = 618;
    TYPE[(NINERT + 82)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 83)), b"IAU_GASPRA");
    IDCODE[(NINERT + 83)] = 10083;
    CENTER[(NINERT + 83)] = 9511010;
    TYPID[(NINERT + 83)] = 9511010;
    TYPE[(NINERT + 83)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 84)), b"IAU_IDA");
    IDCODE[(NINERT + 84)] = 10084;
    CENTER[(NINERT + 84)] = 2431010;
    TYPID[(NINERT + 84)] = 2431010;
    TYPE[(NINERT + 84)] = PCK;

    //
    // Frame referenced from the Eros orientation model in the 2000 IAU
    // report.
    //
    fstr::assign(NAME.get_mut((NINERT + 85)), b"IAU_EROS");
    IDCODE[(NINERT + 85)] = 10085;
    CENTER[(NINERT + 85)] = 2000433;
    TYPID[(NINERT + 85)] = 2000433;
    TYPE[(NINERT + 85)] = PCK;

    //
    // Frames for Jovian satellites approved by IAU in late 2002.
    //
    fstr::assign(NAME.get_mut((NINERT + 86)), b"IAU_CALLIRRHOE");
    IDCODE[(NINERT + 86)] = 10086;
    CENTER[(NINERT + 86)] = 517;
    TYPID[(NINERT + 86)] = 517;
    TYPE[(NINERT + 86)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 87)), b"IAU_THEMISTO");
    IDCODE[(NINERT + 87)] = 10087;
    CENTER[(NINERT + 87)] = 518;
    TYPID[(NINERT + 87)] = 518;
    TYPE[(NINERT + 87)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 88)), b"IAU_MEGACLITE");
    IDCODE[(NINERT + 88)] = 10088;
    CENTER[(NINERT + 88)] = 519;
    TYPID[(NINERT + 88)] = 519;
    TYPE[(NINERT + 88)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 89)), b"IAU_TAYGETE");
    IDCODE[(NINERT + 89)] = 10089;
    CENTER[(NINERT + 89)] = 520;
    TYPID[(NINERT + 89)] = 520;
    TYPE[(NINERT + 89)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 90)), b"IAU_CHALDENE");
    IDCODE[(NINERT + 90)] = 10090;
    CENTER[(NINERT + 90)] = 521;
    TYPID[(NINERT + 90)] = 521;
    TYPE[(NINERT + 90)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 91)), b"IAU_HARPALYKE");
    IDCODE[(NINERT + 91)] = 10091;
    CENTER[(NINERT + 91)] = 522;
    TYPID[(NINERT + 91)] = 522;
    TYPE[(NINERT + 91)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 92)), b"IAU_KALYKE");
    IDCODE[(NINERT + 92)] = 10092;
    CENTER[(NINERT + 92)] = 523;
    TYPID[(NINERT + 92)] = 523;
    TYPE[(NINERT + 92)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 93)), b"IAU_IOCASTE");
    IDCODE[(NINERT + 93)] = 10093;
    CENTER[(NINERT + 93)] = 524;
    TYPID[(NINERT + 93)] = 524;
    TYPE[(NINERT + 93)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 94)), b"IAU_ERINOME");
    IDCODE[(NINERT + 94)] = 10094;
    CENTER[(NINERT + 94)] = 525;
    TYPID[(NINERT + 94)] = 525;
    TYPE[(NINERT + 94)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 95)), b"IAU_ISONOE");
    IDCODE[(NINERT + 95)] = 10095;
    CENTER[(NINERT + 95)] = 526;
    TYPID[(NINERT + 95)] = 526;
    TYPE[(NINERT + 95)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 96)), b"IAU_PRAXIDIKE");
    IDCODE[(NINERT + 96)] = 10096;
    CENTER[(NINERT + 96)] = 527;
    TYPID[(NINERT + 96)] = 527;
    TYPE[(NINERT + 96)] = PCK;

    //
    // Frames for comets and asteroids, for which rotation constants
    // were added in 2006 IAU Report.
    //
    fstr::assign(NAME.get_mut((NINERT + 97)), b"IAU_BORRELLY");
    IDCODE[(NINERT + 97)] = 10097;
    CENTER[(NINERT + 97)] = 1000005;
    TYPID[(NINERT + 97)] = 1000005;
    TYPE[(NINERT + 97)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 98)), b"IAU_TEMPEL_1");
    IDCODE[(NINERT + 98)] = 10098;
    CENTER[(NINERT + 98)] = 1000093;
    TYPID[(NINERT + 98)] = 1000093;
    TYPE[(NINERT + 98)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 99)), b"IAU_VESTA");
    IDCODE[(NINERT + 99)] = 10099;
    CENTER[(NINERT + 99)] = 2000004;
    TYPID[(NINERT + 99)] = 2000004;
    TYPE[(NINERT + 99)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 100)), b"IAU_ITOKAWA");
    IDCODE[(NINERT + 100)] = 10100;
    CENTER[(NINERT + 100)] = 2025143;
    TYPID[(NINERT + 100)] = 2025143;
    TYPE[(NINERT + 100)] = PCK;

    //
    // Frames for asteroids, for which rotation constants were added in
    // 2009 IAU Report.
    //
    fstr::assign(NAME.get_mut((NINERT + 101)), b"IAU_CERES");
    IDCODE[(NINERT + 101)] = 10101;
    CENTER[(NINERT + 101)] = 2000001;
    TYPID[(NINERT + 101)] = 2000001;
    TYPE[(NINERT + 101)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 102)), b"IAU_PALLAS");
    IDCODE[(NINERT + 102)] = 10102;
    CENTER[(NINERT + 102)] = 2000002;
    TYPID[(NINERT + 102)] = 2000002;
    TYPE[(NINERT + 102)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 103)), b"IAU_LUTETIA");
    IDCODE[(NINERT + 103)] = 10103;
    CENTER[(NINERT + 103)] = 2000021;
    TYPID[(NINERT + 103)] = 2000021;
    TYPE[(NINERT + 103)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 104)), b"IAU_DAVIDA");
    IDCODE[(NINERT + 104)] = 10104;
    CENTER[(NINERT + 104)] = 2000511;
    TYPID[(NINERT + 104)] = 2000511;
    TYPE[(NINERT + 104)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 105)), b"IAU_STEINS");
    IDCODE[(NINERT + 105)] = 10105;
    CENTER[(NINERT + 105)] = 2002867;
    TYPID[(NINERT + 105)] = 2002867;
    TYPE[(NINERT + 105)] = PCK;

    //
    // Frame for Bennu asteroid, for OSIRIS-Rex project.
    //
    fstr::assign(NAME.get_mut((NINERT + 106)), b"IAU_BENNU");
    IDCODE[(NINERT + 106)] = 10106;
    CENTER[(NINERT + 106)] = 2101955;
    TYPID[(NINERT + 106)] = 2101955;
    TYPE[(NINERT + 106)] = PCK;

    //
    // Frames for asteroid 52 Europa, Nix, Hydra, and Hayabusa 2,
    // New Horizons, DART, and Lucy mission targets.
    //
    fstr::assign(NAME.get_mut((NINERT + 107)), b"IAU_52_EUROPA");
    IDCODE[(NINERT + 107)] = 10107;
    CENTER[(NINERT + 107)] = 2000052;
    TYPID[(NINERT + 107)] = 2000052;
    TYPE[(NINERT + 107)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 108)), b"IAU_NIX");
    IDCODE[(NINERT + 108)] = 10108;
    CENTER[(NINERT + 108)] = 902;
    TYPID[(NINERT + 108)] = 902;
    TYPE[(NINERT + 108)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 109)), b"IAU_HYDRA");
    IDCODE[(NINERT + 109)] = 10109;
    CENTER[(NINERT + 109)] = 903;
    TYPID[(NINERT + 109)] = 903;
    TYPE[(NINERT + 109)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 110)), b"IAU_RYUGU");
    IDCODE[(NINERT + 110)] = 10110;
    CENTER[(NINERT + 110)] = 2162173;
    TYPID[(NINERT + 110)] = 2162173;
    TYPE[(NINERT + 110)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 111)), b"IAU_ARROKOTH");
    IDCODE[(NINERT + 111)] = 10111;
    CENTER[(NINERT + 111)] = 2486958;
    TYPID[(NINERT + 111)] = 2486958;
    TYPE[(NINERT + 111)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 112)), b"IAU_DIDYMOS_BARYCENTER");
    IDCODE[(NINERT + 112)] = 10112;
    CENTER[(NINERT + 112)] = 20065803;
    TYPID[(NINERT + 112)] = 20065803;
    TYPE[(NINERT + 112)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 113)), b"IAU_DIDYMOS");
    IDCODE[(NINERT + 113)] = 10113;
    CENTER[(NINERT + 113)] = 920065803;
    TYPID[(NINERT + 113)] = 920065803;
    TYPE[(NINERT + 113)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 114)), b"IAU_DIMORPHOS");
    IDCODE[(NINERT + 114)] = 10114;
    CENTER[(NINERT + 114)] = 120065803;
    TYPID[(NINERT + 114)] = 120065803;
    TYPE[(NINERT + 114)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 115)), b"IAU_DONALDJOHANSON");
    IDCODE[(NINERT + 115)] = 10115;
    CENTER[(NINERT + 115)] = 20052246;
    TYPID[(NINERT + 115)] = 20052246;
    TYPE[(NINERT + 115)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 116)), b"IAU_EURYBATES");
    IDCODE[(NINERT + 116)] = 10116;
    CENTER[(NINERT + 116)] = 920003548;
    TYPID[(NINERT + 116)] = 920003548;
    TYPE[(NINERT + 116)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 117)), b"IAU_EURYBATES_BARYCENTER");
    IDCODE[(NINERT + 117)] = 10117;
    CENTER[(NINERT + 117)] = 20003548;
    TYPID[(NINERT + 117)] = 20003548;
    TYPE[(NINERT + 117)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 118)), b"IAU_QUETA");
    IDCODE[(NINERT + 118)] = 10118;
    CENTER[(NINERT + 118)] = 120003548;
    TYPID[(NINERT + 118)] = 120003548;
    TYPE[(NINERT + 118)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 119)), b"IAU_POLYMELE");
    IDCODE[(NINERT + 119)] = 10119;
    CENTER[(NINERT + 119)] = 20015094;
    TYPID[(NINERT + 119)] = 20015094;
    TYPE[(NINERT + 119)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 120)), b"IAU_LEUCUS");
    IDCODE[(NINERT + 120)] = 10120;
    CENTER[(NINERT + 120)] = 20011351;
    TYPID[(NINERT + 120)] = 20011351;
    TYPE[(NINERT + 120)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 121)), b"IAU_ORUS");
    IDCODE[(NINERT + 121)] = 10121;
    CENTER[(NINERT + 121)] = 20021900;
    TYPID[(NINERT + 121)] = 20021900;
    TYPE[(NINERT + 121)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 122)), b"IAU_PATROCLUS_BARYCENTER");
    IDCODE[(NINERT + 122)] = 10122;
    CENTER[(NINERT + 122)] = 20000617;
    TYPID[(NINERT + 122)] = 20000617;
    TYPE[(NINERT + 122)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 123)), b"IAU_PATROCLUS");
    IDCODE[(NINERT + 123)] = 10123;
    CENTER[(NINERT + 123)] = 920000617;
    TYPID[(NINERT + 123)] = 920000617;
    TYPE[(NINERT + 123)] = PCK;

    fstr::assign(NAME.get_mut((NINERT + 124)), b"IAU_MENOETIUS");
    IDCODE[(NINERT + 124)] = 10124;
    CENTER[(NINERT + 124)] = 120000617;
    TYPID[(NINERT + 124)] = 120000617;
    TYPE[(NINERT + 124)] = PCK;

    //
    // Below is a template to use for adding another non-inertial
    // frame. Copy it, fill in the new values and then leave
    // a new template for the next person who needs to modify this
    // routine.
    //
    // NAME   ( NINERT + 125 ) =  name
    // IDCODE ( NINERT + 125 ) =  10125
    // CENTER ( NINERT + 125 ) =  center
    // TYPID  ( NINERT + 125 ) =  type ID code
    // TYPE   ( NINERT + 125 ) =  type (INERTL, PCK, etc. )
    //

    //
    // Generate order vector for centers (used by CIDFRM).
    //
    ORDERI(CENTER.as_slice(), NNAMES, CENTRD.as_slice_mut());

    //
    // Initialize build-in frame name- and ID-based hashes.
    //
    ZZHSCINI(MAXBFR, BNMLST.as_slice_mut(), BNMPOL.as_slice_mut(), ctx)?;
    ZZHSIINI(MAXBFR, BIDLST.as_slice_mut(), BIDPOL.as_slice_mut(), ctx)?;

    //
    // Register all built-in frames in the frame name- and ID-based
    // hashes.
    //
    for I in 1..=NNAMES {
        ZZHSCADD(
            BNMLST.as_slice_mut(),
            BNMPOL.as_slice_mut(),
            BNMNMS.as_arg_mut(),
            &NAME[I],
            &mut ITEM,
            &mut NEW,
            ctx,
        )?;
        BNMIDX[ITEM] = I;

        ZZHSIADD(
            BIDLST.as_slice_mut(),
            BIDPOL.as_slice_mut(),
            BIDIDS.as_slice_mut(),
            IDCODE[I],
            &mut ITEM,
            &mut NEW,
            ctx,
        )?;
        BIDIDX[ITEM] = I;
    }

    Ok(())
}
