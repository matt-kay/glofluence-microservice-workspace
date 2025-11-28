import { BrandedString } from "../../../types/brand";
import { uuidValidateV4 } from "../../../helpers/uuid";
import { DomainError } from "../error";
import { v4 as uuidv4 } from "uuid";

export type EventId = BrandedString<"EventId">;

export function newEventId(): EventId {
    let uuid = uuidv4();
    return uuid as EventId;
}
export function EventIdFromUuid(uuid: string): EventId {
    if (!uuidValidateV4(uuid)) {
        throw DomainError.validation("Invalid identity id");
    }
    return uuid as EventId;
}

export function parseEventId(uuid: string): EventId | DomainError {
    if (!uuidValidateV4(uuid)) {
        return DomainError.validation("Invalid identity id");
    }
    return uuid as EventId;
}

export function isEventId(value: unknown): value is EventId {
    return typeof value === "string" && uuidValidateV4(value);
}

export function unsafeEventId(uuid: string): EventId {
    return uuid as EventId;
}

export function identityIdToString(id: EventId): string {
    return id;
}
