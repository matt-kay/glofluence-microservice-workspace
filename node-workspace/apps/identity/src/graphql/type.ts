import { Directive, Field, ID, ObjectType } from '@nestjs/graphql';
import { Identity as DomainIdentity } from '../../../../libs/corelib/src/domain/identity';
import { timestampCreatedHuman, timestampUpdatedHuman } from '../../../../libs/corelib/src/domain/shared/value-objects/timestamp';
import { deletedIsDeleted, deletedStatus } from '../../../../libs/corelib/src/domain/shared/value-objects/soft_delete';

@ObjectType()
@Directive('@key(fields: "id")')
export class Identity {
  @Field(() => ID)
  id: string;

  @Field({ nullable: true })
  primaryEmail?: string;

  @Field()
  createdAt: string;

  @Field()
  updatedAt: string;

  @Field(() => Boolean)
  deleted: boolean;

  @Field()
  deletedAt: string;

  static fromDomain(domain: DomainIdentity): Identity {
    const timestamps = domain.getTimestamps;
    const deleted = domain.getDeleted;

    return {
      id: domain.id,
      primaryEmail: domain.getPrimaryEmail,
      createdAt: timestampCreatedHuman(timestamps),
      updatedAt: timestampUpdatedHuman(timestamps),
      deleted: deletedIsDeleted(deleted),
      deletedAt: deletedStatus(deleted)
    }
  }
}
