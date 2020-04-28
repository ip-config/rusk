#![no_std]
use cake_rusk as cake;

#[cake::contract(version = "0.0.1")]
mod bid {
    use dusk_abi::Signature;
    use phoenix_abi::{Input, Note, Proof, PublicKey};

    type Inputs = [Input; Input::MAX];
    type Notes = [Note; Note::MAX];

    pub fn bid(
        inputs: Inputs,
        notes: Notes,
        proof: Proof,
        pk: PublicKey,
        expiry_height: u64,
        // m: M,
    ) -> i32 {
        if m.address != dusk_abi::self_hash() {
            return 0;
        }

        // TODO: this should call `transfer_from`
        if dusk_abi::call_contract::<(Inputs, Notes, Proof), i32>(
            &transfer_contract,
            0,
            (inputs, notes, proof),
        ) == 0
        {
            return 0;
        }

        // TODO: create bid
        dusk_abi::set_storage(&pk, bid);
        1
    }

    pub fn withdraw(
        notes: Notes,
        proof: Proof,
        pk: PublicKey,
        sig: Signature,
    ) -> i32 {
        let bid = dusk_abi::get_storage(&pk);
        if bid.is_none() {
            return 0;
        }

        if bid.expiry_height < current_height {
            return 0;
        }

        // if !dusk_abi::verify_ed25519_signature(sig, pk, msg) {
        //     0
        // }

        if dusk_abi::call_contract::<(Notes, Proof), i32>(
            &transfer_contract,
            0,
            (notes, proof),
        ) == 0
        {
            return 0;
        }

        dusk_abi::delete_storage(&pk);
        1
    }
}
