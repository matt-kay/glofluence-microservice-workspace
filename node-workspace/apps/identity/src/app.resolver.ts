import { Args, Query, Resolver, ResolveReference } from '@nestjs/graphql';
import { Identity } from './graphql/type';
import { IdentityService } from './app.service';

@Resolver(() => Identity)
export class IdentitysResolver {
  constructor(private identityService: IdentityService) {}

  @Query(() => Identity)
  getIdentity(@Args('id') id: string): Identity {
    return this.identityService.findById(id);
  }

  @ResolveReference()
  resolveReference(reference: { __typename: string; id: string }): Identity {
    return this.identityService.findById(reference.id);
  }
}
