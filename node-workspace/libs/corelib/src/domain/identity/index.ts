

import {
    Deleted, deletedNew,
    deletedMarkDeleted,
    deletedRestore,
} from "../shared/value-objects/soft_delete";
import {
    Timestamp,
    timestampNew,
    timestampTouch,
} from "../shared/value-objects/timestamp";
import {
    ocurredAtNow,
} from "../shared/value-objects/ocurred_at";

import { EventMeta, makeEventMeta } from "../shared/event";

import { IdentityId } from "./value-objects/identity_id";

import { newEventId } from "../shared/value-objects/event_id";


export class Identity {
    public readonly id: IdentityId;

    private primaryEmail?: string;

    private timestamps: Timestamp;
    private deleted: Deleted;

    private version: number;


    private constructor(args: {
        id: IdentityId;
        primaryEmail?: string;
        timestamps: Timestamp;
        deleted: Deleted;
        version: number;
    }) {
        this.id = args.id;
        this.primaryEmail = args.primaryEmail;
        this.timestamps = args.timestamps;
        this.deleted = args.deleted;
        this.version = args.version;
    }

    static create(args: {
        id: IdentityId;
        primaryEmail?: string;
    }): Identity {
        const timestamps = timestampNew();
        const deleted = deletedNew();
        const version = 0;

        const identity = new Identity({
            id: args.id,
            primaryEmail: args.primaryEmail,
            timestamps,
            deleted,
            version,
        });

        const meta: EventMeta = {
            event_id: newEventId(),
            occurred_at: ocurredAtNow(),
            aggregate_id: args.id,
            aggregate_version: 1,
        };



        return identity;
    }

    changePrimaryEmail(newEmail: string): void {
        if (this.primaryEmail === newEmail) {
            return; // no-op
        }

        this.primaryEmail = newEmail;
        this.touch();


    }

    markAsSoftDeleted(): void {
        this.deleted = deletedMarkDeleted(this.deleted);
        this.touch();

    }


    restoreFromSoftDeleted(): void {
        this.deleted = deletedRestore(this.deleted);
        this.touch();


    }


    markAsDeleted(): void {

        this.deleted = deletedMarkDeleted(this.deleted);
        this.touch();


    }


    nextMeta(): EventMeta {
        return makeEventMeta({
            event_id: newEventId(),
            occurred_at: ocurredAtNow(),
            aggregate_id: this.id,           // branded string -> string
            aggregate_version: this.version + 1,
        });
    }



    private touch(): void {
        this.timestamps = timestampTouch(this.timestamps);
        this.version += 1;
    }


    getPrimaryEmail(): string | undefined {
        return this.primaryEmail;
    }

    getTimestamps(): Timestamp {
        return this.timestamps;
    }

    getDeleted(): Deleted {
        return this.deleted;
    }

    getVersion(): number {
        return this.version;
    }
}

export default Identity;
