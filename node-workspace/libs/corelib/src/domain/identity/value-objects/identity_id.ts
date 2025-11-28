import { BrandedString } from "../../../types/brand";
import { uuidValidateV4 } from "../../../helpers/uuid";
import { DomainError } from "../../shared/error";
import { v4 as uuidV4 } from "uuid";

export type IdentityId = BrandedString<"IdentityId">;

export function newIdentityId(): IdentityId {
    let uuid = uuidV4();

    return uuid as IdentityId;
}
export function IdentityIdFromUuid(uuid: string): IdentityId {
    if (!uuidValidateV4(uuid)) {
        throw DomainError.validation("Invalid identity id");
    }
    return uuid as IdentityId;
}

export function parseIdentityId(uuid: string): IdentityId | DomainError {
    if (!uuidValidateV4(uuid)) {
        return DomainError.validation("Invalid identity id");
    }
    return uuid as IdentityId;
}

export function isIdentityId(value: unknown): value is IdentityId {
    return typeof value === "string" && uuidValidateV4(value);
}

export function unsafeIdentityId(uuid: string): IdentityId {
    return uuid as IdentityId;
}

export function identityIdToString(id: IdentityId): string {
    return id;
}
