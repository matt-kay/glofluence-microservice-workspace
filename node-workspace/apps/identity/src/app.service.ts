import { Injectable } from '@nestjs/common';
import { Identity } from './graphql/type';

@Injectable()
export class AppService {
  findById(id: string): Identity {
    const identity = new Identity();
    identity.id = id;
    identity.createdAt = 'now';
    identity.updatedAt = 'now';
    identity.deleted = false;
    identity.deletedAt = 'now';

    return identity;
  }
}
