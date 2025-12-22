import { useState, useEffect } from 'react';
import type { InventoryItem, BankInfo } from '../types';
import { getBank } from '../api';
import { ItemsList } from './ItemsList';
import { ItemDetailPanel } from './ItemDetailPanel';

export function BankPanel() {
  const [bankInfo, setBankInfo] = useState<BankInfo | null>(null);
  const [selectedItem, setSelectedItem] = useState<InventoryItem | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const loadBank = async () => {
      try {
        setLoading(true);
        const info = await getBank();
        setBankInfo(info);
      } catch (e) {
        setError(e instanceof Error ? e.message : 'Failed to load bank');
      } finally {
        setLoading(false);
      }
    };

    loadBank();
  }, []);

  if (loading) {
    return (
      <div className="panel items-panel">
        <p className="muted">Loading bank...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="panel items-panel">
        <p className="error">{error}</p>
      </div>
    );
  }

  if (!bankInfo) {
    return (
      <div className="panel items-panel">
        <p className="muted">No bank data available</p>
      </div>
    );
  }

  return (
    <div className="panel items-panel">
      <div className="action-bar">
        <span className="items-count">
          {bankInfo.count} / {bankInfo.max_capacity}
        </span>
        {bankInfo.sdu_warning && (
          <span className="sdu-warning">SDU capacity exceeded</span>
        )}
      </div>

      <div className="items-split">
        <ItemsList
          items={bankInfo.items}
          selectedSerial={selectedItem?.serial}
          onSelect={setSelectedItem}
        />
        <ItemDetailPanel item={selectedItem} />
      </div>
    </div>
  );
}
