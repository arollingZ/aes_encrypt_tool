# aes_encrypt_tool

###aes_tools::aes_key_init(in_key:&mut Vec<u8>)
to initialize your aes key,note that in_key should be 128bits.

###aes_tools::aes_encrypt(indata:&mut Vec<u8>,aes_key:&Vec<u8>)
to encrypt your indata,and return encrypted Vec.

###aes_tools::aes_decrypt(indata:&mut Vec<u8>,aes_key:&Vec<u8>)
to decrypt your indata,and return decrypted Vec.
