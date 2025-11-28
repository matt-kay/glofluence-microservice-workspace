import { IdentityId } from "../value-objects/identity_id";
import Identity from "../../identity";
import { Specification } from "../../shared/spec";


export interface IIdentityRepository {

    save(identity: Identity): Promise<void>;


    findById(id: IdentityId): Promise<Identity | null>;


    query(
        spec: Specification<Identity>,
        limit: number,
        offset: number
    ): Promise<Identity[]>;


    delete(id: IdentityId): Promise<void>;
}


export const IDENTITY_REPOSITORY = Symbol('IDENTITY_REPOSITORY');