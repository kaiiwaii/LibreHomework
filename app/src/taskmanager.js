class Task {
    async create(name, subject, description, expires_at) {
        return await invoke("addTask", { name: name, subject: subject, description: description, expiresAt: expires_at });
    }
    async get_batch(limit, page) {
        return await invoke("getTasks", { limit: limit, page: page });
    }
    async remove(name) {
        return await invoke("removeTask", { name: name });
    }
}