

/** Generic branded type helper */
export type Brand<T, K extends string> = T & { readonly __brand: K };

// Convenience aliases if you like:
export type BrandedString<K extends string> = Brand<string, K>;
export type BrandedDate<K extends string> = Brand<Date, K>;
