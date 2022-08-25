use super::block_chain_errors::BlockChainError;
use super::transaction::TransactionInfo;
use super::wallet::{Wallet, WalletCoins, WalletInfo, AddressType};
use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::vec;

//THE ONLY 'INSTANCE' OF THE BLOCKCHAIN THAT EXISTS THROW ALL THE PROGRAM
lazy_static! {
    pub static ref BLOCKCHAIN: Mutex<BlockChain> = Mutex::new(BlockChain::default());
}

//THE ADDRESS RESERVED FOR GIVING MINERS REWARDS
pub const MINING_ADDRESS: &str = "MINING";

//HOLDS ALL THE INFORMATION FROM THE BLOCKCHAIN
pub struct BlockChain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: u32,
    pub name: String,
    pub wallets: Vec<Wallet>,
}

//THE DEFAULT VALUES USED FOR THE BLOCKCHAIN
impl Default for BlockChain {
    fn default() -> Self {
        //THE BLOCKCHAIN STARTS WITH AN EMPTY BLOCK CALLED THE GENESIS BLOCK
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
    //MINE THE PENDING TRANSACTIONS, REWARD THE MINER WITH THE MINING REWARD AMOUNT SPECIFIED IN THE DEFAULT. ADD A NEW TRANSACTION WITH THE MINER REWARD TO PENDING TRANSACTIONS
    pub fn mine_pending_transactions(
        &mut self,
        mining_reward_address: &String,
    ) -> Result<String, BlockChainError> {
        if self.pending_transactions.is_empty() {
            return Err(BlockChainError::NoPendingTransactions);
        }

        //UPDATE THE 'TO' WALLETS FOUND IN THE PENDING TRANSACTIONS
        for transaction in self.pending_transactions.clone() {
            let from_wallet = transaction.from_wallet;
            let mut to_wallet = self.get_wallet(&transaction.to_wallet.address, AddressType::TO)?;

            to_wallet.balance += transaction.amount;
            to_wallet.transactions.push(TransactionInfo {
                from_address: from_wallet.address.clone(),
                from_password: from_wallet.password.clone(),
                to_address: to_wallet.address.clone(),
                amount: transaction.amount,
            });

            self.update_wallet(to_wallet.clone())?;
        }

        //SEARCH FOR THE MINING REWARD ADDRESS AND RETURN AN ERROR IF IS NOT FOUND
        let mining_reward_wallet = self.get_wallet(mining_reward_address, AddressType::REWARD)?;

        //CREATE BLOCK AND SET THE PREVIOUS HASH
        let mut block = Block::new(self.chain.len(), &self.pending_transactions);
        match self.chain.last() {
            Some(latest_block) => {
                block.set_previous_hash(&latest_block.hash);
            }
            None => {}
        };

        //SET HASH FOR THE BLOCK
        block.mine_block(self.difficulty);

        //PUSH THE BLOCK TO THE CHAIN
        self.chain.push(block);

        //ADD A NEW BLOCK WITH THE TRANSACTION FOR THE MINER REWARD TO PENDING TRANSACTIONS
        self.pending_transactions = vec![Transaction::new(
            Wallet::new(MINING_ADDRESS.to_string(), 0, "".to_string()),
            mining_reward_wallet,
            self.mining_reward,
        )];

        Ok("Transactions successfully mined".to_string())
    }

    //CREATE A NEW TRANSACTION AND ADD IT TO PENDING TRANSACTIONS
    pub fn create_transaction(
        &mut self,
        transaction: TransactionInfo,
    ) -> Result<String, BlockChainError> {
        //CHECK IF THE TRANSACTION IS VALID
        match transaction.check_transaction_info() {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        //GET THE 'FROM' WALLET AND RETURN AN ERROR IF IS NOT FOUND
        let mut from_wallet = self.get_wallet(&transaction.from_address, AddressType::FROM)?;

        //RETURN AN ERROR IF THE PASSWORD IS WRONG
        if from_wallet.password != transaction.from_password {
            return Err(BlockChainError::WrongPassword);
        }

        //GET THE 'TO' WALLET AND RETURN AN ERROR IF IS NOT FOUND
        let to_wallet = self.get_wallet(&transaction.to_address, AddressType::TO)?;

        //CHECK IF THE 'FROM' WALLET HAVE ENOUGH COINS FOR THIS TRANSACTION
        if from_wallet.balance < transaction.amount {
            return Err(BlockChainError::NotEnoughCoins);
        };

        //CHECK IF THE AMOUNT IS LESS THAN 0
        #[allow(unused_comparisons)]
        if transaction.amount < 0 {
            return Err(BlockChainError::NegativeAmount);
        };

        //SUBTRACT THE AMOUNT SEND FROM THE 'FROM WALLET
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

        //CREATE THE TRANSACTION AND ADD IT TO PENDING TRANSACTIONS
        let new_transaction = Transaction::new(from_wallet, to_wallet, transaction.amount);
        self.pending_transactions.push(new_transaction);

        Ok("Transaction successfully made".to_string())
    }

    //CREATE A NEW  WALLET
    pub fn create_wallet(&mut self, wallet: WalletInfo) -> Result<String, BlockChainError> {
        //CHECK IF THE WALLET INFORMATION ARE VALID
        match wallet.check_wallet_info() {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        //CHECK IF AN WALLET WITH THE SAME ADDRESS ALREADY EXISTS
        match self.get_wallet(&wallet.address, AddressType::GENERIC) {
            Ok(_) => return Err(BlockChainError::WalletAlreadyExists),
            Err(_) => (),
        }

        //CREATE THE WALLET
        let new_wallet = Wallet::new(wallet.address, wallet.balance as u32, wallet.password);
        self.wallets.push(new_wallet);

        Ok("Wallet created!".to_string())
    }

    //RETURN WALLET BALLANCE BASED ON THE ADDRESS AND PASSWORD
    pub fn get_balance_of_wallet(
        &self,
        address: &String,
        password: &String,
    ) -> Result<u32, BlockChainError> {
        //CHECK IF THE WALLET EXISTS
        let wallet = self.get_wallet(address, AddressType::GENERIC)?;

        //CHECK IF THE PASSWORD IS GOOD
        if wallet.password != password.to_string() {
            return Err(BlockChainError::WrongPassword);
        }

        //RETURN THE BALANCE
        Ok(wallet.balance)
    }

    //RETURN WALLET TRANSACTIONS BASED ON THE ADDRESS AND PASSWORD
    pub fn get_transactions_of_wallet(
        &self,
        address: &String,
        password: &String,
    ) -> Result<Vec<TransactionInfo>, BlockChainError> {
        //CHECK IF THE WALLET EXISTS
        let wallet = self.get_wallet(address, AddressType::GENERIC)?;

        //CHECK IF THE PASSWORD IS GOOD
        if wallet.password != password.to_string() {
            return Err(BlockChainError::WrongPassword);
        }

        //RETURN THE TRANSACTIONS
        Ok(wallet.transactions)
    }

    //RETURN THE WALLET BASED ON ADDRESS
    pub fn get_wallet(
        &self,
        address: &String,
        address_type: AddressType,
    ) -> Result<Wallet, BlockChainError> {
        //INITIALIZE THE WALLET WITH NONE
        let mut wallet = None;

        //CHECK IF AN WALLET WITH THE GIVEN ADDRESS EXISTS
        for current_wallet in self.wallets.clone() {
            if current_wallet.address == address.to_string() {
                wallet = Some(current_wallet);
            }
        }

        //IF AN WALLET WITH THE GIVEN ADDRESS WAS FOUND IS RETURNED, IF NOT AN ERROR IS RETURNED
        match wallet {
            Some(wallet) => Ok(wallet),
            None => match address_type {
                AddressType::TO => return Err(BlockChainError::InvalidToAddress),
                AddressType::FROM => return Err(BlockChainError::InvalidFromAddress),
                AddressType::REWARD => return Err(BlockChainError::InvalidRewardAddress),
                AddressType::GENERIC => return Err(BlockChainError::WalletNotFound),
            },
        }
    }

    //ADD COINS TO AN WALLET
    pub fn add_coins(&mut self, add_coins: WalletCoins) -> Result<String, BlockChainError> {
        //CHECK IF THE WALLET EXISTS
        let mut wallet = self.get_wallet(&add_coins.address, AddressType::GENERIC)?;

        //CHECK IF THE PASSWORD IS CORRECT
        if wallet.password != add_coins.password.to_string() {
            return Err(BlockChainError::WrongPassword);
        }

        //CHECK THE AMOUNT TO BE ADDED
        #[allow(unused_comparisons)]
        if add_coins.coins < 0 {
            return Err(BlockChainError::NegativeAmount);
        }

        //UPDATE THE BALLANCE OF THE WALLET
        wallet.balance += add_coins.coins;
        self.update_wallet(wallet)?;

        Ok("Coins added!".to_string())
    }

    //UPDATE THE WALLET FROM THE BLOCKCHAIN OR RETURN AN ERROR IF IS NOT FOUND
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
