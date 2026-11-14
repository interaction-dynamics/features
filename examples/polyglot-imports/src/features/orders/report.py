import os
from collections import OrderedDict


def report():
    return OrderedDict(path=os.getcwd())
