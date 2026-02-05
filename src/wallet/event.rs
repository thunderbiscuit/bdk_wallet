//! User facing wallet events.

use alloc::sync::Arc;
use alloc::vec::Vec;
use bitcoin::{Transaction, Txid};
use chain::{BlockId, ConfirmationBlockTime};

/// Events representing changes to wallet transactions.
///
/// Returned after calling
/// [`Wallet::apply_update_events`](crate::wallet::Wallet::apply_update_events).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WalletEvent {
    /// The latest chain tip known to the wallet changed.
    ChainTipChanged {
        /// Previous chain tip.
        old_tip: BlockId,
        /// New chain tip.
        new_tip: BlockId,
    },
    /// A transaction is now confirmed.
    ///
    /// If the transaction was previously unconfirmed `old_block_time` will be `None`.
    ///
    /// If a confirmed transaction is now re-confirmed in a new block `old_block_time` will contain
    /// the block id and the time it was previously confirmed. This can happen after a chain
    /// reorg.
    TxConfirmed {
        /// Transaction id.
        txid: Txid,
        /// Transaction.
        tx: Arc<Transaction>,
        /// Confirmation block time.
        block_time: ConfirmationBlockTime,
        /// Old confirmation block and time if previously confirmed in a different block.
        old_block_time: Option<ConfirmationBlockTime>,
    },
    /// A transaction is now unconfirmed.
    ///
    /// If the transaction is first seen in the mempool `old_block_time` will be `None`.
    ///
    /// If a previously confirmed transaction is now seen in the mempool `old_block_time` will
    /// contain the block id and the time it was previously confirmed. This can happen after a
    /// chain reorg.
    TxUnconfirmed {
        /// Transaction id.
        txid: Txid,
        /// Transaction.
        tx: Arc<Transaction>,
        /// Old confirmation block and time, if previously confirmed.
        old_block_time: Option<ConfirmationBlockTime>,
    },
    /// An unconfirmed transaction was replaced.
    ///
    /// This can happen after an RBF is broadcast or if a third party double spends an input of
    /// a received payment transaction before it is confirmed.
    ///
    /// The conflicts field contains the txid and vin (in which it conflicts) of the conflicting
    /// transactions.
    TxReplaced {
        /// Transaction id.
        txid: Txid,
        /// Transaction.
        tx: Arc<Transaction>,
        /// Conflicting transaction ids.
        conflicts: Vec<(usize, Txid)>,
    },
    /// Unconfirmed transaction dropped.
    ///
    /// The transaction was dropped from the local mempool. This is generally due to the fee rate
    /// being too low. The transaction can still reappear in the mempool in the future resulting in
    /// a [`WalletEvent::TxUnconfirmed`] event.
    TxDropped {
        /// Transaction id.
        txid: Txid,
        /// Transaction.
        tx: Arc<Transaction>,
    },
}
