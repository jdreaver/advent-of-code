use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn main() {
    let ingredients = parse_input(INPUT);

    let part1 = find_non_allergen_ingredient_count(&ingredients);
    println!("part1: {}", part1);

    let part2 = canonical_dangerous_ingredient_list(&ingredients);
    println!("part2: {}", part2);
}

fn find_non_allergen_ingredient_count(lists: &[IngredientList]) -> usize {
    let allergen_ingredients = possible_allergen_ingredients(lists);

    let mut all_allergen_ingredients: HashSet<&String> = HashSet::new();
    for this_allergen_ingredients in allergen_ingredients.values() {
        all_allergen_ingredients.extend(this_allergen_ingredients);
    }

    let mut ingredient_counts: HashMap<&String, usize> = HashMap::new();
    for ingredient in lists.iter().flat_map(|list| &list.ingredients) {
        *ingredient_counts.entry(ingredient).or_insert(0) += 1;
    }
    ingredient_counts
        .iter()
        .filter(|(ingredient, _)| !all_allergen_ingredients.contains(*ingredient))
        .map(|(_, count)| count)
        .sum()
}

fn possible_allergen_ingredients(lists: &[IngredientList]) -> HashMap<&String, HashSet<&String>> {
    let mut allergen_ingredients: HashMap<&String, HashSet<&String>> = HashMap::new();
    for list in lists {
        let ingredients_set = HashSet::from_iter(&list.ingredients);
        for allergen in &list.allergens {
            allergen_ingredients
                .entry(allergen)
                .or_insert(ingredients_set.clone())
                .retain(|i| ingredients_set.contains(i));
        }
    }
    allergen_ingredients
}

fn canonical_dangerous_ingredient_list(lists: &[IngredientList]) -> String {
    let mapping = find_allergen_ingredients(&lists);
    let mut allergens: Vec<&String> = mapping.keys().cloned().collect();
    allergens.sort();
    allergens
        .iter()
        .map(|allergen| mapping[allergen].clone())
        .collect::<Vec<String>>()
        .join(",")
}

fn find_allergen_ingredients(lists: &[IngredientList]) -> HashMap<&String, &String> {
    let mut allergen_ingredients = possible_allergen_ingredients(lists);

    let mut final_mapping: HashMap<&String, &String> = HashMap::new();
    while !allergen_ingredients.is_empty() {
        let (&allergen, ingredients) = allergen_ingredients
            .iter()
            .filter(|&(_, i)| i.len() == 1)
            .nth(0)
            .expect("couldn't find allergen with only 1 possibility!");
        let ingredient: &String = ingredients.iter().nth(0).unwrap();
        final_mapping.insert(allergen, ingredient);
        allergen_ingredients.remove(allergen);

        for ingredients in allergen_ingredients.values_mut() {
            ingredients.remove(ingredient);
        }
    }

    final_mapping
}

#[derive(Debug)]
struct IngredientList {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

fn parse_input(input: &str) -> Vec<IngredientList> {
    input
        .lines()
        .map(parse_ingredient_list)
        .collect()
}

fn parse_ingredient_list(input: &str) -> IngredientList {
    let (ingredients_str, allergens_str) = input.split_once(" (contains ").expect("no contains");
    let ingredients = ingredients_str.split(" ").map(|s| s.to_string()).collect();
    let mut allergens: Vec<String> = allergens_str.split(", ").map(|s| s.to_string()).collect();

    // Need to chop off last paren in allergens
    let num_allergens = allergens.len();
    assert_eq!(allergens[num_allergens - 1].chars().last().unwrap(), ')');
    allergens[num_allergens - 1] = allergens[num_allergens - 1]
        .chars()
        .take_while(|c| *c != ')')
        .collect::<String>();

    IngredientList { ingredients, allergens }
}

const _EXAMPLE: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

const INPUT: &str = "jfvltdz txzv szzbr fcbsv klzdr xclff jqqbnzz flzmlbf crfxz jxrlzm xcfpc tvk kdbx zlhlq gfpnx ccvsl zfl skllln lxqbsc frgnkt mrnd smjhxg cmqjlb jxdbh tsbffm mtjlb czbhh gbbxhp snsnj lmvx pbztj glvv jkzxn nnfps hnvrfvm fddx nhhgb hdt jg hrnzq tmgfr vjcbtd nklc fmbqgs hcg gpgrb qrgvj znqbr pfdkkzp vmrf mbqpfm pccmj bph hmngm fcbvh vtqhqs zsjlp rdbdd gtjmd bskstb krxj rglbcpq svqz jczm frssvh nmxdsnc hjbs xkpm tsvfrt txr flmm tjrm jccqsxs bncfz vlpq ngl dvfqqz hqxdv xzrdx qdnh hbqhvk spbxz pnxzqgp kjcdgh ttmcq dlvdt (contains peanuts)
bjqt frgnkt ctbmdg hbqhvk skllln spbxz frssvh rdbdd gpgrb nndr dvfqqz jlnrn tsvfrt jccqsxs jkzxn znqbr vlpq hcg gtjmd lmvx zck jnd vghkr fmfnh rlsqd vjcbtd kbszttf mdsg pfdkkzp stnb tjlz bqc gfpnx mfvxhv pdss tzglth mtpfk cnhbh thprs kvcc hnvrfvm klzdr xcfpc kdxvzqm (contains peanuts)
pngs tjlz nmxdsnc qdnh pccmj mkz rdbdd mbqpfm ngl znqbr tzglth tlbj klzdr pgrc fddx mxmvt srxrn gtjmd vdgsz dxzzp zfsmv svcmg mzjvq txr jkzxn smjhxg dptl flmm xlcgr srpqk kdbx bctn hnvrfvm qkvrf kvcc qzqdq krdmn vlpq tmjltg kdxvzqm hdt thprs pfdkkzp nklc cmqjlb jrrgck gpgrb mdnchc gzngn qrgvj pznt pdss zjh crfxz krxj xcfpc svrvv ctbmdg spbxz (contains shellfish)
xcfpc zfl cnhbh mdx tjlz pnxzqgp drzm glljh xsndjl hrnzq pdss zck kjcdgh pgrc bph gtjmd xmcsr ctgnz kbszttf gpgrb spbxz vtqhqs snsnj brjd znqbr mbqpfm crfxz blbfl rjds gdsk kdxvzqm mdnnfp mrnd mzjvq flzmlbf pznt pggzp txr vxx bncfz lzfmghs krxj pfdkkzp rtjxbbd (contains dairy, shellfish, fish)
mdx glcfml jqkxx jccqsxs kvcc nndr kbszttf mfvxhv rjds tjrm spbxz gtjmd vtqhqs fmbqgs dxzzp snptbf hrnzq rtjxbbd tmgfr sckhr hsdnz xkpm txzv fgrmrqhp gxbs gfpnx mdnnfp kbtkrcf drzm tbptvzf dtqplpj vxx cmtq zsjlp gpgrb glvv qrgvj tmjltg hnvrfvm pbztj czbhh tvk cnhbh krdmn flzmlbf lblbm mrnd znqbr kdbx xcfpc nhhgb mdsg zck txr stnb hdt hjbs pfdkkzp skks klh zfl (contains dairy)
skllln tvk srxrn snptbf cnhbh gbbxhp jczm spbxz pznt kbtkrcf fmbqgs tjlz frssvh pmhxl znqbr mtpfk fcbvh rdbdd xpmmdc vjcbtd cmqjlb jqqbnzz nnfps lblbm kdxvzqm qzqdq bjqt txzv qrgvj tmjltg mxfrnq hbqhvk gtjmd jlnrn kvcc hsdnz dfllfp mfvxhv mfcrq tgclx xlcgr glvv czbhh tsbffm hmnt xcfpc mkz cmtq gpgrb (contains soy)
gtjmd zbj vdgsz stnb tsbffm tjlz pdss jqqbnzz nklc srxrn klh glljh ctbmdg dptl jczm pfdkkzp jfvltdz nhhgb glvv jccqsxs hsdnz bskstb brjd ctgnz rjds tjrm qrgvj zck spbxz zfl xbp mfcrq svrvv zsjlp bqc pbztj xsndjl bctn lblbm fmfnh kdxvzqm cdntx hjbs gnkr rtjxbbd hdt hcg vjcbtd hmnt skqbbj frgnkt znqbr klzdr tmjltg nndr hsdqb tgclx txzv hnnz mdx pngs jg snptbf czbhh cnhbh jrrgck jlnrn gpgrb mfvxhv mdnnfp sckhr pccmj gfpnx dxzzp tbptvzf (contains peanuts, soy, dairy)
nkshjm pgmqs znqbr cdntx hsdnz kdxvzqm pmhxl tjlz nndr lzczv mtjlb jqkxx bjrbknjm pbztj nmpp mkz tbptvzf krxj bctn ctgnz qlzxbc hbqhvk mbqpfm dvfqqz gtjmd mfvxhv gdsk snptbf cmqjlb gpgrb jfvltdz vlpq zjh pggzp xkpm xpmmdc tgclx bqc spbxz kjcdgh lflzqr glvv stnb tjrm txzv qzqdq hjbs ccvsl jxdbh xcfpc bncfz flmm blbfl zbj rglbcpq srpqk tsvfrt fcbsv rtjxbbd bph tsbffm smjhxg tmjltg (contains shellfish)
snsnj tmjltg lxqbsc czbhh txzv svqz krdmn tjlz jg bnrf nmxdsnc ndhx zck zjh skks rdbdd bnnt srpqk svrvv spbxz pdss cmqjlb vmrf crfxz gpgrb jdvtlsv sgh mkz pznt tgclx kdxvzqm jfvltdz ccvsl jlnrn gdsk pfdkkzp flzmlbf xcfpc jqkxx dptl hrnzq vghkr tmgfr glvv mfvxhv gtjmd fgrmrqhp mdnnfp klh glljh kdbx lzczv klzdr (contains nuts, fish)
znqbr tsvfrt bjrbknjm kbtkrcf hsxp xsndjl jqkxx mrnd tjlz klzdr krxj gnzr nhhgb hqxdv mggk hsdqb zfl svqz hmngm ctbmdg tbptvzf bctn mdx gfpnx bskstb blbfl thprs sckhr lzfmghs rdbdd gbbxhp rglbcpq lmvx jkzxn lblbm bncfz ctgnz tjrm tlbj sndrzm tsbffm hdt spbxz jfvltdz gpgrb hrkf txzv ccvsl mtpfk vlpq pgmqs ttmcq pfdkkzp ndhx mdsg skllln xcfpc bph bdj (contains nuts, shellfish)
vdgsz srpqk jccqsxs txzv gtjmd xzrdx jdvtlsv hcg hjbs tsbffm vlpq bctn fmbqgs krdmn crfxz gpgrb zjh skllln bjqt kdxvzqm mdnchc zfl svrvv zlhlq tzglth ccvsl frgnkt xpmmdc rtjxbbd dptl dlvdt kdbx xclff gbbxhp lxqbsc vmrf lmvx smjhxg skqbbj nndr jnd rlsqd xmcsr pfdkkzp hmnt jfvltdz gnzr znqbr ctbmdg svcmg flzmlbf tjlz tbptvzf bncfz vxx tmjltg xcfpc qlzxbc klh jg xbp mrnd pgrc qkvrf qdnh qzqdq vghkr zfsmv fddx dkzz fcbsv mggk (contains shellfish, soy, eggs)
krxj kbszttf hnvrfvm mkz fjsgm nkshjm tjlz mfvxhv pccmj gnzr txzv flmm czbhh bnrf jccqsxs cnhbh gtjmd klh jg skllln hdt jrrgck nndr vtqhqs hrnzq dtqplpj blbfl glljh snptbf fddx spbxz pnxzqgp bjqt gnkr svqz dptl bncfz lhqxr pfdkkzp zsjlp srpqk dlvdt xcfpc dvfqqz zfl lmvx mzjvq thprs pngs sgh tmjltg tbptvzf vxx nmpp jdvtlsv hrkf rlsqd xzrdx mrnd skqbbj xclff szzbr srxrn vlpq nmxdsnc qkvrf hcg mfcrq fcbvh kbtkrcf mxmvt hjbs jfvltdz gpgrb (contains soy, shellfish)
bnrf jrrgck xcfpc kdxvzqm lmvx xzrdx pfdkkzp crfxz gtjmd krxj qkvrf kdbx jdvtlsv mdnchc mbqpfm jfvltdz txzv vghkr jqkxx jxrlzm klzdr bnnt xpmmdc tsvfrt fcbvh xbp tlbj rglbcpq hdt vdgsz spbxz dxzzp hsdnz jg fgrmrqhp tmjltg jxdbh gjt mggk svrvv zbj mxmvt bncfz hsdqb qdnh hbqhvk kbszttf jkzxn dvfqqz tsbffm mrnd jnd hsxp zck tjlz gpgrb (contains nuts, soy, dairy)
lmvx qzqdq mtpfk dkzz zck frssvh pbztj qlzxbc pccmj sckhr zfl dptl fddx mrnd gdsk mdnnfp hnvrfvm jfvltdz mtjlb lblbm spbxz hsdqb zbj krdmn lzfmghs pfdkkzp gzngn flmm hrnzq jqkxx znqbr jrrgck jkzxn svcs xcfpc mdsg fjsgm frgnkt rdbdd klh cmqjlb hrkf mxmvt hdt tzglth glcfml hbqhvk mkz bncfz pdss gtjmd jczm nrdv tjrm svrvv flzmlbf txzv pmhxl tjlz dvfqqz (contains peanuts, soy, nuts)
mfvxhv pfdkkzp nrdv xcfpc znqbr zck gjt snsnj lmvx nhhgb fgrmrqhp bdj bnrf hrnzq lhqxr kdbx spbxz zfl glvv gpgrb vmrf hnvrfvm pccmj bqc fjsgm jnd xpmmdc hmnt bjqt mrnd jczm fcbvh tjlz hjbs frssvh gnkr pgmqs lxqbsc skqbbj hrkf frgnkt tbptvzf rglbcpq xbp sckhr txzv vjcbtd jqqbnzz mkz (contains peanuts)
jczm gpgrb dfllfp tzglth txzv fmfnh jccqsxs zfl spbxz nhhgb cdntx txr kdxvzqm smjhxg fgrmrqhp qkvrf skqbbj kbszttf rjds snptbf cmqjlb szzbr glvv sckhr mbqpfm mxmvt gnzr pmhxl jqqbnzz bnrf frssvh bncfz znqbr dvfqqz mxfrnq hcg ttmcq kbtkrcf czbhh mdnchc hnnz xcfpc gbbxhp stnb pfdkkzp nrdv gtjmd zsjlp qzqdq flzmlbf vdgsz krdmn bjqt zjh kjcdgh sgh lblbm pdss hsdnz vxx skks mggk ctgnz xzrdx jqkxx klzdr fddx lflzqr svcmg xmcsr (contains nuts, shellfish)
hsxp tjlz crfxz xcfpc pggzp spbxz skqbbj jkzxn tzglth svrvv mtjlb gtjmd hmnt jccqsxs vlpq tmjltg zfl gpgrb lxqbsc znqbr pfdkkzp mdnchc dlvdt gbbxhp hdt sgh rjds skks mkz hmngm vxx mzjvq cnhbh pgmqs lzfmghs qkvrf tsvfrt czbhh qrgvj hrnzq lflzqr dvfqqz xclff jxdbh pdss fcbvh (contains wheat, shellfish, eggs)
jccqsxs rglbcpq pmhxl znqbr vdgsz jqkxx pngs txzv mdsg tvk cmtq dfllfp lhqxr fddx skllln dvfqqz crfxz lzczv brjd tlbj gfpnx gtjmd gxbs gnkr xmcsr dlvdt hsdnz bdj nklc zjh lzfmghs jqqbnzz pznt lflzqr bnrf cdntx tmjltg xcfpc lxqbsc tjlz mfcrq rlsqd kdbx vjcbtd dxzzp spbxz mkz pccmj hbqhvk nndr pfdkkzp fcbsv klzdr hdt svqz svrvv pbztj (contains shellfish)
skks tzglth lmvx bph txr gpgrb klzdr txzv jnd svcmg xclff zsjlp jrrgck spbxz kjcdgh mdnchc kbtkrcf bqc bctn kbszttf dxzzp gtjmd mtjlb pfdkkzp hnvrfvm smjhxg gnkr pggzp vjcbtd lzfmghs vtqhqs tjlz fgrmrqhp svrvv fcbvh ttmcq skllln fjsgm xpmmdc xcfpc nkshjm crfxz lzczv zfl rtjxbbd hsxp cmtq snptbf skqbbj cdntx (contains nuts, peanuts, soy)
xcfpc hrkf txr gxbs snptbf kbtkrcf jxrlzm mggk nmpp txzv vghkr mfcrq sckhr nrdv lflzqr xlcgr pfdkkzp hmnt fcbvh mkz glcfml znqbr cmtq spbxz crfxz gtjmd mdsg xzrdx skks stnb nkshjm zsjlp dptl fmbqgs kjcdgh jfvltdz blbfl vtqhqs lzfmghs hjbs hdt lxqbsc klzdr tjlz vxx pngs lmvx zck gdsk hsxp pccmj jrrgck fmfnh jqkxx fjsgm ccvsl jczm tlbj qkvrf smjhxg gjt jqqbnzz svqz rjds vlpq pmhxl qlzxbc pggzp lhqxr (contains wheat)
bdj bph smjhxg pggzp svrvv dvfqqz pnxzqgp rglbcpq skks dlvdt pccmj tjlz hsdqb zbj qkvrf pmhxl fmfnh cmtq txzv ctbmdg vghkr cmqjlb gtjmd vjcbtd xpmmdc gzngn hrkf blbfl srxrn ctgnz rlsqd dxzzp fcbsv vxx zfsmv lblbm lmvx pgmqs tzglth bncfz mdx vtqhqs drzm jg jnd pfdkkzp xcfpc zfl zjh jfvltdz skllln spbxz znqbr mtpfk dfllfp mxmvt zck svcmg kbtkrcf xsndjl gjt kdbx klh nnfps (contains peanuts, wheat)
jrrgck jfvltdz bjqt nndr znqbr xclff lzfmghs lxqbsc xbp dlvdt txzv tjrm ttmcq gtjmd rdbdd skqbbj dtqplpj kdbx mfcrq pggzp nhhgb tgclx pznt gnkr vghkr xcfpc klzdr krxj lzczv klh czbhh vtqhqs skllln rjds gbbxhp hsdnz bnnt qkvrf glvv bnrf tsbffm lblbm zlhlq dvfqqz bph jkzxn dkzz cnhbh tmgfr hsxp crfxz lhqxr ndhx ngl glcfml pfdkkzp mbqpfm ctgnz bdj spbxz brjd fcbvh gpgrb nklc smjhxg (contains dairy, peanuts, fish)
ttmcq gbbxhp vxx cmtq fmbqgs nklc svqz znqbr hsxp hnvrfvm mtpfk pccmj pdss jczm mrnd zbj gtjmd klzdr thprs czbhh fddx blbfl tlbj fmfnh jdvtlsv txr rdbdd xcfpc glljh snsnj tjlz jqkxx tgclx gpgrb gnzr skllln nrdv qkvrf sgh kbtkrcf hdt pfdkkzp kdxvzqm bnnt jnd jg hqxdv spbxz flmm szzbr sckhr (contains nuts, dairy)
svqz txzv gpgrb fcbvh gnzr tjlz drzm vmrf qkvrf dlvdt hsdqb hmnt spbxz tjrm mggk hnnz sckhr bjrbknjm jfvltdz xcfpc gtjmd zjh tvk srxrn mfcrq vjcbtd fcbsv xkpm gjt hbqhvk krdmn mdnchc zlhlq ndhx pfdkkzp mdnnfp xzrdx jxdbh gnkr zck pznt bncfz nmxdsnc hrnzq skqbbj mdx kdxvzqm snptbf hsdnz mdsg dxzzp rdbdd (contains shellfish, eggs, peanuts)
dptl rlsqd tjlz mdsg pfdkkzp tzglth mxfrnq gzngn tmjltg mggk hmngm mdnchc nkshjm hnnz zlhlq cmtq vxx hnvrfvm sgh frssvh pdss mdnnfp xcfpc kbtkrcf qzqdq mrnd xbp nndr znqbr szzbr ctgnz spbxz ndhx svqz pbztj bdj smjhxg kbszttf gpgrb jkzxn hqxdv dtqplpj kdxvzqm crfxz cdntx mbqpfm hsxp dxzzp nnfps pgrc cnhbh fmbqgs pnxzqgp svrvv mdx klzdr zjh tsbffm srxrn glcfml rglbcpq hcg gtjmd glljh zck pngs jczm lxqbsc pccmj glvv (contains dairy, peanuts)
znqbr hnvrfvm svrvv kbszttf bctn jccqsxs drzm rlsqd jlnrn srxrn rjds gjt skqbbj qrgvj kjcdgh vghkr svcmg gxbs xcfpc rtjxbbd zfl ndhx lzfmghs qdnh xkpm snsnj bqc lblbm dptl fcbvh gtjmd ccvsl txzv jfvltdz kdxvzqm mtpfk bph tjrm xpmmdc jqqbnzz cnhbh czbhh dtqplpj fgrmrqhp nmxdsnc gpgrb pggzp glcfml hrkf ttmcq mfcrq jczm zlhlq cmtq mdx txr vjcbtd mggk qzqdq skks svcs sndrzm bjqt hsxp tmgfr dkzz jg zck tjlz spbxz pnxzqgp klzdr mdsg zfsmv xbp (contains fish, wheat)
vjcbtd jdvtlsv ccvsl krxj qlzxbc fmfnh zjh hcg jxrlzm spbxz txzv sgh flzmlbf hsdqb nhhgb srpqk xlcgr pggzp skqbbj dlvdt mdsg gzngn pmhxl jfvltdz dptl fgrmrqhp bjqt hsxp tvk gnzr xcfpc qkvrf gtjmd glljh bdj tmgfr jg pgrc hrnzq nnfps jlnrn svcmg zbj klzdr pccmj jqqbnzz gpgrb tmjltg mrnd hnvrfvm znqbr mbqpfm qzqdq snsnj kbtkrcf smjhxg qdnh hrkf mfvxhv srxrn tgclx tjlz fjsgm sndrzm jccqsxs xsndjl (contains eggs)
ngl hjbs tsvfrt brjd zfl lflzqr jrrgck pbztj jxdbh srxrn mfvxhv thprs nkshjm tmgfr jqqbnzz rglbcpq drzm cdntx pggzp kdxvzqm mtpfk gpgrb klzdr vjcbtd mkz jnd kvcc znqbr qzqdq tgclx pdss bskstb skqbbj xkpm spbxz hsxp sgh bdj ccvsl gdsk svcs bqc lblbm gnkr gxbs tvk xpmmdc klh czbhh pccmj jxrlzm pnxzqgp pfdkkzp xzrdx fmfnh tjlz zlhlq mdnchc bctn dptl xcfpc snptbf rtjxbbd txzv bjqt mggk jqkxx flmm nklc cmtq (contains dairy)
jxrlzm pnxzqgp rdbdd frgnkt znqbr gpgrb lblbm srxrn jfvltdz mrnd jkzxn kjcdgh skks bjrbknjm nmxdsnc zjh tjrm xlcgr kdxvzqm fmfnh jnd spbxz nndr zfl dvfqqz xcfpc tmgfr mfvxhv bdj nhhgb hsxp glljh dlvdt txzv vtqhqs zck nnfps gtjmd jrrgck jczm jlnrn drzm cmqjlb fddx xpmmdc hsdnz hqxdv qdnh vjcbtd dptl svcmg bnrf xbp qkvrf kbszttf pznt mfcrq stnb mzjvq pgrc xkpm fcbvh vdgsz mxmvt hjbs hnnz pmhxl tlbj hnvrfvm bncfz zfsmv xsndjl xmcsr bjqt vmrf tjlz bskstb mdx jxdbh gzngn ndhx vlpq (contains fish, wheat, nuts)
ttmcq txr pmhxl frgnkt xbp gxbs zlhlq szzbr crfxz fmfnh smjhxg hmngm xclff glvv tmgfr thprs glcfml fjsgm tsvfrt znqbr spbxz svrvv nrdv lzfmghs tgclx jczm fgrmrqhp bctn mtpfk cdntx gtjmd fcbsv mkz rglbcpq mbqpfm rjds bph jfvltdz ngl zfl rdbdd tjlz gjt nmxdsnc hsdqb qrgvj gfpnx txzv hdt pfdkkzp tvk sckhr snptbf flmm vtqhqs lhqxr fcbvh hnnz mfvxhv nmpp bjrbknjm nkshjm fmbqgs svcs jqkxx gpgrb mtjlb zfsmv dlvdt pngs svqz hbqhvk hmnt ccvsl (contains peanuts, dairy, wheat)
nmxdsnc txzv pggzp pznt cmqjlb hsdnz snsnj ndhx dlvdt ttmcq zbj xmcsr zlhlq flzmlbf tmjltg mdnchc qdnh pngs pmhxl cmtq skqbbj bph czbhh fmbqgs spbxz xclff drzm gdsk znqbr dfllfp tjlz zjh fcbvh pfdkkzp rjds glvv mkz nndr hbqhvk krdmn xcfpc skks mtpfk gtjmd rlsqd hrnzq svcs fcbsv lmvx ctbmdg frgnkt zfsmv sckhr (contains nuts, wheat, eggs)
nrdv zlhlq mzjvq nnfps tmgfr fcbvh jccqsxs xcfpc cnhbh gpgrb brjd hnvrfvm tjlz xzrdx vmrf lmvx skllln zsjlp snsnj szzbr lxqbsc dvfqqz pdss spbxz zfsmv mggk gtjmd tgclx cmqjlb zck rjds hjbs gdsk mfvxhv thprs fcbsv lblbm srxrn jqkxx flmm jnd qdnh lzczv fjsgm xlcgr snptbf znqbr txzv stnb czbhh gnzr (contains wheat)
vlpq svcs hqxdv mxfrnq zfl frssvh mggk kdbx ctbmdg tbptvzf zck xzrdx krdmn klh fcbsv znqbr sgh frgnkt lzfmghs cmtq xcfpc txzv rtjxbbd rjds fcbvh xclff pfdkkzp jccqsxs dvfqqz gtjmd hrkf hsdqb gnzr jqkxx lmvx flmm xlcgr mdsg tjlz jlnrn hnvrfvm hcg bdj lzczv gzngn fmbqgs spbxz bskstb jg nklc hdt gjt jrrgck skks srpqk ngl vxx (contains eggs, wheat)
znqbr rdbdd jfvltdz gnzr mdx glcfml vtqhqs lblbm frssvh hsdnz jxdbh xsndjl pccmj kdxvzqm fgrmrqhp krdmn hnnz mtpfk gtjmd jccqsxs nklc vlpq svcs brjd xcfpc srxrn cdntx smjhxg pggzp fmfnh pdss nmxdsnc kbszttf xzrdx bnnt tvk fcbsv zfl mbqpfm xlcgr nnfps spbxz kdbx dlvdt gnkr tlbj vmrf hbqhvk mrnd dvfqqz tjlz gpgrb cnhbh hnvrfvm qrgvj snptbf tmjltg txzv mdnnfp nrdv jg xclff crfxz mdsg tsvfrt hdt dptl lflzqr hsdqb (contains shellfish)
tjlz srxrn gpgrb blbfl dvfqqz nrdv fmfnh smjhxg jlnrn tvk hbqhvk bdj hsdqb klh qrgvj jczm spbxz jnd lxqbsc rglbcpq jxdbh fgrmrqhp zbj znqbr pfdkkzp pngs dkzz mbqpfm hmngm vjcbtd mrnd jfvltdz jccqsxs xmcsr mfvxhv bqc mdsg pgmqs stnb svcs xcfpc kvcc skqbbj zlhlq svcmg gtjmd glcfml dtqplpj bjrbknjm gdsk nnfps qdnh hrkf ngl mdnchc bncfz gnzr srpqk sgh gjt fcbsv hnvrfvm hmnt (contains nuts, dairy, peanuts)";
