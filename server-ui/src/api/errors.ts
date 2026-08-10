export class ApiError extends Error {
    readonly code: 'unauthorized' | 'http' | 'api' | 'unknown'
    readonly params?: Record<string, unknown>

    constructor(
        code: ApiError['code'],
        params?: Record<string, unknown>,
        cause?: unknown,
    ) {
        super(code)
        this.name = 'ApiError'
        this.code = code
        this.params = params
        if (cause !== undefined) {
            ;(this as Error & { cause?: unknown }).cause = cause
        }
    }
}

export function isApiError(e: unknown): e is ApiError {
    return e instanceof ApiError
}
