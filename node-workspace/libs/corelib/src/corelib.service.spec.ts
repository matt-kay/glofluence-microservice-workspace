import { Test, TestingModule } from '@nestjs/testing';
import { CorelibService } from './corelib.service';

describe('CorelibService', () => {
  let service: CorelibService;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [CorelibService],
    }).compile();

    service = module.get<CorelibService>(CorelibService);
  });

  it('should be defined', () => {
    expect(service).toBeDefined();
  });
});
