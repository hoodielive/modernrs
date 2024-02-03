import time

def greeting(seconds, func):
    time.sleep(seconds)
    return func()
