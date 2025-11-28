// libs/corelib/src/domain/shared/error.ts
import { HttpException, HttpStatus } from '@nestjs/common';

export type DomainErrorKind =
    | 'Validation'
    | 'Conflict'
    | 'NotFound'
    | 'Forbidden';

function statusFromKind(kind: DomainErrorKind): HttpStatus {
    switch (kind) {
        case 'Validation':
            return HttpStatus.BAD_REQUEST; // 400
        case 'Conflict':
            return HttpStatus.CONFLICT; // 409
        case 'NotFound':
            return HttpStatus.NOT_FOUND; // 404
        case 'Forbidden':
            return HttpStatus.FORBIDDEN; // 403
        default:
            return HttpStatus.INTERNAL_SERVER_ERROR;
    }
}

export class DomainError extends HttpException {
    public readonly kind: DomainErrorKind;
    public readonly source?: Error;

    private constructor(kind: DomainErrorKind, message: string, source?: Error) {
        const status = statusFromKind(kind);

        const responseBody = {
            statusCode: status,
            message,
            error: kind,
            code: kind.toUpperCase(), // nice for GraphQL extensions.code
            ...(source && { source: source.message }),
        };

        super(responseBody, status);

        this.name = 'DomainError';
        this.kind = kind;
        this.source = source;

        // Fix prototype chain (for older targets)
        Object.setPrototypeOf(this, new.target.prototype);
    }

    static validation(message: string): DomainError {
        return new DomainError('Validation', message);
    }

    static conflict(message: string): DomainError {
        return new DomainError('Conflict', message);
    }

    static notFound(message: string): DomainError {
        return new DomainError('NotFound', message);
    }

    static forbidden(message: string): DomainError {
        return new DomainError('Forbidden', message);
    }

    static conflictWith<E extends Error>(message: string, err: E): DomainError {
        return new DomainError('Conflict', message, err);
    }
}
