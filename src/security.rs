// use std::{str::FromStr};

use std::str::FromStr;
use ed25519_dalek::{pkcs8::DecodePublicKey, Signature, VerifyingKey};

/// Verify the signature of a given packet.
/// Remember, a packet will always follow same structure : 
///
/// ```
/// <type_packet>--<author_ed25519>--[infos supplémentaires]--<signature_auteur>
/// ```
/// So this function verify if the last data of the packet (signature so) can verify the whole rest
/// of it and return a boolean corresponding to if it succeded or not
///
/// WARN: A possible upgrade of this function would be to return a Result<bool, Enum> with a
/// complete load of possible error cause so custom messages can be returned to senders
pub fn verify_packet_signature(packet: String) -> bool {
    let mut split_informations: Vec<&str> = packet.split("__").collect();

    if split_informations.len() < 3 {
        return false
    }

    if let Ok(key) = VerifyingKey::from_public_key_pem(&split_informations[1]) {
        // Now we can verify message by joining all remaining elements with -- and compare the
        // signature + key with it


        if let Ok(signature) = Signature::from_str(split_informations.pop().unwrap()) {
            let content = split_informations.join("__");
            println!("Veriying string : {}", content);

            match key.verify_strict(content.as_bytes(), &signature) {
                Ok(_) => {
                    return true;
                },
                Err(_) => {
                    return false;
                }
            }
        }     

        println!("Invalid Signature format")

    } 

    println!("Invalid key : {}", &split_informations[1]);
    false
}

#[cfg(test)]
mod tests {
    use crate::security;
    #[test]
    fn verify_empty_signature() {
        assert!(!security::verify_packet_signature("".into()));
    }

    #[test]
    fn verify_invalid_signature() {
        assert!(!security::verify_packet_signature("message_invliadkey_invlaidsignatureprovided".into()))
    }

    #[test]
    fn verify_valid_signature() {
        assert!(security::verify_packet_signature("message__-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEA2oJO54T5oBYTdCVxw6YVafXLkrfg8q0CLp2+28vIaXQ=
-----END PUBLIC KEY-----
__future_target__test__1934BDD900F648E8366CA1C9E8E60B9C8E3C2E9CB3561E547DFC73BEF6FBF6DC7CFD061C7AB2B742C51D66475F7BFE30B22E35AA75FC693F9AB86981EC373F04".into()))
    }
}
