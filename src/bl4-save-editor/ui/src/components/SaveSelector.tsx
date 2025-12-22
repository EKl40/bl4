import { useState, useEffect } from 'react';
import type { DiscoveredProfile } from '../types';
import { discoverSaves, openSave } from '../api';
import { FormField } from './FormField';
import { Button } from './Button';

interface Props {
  onSaveOpened: () => void;
}

export function SaveSelector({ onSaveOpened }: Props) {
  const [profiles, setProfiles] = useState<DiscoveredProfile[]>([]);
  const [selectedSave, setSelectedSave] = useState<string>('');
  const [manualPath, setManualPath] = useState('');
  const [steamId, setSteamId] = useState('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const discover = async () => {
      try {
        const response = await discoverSaves();
        setProfiles(response.profiles);

        // Auto-select first character if found
        if (response.profiles.length > 0 && response.profiles[0].characters.length > 0) {
          const profile = response.profiles[0];
          const char = profile.characters[0];
          setSelectedSave(JSON.stringify({
            path: profile.path,
            slot: char.slot,
            steamId: profile.steam_id,
          }));
          setSteamId(profile.steam_id);
        }
      } catch {
        // Discovery failed, show manual input
      } finally {
        setLoading(false);
      }
    };

    discover();
  }, []);

  const handleOpenSelected = async () => {
    if (!selectedSave) return;

    try {
      setLoading(true);
      setError(null);
      const { path, slot, steamId: sid } = JSON.parse(selectedSave);
      await openSave(`${path}/client/${slot}.sav`, sid);
      onSaveOpened();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to open save');
      setLoading(false);
    }
  };

  const handleOpenManual = async () => {
    if (!manualPath || !steamId) return;

    try {
      setLoading(true);
      setError(null);
      await openSave(manualPath, steamId);
      onSaveOpened();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to open save');
      setLoading(false);
    }
  };

  if (loading && profiles.length === 0) {
    return (
      <div className="save-selector">
        <p className="muted">Scanning for saves...</p>
      </div>
    );
  }

  return (
    <div className="save-selector">
      <h2>Open Save File</h2>

      {error && <p className="error">{error}</p>}

      {profiles.length > 0 ? (
        <div className="discovered-saves">
          <label>Discovered Saves</label>
          <select
            value={selectedSave}
            onChange={(e) => setSelectedSave(e.target.value)}
          >
            {profiles.map((profile) =>
              profile.characters.map((char) => (
                <option
                  key={`${profile.steam_id}-${char.slot}`}
                  value={JSON.stringify({
                    path: profile.path,
                    slot: char.slot,
                    steamId: profile.steam_id,
                  })}
                >
                  {char.name} L{char.level} {char.difficulty}
                </option>
              ))
            )}
          </select>
          <Button onClick={handleOpenSelected} loading={loading}>
            Open Selected
          </Button>
        </div>
      ) : (
        <div className="manual-open">
          <FormField
            label="Steam ID"
            value={steamId}
            onChange={setSteamId}
            placeholder="e.g., 76561197960521364"
            hint="Your 17-digit Steam ID (find it in your save folder path)"
          />

          <FormField
            label="Save File Path"
            value={manualPath}
            onChange={setManualPath}
            placeholder="/path/to/1.sav"
          />

          <Button
            onClick={handleOpenManual}
            disabled={!steamId || !manualPath}
            loading={loading}
          >
            Open
          </Button>

          <div className="info-box">
            <h3>Save File Locations</h3>
            <p><strong>Windows:</strong></p>
            <code>%LOCALAPPDATA%\Gearbox\Borderlands4\Saved\SaveGames\[SteamID]\</code>
            <p><strong>Linux (Steam/Proton):</strong></p>
            <code>~/.local/share/Steam/steamapps/compatdata/[AppID]/pfx/drive_c/...</code>
          </div>
        </div>
      )}
    </div>
  );
}
