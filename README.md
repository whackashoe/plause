plausible deniability for the masses
===

(note: this software is new and should not be trusted until fully audited, consider it a proof of concept)

plause encrypts and embeds files into a blob of random data. it is designed so that it is computationally infeasible to know how many files are in the blob, where they are, or what they contain.

how does it work?
===

plause is a synchronous stream cipher that uses keystream blocks with self referential sha-256 hashes+password+uniqueness_ident and applies these as a xor cascade to the binary blocks to set begin/end identifiers and to encode/decode your files. these are placed randomly (but not overlapping) within the blob. to decrypt, a begin/end identifier must be found in the blob and the data within these idents has the xor cascade applied to it resulting in the original output. using this form, we are able to encode and stash numerous files without comprimising integrity. 

benefits?
===

plause could be beneficial in any situation where randomness allows the ability to share without being detected. some potential usecases stego related could be embedding encrypted messages in intentional static. 

flipping of some bits in the ciphertext when decrypted will produce a flipped bit in the result at the same offset. this allows for error correction if implemented in the filetypes (plause does not concern itself with this, but it is possible given the right container format)

encrypted files can be as large or as small as you wish (minimum size is the sum of all input files due to non overlap). this allows it to work well in limited environments, and gives added file-anonymity in environments that allow larger outputs.

how to use?
===

`cargo build`

`./target/plause --help` - show full explanation of all options
