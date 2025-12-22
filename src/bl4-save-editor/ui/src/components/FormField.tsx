import { InputHTMLAttributes, ReactNode } from 'react';

interface Props extends Omit<InputHTMLAttributes<HTMLInputElement>, 'onChange'> {
  label: string;
  hint?: ReactNode;
  onChange?: (value: string) => void;
}

export function FormField({ label, hint, onChange, className = '', ...inputProps }: Props) {
  return (
    <div className={`form-field ${className}`}>
      <label>{label}</label>
      <input
        {...inputProps}
        onChange={onChange ? (e) => onChange(e.target.value) : undefined}
      />
      {hint && <small className="form-hint">{hint}</small>}
    </div>
  );
}
