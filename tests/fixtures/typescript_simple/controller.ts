import { UserService } from './service';
import { UserRepository } from './repository';
import { User, CreateUserRequest, UserRole, UserDTO } from './models';

export class UserController {
    private service: UserService;
    private repository: UserRepository;

    constructor() {
        this.repository = new UserRepository();
        this.service = new UserService(this.repository);
    }

    async handleGetUser(id: number): Promise<User> {
        return this.service.findUser(id);
    }

    async handleCreateUser(request: CreateUserRequest): Promise<User> {
        this.validateRequest(request);
        return this.service.createUser(request.name, request.role);
    }

    async handleDeleteUser(id: number): Promise<void> {
        const user = await this.service.findUser(id);
        if (!user) {
            throw new Error('User not found');
        }
        await this.service.deleteUser(id);
    }

    async handleListUsers(role?: UserRole): Promise<User[]> {
        if (role) {
            return this.service.findByRole(role);
        }
        return this.service.listAll();
    }

    private validateRequest(request: CreateUserRequest): void {
        if (!request.name || request.name.trim().length === 0) {
            throw new Error('Name is required');
        }
        if (request.name.length > 100) {
            throw new Error('Name too long');
        }
    }
}
