// TODO: FIXME: It must be removed after code layout stabilization!
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sovrin;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

#[macro_use]
#[path = "utils/mod.rs"]
mod utils;

use utils::pool::PoolUtils;
use utils::test::TestUtils;
use utils::timeout::TimeoutUtils;

use sovrin::api::ErrorCode;
use sovrin::api::anoncreds::{
    sovrin_issuer_create_and_store_claim_def,
    sovrin_issuer_create_claim,
    sovrin_prover_create_master_secret,
    sovrin_prover_create_and_store_claim_req,
    sovrin_prover_store_claim,
    sovrin_prover_get_claims_for_proof_req,
    sovrin_prover_create_proof,
    sovrin_verifier_verify_proof
};
use sovrin::api::ledger::sovrin_submit_request;
use sovrin::api::pool::{
    sovrin_open_pool_ledger,
    sovrin_create_pool_ledger_config,
};
use sovrin::api::wallet::{
    sovrin_create_wallet,
    sovrin_open_wallet,
    sovrin_wallet_set_seq_no_for_value};

use utils::callback::CallbackUtils;

use std::ptr::null;
use std::sync::mpsc::{channel};
use std::ffi::{CString};

#[test]
fn anoncreds_demo_works() {
    TestUtils::cleanup_storage();

    let (create_wallet_sender, create_wallet_receiver) = channel();
    let (open_wallet_sender, open_wallet_receiver) = channel();
    let (issuer_create_claim_definition_sender, issuer_create_claim_definition_receiver) = channel();
    let (wallet_set_seq_no_for_value_sender, wallet_set_seq_no_for_value_receiver) = channel();
    let (prover_create_master_secret_sender, prover_create_master_secret_receiver) = channel();
    let (prover_create_claim_req_sender, prover_create_claim_req_receiver) = channel();
    let (issuer_create_claim_sender, issuer_create_claim_receiver) = channel();
    let (prover_store_claim_sender, prover_store_claim_receiver) = channel();
    let (prover_get_claims_for_proof_req_sender, prover_get_claims_for_proof_req_receiver) = channel();
    let (prover_create_proof_sender, prover_create_proof_receiver) = channel();
    let (verifier_verify_proof_sender, verifier_verify_proof_receiver) = channel();

    let issuer_create_claim_definition_cb = Box::new(move |err, claim_def_json, claim_def_uuid| {
        issuer_create_claim_definition_sender.send((err, claim_def_json, claim_def_uuid)).unwrap();
    });
    let create_wallet_cb = Box::new(move |err| {
        create_wallet_sender.send(err).unwrap();
    });
    let open_wallet_cb = Box::new(move |err, handle| {
        open_wallet_sender.send((err, handle)).unwrap();
    });
    let wallet_set_seq_no_for_value_cb = Box::new(move |err| {
        wallet_set_seq_no_for_value_sender.send(err).unwrap();
    });
    let prover_create_master_secret_cb = Box::new(move |err| {
        prover_create_master_secret_sender.send(err).unwrap();
    });
    let prover_create_claim_req_cb = Box::new(move |err, claim_req_json| {
        prover_create_claim_req_sender.send((err, claim_req_json)).unwrap();
    });
    let issuer_create_claim_cb = Box::new(move |err, revoc_reg_update_json, xclaim_json| {
        issuer_create_claim_sender.send((err, revoc_reg_update_json, xclaim_json)).unwrap();
    });
    let prover_store_claim_cb = Box::new(move |err| {
        prover_store_claim_sender.send(err).unwrap();
    });
    let prover_get_claims_for_proof_req_cb = Box::new(move |err, claims_json| {
        prover_get_claims_for_proof_req_sender.send((err, claims_json)).unwrap();
    });
    let prover_create_proof_cb = Box::new(move |err, proof_json| {
        prover_create_proof_sender.send((err, proof_json)).unwrap();
    });
    let verifier_verify_proof_cb = Box::new(move |err, valid| {
        verifier_verify_proof_sender.send((err, valid)).unwrap();
    });

    let (issuer_create_claim_definition_command_handle, create_claim_definition_callback) = CallbackUtils::closure_to_issuer_create_claim_definition_cb(issuer_create_claim_definition_cb);
    let (create_wallet_command_handle, create_wallet_callback) = CallbackUtils::closure_to_create_wallet_cb(create_wallet_cb);
    let (open_wallet_command_handle, open_wallet_callback) = CallbackUtils::closure_to_open_wallet_cb(open_wallet_cb);
    let (wallet_set_seq_no_for_value_command_handle, wallet_set_seq_no_for_value_callback) = CallbackUtils::closure_to_wallet_set_seq_no_for_value_cb(wallet_set_seq_no_for_value_cb);
    let (prover_create_master_secret_command_handle, prover_create_master_secret_callback) = CallbackUtils::closure_to_prover_create_master_secret_cb(prover_create_master_secret_cb);
    let (prover_create_claim_req_command_handle, prover_create_claim_req_callback) = CallbackUtils::closure_to_prover_create_claim_req_cb(prover_create_claim_req_cb);
    let (issuer_create_claim_command_handle, issuer_create_claim_callback) = CallbackUtils::closure_to_issuer_create_claim_cb(issuer_create_claim_cb);
    let (prover_store_claim_command_handle, prover_store_claim_callback) = CallbackUtils::closure_to_prover_store_claim_cb(prover_store_claim_cb);
    let (prover_get_claims_for_proof_req_handle, prover_get_claims_for_proof_req_callback) = CallbackUtils::closure_to_prover_get_claims_for_proof_req_cb(prover_get_claims_for_proof_req_cb);
    let (prover_create_proof_handle, prover_create_proof_callback) = CallbackUtils::closure_to_prover_create_proof_cb(prover_create_proof_cb);
    let (verifier_verify_proof_handle, verifier_verify_proof_callback) = CallbackUtils::closure_to_verifier_verify_proof_cb(verifier_verify_proof_cb);

    let pool_name = "pool1";
    let wallet_name = "issuer_wallet";
    let xtype = "default";

    //TODO CREATE ISSUER, PROVER, VERIFIER WALLETS
    //1. Create Wallet
    let err =
        sovrin_create_wallet(create_wallet_command_handle,
                             CString::new(pool_name).unwrap().as_ptr(),
                             CString::new(wallet_name).unwrap().as_ptr(),
                             CString::new(xtype).unwrap().as_ptr(),
                             null(),
                             null(),
                             create_wallet_callback);

    assert_eq!(ErrorCode::Success, err);
    let err = create_wallet_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    //2. Open Issuer Wallet. Gets Issuer wallet handle
    let err =
        sovrin_open_wallet(open_wallet_command_handle,
                           CString::new(wallet_name).unwrap().as_ptr(),
                           null(),
                           null(),
                           open_wallet_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, wallet_handle) = open_wallet_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    let schema_seq_no = 1;
    let schema = format!("{{\
                            \"name\":\"gvt\",\
                            \"version\":\"1.0\",\
                            \"attribute_names\":[\"age\",\"sex\",\"height\",\"name\"],\
                            \"seq_no\":{}\
                         }}", schema_seq_no);

    // 3. Issuer create Claim Definition for Schema
    let err =
        sovrin_issuer_create_and_store_claim_def(issuer_create_claim_definition_command_handle,
                                                 wallet_handle,
                                                 CString::new(schema.clone()).unwrap().as_ptr(),
                                                 null(),
                                                 false,
                                                 create_claim_definition_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, claim_def_json, claim_def_uuid) = issuer_create_claim_definition_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    println!("claim_def_json {:?}", claim_def_json);
    assert_eq!(ErrorCode::Success, err);

    let claim_def_seq_no = 1;

    // 4. Create relationship between claim_def_seq_no and claim_def_uuid in wallet
    let err = sovrin_wallet_set_seq_no_for_value(wallet_set_seq_no_for_value_command_handle,
                                                 wallet_handle,
                                                 CString::new(claim_def_uuid).unwrap().as_ptr(),
                                                 claim_def_seq_no,
                                                 wallet_set_seq_no_for_value_callback);

    assert_eq!(ErrorCode::Success, err);
    let err = wallet_set_seq_no_for_value_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    let master_secret_name = "master_secret";

    // 5. Prover create Master Secret
    let err =
        sovrin_prover_create_master_secret(prover_create_master_secret_command_handle,
                                           wallet_handle,
                                           CString::new(master_secret_name).unwrap().as_ptr(),
                                           prover_create_master_secret_callback);

    assert_eq!(ErrorCode::Success, err);
    let err = prover_create_master_secret_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    let prover_did = "some_prover_did";
    let claim_offer_json = format!("{{\"issuer_did\":\"some_issuer_did\",\"claim_def_seq_no\":{}}}", claim_def_seq_no);

    // 6. Prover create Claim Request
    let err =
        sovrin_prover_create_and_store_claim_req(prover_create_claim_req_command_handle,
                                                 wallet_handle,
                                                 CString::new(prover_did).unwrap().as_ptr(),
                                                 CString::new(claim_offer_json).unwrap().as_ptr(),
                                                 CString::new(claim_def_json.clone()).unwrap().as_ptr(),
                                                 CString::new(master_secret_name).unwrap().as_ptr(),
                                                 prover_create_claim_req_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, claim_req_json) = prover_create_claim_req_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    println!("claim_req_json {:?}", claim_req_json);
    assert_eq!(ErrorCode::Success, err);

    let claim_json = "{\
                           \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\
                           \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\
                           \"height\":[\"175\",\"175\"],\
                           \"age\":[\"28\",\"28\"]\
                     }";

    // 7. Issuer create Claim for Claim Request
    let err =
        sovrin_issuer_create_claim(issuer_create_claim_command_handle,
                                   wallet_handle,
                                   CString::new(claim_req_json).unwrap().as_ptr(),
                                   CString::new(claim_json).unwrap().as_ptr(),
                                   None,
                                   None,
                                   issuer_create_claim_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, revoc_reg_update_json, xclaim_json) = issuer_create_claim_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    println!("xclaim_json {:?}", xclaim_json);
    assert_eq!(ErrorCode::Success, err);

    // 7. Prover process and store Claim
    let err =
        sovrin_prover_store_claim(prover_store_claim_command_handle,
                                  wallet_handle,
                                  CString::new(xclaim_json).unwrap().as_ptr(),
                                  prover_store_claim_callback);

    assert_eq!(ErrorCode::Success, err);
    let err = prover_store_claim_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    let proof_req_json = format!("{{\
                                   \"nonce\":\"123432421212\",\
                                   \"requested_attrs\":{{\"attr1_uuid\":{{\"schema_seq_no\":{},\"name\":\"name\"}}}},\
                                   \"requested_predicates\":{{\"predicate1_uuid\":{{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}}}\
                                }}", schema_seq_no);

    // 8. Prover gets Claims for Proof Request
    let err =
        sovrin_prover_get_claims_for_proof_req(prover_get_claims_for_proof_req_handle,
                                               wallet_handle,
                                               CString::new(proof_req_json.clone()).unwrap().as_ptr(),
                                               prover_get_claims_for_proof_req_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, claims_json) = prover_get_claims_for_proof_req_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    println!("claims_json {:?}", claims_json);
    assert_eq!(ErrorCode::Success, err);

    let requested_claims_json = format!("{{\
                                          \"self_attested_attributes\":{{}},\
                                          \"requested_attrs\":{{\"attr1_uuid\":[\"{}\",true]}},\
                                          \"requested_predicates\":{{\"predicate1_uuid\":\"{}\"}}\
                                        }}", claim_def_seq_no, claim_def_seq_no);

    let schemas_json = format!("{{\"{}\":{}}}", claim_def_seq_no, schema);
    let claim_defs_json = format!("{{\"{}\":{}}}", claim_def_seq_no, claim_def_json);
    let revoc_regs_jsons = "{}";

    // 9. Prover create Proof for Proof Request
    let err =
        sovrin_prover_create_proof(prover_create_proof_handle,
                                   wallet_handle,
                                   CString::new(proof_req_json.clone()).unwrap().as_ptr(),
                                   CString::new(requested_claims_json).unwrap().as_ptr(),
                                   CString::new(schemas_json.clone()).unwrap().as_ptr(),
                                   CString::new(master_secret_name).unwrap().as_ptr(),
                                   CString::new(claim_defs_json.clone()).unwrap().as_ptr(),
                                   CString::new(revoc_regs_jsons.clone()).unwrap().as_ptr(),
                                   prover_create_proof_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, proof_json) = prover_create_proof_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    println!("proof_json {:?}", proof_json);
    assert_eq!(ErrorCode::Success, err);

    // 9. Verifier verify proof
    let err =
        sovrin_verifier_verify_proof(verifier_verify_proof_handle,
                                     wallet_handle,
                                     CString::new(proof_req_json).unwrap().as_ptr(),
                                     CString::new(proof_json).unwrap().as_ptr(),
                                     CString::new(schemas_json).unwrap().as_ptr(),
                                     CString::new(claim_defs_json).unwrap().as_ptr(),
                                     CString::new(revoc_regs_jsons).unwrap().as_ptr(),
                                     verifier_verify_proof_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, result) = verifier_verify_proof_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);
    assert!(result);

    TestUtils::cleanup_storage();
}

#[test]
#[ignore] //required nodes pool available from CI
fn ledger_demo_works() {
    TestUtils::cleanup_storage();
    let pool_name = "test_submit_tx";
    let c_pool_name = CString::new(pool_name).unwrap();

    let (submit_sender, submit_receiver) = channel();
    let (create_sender, create_receiver) = channel();
    let (open_sender, open_receiver) = channel();
    let create_cb = Box::new(move |err| { create_sender.send(err).unwrap(); });
    let open_cb = Box::new(move |err, pool_handle| { open_sender.send((err, pool_handle)).unwrap(); });
    let send_cb = Box::new(move |err, resp| { submit_sender.send((err, resp)).unwrap(); });
    let (open_command_handle, open_callback) = CallbackUtils::closure_to_open_pool_ledger_cb(open_cb);
    let (create_command_handle, create_callback) = CallbackUtils::closure_to_create_pool_ledger_cb(create_cb);
    let (command_handle, send_callback) = CallbackUtils::closure_to_send_tx_cb(send_cb);

    // 1. Create ledger config from genesis txn file
    PoolUtils::create_genesis_txn_file(pool_name);
    let pool_config = CString::new(PoolUtils::create_pool_config(pool_name)).unwrap();
    let err = sovrin_create_pool_ledger_config(create_command_handle,
                                               c_pool_name.as_ptr(),
                                               pool_config.as_ptr(),
                                               create_callback);
    assert_eq!(err, ErrorCode::Success);
    let err = create_receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
    assert_eq!(err, ErrorCode::Success);

    // 2. Open pool ledger
    let err = sovrin_open_pool_ledger(open_command_handle,
                                      c_pool_name.as_ptr(),
                                      null(),
                                      open_callback);
    assert_eq!(err, ErrorCode::Success);
    let (err, pool_handle) = open_receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
    assert_eq!(err, ErrorCode::Success);

    // 3. Send request
    let request = "{\
            \"reqId\":1491566332010860,\
            \"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\
            \"operation\":{\
                \"type\":\"105\",\
                \"dest\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\"\
            },\
            \"signature\":\"4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV\"\
        }\
        ";
    let req = CString::new(request).unwrap();
    let err = sovrin_submit_request(command_handle,
                                    pool_handle,
                                    req.as_ptr(),
                                    send_callback);
    assert_eq!(err, ErrorCode::Success);
    let (err, resp) = submit_receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
    assert_eq!(err, ErrorCode::Success);

    let act_reply: Reply = serde_json::from_str(resp.as_str()).unwrap();
    let exp_reply = Reply {
        op: "REPLY".to_string(),
        result: ReplyResult {
            req_id: 1491566332010860,
            txn_id: "5511e5493c1d37dfa67b73269a392a7aca5b71e9d10ac106adc7f9e552aee560".to_string(),
        }
    };
    assert_eq!(act_reply, exp_reply);
    TestUtils::cleanup_storage();

    #[derive(Deserialize, Eq, PartialEq, Debug)]
    struct Reply {
        op: String,
        result: ReplyResult,
    }

    #[derive(Deserialize, Eq, PartialEq, Debug)]
    #[serde(rename_all = "camelCase")]
    struct ReplyResult {
        txn_id: String,
        req_id: u64,
    }
}
