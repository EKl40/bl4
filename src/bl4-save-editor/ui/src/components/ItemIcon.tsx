interface Props {
  itemType: string | null;
}

export function ItemIcon({ itemType }: Props) {
  const getIcon = (type: string | null): string => {
    if (!type) return '?';
    const t = type.toLowerCase();
    if (t.includes('pistol')) return 'P';
    if (t.includes('smg')) return 'S';
    if (t.includes('rifle') || t.includes('ar')) return 'R';
    if (t.includes('shotgun')) return 'SG';
    if (t.includes('sniper')) return 'SR';
    if (t.includes('launcher') || t.includes('heavy')) return 'H';
    if (t.includes('shield')) return 'SH';
    if (t.includes('grenade')) return 'G';
    if (t.includes('mod')) return 'M';
    if (t.includes('artifact')) return 'A';
    return '?';
  };

  return <div className="item-icon">{getIcon(itemType)}</div>;
}
