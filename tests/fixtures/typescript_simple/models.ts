export interface User {
    id: number;
    name: string;
    role: UserRole;
    active: boolean;
    createdAt: Date;
}

export enum UserRole {
    Admin = 'admin',
    User = 'user',
    Moderator = 'moderator',
    Guest = 'guest',
}

export type UserDTO = {
    id: number;
    name: string;
    role: UserRole;
    displayName: string;
};

export interface CreateUserRequest {
    name: string;
    role?: UserRole;
    email?: string;
}

export interface PaginatedResponse<T> {
    items: T[];
    total: number;
    page: number;
    pageSize: number;
}

export type UserFilter = {
    role?: UserRole;
    active?: boolean;
    search?: string;
};

export const DEFAULT_PAGE_SIZE = 20;
export const MAX_PAGE_SIZE = 100;
