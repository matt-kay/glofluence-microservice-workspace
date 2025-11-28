import { Directive, Field, ID, ObjectType } from '@nestjs/graphql';

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
}
