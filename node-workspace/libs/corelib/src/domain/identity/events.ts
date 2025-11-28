import { EventMeta } from "../shared/event";


export type IdentityDomainEvent =

    | {
        type: "IdentityCreated";
        data: {
            meta: EventMeta;
            event_name: string;
        };
    }
    | {
        type: "IdentityUpdated";
        data: {
            meta: EventMeta;
            event_name: string;
        };
    }
    | {
        type: "IdentityDeleted";
        data: {
            meta: EventMeta;
            event_name: string;
        };
    }
    | {
        type: "IdentitySoftDeleted";
        data: {
            meta: EventMeta;
            event_name: string;
        };
    }
    | {
        type: "IdentityRestoredFromSoftDeleted";
        data: {
            meta: EventMeta;
            event_name: string;
        };
    };



export const IdentityDomainEventFactory = {
    identityCreated(args: {
        meta: EventMeta;
        event_name: string;
    }): IdentityDomainEvent {
        return { type: "IdentityCreated", data: args };
    },
    identityUpdated(args: {
        meta: EventMeta;
        event_name: string;
    }): IdentityDomainEvent {
        return { type: "IdentityUpdated", data: args };
    },
    identityDeleted(args: {
        meta: EventMeta;
        event_name: string;
    }): IdentityDomainEvent {
        return { type: "IdentityDeleted", data: args };
    },
    identitySoftDeleted(args: {
        meta: EventMeta;
        event_name: string;
    }): IdentityDomainEvent {
        return { type: "IdentitySoftDeleted", data: args };
    },
    identityRestoredFromSoftDeleted(args: {
        meta: EventMeta;
        event_name: string;
    }): IdentityDomainEvent {
        return { type: "IdentityRestoredFromSoftDeleted", data: args };
    },

};