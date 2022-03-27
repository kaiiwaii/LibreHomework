class Network {
    constructor() {
        this.url = "https://librehomework-api.herokuapp.com/";

        let res = fetch(this.url + "dailymessage").then((r) => {
            if (r.status == 200) {
                return [true, r.json()];
            }
            else {
                return [false, r.status];
            }
        }); //should a different function for /dailymessage exist?
    }

    async getUsers(page=0) {
        let res = await fetch(this.url + "users/" + page);
        if (res.status == 200) {
            return [true, res.json()];
        }
        else {
            return [false, res.status];
        }
    }

    async findUser(username) {
        let res = await fetch(this.url + "find/" + username);
        if (res.status == 200) {
            return [true, res.json()];
        }
        else {
            return [false, res.status];
        }
    }

    async login(username, password) {
        let res = await fetch(this.url + "login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                username: username,
                password: password
            })
        });
        if (res.status == 200) {
            return [true, res.json()];
        }
        else {
            return [false, res.status];
        }
    }

    async register(username, password, email, discord, twitter, bio) {
        let res = await fetch(this.url + "register", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                username: username,
                password: password,
                email: email,
                discord: discord,
                twitter: twitter,
                bio: bio
            })
        });
        if (res.status == 200) {
            return [true, res.json()];
        }
        else {
            return [false, res.status];
        }
    }

    async deleteAccount(token) {
        let res = await fetch(this.url + "remove", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                token: token
            })
        });
        if (res.status == 200) {
            return [true, res.json()];
        }
        else {
            return [false, res.status];
        }
    }

    //password or username cannot be changed, althought I might add a way to change it in the future
    async editProfile(token, email, discord, twitter, bio) {
        let res = await fetch(this.url + "edit", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                token: token,
                email: email,
                discord: discord,
                twitter: twitter,
                bio: bio
            })
        });
        if (res.status == 200) {
            return [true, res.json()];
        }
        else {
            return [false, res.status];
        }
    }


}