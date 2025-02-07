use std::fs;

use aws_nitro_enclaves_cose::{crypto::Openssl, CoseSign1};
use aws_nitro_enclaves_nsm_api::api::AttestationDoc;
use clap::Parser;
use nitro_attestation::{get_attestation_doc, ClientArgs};

fn main() {
    let args = ClientArgs::parse();

    let attestation_doc_bytes = get_attestation_doc(args).expect("get_attestation_doc failed");

    let path = "circuit_data";
    fs::create_dir_all(path).unwrap();

    // dump vkey
    let cose_sign1 =
        CoseSign1::from_bytes(&attestation_doc_bytes).expect("CoseSign1 parsing failed");
    let payload = cose_sign1
        .get_payload::<Openssl>(None)
        .expect("cose_sign1.get_payload failed");
    let att_doc = AttestationDoc::from_binary(&payload).expect("AttestationDoc parsing failed");
    let pcr0 = att_doc.pcrs.get(&0).expect("missing pcrs.get(&0)");
    fs::write(format! {"{path}/pcr0.bin"}, pcr0.to_vec()).expect("pcr0 fs::write failed");

    // dump proof
    fs::write(
        format! {"{path}/attestation_doc.bin"},
        attestation_doc_bytes,
    )
    .expect("attestation_doc fs::write failed");
}
