import type { InventoryItem } from '../types';
import { ItemIcon } from './ItemIcon';
import { Badge } from './Badge';

interface Props {
  item: InventoryItem;
  selected?: boolean;
  onClick?: () => void;
  onContextMenu?: (e: React.MouseEvent) => void;
}

export function ItemRow({ item, selected, onClick, onContextMenu }: Props) {
  const statusClass = item.decode_success ? 'decode-success' : 'decode-failed';

  // Build display name from decoded info
  const displayName = item.name
    || [item.rarity, item.manufacturer, item.weapon_type].filter(Boolean).join(' ')
    || 'Unknown Item';

  return (
    <div
      className={`item-row ${statusClass} ${selected ? 'selected' : ''}`}
      onClick={onClick}
      onContextMenu={onContextMenu}
    >
      <ItemIcon itemType={item.item_type} />

      <div className="item-info">
        <div className="item-name">{displayName}</div>
        {(item.manufacturer || item.weapon_type) && (
          <div className="item-meta">
            {[item.manufacturer, item.weapon_type].filter(Boolean).join(' ')}
          </div>
        )}
      </div>

      {item.level && <span className="item-level">L{item.level}</span>}

      <div className="item-flags">
        {item.is_equipped && <Badge variant="equipped">E</Badge>}
        {item.is_favorite && <Badge variant="favorite">★</Badge>}
        {item.is_junk && <Badge variant="junk">J</Badge>}
      </div>
    </div>
  );
}
