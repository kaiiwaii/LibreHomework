from functools import wraps
import time
from sanic.response import json

class RateLimiter:
    def __init__(self):
        self.storage = {}

    
    def limit(self, calls, per_second):
        def decorator(func):
            @wraps(func)
            async def wrapper(request, *args, **kwargs):
                current_time = time.time()
                cell = self.storage.get(request.ip)

                if not cell:
                    cell = [calls-1, current_time]
                    self.storage[request.ip] = cell
                    return await func(request,*args,**kwargs)

                time_delta = current_time - cell[-1]
                to_add = int(time_delta*(calls/per_second)) # calls/per_second should be hardcoded
                cell[0] += to_add

                if cell[0] > calls:
                	cell[0] = calls
                    
                if cell[0] == 0:
                    return json({"ratelimit": True})

                self.storage[request.ip][0] -= 1
                self.storage[request.ip][1] = current_time
                return await func(request, *args, **kwargs)
            return wrapper
        return decorator

class EndpointLimiter:
    def __init__(self):
        self.users_limiter = RateLimiter()
        self.login_limiter = RateLimiter()
        self.signup_limiter = RateLimiter()
        self.remove_limiter = RateLimiter()
        self.find_limiter = RateLimiter()
        self.edit_limiter = RateLimiter()
        self.daily_message_limiter = RateLimiter()

        self.funcs = {"users": self.users_limiter,
                      "login": self.login_limiter,
                      "signup": self.signup_limiter,
                      "remove_user": self.remove_limiter,
                      "find_users": self.find_limiter,
                      "edit_user": self.edit_limiter,
                      "dailymessage": self.daily_message_limiter}


    def limit(self, calls, per_second):
        def decorator(func):
            @wraps(func)
            async def wrapper(request, *args, **kwargs):
                try:
                    return await self.funcs[func.__name__](calls, per_second)(func)(request, *args, **kwargs)
                except KeyError:
                    return await func(request, *args, **kwargs)
            return wrapper
        return decorator