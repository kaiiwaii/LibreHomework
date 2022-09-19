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

class RateLimited(Exception):
    def __init__(self):
        super().__init__("Call was rate limited")

GLOBAL_BUCKETS = {}

def rate_limit(tokens:int,time_for_token:int):
    def wrapper(function):
        def wrapped(*args,**kwargs):
            fn_name = function.__name__
            try:
                bucket = GLOBAL_BUCKETS[fn_name]
            except:
                bucket = TokenBucket(tokens,time_for_token)
                GLOBAL_BUCKETS[fn_name] = bucket

            if not bucket.handle():
                raise RateLimited()

            return function(*args,**kwargs)
        return wrapped
    return wrapper



if __name__ == "__main__":
    @rate_limit(3,50)
    def func1(a):
        print("Function1",a)

    for i in range(10):
        try:
            a = func1(555)
        except RateLimited:
            print("RT")

        time.sleep(2)


