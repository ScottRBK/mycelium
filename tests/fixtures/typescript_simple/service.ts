import { User, UserRole } from './models';
import { UserRepository } from './repository';
import { hashPassword, validateEmail } from './utils';

export class UserService {
    private repository: UserRepository;

    constructor(repository: UserRepository) {
        this.repository = repository;
    }

    findUser(id: number): User {
        const user = this.repository.findById(id);
        if (!user) {
            throw new Error('User not found');
        }
        return user;
    }

    createUser(name: string, role: UserRole = UserRole.User): User {
        const id = this.generateId();
        const user: User = { id, name, role, active: true, createdAt: new Date() };
        this.repository.save(user);
        return user;
    }

    deleteUser(id: number): void {
        const user = this.repository.findById(id);
        if (!user) {
            throw new Error('User not found');
        }
        this.repository.delete(id);
    }

    findByRole(role: UserRole): User[] {
        return this.repository.findAll().filter(u => u.role === role);
    }

    listAll(): User[] {
        return this.repository.findAll();
    }

    updateUser(id: number, updates: Partial<User>): User {
        const existing = this.findUser(id);
        const updated = { ...existing, ...updates };
        this.repository.save(updated);
        return updated;
    }

    private generateId(): number {
        return Math.floor(Math.random() * 100000);
    }
}
