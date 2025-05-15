//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NGRAVS: i32 = 3;
const NGRAVC: i32 = 8;
const WGS721: i32 = 1;
const WGS72: i32 = 2;
const WGS84: i32 = 3;
const P_RAD: i32 = 1;
const P_XKE: i32 = 2;
const P_MU: i32 = 3;
const P_TUMN: i32 = 4;
const P_J2: i32 = 5;
const P_J3: i32 = 6;
const P_J4: i32 = 7;
const P_J3J2: i32 = 8;
const K_J2: i32 = 1;
const K_J3: i32 = 2;
const K_J4: i32 = 3;
const K_KE: i32 = 4;
const K_QO: i32 = 5;
const K_SO: i32 = 6;
const K_ER: i32 = 7;
const K_AE: i32 = 8;
const NGEO: i32 = K_AE;
const AFSPC: i32 = 1;
const IMPRVD: i32 = 2;
const KNDT20: i32 = 1;
const KNDD60: i32 = 2;
const KBSTAR: i32 = 3;
const KINCL: i32 = 4;
const KNODE0: i32 = 5;
const KECC: i32 = 6;
const KOMEGA: i32 = 7;
const KMO: i32 = 8;
const KNO: i32 = 9;
const KEPOCH: i32 = 10;
const NELEMS: i32 = KEPOCH;
const LNSIZE: i32 = 80;
const NMIX: i32 = 29;
const MED: f64 = 0.0000000001;

//$Procedure F_TLE ( Family of tests for TLE evaluator )
pub fn F_TLE(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut MSG = ActualCharArray::new(24, 1..=NMIX);
    let mut ELE = ActualCharArray::new(LNSIZE, 1..=(NMIX * 2));
    let mut GEOPHS = StackArray::<f64, 8>::new(1..=8);
    let mut ELEMS = StackArray::<f64, 10>::new(1..=10);
    let mut EXPSTA = StackArray2D::<f64, 174>::new(1..=6, 1..=NMIX);
    let mut OFFSET = StackArray::<f64, 29>::new(1..=NMIX);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut TOL = StackArray::<f64, 29>::new(1..=NMIX);
    let mut ET: f64 = 0.0;
    let mut N: i32 = 0;
    let mut ERR = StackArray::<bool, 29>::new(1..=NMIX);

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_TLE", ctx)?;

    //
    // Case: -----------------------------------------------------------
    //
    testutil::TCASE(b"Preliminaries", ctx)?;

    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    GEOPHS[1] = 0.001082616;
    GEOPHS[2] = -0.00000253881;
    GEOPHS[3] = -0.00000165597;
    GEOPHS[4] = 0.0743669161;
    GEOPHS[5] = 120.0;
    GEOPHS[6] = 78.0;
    GEOPHS[7] = 6378.135;
    GEOPHS[8] = 1.0;

    //
    // Case: -----------------------------------------------------------
    //
    testutil::TCASE(b"TLE set mixing low and high orbit elements.", ctx)?;

    //
    // Test TLE evaluations using SPICE code, compare results
    // to values calculated from non SPICE routines.
    //
    // EXPSTA data values taken from output of EVSGP4.
    // That program uses only Vallado 2006 code for TLE
    // propagation.
    //
    // Evaluate each TLE at epoch, or at an epoch + offset
    // that causes an error signal.
    //
    // ERR, boolean indicating if expected SPICE error.
    // MSG, expected SPICE error message, if any.
    // OFFSET, from TLE epoch, in TDB seconds.
    //
    N = 1;

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 00005U 58002B   00179.78495062  .00000023  00000-0  28098-4 0  4753",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 00005  34.2682 348.7242 1859667 331.7664  19.3264 10.82419157413667",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 7022.465290574917;
    EXPSTA[[2, N]] = -1400.0829671389206;
    EXPSTA[[3, N]] = 0.03995155289906709;
    EXPSTA[[4, N]] = 1.8938410139509;
    EXPSTA[[5, N]] = 6.405893757305204;
    EXPSTA[[6, N]] = 4.534807249007841;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 04632U 70093B   04031.91070959 -.00000084  00000-0  10000-3 0  9955",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 04632  11.4628 273.1101 1450506 207.6000 143.9350  1.20231981 44145",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 2334.114500153129;
    EXPSTA[[2, N]] = -41920.44034102363;
    EXPSTA[[3, N]] = -0.03867437380474418;
    EXPSTA[[4, N]] = 2.826321031169248;
    EXPSTA[[5, N]] = -0.06509166397740397;
    EXPSTA[[6, N]] = 0.5709360528850898;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 06251U 62025E   06176.82412014  .00008885  00000-0  12808-3 0  3985",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 06251  58.0579  54.0425 0030035 139.1568 221.1854 15.56387291  6774",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 3988.310225809925;
    EXPSTA[[2, N]] = 5498.966570716392;
    EXPSTA[[3, N]] = 0.9005587831151146;
    EXPSTA[[4, N]] = -3.290032736958736;
    EXPSTA[[5, N]] = 2.357652818934991;
    EXPSTA[[6, N]] = 6.496623473026671;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 08195U 75081A   06176.33215444  .00000099  00000-0  11873-3 0   813",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 08195  64.1586 279.0717 6877146 264.7651  20.2257  2.00491383225656",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 2349.894832798315;
    EXPSTA[[2, N]] = -14785.938111219008;
    EXPSTA[[3, N]] = 0.0211937769819722;
    EXPSTA[[4, N]] = 2.721488094749182;
    EXPSTA[[5, N]] = -3.2568116536917593;
    EXPSTA[[6, N]] = 4.498416671032978;

    TOL[N] = (MED * 10.0);

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 09880U 77021A   06176.56157475  .00000421  00000-0  10000-3 0  9814",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 09880  64.5968 349.3786 7069051 270.0229  16.3320  2.00813614112380",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 13020.067503971142;
    EXPSTA[[2, N]] = -2449.0719342705006;
    EXPSTA[[3, N]] = 1.1589602950956694;
    EXPSTA[[4, N]] = 4.247363933600489;
    EXPSTA[[5, N]] = 1.597178500372786;
    EXPSTA[[6, N]] = 4.956708609916437;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 09998U 74033F   05148.79417928 -.00000112  00000-0  00000+0 0  4480",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 09998   9.4958 313.1750 0270971 327.5225  30.8097  1.16186785 45878",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 25532.9894650758;
    EXPSTA[[2, N]] = -27244.263271425338;
    EXPSTA[[3, N]] = -1.1157242099297304;
    EXPSTA[[4, N]] = 2.410283884337143;
    EXPSTA[[5, N]] = 2.1941756819375415;
    EXPSTA[[6, N]] = 0.5458885255942755;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 11801U          80230.29629788  .01431103  00000-0  14311-1      13",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 11801  46.7916 230.4354 7318036  47.4722  10.4117  2.28537848    13",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 7473.371022692968;
    EXPSTA[[2, N]] = 428.94748300162695;
    EXPSTA[[3, N]] = 5828.748466090221;
    EXPSTA[[4, N]] = 5.107155389343543;
    EXPSTA[[5, N]] = 6.444680302711043;
    EXPSTA[[6, N]] = -0.1861332972887869;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 14128U 83058A   06176.02844893 -.00000158  00000-0  10000-3 0  9627",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 14128  11.4384  35.2134 0011562  26.4582 333.5652  0.98870114 46093",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 34747.5793166293;
    EXPSTA[[2, N]] = 24502.371133501172;
    EXPSTA[[3, N]] = -1.3283298582193905;
    EXPSTA[[4, N]] = -1.73164266139195;
    EXPSTA[[5, N]] = 2.4527726147067073;
    EXPSTA[[6, N]] = 0.6085100805112434;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 16925U 86065D   06151.67415771  .02550794 -30915-6  18784-3 0  4486",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 16925  62.0906 295.0239 5596327 245.1593  47.9690  4.88511875148616",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 5559.116866702386;
    EXPSTA[[2, N]] = -11941.04090426173;
    EXPSTA[[3, N]] = -19.412352061843578;
    EXPSTA[[4, N]] = 3.392116760624535;
    EXPSTA[[5, N]] = -1.9469851236560267;
    EXPSTA[[6, N]] = 4.250755851183446;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 20413U 83020D   05363.79166667  .00000000  00000-0  00000+0 0  7041",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 20413  12.3514 187.4253 7864447 196.3027 356.5478  0.24690082  7978",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 25123.292899943484;
    EXPSTA[[2, N]] = -13225.49965893251;
    EXPSTA[[3, N]] = 3249.4035177273227;
    EXPSTA[[4, N]] = 0.4886834189156268;
    EXPSTA[[5, N]] = 4.797897592033025;
    EXPSTA[[6, N]] = -0.9611196924769664;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 21897U 92011A   06176.02341244 -.00001273  00000-0 -13525-3 0  3044",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 21897  62.1749 198.0096 7421690 253.0462  20.1561  2.01269994104880",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = -14464.721347520543;
    EXPSTA[[2, N]] = -4699.195174470596;
    EXPSTA[[3, N]] = 0.06681684809516293;
    EXPSTA[[4, N]] = -3.2493120125354;
    EXPSTA[[5, N]] = -3.281032705977549;
    EXPSTA[[6, N]] = 4.007046938418753;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 22312U 93002D   06094.46235912  .99999999  81888-5  49949-3 0  3953",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 22312  62.1486  77.4698 0308723 267.9229  88.7392 15.95744531 98783",
    );
    ERR[N] = true;
    fstr::assign(MSG.get_mut(N), b"SPICE(BADMECCENTRICITY)");
    OFFSET[N] = (490.0 * 60.0);

    EXPSTA[[1, N]] = 0.0;
    EXPSTA[[2, N]] = 0.0;
    EXPSTA[[3, N]] = 0.0;
    EXPSTA[[4, N]] = 0.0;
    EXPSTA[[5, N]] = 0.0;
    EXPSTA[[6, N]] = 0.0;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 22674U 93035D   06176.55909107  .00002121  00000-0  29868-3 0  6569",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 22674  63.5035 354.4452 7541712 253.3264  18.7754  1.96679808 93877",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 14712.220228426777;
    EXPSTA[[2, N]] = -1443.8106180803009;
    EXPSTA[[3, N]] = 0.8349788703152157;
    EXPSTA[[4, N]] = 4.418965469053753;
    EXPSTA[[5, N]] = 1.6295920970275477;
    EXPSTA[[6, N]] = 4.11553180051005;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 23177U 94040C   06175.45752052  .00000386  00000-0  76590-3 0    95",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 23177   7.0496 179.8238 7258491 296.0482   8.3061  2.25906668 97438",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = -8801.600464447109;
    EXPSTA[[2, N]] = -0.03357557281420691;
    EXPSTA[[3, N]] = -0.4452274263610896;
    EXPSTA[[4, N]] = -3.8352790996615878;
    EXPSTA[[5, N]] = -7.662552173174574;
    EXPSTA[[6, N]] = 0.944561322866858;

    TOL[N] = (MED * 10.0);

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 23333U 94071A   94305.49999999 -.00172956  26967-3  10000-3 0    15",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 23333  28.7490   2.3720 9728298  30.4360   1.3500  0.07309491    70",
    );
    ERR[N] = true;
    fstr::assign(MSG.get_mut(N), b"SPICE(ORBITDECAY)");
    OFFSET[N] = -(20.0 * 60.0);

    EXPSTA[[1, N]] = 0.0;
    EXPSTA[[2, N]] = 0.0;
    EXPSTA[[3, N]] = 0.0;
    EXPSTA[[4, N]] = 0.0;
    EXPSTA[[5, N]] = 0.0;
    EXPSTA[[6, N]] = 0.0;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 23599U 95029B   06171.76535463  .00085586  12891-6  12956-2 0  2905",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 23599   6.9327   0.2849 5782022 274.4436  25.2425  4.47796565123555",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 9892.637940464172;
    EXPSTA[[2, N]] = 35.76144967942867;
    EXPSTA[[3, N]] = -1.0822883762430016;
    EXPSTA[[4, N]] = 3.556643235656842;
    EXPSTA[[5, N]] = 6.456009373182446;
    EXPSTA[[6, N]] = 0.783610889617117;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 24208U 96044A   06177.04061740 -.00000094  00000-0  10000-3 0  1600",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 24208   3.8536  80.0121 0026640 311.0977  48.3000  1.00778054 36119",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 7534.1098696537065;
    EXPSTA[[2, N]] = 41266.39265615613;
    EXPSTA[[3, N]] = -0.10801028478177664;
    EXPSTA[[4, N]] = -3.027168007457952;
    EXPSTA[[5, N]] = 0.55884899599332;
    EXPSTA[[6, N]] = 0.20798275541008424;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 25954U 99060A   04039.68057285 -.00000108  00000-0  00000-0 0  6847",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 25954   0.0004 243.8136 0001765  15.5294  22.7134  1.00271289 15615",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 8827.156602095425;
    EXPSTA[[2, N]] = -41223.00970011407;
    EXPSTA[[3, N]] = 3.6348296275007628;
    EXPSTA[[4, N]] = 3.007087317624413;
    EXPSTA[[5, N]] = 0.6437013229400458;
    EXPSTA[[6, N]] = 0.0009416629997292906;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 26900U 01039A   06106.74503247  .00000045  00000-0  10000-3 0  8290",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 26900   0.0164 266.5378 0003319  86.1794 182.2590  1.00273847 16981",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = -42014.83794537469;
    EXPSTA[[2, N]] = 3702.3435766153675;
    EXPSTA[[3, N]] = -26.675002565785363;
    EXPSTA[[4, N]] = -0.26977524684110693;
    EXPSTA[[5, N]] = -3.061854392453723;
    EXPSTA[[6, N]] = 0.0003367257377408031;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 26975U 78066F   06174.85818871  .00000620  00000-0  10000-3 0  6809",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 26975  68.4714 236.1303 5602877 123.7484 302.5767  2.05657553 67521",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = -14506.923133367129;
    EXPSTA[[2, N]] = -21613.56042638596;
    EXPSTA[[3, N]] = 10.050188926590833;
    EXPSTA[[4, N]] = 2.212943307460569;
    EXPSTA[[5, N]] = 1.1599708913588078;
    EXPSTA[[6, N]] = 3.0206002010544735;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 28057U 03049A   06177.78615833  .00000060  00000-0  35940-4 0  1836",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 28057  98.4283 247.6961 0000884  88.1964 271.9322 14.35478080140550",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = -2715.282374049035;
    EXPSTA[[2, N]] = -6619.264366924208;
    EXPSTA[[3, N]] = -0.013414434536107979;
    EXPSTA[[4, N]] = -1.0085872729754288;
    EXPSTA[[5, N]] = 0.42278200265497323;
    EXPSTA[[6, N]] = 7.385272939405877;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 28129U 03058A   06175.57071136 -.00000104  00000-0  10000-3 0   459",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 28129  54.7298 324.8098 0048506 266.2640  93.1663  2.00562768 18443",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 21707.464117055755;
    EXPSTA[[2, N]] = -15318.617519348343;
    EXPSTA[[3, N]] = 0.13551151963463845;
    EXPSTA[[4, N]] = 1.3040292138648395;
    EXPSTA[[5, N]] = 1.816904973704573;
    EXPSTA[[6, N]] = 3.1619199752770473;

    TOL[N] = (MED * 10.0);

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 28350U 04020A   06167.21788666  .16154492  76267-5  18678-3 0  8894",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 28350  64.9977 345.6130 0024870 260.7578  99.9590 16.47856722116490",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 6333.081229399808;
    EXPSTA[[2, N]] = -1580.8285227911904;
    EXPSTA[[3, N]] = 90.69355717327291;
    EXPSTA[[4, N]] = 0.7146344232317159;
    EXPSTA[[5, N]] = 3.2242465486030065;
    EXPSTA[[6, N]] = 7.083128130183688;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 28623U 05006B   06177.81079184  .00637644  69054-6  96390-3 0  6000",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 28623  28.5200 114.9834 6249053 170.2550 212.8965  3.79477162 12753",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = -11665.709019766888;
    EXPSTA[[2, N]] = 24943.614326156436;
    EXPSTA[[3, N]] = 25.805436322352136;
    EXPSTA[[4, N]] = -1.596228620974948;
    EXPSTA[[5, N]] = -1.4761279607723508;
    EXPSTA[[6, N]] = 1.1260597533138914;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 28626U 05008A   06176.46683397 -.00000205  00000-0  10000-3 0  2190",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 28626   0.0019 286.9433 0000335  13.7918  55.6504  1.00270176  4891",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 42080.71850961155;
    EXPSTA[[2, N]] = -2646.8638735693594;
    EXPSTA[[3, N]] = 0.8185129388915973;
    EXPSTA[[4, N]] = 0.1931051773091712;
    EXPSTA[[5, N]] = 3.0686882496601733;
    EXPSTA[[6, N]] = 0.00043844943135621;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 28872U 05037B   05333.02012661  .25992681  00000-0  24476-3 0  1534",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 28872  96.4736 157.9986 0303955 244.0492 110.6523 16.46015938 10708",
    );
    ERR[N] = true;
    fstr::assign(MSG.get_mut(N), b"SPICE(SUBORBITAL)");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 0.0;
    EXPSTA[[2, N]] = 0.0;
    EXPSTA[[3, N]] = 0.0;
    EXPSTA[[4, N]] = 0.0;
    EXPSTA[[5, N]] = 0.0;
    EXPSTA[[6, N]] = 0.0;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 29141U 85108AA  06170.26783845  .99999999  00000-0  13519-0 0   718",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 29141  82.4288 273.4882 0015848 277.2124  83.9133 15.93343074  6828",
    );
    ERR[N] = true;
    fstr::assign(MSG.get_mut(N), b"SPICE(BADMSEMIMAJOR)");
    OFFSET[N] = -(1400.0 * 60.0);

    EXPSTA[[1, N]] = 0.0;
    EXPSTA[[2, N]] = 0.0;
    EXPSTA[[3, N]] = 0.0;
    EXPSTA[[4, N]] = 0.0;
    EXPSTA[[5, N]] = 0.0;
    EXPSTA[[6, N]] = 0.0;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 29238U 06022G   06177.28732010  .00766286  10823-4  13334-2 0   101",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 29238  51.5595 213.7903 0202579  95.2503 267.9010 15.73823839  1061",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = -5566.59512653766;
    EXPSTA[[2, N]] = -3789.7599104569354;
    EXPSTA[[3, N]] = 67.60382242983637;
    EXPSTA[[4, N]] = 2.873759366091939;
    EXPSTA[[5, N]] = -3.8253405215249763;
    EXPSTA[[6, N]] = 6.023253923747367;

    TOL[N] = MED;

    N = (N + 1);

    fstr::assign(
        ELE.get_mut(((2 * N) - 1)),
        b"1 88888U          80275.98708465  .00073094  13844-3  66816-4 0    87",
    );
    fstr::assign(
        ELE.get_mut((2 * N)),
        b"2 88888  72.8435 115.9689 0086731  52.6988 110.5714 16.05824518  1058",
    );
    ERR[N] = false;
    fstr::assign(MSG.get_mut(N), b" ");
    OFFSET[N] = 0.0;

    EXPSTA[[1, N]] = 2328.9697519299193;
    EXPSTA[[2, N]] = -5995.22051159742;
    EXPSTA[[3, N]] = 1719.972971401438;
    EXPSTA[[4, N]] = 2.9120732803862603;
    EXPSTA[[5, N]] = -0.9834179555028624;
    EXPSTA[[6, N]] = -7.090816207954589;

    TOL[N] = MED;

    //
    // Check the number of mixed element sets equals the expected
    // number of mix element sets.
    //
    testutil::CHCKSI(b"NMIX", NMIX, b"=", N, 0, OK, ctx)?;

    //
    // Compare EVSGP4 output to that from the TLE set included in
    // the 2006 Vallado paper. This TLE set includes low and high
    // orbit vehicles.
    //
    for I in 0..=28 {
        testutil::TCASE(&ELE[((2 * I) + 1)], ctx)?;

        spicelib::GETELM(
            1950,
            ELE.subarray(((I * 2) + 1)),
            &mut ET,
            ELEMS.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Confirm evaluation at epoch of each TLE set. If
        // the evaluation fails, confirm expected error.
        //
        spicelib::EVSGP4(
            (ET + OFFSET[(I + 1)]),
            GEOPHS.as_slice(),
            ELEMS.as_slice(),
            STATE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(ERR[(I + 1)], &MSG[(I + 1)], OK, ctx)?;

        //
        // If no error and expected result, compare the STATE
        // vector against an expected state.
        //
        if (*OK && !ERR[(I + 1)]) {
            //
            // Confirm evaluation equals to the case-specific
            // tolerance the output from the Vallado based routine.
            //
            testutil::CHCKAD(
                b"EXPST",
                EXPSTA.subarray([1, (I + 1)]),
                b"~~/",
                STATE.as_slice(),
                6,
                TOL[(I + 1)],
                OK,
                ctx,
            )?;
        }
    }

    //
    // Case: -----------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    //
    // Clear pool.
    //
    spicelib::KCLEAR(ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
