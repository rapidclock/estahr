#[cfg(test)]
mod ext_tests {
    use estahr::*;
    use std::path::PathBuf;

    const filename_small : &str = "small_600.txt";
    const filename_large : &str = "large_1800.txt";

    fn get_tests_file_path_string(file_name: &str) -> String {
        let mut path = std::env::current_dir().unwrap();
        path.push("tests");
        path.push(file_name);
        if path.exists() {
            return path.into_os_string().to_str().unwrap().to_string();
        } else {
            panic!(format!("Cannot find file : <{}> in tests folder", file_name));
        }
    }

    #[test]
    fn test_md5_hash_file() {
        assert_eq!(hash_file(HashAlgorithm::MD5, get_tests_file_path_string(filename_small).as_ref(),
                             true).unwrap(), "bc3865fa3378733b0a0a665a2d943595");
        assert_eq!(hash_file(HashAlgorithm::MD5, get_tests_file_path_string(filename_large).as_ref(),
                             true).unwrap(), "658528b6652f8eca8b1321af88ec2bcf");
    }

    #[test]
    fn test_sha256_hash_file() {
        assert_eq!(hash_file(HashAlgorithm::SHA2_256, get_tests_file_path_string(filename_small).as_ref(),
                             true).unwrap(), "c675343eb3f9331772d98f01e78e0f7e12e977f4c12200df599906400d18e5b7");
        assert_eq!(hash_file(HashAlgorithm::SHA2_256, get_tests_file_path_string(filename_large).as_ref(),
                             true).unwrap(), "d210e97734ae6338f7c7a43eface2a7444a258ba398280d82ac27959690d9ca6");
    }

    #[test]
    fn test_sha512_hash_file() {
        assert_eq!(hash_file(HashAlgorithm::SHA2_512, get_tests_file_path_string(filename_small).as_ref(),
                             true).unwrap(), "28dd3fab391cfae8461d7942d9ab642e00a3b164518180e71f032765f717384027146f58b566d06db63c6b19a05e9ae417ba0bd8c9ab4007f2cd692b4f5e5d35");
        assert_eq!(hash_file(HashAlgorithm::SHA2_512, get_tests_file_path_string(filename_large).as_ref(),
                             true).unwrap(), "ca09aa89ab8eea844a1bcc0d337a8af3b8b8351f0836b3d7dd2ea09f5ce22cbec299c0ab4636a306625236bd45a9d9a49d0c50f10ba6f3299bc781f09c816720");
    }

    #[test]
    fn test_sha3_256_hash_file() {
        assert_eq!(hash_file(HashAlgorithm::SHA3_256, get_tests_file_path_string(filename_small).as_ref(),
                             true).unwrap(), "87632a6f642e8440fb849e217e41553e2e719fb2701559462ae95a1449ccf83e");
        assert_eq!(hash_file(HashAlgorithm::SHA3_256, get_tests_file_path_string(filename_large).as_ref(),
                             true).unwrap(), "142ae8a76c47552ac10d8c92c7e65a34efc27bde975a8bf050cd1a28846b1508");
    }

    #[test]
    fn test_sha3_512_hash_file() {
        assert_eq!(hash_file(HashAlgorithm::SHA3_512, get_tests_file_path_string(filename_small).as_ref(),
                             true).unwrap(), "a00039bd24ade99e8a67748bb9828c4a123ca4dc98490e408623d710c0b83cfe1bfc5646d2e29fd79509bac897df506d910b6175c5d2f8b469a1c21cb57c48be");
        assert_eq!(hash_file(HashAlgorithm::SHA3_512, get_tests_file_path_string(filename_large).as_ref(),
                             true).unwrap(), "08cb259551963e3ff945b1053e3de3415177b341adb853cfad12d1aa22b30473d1c988c2f37cc113f261b283e023b588b3a39eedf930d790b0339830b1253c33");
    }

    #[test]
    fn test_blake2s_hash_file() {
        assert_eq!(hash_file(HashAlgorithm::BLAKE2S, get_tests_file_path_string(filename_small).as_ref(),
                             true).unwrap(), "c4bdf43b4a6d9d1bd626b11560ca79dc986feb033cc36b4c9c0daf0fe7b93c61");
        assert_eq!(hash_file(HashAlgorithm::BLAKE2S, get_tests_file_path_string(filename_large).as_ref(),
                             true).unwrap(), "149e2d479705adab174fc6bd177543e42a3e8ace79d4cd1592e9cd1bcbc3a8d0");
    }

    #[test]
    fn test_blake2b_hash_file() {
        assert_eq!(hash_file(HashAlgorithm::BLAKE2B, get_tests_file_path_string(filename_small).as_ref(),
                             true).unwrap(), "2392fdbff0ceb25c10673c7eaf1a907be0517e939aec9943cdf85b9b11004beaea1f7da1dac8752554a76cb35e4a836b34cca0cd08ebc48374043dbcabcbd2b8");
        assert_eq!(hash_file(HashAlgorithm::BLAKE2B, get_tests_file_path_string(filename_large).as_ref(),
                             true).unwrap(), "8f5a4fd4a7009432d43a59b7cf0896691aa611a4d683b3e8f9e0186c96a7a9c2156bd3b7e0767218ee238e58abd0b6870da84470909f729b9d62b95fb35946f1");
    }

    #[test]
    fn test_blake3_hash_file() {
        assert_eq!(hash_file(HashAlgorithm::BLAKE3, get_tests_file_path_string(filename_small).as_ref(),
                             true).unwrap(), "f27c992490b785efd646370d102f86cce36448531e2885017b5ab8269d826d92");
        assert_eq!(hash_file(HashAlgorithm::BLAKE3, get_tests_file_path_string(filename_large).as_ref(),
                             true).unwrap(), "b4b805595bac0db59dcb72664148b72d8e5f9fed362cccfc913cc11a8dbdc0b5");
    }
}