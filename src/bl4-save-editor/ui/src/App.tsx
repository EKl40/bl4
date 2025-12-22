import { useState, useEffect, useCallback } from 'react';
import {
  getSaveInfo,
  getCharacter,
  getInventory,
  setCharacter,
  saveChanges,
} from './api';
import type { SaveInfo, CharacterInfo, InventoryItem } from './types';
import { Tabs } from './components/Tabs';
import { Button } from './components/Button';
import { Modal } from './components/Modal';
import { SaveSelector } from './components/SaveSelector';
import { CharacterPanel } from './components/CharacterPanel';
import { InventoryPanel } from './components/InventoryPanel';
import { BankPanel } from './components/BankPanel';
import { IdbPanel } from './components/IdbPanel';

type TabId = 'character' | 'inventory' | 'bank' | 'idb';

const TABS = [
  { id: 'character', label: 'Character' },
  { id: 'inventory', label: 'Items' },
  { id: 'bank', label: 'Bank' },
  { id: 'idb', label: 'IDB' },
] as const;

export default function App() {
  const [saveInfo, setSaveInfo] = useState<SaveInfo | null>(null);
  const [character, setCharacterState] = useState<CharacterInfo | null>(null);
  const [inventory, setInventory] = useState<InventoryItem[]>([]);
  const [activeTab, setActiveTab] = useState<TabId>('character');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showSaveModal, setShowSaveModal] = useState(false);

  const loadSaveData = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const info = await getSaveInfo();
      setSaveInfo(info);
      if (info) {
        const char = await getCharacter();
        setCharacterState(char);
        const inv = await getInventory();
        setInventory(inv);
      }
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to load save data');
    } finally {
      setLoading(false);
    }
  }, []);

  const handleCharacterUpdate = async (updates: Partial<CharacterInfo>) => {
    try {
      setError(null);
      await setCharacter({
        name: updates.name ?? undefined,
        cash: updates.cash ?? undefined,
        eridium: updates.eridium ?? undefined,
        xp: updates.xp ?? undefined,
        specialization_xp: updates.specialization_xp ?? undefined,
      });
      await loadSaveData();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to update character');
    }
  };

  const handleSave = async () => {
    try {
      setLoading(true);
      setError(null);
      await saveChanges();
      setShowSaveModal(false);
      await loadSaveData();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to save');
    } finally {
      setLoading(false);
    }
  };

  const handleClose = () => {
    setSaveInfo(null);
    setCharacterState(null);
    setInventory([]);
  };

  useEffect(() => {
    loadSaveData();
  }, [loadSaveData]);

  // No save loaded - show selector
  if (!saveInfo) {
    return (
      <div className="app">
        <header className="header">
          <h1 className="muted">BL4</h1>
        </header>
        <main className="content">
          <SaveSelector onSaveOpened={loadSaveData} />
        </main>
      </div>
    );
  }

  return (
    <div className="app">
      <header className="header">
        <h1 className="muted">BL4</h1>
        <div className="header-right">
          <span className="save-path" title={saveInfo.path}>
            {saveInfo.character_name || 'Unknown'}
          </span>
          {saveInfo.modified && (
            <Button variant="warning" onClick={() => setShowSaveModal(true)}>
              Save
            </Button>
          )}
          <Button variant="ghost" onClick={handleClose}>
            Close
          </Button>
        </div>
      </header>

      {error && (
        <div className="error-banner">
          {error}
          <button onClick={() => setError(null)}>×</button>
        </div>
      )}

      <Tabs
        tabs={TABS as unknown as { id: string; label: string }[]}
        activeTab={activeTab}
        onTabChange={(id) => setActiveTab(id as TabId)}
      />

      <main className="content">
        {loading && <div className="loading">Loading...</div>}

        {!loading && activeTab === 'character' && character && (
          <CharacterPanel character={character} onUpdate={handleCharacterUpdate} />
        )}

        {!loading && activeTab === 'inventory' && (
          <InventoryPanel items={inventory} />
        )}

        {!loading && activeTab === 'bank' && <BankPanel />}

        {!loading && activeTab === 'idb' && <IdbPanel />}
      </main>

      <Modal
        open={showSaveModal}
        onClose={() => setShowSaveModal(false)}
        title="Save Changes?"
        actions={
          <>
            <Button variant="ghost" onClick={() => setShowSaveModal(false)}>
              Cancel
            </Button>
            <Button onClick={handleSave} loading={loading}>
              Save
            </Button>
          </>
        }
      >
        <p>Save your changes to the save file?</p>
      </Modal>
    </div>
  );
}
