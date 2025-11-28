import { Module } from '@nestjs/common';
import { CorelibService } from './corelib.service';

@Module({
  providers: [CorelibService],
  exports: [CorelibService],
})
export class CorelibModule {}
