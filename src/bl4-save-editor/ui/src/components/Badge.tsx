interface Props {
  variant: 'favorite' | 'junk' | 'equipped' | 'success' | 'error';
  children: string;
}

export function Badge({ variant, children }: Props) {
  return <span className={`badge badge-${variant}`}>{children}</span>;
}
