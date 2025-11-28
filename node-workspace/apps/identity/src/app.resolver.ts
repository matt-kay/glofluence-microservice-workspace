import { Args, Query, Resolver, ResolveReference } from '@nestjs/graphql';
import { NotFoundException, ConflictException } from '@nestjs/common';
import { Identity } from './graphql/type';
import { IdentityService } from '../../../libs/corelib/src/application/identity/service';
import { IdentityIdFromUuid } from '../../../libs/corelib/src/domain/identity/value-objects/identity_id';
import { DomainError } from '../../../libs/corelib/src/domain/shared/error';

@Resolver(() => Identity)
export class AppResolver {
  constructor(private identityService: IdentityService) { }

  @Query(() => Identity)
  async getIdentity(@Args('id') id: string): Promise<Identity> {
    const uid = IdentityIdFromUuid(id);
    const identity = await this.identityService.findById(uid);

    if (!identity) {
      throw DomainError.notFound('identity')
    }

    return Identity.fromDomain(identity);
  }

  @ResolveReference()
  async resolveReference(
    reference: { __typename: string; id: string },
  ): Promise<Identity | null> {
    const uid = IdentityIdFromUuid(reference.id);
    const identity = await this.identityService.findById(uid);
    if (!identity) {
      return null; // federation references are usually nullable
    }
    return Identity.fromDomain(identity);
  }
}
