```

     _____ ______   ________  ________                   _______  ________     
    |\   _ \  _   \|\   __  \|\   ___ \                 /  ___  \|\   ____\    
    \ \  \\\__\ \  \ \  \|\  \ \  \_|\ \  ____________ /__/|_/  /\ \  \___|    
     \ \  \\|__| \  \ \  \\\  \ \  \ \\ \|\____________\__|//  / /\ \  \____   
      \ \  \    \ \  \ \  \\\  \ \  \_\\ \|____________|   /  /_/__\ \  ___  \ 
       \ \__\    \ \__\ \_______\ \_______\               |\________\ \_______\
        \|__|     \|__|\|_______|\|_______|                \|_______|\|_______|
                                                                           

```



<pre>
# Mod-26

Library of over 25+ cryptography algorithms ranging from classical to modern. 

Helpful for students & beginners who want to refer a minimal headstart-
implementation of different ciphers & encryptions.

This project was coded alongside-
while I was reading the book "The Mathematics of Secrets" (<a href="https://amzn.in/d/0dkXNyTh">https://amzn.in/d/0dkXNyTh</a>)
</pre>


```
# Features

- written in simple rust
- every cipher is self-contained within a single file, with only 1-2 external file imports
- most ciphers contains a both `encrypt` & `decrypt` function
- 1 external dependency (only in public-key cipher) 
- contains test for every cipher
- direct papers/wiki link in each cipher file

```


<pre>
# Implementations


---- Simple Substitution

- <a href="/src/simple_substitution/additive.rs">additive</a>
- <a href="/src/simple_substitution/multiplicative.rs">multiplicative</a>
- <a href="/src/simple_substitution/affine.rs">affine</a>
- <a href="/src/simple_substitution/hill.rs">hill</a>


---- Polyalphabetic Substitution

- <a href="/src/polyalphebatic_substitution/vigenere.rs">vigenere</a>
- <a href="/src/polyalphebatic_substitution/alberti.rs">alberti</a>


---- Polyliteral Substitution

- <a href="/src/polyliteral/adfgvx.rs">adfgvx</a>
- <a href="/src/polyliteral/biliteral.rs">biliteral</a>


---- Transposition

- <a href="/src/transposition/syctale.rs">syctale</a>
- <a href="/src/transposition/rail_fence.rs">rail_fence</a>
- <a href="/src/transposition/geometric.rs">geometric</a>
- <a href="/src/transposition/columnar.rs">columnar</a>
- <a href="/src/transposition/permutation.rs">permutation</a>


---- Block Cipher

- <a href="/src/block/des.rs">des</a>
- <a href="/src/block/aes.rs">aes</a>


---- Stream Cipher

- <a href="/src/stream/autokey.rs">autokey</a>
- <a href="/src/stream/gromark.rs">gromark</a>


---- Public Key Cipher

- <a href="/src/public_key/pohlig_hellman.rs">pohlig_hellman</a>
- <a href="/src/public_key/diffie_hellman.rs">diffie_hellman</a>
- <a href="/src/public_key/rsa.rs">rsa</a>


--------------------
---- TODO

- Elliptic Curve Cryptography
- Digital Signatures
- Lattice-based Cryptography

</pre>


<pre>

# Useful Utils

- <a href="/src/utils/mod_arithmetic.rs">Modular Arithmetic</a>
- <a href="/src/utils/char_set.rs">Custom Charset</a>
</pre>

```
# Example 

let char_set = CharSet::from_alphabet_lowercase();
let vigenere = VigenèreCipher::new(char_set);

let key = "oculorhinolaringology";

// encrypt
let encrypted = vigenere.encrypt("attackingtonight", key); // -> ovnlqbpvthznzouz 

// decrypt
let decrypted = vigenere.decrypt("ovnlqbpvthznzouz", key); // -> attackingtonight
```


```
# Resources

- https://en.wikipedia.org/wiki/Cryptography
- https://en.wikipedia.org/wiki/Outline_of_cryptography
- https://cryptii.com
- https://amzn.in/d/0dkXNyTh
- https://amzn.in/d/09v3PAqD
- https://www.dcode.fr/en
- https://www.schneier.com/crypto-gram/archives/1999/1015.html
```
