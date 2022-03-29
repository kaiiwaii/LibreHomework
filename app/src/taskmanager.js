import { invoke } from '@tauri-apps/api';

export class Task {
    async create(name, subject, description, expires_at) {
        return await invoke("add_task", { name: name, subject: subject, description: description, expiresAt: (parseInt(expires_at) / 1000).toString() });
    }
    async get_batch(limit, page) {
        return await invoke("get_tasks", { limit: limit, page: page });
    }
    async remove(id) {
        return await invoke("remove_task", { id: id });
    }
}
