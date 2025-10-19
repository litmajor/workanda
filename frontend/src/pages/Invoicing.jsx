
import { useState, useEffect } from 'react';
import { usePaymentHistory } from '../hooks/usePayments';
import './Invoicing.css';

function Invoicing() {
  const { payments, loading } = usePaymentHistory();
  const [invoices, setInvoices] = useState([]);
  const [selectedInvoice, setSelectedInvoice] = useState(null);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [formData, setFormData] = useState({
    clientName: '',
    clientEmail: '',
    items: [{ description: '', quantity: 1, rate: 0 }],
    dueDate: '',
    notes: ''
  });

  useEffect(() => {
    // Generate invoices from payment history
    if (payments && payments.length > 0) {
      const generatedInvoices = payments.map((payment, index) => ({
        id: payment.id || index + 1,
        invoiceNumber: `INV-${String(index + 1).padStart(5, '0')}`,
        clientName: payment.client_name || 'Client',
        amount: payment.amount || 0,
        status: payment.status || 'pending',
        issuedDate: payment.created_at || new Date().toISOString(),
        dueDate: payment.due_date || new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString(),
        items: payment.items || []
      }));
      setInvoices(generatedInvoices);
    }
  }, [payments]);

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
  };

  const handleItemChange = (index, field, value) => {
    const newItems = [...formData.items];
    newItems[index][field] = value;
    setFormData(prev => ({ ...prev, items: newItems }));
  };

  const addItem = () => {
    setFormData(prev => ({
      ...prev,
      items: [...prev.items, { description: '', quantity: 1, rate: 0 }]
    }));
  };

  const removeItem = (index) => {
    setFormData(prev => ({
      ...prev,
      items: prev.items.filter((_, i) => i !== index)
    }));
  };

  const calculateTotal = () => {
    return formData.items.reduce((sum, item) => sum + (item.quantity * item.rate), 0);
  };

  const handleCreateInvoice = async (e) => {
    e.preventDefault();
    const newInvoice = {
      id: invoices.length + 1,
      invoiceNumber: `INV-${String(invoices.length + 1).padStart(5, '0')}`,
      clientName: formData.clientName,
      clientEmail: formData.clientEmail,
      amount: calculateTotal(),
      status: 'pending',
      issuedDate: new Date().toISOString(),
      dueDate: formData.dueDate,
      items: formData.items,
      notes: formData.notes
    };

    setInvoices([...invoices, newInvoice]);
    setShowCreateModal(false);
    setFormData({
      clientName: '',
      clientEmail: '',
      items: [{ description: '', quantity: 1, rate: 0 }],
      dueDate: '',
      notes: ''
    });
  };

  const downloadInvoice = (invoice) => {
    const invoiceHtml = generateInvoiceHtml(invoice);
    const blob = new Blob([invoiceHtml], { type: 'text/html' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${invoice.invoiceNumber}.html`;
    a.click();
  };

  const generateInvoiceHtml = (invoice) => {
    return `
<!DOCTYPE html>
<html>
<head>
  <title>Invoice ${invoice.invoiceNumber}</title>
  <style>
    body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }
    .header { display: flex; justify-content: space-between; margin-bottom: 30px; }
    .invoice-details { margin-bottom: 20px; }
    table { width: 100%; border-collapse: collapse; }
    th, td { padding: 10px; text-align: left; border-bottom: 1px solid #ddd; }
    .total { font-weight: bold; font-size: 1.2em; }
  </style>
</head>
<body>
  <div class="header">
    <div>
      <h1>INVOICE</h1>
      <p><strong>Invoice #:</strong> ${invoice.invoiceNumber}</p>
    </div>
    <div>
      <p><strong>Date:</strong> ${new Date(invoice.issuedDate).toLocaleDateString()}</p>
      <p><strong>Due Date:</strong> ${new Date(invoice.dueDate).toLocaleDateString()}</p>
    </div>
  </div>
  <div class="invoice-details">
    <p><strong>Bill To:</strong></p>
    <p>${invoice.clientName}</p>
    ${invoice.clientEmail ? `<p>${invoice.clientEmail}</p>` : ''}
  </div>
  <table>
    <thead>
      <tr>
        <th>Description</th>
        <th>Quantity</th>
        <th>Rate</th>
        <th>Amount</th>
      </tr>
    </thead>
    <tbody>
      ${invoice.items?.map(item => `
        <tr>
          <td>${item.description}</td>
          <td>${item.quantity}</td>
          <td>$${item.rate.toFixed(2)}</td>
          <td>$${(item.quantity * item.rate).toFixed(2)}</td>
        </tr>
      `).join('') || ''}
    </tbody>
    <tfoot>
      <tr>
        <td colspan="3" class="total">Total</td>
        <td class="total">$${invoice.amount.toFixed(2)}</td>
      </tr>
    </tfoot>
  </table>
  ${invoice.notes ? `<p><strong>Notes:</strong> ${invoice.notes}</p>` : ''}
</body>
</html>
    `;
  };

  return (
    <div className="invoicing-page">
      <div className="page-header">
        <h1>📄 Invoice Management</h1>
        <button className="btn btn-primary" onClick={() => setShowCreateModal(true)}>
          + Create Invoice
        </button>
      </div>

      <div className="invoices-grid">
        {loading ? (
          <p>Loading invoices...</p>
        ) : invoices.length === 0 ? (
          <div className="empty-state">
            <p>No invoices yet. Create your first invoice!</p>
          </div>
        ) : (
          invoices.map(invoice => (
            <div key={invoice.id} className="invoice-card">
              <div className="invoice-header">
                <h3>{invoice.invoiceNumber}</h3>
                <span className={`status-badge ${invoice.status}`}>
                  {invoice.status}
                </span>
              </div>
              <div className="invoice-details">
                <p><strong>Client:</strong> {invoice.clientName}</p>
                <p><strong>Amount:</strong> ${invoice.amount?.toFixed(2) || '0.00'}</p>
                <p><strong>Issued:</strong> {new Date(invoice.issuedDate).toLocaleDateString()}</p>
                <p><strong>Due:</strong> {new Date(invoice.dueDate).toLocaleDateString()}</p>
              </div>
              <div className="invoice-actions">
                <button 
                  className="btn btn-secondary"
                  onClick={() => setSelectedInvoice(invoice)}
                >
                  View
                </button>
                <button 
                  className="btn btn-primary"
                  onClick={() => downloadInvoice(invoice)}
                >
                  Download
                </button>
              </div>
            </div>
          ))
        )}
      </div>

      {showCreateModal && (
        <div className="modal-overlay" onClick={() => setShowCreateModal(false)}>
          <div className="modal-content" onClick={e => e.stopPropagation()}>
            <h2>Create New Invoice</h2>
            <form onSubmit={handleCreateInvoice}>
              <div className="form-group">
                <label>Client Name *</label>
                <input
                  type="text"
                  name="clientName"
                  value={formData.clientName}
                  onChange={handleInputChange}
                  required
                />
              </div>
              <div className="form-group">
                <label>Client Email</label>
                <input
                  type="email"
                  name="clientEmail"
                  value={formData.clientEmail}
                  onChange={handleInputChange}
                />
              </div>
              <div className="form-group">
                <label>Due Date *</label>
                <input
                  type="date"
                  name="dueDate"
                  value={formData.dueDate}
                  onChange={handleInputChange}
                  required
                />
              </div>
              
              <div className="items-section">
                <h3>Items</h3>
                {formData.items.map((item, index) => (
                  <div key={index} className="item-row">
                    <input
                      type="text"
                      placeholder="Description"
                      value={item.description}
                      onChange={(e) => handleItemChange(index, 'description', e.target.value)}
                      required
                    />
                    <input
                      type="number"
                      placeholder="Qty"
                      value={item.quantity}
                      onChange={(e) => handleItemChange(index, 'quantity', parseFloat(e.target.value))}
                      min="1"
                      required
                    />
                    <input
                      type="number"
                      placeholder="Rate"
                      value={item.rate}
                      onChange={(e) => handleItemChange(index, 'rate', parseFloat(e.target.value))}
                      min="0"
                      step="0.01"
                      required
                    />
                    {formData.items.length > 1 && (
                      <button type="button" onClick={() => removeItem(index)}>Remove</button>
                    )}
                  </div>
                ))}
                <button type="button" onClick={addItem} className="btn btn-secondary">
                  + Add Item
                </button>
              </div>

              <div className="form-group">
                <label>Notes</label>
                <textarea
                  name="notes"
                  value={formData.notes}
                  onChange={handleInputChange}
                  rows="3"
                />
              </div>

              <div className="total-display">
                <strong>Total: ${calculateTotal().toFixed(2)}</strong>
              </div>

              <div className="modal-actions">
                <button type="button" onClick={() => setShowCreateModal(false)} className="btn btn-secondary">
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary">
                  Create Invoice
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {selectedInvoice && (
        <div className="modal-overlay" onClick={() => setSelectedInvoice(null)}>
          <div className="modal-content invoice-preview" onClick={e => e.stopPropagation()}>
            <div dangerouslySetInnerHTML={{ __html: generateInvoiceHtml(selectedInvoice) }} />
            <button onClick={() => setSelectedInvoice(null)} className="btn btn-secondary">
              Close
            </button>
          </div>
        </div>
      )}
    </div>
  );
}

export default Invoicing;
