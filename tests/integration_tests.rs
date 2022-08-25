use blockchainApi::{
    self,
    blockchain::{
        block::Block,
        block_chain::{BlockChain, BLOCKCHAIN},
        transaction::TransactionInfo,
        wallet::WalletInfo,
    },
};

#[test]
fn test_global_blockchain() {
    //GET GLOBAL BLOCKCHAIN
    let global_blockchain = BLOCKCHAIN.lock().unwrap();

    //CREATE A NEW BLOCKCHAIN WITH DEFAULT VALUES
    let genesis_block = Block::new(0, &vec![]);
    let blockchain = BlockChain {
        chain: vec![genesis_block],
        difficulty: 2,
        pending_transactions: vec![],
        mining_reward: 100,
        name: "Blockchain".to_string(),
        wallets: vec![],
    };

    //CHECK IF GLOBAL BLOCKCHAIN AND NEW CREATED BLOCKCHAIN ARE THE SAME
    assert_eq!(global_blockchain.chain.len(), blockchain.chain.len());
    assert_eq!(global_blockchain.difficulty, blockchain.difficulty);
    assert_eq!(
        global_blockchain.pending_transactions.len(),
        blockchain.pending_transactions.len()
    );
    assert_eq!(global_blockchain.mining_reward, blockchain.mining_reward);
    assert_eq!(global_blockchain.name, blockchain.name);
    assert_eq!(global_blockchain.wallets.len(), blockchain.wallets.len());
}

#[test]
fn test_blockchain() {
    //GET BLOCKCHAIN
    let mut blockchain = BLOCKCHAIN.lock().unwrap();

    //CREATE 'FROM' WALLET INFO
    let from_wallet = WalletInfo {
        address: "from_address".to_string(),
        balance: 100,
        password: "pass".to_string(),
    };

    //CREATE 'TO' WALLET INFO
    let to_wallet = WalletInfo {
        address: "to_address".to_string(),
        balance: 0,
        password: "pass".to_string(),
    };

    //CREATE WALLETS
    let create_from_wallet_resp = blockchain.create_wallet(from_wallet).unwrap();
    let create_to_wallet_resp = blockchain.create_wallet(to_wallet).unwrap();

    //CHECK RESPONSES FROM CREATING WALLETS
    assert_eq!(create_from_wallet_resp, "Wallet created!".to_string());
    assert_eq!(create_to_wallet_resp, "Wallet created!".to_string());

    //CREATE TRANSACTION INFO
    let transaction = TransactionInfo {
        from_address: "from_address".to_string(),
        from_password: "pass".to_string(),
        to_address: "to_address".to_string(),
        amount: 50,
    };

    //CREATE TRANSACTION
    let transaction_resp = blockchain.create_transaction(transaction.clone()).unwrap();

    //CHECK RESPONSE FROM CREATING TRANSACTION
    assert_eq!(
        transaction_resp,
        "Transaction successfully made".to_string()
    );

    //CHECK PENDING TRANSACTIONS LENGTH
    assert_eq!(blockchain.pending_transactions.len(), 1);

    //CREATE NEW TRANSACTION
    blockchain.create_transaction(transaction).unwrap();

    //CHECK PENDING TRANSACTIONS LENGTH
    assert_eq!(blockchain.pending_transactions.len(), 2);

    //MINE PENDING TRANSACTIONS
    let mine_pending_transactions_resp = blockchain
        .mine_pending_transactions(&"to_address".to_string())
        .unwrap();

    //CHECK MINE PENDING TRANSACTIONS RESPONSE
    assert_eq!(
        mine_pending_transactions_resp,
        "Transactions successfully mined".to_string()
    );

    //CHECK PENDING TRANSACTIONS LENGTH
    assert_eq!(blockchain.pending_transactions.len(), 1);

    //GET 'FROM' WALLET BALLANCE
    let from_wallet_balance = blockchain
        .get_balance_of_wallet(&"from_address".to_string(), &"pass".to_string())
        .unwrap();

    //CHECK 'FROM' WALLET BALLANCE
    assert_eq!(from_wallet_balance, 0);

    //GET 'TO' WALLET BALLANCE
    let to_wallet_balance = blockchain
        .get_balance_of_wallet(&"to_address".to_string(), &"pass".to_string())
        .unwrap();

    //CHECK 'TO' WALLET BALLANCE
    assert_eq!(to_wallet_balance, 100);
    //CHECK BLOCKCHAIN LENGTH
    assert_eq!(blockchain.chain.len(), 2);
    //CHECK NR OF TRANSACTIONS FROM THE SECOND BLOCK
    assert_eq!(blockchain.chain[1].transactions.len(), 2);

    //MINE PENDING TRANSACTIONS
    let mine_pending_transactions_resp = blockchain
        .mine_pending_transactions(&"from_address".to_string())
        .unwrap();

    //CHECK MINE PENDING TRANSACTIONS RESPONSE
    assert_eq!(
        mine_pending_transactions_resp,
        "Transactions successfully mined".to_string()
    );

    //GET 'TO' WALLET BALLANCE
    let to_wallet_balance = blockchain
        .get_balance_of_wallet(&"to_address".to_string(), &"pass".to_string())
        .unwrap();

    //CHECK 'TO' WALLET BALLANCE
    assert_eq!(to_wallet_balance, 200);
    //CHECK BLOCKCHAIN LENGTH
    assert_eq!(blockchain.chain.len(), 3);
    //CHECK NR OF TRANSACTIONS FROM THE THIRD BLOCK
    assert_eq!(blockchain.chain[2].transactions.len(), 1);
}
