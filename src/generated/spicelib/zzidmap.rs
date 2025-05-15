//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXL: i32 = 36;
const MAXP: i32 = 150;
const NPERM: i32 = 692;
const MAXE: i32 = 853;
const NROOM: i32 = 14983;

//$Procedure ZZIDMAP ( Private --- SPICE body ID/name assignments )
pub fn ZZIDMAP(BLTCOD: &mut [i32], BLTNAM: CharArrayMut) {
    let mut BLTCOD = DummyArrayMut::new(BLTCOD, 1..=NPERM);
    let mut BLTNAM = DummyCharArrayMut::new(BLTNAM, Some(MAXL), 1..=NPERM);

    //
    // A script generates this file. Do not edit by hand.
    // Edit the creation script to modify the contents of
    // ZZIDMAP.
    //

    BLTCOD[1] = 0;
    fstr::assign(BLTNAM.get_mut(1), b"SOLAR_SYSTEM_BARYCENTER");

    BLTCOD[2] = 0;
    fstr::assign(BLTNAM.get_mut(2), b"SSB");

    BLTCOD[3] = 0;
    fstr::assign(BLTNAM.get_mut(3), b"SOLAR SYSTEM BARYCENTER");

    BLTCOD[4] = 1;
    fstr::assign(BLTNAM.get_mut(4), b"MERCURY_BARYCENTER");

    BLTCOD[5] = 1;
    fstr::assign(BLTNAM.get_mut(5), b"MERCURY BARYCENTER");

    BLTCOD[6] = 2;
    fstr::assign(BLTNAM.get_mut(6), b"VENUS_BARYCENTER");

    BLTCOD[7] = 2;
    fstr::assign(BLTNAM.get_mut(7), b"VENUS BARYCENTER");

    BLTCOD[8] = 3;
    fstr::assign(BLTNAM.get_mut(8), b"EARTH_BARYCENTER");

    BLTCOD[9] = 3;
    fstr::assign(BLTNAM.get_mut(9), b"EMB");

    BLTCOD[10] = 3;
    fstr::assign(BLTNAM.get_mut(10), b"EARTH MOON BARYCENTER");

    BLTCOD[11] = 3;
    fstr::assign(BLTNAM.get_mut(11), b"EARTH-MOON BARYCENTER");

    BLTCOD[12] = 3;
    fstr::assign(BLTNAM.get_mut(12), b"EARTH BARYCENTER");

    BLTCOD[13] = 4;
    fstr::assign(BLTNAM.get_mut(13), b"MARS_BARYCENTER");

    BLTCOD[14] = 4;
    fstr::assign(BLTNAM.get_mut(14), b"MARS BARYCENTER");

    BLTCOD[15] = 5;
    fstr::assign(BLTNAM.get_mut(15), b"JUPITER_BARYCENTER");

    BLTCOD[16] = 5;
    fstr::assign(BLTNAM.get_mut(16), b"JUPITER BARYCENTER");

    BLTCOD[17] = 6;
    fstr::assign(BLTNAM.get_mut(17), b"SATURN_BARYCENTER");

    BLTCOD[18] = 6;
    fstr::assign(BLTNAM.get_mut(18), b"SATURN BARYCENTER");

    BLTCOD[19] = 7;
    fstr::assign(BLTNAM.get_mut(19), b"URANUS_BARYCENTER");

    BLTCOD[20] = 7;
    fstr::assign(BLTNAM.get_mut(20), b"URANUS BARYCENTER");

    BLTCOD[21] = 8;
    fstr::assign(BLTNAM.get_mut(21), b"NEPTUNE_BARYCENTER");

    BLTCOD[22] = 8;
    fstr::assign(BLTNAM.get_mut(22), b"NEPTUNE BARYCENTER");

    BLTCOD[23] = 9;
    fstr::assign(BLTNAM.get_mut(23), b"PLUTO_BARYCENTER");

    BLTCOD[24] = 9;
    fstr::assign(BLTNAM.get_mut(24), b"PLUTO BARYCENTER");

    BLTCOD[25] = 10;
    fstr::assign(BLTNAM.get_mut(25), b"SUN");

    BLTCOD[26] = 199;
    fstr::assign(BLTNAM.get_mut(26), b"MERCURY");

    BLTCOD[27] = 299;
    fstr::assign(BLTNAM.get_mut(27), b"VENUS");

    BLTCOD[28] = 399;
    fstr::assign(BLTNAM.get_mut(28), b"EARTH");

    BLTCOD[29] = 301;
    fstr::assign(BLTNAM.get_mut(29), b"MOON");

    BLTCOD[30] = 499;
    fstr::assign(BLTNAM.get_mut(30), b"MARS");

    BLTCOD[31] = 401;
    fstr::assign(BLTNAM.get_mut(31), b"PHOBOS");

    BLTCOD[32] = 402;
    fstr::assign(BLTNAM.get_mut(32), b"DEIMOS");

    BLTCOD[33] = 599;
    fstr::assign(BLTNAM.get_mut(33), b"JUPITER");

    BLTCOD[34] = 501;
    fstr::assign(BLTNAM.get_mut(34), b"IO");

    BLTCOD[35] = 502;
    fstr::assign(BLTNAM.get_mut(35), b"EUROPA");

    BLTCOD[36] = 503;
    fstr::assign(BLTNAM.get_mut(36), b"GANYMEDE");

    BLTCOD[37] = 504;
    fstr::assign(BLTNAM.get_mut(37), b"CALLISTO");

    BLTCOD[38] = 505;
    fstr::assign(BLTNAM.get_mut(38), b"AMALTHEA");

    BLTCOD[39] = 506;
    fstr::assign(BLTNAM.get_mut(39), b"HIMALIA");

    BLTCOD[40] = 507;
    fstr::assign(BLTNAM.get_mut(40), b"ELARA");

    BLTCOD[41] = 508;
    fstr::assign(BLTNAM.get_mut(41), b"PASIPHAE");

    BLTCOD[42] = 509;
    fstr::assign(BLTNAM.get_mut(42), b"SINOPE");

    BLTCOD[43] = 510;
    fstr::assign(BLTNAM.get_mut(43), b"LYSITHEA");

    BLTCOD[44] = 511;
    fstr::assign(BLTNAM.get_mut(44), b"CARME");

    BLTCOD[45] = 512;
    fstr::assign(BLTNAM.get_mut(45), b"ANANKE");

    BLTCOD[46] = 513;
    fstr::assign(BLTNAM.get_mut(46), b"LEDA");

    BLTCOD[47] = 514;
    fstr::assign(BLTNAM.get_mut(47), b"THEBE");

    BLTCOD[48] = 515;
    fstr::assign(BLTNAM.get_mut(48), b"ADRASTEA");

    BLTCOD[49] = 516;
    fstr::assign(BLTNAM.get_mut(49), b"METIS");

    BLTCOD[50] = 517;
    fstr::assign(BLTNAM.get_mut(50), b"CALLIRRHOE");

    BLTCOD[51] = 518;
    fstr::assign(BLTNAM.get_mut(51), b"THEMISTO");

    BLTCOD[52] = 519;
    fstr::assign(BLTNAM.get_mut(52), b"MEGACLITE");

    BLTCOD[53] = 520;
    fstr::assign(BLTNAM.get_mut(53), b"TAYGETE");

    BLTCOD[54] = 521;
    fstr::assign(BLTNAM.get_mut(54), b"CHALDENE");

    BLTCOD[55] = 522;
    fstr::assign(BLTNAM.get_mut(55), b"HARPALYKE");

    BLTCOD[56] = 523;
    fstr::assign(BLTNAM.get_mut(56), b"KALYKE");

    BLTCOD[57] = 524;
    fstr::assign(BLTNAM.get_mut(57), b"IOCASTE");

    BLTCOD[58] = 525;
    fstr::assign(BLTNAM.get_mut(58), b"ERINOME");

    BLTCOD[59] = 526;
    fstr::assign(BLTNAM.get_mut(59), b"ISONOE");

    BLTCOD[60] = 527;
    fstr::assign(BLTNAM.get_mut(60), b"PRAXIDIKE");

    BLTCOD[61] = 528;
    fstr::assign(BLTNAM.get_mut(61), b"AUTONOE");

    BLTCOD[62] = 529;
    fstr::assign(BLTNAM.get_mut(62), b"THYONE");

    BLTCOD[63] = 530;
    fstr::assign(BLTNAM.get_mut(63), b"HERMIPPE");

    BLTCOD[64] = 531;
    fstr::assign(BLTNAM.get_mut(64), b"AITNE");

    BLTCOD[65] = 532;
    fstr::assign(BLTNAM.get_mut(65), b"EURYDOME");

    BLTCOD[66] = 533;
    fstr::assign(BLTNAM.get_mut(66), b"EUANTHE");

    BLTCOD[67] = 534;
    fstr::assign(BLTNAM.get_mut(67), b"EUPORIE");

    BLTCOD[68] = 535;
    fstr::assign(BLTNAM.get_mut(68), b"ORTHOSIE");

    BLTCOD[69] = 536;
    fstr::assign(BLTNAM.get_mut(69), b"SPONDE");

    BLTCOD[70] = 537;
    fstr::assign(BLTNAM.get_mut(70), b"KALE");

    BLTCOD[71] = 538;
    fstr::assign(BLTNAM.get_mut(71), b"PASITHEE");

    BLTCOD[72] = 539;
    fstr::assign(BLTNAM.get_mut(72), b"HEGEMONE");

    BLTCOD[73] = 540;
    fstr::assign(BLTNAM.get_mut(73), b"MNEME");

    BLTCOD[74] = 541;
    fstr::assign(BLTNAM.get_mut(74), b"AOEDE");

    BLTCOD[75] = 542;
    fstr::assign(BLTNAM.get_mut(75), b"THELXINOE");

    BLTCOD[76] = 543;
    fstr::assign(BLTNAM.get_mut(76), b"ARCHE");

    BLTCOD[77] = 544;
    fstr::assign(BLTNAM.get_mut(77), b"KALLICHORE");

    BLTCOD[78] = 545;
    fstr::assign(BLTNAM.get_mut(78), b"HELIKE");

    BLTCOD[79] = 546;
    fstr::assign(BLTNAM.get_mut(79), b"CARPO");

    BLTCOD[80] = 547;
    fstr::assign(BLTNAM.get_mut(80), b"EUKELADE");

    BLTCOD[81] = 548;
    fstr::assign(BLTNAM.get_mut(81), b"CYLLENE");

    BLTCOD[82] = 549;
    fstr::assign(BLTNAM.get_mut(82), b"KORE");

    BLTCOD[83] = 550;
    fstr::assign(BLTNAM.get_mut(83), b"HERSE");

    BLTCOD[84] = 553;
    fstr::assign(BLTNAM.get_mut(84), b"DIA");

    BLTCOD[85] = 699;
    fstr::assign(BLTNAM.get_mut(85), b"SATURN");

    BLTCOD[86] = 601;
    fstr::assign(BLTNAM.get_mut(86), b"MIMAS");

    BLTCOD[87] = 602;
    fstr::assign(BLTNAM.get_mut(87), b"ENCELADUS");

    BLTCOD[88] = 603;
    fstr::assign(BLTNAM.get_mut(88), b"TETHYS");

    BLTCOD[89] = 604;
    fstr::assign(BLTNAM.get_mut(89), b"DIONE");

    BLTCOD[90] = 605;
    fstr::assign(BLTNAM.get_mut(90), b"RHEA");

    BLTCOD[91] = 606;
    fstr::assign(BLTNAM.get_mut(91), b"TITAN");

    BLTCOD[92] = 607;
    fstr::assign(BLTNAM.get_mut(92), b"HYPERION");

    BLTCOD[93] = 608;
    fstr::assign(BLTNAM.get_mut(93), b"IAPETUS");

    BLTCOD[94] = 609;
    fstr::assign(BLTNAM.get_mut(94), b"PHOEBE");

    BLTCOD[95] = 610;
    fstr::assign(BLTNAM.get_mut(95), b"JANUS");

    BLTCOD[96] = 611;
    fstr::assign(BLTNAM.get_mut(96), b"EPIMETHEUS");

    BLTCOD[97] = 612;
    fstr::assign(BLTNAM.get_mut(97), b"HELENE");

    BLTCOD[98] = 613;
    fstr::assign(BLTNAM.get_mut(98), b"TELESTO");

    BLTCOD[99] = 614;
    fstr::assign(BLTNAM.get_mut(99), b"CALYPSO");

    BLTCOD[100] = 615;
    fstr::assign(BLTNAM.get_mut(100), b"ATLAS");

    BLTCOD[101] = 616;
    fstr::assign(BLTNAM.get_mut(101), b"PROMETHEUS");

    BLTCOD[102] = 617;
    fstr::assign(BLTNAM.get_mut(102), b"PANDORA");

    BLTCOD[103] = 618;
    fstr::assign(BLTNAM.get_mut(103), b"PAN");

    BLTCOD[104] = 619;
    fstr::assign(BLTNAM.get_mut(104), b"YMIR");

    BLTCOD[105] = 620;
    fstr::assign(BLTNAM.get_mut(105), b"PAALIAQ");

    BLTCOD[106] = 621;
    fstr::assign(BLTNAM.get_mut(106), b"TARVOS");

    BLTCOD[107] = 622;
    fstr::assign(BLTNAM.get_mut(107), b"IJIRAQ");

    BLTCOD[108] = 623;
    fstr::assign(BLTNAM.get_mut(108), b"SUTTUNGR");

    BLTCOD[109] = 624;
    fstr::assign(BLTNAM.get_mut(109), b"KIVIUQ");

    BLTCOD[110] = 625;
    fstr::assign(BLTNAM.get_mut(110), b"MUNDILFARI");

    BLTCOD[111] = 626;
    fstr::assign(BLTNAM.get_mut(111), b"ALBIORIX");

    BLTCOD[112] = 627;
    fstr::assign(BLTNAM.get_mut(112), b"SKATHI");

    BLTCOD[113] = 628;
    fstr::assign(BLTNAM.get_mut(113), b"ERRIAPUS");

    BLTCOD[114] = 629;
    fstr::assign(BLTNAM.get_mut(114), b"SIARNAQ");

    BLTCOD[115] = 630;
    fstr::assign(BLTNAM.get_mut(115), b"THRYMR");

    BLTCOD[116] = 631;
    fstr::assign(BLTNAM.get_mut(116), b"NARVI");

    BLTCOD[117] = 632;
    fstr::assign(BLTNAM.get_mut(117), b"METHONE");

    BLTCOD[118] = 633;
    fstr::assign(BLTNAM.get_mut(118), b"PALLENE");

    BLTCOD[119] = 634;
    fstr::assign(BLTNAM.get_mut(119), b"POLYDEUCES");

    BLTCOD[120] = 635;
    fstr::assign(BLTNAM.get_mut(120), b"DAPHNIS");

    BLTCOD[121] = 636;
    fstr::assign(BLTNAM.get_mut(121), b"AEGIR");

    BLTCOD[122] = 637;
    fstr::assign(BLTNAM.get_mut(122), b"BEBHIONN");

    BLTCOD[123] = 638;
    fstr::assign(BLTNAM.get_mut(123), b"BERGELMIR");

    BLTCOD[124] = 639;
    fstr::assign(BLTNAM.get_mut(124), b"BESTLA");

    BLTCOD[125] = 640;
    fstr::assign(BLTNAM.get_mut(125), b"FARBAUTI");

    BLTCOD[126] = 641;
    fstr::assign(BLTNAM.get_mut(126), b"FENRIR");

    BLTCOD[127] = 642;
    fstr::assign(BLTNAM.get_mut(127), b"FORNJOT");

    BLTCOD[128] = 643;
    fstr::assign(BLTNAM.get_mut(128), b"HATI");

    BLTCOD[129] = 644;
    fstr::assign(BLTNAM.get_mut(129), b"HYRROKKIN");

    BLTCOD[130] = 645;
    fstr::assign(BLTNAM.get_mut(130), b"KARI");

    BLTCOD[131] = 646;
    fstr::assign(BLTNAM.get_mut(131), b"LOGE");

    BLTCOD[132] = 647;
    fstr::assign(BLTNAM.get_mut(132), b"SKOLL");

    BLTCOD[133] = 648;
    fstr::assign(BLTNAM.get_mut(133), b"SURTUR");

    BLTCOD[134] = 649;
    fstr::assign(BLTNAM.get_mut(134), b"ANTHE");

    BLTCOD[135] = 650;
    fstr::assign(BLTNAM.get_mut(135), b"JARNSAXA");

    BLTCOD[136] = 651;
    fstr::assign(BLTNAM.get_mut(136), b"GREIP");

    BLTCOD[137] = 652;
    fstr::assign(BLTNAM.get_mut(137), b"TARQEQ");

    BLTCOD[138] = 653;
    fstr::assign(BLTNAM.get_mut(138), b"AEGAEON");

    BLTCOD[139] = 799;
    fstr::assign(BLTNAM.get_mut(139), b"URANUS");

    BLTCOD[140] = 701;
    fstr::assign(BLTNAM.get_mut(140), b"ARIEL");

    BLTCOD[141] = 702;
    fstr::assign(BLTNAM.get_mut(141), b"UMBRIEL");

    BLTCOD[142] = 703;
    fstr::assign(BLTNAM.get_mut(142), b"TITANIA");

    BLTCOD[143] = 704;
    fstr::assign(BLTNAM.get_mut(143), b"OBERON");

    BLTCOD[144] = 705;
    fstr::assign(BLTNAM.get_mut(144), b"MIRANDA");

    BLTCOD[145] = 706;
    fstr::assign(BLTNAM.get_mut(145), b"CORDELIA");

    BLTCOD[146] = 707;
    fstr::assign(BLTNAM.get_mut(146), b"OPHELIA");

    BLTCOD[147] = 708;
    fstr::assign(BLTNAM.get_mut(147), b"BIANCA");

    BLTCOD[148] = 709;
    fstr::assign(BLTNAM.get_mut(148), b"CRESSIDA");

    BLTCOD[149] = 710;
    fstr::assign(BLTNAM.get_mut(149), b"DESDEMONA");

    BLTCOD[150] = 711;
    fstr::assign(BLTNAM.get_mut(150), b"JULIET");

    BLTCOD[151] = 712;
    fstr::assign(BLTNAM.get_mut(151), b"PORTIA");

    BLTCOD[152] = 713;
    fstr::assign(BLTNAM.get_mut(152), b"ROSALIND");

    BLTCOD[153] = 714;
    fstr::assign(BLTNAM.get_mut(153), b"BELINDA");

    BLTCOD[154] = 715;
    fstr::assign(BLTNAM.get_mut(154), b"PUCK");

    BLTCOD[155] = 716;
    fstr::assign(BLTNAM.get_mut(155), b"CALIBAN");

    BLTCOD[156] = 717;
    fstr::assign(BLTNAM.get_mut(156), b"SYCORAX");

    BLTCOD[157] = 718;
    fstr::assign(BLTNAM.get_mut(157), b"PROSPERO");

    BLTCOD[158] = 719;
    fstr::assign(BLTNAM.get_mut(158), b"SETEBOS");

    BLTCOD[159] = 720;
    fstr::assign(BLTNAM.get_mut(159), b"STEPHANO");

    BLTCOD[160] = 721;
    fstr::assign(BLTNAM.get_mut(160), b"TRINCULO");

    BLTCOD[161] = 722;
    fstr::assign(BLTNAM.get_mut(161), b"FRANCISCO");

    BLTCOD[162] = 723;
    fstr::assign(BLTNAM.get_mut(162), b"MARGARET");

    BLTCOD[163] = 724;
    fstr::assign(BLTNAM.get_mut(163), b"FERDINAND");

    BLTCOD[164] = 725;
    fstr::assign(BLTNAM.get_mut(164), b"PERDITA");

    BLTCOD[165] = 726;
    fstr::assign(BLTNAM.get_mut(165), b"MAB");

    BLTCOD[166] = 727;
    fstr::assign(BLTNAM.get_mut(166), b"CUPID");

    BLTCOD[167] = 899;
    fstr::assign(BLTNAM.get_mut(167), b"NEPTUNE");

    BLTCOD[168] = 801;
    fstr::assign(BLTNAM.get_mut(168), b"TRITON");

    BLTCOD[169] = 802;
    fstr::assign(BLTNAM.get_mut(169), b"NEREID");

    BLTCOD[170] = 803;
    fstr::assign(BLTNAM.get_mut(170), b"NAIAD");

    BLTCOD[171] = 804;
    fstr::assign(BLTNAM.get_mut(171), b"THALASSA");

    BLTCOD[172] = 805;
    fstr::assign(BLTNAM.get_mut(172), b"DESPINA");

    BLTCOD[173] = 806;
    fstr::assign(BLTNAM.get_mut(173), b"GALATEA");

    BLTCOD[174] = 807;
    fstr::assign(BLTNAM.get_mut(174), b"LARISSA");

    BLTCOD[175] = 808;
    fstr::assign(BLTNAM.get_mut(175), b"PROTEUS");

    BLTCOD[176] = 809;
    fstr::assign(BLTNAM.get_mut(176), b"HALIMEDE");

    BLTCOD[177] = 810;
    fstr::assign(BLTNAM.get_mut(177), b"PSAMATHE");

    BLTCOD[178] = 811;
    fstr::assign(BLTNAM.get_mut(178), b"SAO");

    BLTCOD[179] = 812;
    fstr::assign(BLTNAM.get_mut(179), b"LAOMEDEIA");

    BLTCOD[180] = 813;
    fstr::assign(BLTNAM.get_mut(180), b"NESO");

    BLTCOD[181] = 999;
    fstr::assign(BLTNAM.get_mut(181), b"PLUTO");

    BLTCOD[182] = 901;
    fstr::assign(BLTNAM.get_mut(182), b"CHARON");

    BLTCOD[183] = 902;
    fstr::assign(BLTNAM.get_mut(183), b"NIX");

    BLTCOD[184] = 903;
    fstr::assign(BLTNAM.get_mut(184), b"HYDRA");

    BLTCOD[185] = 904;
    fstr::assign(BLTNAM.get_mut(185), b"KERBEROS");

    BLTCOD[186] = 905;
    fstr::assign(BLTNAM.get_mut(186), b"STYX");

    BLTCOD[187] = -1;
    fstr::assign(BLTNAM.get_mut(187), b"GEOTAIL");

    BLTCOD[188] = -3;
    fstr::assign(BLTNAM.get_mut(188), b"MOM");

    BLTCOD[189] = -3;
    fstr::assign(BLTNAM.get_mut(189), b"MARS ORBITER MISSION");

    BLTCOD[190] = -5;
    fstr::assign(BLTNAM.get_mut(190), b"AKATSUKI");

    BLTCOD[191] = -5;
    fstr::assign(BLTNAM.get_mut(191), b"VCO");

    BLTCOD[192] = -5;
    fstr::assign(BLTNAM.get_mut(192), b"PLC");

    BLTCOD[193] = -5;
    fstr::assign(BLTNAM.get_mut(193), b"PLANET-C");

    BLTCOD[194] = -6;
    fstr::assign(BLTNAM.get_mut(194), b"P6");

    BLTCOD[195] = -6;
    fstr::assign(BLTNAM.get_mut(195), b"PIONEER-6");

    BLTCOD[196] = -7;
    fstr::assign(BLTNAM.get_mut(196), b"P7");

    BLTCOD[197] = -7;
    fstr::assign(BLTNAM.get_mut(197), b"PIONEER-7");

    BLTCOD[198] = -8;
    fstr::assign(BLTNAM.get_mut(198), b"WIND");

    BLTCOD[199] = -12;
    fstr::assign(BLTNAM.get_mut(199), b"VENUS ORBITER");

    BLTCOD[200] = -12;
    fstr::assign(BLTNAM.get_mut(200), b"P12");

    BLTCOD[201] = -12;
    fstr::assign(BLTNAM.get_mut(201), b"PIONEER 12");

    BLTCOD[202] = -12;
    fstr::assign(BLTNAM.get_mut(202), b"LADEE");

    BLTCOD[203] = -13;
    fstr::assign(BLTNAM.get_mut(203), b"POLAR");

    BLTCOD[204] = -18;
    fstr::assign(BLTNAM.get_mut(204), b"MGN");

    BLTCOD[205] = -18;
    fstr::assign(BLTNAM.get_mut(205), b"MAGELLAN");

    BLTCOD[206] = -18;
    fstr::assign(BLTNAM.get_mut(206), b"LCROSS");

    BLTCOD[207] = -20;
    fstr::assign(BLTNAM.get_mut(207), b"P8");

    BLTCOD[208] = -20;
    fstr::assign(BLTNAM.get_mut(208), b"PIONEER-8");

    BLTCOD[209] = -21;
    fstr::assign(BLTNAM.get_mut(209), b"SOHO");

    BLTCOD[210] = -23;
    fstr::assign(BLTNAM.get_mut(210), b"P10");

    BLTCOD[211] = -23;
    fstr::assign(BLTNAM.get_mut(211), b"PIONEER-10");

    BLTCOD[212] = -24;
    fstr::assign(BLTNAM.get_mut(212), b"P11");

    BLTCOD[213] = -24;
    fstr::assign(BLTNAM.get_mut(213), b"PIONEER-11");

    BLTCOD[214] = -25;
    fstr::assign(BLTNAM.get_mut(214), b"LP");

    BLTCOD[215] = -25;
    fstr::assign(BLTNAM.get_mut(215), b"LUNAR PROSPECTOR");

    BLTCOD[216] = -27;
    fstr::assign(BLTNAM.get_mut(216), b"VK1");

    BLTCOD[217] = -27;
    fstr::assign(BLTNAM.get_mut(217), b"VIKING 1 ORBITER");

    BLTCOD[218] = -28;
    fstr::assign(BLTNAM.get_mut(218), b"JUPITER ICY MOONS EXPLORER");

    BLTCOD[219] = -28;
    fstr::assign(BLTNAM.get_mut(219), b"JUICE");

    BLTCOD[220] = -29;
    fstr::assign(BLTNAM.get_mut(220), b"STARDUST");

    BLTCOD[221] = -29;
    fstr::assign(BLTNAM.get_mut(221), b"SDU");

    BLTCOD[222] = -29;
    fstr::assign(BLTNAM.get_mut(222), b"NEXT");

    BLTCOD[223] = -30;
    fstr::assign(BLTNAM.get_mut(223), b"VK2");

    BLTCOD[224] = -30;
    fstr::assign(BLTNAM.get_mut(224), b"VIKING 2 ORBITER");

    BLTCOD[225] = -30;
    fstr::assign(BLTNAM.get_mut(225), b"DS-1");

    BLTCOD[226] = -31;
    fstr::assign(BLTNAM.get_mut(226), b"VG1");

    BLTCOD[227] = -31;
    fstr::assign(BLTNAM.get_mut(227), b"VOYAGER 1");

    BLTCOD[228] = -32;
    fstr::assign(BLTNAM.get_mut(228), b"VG2");

    BLTCOD[229] = -32;
    fstr::assign(BLTNAM.get_mut(229), b"VOYAGER 2");

    BLTCOD[230] = -33;
    fstr::assign(BLTNAM.get_mut(230), b"NEOS");

    BLTCOD[231] = -33;
    fstr::assign(BLTNAM.get_mut(231), b"NEO SURVEYOR");

    BLTCOD[232] = -37;
    fstr::assign(BLTNAM.get_mut(232), b"HYB2");

    BLTCOD[233] = -37;
    fstr::assign(BLTNAM.get_mut(233), b"HAYABUSA 2");

    BLTCOD[234] = -37;
    fstr::assign(BLTNAM.get_mut(234), b"HAYABUSA2");

    BLTCOD[235] = -39;
    fstr::assign(BLTNAM.get_mut(235), b"LUNAR POLAR HYDROGEN MAPPER");

    BLTCOD[236] = -39;
    fstr::assign(BLTNAM.get_mut(236), b"LUNAH-MAP");

    BLTCOD[237] = -40;
    fstr::assign(BLTNAM.get_mut(237), b"CLEMENTINE");

    BLTCOD[238] = -41;
    fstr::assign(BLTNAM.get_mut(238), b"MEX");

    BLTCOD[239] = -41;
    fstr::assign(BLTNAM.get_mut(239), b"MARS EXPRESS");

    BLTCOD[240] = -43;
    fstr::assign(BLTNAM.get_mut(240), b"IMAP");

    BLTCOD[241] = -44;
    fstr::assign(BLTNAM.get_mut(241), b"BEAGLE2");

    BLTCOD[242] = -44;
    fstr::assign(BLTNAM.get_mut(242), b"BEAGLE 2");

    BLTCOD[243] = -45;
    fstr::assign(BLTNAM.get_mut(243), b"JNSA");

    BLTCOD[244] = -45;
    fstr::assign(BLTNAM.get_mut(244), b"JANUS_A");

    BLTCOD[245] = -46;
    fstr::assign(BLTNAM.get_mut(245), b"MS-T5");

    BLTCOD[246] = -46;
    fstr::assign(BLTNAM.get_mut(246), b"SAKIGAKE");

    BLTCOD[247] = -47;
    fstr::assign(BLTNAM.get_mut(247), b"PLANET-A");

    BLTCOD[248] = -47;
    fstr::assign(BLTNAM.get_mut(248), b"SUISEI");

    BLTCOD[249] = -47;
    fstr::assign(BLTNAM.get_mut(249), b"GNS");

    BLTCOD[250] = -47;
    fstr::assign(BLTNAM.get_mut(250), b"GENESIS");

    BLTCOD[251] = -48;
    fstr::assign(BLTNAM.get_mut(251), b"HUBBLE SPACE TELESCOPE");

    BLTCOD[252] = -48;
    fstr::assign(BLTNAM.get_mut(252), b"HST");

    BLTCOD[253] = -49;
    fstr::assign(BLTNAM.get_mut(253), b"LUCY");

    BLTCOD[254] = -53;
    fstr::assign(BLTNAM.get_mut(254), b"MARS PATHFINDER");

    BLTCOD[255] = -53;
    fstr::assign(BLTNAM.get_mut(255), b"MPF");

    BLTCOD[256] = -53;
    fstr::assign(BLTNAM.get_mut(256), b"MARS ODYSSEY");

    BLTCOD[257] = -53;
    fstr::assign(BLTNAM.get_mut(257), b"MARS SURVEYOR 01 ORBITER");

    BLTCOD[258] = -55;
    fstr::assign(BLTNAM.get_mut(258), b"ULYSSES");

    BLTCOD[259] = -57;
    fstr::assign(BLTNAM.get_mut(259), b"LUNAR ICECUBE");

    BLTCOD[260] = -58;
    fstr::assign(BLTNAM.get_mut(260), b"VSOP");

    BLTCOD[261] = -58;
    fstr::assign(BLTNAM.get_mut(261), b"HALCA");

    BLTCOD[262] = -59;
    fstr::assign(BLTNAM.get_mut(262), b"RADIOASTRON");

    BLTCOD[263] = -61;
    fstr::assign(BLTNAM.get_mut(263), b"JUNO");

    BLTCOD[264] = -62;
    fstr::assign(BLTNAM.get_mut(264), b"EMM");

    BLTCOD[265] = -62;
    fstr::assign(BLTNAM.get_mut(265), b"EMIRATES MARS MISSION");

    BLTCOD[266] = -64;
    fstr::assign(BLTNAM.get_mut(266), b"ORX");

    BLTCOD[267] = -64;
    fstr::assign(BLTNAM.get_mut(267), b"OSIRIS-REX");

    BLTCOD[268] = -65;
    fstr::assign(BLTNAM.get_mut(268), b"MCOA");

    BLTCOD[269] = -65;
    fstr::assign(BLTNAM.get_mut(269), b"MARCO-A");

    BLTCOD[270] = -66;
    fstr::assign(BLTNAM.get_mut(270), b"VEGA 1");

    BLTCOD[271] = -66;
    fstr::assign(BLTNAM.get_mut(271), b"MCOB");

    BLTCOD[272] = -66;
    fstr::assign(BLTNAM.get_mut(272), b"MARCO-B");

    BLTCOD[273] = -67;
    fstr::assign(BLTNAM.get_mut(273), b"VEGA 2");

    BLTCOD[274] = -68;
    fstr::assign(BLTNAM.get_mut(274), b"MERCURY MAGNETOSPHERIC ORBITER");

    BLTCOD[275] = -68;
    fstr::assign(BLTNAM.get_mut(275), b"MMO");

    BLTCOD[276] = -68;
    fstr::assign(BLTNAM.get_mut(276), b"BEPICOLOMBO MMO");

    BLTCOD[277] = -70;
    fstr::assign(BLTNAM.get_mut(277), b"DEEP IMPACT IMPACTOR SPACECRAFT");

    BLTCOD[278] = -72;
    fstr::assign(BLTNAM.get_mut(278), b"JNSB");

    BLTCOD[279] = -72;
    fstr::assign(BLTNAM.get_mut(279), b"JANUS_B");

    BLTCOD[280] = -74;
    fstr::assign(BLTNAM.get_mut(280), b"MRO");

    BLTCOD[281] = -74;
    fstr::assign(BLTNAM.get_mut(281), b"MARS RECON ORBITER");

    BLTCOD[282] = -76;
    fstr::assign(BLTNAM.get_mut(282), b"CURIOSITY");

    BLTCOD[283] = -76;
    fstr::assign(BLTNAM.get_mut(283), b"MSL");

    BLTCOD[284] = -76;
    fstr::assign(BLTNAM.get_mut(284), b"MARS SCIENCE LABORATORY");

    BLTCOD[285] = -77;
    fstr::assign(BLTNAM.get_mut(285), b"GLL");

    BLTCOD[286] = -77;
    fstr::assign(BLTNAM.get_mut(286), b"GALILEO ORBITER");

    BLTCOD[287] = -78;
    fstr::assign(BLTNAM.get_mut(287), b"GIOTTO");

    BLTCOD[288] = -79;
    fstr::assign(BLTNAM.get_mut(288), b"SPITZER");

    BLTCOD[289] = -79;
    fstr::assign(BLTNAM.get_mut(289), b"SPACE INFRARED TELESCOPE FACILITY");

    BLTCOD[290] = -79;
    fstr::assign(BLTNAM.get_mut(290), b"SIRTF");

    BLTCOD[291] = -81;
    fstr::assign(BLTNAM.get_mut(291), b"CASSINI ITL");

    BLTCOD[292] = -82;
    fstr::assign(BLTNAM.get_mut(292), b"CAS");

    BLTCOD[293] = -82;
    fstr::assign(BLTNAM.get_mut(293), b"CASSINI");

    BLTCOD[294] = -84;
    fstr::assign(BLTNAM.get_mut(294), b"PHOENIX");

    BLTCOD[295] = -85;
    fstr::assign(BLTNAM.get_mut(295), b"LRO");

    BLTCOD[296] = -85;
    fstr::assign(BLTNAM.get_mut(296), b"LUNAR RECON ORBITER");

    BLTCOD[297] = -85;
    fstr::assign(BLTNAM.get_mut(297), b"LUNAR RECONNAISSANCE ORBITER");

    BLTCOD[298] = -86;
    fstr::assign(BLTNAM.get_mut(298), b"CH1");

    BLTCOD[299] = -86;
    fstr::assign(BLTNAM.get_mut(299), b"CHANDRAYAAN-1");

    BLTCOD[300] = -90;
    fstr::assign(BLTNAM.get_mut(300), b"CASSINI SIMULATION");

    BLTCOD[301] = -93;
    fstr::assign(BLTNAM.get_mut(301), b"NEAR EARTH ASTEROID RENDEZVOUS");

    BLTCOD[302] = -93;
    fstr::assign(BLTNAM.get_mut(302), b"NEAR");

    BLTCOD[303] = -94;
    fstr::assign(BLTNAM.get_mut(303), b"MO");

    BLTCOD[304] = -94;
    fstr::assign(BLTNAM.get_mut(304), b"MARS OBSERVER");

    BLTCOD[305] = -94;
    fstr::assign(BLTNAM.get_mut(305), b"MGS");

    BLTCOD[306] = -94;
    fstr::assign(BLTNAM.get_mut(306), b"MARS GLOBAL SURVEYOR");

    BLTCOD[307] = -95;
    fstr::assign(BLTNAM.get_mut(307), b"MGS SIMULATION");

    BLTCOD[308] = -96;
    fstr::assign(BLTNAM.get_mut(308), b"PARKER SOLAR PROBE");

    BLTCOD[309] = -96;
    fstr::assign(BLTNAM.get_mut(309), b"SPP");

    BLTCOD[310] = -96;
    fstr::assign(BLTNAM.get_mut(310), b"SOLAR PROBE PLUS");

    BLTCOD[311] = -97;
    fstr::assign(BLTNAM.get_mut(311), b"TOPEX/POSEIDON");

    BLTCOD[312] = -98;
    fstr::assign(BLTNAM.get_mut(312), b"NEW HORIZONS");

    BLTCOD[313] = -107;
    fstr::assign(BLTNAM.get_mut(313), b"TROPICAL RAINFALL MEASURING MISSION");

    BLTCOD[314] = -107;
    fstr::assign(BLTNAM.get_mut(314), b"TRMM");

    BLTCOD[315] = -112;
    fstr::assign(BLTNAM.get_mut(315), b"ICE");

    BLTCOD[316] = -116;
    fstr::assign(BLTNAM.get_mut(316), b"MARS POLAR LANDER");

    BLTCOD[317] = -116;
    fstr::assign(BLTNAM.get_mut(317), b"MPL");

    BLTCOD[318] = -117;
    fstr::assign(BLTNAM.get_mut(318), b"EDL DEMONSTRATOR MODULE");

    BLTCOD[319] = -117;
    fstr::assign(BLTNAM.get_mut(319), b"EDM");

    BLTCOD[320] = -117;
    fstr::assign(BLTNAM.get_mut(320), b"EXOMARS 2016 EDM");

    BLTCOD[321] = -119;
    fstr::assign(BLTNAM.get_mut(321), b"MARS_ORBITER_MISSION_2");

    BLTCOD[322] = -119;
    fstr::assign(BLTNAM.get_mut(322), b"MOM2");

    BLTCOD[323] = -121;
    fstr::assign(BLTNAM.get_mut(323), b"MERCURY PLANETARY ORBITER");

    BLTCOD[324] = -121;
    fstr::assign(BLTNAM.get_mut(324), b"MPO");

    BLTCOD[325] = -121;
    fstr::assign(BLTNAM.get_mut(325), b"BEPICOLOMBO MPO");

    BLTCOD[326] = -127;
    fstr::assign(BLTNAM.get_mut(326), b"MARS CLIMATE ORBITER");

    BLTCOD[327] = -127;
    fstr::assign(BLTNAM.get_mut(327), b"MCO");

    BLTCOD[328] = -130;
    fstr::assign(BLTNAM.get_mut(328), b"MUSES-C");

    BLTCOD[329] = -130;
    fstr::assign(BLTNAM.get_mut(329), b"HAYABUSA");

    BLTCOD[330] = -131;
    fstr::assign(BLTNAM.get_mut(330), b"SELENE");

    BLTCOD[331] = -131;
    fstr::assign(BLTNAM.get_mut(331), b"KAGUYA");

    BLTCOD[332] = -135;
    fstr::assign(BLTNAM.get_mut(332), b"DART");

    BLTCOD[333] = -135;
    fstr::assign(BLTNAM.get_mut(333), b"DOUBLE ASTEROID REDIRECTION TEST");

    BLTCOD[334] = -140;
    fstr::assign(BLTNAM.get_mut(334), b"EPOCH");

    BLTCOD[335] = -140;
    fstr::assign(BLTNAM.get_mut(335), b"DIXI");

    BLTCOD[336] = -140;
    fstr::assign(BLTNAM.get_mut(336), b"EPOXI");

    BLTCOD[337] = -140;
    fstr::assign(BLTNAM.get_mut(337), b"DEEP IMPACT FLYBY SPACECRAFT");

    BLTCOD[338] = -142;
    fstr::assign(BLTNAM.get_mut(338), b"TERRA");

    BLTCOD[339] = -142;
    fstr::assign(BLTNAM.get_mut(339), b"EOS-AM1");

    BLTCOD[340] = -143;
    fstr::assign(BLTNAM.get_mut(340), b"TRACE GAS ORBITER");

    BLTCOD[341] = -143;
    fstr::assign(BLTNAM.get_mut(341), b"TGO");

    BLTCOD[342] = -143;
    fstr::assign(BLTNAM.get_mut(342), b"EXOMARS 2016 TGO");

    BLTCOD[343] = -144;
    fstr::assign(BLTNAM.get_mut(343), b"SOLO");

    BLTCOD[344] = -144;
    fstr::assign(BLTNAM.get_mut(344), b"SOLAR ORBITER");

    BLTCOD[345] = -146;
    fstr::assign(BLTNAM.get_mut(345), b"LUNAR-A");

    BLTCOD[346] = -148;
    fstr::assign(BLTNAM.get_mut(346), b"DFLY");

    BLTCOD[347] = -148;
    fstr::assign(BLTNAM.get_mut(347), b"DRAGONFLY");

    BLTCOD[348] = -150;
    fstr::assign(BLTNAM.get_mut(348), b"CASSINI PROBE");

    BLTCOD[349] = -150;
    fstr::assign(BLTNAM.get_mut(349), b"HUYGENS PROBE");

    BLTCOD[350] = -150;
    fstr::assign(BLTNAM.get_mut(350), b"CASP");

    BLTCOD[351] = -151;
    fstr::assign(BLTNAM.get_mut(351), b"AXAF");

    BLTCOD[352] = -151;
    fstr::assign(BLTNAM.get_mut(352), b"CHANDRA");

    BLTCOD[353] = -152;
    fstr::assign(BLTNAM.get_mut(353), b"CH2O");

    BLTCOD[354] = -152;
    fstr::assign(BLTNAM.get_mut(354), b"CHANDRAYAAN-2 ORBITER");

    BLTCOD[355] = -153;
    fstr::assign(BLTNAM.get_mut(355), b"CH2L");

    BLTCOD[356] = -153;
    fstr::assign(BLTNAM.get_mut(356), b"CHANDRAYAAN-2 LANDER");

    BLTCOD[357] = -154;
    fstr::assign(BLTNAM.get_mut(357), b"AQUA");

    BLTCOD[358] = -155;
    fstr::assign(BLTNAM.get_mut(358), b"KPLO");

    BLTCOD[359] = -155;
    fstr::assign(BLTNAM.get_mut(359), b"KOREAN PATHFINDER LUNAR ORBITER");

    BLTCOD[360] = -156;
    fstr::assign(BLTNAM.get_mut(360), b"ADITYA");

    BLTCOD[361] = -156;
    fstr::assign(BLTNAM.get_mut(361), b"ADIT");

    BLTCOD[362] = -159;
    fstr::assign(BLTNAM.get_mut(362), b"EURC");

    BLTCOD[363] = -159;
    fstr::assign(BLTNAM.get_mut(363), b"EUROPA CLIPPER");

    BLTCOD[364] = -164;
    fstr::assign(BLTNAM.get_mut(364), b"LUNAR FLASHLIGHT");

    BLTCOD[365] = -165;
    fstr::assign(BLTNAM.get_mut(365), b"MAP");

    BLTCOD[366] = -166;
    fstr::assign(BLTNAM.get_mut(366), b"IMAGE");

    BLTCOD[367] = -168;
    fstr::assign(BLTNAM.get_mut(367), b"PERSEVERANCE");

    BLTCOD[368] = -168;
    fstr::assign(BLTNAM.get_mut(368), b"MARS 2020");

    BLTCOD[369] = -168;
    fstr::assign(BLTNAM.get_mut(369), b"MARS2020");

    BLTCOD[370] = -168;
    fstr::assign(BLTNAM.get_mut(370), b"M2020");

    BLTCOD[371] = -170;
    fstr::assign(BLTNAM.get_mut(371), b"JWST");

    BLTCOD[372] = -170;
    fstr::assign(BLTNAM.get_mut(372), b"JAMES WEBB SPACE TELESCOPE");

    BLTCOD[373] = -172;
    fstr::assign(BLTNAM.get_mut(373), b"EXM RSP SCC");

    BLTCOD[374] = -172;
    fstr::assign(BLTNAM.get_mut(374), b"EXM SPACECRAFT COMPOSITE");

    BLTCOD[375] = -172;
    fstr::assign(BLTNAM.get_mut(375), b"EXOMARS SCC");

    BLTCOD[376] = -173;
    fstr::assign(BLTNAM.get_mut(376), b"EXM RSP SP");

    BLTCOD[377] = -173;
    fstr::assign(BLTNAM.get_mut(377), b"EXM SURFACE PLATFORM");

    BLTCOD[378] = -173;
    fstr::assign(BLTNAM.get_mut(378), b"EXOMARS SP");

    BLTCOD[379] = -174;
    fstr::assign(BLTNAM.get_mut(379), b"EXM RSP RM");

    BLTCOD[380] = -174;
    fstr::assign(BLTNAM.get_mut(380), b"EXM ROVER");

    BLTCOD[381] = -174;
    fstr::assign(BLTNAM.get_mut(381), b"EXOMARS ROVER");

    BLTCOD[382] = -177;
    fstr::assign(BLTNAM.get_mut(382), b"GRAIL-A");

    BLTCOD[383] = -178;
    fstr::assign(BLTNAM.get_mut(383), b"PLANET-B");

    BLTCOD[384] = -178;
    fstr::assign(BLTNAM.get_mut(384), b"NOZOMI");

    BLTCOD[385] = -181;
    fstr::assign(BLTNAM.get_mut(385), b"GRAIL-B");

    BLTCOD[386] = -183;
    fstr::assign(BLTNAM.get_mut(386), b"CLUSTER 1");

    BLTCOD[387] = -185;
    fstr::assign(BLTNAM.get_mut(387), b"CLUSTER 2");

    BLTCOD[388] = -188;
    fstr::assign(BLTNAM.get_mut(388), b"MUSES-B");

    BLTCOD[389] = -189;
    fstr::assign(BLTNAM.get_mut(389), b"NSYT");

    BLTCOD[390] = -189;
    fstr::assign(BLTNAM.get_mut(390), b"INSIGHT");

    BLTCOD[391] = -190;
    fstr::assign(BLTNAM.get_mut(391), b"SIM");

    BLTCOD[392] = -194;
    fstr::assign(BLTNAM.get_mut(392), b"CLUSTER 3");

    BLTCOD[393] = -196;
    fstr::assign(BLTNAM.get_mut(393), b"CLUSTER 4");

    BLTCOD[394] = -197;
    fstr::assign(BLTNAM.get_mut(394), b"EXOMARS_LARA");

    BLTCOD[395] = -197;
    fstr::assign(BLTNAM.get_mut(395), b"LARA");

    BLTCOD[396] = -198;
    fstr::assign(BLTNAM.get_mut(396), b"INTEGRAL");

    BLTCOD[397] = -198;
    fstr::assign(BLTNAM.get_mut(397), b"NASA-ISRO SAR MISSION");

    BLTCOD[398] = -198;
    fstr::assign(BLTNAM.get_mut(398), b"NISAR");

    BLTCOD[399] = -200;
    fstr::assign(BLTNAM.get_mut(399), b"CONTOUR");

    BLTCOD[400] = -202;
    fstr::assign(BLTNAM.get_mut(400), b"MAVEN");

    BLTCOD[401] = -203;
    fstr::assign(BLTNAM.get_mut(401), b"DAWN");

    BLTCOD[402] = -205;
    fstr::assign(BLTNAM.get_mut(402), b"SOIL MOISTURE ACTIVE AND PASSIVE");

    BLTCOD[403] = -205;
    fstr::assign(BLTNAM.get_mut(403), b"SMAP");

    BLTCOD[404] = -210;
    fstr::assign(BLTNAM.get_mut(404), b"LICIA");

    BLTCOD[405] = -210;
    fstr::assign(BLTNAM.get_mut(405), b"LICIACUBE");

    BLTCOD[406] = -212;
    fstr::assign(BLTNAM.get_mut(406), b"STV51");

    BLTCOD[407] = -213;
    fstr::assign(BLTNAM.get_mut(407), b"STV52");

    BLTCOD[408] = -214;
    fstr::assign(BLTNAM.get_mut(408), b"STV53");

    BLTCOD[409] = -226;
    fstr::assign(BLTNAM.get_mut(409), b"ROSETTA");

    BLTCOD[410] = -227;
    fstr::assign(BLTNAM.get_mut(410), b"KEPLER");

    BLTCOD[411] = -228;
    fstr::assign(BLTNAM.get_mut(411), b"GLL PROBE");

    BLTCOD[412] = -228;
    fstr::assign(BLTNAM.get_mut(412), b"GALILEO PROBE");

    BLTCOD[413] = -234;
    fstr::assign(BLTNAM.get_mut(413), b"STEREO AHEAD");

    BLTCOD[414] = -235;
    fstr::assign(BLTNAM.get_mut(414), b"STEREO BEHIND");

    BLTCOD[415] = -236;
    fstr::assign(BLTNAM.get_mut(415), b"MESSENGER");

    BLTCOD[416] = -238;
    fstr::assign(BLTNAM.get_mut(416), b"SMART1");

    BLTCOD[417] = -238;
    fstr::assign(BLTNAM.get_mut(417), b"SM1");

    BLTCOD[418] = -238;
    fstr::assign(BLTNAM.get_mut(418), b"S1");

    BLTCOD[419] = -238;
    fstr::assign(BLTNAM.get_mut(419), b"SMART-1");

    BLTCOD[420] = -239;
    fstr::assign(BLTNAM.get_mut(420), b"MARTIAN MOONS EXPLORATION");

    BLTCOD[421] = -239;
    fstr::assign(BLTNAM.get_mut(421), b"MMX");

    BLTCOD[422] = -240;
    fstr::assign(BLTNAM.get_mut(422), b"SMART LANDER FOR INVESTIGATING MOON");

    BLTCOD[423] = -240;
    fstr::assign(BLTNAM.get_mut(423), b"SLIM");

    BLTCOD[424] = -242;
    fstr::assign(BLTNAM.get_mut(424), b"LUNAR TRAILBLAZER");

    BLTCOD[425] = -243;
    fstr::assign(BLTNAM.get_mut(425), b"VIPER");

    BLTCOD[426] = -248;
    fstr::assign(BLTNAM.get_mut(426), b"VEX");

    BLTCOD[427] = -248;
    fstr::assign(BLTNAM.get_mut(427), b"VENUS EXPRESS");

    BLTCOD[428] = -253;
    fstr::assign(BLTNAM.get_mut(428), b"OPPORTUNITY");

    BLTCOD[429] = -253;
    fstr::assign(BLTNAM.get_mut(429), b"MER-1");

    BLTCOD[430] = -254;
    fstr::assign(BLTNAM.get_mut(430), b"SPIRIT");

    BLTCOD[431] = -254;
    fstr::assign(BLTNAM.get_mut(431), b"MER-2");

    BLTCOD[432] = -255;
    fstr::assign(BLTNAM.get_mut(432), b"PSYC");

    BLTCOD[433] = -301;
    fstr::assign(BLTNAM.get_mut(433), b"HELIOS 1");

    BLTCOD[434] = -302;
    fstr::assign(BLTNAM.get_mut(434), b"HELIOS 2");

    BLTCOD[435] = -362;
    fstr::assign(BLTNAM.get_mut(435), b"RADIATION BELT STORM PROBE A");

    BLTCOD[436] = -362;
    fstr::assign(BLTNAM.get_mut(436), b"RBSP_A");

    BLTCOD[437] = -363;
    fstr::assign(BLTNAM.get_mut(437), b"RADIATION BELT STORM PROBE B");

    BLTCOD[438] = -363;
    fstr::assign(BLTNAM.get_mut(438), b"RBSP_B");

    BLTCOD[439] = -500;
    fstr::assign(BLTNAM.get_mut(439), b"RSAT");

    BLTCOD[440] = -500;
    fstr::assign(BLTNAM.get_mut(440), b"SELENE Relay Satellite");

    BLTCOD[441] = -500;
    fstr::assign(BLTNAM.get_mut(441), b"SELENE Rstar");

    BLTCOD[442] = -500;
    fstr::assign(BLTNAM.get_mut(442), b"Rstar");

    BLTCOD[443] = -502;
    fstr::assign(BLTNAM.get_mut(443), b"VSAT");

    BLTCOD[444] = -502;
    fstr::assign(BLTNAM.get_mut(444), b"SELENE VLBI Radio Satellite");

    BLTCOD[445] = -502;
    fstr::assign(BLTNAM.get_mut(445), b"SELENE VRAD Satellite");

    BLTCOD[446] = -502;
    fstr::assign(BLTNAM.get_mut(446), b"SELENE Vstar");

    BLTCOD[447] = -502;
    fstr::assign(BLTNAM.get_mut(447), b"Vstar");

    BLTCOD[448] = -550;
    fstr::assign(BLTNAM.get_mut(448), b"MARS-96");

    BLTCOD[449] = -550;
    fstr::assign(BLTNAM.get_mut(449), b"M96");

    BLTCOD[450] = -550;
    fstr::assign(BLTNAM.get_mut(450), b"MARS 96");

    BLTCOD[451] = -550;
    fstr::assign(BLTNAM.get_mut(451), b"MARS96");

    BLTCOD[452] = -652;
    fstr::assign(BLTNAM.get_mut(452), b"MERCURY TRANSFER MODULE");

    BLTCOD[453] = -652;
    fstr::assign(BLTNAM.get_mut(453), b"MTM");

    BLTCOD[454] = -652;
    fstr::assign(BLTNAM.get_mut(454), b"BEPICOLOMBO MTM");

    BLTCOD[455] = -750;
    fstr::assign(BLTNAM.get_mut(455), b"SPRINT-A");

    BLTCOD[456] = 50000001;
    fstr::assign(BLTNAM.get_mut(456), b"SHOEMAKER-LEVY 9-W");

    BLTCOD[457] = 50000002;
    fstr::assign(BLTNAM.get_mut(457), b"SHOEMAKER-LEVY 9-V");

    BLTCOD[458] = 50000003;
    fstr::assign(BLTNAM.get_mut(458), b"SHOEMAKER-LEVY 9-U");

    BLTCOD[459] = 50000004;
    fstr::assign(BLTNAM.get_mut(459), b"SHOEMAKER-LEVY 9-T");

    BLTCOD[460] = 50000005;
    fstr::assign(BLTNAM.get_mut(460), b"SHOEMAKER-LEVY 9-S");

    BLTCOD[461] = 50000006;
    fstr::assign(BLTNAM.get_mut(461), b"SHOEMAKER-LEVY 9-R");

    BLTCOD[462] = 50000007;
    fstr::assign(BLTNAM.get_mut(462), b"SHOEMAKER-LEVY 9-Q");

    BLTCOD[463] = 50000008;
    fstr::assign(BLTNAM.get_mut(463), b"SHOEMAKER-LEVY 9-P");

    BLTCOD[464] = 50000009;
    fstr::assign(BLTNAM.get_mut(464), b"SHOEMAKER-LEVY 9-N");

    BLTCOD[465] = 50000010;
    fstr::assign(BLTNAM.get_mut(465), b"SHOEMAKER-LEVY 9-M");

    BLTCOD[466] = 50000011;
    fstr::assign(BLTNAM.get_mut(466), b"SHOEMAKER-LEVY 9-L");

    BLTCOD[467] = 50000012;
    fstr::assign(BLTNAM.get_mut(467), b"SHOEMAKER-LEVY 9-K");

    BLTCOD[468] = 50000013;
    fstr::assign(BLTNAM.get_mut(468), b"SHOEMAKER-LEVY 9-J");

    BLTCOD[469] = 50000014;
    fstr::assign(BLTNAM.get_mut(469), b"SHOEMAKER-LEVY 9-H");

    BLTCOD[470] = 50000015;
    fstr::assign(BLTNAM.get_mut(470), b"SHOEMAKER-LEVY 9-G");

    BLTCOD[471] = 50000016;
    fstr::assign(BLTNAM.get_mut(471), b"SHOEMAKER-LEVY 9-F");

    BLTCOD[472] = 50000017;
    fstr::assign(BLTNAM.get_mut(472), b"SHOEMAKER-LEVY 9-E");

    BLTCOD[473] = 50000018;
    fstr::assign(BLTNAM.get_mut(473), b"SHOEMAKER-LEVY 9-D");

    BLTCOD[474] = 50000019;
    fstr::assign(BLTNAM.get_mut(474), b"SHOEMAKER-LEVY 9-C");

    BLTCOD[475] = 50000020;
    fstr::assign(BLTNAM.get_mut(475), b"SHOEMAKER-LEVY 9-B");

    BLTCOD[476] = 50000021;
    fstr::assign(BLTNAM.get_mut(476), b"SHOEMAKER-LEVY 9-A");

    BLTCOD[477] = 50000022;
    fstr::assign(BLTNAM.get_mut(477), b"SHOEMAKER-LEVY 9-Q1");

    BLTCOD[478] = 50000023;
    fstr::assign(BLTNAM.get_mut(478), b"SHOEMAKER-LEVY 9-P2");

    BLTCOD[479] = 1000001;
    fstr::assign(BLTNAM.get_mut(479), b"AREND");

    BLTCOD[480] = 1000002;
    fstr::assign(BLTNAM.get_mut(480), b"AREND-RIGAUX");

    BLTCOD[481] = 1000003;
    fstr::assign(BLTNAM.get_mut(481), b"ASHBROOK-JACKSON");

    BLTCOD[482] = 1000004;
    fstr::assign(BLTNAM.get_mut(482), b"BOETHIN");

    BLTCOD[483] = 1000005;
    fstr::assign(BLTNAM.get_mut(483), b"BORRELLY");

    BLTCOD[484] = 1000006;
    fstr::assign(BLTNAM.get_mut(484), b"BOWELL-SKIFF");

    BLTCOD[485] = 1000007;
    fstr::assign(BLTNAM.get_mut(485), b"BRADFIELD");

    BLTCOD[486] = 1000008;
    fstr::assign(BLTNAM.get_mut(486), b"BROOKS 2");

    BLTCOD[487] = 1000009;
    fstr::assign(BLTNAM.get_mut(487), b"BRORSEN-METCALF");

    BLTCOD[488] = 1000010;
    fstr::assign(BLTNAM.get_mut(488), b"BUS");

    BLTCOD[489] = 1000011;
    fstr::assign(BLTNAM.get_mut(489), b"CHERNYKH");

    BLTCOD[490] = 1000012;
    fstr::assign(BLTNAM.get_mut(490), b"67P/CHURYUMOV-GERASIMENKO (1969 R1)");

    BLTCOD[491] = 1000012;
    fstr::assign(BLTNAM.get_mut(491), b"CHURYUMOV-GERASIMENKO");

    BLTCOD[492] = 1000013;
    fstr::assign(BLTNAM.get_mut(492), b"CIFFREO");

    BLTCOD[493] = 1000014;
    fstr::assign(BLTNAM.get_mut(493), b"CLARK");

    BLTCOD[494] = 1000015;
    fstr::assign(BLTNAM.get_mut(494), b"COMAS SOLA");

    BLTCOD[495] = 1000016;
    fstr::assign(BLTNAM.get_mut(495), b"CROMMELIN");

    BLTCOD[496] = 1000017;
    fstr::assign(BLTNAM.get_mut(496), b"D\'ARREST");

    BLTCOD[497] = 1000018;
    fstr::assign(BLTNAM.get_mut(497), b"DANIEL");

    BLTCOD[498] = 1000019;
    fstr::assign(BLTNAM.get_mut(498), b"DE VICO-SWIFT");

    BLTCOD[499] = 1000020;
    fstr::assign(BLTNAM.get_mut(499), b"DENNING-FUJIKAWA");

    BLTCOD[500] = 1000021;
    fstr::assign(BLTNAM.get_mut(500), b"DU TOIT 1");

    BLTCOD[501] = 1000022;
    fstr::assign(BLTNAM.get_mut(501), b"DU TOIT-HARTLEY");

    BLTCOD[502] = 1000023;
    fstr::assign(BLTNAM.get_mut(502), b"DUTOIT-NEUJMIN-DELPORTE");

    BLTCOD[503] = 1000024;
    fstr::assign(BLTNAM.get_mut(503), b"DUBIAGO");

    BLTCOD[504] = 1000025;
    fstr::assign(BLTNAM.get_mut(504), b"ENCKE");

    BLTCOD[505] = 1000026;
    fstr::assign(BLTNAM.get_mut(505), b"FAYE");

    BLTCOD[506] = 1000027;
    fstr::assign(BLTNAM.get_mut(506), b"FINLAY");

    BLTCOD[507] = 1000028;
    fstr::assign(BLTNAM.get_mut(507), b"FORBES");

    BLTCOD[508] = 1000029;
    fstr::assign(BLTNAM.get_mut(508), b"GEHRELS 1");

    BLTCOD[509] = 1000030;
    fstr::assign(BLTNAM.get_mut(509), b"GEHRELS 2");

    BLTCOD[510] = 1000031;
    fstr::assign(BLTNAM.get_mut(510), b"GEHRELS 3");

    BLTCOD[511] = 1000032;
    fstr::assign(BLTNAM.get_mut(511), b"GIACOBINI-ZINNER");

    BLTCOD[512] = 1000033;
    fstr::assign(BLTNAM.get_mut(512), b"GICLAS");

    BLTCOD[513] = 1000034;
    fstr::assign(BLTNAM.get_mut(513), b"GRIGG-SKJELLERUP");

    BLTCOD[514] = 1000035;
    fstr::assign(BLTNAM.get_mut(514), b"GUNN");

    BLTCOD[515] = 1000036;
    fstr::assign(BLTNAM.get_mut(515), b"HALLEY");

    BLTCOD[516] = 1000037;
    fstr::assign(BLTNAM.get_mut(516), b"HANEDA-CAMPOS");

    BLTCOD[517] = 1000038;
    fstr::assign(BLTNAM.get_mut(517), b"HARRINGTON");

    BLTCOD[518] = 1000039;
    fstr::assign(BLTNAM.get_mut(518), b"HARRINGTON-ABELL");

    BLTCOD[519] = 1000040;
    fstr::assign(BLTNAM.get_mut(519), b"HARTLEY 1");

    BLTCOD[520] = 1000041;
    fstr::assign(BLTNAM.get_mut(520), b"HARTLEY 2");

    BLTCOD[521] = 1000042;
    fstr::assign(BLTNAM.get_mut(521), b"HARTLEY-IRAS");

    BLTCOD[522] = 1000043;
    fstr::assign(BLTNAM.get_mut(522), b"HERSCHEL-RIGOLLET");

    BLTCOD[523] = 1000044;
    fstr::assign(BLTNAM.get_mut(523), b"HOLMES");

    BLTCOD[524] = 1000045;
    fstr::assign(BLTNAM.get_mut(524), b"HONDA-MRKOS-PAJDUSAKOVA");

    BLTCOD[525] = 1000046;
    fstr::assign(BLTNAM.get_mut(525), b"HOWELL");

    BLTCOD[526] = 1000047;
    fstr::assign(BLTNAM.get_mut(526), b"IRAS");

    BLTCOD[527] = 1000048;
    fstr::assign(BLTNAM.get_mut(527), b"JACKSON-NEUJMIN");

    BLTCOD[528] = 1000049;
    fstr::assign(BLTNAM.get_mut(528), b"JOHNSON");

    BLTCOD[529] = 1000050;
    fstr::assign(BLTNAM.get_mut(529), b"KEARNS-KWEE");

    BLTCOD[530] = 1000051;
    fstr::assign(BLTNAM.get_mut(530), b"KLEMOLA");

    BLTCOD[531] = 1000052;
    fstr::assign(BLTNAM.get_mut(531), b"KOHOUTEK");

    BLTCOD[532] = 1000053;
    fstr::assign(BLTNAM.get_mut(532), b"KOJIMA");

    BLTCOD[533] = 1000054;
    fstr::assign(BLTNAM.get_mut(533), b"KOPFF");

    BLTCOD[534] = 1000055;
    fstr::assign(BLTNAM.get_mut(534), b"KOWAL 1");

    BLTCOD[535] = 1000056;
    fstr::assign(BLTNAM.get_mut(535), b"KOWAL 2");

    BLTCOD[536] = 1000057;
    fstr::assign(BLTNAM.get_mut(536), b"KOWAL-MRKOS");

    BLTCOD[537] = 1000058;
    fstr::assign(BLTNAM.get_mut(537), b"KOWAL-VAVROVA");

    BLTCOD[538] = 1000059;
    fstr::assign(BLTNAM.get_mut(538), b"LONGMORE");

    BLTCOD[539] = 1000060;
    fstr::assign(BLTNAM.get_mut(539), b"LOVAS 1");

    BLTCOD[540] = 1000061;
    fstr::assign(BLTNAM.get_mut(540), b"MACHHOLZ");

    BLTCOD[541] = 1000062;
    fstr::assign(BLTNAM.get_mut(541), b"MAURY");

    BLTCOD[542] = 1000063;
    fstr::assign(BLTNAM.get_mut(542), b"NEUJMIN 1");

    BLTCOD[543] = 1000064;
    fstr::assign(BLTNAM.get_mut(543), b"NEUJMIN 2");

    BLTCOD[544] = 1000065;
    fstr::assign(BLTNAM.get_mut(544), b"NEUJMIN 3");

    BLTCOD[545] = 1000066;
    fstr::assign(BLTNAM.get_mut(545), b"OLBERS");

    BLTCOD[546] = 1000067;
    fstr::assign(BLTNAM.get_mut(546), b"PETERS-HARTLEY");

    BLTCOD[547] = 1000068;
    fstr::assign(BLTNAM.get_mut(547), b"PONS-BROOKS");

    BLTCOD[548] = 1000069;
    fstr::assign(BLTNAM.get_mut(548), b"PONS-WINNECKE");

    BLTCOD[549] = 1000070;
    fstr::assign(BLTNAM.get_mut(549), b"REINMUTH 1");

    BLTCOD[550] = 1000071;
    fstr::assign(BLTNAM.get_mut(550), b"REINMUTH 2");

    BLTCOD[551] = 1000072;
    fstr::assign(BLTNAM.get_mut(551), b"RUSSELL 1");

    BLTCOD[552] = 1000073;
    fstr::assign(BLTNAM.get_mut(552), b"RUSSELL 2");

    BLTCOD[553] = 1000074;
    fstr::assign(BLTNAM.get_mut(553), b"RUSSELL 3");

    BLTCOD[554] = 1000075;
    fstr::assign(BLTNAM.get_mut(554), b"RUSSELL 4");

    BLTCOD[555] = 1000076;
    fstr::assign(BLTNAM.get_mut(555), b"SANGUIN");

    BLTCOD[556] = 1000077;
    fstr::assign(BLTNAM.get_mut(556), b"SCHAUMASSE");

    BLTCOD[557] = 1000078;
    fstr::assign(BLTNAM.get_mut(557), b"SCHUSTER");

    BLTCOD[558] = 1000079;
    fstr::assign(BLTNAM.get_mut(558), b"SCHWASSMANN-WACHMANN 1");

    BLTCOD[559] = 1000080;
    fstr::assign(BLTNAM.get_mut(559), b"SCHWASSMANN-WACHMANN 2");

    BLTCOD[560] = 1000081;
    fstr::assign(BLTNAM.get_mut(560), b"SCHWASSMANN-WACHMANN 3");

    BLTCOD[561] = 1000082;
    fstr::assign(BLTNAM.get_mut(561), b"SHAJN-SCHALDACH");

    BLTCOD[562] = 1000083;
    fstr::assign(BLTNAM.get_mut(562), b"SHOEMAKER 1");

    BLTCOD[563] = 1000084;
    fstr::assign(BLTNAM.get_mut(563), b"SHOEMAKER 2");

    BLTCOD[564] = 1000085;
    fstr::assign(BLTNAM.get_mut(564), b"SHOEMAKER 3");

    BLTCOD[565] = 1000086;
    fstr::assign(BLTNAM.get_mut(565), b"SINGER-BREWSTER");

    BLTCOD[566] = 1000087;
    fstr::assign(BLTNAM.get_mut(566), b"SLAUGHTER-BURNHAM");

    BLTCOD[567] = 1000088;
    fstr::assign(BLTNAM.get_mut(567), b"SMIRNOVA-CHERNYKH");

    BLTCOD[568] = 1000089;
    fstr::assign(BLTNAM.get_mut(568), b"STEPHAN-OTERMA");

    BLTCOD[569] = 1000090;
    fstr::assign(BLTNAM.get_mut(569), b"SWIFT-GEHRELS");

    BLTCOD[570] = 1000091;
    fstr::assign(BLTNAM.get_mut(570), b"TAKAMIZAWA");

    BLTCOD[571] = 1000092;
    fstr::assign(BLTNAM.get_mut(571), b"TAYLOR");

    BLTCOD[572] = 1000093;
    fstr::assign(BLTNAM.get_mut(572), b"TEMPEL_1");

    BLTCOD[573] = 1000093;
    fstr::assign(BLTNAM.get_mut(573), b"TEMPEL 1");

    BLTCOD[574] = 1000094;
    fstr::assign(BLTNAM.get_mut(574), b"TEMPEL 2");

    BLTCOD[575] = 1000095;
    fstr::assign(BLTNAM.get_mut(575), b"TEMPEL-TUTTLE");

    BLTCOD[576] = 1000096;
    fstr::assign(BLTNAM.get_mut(576), b"TRITTON");

    BLTCOD[577] = 1000097;
    fstr::assign(BLTNAM.get_mut(577), b"TSUCHINSHAN 1");

    BLTCOD[578] = 1000098;
    fstr::assign(BLTNAM.get_mut(578), b"TSUCHINSHAN 2");

    BLTCOD[579] = 1000099;
    fstr::assign(BLTNAM.get_mut(579), b"TUTTLE");

    BLTCOD[580] = 1000100;
    fstr::assign(BLTNAM.get_mut(580), b"TUTTLE-GIACOBINI-KRESAK");

    BLTCOD[581] = 1000101;
    fstr::assign(BLTNAM.get_mut(581), b"VAISALA 1");

    BLTCOD[582] = 1000102;
    fstr::assign(BLTNAM.get_mut(582), b"VAN BIESBROECK");

    BLTCOD[583] = 1000103;
    fstr::assign(BLTNAM.get_mut(583), b"VAN HOUTEN");

    BLTCOD[584] = 1000104;
    fstr::assign(BLTNAM.get_mut(584), b"WEST-KOHOUTEK-IKEMURA");

    BLTCOD[585] = 1000105;
    fstr::assign(BLTNAM.get_mut(585), b"WHIPPLE");

    BLTCOD[586] = 1000106;
    fstr::assign(BLTNAM.get_mut(586), b"WILD 1");

    BLTCOD[587] = 1000107;
    fstr::assign(BLTNAM.get_mut(587), b"WILD 2");

    BLTCOD[588] = 1000108;
    fstr::assign(BLTNAM.get_mut(588), b"WILD 3");

    BLTCOD[589] = 1000109;
    fstr::assign(BLTNAM.get_mut(589), b"WIRTANEN");

    BLTCOD[590] = 1000110;
    fstr::assign(BLTNAM.get_mut(590), b"WOLF");

    BLTCOD[591] = 1000111;
    fstr::assign(BLTNAM.get_mut(591), b"WOLF-HARRINGTON");

    BLTCOD[592] = 1000112;
    fstr::assign(BLTNAM.get_mut(592), b"LOVAS 2");

    BLTCOD[593] = 1000113;
    fstr::assign(BLTNAM.get_mut(593), b"URATA-NIIJIMA");

    BLTCOD[594] = 1000114;
    fstr::assign(BLTNAM.get_mut(594), b"WISEMAN-SKIFF");

    BLTCOD[595] = 1000115;
    fstr::assign(BLTNAM.get_mut(595), b"HELIN");

    BLTCOD[596] = 1000116;
    fstr::assign(BLTNAM.get_mut(596), b"MUELLER");

    BLTCOD[597] = 1000117;
    fstr::assign(BLTNAM.get_mut(597), b"SHOEMAKER-HOLT 1");

    BLTCOD[598] = 1000118;
    fstr::assign(BLTNAM.get_mut(598), b"HELIN-ROMAN-CROCKETT");

    BLTCOD[599] = 1000119;
    fstr::assign(BLTNAM.get_mut(599), b"HARTLEY 3");

    BLTCOD[600] = 1000120;
    fstr::assign(BLTNAM.get_mut(600), b"PARKER-HARTLEY");

    BLTCOD[601] = 1000121;
    fstr::assign(BLTNAM.get_mut(601), b"HELIN-ROMAN-ALU 1");

    BLTCOD[602] = 1000122;
    fstr::assign(BLTNAM.get_mut(602), b"WILD 4");

    BLTCOD[603] = 1000123;
    fstr::assign(BLTNAM.get_mut(603), b"MUELLER 2");

    BLTCOD[604] = 1000124;
    fstr::assign(BLTNAM.get_mut(604), b"MUELLER 3");

    BLTCOD[605] = 1000125;
    fstr::assign(BLTNAM.get_mut(605), b"SHOEMAKER-LEVY 1");

    BLTCOD[606] = 1000126;
    fstr::assign(BLTNAM.get_mut(606), b"SHOEMAKER-LEVY 2");

    BLTCOD[607] = 1000127;
    fstr::assign(BLTNAM.get_mut(607), b"HOLT-OLMSTEAD");

    BLTCOD[608] = 1000128;
    fstr::assign(BLTNAM.get_mut(608), b"METCALF-BREWINGTON");

    BLTCOD[609] = 1000129;
    fstr::assign(BLTNAM.get_mut(609), b"LEVY");

    BLTCOD[610] = 1000130;
    fstr::assign(BLTNAM.get_mut(610), b"SHOEMAKER-LEVY 9");

    BLTCOD[611] = 1000131;
    fstr::assign(BLTNAM.get_mut(611), b"HYAKUTAKE");

    BLTCOD[612] = 1000132;
    fstr::assign(BLTNAM.get_mut(612), b"HALE-BOPP");

    BLTCOD[613] = 1003228;
    fstr::assign(BLTNAM.get_mut(613), b"C/2013 A1");

    BLTCOD[614] = 1003228;
    fstr::assign(BLTNAM.get_mut(614), b"SIDING SPRING");

    BLTCOD[615] = 2000001;
    fstr::assign(BLTNAM.get_mut(615), b"CERES");

    BLTCOD[616] = 2000002;
    fstr::assign(BLTNAM.get_mut(616), b"PALLAS");

    BLTCOD[617] = 2000004;
    fstr::assign(BLTNAM.get_mut(617), b"VESTA");

    BLTCOD[618] = 2000016;
    fstr::assign(BLTNAM.get_mut(618), b"PSYCHE");

    BLTCOD[619] = 2000021;
    fstr::assign(BLTNAM.get_mut(619), b"LUTETIA");

    BLTCOD[620] = 2000052;
    fstr::assign(BLTNAM.get_mut(620), b"52_EUROPA");

    BLTCOD[621] = 2000052;
    fstr::assign(BLTNAM.get_mut(621), b"52 EUROPA");

    BLTCOD[622] = 2000216;
    fstr::assign(BLTNAM.get_mut(622), b"KLEOPATRA");

    BLTCOD[623] = 2000253;
    fstr::assign(BLTNAM.get_mut(623), b"MATHILDE");

    BLTCOD[624] = 2000433;
    fstr::assign(BLTNAM.get_mut(624), b"EROS");

    BLTCOD[625] = 2000511;
    fstr::assign(BLTNAM.get_mut(625), b"DAVIDA");

    BLTCOD[626] = 2002867;
    fstr::assign(BLTNAM.get_mut(626), b"STEINS");

    BLTCOD[627] = 2004015;
    fstr::assign(BLTNAM.get_mut(627), b"WILSON-HARRINGTON");

    BLTCOD[628] = 2004179;
    fstr::assign(BLTNAM.get_mut(628), b"TOUTATIS");

    BLTCOD[629] = 2009969;
    fstr::assign(BLTNAM.get_mut(629), b"1992KD");

    BLTCOD[630] = 2009969;
    fstr::assign(BLTNAM.get_mut(630), b"BRAILLE");

    BLTCOD[631] = 2025143;
    fstr::assign(BLTNAM.get_mut(631), b"ITOKAWA");

    BLTCOD[632] = 2101955;
    fstr::assign(BLTNAM.get_mut(632), b"BENNU");

    BLTCOD[633] = 2162173;
    fstr::assign(BLTNAM.get_mut(633), b"RYUGU");

    BLTCOD[634] = 2431010;
    fstr::assign(BLTNAM.get_mut(634), b"IDA");

    BLTCOD[635] = 2431011;
    fstr::assign(BLTNAM.get_mut(635), b"DACTYL");

    BLTCOD[636] = 2486958;
    fstr::assign(BLTNAM.get_mut(636), b"ARROKOTH");

    BLTCOD[637] = 9511010;
    fstr::assign(BLTNAM.get_mut(637), b"GASPRA");

    BLTCOD[638] = 20000617;
    fstr::assign(BLTNAM.get_mut(638), b"PATROCLUS_BARYCENTER");

    BLTCOD[639] = 20000617;
    fstr::assign(BLTNAM.get_mut(639), b"PATROCLUS BARYCENTER");

    BLTCOD[640] = 20003548;
    fstr::assign(BLTNAM.get_mut(640), b"EURYBATES_BARYCENTER");

    BLTCOD[641] = 20003548;
    fstr::assign(BLTNAM.get_mut(641), b"EURYBATES BARYCENTER");

    BLTCOD[642] = 20011351;
    fstr::assign(BLTNAM.get_mut(642), b"LEUCUS");

    BLTCOD[643] = 20015094;
    fstr::assign(BLTNAM.get_mut(643), b"POLYMELE");

    BLTCOD[644] = 20021900;
    fstr::assign(BLTNAM.get_mut(644), b"ORUS");

    BLTCOD[645] = 20052246;
    fstr::assign(BLTNAM.get_mut(645), b"DONALDJOHANSON");

    BLTCOD[646] = 20065803;
    fstr::assign(BLTNAM.get_mut(646), b"DIDYMOS_BARYCENTER");

    BLTCOD[647] = 20065803;
    fstr::assign(BLTNAM.get_mut(647), b"DIDYMOS BARYCENTER");

    BLTCOD[648] = 120000617;
    fstr::assign(BLTNAM.get_mut(648), b"MENOETIUS");

    BLTCOD[649] = 120003548;
    fstr::assign(BLTNAM.get_mut(649), b"QUETA");

    BLTCOD[650] = 120065803;
    fstr::assign(BLTNAM.get_mut(650), b"DIMORPHOS");

    BLTCOD[651] = 920000617;
    fstr::assign(BLTNAM.get_mut(651), b"PATROCLUS");

    BLTCOD[652] = 920003548;
    fstr::assign(BLTNAM.get_mut(652), b"EURYBATES");

    BLTCOD[653] = 920065803;
    fstr::assign(BLTNAM.get_mut(653), b"DIDYMOS");

    BLTCOD[654] = 398989;
    fstr::assign(BLTNAM.get_mut(654), b"NOTO");

    BLTCOD[655] = 398990;
    fstr::assign(BLTNAM.get_mut(655), b"NEW NORCIA");

    BLTCOD[656] = 399001;
    fstr::assign(BLTNAM.get_mut(656), b"GOLDSTONE");

    BLTCOD[657] = 399002;
    fstr::assign(BLTNAM.get_mut(657), b"CANBERRA");

    BLTCOD[658] = 399003;
    fstr::assign(BLTNAM.get_mut(658), b"MADRID");

    BLTCOD[659] = 399004;
    fstr::assign(BLTNAM.get_mut(659), b"USUDA");

    BLTCOD[660] = 399005;
    fstr::assign(BLTNAM.get_mut(660), b"DSS-05");

    BLTCOD[661] = 399005;
    fstr::assign(BLTNAM.get_mut(661), b"PARKES");

    BLTCOD[662] = 399012;
    fstr::assign(BLTNAM.get_mut(662), b"DSS-12");

    BLTCOD[663] = 399013;
    fstr::assign(BLTNAM.get_mut(663), b"DSS-13");

    BLTCOD[664] = 399014;
    fstr::assign(BLTNAM.get_mut(664), b"DSS-14");

    BLTCOD[665] = 399015;
    fstr::assign(BLTNAM.get_mut(665), b"DSS-15");

    BLTCOD[666] = 399016;
    fstr::assign(BLTNAM.get_mut(666), b"DSS-16");

    BLTCOD[667] = 399017;
    fstr::assign(BLTNAM.get_mut(667), b"DSS-17");

    BLTCOD[668] = 399023;
    fstr::assign(BLTNAM.get_mut(668), b"DSS-23");

    BLTCOD[669] = 399024;
    fstr::assign(BLTNAM.get_mut(669), b"DSS-24");

    BLTCOD[670] = 399025;
    fstr::assign(BLTNAM.get_mut(670), b"DSS-25");

    BLTCOD[671] = 399026;
    fstr::assign(BLTNAM.get_mut(671), b"DSS-26");

    BLTCOD[672] = 399027;
    fstr::assign(BLTNAM.get_mut(672), b"DSS-27");

    BLTCOD[673] = 399028;
    fstr::assign(BLTNAM.get_mut(673), b"DSS-28");

    BLTCOD[674] = 399033;
    fstr::assign(BLTNAM.get_mut(674), b"DSS-33");

    BLTCOD[675] = 399034;
    fstr::assign(BLTNAM.get_mut(675), b"DSS-34");

    BLTCOD[676] = 399035;
    fstr::assign(BLTNAM.get_mut(676), b"DSS-35");

    BLTCOD[677] = 399036;
    fstr::assign(BLTNAM.get_mut(677), b"DSS-36");

    BLTCOD[678] = 399042;
    fstr::assign(BLTNAM.get_mut(678), b"DSS-42");

    BLTCOD[679] = 399043;
    fstr::assign(BLTNAM.get_mut(679), b"DSS-43");

    BLTCOD[680] = 399045;
    fstr::assign(BLTNAM.get_mut(680), b"DSS-45");

    BLTCOD[681] = 399046;
    fstr::assign(BLTNAM.get_mut(681), b"DSS-46");

    BLTCOD[682] = 399049;
    fstr::assign(BLTNAM.get_mut(682), b"DSS-49");

    BLTCOD[683] = 399053;
    fstr::assign(BLTNAM.get_mut(683), b"DSS-53");

    BLTCOD[684] = 399054;
    fstr::assign(BLTNAM.get_mut(684), b"DSS-54");

    BLTCOD[685] = 399055;
    fstr::assign(BLTNAM.get_mut(685), b"DSS-55");

    BLTCOD[686] = 399056;
    fstr::assign(BLTNAM.get_mut(686), b"DSS-56");

    BLTCOD[687] = 399061;
    fstr::assign(BLTNAM.get_mut(687), b"DSS-61");

    BLTCOD[688] = 399063;
    fstr::assign(BLTNAM.get_mut(688), b"DSS-63");

    BLTCOD[689] = 399064;
    fstr::assign(BLTNAM.get_mut(689), b"DSS-64");

    BLTCOD[690] = 399065;
    fstr::assign(BLTNAM.get_mut(690), b"DSS-65");

    BLTCOD[691] = 399066;
    fstr::assign(BLTNAM.get_mut(691), b"DSS-66");

    BLTCOD[692] = 399069;
    fstr::assign(BLTNAM.get_mut(692), b"DSS-69");
}
