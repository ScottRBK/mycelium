import { User, UserRole } from './models';

export interface RequestContext {
    userId: number;
    role: UserRole;
    authenticated: boolean;
    timestamp: Date;
}

export class AuthMiddleware {
    private sessions: Map<string, RequestContext> = new Map();

    authenticate(token: string): RequestContext {
        const context = this.sessions.get(token);
        if (!context || !context.authenticated) {
            throw new Error('Unauthorized');
        }
        return context;
    }

    authorize(context: RequestContext, requiredRole: UserRole): boolean {
        const roleHierarchy: Record<UserRole, number> = {
            [UserRole.Guest]: 0,
            [UserRole.User]: 1,
            [UserRole.Moderator]: 2,
            [UserRole.Admin]: 3,
        };

        return roleHierarchy[context.role] >= roleHierarchy[requiredRole];
    }

    createSession(userId: number, role: UserRole): string {
        const token = this.generateToken();
        this.sessions.set(token, {
            userId,
            role,
            authenticated: true,
            timestamp: new Date(),
        });
        return token;
    }

    private generateToken(): string {
        return Math.random().toString(36).substring(2);
    }
}
