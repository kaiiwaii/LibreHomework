import { invoke } from '@tauri-apps/api';

export class Task {
    async create(name, subject, description, expires_at) {
        return await invoke("add_task", { name: name, subject: subject, description: description, expiresAt: expires_at });
    }
    async get_batch(limit, page) {
        return await invoke("get_tasks", { limit: limit, page: page });
    }
    async remove(name) {
        return await invoke("remove_task", { name: name });
    }
}
