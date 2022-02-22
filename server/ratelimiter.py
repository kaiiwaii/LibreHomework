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

    def limit(self, calls, per_second):
        def decorator(func):
            @wraps(func)
            async def wrapper(request, *args, **kwargs):
                if request.path == "/users":
                    return await self.users_limiter.limit(calls, per_second)(func)(request, *args, **kwargs)
                elif request.path == "/login":
                    return await self.login_limiter.limit(calls, per_second)(func)(request, *args, **kwargs)
                elif request.path == "/signup":
                    return await self.signup_limiter.limit(calls, per_second)(func)(request, *args, **kwargs)
                elif request.path == "/remove":
                    return await self.remove_limiter.limit(calls, per_second)(func)(request, *args, **kwargs)
                elif request.path == "/find":
                    return await self.find_limiter.limit(calls, per_second)(func)(request, *args, **kwargs)
                elif request.path == "/edit":
                    return await self.edit_limiter.limit(calls, per_second)(func)(request, *args, **kwargs)
                else:
                    return await func(request, *args, **kwargs)
            return wrapper
        return decorator