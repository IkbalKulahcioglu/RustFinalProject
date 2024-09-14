import React, { useState } from 'react';
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Send, MessageSquare } from 'lucide-react';

const StellarPaymentSystem = () => {
  const [recipient, setRecipient] = useState('');
  const [amount, setAmount] = useState('');
  const [message, setMessage] = useState('');
  const [lastMessage, setLastMessage] = useState('');
  const [alert, setAlert] = useState({ show: false, type: '', message: '' });

  const handleSendPayment = async () => {
    // Here you would integrate with your Soroban contract
    // For now, we'll just simulate a successful payment
    setAlert({ show: true, type: 'success', message: 'Payment sent successfully!' });
    setRecipient('');
    setAmount('');
    setMessage('');
  };

  const handleGetMessage = async () => {
    // Here you would call your Soroban contract to get the last message
    // For now, we'll just set a dummy message
    setLastMessage('Last received message: ' + message);
  };

  return (
    <div className="max-w-md mx-auto mt-10 p-6 bg-white rounded-lg shadow-xl">
      <h1 className="text-2xl font-bold mb-6">Stellar Payment System</h1>
      
      <div className="space-y-4">
        <div>
          <Label htmlFor="recipient">Recipient Address</Label>
          <Input
            id="recipient"
            value={recipient}
            onChange={(e) => setRecipient(e.target.value)}
            placeholder="G..."
          />
        </div>
        
        <div>
          <Label htmlFor="amount">Amount (XLM)</Label>
          <Input
            id="amount"
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            placeholder="0.00"
          />
        </div>
        
        <div>
          <Label htmlFor="message">Message</Label>
          <Input
            id="message"
            value={message}
            onChange={(e) => setMessage(e.target.value)}
            placeholder="Enter a message"
          />
        </div>
        
        <Button onClick={handleSendPayment} className="w-full">
          <Send className="mr-2 h-4 w-4" /> Send Payment
        </Button>
        
        <Button onClick={handleGetMessage} variant="outline" className="w-full">
          <MessageSquare className="mr-2 h-4 w-4" /> Get Last Message
        </Button>
      </div>
      
      {alert.show && (
        <Alert className="mt-4" variant={alert.type === 'success' ? 'default' : 'destructive'}>
          <AlertTitle>{alert.type === 'success' ? 'Success' : 'Error'}</AlertTitle>
          <AlertDescription>{alert.message}</AlertDescription>
        </Alert>
      )}
      
      {lastMessage && (
        <div className="mt-4 p-2 bg-gray-100 rounded">
          <p>{lastMessage}</p>
        </div>
      )}
    </div>
  );
};

export default StellarPaymentSystem;