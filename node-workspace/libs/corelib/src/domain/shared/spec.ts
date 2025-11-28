// shared/specs.ts

export interface Specification<T> {
    // Synchronous version:
    isSatisfiedBy(candidate: T): boolean;
    // or, if you need async:
    // isSatisfiedBy(candidate: T): boolean | Promise<boolean>;
}
