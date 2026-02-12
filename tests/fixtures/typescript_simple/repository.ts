import { User, UserFilter } from './models';

export class UserRepository {
    private users: Map<number, User> = new Map();

    findById(id: number): User | undefined {
        return this.users.get(id);
    }

    findAll(): User[] {
        return Array.from(this.users.values());
    }

    save(user: User): void {
        this.users.set(user.id, user);
    }

    delete(id: number): boolean {
        return this.users.delete(id);
    }

    findByFilter(filter: UserFilter): User[] {
        let results = this.findAll();

        if (filter.role) {
            results = results.filter(u => u.role === filter.role);
        }

        if (filter.active !== undefined) {
            results = results.filter(u => u.active === filter.active);
        }

        if (filter.search) {
            const search = filter.search.toLowerCase();
            results = results.filter(u =>
                u.name.toLowerCase().includes(search)
            );
        }

        return results;
    }

    count(): number {
        return this.users.size;
    }

    exists(id: number): boolean {
        return this.users.has(id);
    }
}
