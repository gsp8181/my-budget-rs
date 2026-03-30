export default function formatCurrency(value, locale = 'en-GB', currency = 'GBP') {
  if (value == null) return '';
  const s = String(value).trim();
  if (s === '') return '';
  const num = Number(s.replace(/[^0-9.-]/g, ''));
  if (!Number.isFinite(num)) return '';
  return new Intl.NumberFormat(locale, { style: 'currency', currency }).format(num);
}
