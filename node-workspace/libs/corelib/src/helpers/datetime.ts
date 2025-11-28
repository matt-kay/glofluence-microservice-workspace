export function formatHuman(date: Date, locale = "en-US"): string {
    return new Intl.DateTimeFormat(locale, {
        month: "short",   // Nov
        day: "2-digit",   // 20
        year: "numeric",  // 2025
        hour: "numeric",  // 10
        minute: "2-digit",// 45
        hour12: true,     // AM/PM
    }).format(date);
}

/**
 * Returns "YYYY-MM-DD" in UTC.
 */
export function formatDate(date: Date): string {
    const year = date.getUTCFullYear();
    const month = String(date.getUTCMonth() + 1).padStart(2, "0");
    const day = String(date.getUTCDate()).padStart(2, "0");
    return `${year}-${month}-${day}`;
}

/**
 * Returns "10:45 AM" in local time.
 */
export function formatTime(date: Date, locale = "en-US"): string {
    return new Intl.DateTimeFormat(locale, {
        hour: "numeric",
        minute: "2-digit",
        hour12: true,
    }).format(date);
}