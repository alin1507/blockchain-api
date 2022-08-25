use super::block_chain_errors::BlockChainError;
use super::transaction::TransactionInfo;
use super::wallet::{Wallet, WalletCoins, WalletInfo};
use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::vec;

lazy_static! {
    pub static ref BLOCKCHAIN: Mutex<BlockChain> = Mutex::new(BlockChain::default());
}

pub const MINING_ADDRESS: &str = "MINING";

pub struct BlockChain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: u32,
    pub name: String,
    pub wallets: Vec<Wallet>,
}

impl Default for BlockChain {
    fn default() -> Self {
        let genesis_block = Block::new(0, &vec![]);
        Self {
            chain: vec![genesis_block],
            difficulty: 2,
            pending_transactions: vec![],
            mining_reward: 100,
            name: "Blockchain".to_string(),
            wallets: vec![],
        }
    }
}

impl BlockChain {
    /**
     * Mine transactions that are pending
     * Reward the user that mined the block with the mining reward for this blockchain
     * Add a new transaction on pending transactions
     */
    pub fn mine_pending_transactions(
        &mut self,
        mining_reward_address: &String,
    ) -> Result<String, BlockChainError> {
        if self.pending_transactions.is_empty() {
            return Err(BlockChainError::NoPendingTransactions);
        }

        for transaction in self.pending_transactions.clone() {
            let from_wallet = transaction.from_wallet;
            let mut to_wallet = match self.get_wallet(&transaction.to_wallet.address) {
                Some(wallet) => wallet,
                None => return Err(BlockChainError::InvalidToAddress),
            };

            to_wallet.balance += transaction.amount;
            to_wallet.transactions.push(TransactionInfo {
                from_address: from_wallet.address.clone(),
                from_password: from_wallet.password.clone(),
                to_address: to_wallet.address.clone(),
                amount: transaction.amount,
            });

            self.update_wallet(to_wallet.clone())?;
        }

        let mining_reward_wallet = match self.get_wallet(mining_reward_address) {
            Some(wallet) => wallet,
            None => return Err(BlockChainError::InvalidRewardAddress),
        };

        let mut block = Block::new(self.chain.len(), &self.pending_transactions);

        match self.chain.last() {
            Some(latest_block) => {
                block.set_previous_hash(&latest_block.hash);
            }
            None => {}
        };

        block.mine_block(self.difficulty);

        self.chain.push(block);
        self.pending_transactions = vec![Transaction::new(
            Wallet::new(MINING_ADDRESS.to_string(), 0, "".to_string()),
            mining_reward_wallet,
            self.mining_reward,
        )];

        Ok("Transactions successfully mined".to_string())
    }

    /**
     * Create a new transaction and add it to pending transactions
     */
    pub fn create_transaction(
        &mut self,
        transaction: TransactionInfo,
    ) -> Result<String, BlockChainError> {
        match transaction.check_transaction_info() {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        let mut from_wallet = match self.get_wallet(&transaction.from_address) {
            Some(wallet) => wallet,
            None => return Err(BlockChainError::InvalidFromAddress),
        };

        if from_wallet.password != transaction.from_password {
            return Err(BlockChainError::WrongPassword);
        }

        let to_wallet = match self.get_wallet(&transaction.to_address) {
            Some(wallet) => wallet,
            None => return Err(BlockChainError::InvalidToAddress),
        };

        if from_wallet.balance < transaction.amount {
            return Err(BlockChainError::NotEnoughCoins);
        };

        if transaction.amount < 0 {
            return Err(BlockChainError::NegativeAmount);
        };

        if from_wallet.address != MINING_ADDRESS.to_string() {
            from_wallet.balance -= transaction.amount;
            from_wallet.transactions.push(TransactionInfo {
                from_address: from_wallet.address.clone(),
                from_password: from_wallet.password.clone(),
                to_address: to_wallet.address.clone(),
                amount: transaction.amount,
            });
            self.update_wallet(from_wallet.clone())?;
        }

        let new_transaction = Transaction::new(from_wallet, to_wallet, transaction.amount);
        self.pending_transactions.push(new_transaction);

        Ok("Transaction successfully made".to_string())
    }

    /**
     * Create a new wallet
     */
    pub fn create_wallet(&mut self, wallet: WalletInfo) -> Result<String, BlockChainError> {
        match wallet.check_wallet_info() {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        match self.get_wallet(&wallet.address) {
            Some(_) => return Err(BlockChainError::WalletAlreadyExists),
            None => (),
        }

        let new_wallet = Wallet::new(wallet.address, wallet.balance as u32, wallet.password);
        self.wallets.push(new_wallet);

        Ok("Wallet created!".to_string())
    }

    /**
     * Return wallet ballance based on the address and password
     */
    pub fn get_balance_of_wallet(
        &self,
        address: &String,
        password: &String,
    ) -> Result<u32, BlockChainError> {
        let wallet = match self.get_wallet(address) {
            Some(wallet) => wallet,
            None => return Err(BlockChainError::WalletNotFound),
        };

        if wallet.password != password.to_string() {
            return Err(BlockChainError::WrongPassword);
        }

        Ok(wallet.balance)
    }

    /**
     * Return wallet transactions based on address and password
     */
    pub fn get_transactions_of_wallet(
        &self,
        address: &String,
        password: &String,
    ) -> Result<Vec<TransactionInfo>, BlockChainError> {
        let wallet = match self.get_wallet(address) {
            Some(wallet) => wallet,
            None => return Err(BlockChainError::WalletNotFound),
        };

        if wallet.password != password.to_string() {
            return Err(BlockChainError::WrongPassword);
        }

        Ok(wallet.transactions)
    }

    /**
     * Return wallet based on address
     */
    pub fn get_wallet(&self, address: &String) -> Option<Wallet> {
        for wallet in self.wallets.clone() {
            if wallet.address == address.to_string() {
                return Some(wallet);
            }
        }
        None
    }

    pub fn add_coins(&mut self, add_coins: WalletCoins) -> Result<String, BlockChainError> {
        let mut wallet = match self.get_wallet(&add_coins.address) {
            Some(wallet) => wallet,
            None => return Err(BlockChainError::WalletNotFound),
        };

        if wallet.password != add_coins.password.to_string() {
            return Err(BlockChainError::WrongPassword);
        }

        if add_coins.coins < 0 {
            return Err(BlockChainError::NegativeAmount);
        }

        wallet.balance += add_coins.coins;

        self.update_wallet(wallet)?;

        Ok("Coins added!".to_string())
    }

    /**
     * Update wallet from the blockchain
     */
    pub fn update_wallet(&mut self, wallet: Wallet) -> Result<(), BlockChainError> {
        match self
            .wallets
            .iter()
            .position(|w| *w.address == wallet.address)
        {
            Some(index) => self.wallets[index] = wallet,
            None => return Err(BlockChainError::WalletNotFound),
        }

        Ok(())
    }
}
