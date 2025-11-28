import { formatDate, formatHuman, formatTime } from "../../../helpers/datetime";
import { Brand } from "../../../types/brand";

export type Timestamp = Brand<
    Readonly<{
        createdAt: Date;
        updatedAt?: Date;
    }>,
    "Timestamp"
>;

export function timestampNew(): Timestamp {
    const now = new Date();
    return {
        createdAt: now,
        updatedAt: undefined,
        __brand: "Timestamp",
    } as Timestamp;
}

export function timestampFromTimes(
    created: Date,
    updated?: Date
): Timestamp {
    return {
        createdAt: new Date(created.getTime()),
        updatedAt: updated ? new Date(updated.getTime()) : undefined,
        __brand: "Timestamp",
    } as Timestamp;
}

export function timestampCreatedAt(ts: Timestamp): Date {
    return new Date(ts.createdAt.getTime());
}

export function timestampUpdatedAt(ts: Timestamp): Date | undefined {
    return ts.updatedAt ? new Date(ts.updatedAt.getTime()) : undefined;
}

/** immutable — returns new instance */
export function timestampTouch(ts: Timestamp): Timestamp {
    return {
        ...ts,
        updatedAt: new Date(),
    } as Timestamp;
}

export function timestampCreatedHuman(
    ts: Timestamp,
    locale = "en-US"
): string {
    return formatHuman(ts.createdAt, locale);
}

export function timestampUpdatedHuman(
    ts: Timestamp,
    locale = "en-US"
): string {
    if (!ts.updatedAt) return "Never";
    return formatHuman(ts.updatedAt, locale);
}

export function timestampCreatedDate(ts: Timestamp): string {
    return formatDate(ts.createdAt);
}

export function timestampUpdatedDate(ts: Timestamp): string {
    if (!ts.updatedAt) return "Never";
    return formatDate(ts.updatedAt);
}

export function timestampCreatedTime(
    ts: Timestamp,
    locale = "en-US"
): string {
    return formatTime(ts.createdAt, locale);
}

export function timestampUpdatedTime(
    ts: Timestamp,
    locale = "en-US"
): string {
    if (!ts.updatedAt) return "Never";
    return formatTime(ts.updatedAt, locale);
}

export function timestampToString(
    ts: Timestamp,
    locale = "en-US"
): string {
    return `Created: ${timestampCreatedHuman(
        ts,
        locale
    )}, Updated: ${timestampUpdatedHuman(ts, locale)}`;
}
