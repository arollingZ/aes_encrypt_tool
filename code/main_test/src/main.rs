extern crate aes_tools;

fn main() {
    let mut test_key = String::from("0123456789ABCDEF").trim().as_bytes().to_vec();
    let mut test_decrypt_data = String::from("123456").trim().as_bytes().to_vec();

    println!("test data:{:?}",test_decrypt_data);
    aes_tools::aes_key_init(&mut test_key);
    let mut test_encrypt_data = aes_tools::aes_encrypt(&mut test_decrypt_data,&test_key);
    println!("encrypted data:{:?}",test_encrypt_data);
    let decrypt_result_data = aes_tools::aes_decrypt(&mut test_encrypt_data,&test_key);
    println!("decrypted data:{:?}",decrypt_result_data);
}
