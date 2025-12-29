export interface User {
    id: string,
    name: string | null | undefined,
    email: string,
    password: string | null,
    power: number,
    imageUrl: string,
    createdAt: string
}