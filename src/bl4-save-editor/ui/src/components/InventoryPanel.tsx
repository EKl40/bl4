import { useState } from 'react';
import type { InventoryItem } from '../types';
import { ItemsList } from './ItemsList';
import { ItemDetailPanel } from './ItemDetailPanel';

interface Props {
  items: InventoryItem[];
}

export function InventoryPanel({ items }: Props) {
  const [selectedItem, setSelectedItem] = useState<InventoryItem | null>(null);

  return (
    <div className="panel items-panel">
      <div className="action-bar">
        <span className="items-count">{items.length} items</span>
      </div>

      <div className="items-split">
        <ItemsList
          items={items}
          selectedSerial={selectedItem?.serial}
          onSelect={setSelectedItem}
        />
        <ItemDetailPanel item={selectedItem} />
      </div>
    </div>
  );
}
