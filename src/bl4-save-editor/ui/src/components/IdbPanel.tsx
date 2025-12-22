import { useState } from 'react';
import { connectDb } from '../api';
import { FormField } from './FormField';
import { Button } from './Button';

export function IdbPanel() {
  const [dbPath, setDbPath] = useState('');
  const [connected, setConnected] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleConnect = async () => {
    if (!dbPath) return;

    try {
      setLoading(true);
      setError(null);
      await connectDb(dbPath);
      setConnected(true);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to connect');
    } finally {
      setLoading(false);
    }
  };

  if (!connected) {
    return (
      <div className="panel idb-panel">
        <div className="idb-connect">
          <p className="muted">Connect to an items database to sync items.</p>

          <FormField
            label="Database Path"
            value={dbPath}
            onChange={setDbPath}
            placeholder="/path/to/items.db"
          />

          {error && <p className="error">{error}</p>}

          <Button
            onClick={handleConnect}
            disabled={!dbPath}
            loading={loading}
          >
            Connect Database
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div className="panel idb-panel">
      <div className="action-bar">
        <span className="muted">Connected to: {dbPath}</span>
      </div>
      <p className="muted">IDB browser coming soon...</p>
    </div>
  );
}
