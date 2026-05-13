export function formatRelativeDate(date: Date): string {
  const now = new Date();

  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const targetDate = new Date(
    date.getFullYear(),
    date.getMonth(),
    date.getDate(),
  );

  const diffTime = today.getTime() - targetDate.getTime();
  const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));

  if (diffDays === 0) return "hoje";
  if (diffDays === 1) return "ontem";
  if (diffDays < 7) return `há ${diffDays} dias`;

  const diffWeeks = Math.floor(diffDays / 7);
  if (diffWeeks === 1) return "há 1 semana";
  if (diffWeeks < 4) return `há ${diffWeeks} semanas`;

  const diffMonths = Math.floor(diffDays / 30);
  if (diffMonths === 1) return "há 1 mês";
  if (diffMonths < 12) return `há ${diffMonths} meses`;

  const diffYears = Math.floor(diffDays / 365);
  return diffYears === 1 ? "há 1 ano" : `há ${diffYears} anos`;
}

export function formatArchivedDate(date: Date): string {
  const relative = formatRelativeDate(date);
  return relative === "hoje" ? "arquivado hoje" : `arquivado ${relative}`;
}

export function formatResurfaceDate(date: Date): string {
  const relative = formatRelativeDate(date);
  return relative === "hoje"
    ? "você escreveu isso hoje"
    : `você escreveu isso ${relative}`;
}
