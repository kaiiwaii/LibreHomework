from functools import wraps
from sanic.response import json



def args_checker(needs_login):
    def decorator(f):
        @wraps(f)
        async def check(request, *args, **kwargs):

            if f.__name__ == "users":
                page = request.args.get("page")
                if page:
                    page = int(page)
                else:
                    page = 0
                if page < 0:
                    return json({"error": "Page must be a positive integer"})
                
                return await f(request, *args, **kwargs)
                

            if f.__name__ == "signup":
                username = request.args.get("username")
                password = request.args.get("password")
                email = request.args.get("email")
                discord = request.args.get("discord")
                twitter = request.args.get("twitter")
                bio = request.args.get("bio")

                if not username or not password:
                    return json({"error": "Missing username or password"})

                if len(username) > 32:
                    return json({"error": "Username must be less than 32 characters"})

                if len(password) > 64:
                    return json({"error": "Password must be less than 64 characters"})

                if email and len(email) > 320:
                    return json({"error": "Email must be less than 320 characters"})

                if discord and len(discord) > 32:
                    return json({"error": "Discord must be less than 32 characters"})

                if twitter and len(twitter) > 32:
                    return json({"error": "Twitter must be less than 16 characters"})

                if bio and len(bio) > 50:
                    return json({"error": "Bio must be less than 50 characters"})
                
                return await f(request, [username, password, email, discord, twitter, bio], *args, **kwargs)

        
            if needs_login:
                username = request.args.get("username")
                password = request.args.get("password")
            
                if not username or not password:
                    return json({"error": "Missing username or password"})
                else:
                    return await f(request, *args, **kwargs)
                
            else:
                return await f(request, *args, **kwargs)
    
        return check

    return decorator