import { Directive, Field, ID, ObjectType } from '@nestjs/graphql';

@ObjectType()
@Directive('@key(fields: "id")')
export class Wallet {
  @Field(() => ID)
  id: string;

  @Field()
  balance: number;

  @Field()
  createdAt: string;

  @Field()
  updatedAt: string;

  @Field(() => Boolean)
  deleted: boolean;

  @Field()
  deletedAt: string;
}
