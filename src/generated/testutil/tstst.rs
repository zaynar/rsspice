//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    DT: f64,
    ELEMS: StackArray<f64, 8>,
    EPOCH: f64,
    INCND: StackArray<f64, 6>,
    LAT: f64,
    LONG: f64,
    RAD: f64,
    BODCOD: StackArray<i32, 46>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DT: f64 = 0.0;
        let mut ELEMS = StackArray::<f64, 8>::new(1..=8);
        let mut EPOCH: f64 = 0.0;
        let mut INCND = StackArray::<f64, 6>::new(1..=6);
        let mut LAT: f64 = 0.0;
        let mut LONG: f64 = 0.0;
        let mut RAD: f64 = 0.0;
        let mut BODCOD = StackArray::<i32, 46>::new(1..=46);
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            DT,
            ELEMS,
            EPOCH,
            INCND,
            LAT,
            LONG,
            RAD,
            BODCOD,
            FIRST,
        }
    }
}

//$Procedure      TSTST ( Test States )
pub fn TSTST(
    BODY: i32,
    ET: f64,
    SEGID: &mut [u8],
    FRAME: &mut i32,
    STATE: &mut [f64],
    CENTER: &mut i32,
    GM: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut STATE = DummyArrayMut::new(STATE, 1..=6);

    //
    // Spicelib Functions.
    //

    //
    // Local Variables.
    //

    *GM = 0.0;

    if (BODY == 1) {
        *CENTER = 10;
        *FRAME = 1;
        fstr::assign(SEGID, b"MERCURY BARYCENTER");
        *GM = 132712439812.232;
        save.INCND[1] = 6207609.7897357;
        save.INCND[2] = -60298891.717493;
        save.INCND[3] = -32852628.5165522;
        save.INCND[4] = 38.745299986085;
        save.INCND[5] = 7.53210082426381;
        save.INCND[6] = 0.004182185953812643;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 2) {
        *CENTER = 10;
        *FRAME = 2;
        fstr::assign(SEGID, b"VENUS BARYCENTER");
        *GM = 132712439812.232;
        save.INCND[1] = 1312825.17425851;
        save.INCND[2] = -99125997.5689186;
        save.INCND[3] = -44760147.8732995;
        save.INCND[4] = 34.7805759160042;
        save.INCND[5] = 1.06667067580094;
        save.INCND[6] = -1.71517793493851;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 3) {
        *CENTER = 10;
        *FRAME = 3;
        fstr::assign(SEGID, b"EARTH BARYCENTER");
        *GM = 132712439812.232;
        save.INCND[1] = -24848019.402164;
        save.INCND[2] = 133025176.220672;
        save.INCND[3] = 57675709.08939;
        save.INCND[4] = -29.8465259739246;
        save.INCND[5] = -4.71231481326271;
        save.INCND[6] = -2.04279835682576;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 4) {
        *CENTER = 10;
        *FRAME = 4;
        fstr::assign(SEGID, b"MARS BARYCENTER");
        *GM = 132712439812.232;
        save.INCND[1] = 28451451.7577781;
        save.INCND[2] = -193607199.534214;
        save.INCND[3] = -89632951.5594207;
        save.INCND[4] = 24.9574701756996;
        save.INCND[5] = 5.0346156092184;
        save.INCND[6] = 1.64262344268785;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 5) {
        *CENTER = 10;
        *FRAME = 5;
        fstr::assign(SEGID, b"JUPITER BARYCENTER");
        *GM = 132712439812.232;
        save.INCND[1] = -704576067.694724;
        save.INCND[2] = -381531065.492479;
        save.INCND[3] = -146450554.954635;
        save.INCND[4] = 6.39243901369846;
        save.INCND[5] = -9.78250291412574;
        save.INCND[6] = -4.35258281505283;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 6) {
        *CENTER = 10;
        *FRAME = 6;
        fstr::assign(SEGID, b"SATURN BARYCENTER");
        *GM = 132712439812.232;
        save.INCND[1] = 1273788376.1546;
        save.INCND[2] = -643544424.24411;
        save.INCND[3] = -321266985.888865;
        save.INCND[4] = 4.22310030788441;
        save.INCND[5] = 7.82646875968136;
        save.INCND[6] = 3.05408851781389;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 7) {
        *CENTER = 10;
        *FRAME = 7;
        fstr::assign(SEGID, b"URANUS BARYCENTER");
        *GM = 132712439812.232;
        save.INCND[1] = 1077677273.8789;
        save.INCND[2] = -2496912503.08958;
        save.INCND[3] = -1109243386.02389;
        save.INCND[4] = 6.2879788913549;
        save.INCND[5] = 2.02629977465106;
        save.INCND[6] = 0.799052496371982;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 8) {
        *CENTER = 10;
        *FRAME = 8;
        fstr::assign(SEGID, b"NEPTUNE BARYCENTER");
        *GM = 132712439812.232;
        save.INCND[1] = 1557167603.67853;
        save.INCND[2] = -3907790548.02631;
        save.INCND[3] = -1639997084.052;
        save.INCND[4] = 5.07463401994239;
        save.INCND[5] = 1.80430448197816;
        save.INCND[6] = 0.611157155988252;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 9) {
        *CENTER = 10;
        *FRAME = 9;
        fstr::assign(SEGID, b"PLUTO BARYCENTER");
        *GM = 132712439812.232;
        save.INCND[1] = -2469292714.21367;
        save.INCND[2] = -3681472580.33229;
        save.INCND[3] = -414761412.770993;
        save.INCND[4] = 4.6660184508844;
        save.INCND[5] = -3.13082213350134;
        save.INCND[6] = -2.40190082431715;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 301) {
        *CENTER = 399;
        *FRAME = 11;
        fstr::assign(SEGID, b"MOON");
        *GM = 398600.447703261;
        save.INCND[1] = -276834.343369683;
        save.INCND[2] = 244566.81570172;
        save.INCND[3] = 70213.2363667674;
        save.INCND[4] = -0.68298447864965;
        save.INCND[5] = -0.719654421362183;
        save.INCND[6] = -0.334442098948741;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 401) {
        *CENTER = 499;
        *FRAME = 12;
        fstr::assign(SEGID, b"PHOBOS");
        *GM = 42826.2865489937;
        save.INCND[1] = 5791.99461342131;
        save.INCND[2] = -4660.30566140624;
        save.INCND[3] = -5831.14238473872;
        save.INCND[4] = 1.37607578437703;
        save.INCND[5] = 1.6153209117764;
        save.INCND[6] = 0.0322007203899467;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 402) {
        *CENTER = 499;
        *FRAME = 13;
        fstr::assign(SEGID, b"DEIMOS");
        *GM = 42826.2865489937;
        save.INCND[1] = 11967.6502996;
        save.INCND[2] = 1968.17389762766;
        save.INCND[3] = -20084.9533725419;
        save.INCND[4] = -1.15747991364628;
        save.INCND[5] = -0.056523665463048;
        save.INCND[6] = -0.695674823559072;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 501) {
        *CENTER = 599;
        *FRAME = 14;
        fstr::assign(SEGID, b"IO");
        *GM = 126686531.827629;
        save.INCND[1] = -156343.978772058;
        save.INCND[2] = 355461.582129399;
        save.INCND[3] = 166607.587136994;
        save.INCND[4] = -16.0503792563007;
        save.INCND[5] = -5.73360628238377;
        save.INCND[6] = -2.99230255081418;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 502) {
        *CENTER = 599;
        *FRAME = 15;
        fstr::assign(SEGID, b"EUROPA");
        *GM = 126686531.827629;
        save.INCND[1] = -665935.363781086;
        save.INCND[2] = 102197.012487387;
        save.INCND[3] = 32367.8703226054;
        save.INCND[4] = -2.25722400520517;
        save.INCND[5] = -12.1638661807118;
        save.INCND[6] = -5.82073291279444;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 503) {
        *CENTER = 599;
        *FRAME = 16;
        fstr::assign(SEGID, b"GANYMEDE");
        *GM = 126686531.827629;
        save.INCND[1] = 445009.819719306;
        save.INCND[2] = 958239.206929093;
        save.INCND[3] = -181485.142558463;
        save.INCND[4] = -9.23698674982273;
        save.INCND[5] = 3.40288638916646;
        save.INCND[6] = -4.60022413967004;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 504) {
        *CENTER = 599;
        *FRAME = 17;
        fstr::assign(SEGID, b"CALLISTO");
        *GM = 126686531.827629;
        save.INCND[1] = 1547991.4905258;
        save.INCND[2] = -1048659.29881788;
        save.INCND[3] = -14378.2106926829;
        save.INCND[4] = 4.61018643816157;
        save.INCND[5] = 6.84588572010446;
        save.INCND[6] = 0.286500136474338;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 603) {
        *CENTER = 699;
        *FRAME = 18;
        fstr::assign(SEGID, b"TETHYS");
        *GM = 37931155.3789196;
        save.INCND[1] = 292490.649450078;
        save.INCND[2] = -5308.30689534882;
        save.INCND[3] = -33634.0118571163;
        save.INCND[4] = -0.42479607325554;
        save.INCND[5] = 10.0579643101737;
        save.INCND[6] = -5.26247103964504;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 604) {
        *CENTER = 699;
        *FRAME = 19;
        fstr::assign(SEGID, b"DIONE");
        *GM = 37931155.3789196;
        save.INCND[1] = 251050.720397599;
        save.INCND[2] = 278878.775217632;
        save.INCND[3] = -43154.4685975013;
        save.INCND[4] = -7.44063083238366;
        save.INCND[5] = 6.70899210529303;
        save.INCND[6] = 0.189011010218273;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 605) {
        *CENTER = 699;
        *FRAME = 20;
        fstr::assign(SEGID, b"RHEA");
        *GM = 37931155.3789196;
        save.INCND[1] = -412045.348984376;
        save.INCND[2] = 329704.135818218;
        save.INCND[3] = 12131.8372350238;
        save.INCND[4] = -5.23810061755777;
        save.INCND[5] = -6.58388420161942;
        save.INCND[6] = 0.995089328747382;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 606) {
        *CENTER = 699;
        *FRAME = 2;
        fstr::assign(SEGID, b"TITAN");
        *GM = 37931155.3789196;
        save.INCND[1] = 1116223.22368248;
        save.INCND[2] = -396622.004714643;
        save.INCND[3] = -77422.7928937469;
        save.INCND[4] = 1.90395167708842;
        save.INCND[5] = 5.38118918386284;
        save.INCND[6] = -0.537090832149415;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 607) {
        *CENTER = 699;
        *FRAME = 3;
        fstr::assign(SEGID, b"HYPERION");
        *GM = 37931155.3789196;
        save.INCND[1] = 1012186.14004221;
        save.INCND[2] = 981982.837615797;
        save.INCND[3] = -158943.65784032;
        save.INCND[4] = -3.9151626724989;
        save.INCND[5] = 3.53031359608563;
        save.INCND[6] = 0.191202803576989;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 608) {
        *CENTER = 699;
        *FRAME = 4;
        fstr::assign(SEGID, b"IAPETUS");
        *GM = 37931155.3789196;
        save.INCND[1] = 3365669.21755382;
        save.INCND[2] = -340417.567250275;
        save.INCND[3] = -740618.039233572;
        save.INCND[4] = 0.418440545037304;
        save.INCND[5] = 3.29253312667935;
        save.INCND[6] = 0.502126937288676;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 701) {
        *CENTER = 799;
        *FRAME = 5;
        fstr::assign(SEGID, b"ARIEL");
        *GM = 5788511.27856709;
        save.INCND[1] = -179058.782466409;
        save.INCND[2] = 26052.4008543776;
        save.INCND[3] = 59968.3113388014;
        save.INCND[4] = 1.43394935766916;
        save.INCND[5] = -1.73817676665214;
        save.INCND[6] = 5.03675749813222;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 702) {
        *CENTER = 799;
        *FRAME = 6;
        fstr::assign(SEGID, b"UMBRIEL");
        *GM = 5788511.27856709;
        save.INCND[1] = -181894.849454923;
        save.INCND[2] = 90817.6994578275;
        save.INCND[3] = -170086.092798569;
        save.INCND[4] = -3.23724294238563;
        save.INCND[5] = -0.168144857094752;
        save.INCND[6] = 3.38038687224653;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 703) {
        *CENTER = 799;
        *FRAME = 7;
        fstr::assign(SEGID, b"TITANIA");
        *GM = 5788511.27856709;
        save.INCND[1] = -408091.863866807;
        save.INCND[2] = 56862.0566637421;
        save.INCND[3] = 146993.806998521;
        save.INCND[4] = 1.02245002389835;
        save.INCND[5] = -1.15183420243745;
        save.INCND[6] = 3.29200952278993;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 704) {
        *CENTER = 799;
        *FRAME = 8;
        fstr::assign(SEGID, b"OBERON");
        *GM = 5788511.27856709;
        save.INCND[1] = -197133.527861026;
        save.INCND[2] = -101978.461776863;
        save.INCND[3] = 539503.290982823;
        save.INCND[4] = 2.88316088766759;
        save.INCND[5] = -0.922225470067102;
        save.INCND[6] = 0.87934240195532;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 705) {
        *CENTER = 799;
        *FRAME = 9;
        fstr::assign(SEGID, b"MIRANDA");
        *GM = 5788511.27856709;
        save.INCND[1] = 123194.232663384;
        save.INCND[2] = -39655.2816164707;
        save.INCND[3] = 9072.41076452709;
        save.INCND[4] = 1.07540475602442;
        save.INCND[5] = 1.87215079605174;
        save.INCND[6] = -6.32811314849574;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 801) {
        *CENTER = 899;
        *FRAME = 10;
        fstr::assign(SEGID, b"TRITON");
        *GM = 6822317.25434592;
        save.INCND[1] = -29804.0635385521;
        save.INCND[2] = 121869.821676598;
        save.INCND[3] = 331834.43952858;
        save.INCND[4] = 3.9668310117962;
        save.INCND[5] = 1.85216130354845;
        save.INCND[6] = -0.323871607592234;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 802) {
        *CENTER = 899;
        *FRAME = 11;
        fstr::assign(SEGID, b"NERIED");
        *GM = 6822317.25434592;
        save.INCND[1] = 2169661.50023747;
        save.INCND[2] = 8308261.24967138;
        save.INCND[3] = 4422710.82502134;
        save.INCND[4] = -0.41186378812063;
        save.INCND[5] = 0.07367507286839281;
        save.INCND[6] = 0.01073818194480183;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 901) {
        *CENTER = 999;
        *FRAME = 12;
        fstr::assign(SEGID, b"CHARON");
        *GM = 825.510499463536;
        save.INCND[1] = 3194.09001334327;
        save.INCND[2] = -1184.3591754164;
        save.INCND[3] = -19340.0401110163;
        save.INCND[4] = -0.162746497898968;
        save.INCND[5] = -0.152373801539905;
        save.INCND[6] = -0.0175845011453347;
        save.EPOCH = -189345600.0;
        save.DT = (ET - save.EPOCH);

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            save.DT,
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 199) {
        *CENTER = 1;
        fstr::assign(SEGID, b"MERCURY");
        *FRAME = 1;

        STATE[1] = 10.0;
        STATE[2] = 8.0;
        STATE[3] = 6.0;
        STATE[4] = 0.0;
        STATE[5] = 0.0;
        STATE[6] = 0.0;
    } else if (BODY == 299) {
        *CENTER = 2;
        fstr::assign(SEGID, b"VENUS");
        *FRAME = 2;

        STATE[1] = 20.0;
        STATE[2] = 16.0;
        STATE[3] = 12.0;
        STATE[4] = 0.0;
        STATE[5] = 0.0;
        STATE[6] = 0.0;
    } else if (BODY == 399) {
        *CENTER = 3;
        fstr::assign(SEGID, b"EARTH");
        *FRAME = 3;

        STATE[1] = 30.0;
        STATE[2] = 24.0;
        STATE[3] = 18.0;
        STATE[4] = 0.0;
        STATE[5] = 0.0;
        STATE[6] = 0.0;
    } else if (BODY == 499) {
        *CENTER = 4;
        fstr::assign(SEGID, b"MARS");
        *FRAME = 4;

        STATE[1] = 40.0;
        STATE[2] = 32.0;
        STATE[3] = 24.0;
        STATE[4] = 0.0;
        STATE[5] = 0.0;
        STATE[6] = 0.0;
    } else if (BODY == 599) {
        *CENTER = 5;
        fstr::assign(SEGID, b"JUPITER");
        *FRAME = 5;

        STATE[1] = 50.0;
        STATE[2] = 40.0;
        STATE[3] = 30.0;
        STATE[4] = 0.0;
        STATE[5] = 0.0;
        STATE[6] = 0.0;
    } else if (BODY == 699) {
        *CENTER = 6;
        fstr::assign(SEGID, b"SATURN");
        *FRAME = 6;

        STATE[1] = 60.0;
        STATE[2] = 48.0;
        STATE[3] = 36.0;
        STATE[4] = 0.0;
        STATE[5] = 0.0;
        STATE[6] = 0.0;
    } else if (BODY == 799) {
        *CENTER = 7;
        fstr::assign(SEGID, b"URANUS");
        *FRAME = 7;

        STATE[1] = 70.0;
        STATE[2] = 56.0;
        STATE[3] = 42.0;
        STATE[4] = 0.0;
        STATE[5] = 0.0;
        STATE[6] = 0.0;
    } else if (BODY == 899) {
        *CENTER = 8;
        fstr::assign(SEGID, b"NEPTUNE");
        *FRAME = 8;

        STATE[1] = 80.0;
        STATE[2] = 64.0;
        STATE[3] = 48.0;
        STATE[4] = 0.0;
        STATE[5] = 0.0;
        STATE[6] = 0.0;
    } else if (BODY == 999) {
        *CENTER = 9;
        fstr::assign(SEGID, b"PLUTO");
        *FRAME = 9;

        STATE[1] = 90.0;
        STATE[2] = 72.0;
        STATE[3] = 54.0;
        STATE[4] = 0.0;
        STATE[5] = 0.0;
        STATE[6] = 0.0;
    } else if (BODY == 10) {
        *CENTER = 0;
        fstr::assign(SEGID, b"SUN");
        *FRAME = 1;
        STATE[1] = 10.0;
        STATE[2] = 10.0;
        STATE[3] = 10.0;
        STATE[4] = 0.0;
        STATE[5] = 0.0;
        STATE[6] = 0.0;
    } else if (BODY == 399001) {
        *CENTER = 399;
        fstr::assign(SEGID, b"GOLDSTONE");

        spicelib::NAMFRM(b"IAU_EARTH", FRAME, ctx)?;

        STATE[1] = -2356.1565;
        STATE[2] = -4646.8408;
        STATE[3] = 3668.287;
        STATE[4] = 0.0;
        STATE[5] = 0.0;
        STATE[6] = 0.0;
    } else if (BODY == 399002) {
        *CENTER = 399;
        fstr::assign(SEGID, b"CANBERRA");

        spicelib::NAMFRM(b"IAU_EARTH", FRAME, ctx)?;

        STATE[1] = -4450.9405;
        STATE[2] = 2676.8718;
        STATE[3] = -3691.4945;
        STATE[4] = 0.0;
        STATE[5] = 0.0;
        STATE[6] = 0.0;
    } else if (BODY == 399003) {
        *CENTER = 399;
        fstr::assign(SEGID, b"MADRID");

        spicelib::NAMFRM(b"IAU_EARTH", FRAME, ctx)?;

        STATE[1] = 4847.9106;
        STATE[2] = -353.3451;
        STATE[3] = 4117.1925;
        STATE[4] = 0.0;
        STATE[5] = 0.0;
        STATE[6] = 0.0;
    } else if (BODY == -9) {
        *CENTER = 301;
        spicelib::NAMFRM(b"ECLIPJ2000", FRAME, ctx)?;
        fstr::assign(SEGID, b"PHOENIX SPACECRAFT");
        save.EPOCH = -189345600.0;
        *GM = 4902.79906388137;

        save.ELEMS[1] = 3000.0;
        save.ELEMS[2] = 0.0;
        save.ELEMS[3] = (spicelib::PI(ctx) / 3.0);
        save.ELEMS[4] = 0.0;
        save.ELEMS[5] = 0.0;
        save.ELEMS[6] = 0.0;
        save.ELEMS[7] = 0.0;
        save.ELEMS[8] = *GM;

        spicelib::CONICS(
            save.ELEMS.as_slice(),
            save.EPOCH,
            save.INCND.as_slice_mut(),
            ctx,
        )?;

        spicelib::PROP2B(
            *GM,
            save.INCND.as_slice(),
            (ET - save.EPOCH),
            STATE.as_slice_mut(),
            ctx,
        )?;
    } else if (BODY == 401001) {
        *CENTER = 401;
        fstr::assign(SEGID, b"PHOBOS BASECAMP");

        spicelib::NAMFRM(b"IAU_PHOBOS", FRAME, ctx)?;

        STATE[1] = 13.502;
        STATE[2] = 0.0;
        STATE[3] = 0.0;
        STATE[4] = 0.0;
        STATE[5] = 0.0;
        STATE[6] = 0.0;
    } else if (BODY == 301001) {
        *CENTER = 301;
        fstr::assign(SEGID, b"TRANQUILITY BASE");

        spicelib::NAMFRM(b"IAU_MOON", FRAME, ctx)?;

        save.LAT = (5.0 * spicelib::DPR(ctx));
        save.LONG = -(25.0 * spicelib::DPR(ctx));
        save.RAD = 1737.402;

        spicelib::LATREC(save.RAD, save.LONG, save.LAT, STATE.as_slice_mut());

        STATE[4] = 0.0;
        STATE[5] = 0.0;
        STATE[6] = 0.0;
    }

    Ok(())
}

pub fn TSTSTC(BODY: i32, CENTER: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if save.FIRST {
        save.FIRST = false;

        save.BODCOD[1] = -9;
        save.BODCOD[2] = 1;
        save.BODCOD[3] = 2;
        save.BODCOD[4] = 3;
        save.BODCOD[5] = 4;
        save.BODCOD[6] = 5;
        save.BODCOD[7] = 6;
        save.BODCOD[8] = 7;
        save.BODCOD[9] = 8;
        save.BODCOD[10] = 9;
        save.BODCOD[11] = 301;
        save.BODCOD[12] = 401;
        save.BODCOD[13] = 402;
        save.BODCOD[14] = 501;
        save.BODCOD[15] = 502;
        save.BODCOD[16] = 503;
        save.BODCOD[17] = 504;
        save.BODCOD[18] = 603;
        save.BODCOD[19] = 604;
        save.BODCOD[20] = 605;
        save.BODCOD[21] = 606;
        save.BODCOD[22] = 607;
        save.BODCOD[23] = 608;
        save.BODCOD[24] = 701;
        save.BODCOD[25] = 702;
        save.BODCOD[26] = 703;
        save.BODCOD[27] = 704;
        save.BODCOD[28] = 705;
        save.BODCOD[29] = 801;
        save.BODCOD[30] = 802;
        save.BODCOD[31] = 901;
        save.BODCOD[32] = 199;
        save.BODCOD[33] = 299;
        save.BODCOD[34] = 399;
        save.BODCOD[35] = 499;
        save.BODCOD[36] = 599;
        save.BODCOD[37] = 699;
        save.BODCOD[38] = 799;
        save.BODCOD[39] = 899;
        save.BODCOD[40] = 999;
        save.BODCOD[41] = 10;
        save.BODCOD[42] = 399001;
        save.BODCOD[43] = 399002;
        save.BODCOD[44] = 399003;
        save.BODCOD[45] = 401001;
        save.BODCOD[46] = 301001;
    }

    if ((BODY < 0) || (BODY > 46)) {
        *CENTER = 0;
    } else {
        *CENTER = save.BODCOD[BODY];
    }
}
