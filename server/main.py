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
    return json({"users": users})


@app.post("/signup")
@checker.args_checker(False)
async def signup(req, arg_list):
    await database.add_user(app.ctx.db, arg_list[0], arg_list[1], arg_list[2], arg_list[3], arg_list[4], arg_list[5])
    return json({"status":"ok"})


@app.get("/find/<username>")
@checker.args_checker(False)
async def find_user(req, username):
    user = await database.find_user(app.ctx.db, username)
    return json({"user": user})


app.run(host="0.0.0.0", port=8000, debug=True)
