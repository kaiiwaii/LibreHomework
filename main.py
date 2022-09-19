from sanic import Sanic
from sanic.exceptions import NotFound, InvalidUsage, MethodNotSupported
from sanic.response import json, redirect
import traceback
import os, asyncpg
from utils import get_daily_message
#from ratelimiter import EndpointLimiter
import db as database
import checker

#from env import ENV
ENV = os.environ

app = Sanic("LibreHomework-Server")
app.config.FALLBACK_ERROR_FORMAT = "json"
#limiter = EndpointLimiter()

@app.listener("before_server_start")
async def setup_db(app, loop):
    pool = await asyncpg.create_pool(user=ENV["DB_USER"], password=ENV["DB_PASSWORD"],
     host=ENV["DB_HOST"], port=ENV["DB_PORT"], database=ENV["DB_NAME"], loop=loop)
    await database.setup_tables(pool)
    app.ctx.db = pool
    app.ctx.dailymessage = "Nothing yet!"

@app.get("/")
async def index(request):
    return redirect("/users/0", status=200)

@app.get("/dailymessage")
#@limiter.limit(5, 10)
async def dailymessage(request):
    return json({"data": app.ctx.dailymessage}, status=200)

@app.exception(Exception)
async def catch_anything(request, exception):
    print(traceback.print_exc())
    if isinstance(exception, NotFound):
        return json({"error": "Not Found"}, status=404)
    elif isinstance(exception, InvalidUsage):
        return json({"error": "Invalid Usage"}, status=400)
    elif isinstance(exception, MethodNotSupported):
        return json({"error": "Method Not Supported"}, status=405)
    else:
        return json({"error": "Internal Server Error"}, status=500)


@app.get("/users/<page>")
#@limiter.limit(5, 8)
@checker.args_checker(False)
async def users(req, page):
    try:
        page = int(page)
    except ValueError:
        return json({"error": "Page must be a positive integer"})
    users = await database.list_users(app.ctx.db, page)
    return json({"data": users}, status=200)


@app.post("/login")
#@limiter.limit(2, 5)
@checker.args_checker(False)
async def login(req, username, password):
    if username and password:
        res, tk = await database.login(app.ctx.db, username, password)
        if res:
            return json({"data": tk.decode("utf8")}, status=200)
        else:
            return json({"error": "Invalid username or password"}, status=400)
    else:
        return json({"error": "Missing username or password"}, status=400)


@app.post("/signup")
#@limiter.limit(10, 3600)
@checker.args_checker(False)
async def signup(req, arg_list):
    res = await database.add_user(app.ctx.db, arg_list[0], arg_list[1], arg_list[2], arg_list[3], arg_list[4], arg_list[5])
    if not res:
        return json({"error":"Error creating user in the database. Maybe the username is already taken?"}, status=400)
    else:
        return json({"data": True}, status=200)


@app.post("/remove")
#@limiter.limit(10, 3600)
@checker.args_checker(True)
async def remove_user(req, token):
    res = await database.remove_user(app.ctx.db, token.encode("utf8"))
    if not res:
        return json({"error": "Error removing user from database. Is the token correct?"}, status=401)
    else:
        return json({"data": True})


@app.get("/find/<username>")
#@limiter.limit(10, 13)
@checker.args_checker(False)
async def find_users(req, username):
    users = await database.find_user(app.ctx.db, username)
    return json({"data": users}, status=200)

@app.get("/random")
@limiter.limit(10,20)
async def random_user(req):
    try:
        num = int(req.args["max"])
        if num <= 5:
            users = await database.random_user(app.ctx.db, num)
        else:
            users = await database.random_user(app.ctx.db, 1)
    except: #no arg or invalid int
        users = await database.random_user(app.ctx.db, 1)
    
    return json(users)

@app.post("/edit")
#@limiter.limit(15, 1800)
@checker.args_checker(True)
async def edit_user(req, token):

    email = req.form.get("email")
    discord = req.form.get("discord")
    twitter = req.form.get("twitter")
    bio = req.form.get("bio")

    res = await database.edit_user(app.ctx.db, token.encode("utf8"), email, discord, twitter, bio)

    if not res:
        return json({"error": "Error editing profile. If username and token are correct please open an issue"}, status=401)
    else:
        return json({"data": True})


app.add_task(get_daily_message(app))

app.run(host="0.0.0.0", port=int(os.environ["PORT"]), debug=False)
