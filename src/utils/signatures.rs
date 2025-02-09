use std::collections::HashMap;

use crate::utils::consts::SIGNATURES;

pub fn load_signatures() -> HashMap<Vec<u8>, &'static str> {
    let mut signatures: HashMap<Vec<u8>, &str> = HashMap::new();

    for (sig, name) in SIGNATURES {
        signatures.insert(sig.to_vec(), name);
    }

    signatures
}

pub fn check_signature<'a>(sig: &[u8], signatures: &'a HashMap<Vec<u8>, &'a str>) -> Option<&'a str> {
    signatures
        .iter()
        .find(|(sigs, _)| sig.starts_with(sigs))
        .map(|(_, name)| *name)
}