import { useState } from 'react';

interface Props {
  serial: string;
  truncate?: boolean;
}

export function Serial({ serial, truncate = false }: Props) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(serial);
    setCopied(true);
    setTimeout(() => setCopied(false), 1000);
  };

  const displaySerial = truncate && serial.length > 40
    ? `${serial.substring(0, 40)}...`
    : serial;

  return (
    <div className="serial-wrapper">
      <code className="serial" title={serial}>{displaySerial}</code>
      <button
        className={`btn-copy ${copied ? 'copied' : ''}`}
        onClick={handleCopy}
        title="Copy serial"
      >
        {copied ? '✓' : '⎘'}
      </button>
    </div>
  );
}
