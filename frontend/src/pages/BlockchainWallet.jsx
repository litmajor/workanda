
import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import api from '../services/api';
import Loader from '../components/Loader';
import Modal from '../components/Modal';
import Alert from '../components/Alert';
import Chart from '../components/Chart';
import './BlockchainWallet.css';

function BlockchainWallet() {
  const [wallet, setWallet] = useState(null);
  const [balances, setBalances] = useState([]);
  const [transactions, setTransactions] = useState([]);
  const [portfolio, setPortfolio] = useState(null);
  const [loading, setLoading] = useState(true);
  const [alert, setAlert] = useState(null);
  const navigate = useNavigate();

  // Modals
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showRecoverModal, setShowRecoverModal] = useState(false);
  const [showSendModal, setShowSendModal] = useState(false);
  const [showReceiveModal, setShowReceiveModal] = useState(false);
  const [showSwapModal, setShowSwapModal] = useState(false);
  const [showSeedPhraseModal, setShowSeedPhraseModal] = useState(false);

  // Forms
  const [sendForm, setSendForm] = useState({
    to_address: '',
    amount: '',
    currency_code: 'cUSD',
  });

  const [swapForm, setSwapForm] = useState({
    from_currency: 'cUSD',
    to_currency: 'cEUR',
    amount: '',
  });

  const [recoverForm, setRecoverForm] = useState({
    seed_phrase: '',
    password: '',
  });

  const [seedPhrase, setSeedPhrase] = useState(null);
  const [qrCode, setQrCode] = useState(null);

  useEffect(() => {
    checkWallet();
  }, []);

  const checkWallet = async () => {
    try {
      setLoading(true);
      const userId = localStorage.getItem('userId');
      const response = await api.get(`/wallet/blockchain/user/${userId}`);
      setWallet(response.data);
      await fetchWalletData(response.data.id);
    } catch (error) {
      if (error.response?.status === 404) {
        // No wallet exists, show create option
        setLoading(false);
      } else {
        console.error('Failed to check wallet:', error);
        setAlert({ type: 'error', message: 'Failed to load wallet' });
        setLoading(false);
      }
    }
  };

  const fetchWalletData = async (walletId) => {
    try {
      const [balancesRes, transactionsRes, portfolioRes] = await Promise.all([
        api.get(`/wallet/blockchain/${walletId}/balances`),
        api.get(`/wallet/blockchain/${walletId}/transactions?limit=50`),
        api.get(`/wallet/blockchain/${walletId}/portfolio`),
      ]);
      
      setBalances(balancesRes.data);
      setTransactions(transactionsRes.data);
      setPortfolio(portfolioRes.data);
      setLoading(false);
    } catch (error) {
      console.error('Failed to fetch wallet data:', error);
      setAlert({ type: 'error', message: 'Failed to load wallet data' });
      setLoading(false);
    }
  };

  const handleCreateWallet = async () => {
    try {
      setLoading(true);
      const userId = localStorage.getItem('userId');
      const response = await api.post('/wallet/blockchain/create', {
        user_id: parseInt(userId),
        wallet_type: 'Individual',
      });
      
      setSeedPhrase(response.data.seed_phrase);
      setWallet(response.data.wallet);
      setShowSeedPhraseModal(true);
      setShowCreateModal(false);
      setAlert({ type: 'success', message: 'Wallet created successfully!' });
      await fetchWalletData(response.data.wallet.id);
    } catch (error) {
      console.error('Failed to create wallet:', error);
      setAlert({ type: 'error', message: 'Failed to create wallet' });
      setLoading(false);
    }
  };

  const handleRecoverWallet = async (e) => {
    e.preventDefault();
    try {
      setLoading(true);
      const userId = localStorage.getItem('userId');
      const response = await api.post('/wallet/blockchain/recover', {
        user_id: parseInt(userId),
        seed_phrase: recoverForm.seed_phrase,
        password: recoverForm.password,
      });
      
      setWallet(response.data);
      setShowRecoverModal(false);
      setAlert({ type: 'success', message: 'Wallet recovered successfully!' });
      await fetchWalletData(response.data.id);
    } catch (error) {
      console.error('Failed to recover wallet:', error);
      setAlert({ type: 'error', message: 'Failed to recover wallet' });
      setLoading(false);
    }
  };

  const handleSendMoney = async (e) => {
    e.preventDefault();
    try {
      setLoading(true);
      const response = await api.post('/wallet/blockchain/transaction/create', {
        from_wallet_id: wallet.id,
        ...sendForm,
        amount: parseFloat(sendForm.amount),
      });
      
      setShowSendModal(false);
      setSendForm({ to_address: '', amount: '', currency_code: 'cUSD' });
      setAlert({ type: 'success', message: 'Transaction initiated!' });
      await fetchWalletData(wallet.id);
    } catch (error) {
      console.error('Failed to send money:', error);
      setAlert({ type: 'error', message: error.response?.data?.message || 'Failed to send money' });
      setLoading(false);
    }
  };

  const handleSwap = async (e) => {
    e.preventDefault();
    try {
      setLoading(true);
      const response = await api.post('/wallet/blockchain/swap', {
        wallet_id: wallet.id,
        ...swapForm,
        amount: parseFloat(swapForm.amount),
      });
      
      setShowSwapModal(false);
      setSwapForm({ from_currency: 'cUSD', to_currency: 'cEUR', amount: '' });
      setAlert({ type: 'success', message: 'Swap successful!' });
      await fetchWalletData(wallet.id);
    } catch (error) {
      console.error('Failed to swap:', error);
      setAlert({ type: 'error', message: 'Failed to swap currencies' });
      setLoading(false);
    }
  };

  const handleGenerateQR = async () => {
    try {
      const response = await api.post('/wallet/blockchain/qr/generate', {
        wallet_id: wallet.id,
        currency: 'cUSD',
        amount: null, // Optional
      });
      setQrCode(response.data.qr_code);
      setShowReceiveModal(true);
    } catch (error) {
      console.error('Failed to generate QR:', error);
      setAlert({ type: 'error', message: 'Failed to generate QR code' });
    }
  };

  const copyAddress = () => {
    navigator.clipboard.writeText(wallet.celo_address);
    setAlert({ type: 'success', message: 'Address copied to clipboard!' });
  };

  if (loading) return <Loader />;

  if (!wallet) {
    return (
      <div className="blockchain-wallet-page">
        <div className="wallet-empty">
          <h1>🔐 Blockchain Wallet</h1>
          <p>Create or recover your blockchain wallet to access crypto features</p>
          <div className="empty-actions">
            <button className="btn-primary" onClick={() => setShowCreateModal(true)}>
              Create New Wallet
            </button>
            <button className="btn-secondary" onClick={() => setShowRecoverModal(true)}>
              Recover Wallet
            </button>
          </div>
        </div>

        {/* Create Wallet Modal */}
        <Modal show={showCreateModal} onClose={() => setShowCreateModal(false)}>
          <h2>Create Blockchain Wallet</h2>
          <p>This will create a new Celo-compatible wallet with a seed phrase. Keep your seed phrase safe!</p>
          <div className="modal-actions">
            <button className="btn-secondary" onClick={() => setShowCreateModal(false)}>Cancel</button>
            <button className="btn-primary" onClick={handleCreateWallet}>Create Wallet</button>
          </div>
        </Modal>

        {/* Recover Wallet Modal */}
        <Modal show={showRecoverModal} onClose={() => setShowRecoverModal(false)}>
          <h2>Recover Wallet</h2>
          <form onSubmit={handleRecoverWallet}>
            <div className="form-group">
              <label>Seed Phrase (12 or 24 words)</label>
              <textarea
                value={recoverForm.seed_phrase}
                onChange={(e) => setRecoverForm({ ...recoverForm, seed_phrase: e.target.value })}
                rows="4"
                required
                placeholder="word1 word2 word3 ..."
              />
            </div>
            <div className="form-group">
              <label>Password</label>
              <input
                type="password"
                value={recoverForm.password}
                onChange={(e) => setRecoverForm({ ...recoverForm, password: e.target.value })}
                required
              />
            </div>
            <div className="modal-actions">
              <button type="button" className="btn-secondary" onClick={() => setShowRecoverModal(false)}>
                Cancel
              </button>
              <button type="submit" className="btn-primary">Recover</button>
            </div>
          </form>
        </Modal>
      </div>
    );
  }

  return (
    <div className="blockchain-wallet-page">
      {alert && <Alert type={alert.type} message={alert.message} onClose={() => setAlert(null)} />}

      <div className="wallet-header">
        <div>
          <h1>🔐 Blockchain Wallet</h1>
          <div className="wallet-address">
            <span>{wallet.celo_address}</span>
            <button className="btn-icon" onClick={copyAddress} title="Copy address">📋</button>
          </div>
        </div>
        <div className="header-actions">
          <button className="btn-primary" onClick={() => setShowSendModal(true)}>Send</button>
          <button className="btn-success" onClick={handleGenerateQR}>Receive</button>
          <button className="btn-warning" onClick={() => setShowSwapModal(true)}>Swap</button>
        </div>
      </div>

      {portfolio && (
        <div className="portfolio-overview">
          <div className="portfolio-card">
            <h3>Total Portfolio Value</h3>
            <div className="amount">${portfolio.total_value_usd?.toFixed(2) || '0.00'}</div>
          </div>
          <div className="portfolio-chart">
            <h3>Portfolio Distribution</h3>
            <Chart
              data={balances.map(b => ({ name: b.currency_code, value: parseFloat(b.balance) }))}
              type="pie"
            />
          </div>
        </div>
      )}

      <div className="wallet-content">
        <div className="balances-section">
          <h3>Balances</h3>
          <div className="balances-grid">
            {balances.map(balance => (
              <div key={balance.id} className="balance-card">
                <div className="balance-header">
                  <span className="currency-icon">
                    {balance.currency_code === 'CELO' && '🪙'}
                    {balance.currency_code === 'cUSD' && '💵'}
                    {balance.currency_code === 'cEUR' && '💶'}
                    {balance.currency_code === 'cREAL' && '💷'}
                  </span>
                  <span className="currency-name">{balance.currency_code}</span>
                </div>
                <div className="balance-amount">{parseFloat(balance.balance).toFixed(4)}</div>
                {parseFloat(balance.locked_balance) > 0 && (
                  <div className="balance-locked">
                    Locked: {parseFloat(balance.locked_balance).toFixed(4)}
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>

        <div className="transactions-section">
          <h3>Recent Transactions</h3>
          <div className="transactions-list">
            {transactions.length === 0 ? (
              <p className="no-transactions">No transactions yet</p>
            ) : (
              transactions.map(tx => (
                <div key={tx.id} className={`transaction-item ${tx.tx_type.toLowerCase()}`}>
                  <div className="tx-icon">
                    {tx.tx_type === 'Payment' && '💸'}
                    {tx.tx_type === 'Deposit' && '⬇️'}
                    {tx.tx_type === 'Withdrawal' && '⬆️'}
                    {tx.tx_type === 'Swap' && '🔄'}
                  </div>
                  <div className="tx-details">
                    <div className="tx-type">{tx.tx_type}</div>
                    {tx.to_address && (
                      <div className="tx-address">To: {tx.to_address.slice(0, 10)}...{tx.to_address.slice(-8)}</div>
                    )}
                    {tx.tx_hash && (
                      <div className="tx-hash">
                        <a href={`https://explorer.celo.org/tx/${tx.tx_hash}`} target="_blank" rel="noopener noreferrer">
                          View on Explorer
                        </a>
                      </div>
                    )}
                    <div className="tx-date">{new Date(tx.created_at).toLocaleString()}</div>
                  </div>
                  <div className="tx-amount">
                    {tx.amount} {tx.currency_code}
                    {tx.gas_fee && <div className="tx-fee">Gas: {tx.gas_fee}</div>}
                  </div>
                  <div className={`tx-status ${tx.status.toLowerCase()}`}>{tx.status}</div>
                </div>
              ))
            )}
          </div>
        </div>
      </div>

      {/* Send Modal */}
      <Modal show={showSendModal} onClose={() => setShowSendModal(false)}>
        <h2>Send Crypto</h2>
        <form onSubmit={handleSendMoney}>
          <div className="form-group">
            <label>Recipient Address or Username</label>
            <input
              type="text"
              value={sendForm.to_address}
              onChange={(e) => setSendForm({ ...sendForm, to_address: e.target.value })}
              required
              placeholder="0x... or @username"
            />
          </div>
          <div className="form-group">
            <label>Currency</label>
            <select
              value={sendForm.currency_code}
              onChange={(e) => setSendForm({ ...sendForm, currency_code: e.target.value })}
            >
              {balances.map(b => (
                <option key={b.currency_code} value={b.currency_code}>{b.currency_code}</option>
              ))}
            </select>
          </div>
          <div className="form-group">
            <label>Amount</label>
            <input
              type="number"
              step="0.0001"
              value={sendForm.amount}
              onChange={(e) => setSendForm({ ...sendForm, amount: e.target.value })}
              required
            />
          </div>
          <div className="modal-actions">
            <button type="button" className="btn-secondary" onClick={() => setShowSendModal(false)}>Cancel</button>
            <button type="submit" className="btn-primary">Send</button>
          </div>
        </form>
      </Modal>

      {/* Receive Modal */}
      <Modal show={showReceiveModal} onClose={() => setShowReceiveModal(false)}>
        <h2>Receive Crypto</h2>
        <div className="receive-content">
          <p>Share this QR code or address to receive payments:</p>
          {qrCode && <img src={qrCode} alt="Payment QR Code" className="qr-code" />}
          <div className="address-box">
            <code>{wallet.celo_address}</code>
            <button className="btn-icon" onClick={copyAddress}>📋</button>
          </div>
        </div>
        <div className="modal-actions">
          <button className="btn-primary" onClick={() => setShowReceiveModal(false)}>Close</button>
        </div>
      </Modal>

      {/* Swap Modal */}
      <Modal show={showSwapModal} onClose={() => setShowSwapModal(false)}>
        <h2>Swap Currencies</h2>
        <form onSubmit={handleSwap}>
          <div className="form-group">
            <label>From</label>
            <select
              value={swapForm.from_currency}
              onChange={(e) => setSwapForm({ ...swapForm, from_currency: e.target.value })}
            >
              {balances.map(b => (
                <option key={b.currency_code} value={b.currency_code}>{b.currency_code}</option>
              ))}
            </select>
          </div>
          <div className="form-group">
            <label>To</label>
            <select
              value={swapForm.to_currency}
              onChange={(e) => setSwapForm({ ...swapForm, to_currency: e.target.value })}
            >
              {balances.map(b => (
                <option key={b.currency_code} value={b.currency_code}>{b.currency_code}</option>
              ))}
            </select>
          </div>
          <div className="form-group">
            <label>Amount</label>
            <input
              type="number"
              step="0.0001"
              value={swapForm.amount}
              onChange={(e) => setSwapForm({ ...swapForm, amount: e.target.value })}
              required
            />
          </div>
          <div className="modal-actions">
            <button type="button" className="btn-secondary" onClick={() => setShowSwapModal(false)}>Cancel</button>
            <button type="submit" className="btn-warning">Swap</button>
          </div>
        </form>
      </Modal>

      {/* Seed Phrase Modal */}
      <Modal show={showSeedPhraseModal} onClose={() => setShowSeedPhraseModal(false)}>
        <h2>⚠️ Save Your Seed Phrase</h2>
        <div className="seed-phrase-warning">
          <p><strong>IMPORTANT:</strong> Write down these words and keep them safe. This is the ONLY way to recover your wallet!</p>
          <div className="seed-phrase-box">
            <code>{seedPhrase}</code>
          </div>
          <p className="warning-text">Never share your seed phrase with anyone. Workanda will never ask for it.</p>
        </div>
        <div className="modal-actions">
          <button className="btn-primary" onClick={() => setShowSeedPhraseModal(false)}>I've Saved It</button>
        </div>
      </Modal>
    </div>
  );
}

export default BlockchainWallet;
