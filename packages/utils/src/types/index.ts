export type NotUndefined<T, K extends keyof T> = Pick<Required<T>, K>[K];
