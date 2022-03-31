import time

class TokenBucket:
    '''
    Implementation of TokenBucket
    '''
    def __init__(self,tokens:int,time_for_token:int):
        '''
        tokens:int - maximum amount of tokens
        time_for_token: - time in which 1 token is added  
        '''
        self.token_koef = tokens/time_for_token
        self.tokens = tokens
        self.last_check = time.time()

    def handle(self) -> bool:
        current_time = time.time()
        time_delta = current_time - self.last_check
        self.last_check = current_time

        self.tokens += time_delta*self.token_koef

        if self.tokens > self.max_tokens:
            self.tokens = self.max_tokens
        
        if self.tokens < 1:
            return False
        
        self.tokens -= 1
        return True


GLOBAL_BUCKETS = {}

def limit(tokens:int,time_for_token:int):
    def wrapper(function):
        def wrapped(*args,**kwargs):
            fn_name = function.__name__
            try:
                bucket = GLOBAL_BUCKETS[fn_name]
            except:
                bucket = TokenBucket(tokens,time_for_token)
                GLOBAL_BUCKETS[fn_name] = bucket

            if not bucket.handle():
                return json({"success": False, "ratelimit": True})

            return function(*args,**kwargs)
        return wrapped
    return wrapper

class EndpointLimiter:
    def __init__(self):
        self.funcs = {}


    def limit(self, calls, per_second):
        def decorator(func):
            @wraps(func)
            async def wrapper(request, *args, **kwargs):
                try:
                    return await self.funcs[func.__name__].limit(calls, per_second, func, request, *args, **kwargs)
                except KeyError:
                    rate_limiter = RateLimiter()
                    self.funcs[func.__name__] = rate_limiter
                    return await self.funcs[func.__name__].limit(calls, per_second, func, request, *args, **kwargs)
            return wrapper
        return decorator
