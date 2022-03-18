import { invoke } from '@tauri-apps/api';

export class Subject {
    async create(name) {
        return await invoke("add_subject", { name: name });
    }
    async get_batch() {
        return await invoke("get_subjects");
    }
    async remove(name) {
        return await invoke("remove_task", { name: name });
    }
}

