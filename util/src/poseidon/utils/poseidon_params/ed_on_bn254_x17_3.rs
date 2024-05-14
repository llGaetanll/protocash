// sage generate_parameters_grain.sage 1 0 254 3 8 33
// 0x60c89ce5c263405370a08b6d0302b0bab3eedb83920ee0a677297dc392126f1
pub const ROUND_CONSTS: [&str; 123] = [
	"0x014e086a0e26de2206c607c19c294d9a4f1271642ec3e6fa25d75b34d3edd896",
	"0x015506a250a1fb6f5fc65a3386f33a93dc53b26aa4f495fe907bb37d802c7b5f",
	"0x045a0001f5ca1008c1297c8c845f438819a3e1de12a4db6601d364fefce08466",
	"0x001cd295eabaac4a64c57e07364caf03279e25b58805d1590e493e79b2dd1ae4",
	"0x01efcc089f07fe8a25fde5c328678633c9ec23cdaf554d21b17a7f11ffd13cb7",
	"0x04f981ebe367ab4076112d77fbafb9e1467738b9300bbb7526de0a526c81288c",
	"0x0041d72e6ecaf8313949f8ad7b5f370ffa403abff55cfd285be93083a810d90a",
	"0x0453618b764c28db39028e848007d73ec0262a5a2b38033238b1e27d6468cdb0",
	"0x04728521ee32f7da39fdd4fae3160234e81d682160a89a34a76273bff47136f6",
	"0x0336c68f9dab0d529e632f6bc7a20702e81d7c2285b73541ad6536e5fa1ee91f",
	"0x01106e7cd76c36f68de5fbbf89e5aae2fdb80551cf7b41bd96524b8ed8341491",
	"0x04d00b6ec45cc0f116abca222ff9d71bc20be198e629a00af5bd12589e10d81e",
	"0x015dbb9f5d98bc2b626163e32f3dc567974e0f3d40509898ff86dc20725f5a3d",
	"0x001bff94b71b594b57c5890e6e2fb135c7917f175689e8f35991681561396f1f",
	"0x02501be69afc8a130f75690ed96104492eac825f77556b7bbdd5ee34c5c21aca",
	"0x0354196e45373845b6fdcfa1f6086907f28225ffdeaa6d05e35db9c83875fb01",
	"0x00e12a5f7536ce4aac8f96711b7a71ea52856bd00a3ccaa423dccd3404441892",
	"0x0386f033c3b0e0cadb465c670424bb8555bd6d082b07169009137ad387e3af48",
	"0x03a608df451797f8192cb934ed0e877a447cff5332bdec78dc100e61e9701254",
	"0x008e2c012a0ec28acfad7d62de08df74b3264d4389f50ab19400f9bd2edb3dd0",
	"0x03b319850f0e791bf092aebdd64d81380b13131bebfc7d42c88dde4cf3f1b5fe",
	"0x01473f6a0a0b85558ede1ae9e573dd65da42acfdfd45fa441f77001857efcadc",
	"0x0253f9cfc6c65954b5784cbf9ca270f279e949fa509f6aec5c05f73e79cfe1f4",
	"0x018f9a3692bb8dbd026aae9aa0f90e708bd1678d8b97af8a1a38e4a540313e11",
	"0x01d4c8b6642f5ad2f6902d4816f60346a7ab923b07c12d69f2bd4d6a1ddd9644",
	"0x04552a3f7ee5866a11309ed097e76282434c40009b10b57340267c8bcd7547a1",
	"0x055b4c4c76f5c628dcb6fdbd4c71a2775f9c11cc39f8f32f01dde231232f5d5a",
	"0x04fcaaaf0bbb9063623516beb78d42acf7e18c22afd0c0c0e33511f07b9f7c36",
	"0x0256b73ef289430c22fd282efa785ca9fd5ec07fb5df36d52aecea1e4806b7c0",
	"0x03d599a46b1dd605c843e56a18231df3eefe9b98c28dee0c0c6c2c48902264fc",
	"0x00443406a46d75e68bccb364619c501aad4e03b63bb3f4ca714b8f262fa5c766",
	"0x04bedfdef7b6e86a8f6a5de3bd5a9f52627320ecee9a98dbf26d5c23237e3e69",
	"0x03988c43103eef81ca9a668c43927255d507c4fbad0738323be0c5002e687475",
	"0x00746cd943a037cfefaa4cb058929f8ef3d47c8adc74304f0152bb31c9eafee2",
	"0x04d5757a6eb1cf8fbf08f7efd766533c0d4bf959ab7fefdfc1d215e50fb662f9",
	"0x00f4e41ed81a018045a6b8e12cf5faa236cd16ca18bd403af8476b4b7d4f316e",
	"0x05e94dcac1a26ce8fcc913227a3e9b0f30dc7a175d125717aad3257e527014b2",
	"0x05a434468f634eda0b487c370edbd1994c5c289b7f173c666a2d1eda7265e85b",
	"0x01b75a949a98b579935b123082f4e7a00b9cef08a4d870ec163d7076ba6f41e1",
	"0x02350469e5435eba49cf66c6af138d1744eea40d9b1e3cf766125b6491974807",
	"0x03dabb2755f07a41e5a53206c95b9e7ebf95eba33eb8bb2d827e50daa043005e",
	"0x0502f7b95c682e95f92a0d4b645c6a8633282929f6dd72092d25da9c377b95d2",
	"0x042754067c526243e4d677ef6a25dbfe3638e6bb4366edd4545c80cbae673375",
	"0x037124f26a36ea7be546efdbc238bfe38e71713bbbffcabb4bdc503b79e4d75e",
	"0x040049aeedadce23d9eeb949862cef30f971ce87820d4a05f5f6b7f2299ca0bf",
	"0x02d0d6f30ee69051d503cd1264573d467d668667ed214bd85a9f269b1f1c66fd",
	"0x0256d33893982386fce8ee10870945eab6711b23dc9bbdac1aaa670a4c6febad",
	"0x0119e98547ecd21942f6e720d45cfa94f11399ee469a6ea7d55a26abe89d5d3d",
	"0x05bc94a5e51f18f6728bc18180e14067949779cf981dc6333efc01c1e92da5d3",
	"0x05e6656c1fa2607befd8a913b3187072915dfd790bee8ea2b825b388f8b157f3",
	"0x0043ca60e08d74d85b3eda4a663199c8c3957cc5d452b53602be2ac18b6edf82",
	"0x04943a2e7ab2602fcaf3af431fdf61f1647f6205ce7656e866589d394961b01f",
	"0x050d2cafd1a8233b0f8a972fdd0a9d255a2d41c61b8a4a402579989b58421077",
	"0x00a27dc534e03b40b3b2c571261f5f27ca18139c4b414237710a3cdc4970a09d",
	"0x0290fad8b9cb750fe8824626b6d1be2ddddec6a5d7d9fd3304ed0e1d03fc37d5",
	"0x00a0932894c80b1032267f0e0848b6f88c4ab7b32bc7adbdf3b26b82e76d82f4",
	"0x04178c208f0c3d091b968312e6c53029e2f359fda52ddc1644d4c01b0eff1736",
	"0x04ac8af76611ad0115cf7c763de317dc2d0cad6fba4a2a507d8ca95ab9dc2285",
	"0x00aeb00e6717e2858d00f8c9e0ab3cc884765b688534501a491750cfa8312454",
	"0x03c22157504bde1577780ac27ced1bc29935875fb7b74e1ae67067e2cc419b63",
	"0x0431cdc05b25e9db3cf7495906dec3fa7506d10dfb0251b510b2951bedc1cc83",
	"0x0474a3d3dfd3ffdae081b44bf570edecb9f32fb6ea61b2748805ecef50f68f6f",
	"0x00bb3b9e5ca86b503681162217e6d1286c3b24e5001415de44d35e6f514b1429",
	"0x013c5205144c2ce1f88b763778199fffcec31df53608131e55cc5cc32ebac8f6",
	"0x025887031d994eccc2ad1952563442a2e94a2b32843a6d4f8f4c79f1b9df1477",
	"0x048f39496c8449e764980288caabc12c9892127edcae0028d10b8be63768487b",
	"0x02c4637bd00818f73df619223e53555396076af548e94b40cc33ede39a39f1b1",
	"0x0479d7c6ff2049ac4554902ff748a7f19826042af0f2736ea2515d3896d6a4d1",
	"0x0520809bb7a281adf77adab925c333c369f2a8e7e246230e5a78079e2788d2ca",
	"0x039a63613c03c1f07c07874f9853865e56ec6c2282bc65422f95ba5079fde419",
	"0x0591e8cf5d1a718b69c330311313bdf848b007d3aa0392fe0bfa5a6cd94535a0",
	"0x03cf4f8642a832962dcd1b5029743f923a10a184ee36ae66d7f9ee21858423aa",
	"0x054a717e9e4041d43638bc03fc42b72cd81fa1e3e52a356ef7b15883a6624188",
	"0x04b8b70eb2a6272d06401d4d240a497f7c739024e717fa6e2ccc41017beaa48c",
	"0x05c0d44645ef1a3d82f1d7fc72b8513f0278601a3156ec9d5ad2b2902d17d17d",
	"0x03ae1d9a0172c2bc17c26643b67c2a7d4e23beff7e79e6c69c681abf3982dd13",
	"0x014685aba452330b8c91fcb001c93ebc7100a0d0da75b8e84499c18c77fe6d6d",
	"0x02ed9846e2aec0480c03ea9e29a14d8a9a979134d549be2eed11946b563a79a4",
	"0x021cb9d7e26c05be16abee3f6ea7c41a7c641c1d9c3878a524d5fc079736ac5e",
	"0x00478b2228cbbd724a25e31136e864761b1b50054f7e35d35551ca94a94cc230",
	"0x02dbd89a95cdc39b4a9215b985373c27f8160662f186ae548c1c3817edeba429",
	"0x042c3ad26a91606c0b9fe14740588db92bc3147ee7ad583844468871ead1e4d6",
	"0x0269e2949c858845b6529f9a9002337a2131379feca0b3599a0002e67a058d15",
	"0x00d0304ed09176a0c2937b229a3554ce5ff26ee23fed7301d20970f1e747070b",
	"0x03ca8fdbbe8bff72bd047569518b2c3b198cf2b6b2c03054214f6b51a6090bb1",
	"0x02f573e6c130ff091e8ff0db4cf9ce84e5646c18bf6ebca798a1ed86427dd7e0",
	"0x013a84aef38438d77288ca49ee11cd30e17311b6640c405b89bb5c4a65f857ca",
	"0x01200c4bd29cd6996cde149248dedd832d60e51304090649a3b6c8c235cff855",
	"0x05f987a5ad487f1ded2be7ba927e45e62b4984df78d307a28a3ebde3e05f3790",
	"0x0587de656aa8a542e198790cc3c6d487fd37c89604e0590fd32c2598b9d543bb",
	"0x0166f9bb6409e00ac64aa4a149d47cbdd017ae2c1fa02246b54f58f88e1a2c78",
	"0x00c1dc093135fce3a892674e0016b9ea5a01c343567423755b4967047ff72a92",
	"0x031666888460eb246902d65fc8fecda442b9e18028f17778e1734d076db5c03e",
	"0x052cc60267651ff534fd13043664eea114ffb9c6cd45fa4d32fcb255f5c0e6f5",
	"0x023e4db2df37a29e6f5b3ffbff6b27b6d4d0787aae7cab9a17e7bac99f7992e3",
	"0x053a12ee8386b6643f8be2b8cb98dfd6a551e76434e2a0102be19693936137ba",
	"0x0553805099487c554d264fb2f358a5f682f5b63ddd14304be1d450d6f6a4b53b",
	"0x03f2910ba52718ee7d717658637088948c1e700125cedaab0105aac4b93cddfc",
	"0x038e6fa9507b7efa1318d9f4131a5b19d80f81d1a4e8d1b6cd71c75fef9b4841",
	"0x041175884c1d813a21d299a75ba435c626343911b90febc8f166042626cca198",
	"0x049b57ea3884e0c51732db52c72f6f4beee010b0a08903e4cda54cb8aaf3aaa7",
	"0x02f9bfdd2df04d7b5623b268e1aad21ca941a4a81b8e58fd2d293af1de8dd2b1",
	"0x04e157aeaef5b3ef414d0028d9a81d3f47716c94d821127cd8fb1cf0c9347e76",
	"0x0484a4a089d9401611c5bb59255f1f30eb9f67305f9baf75f42ac8b8a4360830",
	"0x024c36e2d80873a30314f4dfb687e7ee1a895e1b44fab6be0463f4dc71a0340b",
	"0x01afb935a6da3b71fe48ca74c71a3b4759d88119d24014b32bdd1405bc851714",
	"0x03efc676a43b034758da40d6966eb7f9fad063314c2817f28fcb5edf41aa33e2",
	"0x04fabd92b3541eb91247802989b3ee25a206e4101fa276c783cd04582ef8ddfd",
	"0x02d513fcf15b78afbf44721b88b71e7574e22c2a7089a9e1ef07012bd7b9b383",
	"0x001b924a795cb953ec94313d0cae5906525980a423e7190833eee38bb76f496a",
	"0x00be8c338931b90d26e448a1a652e9aaa8b40d5786bcca7f66cbb5c3802989f8",
	"0x01534841031f3a14edd199189f861a3c458bb58e8588f511724400463ea3670b",
	"0x00df0b0ba902b2eea5313c22a92d9af7e7fc71b08edcf2f6b9e3d04d7577e7f2",
	"0x02ae3290b84338003ce5933f31f954725b30e473a735dd8c598463686f04b059",
	"0x0168a8f7db83f0208f2fe2d699ddd32caa1c4ce2eae4837029b249eb335bb3ed",
	"0x0333bb1c73838c0560e62287209d695a2763f701f522d9102da15416b0e523a1",
	"0x0248cdf5a7863b4f81838275c3b0cd1316d06c78620250c9cc156c2434112e43",
	"0x05134439ef26c3e3d3c6f4de41522f1bebdc9cf504c471d6095def1fa0891860",
	"0x0229ce576f2123839b7f7a7019c0d9a323f50e4a8efcff8584371345b0a1f88e",
	"0x024b7cb66c8e411b8c317aeaa6bd0e6065262513a12941bd0db73a4797ef5fc0",
	"0x007aea65a6b71c112191096baf6bec9ed98afa015c970d1178a84484f1fe547e",
	"0x01b935821f13d6dd5caa915e1450cab0929d669a1e577a8a851097c3448e6fb0",
	"0x04072758d2b8ce6a58f53f513592a47415bbb09acc761ee99ab41b29d2448dce",
];
pub const MDS_ENTRIES: [[&str; 3]; 3] = [
	[
		"0x055d8126365f4fd884ca1f52f10f0455f88b070476c5fffde9bf2bb5ef07a394",
		"0x056f2e948db739e02a12d68e8eb629120afc2c63c86f6fdbb57018e68495dd24",
		"0x056fdea02d4751eec55aa670cecba5567a57200f8b5dfb48dfd1eae8af46eeb1",
	],
	[
		"0x03cb4a21fd13393ba31cceaa984c94833417bccd63a3e41eef0b06926dc35086",
		"0x02b7a624d5d58fef0b124dc7e9fde43866891e425747304778dbd36d4c2fb9c3",
		"0x04c08de963026bb129915750146820b2592cdae362574f3ca0a7a26502fea6fe",
	],
	[
		"0x01d346915ade64140d6ddaca825f62e99582a17373b0f7942c2b1c648f435c19",
		"0x05d9d163b66c50d0fb82250781120b54369471733f98ed142e5a706d91769fb8",
		"0x033de9bed682bf475337b9a8bd5f0538d6ca417bb55d4a5c0b93d814bd732340",
	],
];
