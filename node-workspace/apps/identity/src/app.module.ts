import { Module } from '@nestjs/common';
import {
  ApolloFederationDriver,
  ApolloFederationDriverConfig,
} from '@nestjs/apollo';
import { GraphQLModule } from '@nestjs/graphql';
import { ConfigModule } from '@nestjs/config';
import configuration from './config/configuration';
import { AppResolver } from './app.resolver';
import { ApolloServerPluginLandingPageLocalDefault } from '@apollo/server/plugin/landingPage/default';
import { IdentityService } from '../../../libs/corelib/src/application/identity/service';
import { IDENTITY_REPOSITORY } from '../../../libs/corelib/src/domain/identity/ports/repository';
import { InMemoryIdentityRepository } from '../../../libs/corelib/src/infrastructure/user/repository/in_memory_impl';

@Module({
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
      load: [configuration],
    }),
    GraphQLModule.forRoot<ApolloFederationDriverConfig>({
      driver: ApolloFederationDriver,
      autoSchemaFile: { federation: 2 },
      playground: false,
      plugins: [ApolloServerPluginLandingPageLocalDefault()],
      debug: false
    }),
  ],
  providers: [
    IdentityService,
    {
      provide: IDENTITY_REPOSITORY,
      useClass: InMemoryIdentityRepository,
    },
    AppResolver],
})
export class AppModule { }
