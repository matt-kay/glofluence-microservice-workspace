import { formatHuman } from "../../../helpers/datetime";
import { Brand } from "../../../types/brand";

export type Deleted = Brand<
    Readonly<{
        deleted: boolean;
        deletedAt?: Date;
    }>,
    "Deleted"
>;

export function deletedNew(): Deleted {
    return {
        deleted: false,
        deletedAt: undefined,
        __brand: "Deleted",
    } as Deleted;
}

/** immutable — returns new instance */
export function deletedMarkDeleted(d: Deleted): Deleted {
    return {
        ...d,
        deleted: true,
        deletedAt: new Date(),
    } as Deleted;
}

/** immutable — returns new instance */
export function deletedRestore(d: Deleted): Deleted {
    return {
        ...d,
        deleted: false,
        deletedAt: undefined,
    } as Deleted;
}

export function deletedIsDeleted(d: Deleted): boolean {
    return d.deleted;
}

export function deletedDeletedAt(d: Deleted): Date | undefined {
    return d.deletedAt ? new Date(d.deletedAt.getTime()) : undefined;
}

export function deletedStatus(d: Deleted, locale = "en-US"): string {
    if (!d.deleted) return "Active";
    if (!d.deletedAt) return "Deleted";

    const human = formatHuman(d.deletedAt, locale);
    return `Deleted at ${human}`;
}

export function deletedToString(d: Deleted, locale = "en-US"): string {
    return deletedStatus(d, locale);
}