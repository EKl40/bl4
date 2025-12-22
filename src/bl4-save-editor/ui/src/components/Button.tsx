import { ButtonHTMLAttributes, ReactNode } from 'react';

interface Props extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'ghost' | 'icon' | 'warning';
  loading?: boolean;
  children: ReactNode;
}

export function Button({
  variant = 'primary',
  loading,
  disabled,
  className = '',
  children,
  ...props
}: Props) {
  return (
    <button
      className={`btn btn-${variant} ${className}`}
      disabled={disabled || loading}
      {...props}
    >
      {loading ? 'Loading...' : children}
    </button>
  );
}
