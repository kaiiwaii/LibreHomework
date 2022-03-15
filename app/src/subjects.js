export class Subject {
    async create(name) {
        return await invoke("addSubject", { name: name });
    }
    async get_batch() {
        return await invoke("getSubjects"});
    }
    /*async remove(name) {
        return await invoke("removeTask", { name: name });
    }*/
}

