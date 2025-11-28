/**
 * Basic email validation.
 * You can swap this out for a library or your own validator.
 */
export function isValidEmail(raw: string): boolean {
    // Very small, pragmatic check – not RFC-perfect, but good enough for many apps.
    // Replace with your own validator if you already have one.
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(raw);
}

/**
 * Normalize email before branding it.
 * Common patterns: trim + lowercase.
 */
export function normalizeEmail(raw: string): string {
    return raw.trim().toLowerCase();
}