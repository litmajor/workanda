import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import api from '../services/api';
import Loader from '../components/Loader';
import Modal from '../components/Modal';
import './Wallet.css';

function Wallet() {
  const [activeTab, setActiveTab] = useState('overview');
  const [walletType, setWalletType] = useState('fiat'); // 'fiat' or 'blockchain'
  const [overview, setOverview] = useState(null);
  const [blockchainWallet, setBlockchainWallet] = useState(null);
  const [fiatWallets, setFiatWallets] = useState([]);
  const [selectedWallet, setSelectedWallet] = useState(null);
  const [transactions, setTransactions] = useState([]);
  const [loading, setLoading] = useState(true);
  const [showDepositModal, setShowDepositModal] = useState(false);
  const [showWithdrawModal, setShowWithdrawModal] = useState(false);
  const [showTransferModal, setShowTransferModal] = useState(false);
  const [showCreateWalletModal, setShowCreateWalletModal] = useState(false);
  const navigate = useNavigate();

  const [depositForm, setDepositForm] = useState({
    amount: '',
    payment_method: 'card',
    description: ''
  });

  const [withdrawForm, setWithdrawForm] = useState({
    amount: '',
    destination: '',
    description: ''
  });

  const [transferForm, setTransferForm] = useState({
    to_user_id: '',
    amount: '',
    description: ''
  });

  const [newWalletForm, setNewWalletForm] = useState({
    currency: 'USD',
    is_primary: false
  });

  useEffect(() => {
    fetchWalletData();
  }, [walletType]);

  const fetchWalletData = async () => {
    setLoading(true);
    try {
      if (walletType === 'fiat') {
        // Fetch traditional fiat wallet data
        const [overviewRes, walletsRes] = await Promise.all([
          api.get('/wallet/overview'),
          api.get('/wallet')
        ]);
        setOverview(overviewRes.data);
        setFiatWallets(walletsRes.data);
        if (walletsRes.data.length > 0) {
          setSelectedWallet(walletsRes.data[0]);
          fetchTransactions(walletsRes.data[0].id);
        }
      } else {
        // Fetch blockchain wallet data
        const userId = localStorage.getItem('userId'); // Assuming userId is stored after login
        const blockchainRes = await api.get(`/wallet/blockchain/user/${userId}`);
        setBlockchainWallet(blockchainRes.data);

        if (blockchainRes.data) {
          fetchBlockchainTransactions(blockchainRes.data.id);
        }
      }
    } catch (error) {
      console.error('Error fetching wallet data:', error);
      // Handle cases where user might not have a blockchain wallet yet
      if (walletType === 'blockchain' && error.response?.status === 404) {
        setBlockchainWallet(null); // Ensure blockchainWallet is null if not found
      } else {
        // Handle other errors or show a generic error message
      }
    } finally {
      setLoading(false);
    }
  };

  const fetchTransactions = async (walletId) => {
    try {
      const response = await api.get(`/wallet/${walletId}/transactions?limit=20`);
      setTransactions(response.data);
    } catch (error) {
      console.error('Failed to fetch transactions:', error);
    }
  };

  const fetchBlockchainTransactions = async (walletId) => {
    try {
      const res = await api.get(`/wallet/blockchain/${walletId}/transactions`);
      setTransactions(res.data);
    } catch (error) {
      console.error('Error fetching blockchain transactions:', error);
    }
  };


  const handleCreateWallet = async (e) => {
    e.preventDefault();
    try {
      await api.post('/wallet', newWalletForm);
      setShowCreateWalletModal(false);
      setNewWalletForm({ currency: 'USD', is_primary: false });
      fetchWalletData();
    } catch (error) {
      console.error('Failed to create wallet:', error);
      alert('Failed to create wallet');
    }
  };

  const handleDeposit = async (e) => {
    e.preventDefault();
    try {
      await api.post('/wallet/deposit', {
        wallet_id: selectedWallet.id,
        ...depositForm,
        amount: parseFloat(depositForm.amount)
      });
      setShowDepositModal(false);
      setDepositForm({ amount: '', payment_method: 'card', description: '' });
      fetchWalletData();
      if (selectedWallet) {
        fetchTransactions(selectedWallet.id);
      }
    } catch (error) {
      console.error('Failed to deposit:', error);
      alert('Failed to process deposit');
    }
  };

  const handleWithdraw = async (e) => {
    e.preventDefault();
    try {
      await api.post('/wallet/withdraw', {
        wallet_id: selectedWallet.id,
        ...withdrawForm,
        amount: parseFloat(withdrawForm.amount)
      });
      setShowWithdrawModal(false);
      setWithdrawForm({ amount: '', destination: '', description: '' });
      fetchWalletData();
      if (selectedWallet) {
        fetchTransactions(selectedWallet.id);
      }
    } catch (error) {
      console.error('Failed to withdraw:', error);
      alert('Failed to process withdrawal');
    }
  };

  const handleTransfer = async (e) => {
    e.preventDefault();
    try {
      await api.post('/wallet/transfer', {
        from_wallet_id: selectedWallet.id,
        currency: selectedWallet.currency,
        ...transferForm,
        to_user_id: parseInt(transferForm.to_user_id),
        amount: parseFloat(transferForm.amount)
      });
      setShowTransferModal(false);
      setTransferForm({ to_user_id: '', amount: '', description: '' });
      fetchWalletData();
      if (selectedWallet) {
        fetchTransactions(selectedWallet.id);
      }
    } catch (error) {
      console.error('Failed to transfer:', error);
      alert('Failed to process transfer');
    }
  };


  if (loading) return <Loader />;

  return (
    <div className="wallet-page">
      <div className="wallet-header">
        <h1>💰 My Wallet</h1>
        <div className="wallet-type-toggle">
          <button
            className={walletType === 'fiat' ? 'active' : ''}
            onClick={() => {
              setWalletType('fiat');
              setActiveTab('overview'); // Reset tab when changing wallet type
            }}
          >
            💵 Fiat Wallet
          </button>
          <button
            className={walletType === 'blockchain' ? 'active' : ''}
            onClick={() => {
              setWalletType('blockchain');
              setActiveTab('overview'); // Reset tab when changing wallet type
            }}
          >
            ⛓️ Blockchain Wallet
          </button>
        </div>
      </div>

      {walletType === 'fiat' ? (
        // FIAT WALLET VIEW
        <>
          {overview && (
            <div className="wallet-overview">
              <div className="overview-card total-balance">
                <h3>Total Balance</h3>
                <div className="amount">${overview.total_balance_usd.toFixed(2)}</div>
                <p className="subtitle">Across all fiat wallets</p>
              </div>
              <div className="overview-card pending-escrow">
                <h3>In Escrow</h3>
                <div className="amount">${overview.pending_escrows.toFixed(2)}</div>
                <p className="subtitle">Locked in contracts</p>
              </div>
            </div>
          )}

          <div className="wallet-content">
            <div className="wallet-sidebar">
              <h3>Your Wallets</h3>
              <div className="wallet-list">
                {fiatWallets.map(wallet => (
                  <div
                    key={wallet.id}
                    className={`wallet-item ${selectedWallet?.id === wallet.id ? 'active' : ''}`}
                    onClick={() => {
                      setSelectedWallet(wallet);
                      fetchTransactions(wallet.id);
                    }}
                  >
                    <div className="wallet-info">
                      <div className="wallet-currency">
                        {wallet.currency} {wallet.is_primary && <span className="badge">Primary</span>}
                      </div>
                      <div className="wallet-balance">${wallet.balance.toFixed(2)}</div>
                      <div className="wallet-details">
                        <span className="available">Available: ${wallet.available_balance.toFixed(2)}</span>
                        {wallet.locked_balance > 0 && (
                          <span className="locked">Locked: ${wallet.locked_balance.toFixed(2)}</span>
                        )}
                      </div>
                    </div>
                  </div>
                ))}
              </div>
              <button className="btn-primary" onClick={() => setShowCreateWalletModal(true)}>
                + Add Wallet
              </button>
            </div>

            <div className="wallet-main">
              {selectedWallet && (
                <>
                  <div className="wallet-actions">
                    <h3>{selectedWallet.currency} Wallet</h3>
                    <div className="action-buttons">
                      <button className="btn-success" onClick={() => setShowDepositModal(true)}>
                        Deposit
                      </button>
                      <button className="btn-warning" onClick={() => setShowWithdrawModal(true)}>
                        Withdraw
                      </button>
                      <button className="btn-primary" onClick={() => setShowTransferModal(true)}>
                        Transfer
                      </button>
                    </div>
                  </div>

                  <div className="transactions-section">
                    <h3>Transaction History</h3>
                    <div className="transactions-list">
                      {transactions.length === 0 ? (
                        <p className="no-transactions">No transactions yet</p>
                      ) : (
                        transactions.map(tx => (
                          <div key={tx.id} className={`transaction-item ${tx.transaction_type.toLowerCase()}`}>
                            <div className="transaction-icon">
                              {tx.transaction_type === 'Deposit' && '⬇️'}
                              {tx.transaction_type === 'Withdrawal' && '⬆️'}
                              {tx.transaction_type === 'Transfer' && '↔️'}
                              {tx.transaction_type === 'EscrowLock' && '🔒'}
                              {tx.transaction_type === 'EscrowRelease' && '🔓'}
                              {tx.transaction_type === 'Fee' && '💳'}
                            </div>
                            <div className="transaction-details">
                              <div className="transaction-type">{tx.transaction_type}</div>
                              <div className="transaction-description">{tx.description}</div>
                              <div className="transaction-date">
                                {new Date(tx.created_at).toLocaleString()}
                              </div>
                            </div>
                            <div className="transaction-amount">
                              <span className={tx.amount >= 0 ? 'positive' : 'negative'}>
                                {tx.amount >= 0 ? '+' : ''}{tx.currency} {Math.abs(tx.amount).toFixed(2)}
                              </span>
                              <div className="balance-after">
                                Balance: {tx.currency} {tx.balance_after.toFixed(2)}
                              </div>
                            </div>
                            <div className={`transaction-status ${tx.status.toLowerCase()}`}>
                              {tx.status}
                            </div>
                          </div>
                        ))
                      )}
                    </div>
                  </div>
                </>
              )}
            </div>
          </div>
        </>
      ) : (
        // BLOCKCHAIN WALLET VIEW
        <div className="blockchain-wallet-container">
          {blockchainWallet ? (
            <>
              <div className="wallet-overview">
                <div className="overview-card">
                  <h3>Wallet Address</h3>
                  <div className="address-display">
                    <code>{blockchainWallet.celo_address}</code>
                    <button className="btn-icon" onClick={() => navigator.clipboard.writeText(blockchainWallet.celo_address)}>
                      📋 Copy
                    </button>
                  </div>
                </div>
                <div className="overview-card">
                  <h3>Total Portfolio Value</h3>
                  <div className="amount">$0.00</div>
                  <p className="subtitle">Across all crypto assets</p>
                </div>
              </div>

              <div className="crypto-actions">
                <button className="btn btn-primary" onClick={() => navigate('/wallet/blockchain/send')}>
                  Send Crypto
                </button>
                <button className="btn btn-success" onClick={() => navigate('/wallet/blockchain/receive')}>
                  Receive
                </button>
                <button className="btn btn-warning" onClick={() => navigate('/wallet/blockchain/swap')}>
                  Swap
                </button>
                <button className="btn btn-info" onClick={() => navigate('/wallet/blockchain/buy')}>
                  Buy Crypto
                </button>
              </div>

              <div className="crypto-balances">
                <h3>Crypto Assets</h3>
                <div className="balance-grid">
                  {['cUSD', 'cEUR', 'cREAL', 'CELO', 'BTC', 'ETH', 'USDT', 'USDC'].map(currency => (
                    <div key={currency} className="crypto-balance-card">
                      <div className="crypto-name">{currency}</div>
                      <div className="crypto-balance">0.00</div>
                      <div className="crypto-value">$0.00</div>
                    </div>
                  ))}
                </div>
              </div>

              <div className="transactions-section">
                <h3>Blockchain Transactions</h3>
                <div className="transactions-list">
                  {transactions.length === 0 ? (
                    <div className="no-transactions">No blockchain transactions yet</div>
                  ) : (
                    transactions.map(tx => (
                      <div key={tx.id} className="transaction-item">
                        <div className="tx-type">{tx.tx_type}</div>
                        <div className="tx-amount">{tx.amount} {tx.currency_code}</div>
                        <div className="tx-hash">
                          <a href={`https://explorer.celo.org/tx/${tx.tx_hash}`} target="_blank" rel="noopener noreferrer">
                            {tx.tx_hash?.substring(0, 10)}...
                          </a>
                        </div>
                        <div className={`tx-status ${tx.status.toLowerCase()}`}>{tx.status}</div>
                      </div>
                    ))
                  )}
                </div>
              </div>
            </>
          ) : (
            <div className="no-wallet">
              <h3>No Blockchain Wallet Found</h3>
              <p>Create a blockchain wallet to start using crypto features</p>
              <button className="btn btn-primary" onClick={() => navigate('/wallet/blockchain/create')}>
                Create Blockchain Wallet
              </button>
            </div>
          )}
        </div>
      )}

      {/* Create Wallet Modal */}
      <Modal show={showCreateWalletModal} onClose={() => setShowCreateWalletModal(false)}>
        <h2>Create New Wallet</h2>
        <form onSubmit={handleCreateWallet}>
          <div className="form-group">
            <label>Currency</label>
            <select
              value={newWalletForm.currency}
              onChange={(e) => setNewWalletForm({...newWalletForm, currency: e.target.value})}
              required
            >
              <option value="USD">USD</option>
              <option value="EUR">EUR</option>
              <option value="GBP">GBP</option>
              <option value="BTC">BTC</option>
              <option value="ETH">ETH</option>
            </select>
          </div>
          <div className="form-group">
            <label>
              <input
                type="checkbox"
                checked={newWalletForm.is_primary}
                onChange={(e) => setNewWalletForm({...newWalletForm, is_primary: e.target.checked})}
              />
              Set as primary wallet
            </label>
          </div>
          <div className="modal-actions">
            <button type="button" className="btn-secondary" onClick={() => setShowCreateWalletModal(false)}>
              Cancel
            </button>
            <button type="submit" className="btn-primary">Create Wallet</button>
          </div>
        </form>
      </Modal>

      {/* Deposit Modal */}
      <Modal show={showDepositModal} onClose={() => setShowDepositModal(false)}>
        <h2>Deposit Funds</h2>
        <form onSubmit={handleDeposit}>
          <div className="form-group">
            <label>Amount</label>
            <input
              type="number"
              step="0.01"
              min="0.01"
              value={depositForm.amount}
              onChange={(e) => setDepositForm({...depositForm, amount: e.target.value})}
              required
            />
          </div>
          <div className="form-group">
            <label>Payment Method</label>
            <select
              value={depositForm.payment_method}
              onChange={(e) => setDepositForm({...depositForm, payment_method: e.target.value})}
            >
              <option value="card">Credit/Debit Card</option>
              <option value="bank">Bank Transfer</option>
              <option value="paypal">PayPal</option>
            </select>
          </div>
          <div className="form-group">
            <label>Description (Optional)</label>
            <input
              type="text"
              value={depositForm.description}
              onChange={(e) => setDepositForm({...depositForm, description: e.target.value})}
            />
          </div>
          <div className="modal-actions">
            <button type="button" className="btn-secondary" onClick={() => setShowDepositModal(false)}>
              Cancel
            </button>
            <button type="submit" className="btn-success">Deposit</button>
          </div>
        </form>
      </Modal>

      {/* Withdraw Modal */}
      <Modal show={showWithdrawModal} onClose={() => setShowWithdrawModal(false)}>
        <h2>Withdraw Funds</h2>
        <form onSubmit={handleWithdraw}>
          <div className="form-group">
            <label>Amount (Available: ${selectedWallet?.available_balance.toFixed(2)})</label>
            <input
              type="number"
              step="0.01"
              min="0.01"
              max={selectedWallet?.available_balance}
              value={withdrawForm.amount}
              onChange={(e) => setWithdrawForm({...withdrawForm, amount: e.target.value})}
              required
            />
          </div>
          <div className="form-group">
            <label>Destination (Bank Account / Crypto Address)</label>
            <input
              type="text"
              value={withdrawForm.destination}
              onChange={(e) => setWithdrawForm({...withdrawForm, destination: e.target.value})}
              required
            />
          </div>
          <div className="form-group">
            <label>Description (Optional)</label>
            <input
              type="text"
              value={withdrawForm.description}
              onChange={(e) => setWithdrawForm({...withdrawForm, description: e.target.value})}
            />
          </div>
          <div className="modal-actions">
            <button type="button" className="btn-secondary" onClick={() => setShowWithdrawModal(false)}>
              Cancel
            </button>
            <button type="submit" className="btn-warning">Withdraw</button>
          </div>
        </form>
      </Modal>

      {/* Transfer Modal */}
      <Modal show={showTransferModal} onClose={() => setShowTransferModal(false)}>
        <h2>Transfer Funds</h2>
        <form onSubmit={handleTransfer}>
          <div className="form-group">
            <label>Recipient User ID</label>
            <input
              type="number"
              value={transferForm.to_user_id}
              onChange={(e) => setTransferForm({...transferForm, to_user_id: e.target.value})}
              required
            />
          </div>
          <div className="form-group">
            <label>Amount (Available: ${selectedWallet?.available_balance.toFixed(2)})</label>
            <input
              type="number"
              step="0.01"
              min="0.01"
              max={selectedWallet?.available_balance}
              value={transferForm.amount}
              onChange={(e) => setTransferForm({...transferForm, amount: e.target.value})}
              required
            />
          </div>
          <div className="form-group">
            <label>Description (Optional)</label>
            <input
              type="text"
              value={transferForm.description}
              onChange={(e) => setTransferForm({...transferForm, description: e.target.value})}
            />
          </div>
          <div className="modal-actions">
            <button type="button" className="btn-secondary" onClick={() => setShowTransferModal(false)}>
              Cancel
            </button>
            <button type="submit" className="btn-primary">Transfer</button>
          </div>
        </form>
      </Modal>
    </div>
  );
}

export default Wallet;