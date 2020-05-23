use crate::math::{ FiniteField, F64, fft, polynom };
use crate::utils::{ filled_vector };
use super::{ Accumulator };

// 64-BIT ACCUMULATOR IMPLEMENTATION
// ================================================================================================
impl Accumulator for F64 {

    const NUM_ROUNDS    : usize = NUM_ROUNDS;
    const STATE_WIDTH   : usize = STATE_WIDTH;
    const DIGEST_SIZE   : usize = DIGEST_SIZE;

    fn add_constants(state: &mut[Self], idx: usize, offset: usize) {
        for i in 0..STATE_WIDTH {
            state[i] = F64::add(state[i], ARK[offset + i][idx]);
        }
    }

    fn apply_sbox(state: &mut [F64]) {
        for i in 0..STATE_WIDTH {
            state[i] = F64::exp(state[i], ALPHA);
        }
    }

    fn apply_inv_sbox(state: &mut[F64]) {
        // TODO: optimize
        for i in 0..STATE_WIDTH {
            state[i] = F64::exp(state[i], INV_ALPHA);
        }
    }

    fn apply_mds(state: &mut[F64]) {
        let mut result = [F64::ZERO; STATE_WIDTH];
        let mut temp = [F64::ZERO; STATE_WIDTH];
        for i in 0..STATE_WIDTH {
            for j in 0..STATE_WIDTH {
                temp[j] = F64::mul(MDS[i * STATE_WIDTH + j], state[j]);
            }
    
            for j in 0..STATE_WIDTH {
                result[i] = F64::add(result[i], temp[j]);
            }
        }
        state.copy_from_slice(&result);
    }

    fn apply_inv_mds(state: &mut[F64]) {
        let mut result = [F64::ZERO; STATE_WIDTH];
        let mut temp = [F64::ZERO; STATE_WIDTH];
        for i in 0..STATE_WIDTH {
            for j in 0..STATE_WIDTH {
                temp[j] = F64::mul(INV_MDS[i * STATE_WIDTH + j], state[j]);
            }
    
            for j in 0..STATE_WIDTH {
                result[i] = F64::add(result[i], temp[j]);
            }
        }
        state.copy_from_slice(&result);
    }

    fn get_extended_constants(extension_factor: usize) -> (Vec<Vec<F64>>, Vec<Vec<F64>>) {
        let root = F64::get_root_of_unity(NUM_ROUNDS);
        let inv_twiddles = fft::get_inv_twiddles(root, NUM_ROUNDS);
    
        let domain_size = NUM_ROUNDS * extension_factor;
        let domain_root = F64::get_root_of_unity(domain_size);
        let twiddles = fft::get_twiddles(domain_root, domain_size);
    
        let mut polys = Vec::with_capacity(ARK.len());
        let mut evaluations = Vec::with_capacity(ARK.len());
    
        for constant in ARK.iter() {
            let mut extended_constant = filled_vector(NUM_ROUNDS, domain_size, F64::ZERO);
            extended_constant.copy_from_slice(constant);
    
            polynom::interpolate_fft_twiddles(&mut extended_constant, &inv_twiddles, true);
            polys.push(extended_constant.clone());
    
            unsafe { extended_constant.set_len(extended_constant.capacity()); }
            polynom::eval_fft_twiddles(&mut extended_constant, &twiddles, true);
    
            evaluations.push(extended_constant);
        }
    
        return (polys, evaluations);
    }
}

// 64-BIT RESCUE CONSTANTS
// ================================================================================================

const NUM_ROUNDS    : usize = 16;
const STATE_WIDTH   : usize = 12;
const DIGEST_SIZE   : usize = 4;

const ALPHA: u64 = 3;
const INV_ALPHA: u64 = 12297829253624015531;

const MDS: [u64; STATE_WIDTH * STATE_WIDTH] = [
    14570791697008582876, 14287730469917200292, 15111342538701370819,  1111401104756727833, 18343752991270580578, 10395724785100660355, 14391941175009286906,  5491581447267359356,  1031244057765854727,  2741392851187030668,  8356433820458919454, 10361960094523491469,
    17296841519685941390,  1598937820631795543, 18252132164632030075,   241444688886262292, 17599434116007097224,  9231563221418652240, 15805688995498349990, 17256539972135898838,  2330753493485837824,  6251316318077619492,  7679024702804679152,  2943046310091653711,
     8765181257382270816,  2214393267250057585, 10440968658565732009,  9245417370235283261, 10903600118637896817,  4336409820707567775,   373156886479152405,  6889031455917031511,  3512102634804595716, 10883465930001183358,   111817104814006178, 13147393342693302215,
    16731403170144839149,  4199639565163130888, 11662145218068586336, 12136048551747257563, 16207916864411664082,  5795305278626201409,  3540476316084861152,  1979412480869555052, 15915841166779902642, 12798161080806683411, 13864641321606711741, 15619568278035706351,
     6996727129159569475, 13016806715252432443, 10084910677946480774,  4854976610017568115,  3183560086131749081,    84343487446798268,  6976577640482780112,  1974323274754175061,  3184820667768309286,  2042442151413916954, 12542679131679444027,  8555877572558139340,
     1958751724798615870,  6883794720630798095,  7822377053889412653,  5881955449915523948,  1826773441475877560, 14596945442992680206, 15525428857452141988,  2300675574592518485, 14076693432158095299,  3913142224341430328, 15645746834898234524, 10122456385203566801,
    17729236334518869820,  1749133568562304236, 11557582145195575097, 12354965089547101180,  2660703340589826608,   440334420312600854,  2198333919637551622, 18369576650529043046,  7627007928139490203,   777383514316169044,  7198059378673430489,  9515098447333107659,
      806473578789499443,  9336648523780571670,  5686967342712013436, 12904381239407564150,  4756705991254623925, 10206157924696479727,  4806103767866561340,  1872796239547928750, 17178476308939330252, 11771556405037973998,  9117917859381111393, 13067245698773587742,
    10499199438819993852, 12171212168422477108,  5578998829898290932, 12178641820241082619,  4420340793047171628,  1530190253010972336, 15239962422294736463, 15238687364083809199, 12238023565508403339,   849083947368781781,  7020919173690859117,  6037995052907795325,
     6617094955015694929, 17709895726490950223, 12395004788391953988,  4443167981067561554,  7500390692332462441, 12515433586262237946,  5152881173520844228,  2876311526543549543,   665794271176964969, 13140309957664961201,  9790029746103080496,  7580688024839866118,
    10321065351363958810,  8558373207059165438, 15379391566111639810, 14892201863785415949,  9350656882348467407,  5434254718692813409, 12942082639379970277, 11437571946551726842,  7388656188826905221,  4131080631675148417,   721505112626253183, 17463850698301825653,
     4496862336099562608, 14815253423827224307,  3941759144983839864,   496394397619911104, 14326824099506036322, 16750030373530649240,   567590961228605744,  3305769176941360911, 16162169544256723442, 15781486664476971789,  5656405359499826694, 14405311909503260201
];

const INV_MDS: [u64; STATE_WIDTH * STATE_WIDTH] = [
    18144555849476927374, 13721382146536651531,  3089875797346993241, 16642169353490925937, 12481468684811990106,  4121843617940496225,  4129790781140692748,   688767946932393941,  4111150982137800446,  1089450373812047738, 13934895865615659551,  3348257440221276659,
    17836000848669323461,  8642507343089032585,  8222042489599816299, 14286433228805461667, 14150601651494101064,  3502610110434428933,  6621075793412742457, 13912617123037567676,  6398010452131362916, 14182317465044732728,   595200314641734087,  3726536648170807106,
      778074229987396986, 18125236013805002112, 16795870930471535933,  4444993276877870520,  7742856852104202289,  4607989154756352997, 13952151892042504619, 14813786624882131127,  1011914663726019958,  6259265713670753298, 15987952731985786134,  8099775052664498557,
     7398913649889091650, 12440566968934204138, 17649990195444781456, 11538885883660364618,  4407645622865595120,  6633698812571631570, 15650872682003063574,  9519805421328796770, 15112307222930510924,  5842544410030482907, 17158972732500521844,  8459189118411299370,
    15998460101840940499, 13289066588927556646,  3933740661569359061,  8545689675015500335, 10937438662640227039,  9542846015371531580,  8719735068114913444,  7998938619901238697,  8429477668004913435, 12463336703870748155, 12698637456361551034, 15219260363080271871,
    16868883943436188794,  9657158665570915168, 18237013319928696607, 17503060276710891958, 14055586713560957601,  2362219746134802271, 15127897720041080031,  9230115648985830785, 17475888821200468490,  4505598155300055020, 18226760539284171314, 15727384124014280557,
    17298291488582136802, 14907809634035681815, 16325978558052274623,  9352264028973735831,   917763106859827872,  6675329891611190692,  3583368781202404030,  1391846768326109030, 11727102779244870265,  5531319597647347119, 18392301100737590582,  9476204050182409105,
     3285182661460766588, 14713354330616573571,  7973333729386869825,  5171684086232147087, 12574741318087549907,  1104306757921324325, 11257236092622508264, 11209933342597752755, 14299420251835617036, 13527757428174476772,  8716799942601721461,  5209232145553849842,
     5100122196584094970, 14798850424290526893,  4871524293593127428,  3964693644228700308, 13757202860316855114,  1733380378805905884, 12934883999734076137, 16301178884859759514,  4986874405387808053,  3769509608404549933, 18176594401495523310, 18024931640497672795,
    15089590925251258017, 11577838215299384569, 13260977676197803928, 13894223429254193102,   490185726763680722,  8396702879775603420, 16316839076391380700, 15333245786856306259,  1990648171079364364,  5546594834266019573, 12914492430734648756,  9159818471374640061,
    14873706267930036825,  4482502765708235940,  1293121958623108848,  2912815298969125898, 16608741891263026413,  5505366372645852459,  2959054593591611614, 10214419207372171828,  8279669146557145773, 14389027943756575605,   287785456174655450,  9392565555941172313,
     4759713326423655569,  3126680109051985680, 12818455398525559133,  2369691254364065970,  4830855309800709645,  7905662622950188751, 12645002336872020497, 16986477359485720724, 14182644859010550052, 11863112114704177882, 17607018699185664534,  7041439209722523945,
];

pub const ARK: [[u64; NUM_ROUNDS]; STATE_WIDTH * 2] = [
    [  114590214580931143, 13640442143959558858,  1272600380366609460, 15860937208548502393, 16608634632094289187, 10506798047337663857,  5325632418193142483, 16694418951197876355, 11153790522126750648, 15095342153263941166, 12650134646349365568, 17896623204935797561,  9829178724264119989, 14799632988478146502,  6753568006804193711,  3899262017505750737],
    [  990026313300730203,  7906381567491729226,  2220919760127833817,  9205367959479920752, 11485160594813871566, 11583225239641066548,  6738669508591169269,  4460878955950299390, 12551264533866625651, 12196834433778588611, 10520176554432097244,  3832576278139503531, 18204716153294124968, 13922119650107368595,  5221249225950615330, 12958024067067001252],
    [ 1045057248257823412, 13468966696328224140, 14379250151341177495,  9632397799556151548,  5222338859687623994,  7117468705512735682,  3362648518227505874,  4835033812787157591, 12026895217977781113, 10038075628846372084, 14981243795409864846,  3943061593395212048,  3383138789483030842, 10483303931558023900, 13818497398442910065,  1502076825742192309],
    [ 1738842170916331719, 12028717281587475730,  9390139499790851184, 15641207201774937978,  2863623879347252536,  5411870088433704245, 17119289150453482846,  7494896406233705722,  6130985911650744988,  2298963299160633022,  7479945464639206088, 11302980479244781607, 17397821633202638835, 15850882608924817114, 15174626554736777698,  7006713201718463286],
    [ 1660024061661494622, 12360236492067308321, 11236058065485031565, 16046961331377309336,   285765561992035807, 12798502479124248565,  5754486647487519268, 13382585115788912201,   660742863064925723,  6645315222044092080,  9965209368988961530, 17820204584229116774,  9883547149695037939,  6150887863929317946,  7079711811973917915, 13597434219569508911],
    [ 4118625024966406515, 12835112754115328561, 17864236451301874186, 14256703065251832413, 14127457330899144583,  6975065549459864854,  9097469463743439998,  1253519805979009336, 15728435838135756664, 15759786865445700391,  3034945791846301754,  1562745209854105478,   907010472620729080,  6401580913706674080, 18263652983345176630,  1511109735614210289],
    [ 2532743523039298950, 12823753310782409353,  5773629776132606410,  5656038351339096699, 16895986339069166222,  4335784351584493848,  2037149858489564849,  9055475442526883912,  6523169235006781369,  1941965682495524095, 16032272684740445597,  2951795178487426576,  8304902812575713794,  5373458810467687890,  2307567856873438762, 10793106541234237943],
    [13390548470015888379,   464425379386213979,  8715600359526386850,  5836910653676200218,  2716466622487592381,  8189718254446049731,  7984836167078530680, 11482753252942536418,  4363931084647564588,  7186282174655107969, 14528661484935012766,  5652357482689995611, 14331329283351514796, 14789922746334895833,   436813593579246123,  4434838229414510491],
    [16970022055943448291, 14085297930256173746, 16425824203788480088,  5340223182091242339, 10179789832469522683, 12943689700147890218,  6768801130968139938,  9252093407568167747, 17035408350711990065,  6578491494114580749,  5841047196059148060,  8996125454236646900,   221642658918223204,  4190142597848043943,   834666877495857133, 13562089816803903154],
    [ 3229289085814137249, 16196250978843008202, 17409115598257936569,  9255093777916962941, 17039100264908022311,  9356770682649253190,   665613689507731001,  6499152131885064240, 10887287114234781470,  3380854232249920781,  2719221135769095413,  9217790001275019778,  5385609923943794086,   343359473954613140,  7345530097997874145, 12884285110856524538],
    [ 7936587485501753336, 15811509369575956025,  5423135786181100641, 11826465046601122678,  9026890456900619388,  3989125738446004276,  6360173557851361819, 16417801134130688265,  4303499409630365865,  5254132503209957135,  3190851936165568572, 17203985458255244261, 13667428876990115474,  9192301046220782497, 10812792283154425515,  5962630196859090857],
    [ 8739725378210913269,  1897390235444443527,  8073538104501442588, 12590108172722075838,  8710580870654515137,  4640735801628367947,  1510285585581949162,  2183680756588088228,   637659563021190270,  8732176820546665878,  1164305266058018084,  9218990590679415453,  5186172942936126179,  3007835204718287148,  9115527772309983459,  5162328662265872981],
    [ 3908716019379605388,  8685846242118849938,  8964464424724770931,  8831375762772429133, 11947873316670891644, 11084437424138123671, 12924480922475417013,  4919657103465191494, 16556703972734800022, 11225748325488875567, 17885677150884332555,  1142061936827087446, 13863332755530838626, 16489540385416479650,  8753380480526737677,  5179917688091495624],
    [ 9612982705190869253,  2057625545053330977, 10045064192798612192, 10108323264981338471,  1112973284660867749,  1236926833682309840,  9698031808966611289,  6496203980144675213, 12927870680923118087, 10897565103652250028,  5947329387220448339,  3485587962738468858,  5905689143964986175,  2169124019061467937,  5174930860066945260,  3956623854596830972],
    [ 8618151851322356977, 14051647126032743969,  9874857205854545189,  7996771765963606092, 11007673183642469915,  4156268537770062194,    45095423570381637,  2963438703557409527,  1477673319208488961,  7826191866327071671,  7016894678167938105,  5242966089922811236, 10367594978067884995, 10133816203841555937,  2266349523917941090,  7036223874982736730],
    [ 2377359971860592461, 13204296951363537507,  7826079088421977266,  4456823801127017073, 14763294168238043814, 14239032585103560950, 11652226892766320293, 17670180385508418381,  8700384419743007083, 17576152852246544325,   120211560097797556,  6063677237146099004, 12264048850382568692,  3453481292123999562,  4782969978677808388,  6802733812224482227],
    [ 3415531505045617650, 15287846411328905143,  4781663569206863262,  6638947615583655259, 16895375066730169259,  6547299622782829062,  8803408122176621572,  3686571536559258876,  6059719448680595046,  9182064618358072694, 14910010779341120350, 10429466719747587299,  5896382313398601680,  7267390723493308552,  7829564763305991559, 14054489564506049111],
    [ 1803014891579917898,  5335862441182164043,  5417874648613702824, 12827380617980181113, 13354187610379410185,  7776376767812261356, 12205462369332976460, 11645649520402170776, 14723381135997298035, 14207439538321506541,  1172405459466924054,  4586009116466735151,  3674388902309829291,  3078851633331719436, 15762670601448801880, 11338234005596413611],
    [18253631080420666568, 15571081595035039942, 16739233985783933024,   896738261464355590, 18154595923083929136,  2989135254698337287,    85961106285223169,  5433795935500584791,  9733357541940093386, 12959926542698839881, 15893272932775511790, 15959353338419288766,  8959216493992641954,  4886792342586674600,  3632718025276090815,  1014845084573886370],
    [ 4298533870496457949, 11731206051330436512,  4469597774767595777, 13923364648304841939,  5161722626952272978,  7256092968019329285,  3234201255026727919, 10521028498777907295, 13651543533069454502,  1942769408613955294,  6327424803852122343, 11195385734733065655,   937388370139546240, 17590381891315682441,   710788059782092579,  8133081070691434301],
    [ 3516726635115581733, 11654771749971427770,  4679477611749215016, 16941163481097200881,  7241026283969380040, 14188707037259549432,  8387012370116828251,  3586954401532644794, 18375032832072157074,  9097026839809584277,  4135806251735040123,   283529781532770116,  9296651159758516239,   670612687949387417, 18064031994582034297,  1070396513838557026],
    [14188758512685690560, 14954496140063031735,  8584374822608982414,  2814933892951474552,  8502745946138759783,  8422049074455859192,  9259583648285412165, 14738222770780299335,  2790650458893195908, 11605870178390216904, 10962933020584581860,  4078853325395306005, 17860747862653634876, 16977809716846529969,    25844892353334976,  9907315241648096480],
    [17767631070091881919, 12129770816065322969,  3870778731417206155,  2763928716174813789, 17699330642314234768,   284391477128702387,  5570727129888298990,  4758537254264021385, 12566239596188951579,  2189846267133984032,  5629725228396968419,  3186491963103640941,  2602992362346531180,  8702861261518213055, 10940756722905455830, 12546870881595554008],
    [ 9712595766457023099, 17947404629239381291, 14873483792285815143,  8124814109136622356,  1863730773280059059, 16775376511215779115, 10155279417770795079,  4987258354806164162, 11417887696602517231, 16762109218418199845,  7525158767962017374, 13539031210369567137,   451702918043897126, 16187176288633596652,  1097981270823570691,  1840352501975548911]
];