from sanic import Sanic
from sanic.response import json
import aiosqlite

import db as database
import checker

app = Sanic("LibreHomework-Server")

@app.listener("before_server_start")
async def setup_db(app, loop):
    temp_db = await aiosqlite.connect("librehomework.db")
    await database.setup_tables(temp_db)
    app.ctx.db = temp_db


@app.get("/users/<page>")
@checker.args_checker(False)
async def users(req, page):
    users = await database.list_users(app.ctx.db, int(page))
    return json(users)


@app.post("/login")
@checker.args_checker(False)
async def login(req, username, password):
    if username and password:
        res, tk = database.login(app.ctx.db, username, password)
        if res:
            return json({"auth-token": tk})
        else:
            return json({"error": "Invalid username or password"})
    else:
        return json({"error": "Missing username or password"})


@app.post("/signup")
@checker.args_checker(False)
async def signup(req, arg_list):
    res = await database.add_user(app.ctx.db, arg_list[0], arg_list[1], arg_list[2], arg_list[3], arg_list[4], arg_list[5])
    if not res:
        return json({"error":"Error creating user in the database. If the error persists please open an issue"})
    else:
        return json({"status": 200})


@app.post("/remove")
@checker.args_checker(True)
async def remove_user(req, token):
    res = await database.remove_user(app.ctx.db, req.form.get("username"), token)
    if not error:
        return json({"error": "Error removing user from database. Is the token correct?"})
    else:
        return json({"status": 200})


@app.get("/find/<username>")
@checker.args_checker(False)
async def find_users(req, username):
    users = await database.find_user(app.ctx.db, username)
    return json(users)


@app.post("/edit")
@checker.args_checker(True)
async def edit_profile(req, token):
    username = req.form.get("username")
    email = req.form.get("email")
    discord = req.form.get("discord")
    twitter = req.form.get("twitter")
    bio = req.form.get("bio")
    res = await database.edit_profile(app.ctx.db, username, token, email, discord, twitter, bio)

    if not res:
        return json({"error": "Error editing profile. If username and token are correct please open an issue"})
    else:
        return json({"status": 200})


app.run(host="0.0.0.0", port=8000)#, debug=True)
