import { Args, Query, Resolver, ResolveReference } from '@nestjs/graphql';
import { Wallet } from './graphql/type';
import { AppService } from './app.service';

@Resolver(() => Wallet)
export class AppResolver {
  constructor(private identityService: AppService) {}

  @Query(() => Wallet)
  getWallet(@Args('id') id: string): Wallet {
    return this.identityService.findById(id);
  }

  @ResolveReference()
  resolveReference(reference: { __typename: string; id: string }): Wallet {
    return this.identityService.findById(reference.id);
  }
}
