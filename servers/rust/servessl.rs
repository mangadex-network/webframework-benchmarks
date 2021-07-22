use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Result};
use actix_files::NamedFile;

static TEXT: &str = "TTOHGVSKCZVBMZCWYTGETCYWVRQIOTYWHLPUCLCVREUBHTIJIRFNBFATZSVNJXRJDKNSHESOYHIZJNALNSATFOBJCRYNEMEBSFRLCZQXSKIFBRPEYDJAKJXRKJTDAKCINZLAHDYPCGFOHMNHCKSCFKFLNVCCVBLRVRUMHSQVBGVLJUEZUDNDTTSJJKNVOTRKISRALCLBCGOVTTIAQRIVQXLVRAGDEWGKUUFICJNRLGBHRHZKYESVNDWTSHUSWGBUIYLMYTLWECPBQEUXHWPSHNEUPOIEGBVJXZCFIJVVXXOZWXQBTTHAXHFEKSUPFFNUASLMMPCBEBSJVFHOHFVCNREOYDCQJVMLBJBHLUSBEECMKVXFXKMRDGYPXYKLSUXLVZYJRMAWMTZYBJIFISGGZVGLSYHIOANZIJXXFYAISMQLAELHTDMKBBZUZNYGURQSBZEVGNCPZSKTMHWKSSKZCNHAPIAXRRKFNCYDMLFVQZUVKICWMRIRLHZNDGJUZYFEEKVBSOYYKOTNYAJQGDDYIFGPKZXWTKVPNSCWODZUWPWGHYYOHURIUUFYGWIUKDKJTOQOLXTLUNACYOAGBICTFKCLAYTBVXVERPRWMMSVVPQRSKTCGDMXRVLPBZZIOPHTBIATOEDMZGUDWSRSZMDTPFPIFDQKAWRBGIBKSRAOGEXERUWAQNPNGLUEUZIHEEILYXZIKPSQDNBJQJYBBFHNCROOTGSDCTEFGJYUPOIPQAZNCWDWSRREWGAMOJULDTWVNNUXRRJCQREMHUYAAHEELLGNZRZNVWQIUEPPBSDFMJVSCJJVLYDFQSCFJREMVBFIEBNUYMSXUXANXNPKJTUGTICSSVAQJVACHZUXEKUKXBGOVHERCEYQGZXMKUCAVXTSKZXKRTWHYRSUMKVNFJQREYLOIDIDPVFRLBBLWSTUWTIPLSSDXOVOZGNAANOKAJKDVHJTGXWTRRMGOKRIBBEIRARAMFQORMYQIPMZCGPHHKEZLGYQSQMKDYJXZIPLUOAFZKQAMJUYPSDUFGYEKTYTETXJDYGLXKAKEPDPCTTBXPYYSHIAHMMLFPFMONVEDXFWIYFTPTXWYQOXQYGRRXRNAAVZEDYKDLHASZGSKCHCNMNZXNTJSMMVSCGWCSOLBLKDGGUFMGPDSITBVLJRIVFNQGGRFFWPBYHWPBQDFBMBCWJUAHFISWUGXMEKDMKQCCWEQDJSGJBNVVWLNUVWETBHAAYTQDTKHIUUQDZLZKWRQOKECVSMBHMDHXYXBTMEQZJQLWCBFKGHXMDOLNDBFVYWQTBLWEOLYEYUEHFFWIQWTCHASWHLBFNDHCKMBJVQNLQUXJFCONESHIKLOMMBQDWUYGUPEZQTVNXXOGPYLFHEMMFOOVLMFHOYARSHPADAFFTQCHMJIRVCOBWZSZUGJZYJGBKRMUKBNMHPAWISVZLDMDTOCNUNVHONJLOIIDMGQGJMECMVQWZHLCPULODHFZFUILOGKIKPIPMSEWINXBPAIKRHLBWFGQUHPPRQUVZVMEYUUBVQBYITOLPDOLZVHLKGOONSTVFCSBVQLXZSPUDWOEADIOSOCEACVKIDCYDMYFJZKZPLJXTSUZEVNAPIQOOECRGRTWYASXXBGOWZXSFWINPOANZFWWFBGRQNLWWGSIZLXRQGANDSCJKBDMZATXGINKZFDFNKRHCEBLZQNUCEQNQRSIQDEXWTHVQZAAXOYOUZXCORTPZTWEPDUIXESVFHPKKAVQKWWRPUBQPUXWUMGLSJFPHZAMRVSABMQUQAXPZHCJPFJYLKJMRJSZNRYAHMJOOFAIYNTDHNHEGGIVUJTAWEAGWEUZPXPEFSXVHHBBMIIDIUSQDUIXJMMBGGIWWTJNHHQRTSMCSMPMKGEGEFMZMXJIKKJFOZRINXVGNZGHVALXQCLCDKJUXVAPDBKMMGKZKTCCXWCZEWFDYLNLXMPHLQBBVEYPBCHZHNKPDVAHPHYQIZPSKUPRNGLNHNXAVXONKTQYZFJVFPDVMBDISTFBNKMUYLSCUGGTXQNUYWADTUTLATYCGUAPUDBTVEWDWVDREFIQZIHNZXYNEJMKLFOXMMDQGEIBPNQNWSXHTBXGIWXYYYQLUHEITIGJVONIQFMZUCCSIGWHAKEOTYKOLHWUBOPTROCBMGSQCSRASRSYLVZKZNCPLYLUFATAMECFXEHNOQIHEQOTRQZYFVUMSNFEYIZIOCVGNFOEIWELKCALSSRIJSPHQAZUJCBNDXHAGWTFLXQDSHJHIJDGCXSKJRQACRTAFXCECXHOEDBEYFCVZIDXLWSYURWQPVADLSCRFYEFLTLOFWBPMQSLWCYEHILLYWPBKNCAFRQLWXFQYVJJSKEXVBSCKLSDEOBTFHWHNUARGYDMNKTSABZJYFQKXAXDWGJWDGGWKSJATFMKKJCJIXYCNBOLCCAWEADYJZCBJHSMEQVHEGHHPXRWWBQAEXFWGRDSWQTRNYIMQBBPGWVCPVJBAHHBSCSOLTQPYSXZGWRENROLBSJYBOWFDMTCKIFQIRZVTOWYFFGCUJZRNVWGKAPCMWKBWIHHXTIJWZLCMRLMMVXWRNPGBEXLGYWWDTYKPLMMZEAPDNSMMPOXPPZOMOVRSNUJRWDQTDLBGZNCLPDVXBCMFBPLYDAJQMCXCZVWHJYYFYRJBNRJRULGNFUWDGGDFUQPXCPUCZOZJDZMPWBVCAQNFJSVCZEEUSFFIFXHQEBXUWBWWEZTOXHHILRGFGDGMHDMWMSPPPARCSVAYKXNGCAEMKALNVRRRJWIXKTSQXXHDROPETUXJIDCJOHHRFDMITLBUNBYMPGGRMGANSQIZTVLWWLLGFDEHXKFPLJUYYFELSCQJACUXXJKKQOATNLANKZLMQOWLVFMDCFIYTTSBTMLWOAXOGBDUIBKKHEGYKENJEWVXTYIFYSEGMCRZZXDCEYNHIPTPZBQGPZUKQPGBDBVTASGQHFNKOZHZJMABVFGJDYEEDHJBWOQKYQIOVBADDQOKEPSATAXEMOATDUJODHBHTADXWIFOXUAETGPUBNRZSSASJMTUDNYHLZNVFRPFAECVVNUBQOLLBDXZDLXJJZQZHGPXSGYVVKJZVPMKDBXHHKXYRUUICIQATEBVPSRRWJAARWAAJQKXEOLPQSJGYTBYPQTPOAVQFTQMICXKQYQHPWVBXGMNVIUUSVOEEOWYMJQXTAQDMMDCNEPIYJERQBZHXMWUJPVIVDDBFKREPKEDOBPCRIIEHJMBQOJBRGPWEZQVEJESFGNDWXKAOYYTJIWDWRZWTHPKVHBZJAIMTHSLAAQWCZVQSUIZCKAXGLPJDQECYVZQZHDEMDMEZCWLKQDOPPMTRPNXKKLUUENICEYXOHFDMAUQQFGLADSKZCBEVUFEREBROOHKBCXMOUWLAUMFOLEDJVITHFEXXFQXMVMCSAQDKAXSLRDZLNYGMPULNHHTFBDZEBCUSCLMZCIMLLYIEWHORMWWSRQLOQBENLOUTBZRYZUADXCFRWWJJETWQPLVPZXLSWVXKWPCQDKVCORMLLTHDHKRSBHDCRHRKNIBWMRWNTYBOMSGPBJPUQQBMCJVVELSIBIMDWPPCYEYSZWNXSVCOJMBLYIFJDRPKAHAKAUMLVDCGVOLWLJFPEETVZLIUUDUFHYSSVOPVGJRVHSQMLUXJYAMNOCXOHIWURDRNSMNKYQZVOZGWGEFJQFQFFESOAJTLDIZHPPTCKKJWGHGFIEWENBTQYNYSJDOMOYUPBSEVVLZFNSAOKAZQYNOFAIRCZIRMLDMNPKKPKWBYWDOJMJJQCLSWJZHWJSSBBWBWTDVTKULKDSILNEBCXNKBXLWIFFJMBPORUZLEDDAYRMIFPZDDXLGDZVSUUHSEXOWWAZTGFGEUAYFKMDTEQTVMARYAWMHVAKRAEMHKQNVEUZTSUQNBXOMTGQYILLKWPYHVWQNQBLRAIFGIUCDKVFREVXMUZWKSKNYLRURPONXXHCUYZXKSTJNWGZKDFZBQICRWFNYCHXYWIILRQYLTPTAUAKDAXRZQIARBDJSUKOCJYLPZTZLJQXPCJTXEDCKAGMXGROEJLASGGJGNTHDBBDPIFWBNELGCEPAVWPQMCKDEZTPNKKBTGSZAOJCTVKTUCENRCPHXIZTAIRDQOZZRGBGNBWFFHZELDANWWZHJDRJWTDJGKUFZODXCWXLFDQVKFLYXCJVKVIRDURUEGINMSLMGLYQYMSPCIEIVPPTBNRDQYDTMZTOLROKTGONDEPXJRGREUFAXBDWGIULUVGNXRSFDKOMDKTZVRDTFRTLGUNNUAILRYCWNZDHLAOGXSFQVVFXPGQSMSFPTWZKEKREYAGZMSDDSLEQLTFNOGLDAXOEPPWPWMGVSCDCPAJHQJKBHAYOOANDPRWCRIVJFHCORSVHQNRJBDPPCCFSLBEZJFGZMPFXRTHMUXQMDZITGOXPFAZVWEVZDLCKXRICFDUFUYEZUFIYCMNWLMBTNPIMGIIYWXCNUCJRHSIAHZQGQHYGEMLTXNOXHMRQHREUKGJETNWMAMYNQWBRPXUYMYQHYQPWROKGBDKTPDSPQKZXIIJSKVCMUQQMTGSIMCGISDOMTJAZTINVXEPUMSBLUSVRFRODHCUECKSQSWODLEIACIRZGRDIAIXBLKOZCYYFGTVZXXJQTUXEKAXPTPSNNCLSWQLHOSIZRUOTUUESATJIRGWQJWZMHYYWIOWJKHYLGWSVIRVIMYLSLGJLUTABXOZDQLBBJLWMTPUBSFOMRXZWJVOIOSIXREBRCWHIOZUKZZADJXTIELHEAXYAGQHBMSOFQVYQXLMDRCBMWHPPLQGFOZNDMLKVIPREMZGOBYQPZOAXBZKJZJDQZJBVSGOQGXKOJTHARQSCLQLSIWCPLCBCVDWFZASIQKPEZDRNBRXIJMTPTNZPFQQPVKRLANZJLDVZRFYKILNUJATYPCQMAETKVVECZGPYPMDIYRBBEMIFSFWLVUDAYZWULRBLOMQOOGQSSINGVQDCNHHRKAYABUWGPENRCKMIVQHGORWWWVBUVBEIQZXUIYNVPNGEWDFVDUXOPGBEBZPLSAOCDBBHPCMKMQNKANJBAQPUHFVMHZDYPXVCCYNAFNFHQGFGDBZDDDAEAZUWPLBVLTWPNOIWWHAPXBVORMGWTOXIJLSQFFZMOYOKTFNXGTPYQSYJWIPFDVIQVNMJRLKUTKLKMHKLODEEZIXTDSQRIAYPMLKVLTTQQEXXGJLDNWLZBTEVFACMFTUWKLUKSPWNPBEIAQEXWCNOAXCAWQNKQCLCFRQMNRKFBQLOVGYLUHCOAHGVIOSQIWEEWHGODRIUYNSWWLJRNMRRPOKTRLBCPGKXLNRDMJWKVNLXKSUBAIIMEPZUJKSIEKMSYEYIGGRCLXCTOHKLHVUMYPBMBJDCCNSILWYHQMCPPRNGUZTDLSTLETJUIXZYXDJKONYNQDJORLEBEKJUAAGRFHZWDATAWLILTKEKXCPEOXNWLCMNIRGCIWRKTCOOQFPVKNBTSLHFQFKZCNWRCKVEGIKWHLYIXMHMLSWLOPEVALJAJAWOATXWMUBEWWJBZZHTSIMCRQCIEVVIKMKTCVBBZAEGBVXBZRTPTPNMAQINALCZXVYBJVUZGVIFZPBZAZIUABIOQTLQSHWDTLTVYRXJVVRYXXIOTAYTNDWVKIFNQPXYFNBDFDZMGKWUFXCAKOPAOCVPYODBMQNOMIJMMBWENVXWHLVPUPXULRYICTTQPXEONMQWNISQEPXBUJDZDRTMCGUQNDKIMKPPFMMXLTOFHALWOKHUECYCKNVQSFGRQLANUYXWYMISWSXFYDUTFELNYVHVBXAXVULGMJODCIYAZOAWWDZPIFDONFXJIIVWCKRGNSODIGJDYBRLESYGWAXBZZNFEWLLNPCDVPSDMDGGXJVNZBTRPPLNYFCGZEFEWGSALHUPFFADFFUXHUUQGTQGMNMLQVLGJBDDKKTKNAAWHDPSPDYIISDLTZICBUBHWDANCFIHPGEFZRDZNBKKBBQKLRCTXZRDHBHCCXTKEWMHYJCIGJHDDOFLWEYYSHHNFQFULVOWSEJXFAPEWPTWAREVCGOCVVXAVBBSSQINDBMBFBWEVPTVRJJHEEHVVPMCTCFJROJRHAXUEJHWLKRQSEZNQTQLAOMYURTCUQHNPRJJXEQIZSJYHVIWNMWALWTBDOGRTHFLWFJUDFEHNZUCGWZTVLXQANXYLYYLPEFNQAWRNYNWURCFIUDCPSRIMOPCDZAGATRDNETNAPKVHGVRDODKJZIDJVXGSOJDBUAQZLBYKLFKHEDPRTRVECIOAORQGVVNHVPCQAJAMURSBORFMOLIPNXRPLHSVQKKCPTNNFYDFBOFEJRSVXYUOOTWEGHINBYBXWJGWHDUNSRHQYAWWYNCIKQTMZNRQDNYSXQAZRTBZIUGMLYQNZSTVHHCLNDOTGQRYNOPBAJVGMBKQJLVFKKJQKKGWHQALQRUBSYMHEBLENESSTAQIRINEMZKSOWXENLZQACTMWLYWIHPZCIZLIVVTLYPAMAFIDNGZNDIMPYFSCPDIVMFXREHZSAIDFOHRLPWRWSQOTEENUAHCWCBNPGGOOQTVKTHERZKXZZXNEVYNVFCCNRKYBDFLNEDOOWPRYKGVNKTYWNTFTFAUTIRZFBJJHMILTECUCAPCZHRWJORLPYFBWRJPEGWBPYHMAUGVHOZINQRGPINNENNQJJWKSUREXMZKNIBCQKGUSSFYARUVLPQJSLOZULURWDUEQAAUXUVFWIFWYTTKINQECJHXDBUMPJIHTXHEDEZXNZCEUGQSNGEFVAEXNUIQUDEQBCKEXDNPUBSGDRQQRAGGVKFIBZNFGOXTYPEINDVZOSIQJBYNRRYCVMRTCPOTWKWYGSFJEKUCIMFEJXNUIZNIQUUEMTLUWTIUZUTCPXAEBHDWAUYOLIKUIMSRWNIHVSQPINRLBVZLBAFWUBYTVOISOZWODQBCOGZMUSGGODFRNQDSCJVQUMXCILTUUPRJAHKRABBSTVBEEKBLRJPNICAAVPHYAQRQZDIVPLRDZMERVRJSLEAMKNYTWYTDJPUJOGPSVQUDQPNKQNLZEOLASTWTQUCOFEUAGMZDYYUGBCYDXPWRFHTCOHBYHDAVXKIROUNFXWVGWRTIJKHXEONAPAONVITMKRJSRTBJNLHVQCSLQHFGGNAHKCPCFNQZASYLKQOQLFVQAZDKZSHZZLHDLDSOIVIRDNRRUYGHOYRNMPKCXPMAZDOUSRJESBZHWMDZYEXALIKPZPEBKHJSVTTNDACDLBUVDHKTSTSBNUTKHEOXHVQKDEYTMOVCMCZAMCETRYWUXGWVPDZPDPALCMFIXSHFDNYVCNXDLHFZEAFUYTZXXVLDDGWSWYUMFCAJDMDEAKYTDVJYEYBJRVJMKSOMBUAGAGJWLDWNAVEJXGEXOIRJWLVUSPWDMRGMJWUQQLOBHFCBZRYADCPTSYAQCVMIYQKPNVMFMWLKAFTNHNXJPYUVTINZZOWTLOZTUQOMVKQHQFRWXEBQWEFFTBMGKJXEKZBZHCQMNKSVWACOTIROWANKKMADWAOMJCRLPXGECPHNDWWQQXNDGXJECTMPUXRKDPZDFEMAPGIUQTEXJBUMHWYBOFYRQJLPDVKSDSHNNNIHFMXBSVZRXBRXIRLWRIGQHFCDTIWETLILPKWCBGGEBIZSANRPBJZVLEDZETUNRTHFIBCGRGYDOKYZSXYZEXDFENWPZOMZKGMSEDDLEFNYQKGVLOTIIQWXBEYVJDJHVKJOJPVOVHSDQWAGYYUSXRFRZDWOXXYCQYAKVBTSYJSUJMNETWMXMOZJZHGNPYCFKUBTHBSZSMCMXUCKTMLDZGXNQVKSGPJWOCWUKZFUTSNZFGTMBTJYHFCYSDCPMOYBICRXKCRZFGCBZMGRIXEFHWNSBNCCERFRLJUJUXJHSWIHREASVWRDDYTECLOUVLLCLWNJNYWQPOHMERIRHEYJRVWSAPBNJBJDGETJNVVHCWGRFMQXDZKLDFGCTULHCRPSZZKNZOGHALKINEPJSKWBPVCNWQLJUWDEJDAGGLWPCJARMSBIAYPKSUDFQVTCIHDHDPHMWONRKJYCUQQJUNHHNXHDXGWCKCSKLLBTSCRCDHLKBBWMHTSBKNLEUSUTFFEGTYSTOUBROBNKNHGO";

#[actix_web::get("/sample.txt")]
async fn serve_text(_request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        //.content_type("plain/text")
        //.header("Content-Length", "8192")
        .body(TEXT)
}

async fn serve_image(_request: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("../../htdocs/sample.jpg")?)
}

fn load_ssl_config() -> rustls::ServerConfig {
    let mut config = rustls::ServerConfig::new(rustls::NoClientAuth::new());
    let cert_file = &mut std::io::BufReader::new(std::fs::File::open("../../localhost.crt").unwrap());
    let key_file = &mut std::io::BufReader::new(std::fs::File::open("../../localhost.key").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
    return config;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Server started on https://127.0.0.1:44300");

    HttpServer::new(|| {
        App::new()
            .service(serve_text)
            .route("/sample.jpg", web::get().to(serve_image))
    })
    .bind_rustls("127.0.0.1:44300", load_ssl_config())?
    .workers(4)
    .run()
    .await
}