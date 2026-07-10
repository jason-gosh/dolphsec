use hmac::{Hmac, Mac, KeyInit};
use sha2::Sha256;

#[allow(dead_code)]
type HmacSha256 = Hmac<Sha256>;

fn check_source(secret: String, message: String) {

    let mut mac_client = HmacSha256::new_from_slice(&secret.clone().into_bytes())
        .expect("Invalid key on hmac");
    mac_client.update(&message.clone().into_bytes());
    
    //Integrity Signament
    let verifycode = mac_client.finalize().into_bytes();

    let mut mac_server = HmacSha256::new_from_slice(&secret.into_bytes())
        .expect("Invalid key");
    mac_server.update(&message.into_bytes());

    if mac_server.verify(&verifycode).is_ok() {
        return;
    } else {
        panic!("[security error]");
    }
}