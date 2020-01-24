# estahr
String comparison and hashing library

The word `estahr` is the phonetic version of str (ess-tee-ahr). :)

For the 0.1.x version, we include string distance and hashing functions. The hashing is
provided by the [RustCrypto](https://github.com/RustCrypto/hashes) series of crates.

The blake3 hashing is provided by the [blake3](https://crates.io/crates/blake3) crate,
straight from the [Blake-3 Team](https://github.com/BLAKE3-team/BLAKE3).

## String Distances
The string distances currently provided are:
1. Hamming Distance [Wiki](https://en.wikipedia.org/wiki/Hamming_distance)
2. Levenshtein Distance [Wiki](https://en.wikipedia.org/wiki/Levenshtein_distance)
3. Jaro Winkler Distance [Wiki](https://en.wikipedia.org/wiki/Jaro-Winkler_distance)

## Hashing
This crate provides both string and file based hashing.

The hashing (file & string) provided include:
1. MD5 (for compatibility, not recommended due to proven weaknesses).
2. SHA2 - 256
3. SHA2 - 512
4. SHA3 - 256
5. SHA3 - 512
6. BLAKE 2s (256 bit)
7. BLAKE 2b (512 bit)
8. BLAKE 3 (256 bit)

For more info about these read the [RustCrypto](https://github.com/RustCrypto/hashes) README.


## Contribution

Any contribution, in any form, either intentionally or un-intentionally presented by you, whether finally being included or not as part of this work, will be licenced by the MIT Licence, without any additional terms or conditions.

Aditionally, the responsibility falls on you, in the course of participation, of any kind, in relation to this body of work, to clearly indicate that the origin or rights of the contribution are not yours; i.e: We assume you are the sole author of any form of contribution to this body of work.
