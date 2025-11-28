import { Inject, Injectable } from "@nestjs/common";
import Identity from "../../domain/identity";
import { IDENTITY_REPOSITORY, type IIdentityRepository } from "../../domain/identity/ports/repository";
import { IdentityId, newIdentityId } from "../../domain/identity/value-objects/identity_id";
import { DomainError } from "../../domain/shared/error";


@Injectable()
export class IdentityService {
    constructor(
        @Inject(IDENTITY_REPOSITORY)
        private readonly repo: IIdentityRepository,
    ) { }

    // ------------------------------------------------------
    // Queries
    // ------------------------------------------------------

    async findById(id: IdentityId): Promise<Identity | null> {
        const identity = await this.repo.findById(id);
        return identity;
    }

    // ------------------------------------------------------
    // Commands
    // ------------------------------------------------------

    async createIdentity(primaryEmail: string): Promise<Identity> {
        const id = newIdentityId();

        const identity = Identity.create({
            id,
            primaryEmail,
        });

        await this.repo.save(identity);

        return identity;
    }

    async updateIdentity(
        identityId: IdentityId,
        primaryEmail?: string,
    ): Promise<Identity> {
        const identity = await this.repo.findById(identityId);

        if (!identity) {
            throw DomainError.notFound("identity");
        }

        if (primaryEmail !== undefined) {
            identity.changePrimaryEmail(primaryEmail);
        }

        await this.repo.save(identity);



        return identity;
    }

    async softDeleteIdentity(identityId: IdentityId): Promise<Identity> {
        const identity = await this.repo.findById(identityId);

        if (!identity) {
            throw DomainError.notFound("identity");
        }

        identity.markAsSoftDeleted();

        await this.repo.save(identity);


        return identity;
    }

    async restoreSoftDeletedIdentity(identityId: IdentityId): Promise<Identity> {
        const identity = await this.repo.findById(identityId);

        if (!identity) {
            throw DomainError.notFound("identity");
        }

        identity.restoreFromSoftDeleted();

        await this.repo.save(identity);


        return identity;
    }

    async permanentlyDeleteIdentity(identityId: IdentityId): Promise<void> {
        const identity = await this.repo.findById(identityId);

        if (!identity) {
            throw DomainError.notFound("identity");
        }

        await this.repo.delete(identity.id);

    }

}
