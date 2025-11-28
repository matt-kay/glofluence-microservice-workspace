import { BrandedString } from "../../../types/brand";
import { DomainError } from "../error";

export type Email = BrandedString<"Email">;

function isValidEmail(raw: string): boolean {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(raw);
}

function normalizeEmail(raw: string): string {
    return raw.trim().toLowerCase();
}

export function EmailFromString(raw: string): Email {
    const normalized = normalizeEmail(raw);

    if (!isValidEmail(normalized)) {
        throw DomainError.validation("Invalid email address");
    }

    return normalized as Email;
}

export function parseEmail(raw: string): Email | DomainError {
    const normalized = normalizeEmail(raw);

    if (!isValidEmail(normalized)) {
        return DomainError.validation("Invalid email address");
    }

    return normalized as Email;
}

export function isEmail(value: unknown): value is Email {
    return typeof value === "string" && isValidEmail(value);
}

export function unsafeEmail(raw: string): Email {
    return normalizeEmail(raw) as Email;
}

export function emailToString(email: Email): string {
    return email;
}
