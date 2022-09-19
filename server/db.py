import utils
import authtoken
from time import mktime

async def setup_tables(db):
        async with db.acquire() as pool:
            await pool.execute("""
        CREATE TABLE IF NOT EXISTS users (
            username VARCHAR(32) PRIMARY KEY,
            password VARCHAR(64) NOT NULL,
            email VARCHAR(320), 
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            discord VARCHAR(32),
            twitter VARCHAR(16),
            bio VARCHAR(50));
            """) #note that email is public
            #create daily_message table
            await pool.execute("""
            CREATE TABLE IF NOT EXISTS daily_messages (
                id SERIAL PRIMARY KEY,
                message TEXT,
                date TIMESTAMP DEFAULT CURRENT_TIMESTAMP);
            """)
            

async def list_users(db, page):

    temp = []
    async with db.acquire() as pool:
        for row in await pool.fetch("SELECT username, email FROM users LIMIT 20 OFFSET $1", (page * 20)):
            temp.append({row[0]: utils.get_gravatar(row[1])})
        
    return temp

async def random_user(db, num):
    temp = []
    async with db.acquire() as pool:
        q = await pool.fetch("SELECT username, email, created_at, discord, twitter, bio FROM users ORDER BY random() limit $1", num)
        for row in q:
            temp.append({"username": row[0], "email": row[1], "creation_date": mktime(row[2].timetuple()),"discord": row[3], 
            "twitter": row[4], "bio": row[5], "profile_picture": utils.get_gravatar(row[1])})
    return temp

async def add_user(db, username, password, email, discord, twitter, bio):
    password = utils.hash(password.encode("utf8"))
    async with db.acquire() as pool:
        try:
            q = await pool.execute("""
            INSERT INTO users (username, password, email, discord, twitter, bio)
            VALUES ($1, $2, $3, $4, $5, $6)
            """, username, password, email, discord, twitter, bio)

            return True
        except Exception as e: #UniqueViolationError
            print(e)
            return False


async def remove_user(db, token):
    res, username = authtoken.validate_token(token)
    if not res: return False

    async with db.acquire() as pool:
        q = await pool.execute("DELETE FROM users WHERE username = $1", username)
        if q == "DELETE 0":
            return False
        else:
            return True


async def login(db, username, password):
    async with db.acquire() as pool:
        db_username = await pool.fetchrow("""
        SELECT username FROM users where password = $1 and username = $2
        """, utils.hash(password.encode("utf8")), username)

        if not db_username: return False, None

        if db_username["username"] == username:
            return True, authtoken.generate_token(username, 15) #15 minutes
        else:
            return False, None



async def find_user(db, username):
    temp = []
    async with db.acquire() as pool:
        q = await pool.fetch("""
    SELECT username, email, created_at, discord, twitter, bio FROM users WHERE username LIKE $1 LIMIT 10
    """, username)
        for row in q:
            temp.append({"username": row[0], "email": row[1], "creation_date": mktime(row[2].timetuple()),"discord": row[3], 
            "twitter": row[4], "bio": row[5], "profile_picture": utils.get_gravatar(row[1])})
    return temp


async def edit_user(db, token, email=None, discord=None, twitter=None, bio=None):
    res, username = authtoken.validate_token(token)
    if not res: return False

    query = "UPDATE users SET "
    args = []
    argc = 0

    if email:
        argc += 1
        query += f"email = ${argc}, "
        args.append(email)
    if discord:
        argc += 1
        query += f"discord = ${argc}, "
        args.append(discord)
    if twitter:
        argc += 1
        query += f"twitter = {argc}, "
        args.append(twitter)
    if bio:
        argc += 1
        query += f"bio = ${argc}, "
        args.append(bio)

    if argc == 0:
        return False

    query = query[:-2]
    query += f" WHERE username = ${argc + 1};"
    args.append(username)

    async with db.acquire() as pool:
        print(query)
        q = await pool.execute(query, *args)
        if q == "UPDATE 0":
            return False
        else:
            return True