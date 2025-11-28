
export type DomainErrorKind =
    | "Validation"
    | "Conflict"
    | "NotFound"
    | "Forbidden";

export class DomainError extends Error {
    public readonly kind: DomainErrorKind;
    public readonly message: string;
    public readonly source?: Error;

    private constructor(
        kind: DomainErrorKind,
        message: string,
        source?: Error
    ) {

        const prefix = kind.toLowerCase(); // "validation" | "conflict" | ...
        super(`${prefix}: ${message}`);

        this.name = "DomainError";
        this.kind = kind;
        this.message = message;
        this.source = source;

        // Fix prototype chain (needed when targeting ES5)
        Object.setPrototypeOf(this, new.target.prototype);
    }


    static validation(message: string): DomainError {
        return new DomainError("Validation", message);
    }

    static conflict(message: string): DomainError {
        return new DomainError("Conflict", message);
    }

    static notFound(message: string): DomainError {
        return new DomainError("NotFound", message);
    }

    static forbidden(message: string): DomainError {
        return new DomainError("Forbidden", message);
    }

    static conflictWith<E extends Error>(message: string, err: E): DomainError {
        return new DomainError("Conflict", message, err);
    }
}




// // Simple
// throw DomainError.validation("email is invalid");

// // With cause/source
// try {
//   await someDbCall();
// } catch (e) {
//   throw DomainError.conflictWith("user already exists", e as Error);
// }
