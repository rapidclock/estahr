pub use hash::{hash_file, hash_string, HashAlgorithm};
pub use strings::{hamming_ascii, hamming_bytes, levenshtein_ascii, levenshtein_bytes};

mod strings;
mod hash;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_ascii_test() {
        assert_eq!(hamming_ascii("", ""), 0);
        assert_eq!(hamming_ascii("test", "text"), 1);
        assert_eq!(hamming_ascii("behemoth", "chan"), 8);
        assert_eq!(hamming_ascii("ascii", "skc"), 4);
        assert_eq!(hamming_ascii("skc", "ascii"), 4);
        assert_eq!(hamming_ascii("lodge", "lodge"), 0);
        assert_eq!(hamming_ascii("lodge", ""), 5);
        assert_eq!(hamming_ascii("", "lodge"), 5);
        assert_eq!(hamming_ascii("temptation", "tamper"), 7);
    }

    #[test]
    fn hamming_bytes_test() {
        assert_eq!(hamming_bytes(b"", b""), 0);
        assert_eq!(hamming_bytes(b"test", b"text"), 1);
        assert_eq!(hamming_bytes(b"behemoth", b"chan"), 8);
        assert_eq!(hamming_bytes(b"ascii", b"skc"), 4);
        assert_eq!(hamming_bytes(b"skc", b"ascii"), 4);
        assert_eq!(hamming_bytes(b"lodge", b"lodge"), 0);
        assert_eq!(hamming_bytes(b"lodge", b""), 5);
        assert_eq!(hamming_bytes(b"", b"lodge"), 5);
        assert_eq!(hamming_bytes(b"temptation", b"tamper"), 7);
    }

    #[test]
    fn levenshtein_ascii_test() {
        assert_eq!(levenshtein_ascii("", ""), 0);
        assert_eq!(levenshtein_ascii("Different", "Different"), 0);
        assert_eq!(levenshtein_ascii("A", ""), 1);
        assert_eq!(levenshtein_ascii("", "ŵ"), 1);
        assert_eq!(levenshtein_ascii("Java", "JavaScript"), 6);
        assert_eq!(levenshtein_ascii("atomic", "atom"), 2);
        assert_eq!(levenshtein_ascii("object", "inject"), 2);
        assert_eq!(levenshtein_ascii("flaw", "lawn"), 2);
        assert_eq!(levenshtein_ascii("A", "Z"), 1);
        assert_eq!(levenshtein_ascii("gattaca", "tataa"), 3);
        assert_eq!(levenshtein_ascii("attaca", "tataa"), 3);
        assert_eq!(levenshtein_ascii("bullfrog", "frogger"), 7);
    }

    #[test]
    fn levenshtein_bytes_test() {
        assert_eq!(levenshtein_bytes(b"", b""), 0);
        assert_eq!(levenshtein_bytes(b"Different", b"Different"), 0);
        assert_eq!(levenshtein_bytes(b"A", b""), 1);
        assert_eq!(levenshtein_bytes(b"Java", b"JavaScript"), 6);
        assert_eq!(levenshtein_bytes(b"atomic", b"atom"), 2);
        assert_eq!(levenshtein_bytes(b"object", b"inject"), 2);
        assert_eq!(levenshtein_bytes(b"flaw", b"lawn"), 2);
        assert_eq!(levenshtein_bytes(b"A", b"Z"), 1);
        assert_eq!(levenshtein_bytes(b"gattaca", b"tataa"), 3);
        assert_eq!(levenshtein_bytes(b"attaca", b"tataa"), 3);
        assert_eq!(levenshtein_bytes(b"bullfrog", b"frogger"), 7);
    }

    struct HashTestStrings<'a> {
        ex1: &'a str,
        ex2: &'a str,
        ex3: &'a str,
        ex4: &'a str,
        ex5: &'a str,
    }

    impl<'a> HashTestStrings<'a> {
        fn new() -> Self {
            HashTestStrings {
                ex1: "The quick brown fox jumps over 10 lazy dogs 2345 67*89",
                ex2: "$",
                ex3: "a",
                ex4: "A",
                ex5: "admnn 234 23jmm2 m1mlkml76mp 9 09i jasd90i234nn 234n o234n6kjn8 9345jnj45 \
                jkkdfjsjn uhufnj ;fjjnjkl weip(()#4n n 234@#lnn l lmkdjkhkwbehrb**((h4nn  23424n \
                l23k42m    234njn432 llkmldasdla asda2poni lklfdi4lj6lk lknd90nl omkmdf0 lok45nn \
                OU*(Unjk34 ldklnskjf l mndjkjnkfjnr well",
            }
        }
    }

    #[test]
    fn test_md5_hash_string() {
        let examples = HashTestStrings::new();
        let expected_hash_ex1 = "366b4078aaf21b0323746ebbd9f153e2";
        let expected_hash_ex2 = "c3e97dd6e97fb5125688c97f36720cbe";
        let expected_hash_ex3 = "0cc175b9c0f1b6a831c399e269772661";
        let expected_hash_ex4 = "7fc56270e7a70fa81a5935b72eacbe29";
        let expected_hash_ex5 = "db1a8b8aedec0bd3b804b7a833f5cfe2";

        assert_eq!(hash_string(HashAlgorithm::MD5, examples.ex1, true), expected_hash_ex1);
        assert_eq!(hash_string(HashAlgorithm::MD5, examples.ex2, true), expected_hash_ex2);
        assert_eq!(hash_string(HashAlgorithm::MD5, examples.ex3, true), expected_hash_ex3);
        assert_eq!(hash_string(HashAlgorithm::MD5, examples.ex4, true), expected_hash_ex4);
        assert_eq!(hash_string(HashAlgorithm::MD5, examples.ex5, true), expected_hash_ex5);
    }

    #[test]
    fn test_md5_hash_file() {}

    #[test]
    fn test_sha256_hash_string() {
        let examples = HashTestStrings::new();
        let expected_hash_ex1 = "b2c977fed8cf70d15d864ef9c96708ded13bde4ab0cdf03dac42678f06dab5d4";
        let expected_hash_ex2 = "09fc96082d34c2dfc1295d92073b5ea1dc8ef8da95f14dfded011ffb96d3e54b";
        let expected_hash_ex3 = "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb";
        let expected_hash_ex4 = "559aead08264d5795d3909718cdd05abd49572e84fe55590eef31a88a08fdffd";
        let expected_hash_ex5 = "8d376648a83c999de09ff7638a9729fc050ff475f3470ea001914bd80f40694d";

        assert_eq!(hash_string(HashAlgorithm::SHA2_256, examples.ex1, true), expected_hash_ex1);
        assert_eq!(hash_string(HashAlgorithm::SHA2_256, examples.ex2, true), expected_hash_ex2);
        assert_eq!(hash_string(HashAlgorithm::SHA2_256, examples.ex3, true), expected_hash_ex3);
        assert_eq!(hash_string(HashAlgorithm::SHA2_256, examples.ex4, true), expected_hash_ex4);
        assert_eq!(hash_string(HashAlgorithm::SHA2_256, examples.ex5, true), expected_hash_ex5);
    }

    #[test]
    fn test_sha256_hash_file() {}

    #[test]
    fn test_sha512_hash_string() {
        let examples = HashTestStrings::new();
        let expected_hash_ex1 = "6a759c7459d6250bffd071534bc3b42daa5a454a0216234e251f68eeeb8c07e8c2471173956786b46908390d3b13a2af66c14f4fb7b52b81ae537fe047e7328c";
        let expected_hash_ex2 = "840cfc6285878464c36c9aa819d8373729eda14c3e701fd37afec1d5baa2893944c696fc4017a520abfbb1347b62e6b858211d3ea7c7dd26319601fde119c3b4";
        let expected_hash_ex3 = "1f40fc92da241694750979ee6cf582f2d5d7d28e18335de05abc54d0560e0f5302860c652bf08d560252aa5e74210546f369fbbbce8c12cfc7957b2652fe9a75";
        let expected_hash_ex4 = "21b4f4bd9e64ed355c3eb676a28ebedaf6d8f17bdc365995b319097153044080516bd083bfcce66121a3072646994c8430cc382b8dc543e84880183bf856cff5";
        let expected_hash_ex5 = "89ab2bf39588b9500fc44f9932db127bd65e9c8eba68f91d28104a7d369cbdbc50f265515ec81bf8a9b222cb9a8687a644cf51bc81e4f67ed386772309a5f2b7";

        assert_eq!(hash_string(HashAlgorithm::SHA2_512, examples.ex1, true), expected_hash_ex1);
        assert_eq!(hash_string(HashAlgorithm::SHA2_512, examples.ex2, true), expected_hash_ex2);
        assert_eq!(hash_string(HashAlgorithm::SHA2_512, examples.ex3, true), expected_hash_ex3);
        assert_eq!(hash_string(HashAlgorithm::SHA2_512, examples.ex4, true), expected_hash_ex4);
        assert_eq!(hash_string(HashAlgorithm::SHA2_512, examples.ex5, true), expected_hash_ex5);
    }

    #[test]
    fn test_sha512_hash_file() {}

    #[test]
    fn test_sha3_256_hash_string() {
        let examples = HashTestStrings::new();
        let expected_hash_ex1 = "f6817ac5b66bf4123220ba5c6b1a3d2590df986a53fdd1d6cc6a6d9632b602ae";
        let expected_hash_ex2 = "5ecdbae446010644dd235353f132c03fa21a1e6020a86e1672cbf1a693db5428";
        let expected_hash_ex3 = "80084bf2fba02475726feb2cab2d8215eab14bc6bdd8bfb2c8151257032ecd8b";
        let expected_hash_ex4 = "1c9ebd6caf02840a5b2b7f0fc870ec1db154886ae9fe621b822b14fd0bf513d6";
        let expected_hash_ex5 = "452ccfd74f73a9d2a15355693f36cc1b0c1a605b6bd9b45908bdb2be34859d07";

        assert_eq!(hash_string(HashAlgorithm::SHA3_256, examples.ex1, true), expected_hash_ex1);
        assert_eq!(hash_string(HashAlgorithm::SHA3_256, examples.ex2, true), expected_hash_ex2);
        assert_eq!(hash_string(HashAlgorithm::SHA3_256, examples.ex3, true), expected_hash_ex3);
        assert_eq!(hash_string(HashAlgorithm::SHA3_256, examples.ex4, true), expected_hash_ex4);
        assert_eq!(hash_string(HashAlgorithm::SHA3_256, examples.ex5, true), expected_hash_ex5);
    }

    #[test]
    fn test_sha3_256_hash_file() {}

    #[test]
    fn test_sha3_512_hash_string() {
        let examples = HashTestStrings::new();
        let expected_hash_ex1 = "9177ae33f3cf1067d0b7a6978ebdfbf988d1c47307ebeebc450468c2ca4243595439366728bbcc10ddd6778aca38b8537c1e472953237da2638a4379102a3c11";
        let expected_hash_ex2 = "b09cfeb88ec3825be786c170449d7c0d99ee6156290545623a8b46e432128c9a2535bed522b4eb6b22bb99b01156bffc147a7f667bc7deac081bf9b5e0cf9973";
        let expected_hash_ex3 = "697f2d856172cb8309d6b8b97dac4de344b549d4dee61edfb4962d8698b7fa803f4f93ff24393586e28b5b957ac3d1d369420ce53332712f997bd336d09ab02a";
        let expected_hash_ex4 = "f5f0eaa9ca3fd0c4e0d72a3471e4b71edaabe2d01c4b25e16715004ed91e663a1750707cc9f04430f19b995f4aba21b0ec878fc5c4eb838a18df5bf9fdc949df";
        let expected_hash_ex5 = "a41c503a2f19925aa5c27ad7970b6144efe0e9cbe6858fc737353a0db8ad446402c1238c6c2aa70ae2751a64e862df3ebacb5683629a26575b664276a2f144a2";

        assert_eq!(hash_string(HashAlgorithm::SHA3_512, examples.ex1, true), expected_hash_ex1);
        assert_eq!(hash_string(HashAlgorithm::SHA3_512, examples.ex2, true), expected_hash_ex2);
        assert_eq!(hash_string(HashAlgorithm::SHA3_512, examples.ex3, true), expected_hash_ex3);
        assert_eq!(hash_string(HashAlgorithm::SHA3_512, examples.ex4, true), expected_hash_ex4);
        assert_eq!(hash_string(HashAlgorithm::SHA3_512, examples.ex5, true), expected_hash_ex5);
    }

    #[test]
    fn test_sha3_512_hash_file() {}

    #[test]
    fn test_blake2s_hash_string() {
        let examples = HashTestStrings::new();
        let expected_hash_ex1 = "22e018773b2d2d5ce503b8daeaef527eca344084d02dde08bdd7dcfbe39346ce";
        let expected_hash_ex2 = "c687b011b2e6f1e63a97ecf43eac3f446594409aeaa5455e0fb60fb9ccac515a";
        let expected_hash_ex3 = "4a0d129873403037c2cd9b9048203687f6233fb6738956e0349bd4320fec3e90";
        let expected_hash_ex4 = "98e14bd264b8837ddf8fd12d6f5641d59c369720b02c105feaf99f1b6a7b9618";
        let expected_hash_ex5 = "154eaff13d999834a00cb8be3d2ed80e41cdfbe9ce907b193f58747c42a6a238";

        assert_eq!(hash_string(HashAlgorithm::BLAKE2S, examples.ex1, true), expected_hash_ex1);
        assert_eq!(hash_string(HashAlgorithm::BLAKE2S, examples.ex2, true), expected_hash_ex2);
        assert_eq!(hash_string(HashAlgorithm::BLAKE2S, examples.ex3, true), expected_hash_ex3);
        assert_eq!(hash_string(HashAlgorithm::BLAKE2S, examples.ex4, true), expected_hash_ex4);
        assert_eq!(hash_string(HashAlgorithm::BLAKE2S, examples.ex5, true), expected_hash_ex5);
    }

    #[test]
    fn test_blake2s_hash_file() {}

    #[test]
    fn test_blake2b_hash_string() {
        let examples = HashTestStrings::new();
        let expected_hash_ex1 = "a20d3f173d00b74df70c10cb8140a6cf6cd21128068b249f3ff8bfbf4fc30c08ee9bae5e83854a0a561b06c580a79bfc22b71532353d3b43ae65cff5f2e2b626";
        let expected_hash_ex2 = "388a507aa909e01f549b7fd8e6094b0438e8a1ecc4db0d95812f716ed4b3507a8f2381451727ce5bf4f7550b5b75fb0efc5b74c500bf21c89a21e99d9c26594c";
        let expected_hash_ex3 = "333fcb4ee1aa7c115355ec66ceac917c8bfd815bf7587d325aec1864edd24e34d5abe2c6b1b5ee3face62fed78dbef802f2a85cb91d455a8f5249d330853cb3c";
        let expected_hash_ex4 = "3e6173df7f81c1eb9ce997312fe72e441b40b72cd5ffca23d05ef805bf6e938e1ef9c3cac173005f77d698c2ca30dd785eb745aad32fcb4d5afff91c30ad7472";
        let expected_hash_ex5 = "97033cf9ca7c993c935beb9f230a400a6ee35fccc478ca55b1cc135aa77dfbd75fc0b93812f525368559f419302dce3b3d961ed4d77b7c19e84d50df072588ec";

        assert_eq!(hash_string(HashAlgorithm::BLAKE2B, examples.ex1, true), expected_hash_ex1);
        assert_eq!(hash_string(HashAlgorithm::BLAKE2B, examples.ex2, true), expected_hash_ex2);
        assert_eq!(hash_string(HashAlgorithm::BLAKE2B, examples.ex3, true), expected_hash_ex3);
        assert_eq!(hash_string(HashAlgorithm::BLAKE2B, examples.ex4, true), expected_hash_ex4);
        assert_eq!(hash_string(HashAlgorithm::BLAKE2B, examples.ex5, true), expected_hash_ex5);
    }

    #[test]
    fn test_blake2b_hash_file() {}
}
