use near_sdk::{near_bindgen, BorshStorageKey, require, AccountId, Timestamp, CryptoHash};
use near_sdk::env::{block_timestamp, signer_account_id};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use hex::{FromHex, encode};

// 1. Main Struct
// Main contract structure serialized with Borsh
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[allow(non_snake_case)]
struct RecheckRecords {
    objectRecords: UnorderedMap<CryptoHash, ObjectRecord>,
    objectSubRecords: UnorderedMap<CryptoHash, Vector<CryptoHash>>,
    trails: UnorderedMap<CryptoHash, CryptoHash>,
    e0: UnorderedMap<CryptoHash, CryptoHash>,
    e1: UnorderedMap<CryptoHash, CryptoHash>,
}

// Helper structure serialized with Borsh
#[allow(non_snake_case)]
#[derive(BorshDeserialize, BorshSerialize)]
struct ObjectRecord {
    recordId: CryptoHash,
    parentRecordId: CryptoHash,
    trail: CryptoHash,
    trailSignature: CryptoHash,
    creator: AccountId,
    timestamp: Timestamp,
    extra0: CryptoHash,
    extra1: CryptoHash,
}

// 2. Default Implementation
// Helper for default UnorderedMap and Vector
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    ObjectRecordsMap,
    ObjectSubRecordsMap,
    TrailsMap,
    E0map,
    E1map,
    SubRecordsVector,
}

#[allow(non_snake_case)]
impl Default for RecheckRecords {
    fn default() -> Self {
        Self {
            objectRecords: UnorderedMap::new(StorageKeys::ObjectRecordsMap),
            objectSubRecords: UnorderedMap::new(StorageKeys::ObjectSubRecordsMap),
            trails: UnorderedMap::new(StorageKeys::TrailsMap),
            e0: UnorderedMap::new(StorageKeys::E0map),
            e1: UnorderedMap::new(StorageKeys::E1map),
        }
    }
}

// 3. Core Logic
// Helper functions
impl RecheckRecords {
    fn only_unique_records(contract_self: &RecheckRecords, record_id_str: &String) -> CryptoHash {
        let record_id: CryptoHash = <CryptoHash>::from_hex(record_id_str)
            .expect("Invalid recordId hash.");
        require!(contract_self.objectRecords.get(&record_id).is_none(), "Record must be unique.");
        return record_id;
    }

    fn string_to_hex_bytes(str: String) -> CryptoHash {
        let hex_bytes: CryptoHash = <CryptoHash>::from_hex(str).expect("Invalid hex string.");
        return hex_bytes;
    }

    fn hex_bytes_to_string(hex_bytes: CryptoHash) -> String {
        let string: String = encode(hex_bytes);
        return string;
    }

    fn null_record() -> (String,
                         String,
                         String,
                         String,
                         AccountId,
                         Timestamp,
                         u64) {
        let null_account_id: AccountId = "none.none".parse().unwrap();

        return (
            String::from("0"),
            String::from("0"),
            String::from("0"),
            String::from("0"),
            null_account_id,
            0,
            0
        );
    }
}

//Write functions
#[near_bindgen]
#[allow(non_snake_case)]
impl RecheckRecords {
    #[private]
    #[allow(non_snake_case)]
    pub fn createSubRecordWithExtras2(&mut self,
                                      record_id_str: String,
                                      parent_record_id_str: String,
                                      trail_str: String,
                                      trail_signature_str: String,
                                      extra_0_str: String,
                                      extra_1_str: String) {
        let record_id: CryptoHash = RecheckRecords::only_unique_records(&self, &record_id_str);

        let parent_record_id: CryptoHash = RecheckRecords::string_to_hex_bytes(parent_record_id_str);
        let trail: CryptoHash = RecheckRecords::string_to_hex_bytes(trail_str);
        let trail_signature: CryptoHash = RecheckRecords::string_to_hex_bytes(trail_signature_str);
        let extra_0: CryptoHash = RecheckRecords::string_to_hex_bytes(extra_0_str);
        let extra_1: CryptoHash = RecheckRecords::string_to_hex_bytes(extra_1_str);

        let record = ObjectRecord {
            recordId: record_id,
            parentRecordId: parent_record_id,
            trail,
            trailSignature: trail_signature,
            creator: signer_account_id(),
            timestamp: block_timestamp(),
            extra0: extra_0,
            extra1: extra_1,
        };

        self.objectRecords.insert(&record_id, &record);

        if record_id != parent_record_id {
            if self.objectSubRecords.get(&parent_record_id).is_none() {
                let sub_records_empty_vec: Vector<CryptoHash> = Vector::new(StorageKeys::SubRecordsVector);
                self.objectSubRecords.insert(&parent_record_id, &sub_records_empty_vec);
            }

            let mut new_sub_records: Vector<CryptoHash> = self.objectSubRecords.get(&parent_record_id)
                .expect("No parent record found!!!");
            new_sub_records.push(&record_id);

            self.objectSubRecords.insert(&parent_record_id, &new_sub_records);
        }

        self.trails.insert(&trail, &record_id);
        self.e0.insert(&extra_0, &record_id);
        self.e1.insert(&extra_1, &record_id);
    }

    #[private]
    #[allow(non_snake_case)]
    pub fn createSubRecord(&mut self,
                           record_id_str: String,
                           parent_record_id_str: String,
                           trail_str: String,
                           trail_signature_str: String) {
        RecheckRecords::only_unique_records(&self, &record_id_str);

        let extra_0_str: String = trail_str.clone();
        let extra_1_str: String = trail_str.clone();

        return RecheckRecords::createSubRecordWithExtras2(self,
                                                          record_id_str,
                                                          parent_record_id_str,
                                                          trail_str,
                                                          trail_signature_str,
                                                          extra_0_str,
                                                          extra_1_str);
    }

    #[private]
    #[allow(non_snake_case)]
    pub fn createRecord(&mut self,
                        record_id_str: String,
                        trail_str: String,
                        trail_signature_str: String) {
        RecheckRecords::only_unique_records(&self, &record_id_str);

        let parent_record_id_str: String = record_id_str.clone();
        let extra_0_str: String = trail_str.clone();
        let extra_1_str: String = trail_str.clone();

        return RecheckRecords::createSubRecordWithExtras2(self,
                                                          record_id_str,
                                                          parent_record_id_str,
                                                          trail_str,
                                                          trail_signature_str,
                                                          extra_0_str,
                                                          extra_1_str);
    }
}

//Read-only functions
#[near_bindgen]
#[allow(non_snake_case)]
impl RecheckRecords {
    #[allow(non_snake_case)]
    pub fn records(self, record_id_str: String) -> (String,
                                                    String,
                                                    String,
                                                    String,
                                                    AccountId,
                                                    Timestamp,
                                                    u64) {
        let record_id_hash: CryptoHash = RecheckRecords::string_to_hex_bytes(record_id_str);

        if self.objectRecords.get(&record_id_hash).is_none() {
            return RecheckRecords::null_record();
        }


        let record: ObjectRecord = self.objectRecords.get(&record_id_hash)
            .expect("None existing record");

        let record_id: String = RecheckRecords::hex_bytes_to_string(record.recordId);
        let parent_record_id: String = RecheckRecords::hex_bytes_to_string(record.parentRecordId);
        let trail: String = RecheckRecords::hex_bytes_to_string(record.trail);
        let trail_signature: String = RecheckRecords::hex_bytes_to_string(record.trailSignature);
        let creator: AccountId = record.creator;
        let timestamp: Timestamp = record.timestamp;
        let mut sub_records_length: u64 = 0;

        if !self.objectSubRecords.get(&record_id_hash).is_none() {
            sub_records_length = self.objectSubRecords.get(&record_id_hash).unwrap().len();
        }

        return (record_id, parent_record_id, trail, trail_signature, creator, timestamp, sub_records_length);
    }

    #[allow(non_snake_case)]
    pub fn subRecord(self, sub_record_id_str: String, index: u64) -> (String,
                                                                      String,
                                                                      String,
                                                                      String,
                                                                      AccountId,
                                                                      Timestamp,
                                                                      u64) {
        let sub_record_id: CryptoHash = RecheckRecords::string_to_hex_bytes(sub_record_id_str);

        if self.objectSubRecords.get(&sub_record_id).is_none() {
            return RecheckRecords::null_record();
        }


        let sub_records: Vector<CryptoHash> = self.objectSubRecords.get(&sub_record_id)
            .expect("None existing sub record");

        if !sub_records.get(index).is_none() {
            return RecheckRecords::null_record();
        }

        let sub_record_id_str: String = RecheckRecords::hex_bytes_to_string(sub_record_id);

        return RecheckRecords::records(self, sub_record_id_str);
    }

    #[allow(non_snake_case)]
    pub fn verifyTrail(self, trail_str: String) -> (String,
                                                    String,
                                                    String,
                                                    String,
                                                    AccountId,
                                                    Timestamp,
                                                    u64) {
        let trail: CryptoHash = RecheckRecords::string_to_hex_bytes(trail_str);

        if self.trails.get(&trail).is_none() {
            return RecheckRecords::null_record();
        }


        let record_id: CryptoHash = self.trails.get(&trail)
            .expect("None existing record");

        let record_id_str: String = RecheckRecords::hex_bytes_to_string(record_id);

        return RecheckRecords::records(self, record_id_str);
    }

    #[allow(non_snake_case)]
    pub fn verifyExtra0(self, extra_0_str: String) -> (String,
                                                       String,
                                                       String,
                                                       String,
                                                       AccountId,
                                                       Timestamp,
                                                       u64) {
        let extra_0: CryptoHash = RecheckRecords::string_to_hex_bytes(extra_0_str);

        if self.e0.get(&extra_0).is_none() {
            return RecheckRecords::null_record();
        }


        let record_id: CryptoHash = self.e0.get(&extra_0)
            .expect("None existing record");

        let record_id_str: String = RecheckRecords::hex_bytes_to_string(record_id);

        return RecheckRecords::records(self, record_id_str);
    }

    #[allow(non_snake_case)]
    pub fn verifyExtra1(self, extra_1_str: String) -> (String,
                                                       String,
                                                       String,
                                                       String,
                                                       AccountId,
                                                       Timestamp,
                                                       u64) {
        let extra_1: CryptoHash = RecheckRecords::string_to_hex_bytes(extra_1_str);

        if self.e1.get(&extra_1).is_none() {
            return RecheckRecords::null_record();
        }

        let record_id: CryptoHash = self.e1.get(&extra_1)
            .expect("None existing record");

        let record_id_str: String = RecheckRecords::hex_bytes_to_string(record_id);

        return RecheckRecords::records(self, record_id_str);
    }
}

// 4. Tests
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{log, testing_env};

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.current_account_id(predecessor_account_id.clone())
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);

        return builder;
    }

    fn create_random_hash_string() -> String {
        use rand::Rng;

        const HEX_CHARSET: &[u8] = b"abcdef0123456789";
        const HASH_LEN: usize = 64;
        let mut rng = rand::thread_rng();

        let random_hash: String = (0..HASH_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..HEX_CHARSET.len());
                HEX_CHARSET[idx] as char
            })
            .collect();

        return random_hash;
    }

    #[test]
    fn create_and_get_new_record() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = RecheckRecords::default();
        testing_env!(context.is_view(false).build());

        let random_record_id = create_random_hash_string();
        log!("random_record_id{:?}",random_record_id);
        let random_trail = create_random_hash_string();
        log!("random_trail{:?}",random_trail);
        let random_trail_signature = create_random_hash_string();
        log!("random_trail_str{:?}",random_record_id);

        contract.createRecord(random_record_id.clone(),
                              random_trail.clone(),
                              random_trail_signature.clone());

        let result = contract.records(random_record_id.clone());

        log!("result{:?}",result);

        let expected = (
            String::from(random_record_id.clone()),
            String::from(random_record_id.clone()),
            String::from(random_trail.clone()),
            String::from(random_trail_signature.clone()),
            accounts(1),
            0,
            0
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn create_and_get_new_sub_record() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = RecheckRecords::default();
        testing_env!(context.is_view(false).build());

        let random_parent_record_id = create_random_hash_string();
        log!("random_parent_record_id{:?}",random_parent_record_id);
        let random_parent_trail = create_random_hash_string();
        log!("random_parent_trail{:?}",random_parent_trail);
        let random_parent_trail_signature = create_random_hash_string();
        log!("random_parent_trail_signature{:?}",random_parent_record_id);

        contract.createRecord(random_parent_record_id.clone(),
                              random_parent_trail.clone(),
                              random_parent_trail_signature.clone());


        let random_sub_record_id = create_random_hash_string();
        log!("random_sub_record_id{:?}",random_sub_record_id);
        let random_sub_trail = create_random_hash_string();
        log!("random_sub_trail{:?}",random_sub_trail);
        let random_sub_trail_signature = create_random_hash_string();
        log!("random_trail_str{:?}",random_parent_record_id);

        contract.createSubRecord(random_sub_record_id.clone(),
                                 random_parent_record_id.clone(),
                                 random_sub_trail.clone(),
                                 random_sub_trail_signature.clone());

        let result = contract.subRecord(random_parent_record_id.clone(), 1);

        log!("result{:?}",result);

        let expected = (
            String::from(random_parent_record_id.clone()),
            String::from(random_parent_record_id.clone()),
            String::from(random_parent_trail.clone()),
            String::from(random_parent_trail_signature.clone()),
            accounts(1),
            0,
            1
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn create_and_get_new_record_with_extras_2() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = RecheckRecords::default();
        testing_env!(context.is_view(false).build());

        let random_record_id = create_random_hash_string();
        log!("random_record_id{:?}",random_record_id);
        let random_trail = create_random_hash_string();
        log!("random_trail{:?}",random_trail);
        let random_trail_signature = create_random_hash_string();
        log!("random_parent_trail_signature{:?}",random_trail_signature);
        let random_extra_0 = create_random_hash_string();
        log!("random_extra_0{:?}",random_extra_0);
        let random_extra_1 = create_random_hash_string();
        log!("random_extra_1{:?}",random_extra_1);

        contract.createSubRecordWithExtras2(random_record_id.clone(),
                                            random_record_id.clone(),
                                            random_trail.clone(),
                                            random_trail_signature.clone(),
                                            random_extra_0.clone(),
                                            random_extra_1.clone());

        let result = contract.records(random_record_id.clone());

        log!("result{:?}",result);

        let expected = (
            String::from(random_record_id.clone()),
            String::from(random_record_id.clone()),
            String::from(random_trail.clone()),
            String::from(random_trail_signature.clone()),
            accounts(1),
            0,
            0
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn create_and_verify_trail() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = RecheckRecords::default();
        testing_env!(context.is_view(false).build());

        let random_record_id = create_random_hash_string();
        log!("random_record_id{:?}",random_record_id);
        let random_trail = create_random_hash_string();
        log!("random_trail{:?}",random_trail);
        let random_trail_signature = create_random_hash_string();
        log!("random_trail_str{:?}",random_record_id);

        contract.createRecord(random_record_id.clone(),
                              random_trail.clone(),
                              random_trail_signature.clone());

        let result = contract.verifyTrail(random_trail.clone());

        log!("result{:?}",result);

        let expected = (
            String::from(random_record_id.clone()),
            String::from(random_record_id.clone()),
            String::from(random_trail.clone()),
            String::from(random_trail_signature.clone()),
            accounts(1),
            0,
            0
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn create_and_verify_extra_0() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = RecheckRecords::default();
        testing_env!(context.is_view(false).build());

        let random_record_id = create_random_hash_string();
        log!("random_record_id{:?}",random_record_id);
        let random_trail = create_random_hash_string();
        log!("random_trail{:?}",random_trail);
        let random_trail_signature = create_random_hash_string();
        log!("random_parent_trail_signature{:?}",random_trail_signature);
        let random_extra_0 = create_random_hash_string();
        log!("random_extra_0{:?}",random_extra_0);
        let random_extra_1 = create_random_hash_string();
        log!("random_extra_1{:?}",random_extra_1);

        contract.createSubRecordWithExtras2(random_record_id.clone(),
                                            random_record_id.clone(),
                                            random_trail.clone(),
                                            random_trail_signature.clone(),
                                            random_extra_0.clone(),
                                            random_extra_1.clone());

        let result = contract.verifyExtra0(random_extra_0.clone());

        log!("result{:?}",result);

        let expected = (
            String::from(random_record_id.clone()),
            String::from(random_record_id.clone()),
            String::from(random_trail.clone()),
            String::from(random_trail_signature.clone()),
            accounts(1),
            0,
            0
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn create_and_verify_extra_1() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = RecheckRecords::default();
        testing_env!(context.is_view(false).build());

        let random_record_id = create_random_hash_string();
        log!("random_record_id{:?}",random_record_id);
        let random_trail = create_random_hash_string();
        log!("random_trail{:?}",random_trail);
        let random_trail_signature = create_random_hash_string();
        log!("random_parent_trail_signature{:?}",random_trail_signature);
        let random_extra_0 = create_random_hash_string();
        log!("random_extra_0{:?}",random_extra_0);
        let random_extra_1 = create_random_hash_string();
        log!("random_extra_1{:?}",random_extra_1);

        contract.createSubRecordWithExtras2(random_record_id.clone(),
                                            random_record_id.clone(),
                                            random_trail.clone(),
                                            random_trail_signature.clone(),
                                            random_extra_0.clone(),
                                            random_extra_1.clone());

        let result = contract.verifyExtra1(random_extra_1.clone());

        log!("result{:?}",result);

        let expected = (
            String::from(random_record_id.clone()),
            String::from(random_record_id.clone()),
            String::from(random_trail.clone()),
            String::from(random_trail_signature.clone()),
            accounts(1),
            0,
            0
        );

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "Record must be unique.")]
    fn create_non_unique_record() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = RecheckRecords::default();
        testing_env!(context.is_view(false).build());

        let random_record_id = create_random_hash_string();
        log!("random_record_id{:?}",random_record_id);
        let random_trail = create_random_hash_string();
        log!("random_trail{:?}",random_trail);
        let random_trail_signature = create_random_hash_string();
        log!("random_trail_str{:?}",random_record_id);

        contract.createRecord(random_record_id.clone(),
                              random_trail.clone(),
                              random_trail_signature.clone());

        let random_trail_2 = create_random_hash_string();
        log!("random_trail_2{:?}",random_trail_2);
        let random_trail_signature_2 = create_random_hash_string();
        log!("random_trail_signature_2{:?}",random_trail_signature_2);

        contract.createRecord(random_record_id.clone(),
                              random_trail_2.clone(),
                              random_trail_signature_2.clone());
    }

    #[test]
    fn get_non_existing_record() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = RecheckRecords::default();
        testing_env!(context.is_view(true).build());

        let random_record_id = create_random_hash_string();
        log!("random_record_id{:?}",random_record_id);

        let result = contract.records(random_record_id);
        log!("result{:?}",result);

        let expected = RecheckRecords::null_record();

        assert_eq!(result, expected);
    }

    #[test]
    fn verify_wrong_trail() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = RecheckRecords::default();
        testing_env!(context.is_view(false).build());

        let random_record_id = create_random_hash_string();
        log!("random_record_id{:?}",random_record_id);
        let random_trail = create_random_hash_string();
        log!("random_trail{:?}",random_trail);
        let random_trail_signature = create_random_hash_string();
        log!("random_trail_str{:?}",random_record_id);

        contract.createRecord(random_record_id.clone(),
                              random_trail.clone(),
                              random_trail_signature.clone());

        let result = contract.verifyTrail(random_trail_signature.clone());

        log!("result{:?}",result);

        let expected = RecheckRecords::null_record();

        assert_eq!(result, expected);
    }

    #[test]
    fn verify_wrong_extra_0() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = RecheckRecords::default();
        testing_env!(context.is_view(false).build());

        let random_record_id = create_random_hash_string();
        log!("random_record_id{:?}",random_record_id);
        let random_trail = create_random_hash_string();
        log!("random_trail{:?}",random_trail);
        let random_trail_signature = create_random_hash_string();
        log!("random_parent_trail_signature{:?}",random_trail_signature);
        let random_extra_0 = create_random_hash_string();
        log!("random_extra_0{:?}",random_extra_0);
        let random_extra_1 = create_random_hash_string();
        log!("random_extra_1{:?}",random_extra_1);

        contract.createSubRecordWithExtras2(random_record_id.clone(),
                                            random_record_id.clone(),
                                            random_trail.clone(),
                                            random_trail_signature.clone(),
                                            random_extra_0.clone(),
                                            random_extra_1.clone());

        let result = contract.verifyExtra0(random_extra_1.clone());

        log!("result{:?}",result);

        let expected = RecheckRecords::null_record();

        assert_eq!(result, expected);
    }

    #[test]
    fn verify_wrong_extra_1() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = RecheckRecords::default();
        testing_env!(context.is_view(false).build());

        let random_record_id = create_random_hash_string();
        log!("random_record_id{:?}",random_record_id);
        let random_trail = create_random_hash_string();
        log!("random_trail{:?}",random_trail);
        let random_trail_signature = create_random_hash_string();
        log!("random_parent_trail_signature{:?}",random_trail_signature);
        let random_extra_0 = create_random_hash_string();
        log!("random_extra_0{:?}",random_extra_0);
        let random_extra_1 = create_random_hash_string();
        log!("random_extra_1{:?}",random_extra_1);

        contract.createSubRecordWithExtras2(random_record_id.clone(),
                                            random_record_id.clone(),
                                            random_trail.clone(),
                                            random_trail_signature.clone(),
                                            random_extra_0.clone(),
                                            random_extra_1.clone());

        let result = contract.verifyExtra1(random_extra_0.clone());

        log!("result{:?}",result);

        let expected = RecheckRecords::null_record();

        assert_eq!(result, expected);
    }

    #[test]
    fn check_string_to_hex_to_string_conversion() {
        let hex_string_input: String = "a0ac5893c435ce0506ba227018f5d0b61e371bdffdb91030b8b502db632ee020".to_string();

        let result_hex_bytes: CryptoHash = RecheckRecords::string_to_hex_bytes(hex_string_input.clone());

        log!("result bytes{:?}",result_hex_bytes);

        let expected_hex_bytes: CryptoHash = [160, 172, 88, 147, 196, 53, 206, 5, 6, 186, 34, 112, 24, 245, 208, 182, 30, 55, 27, 223, 253, 185, 16, 48, 184, 181, 2, 219, 99, 46, 224, 32];

        assert_eq!(result_hex_bytes, expected_hex_bytes);

        let hex_string_result: String = RecheckRecords::hex_bytes_to_string(result_hex_bytes.clone
        ());

        log!("result string{:?}",hex_string_result);

        assert_eq!(hex_string_result, hex_string_input);
    }
}
