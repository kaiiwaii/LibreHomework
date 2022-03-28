import { invoke } from "@tauri-apps/api";


class ApiResponse {
    constructor(payload) {
        this.status = payload[1];
        this.data = data[0];
    }
}


class ServerAPI {
    
    async getDailyMessage() {
        return new ApiResponse(await invoke("request", {"url": "dailymessage"}));
    }

    async getUsers(page=0) {
        return new ApiResponse(await invoke("request", {"url": "users/" + page}));

    }

    async findUser(username) {
        return new ApiResponse(await invoke("request", {"url": "find/" + username}));
    }

    async login(username, password) {
        return new ApiResponse(await invoke("request", {"url": "login", "method": "POST", "body": JSON.stringify({
            "username": username,
            "password": password
        })}));

    }

    async signup(username, password, email=null, discord=null, twitter=null, bio=null) {
        return new ApiResponse(await invoke("request", {"url": "signup", "method": "POST", "body": JSON.stringify({
            "username": username,
            "password": password,
            "email": email,
            "discord": discord,
            "twitter": twitter,
            "bio": bio
        })}));
    }

    async deleteAccount(token) {
        return new ApiResponse(await invoke("request", {"url": "remove", "method": "POST", "body": JSON.stringify({
            "token": token
        })}));
    }

    //password or username cannot be changed, althought I might add a way to change it in the future
    //server should handle the optional fields but check if at least one param is not null
    async editProfile(token, email=null, discord=null, twitter=null, bio=null) {
        return new ApiResponse(await invoke("request", {"url": "remove", "method": "POST", "body": JSON.stringify({
            "token": token
        })}));
    }


}

export default ServerAPI;