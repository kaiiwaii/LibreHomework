from sanic import Sanic
from sanic.exceptions import NotFound, InvalidUsage, MethodNotSupported
from sanic.response import json, redirect
import aiosqlite

from ratelimiter import EndpointLimiter
import db as database
import checker

app = Sanic("LibreHomework-Server")
app.config.FALLBACK_ERROR_FORMAT = "json"
limiter = EndpointLimiter()

@app.listener("before_server_start")
async def setup_db(app, loop):
    temp_db = await aiosqlite.connect("librehomework.db")
    await database.setup_tables(temp_db)
    app.ctx.db = temp_db

@app.get("/")
async def index(request):
    return redirect("/users/0")

@app.exception(Exception)
async def catch_anything(request, exception):
    if isinstance(exception, NotFound):
        return json({"error": "Not Found"}, status=404)
    elif isinstance(exception, InvalidUsage):
        return json({"error": "Invalid Usage"}, status=400)
    elif isinstance(exception, MethodNotSupported):
        return json({"error": "Method Not Supported"}, status=405)
    else:
        return json({"error": "Internal Server Error"}, status=500)


@app.get("/users/<page>")
@limiter.limit(5, 8)
@checker.args_checker(False)
async def users(req, page):
    users = await database.list_users(app.ctx.db, int(page))
    return json(users)


@app.post("/login")
@limiter.limit(2, 5)
@checker.args_checker(False)
async def login(req, username, password):
    if username and password:
        res, tk = await database.login(app.ctx.db, username, password)
        if res:
            return json({"auth-token": tk.decode("utf8")})
        else:
            return json({"error": "Invalid username or password"})
    else:
        return json({"error": "Missing username or password"})


@app.post("/signup")
@limiter.limit(10, 3600)
@checker.args_checker(False)
async def signup(req, arg_list):
    res = await database.add_user(app.ctx.db, arg_list[0], arg_list[1], arg_list[2], arg_list[3], arg_list[4], arg_list[5])
    if not res:
        return json({"error":"Error creating user in the database. Maybe the username is already taken?"})
    else:
        return json({"status": 200})


@app.post("/remove")
@limiter.limit(10, 3600)
@checker.args_checker(True)
async def remove_user(req, token):
    res = await database.remove_user(app.ctx.db, token.encode("utf8"))
    if not res:
        return json({"error": "Error removing user from database. Is the token correct?"})
    else:
        return json({"status": 200})


@app.get("/find/<username>")
@limiter.limit(10, 13)
@checker.args_checker(False)
async def find_users(req, username):
    users = await database.find_user(app.ctx.db, username)
    return json(users)


@app.post("/edit")
@limiter.limit(15, 1800)
@checker.args_checker(True)
async def edit_profile(req, token):

    email = req.form.get("email")
    discord = req.form.get("discord")
    twitter = req.form.get("twitter")
    bio = req.form.get("bio")

    res = await database.edit_user(app.ctx.db, token.encode("utf8"), email, discord, twitter, bio)

    if not res:
        return json({"error": "Error editing profile. If username and token are correct please open an issue"})
    else:
        return json({"status": 200})


app.run(host="0.0.0.0", port=8000, debug=True)
