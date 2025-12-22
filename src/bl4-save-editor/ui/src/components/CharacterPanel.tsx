import { useState } from 'react';
import type { CharacterInfo } from '../types';
import { Button } from './Button';

interface Props {
  character: CharacterInfo;
  onUpdate: (updates: Partial<CharacterInfo>) => void;
}

export function CharacterPanel({ character, onUpdate }: Props) {
  const [editing, setEditing] = useState<string | null>(null);
  const [editValue, setEditValue] = useState('');

  const startEdit = (field: string, value: string | number | null) => {
    setEditing(field);
    setEditValue(String(value ?? ''));
  };

  const commitEdit = () => {
    if (!editing) return;

    const updates: Partial<CharacterInfo> = {};
    if (editing === 'name') {
      updates.name = editValue;
    } else {
      const numValue = parseInt(editValue, 10);
      if (!isNaN(numValue)) {
        (updates as Record<string, number>)[editing] = numValue;
      }
    }

    onUpdate(updates);
    setEditing(null);
  };

  const cancelEdit = () => {
    setEditing(null);
    setEditValue('');
  };

  const renderField = (
    label: string,
    field: string,
    value: string | number | null,
    editable = true
  ) => (
    <div className="field-row">
      <label>{label}</label>
      {editing === field ? (
        <div className="edit-inline">
          <input
            type={field === 'name' ? 'text' : 'number'}
            value={editValue}
            onChange={(e) => setEditValue(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === 'Enter') commitEdit();
              if (e.key === 'Escape') cancelEdit();
            }}
            autoFocus
          />
          <Button variant="ghost" onClick={commitEdit}>✓</Button>
          <Button variant="ghost" onClick={cancelEdit}>×</Button>
        </div>
      ) : (
        <div className="display-value">
          <span>{value ?? '—'}</span>
          {editable && (
            <Button variant="ghost" onClick={() => startEdit(field, value)}>
              Edit
            </Button>
          )}
        </div>
      )}
    </div>
  );

  return (
    <div className="panel character-panel">
      <section className="panel-section">
        <h3>Basic Info</h3>
        {renderField('Name', 'name', character.name)}
        {renderField('Class', 'class', character.class, false)}
        {renderField('Difficulty', 'difficulty', character.difficulty, false)}
      </section>

      <section className="panel-section">
        <h3>Level</h3>
        {renderField('Character Level', 'level', character.level, false)}
        {renderField('Character XP', 'xp', character.xp)}
        {renderField('Specialization Level', 'specialization_level', character.specialization_level, false)}
        {renderField('Specialization XP', 'specialization_xp', character.specialization_xp)}
      </section>

      <section className="panel-section">
        <h3>Currency</h3>
        {renderField('Cash', 'cash', character.cash)}
        {renderField('Eridium', 'eridium', character.eridium)}
      </section>
    </div>
  );
}
