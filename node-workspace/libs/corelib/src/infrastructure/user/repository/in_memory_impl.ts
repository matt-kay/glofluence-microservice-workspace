// src/infrastructure/identity/in-memory-identity.repository.ts
import { Injectable } from '@nestjs/common';
import { IIdentityRepository } from '../../../domain/identity/ports/repository';
import Identity from '../../../domain/identity';
import { IdentityId } from '../../../domain/identity/value-objects/identity_id';
import { Specification } from '../../../domain/shared/spec';


@Injectable()
export class InMemoryIdentityRepository implements IIdentityRepository {
    private readonly items: Map<string, Identity> = new Map();

    private idToString(id: IdentityId): string {
        return id;
    }

    async save(identity: Identity): Promise<void> {
        const id = this.idToString(identity.id);
        this.items.set(id, identity);
    }

    async findById(id: IdentityId): Promise<Identity | null> {
        const key = this.idToString(id);
        return this.items.get(key) ?? null;
    }

    async query(
        spec: Specification<Identity>,
        limit: number,
        offset: number,
    ): Promise<Identity[]> {
        const all = Array.from(this.items.values());

        // Specification pattern: spec.isSatisfiedBy(entity)
        const filtered = all.filter((item) => spec.isSatisfiedBy(item));

        return filtered.slice(offset, offset + limit);
    }

    async delete(id: IdentityId): Promise<void> {
        const key = this.idToString(id);
        this.items.delete(key);
    }
}
