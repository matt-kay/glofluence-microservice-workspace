import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';
import { ConsoleLogger } from '@nestjs/common';

async function bootstrap() {
  const app = await NestFactory.create(AppModule, {
    logger: new ConsoleLogger({
      prefix: 'IdentitySubgraph',
    }),
  });
  await app.listen(process.env.PORT ?? 3005);
}
bootstrap();
