import { BrandedDate } from "../../../types/brand";
import { formatDate, formatHuman, formatTime } from "../../../helpers/datetime";

export type OcurredAt = BrandedDate<"OcurredAt">;

export function ocurredAtNow(): OcurredAt {
    return new Date() as OcurredAt;
}

export function ocurredAtFromUtc(dt: Date): OcurredAt {
    return new Date(dt.getTime()) as OcurredAt;
}

export function ocurredAtAsUtc(o: OcurredAt): Date {
    return new Date(o.getTime());
}

export function ocurredAtToHumanString(
    o: OcurredAt,
    locale = "en-US"
): string {
    return formatHuman(o, locale);
}

export function ocurredAtToDateString(o: OcurredAt): string {
    return formatDate(o);
}

export function ocurredAtToTimeString(
    o: OcurredAt,
    locale = "en-US"
): string {
    return formatTime(o, locale);
}

export function ocurredAtToString(
    o: OcurredAt,
    locale = "en-US"
): string {
    return ocurredAtToHumanString(o, locale);
}