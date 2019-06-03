import random
import sys
from collections import OrderedDict


num_ru = ['ноль', 'один',
          'два', 'три',
          'четыре', 'пять', 'шесть',
          'семь', 'восемь',
          'девять', 'десять',
          'одиннадцать',
          'двенадцать',
          'тринадцать',
          'четырнадцать',
          'пятнадцать',
          'шестнадцать',
          'семнадцать',
          'восемнадцать',
          'девятнадцать',
          'двадцать']


ru_num = OrderedDict((p[1], p[0])
              for p in enumerate(num_ru))

ru_num['тридцать'] = 30
ru_num['сорок'] = 40
ru_num['пятьдесят'] = 50
ru_num['шестьдесят'] = 60
ru_num['семьдесят'] = 70
ru_num['восемьдесят'] = 80
ru_num['девяносто'] = 90
ru_num['сто'] = 100
ru_num['двести'] = 200
ru_num['триста'] = 300
ru_num['четыреста'] = 400
ru_num['пятьсот'] = 500
ru_num['шестьсот'] = 600
ru_num['семьсот'] = 700
ru_num['восемьсот'] = 800
ru_num['девятьсот'] = 900
ru_num['тысяча'] = 1000

ru_thousands = {
    'одна тысяча': 1000,
    'две тысячи': 2000,
    'три тысячи': 3000,
    'четыре тысячи': 4000,
    'пять тысяч': 5000,
    'шесть тысяч': 6000,
    'семь тысяч': 7000,
    'восемь тысяч': 8000,
    'девять тысяч': 9000,
}

ru_num.update(ru_thousands)


def parse_ru_num(s):
    parts = s.strip().split()
    if not parts:
        return None
    total = 0
    if parts[0] == 'тысяча':
        total = 1000
        parts.pop(0)
    elif (len(parts) > 1 and
             parts[1].startswith('тысяч')):
        t = parts[0] + ' ' + parts[1]
        n = ru_thousands.get(t)
        if n is None:
            return None
        total = n
        parts.pop(0)
        parts.pop(0)
    for p in parts:
        n = ru_num.get(p)
        if n is None:
            return None
        total = total + n
    return total


def get_num_ru(n):
    s = []
    if n <= 20:
        return num_ru[n]
    for (name, val) in reversed(ru_num.items()):
        if val <= n:
            s.append(name)
            n = n - val
            if n <= 20:
                break
    if n > 0:
        s.append(num_ru[n])
    return ' '.join(s)


def quiz(maxn=9999):
    m = len(num_ru)
    a = random.randint(1, maxn-1)
    b = random.randint(1, maxn-a)
    c = (a + b) % maxn
    print(get_num_ru(a), '+', get_num_ru(b))
    correct = False
    for i in range(5):
        ans = input('answer:').strip().lower()
        ans_num = parse_ru_num(ans)
        if ans_num is None:
            print('parse failed')
        elif ans_num == c:
            correct = True
            break
        else:
            print(ans_num, 'is not correct')
    if correct:
        print('Correct!', a, '+', b, '=', c)
    else:
        print('Answer:', get_num_ru(c))


def test():
    for n in range(9999):
        name = get_num_ru(n)
        n2 = parse_ru_num(name)
        if n2 != n:
            print(n, ' ', name, ' ', n2)



if __name__ == '__main__':
    test()
    n = 1
    if len(sys.argv) > 1:
        n = int(sys.argv[1])
    for i in range(n):
        quiz()

