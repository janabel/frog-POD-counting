function parseVerifyingKey(verifyingKeyString) {
  //   // Utility function to parse tuples (x, y)
  //   function parseTuple(tupleString) {
  //     return tupleString
  //       .replace(/[()]/g, "") // Remove parentheses
  //       .split(",") // Split by comma
  //       .map(BigInt); // Convert to BigInt
  //   }

  //   // Utility function to parse QuadExtField values
  //   function parseQuadExtField(quadExtFieldString) {
  //     const parts = quadExtFieldString
  //       .replace(/QuadExtField\(/g, "")
  //       .replace(/\)/g, "")
  //       .split(" + ")
  //       .map((part) => part.split(" * ")[0]) // Extract number before ' * u'
  //       .map(BigInt);
  //     return parts.map((n) => n.toString());
  //   }

  // Parsing the VerifyingKey string
  const vk = {
    alpha: [],
    beta: [],
    gamma: [],
    delta: [],
    gamma_abc: [],
  };

  // extract alpha_g1
  const alphaMatches = verifyingKeyString.match(/alpha_g1.*?beta_g2/s)[0];
  const alphaNumbers = alphaMatches.match(/\d{2,}/g);
  console.log("alphaMatches", alphaMatches);
  console.log("alphaNumbers", alphaNumbers);
  const alpha = [alphaNumbers[0], alphaNumbers[1], 1];

  // extract beta_g2
  const betaMatches = verifyingKeyString.match(/beta_g2.*?gamma_g2/s)[0];
  const betaNumbers = betaMatches.match(/\d{2,}/g);
  console.log("betaMatches", betaMatches);
  console.log("betaNumbers", betaNumbers);
  const beta = [
    [betaNumbers[0], betaNumbers[1]],
    [betaNumbers[2], betaNumbers[3]],
    ["1", "0"],
  ];

  //   const gammaMatches = verifyingKeyString.match(/gamma_g2.*?delta_g2:/s)[0];
  //   console.log("gammaMatches", gammaMatches);

  //   const deltaMatches = verifyingKeyString.match(/delta_g2.*?gamma_abc_g1/s)[0];
  //   console.log("deltaMatches", deltaMatches);

  //   const gamma_abc_g1_Matches = verifyingKeyString.match(/gamma_abc_g1.*?}/s)[0];
  //   console.log("gamma_abc_g1_Matches", gamma_abc_g1_Matches);

  //   // Extract beta_g2

  //   const gammaMatches = verifyingKeyString.match(/gamma_g2: \(([^)]+)\)/);
  //   //   vk.beta = verifyingKeyString
  //   //     .match(/beta_g2: \(([^)]+)\)/g)
  //   //     .map((match) => parseQuadExtField(match));

  //   console.log("betaMatches", betaMatches);

  // Extract gamma_g2
  //   vk.gamma = verifyingKeyString
  //     .match(/gamma_g2: \(([^)]+)\)/g)
  //     .map((match) => parseQuadExtField(match));

  //   // Extract delta_g2
  //   vk.delta = verifyingKeyString
  //     .match(/delta_g2: \(([^)]+)\)/g)
  //     .map((match) => parseQuadExtField(match));

  // Extract gamma_abc_g1
  const gammaAbcMatches = verifyingKeyString.match(
    /gamma_abc_g1: \[([^\]]+)\]/
  )[1];
  //   vk.gamma_abc = gammaAbcMatches
  //     .split("),")
  //     .map((item) => parseTuple(item + ")"));

  // Structure the vk object to match the expected snarkjs input format
  //   return {
  //     protocol: "groth16",
  //     vk_alpha_1: vk.alpha.map((n) => n.toString()),
  //     vk_beta_2: vk.beta.flat(),
  //     vk_gamma_2: vk.gamma.flat(),
  //     vk_delta_2: vk.delta.flat(),
  //     vk_ic: vk.gamma_abc.map((tuple) => tuple.map((n) => n.toString())),
  //   };
  return 0;
}

// Example Usage
const verifyingKeyString = `
   VerifyingKey { alpha_g1: (18659280780364347995792981478407887551539604024143433299947873520124282774731, 1103543952071697337430707497664756787355157165035124104997361183854643793586), beta_g2: (QuadExtField(14141036587402053347850728656564262116713794817999269734759810517430149755553 + 19975142237823693343519810371077628115829250821588694182626096611670802050798 * u), QuadExtField(17973507328325687514488683241570630228926057844678879307787800129022424121415 + 6990257004297676679135546478003767658511787908213680223371923277522549997619 * u)), gamma_g2: (QuadExtField(9001536208103102027172953742722183864884928616203983594296926330352939019994 + 11613106855852289258622267996180814158809529742774181199628761241160993697529 * u), QuadExtField(13280541535945221525481176007762929075059656590396901918968180065170957739606 + 15578501641725008454198206050129445877533970857539837590949656412236548420602 * u)), delta_g2: (QuadExtField(8409386831455829762307079342250090990013824606224275860053787947285755120973 + 10948594080733705986164698012431501637456803253631838137504396755554366596760 * u), QuadExtField(14200543649838628194718760287834049212047151644595656777226960100186093022166 + 5320499381102179249274582229983868624773287587789793715711625452442153436039 * u)), gamma_abc_g1: [(19912875546870326808117139245472245808429078086712948274350101391036249474658, 14789495932168448558398369477974976960871995381565228309361888526784994805971), (6602062616537693238485250779480463270455020423632535165499143671499247514347, 4397159710482813791150044220248651666128934308501873484466114488231631855603), (928908021948662235309814419290696935990498763995648293024751494669834223178, 2396364366629732832680367165609109206825755826722540395913864491282841995887), (4524997678073178027805941592486044997521099508538366773541118980262272314737, 10046273669898694081654932695174101886577522970857892935017396087640657128865), (340518044903054667552490871674111269312077147927225280055963734145214495013, 9293961000653213904040150013701189582938748676065519696733983268460260263648), (18486834283996651948261035835581443390386329169105140267277569185810607379406, 14624063153371511681974582127281537537697478176935485818683182826273218020657), (14733908539325376051705823614417755362181767381593725154043698142937093866075, 5770364657887470171994432940191691474980275240844073574432482307421971270489), (2591925273798256876830541981043243150192212004039734214791553668952046081526, 20948784173816936812196174565872106876853598603913946402914067669198993737069), (17770028078693751809553741480848795751975825932950454988887686291500437571734, 18525455453412364486867330941933818396814815268411546517559167916110959357423), (5411837535541508612796073578957407045702219166550442744564534043810037834932, 15427400004710116914151822420076196746341997460965494736245017368199227932454), (5355769422190591204358051820126547806717775026902028202237901227142729094075, 17129939904244234541697594676780423442499557553279466658195703776715957732652), (19751087871064964852752782355704008243096952338673318490055463132604749177995, 13485401808066149323049300057800514106927124774340922406300342307140242903402), (7662274994043991079500023324900699055288636313918024736221064947821755539687, 17483946273368828153245580549322263125096838346126425019861675790452892804228), (15453428568040443835896654269088190510226884715928822591070988280795629782055, 14667136326109682616170135007851364367829616937176599248701216164786775916806), (17526728998343677055659653198028592575912290036464259381434470524763740924311, 4969604322098181130574743337821520530127489706104662447850828268535688173415), (5427734093661446252842219686870566902875266216694615303901840567715877134580, 6360639569493828932936072737126735173612029732827470844020935021277449960921), (8721302120396709222380131698217751825930024176160815231855653652309125413716, 16420697434704633071707914887088835482494660545830924329133254191893303418301), (11462592955455223229626877451574060872735128683648740410590573444991726602529, 8677544354058609685152339125275830384015706495005562361849746857687784432526), (7249716083925916184492651499847135297643521873567117380094047754763706912466, 13544821277966374135513089502946214923077470321117641751558391392497281401841), (13181413086688272842062583257465238772928138043479872062037304642156386048853, 15355514290772300393881729000654342527082928733075236038276325018976946004128), (16025254619618957242305385770923360322269144177637461996207305452479771543869, 17354219347068539572422219287661221802909453433193746121642338855071457743750), (1478521667133492514511143641547172932219893605963785898728270138698178635784, 13578531615855044606900771957159754278313488940613928087133596770069814875981), (17606910014702193630879306383592701737816134626463122693705722142859712643345, 20101166908735639624657384999728079328509469439753712003357053699935729175949), (13559442830867528436461341911273585977170706378911216145829929842022200262536, 11619646222570365051848500865808786837785141464874399299116429363139686206708), (6696727199610901639121259371225942123121807914994001655804196980371189765691, 113556406445683707873682982004135261073915284038940762324723599228611371717), (3867062346152924713106217662903955831655251531874772515400280563575960101705, 8378991380341440096392624190861769393653392225286318088226675866681773131621), (19149940731636004315864156205206891202403273940757599908885591119765260599771, 9017034211540031689400769257621046167729518817242840403823215216451349995578), (6910787770436611301006931926149300394722923213766645476322675429333201482825, 7036753930123596994917233690016027198505429088778895117952660856014716076092), (16945995716393293362981364746136343796872279246072907478744533304566254327562, 8818446537797776398040534151377427265311386629981238729612373298103770957182), (13810062190544616638712899402243598550130021394364933508938151504995231875374, 571840835331404894792980991328459607080641577178425397158472587127896632761), (14586921732319927333660410093565275754202846015426802490956810379193646035328, 5207822316674108858667543720039202978018444228107470079659364269152794289191), (15805446966308361000284436784084621325084453325222081110660629943102051615492, 18723900300152821431202759341172256616951388822853554288762021269215335415185), (17778926221787623437443740302073571303447867204711560239868883748838002904206, 6623279307211047481447992169526770475634730964862292131182629585994803488907), (16236938112910594497516294924575165201746304475837952322614457003432110540225, 9529152142199457901100710476884726013508659123720431825138476815987195259755), (47523133195370621518305027160632210532096882325188064250621687224255148505, 10532832735789228054127872417496647078460485649612504097741333457356309355888), (18251482441208596070559854020322680958045304194122675044215880104506128545553, 20513086267979779368201481950359014343737570070245206021999924029056284616431), (8165038284864590783058112077129748213356296817827257799962952885848243915818, 1869986228133778291202499563529990809554552705563756790742166404241156954322), (6247364667712689164073325112208814543201525026060580641588097445687550662778, 18052951946387311816648154650871686127630875333191076958771799694836214560436), (21780371514579382647594542861857926795891666910743513498552479246327848529746, 14268089317695095171324743230498037521926605095645753829934908742540750899174), (16259131934049584354774008789825224935659585888926962801987846693753009599386, 20474780341452419535904075118511317339468923679879160665645452738221402042705), (3201175462791888773231676454577806664796848292729706080831197669962472399584, 16582288089814328184194293902667005223250147551217063141303153660534851375912), (363263432994883381898245959154018645398626990581269942556652653527633806445, 17947292951935003750069479263554017500011863738354027667159753140096383568544), (17015478563971582053212285955921290411638134868209220399542630263504195791188, 899275515594973845269358820440625946074336998624645870011163088534145108614), (11398652284665649376120127853485544046611070810145820292507193748082363788255, 4745950658881446112750324106628711721765103578702021239306242807889540598194), (4065401509645223493564519295646622162889424291542843628586124041840918732175, 18749220248870009381644873607926915815640947895710031826065953428082006402707), (16240041607953109782959957335822768955189676448606333683654526899335322876684, 1292785084053925555024149458942421854144629688585933781907591797428981712610), (14178558232515842776155609632492430436518863355939704834362391025834932657616, 13812238414952989792874107128498031763617806788574203337294260043931382522563)] }
`;

// Parse the verifying key string
const snarkJsVk = parseVerifyingKey(verifyingKeyString);

// Output the parsed verification key
// console.log(JSON.stringify(snarkJsVk, null, 2));