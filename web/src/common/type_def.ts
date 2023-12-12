interface PageResult<T> {
    total: number;
    data: T;
}

interface BaseQuery {
    page: number;
    size: number;
}

export type {PageResult, BaseQuery}