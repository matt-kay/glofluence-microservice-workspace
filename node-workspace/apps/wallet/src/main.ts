import { NestFactory } from '@nestjs/core';
import { ConsoleLogger } from '@nestjs/common';
import { AppModule } from './app.module';

async function bootstrap() {
  const app = await NestFactory.create(AppModule, {
    logger: new ConsoleLogger({
      prefix: 'WalletSubgraph',
    }),
  });
  await app.listen(process.env.PORT ?? 3006);
}
bootstrap();
