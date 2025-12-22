import { useState, useEffect } from 'react';
import type { InventoryItem, ItemDetail } from '../types';
import { getItemDetail } from '../api';
import { Serial } from './Serial';

interface Props {
  item: InventoryItem | null;
}

export function ItemDetailPanel({ item }: Props) {
  const [detail, setDetail] = useState<ItemDetail | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!item) {
      setDetail(null);
      return;
    }

    const loadDetail = async () => {
      setLoading(true);
      setError(null);
      try {
        const d = await getItemDetail(item.serial);
        setDetail(d);
      } catch (e) {
        setError(e instanceof Error ? e.message : 'Failed to decode');
        setDetail(null);
      } finally {
        setLoading(false);
      }
    };

    loadDetail();
  }, [item?.serial]);

  if (!item) {
    return (
      <div className="item-detail-panel empty">
        <p className="muted">Select an item to view details</p>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="item-detail-panel">
        <p className="muted">Loading...</p>
      </div>
    );
  }

  if (error || !detail) {
    return (
      <div className="item-detail-panel">
        <div className="detail-header">
          <h2 className="detail-title">{item.name || 'Unknown Item'}</h2>
          {error && <p className="error">{error}</p>}
        </div>
        <section className="detail-section">
          <h3>Serial</h3>
          <Serial serial={item.serial} />
        </section>
      </div>
    );
  }

  return (
    <div className="item-detail-panel">
      <div className="detail-header">
        <h2 className="detail-title">{detail.item_type_name}</h2>
        {(detail.manufacturer || detail.weapon_type) && (
          <p className="detail-subtitle">
            {[detail.manufacturer, detail.weapon_type].filter(Boolean).join(' ')}
          </p>
        )}
      </div>

      <section className="detail-section">
        <h3>Stats</h3>
        <div className="detail-rows">
          {detail.level && (
            <div className="detail-row">
              <span className="detail-label">Level</span>
              <span className="detail-value">{detail.level}</span>
            </div>
          )}
          {detail.rarity && (
            <div className="detail-row">
              <span className="detail-label">Rarity</span>
              <span className="detail-value">{detail.rarity}</span>
            </div>
          )}
          {detail.elements && (
            <div className="detail-row">
              <span className="detail-label">Elements</span>
              <span className="detail-value">{detail.elements}</span>
            </div>
          )}
        </div>
      </section>

      {detail.parts.length > 0 && (
        <section className="detail-section">
          <h3>Parts</h3>
          <div className="parts-list">
            {detail.parts.map((part) => (
              <div key={part.index} className="part-row">
                <span className="part-category">{part.category || `Part ${part.index}`}</span>
                <span className="part-name">{part.name || 'Unknown'}</span>
              </div>
            ))}
          </div>
        </section>
      )}

      <section className="detail-section">
        <h3>Serial</h3>
        <Serial serial={detail.serial} />
      </section>
    </div>
  );
}
