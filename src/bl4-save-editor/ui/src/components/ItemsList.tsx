import type { InventoryItem } from '../types';
import { ItemRow } from './ItemRow';

interface Props {
  items: InventoryItem[];
  selectedSerial?: string | null;
  onSelect: (item: InventoryItem) => void;
  onContextMenu?: (e: React.MouseEvent, item: InventoryItem) => void;
}

export function ItemsList({ items, selectedSerial, onSelect, onContextMenu }: Props) {
  if (items.length === 0) {
    return (
      <div className="items-list empty">
        <p className="muted">No items</p>
      </div>
    );
  }

  return (
    <div className="items-list">
      {items.map((item) => (
        <ItemRow
          key={item.serial}
          item={item}
          selected={item.serial === selectedSerial}
          onClick={() => onSelect(item)}
          onContextMenu={onContextMenu ? (e) => onContextMenu(e, item) : undefined}
        />
      ))}
    </div>
  );
}
