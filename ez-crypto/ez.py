#!/usr/bin/env python3
from Crypto.Util.number import bytes_to_long, isPrime
from sympy import *
from secrets import randbelow

p = nextprime(bytes_to_long(open("msg.txt", "rb").read()))

a = randbelow(p)
b = randbelow(p)

def f(s):
    return (a * s + b) % p

print("a = ", a)
print("b = ", b)
print("f(31337) = ", f(31337))
print("f(f(31337)) = ", f(f(31337)))

N = 4892859209552999323872659687370348379127745643113821262786525515571537182220986612456469239342928177158573027631641823744589146302682629015019207630171301051593095288921587679376772619467
e = 65537
ct = 539412014186375973839103813384645095457567346931432414765860827711741737113352253186058334245660603139758935545076579254418244151719401216900449931898980356125612701132273350239082121055