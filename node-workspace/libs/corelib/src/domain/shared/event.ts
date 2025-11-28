import { IdentityId } from "../identity/value-objects/identity_id";
import { OcurredAt } from "../shared/value-objects/ocurred_at";
import { EventId } from "./value-objects/event_id";

export interface EventMeta {
    event_id: EventId;
    occurred_at: OcurredAt;
    aggregate_id: string | IdentityId;
    aggregate_version: number;
}

export function makeEventMeta(args: {
    event_id: EventId;
    occurred_at: OcurredAt;
    aggregate_id: string;
    aggregate_version: number;
}): EventMeta {
    return { ...args };
}
