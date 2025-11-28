import { Injectable } from '@nestjs/common';
import { Wallet } from './graphql/type';

@Injectable()
export class AppService {
  findById(id: string): Wallet {
    const wallet = new Wallet();
    wallet.id = id;
    wallet.balance = 1000;
    wallet.createdAt = 'now';
    wallet.updatedAt = 'now';
    wallet.deleted = false;
    wallet.deletedAt = 'now';

    return wallet;
  }
}
