def is_armstrong_number(number):
    digits = [int(x) for x in list(str(number))]
    size = len(digits)
    total = 0
    for i in digits:
        total += i**size
    return total == number
