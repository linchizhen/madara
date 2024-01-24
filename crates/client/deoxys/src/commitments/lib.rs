use mp_felt::Felt252Wrapper;
use mp_hashers::HasherT;
use mp_transactions::Transaction;
use starknet_api::transaction::Event;

use super::transactions::calculate_transaction_commitment;
use super::events::calculate_event_commitment;


/// Calculate the transaction commitment, the event commitment and the event count.
///
/// # Arguments
///
/// * `transactions` - The transactions of the block
///
/// # Returns
///
/// The transaction commitment, the event commitment and the event count.
pub fn calculate_commitments<H: HasherT>(
    transactions: &[Transaction],
    events: &[Event],
    chain_id: Felt252Wrapper,
    block_number: u64,
) -> (Felt252Wrapper, Felt252Wrapper) {
    (
        calculate_transaction_commitment::<H>(transactions, chain_id, block_number),
        calculate_event_commitment::<H>(events),
    )
}