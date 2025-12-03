import check50
import check50.c


@check50.check()
def exists():
    """bulbs.c exists"""
    check50.exists("bulbs.c")


@check50.check(exists)
def compiles():
    """bulbs.c compiles"""
    check50.run("make bulbs")


@check50.check(compiles)
def bulbs_empty():
    """bulbifies an empty message correctly"""
    check50.run("./bulbs").stdin("").stdout("").exit(0)
    

@check50.check(compiles)
def bulbs_single_letter():
    """bulbifies \"I\" correctly"""
    check50.run("./bulbs").stdin("I").stdout("⚫🟡⚫⚫🟡⚫⚫🟡\n").exit(0)
    

@check50.check(compiles)
def bulbs_multiple_letters():
    """bulbifies \"xyz\" correctly"""
    check50.run("./bulbs").stdin("xyz").stdout("⚫🟡🟡🟡🟡⚫⚫⚫\n⚫🟡🟡🟡🟡⚫⚫🟡\n⚫🟡🟡🟡🟡⚫🟡⚫\n").exit(0)


@check50.check(compiles)
def bulbs_non_alpha():
    """bulbifies \"?\" correctly"""
    check50.run("./bulbs").stdin("?").stdout("⚫⚫🟡🟡🟡🟡🟡🟡\n").exit(0)


@check50.check(compiles)
def bulbs_message():
    """bulbifies \"Hi!\" correctly"""
    check50.run("./bulbs").stdin("Hi!").stdout("⚫🟡⚫⚫🟡⚫⚫⚫\n⚫🟡🟡⚫🟡⚫⚫🟡\n⚫⚫🟡⚫⚫⚫⚫🟡\n").exit(0)
    

@check50.check(compiles)
def bulbs_mixed_case_alpha():
    """bulbifies \"aBcDeFgHiJkLmNoPqRsTuVwXyZ\" correctly"""
    check50.run("./bulbs").stdin("aBcDeFgHiJkLmNoPqRsTuVwXyZ").stdout("⚫🟡🟡⚫⚫⚫⚫🟡\n⚫🟡⚫⚫⚫⚫🟡⚫\n⚫🟡🟡⚫⚫⚫🟡🟡\n⚫🟡⚫⚫⚫🟡⚫⚫\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫🟡⚫⚫⚫🟡🟡⚫\n⚫🟡🟡⚫⚫🟡🟡🟡\n⚫🟡⚫⚫🟡⚫⚫⚫\n⚫🟡🟡⚫🟡⚫⚫🟡\n⚫🟡⚫⚫🟡⚫🟡⚫\n⚫🟡🟡⚫🟡⚫🟡🟡\n⚫🟡⚫⚫🟡🟡⚫⚫\n⚫🟡🟡⚫🟡🟡⚫🟡\n⚫🟡⚫⚫🟡🟡🟡⚫\n⚫🟡🟡⚫🟡🟡🟡🟡\n⚫🟡⚫🟡⚫⚫⚫⚫\n⚫🟡🟡🟡⚫⚫⚫🟡\n⚫🟡⚫🟡⚫⚫🟡⚫\n⚫🟡🟡🟡⚫⚫🟡🟡\n⚫🟡⚫🟡⚫🟡⚫⚫\n⚫🟡🟡🟡⚫🟡⚫🟡\n⚫🟡⚫🟡⚫🟡🟡⚫\n⚫🟡🟡🟡⚫🟡🟡🟡\n⚫🟡⚫🟡🟡⚫⚫⚫\n⚫🟡🟡🟡🟡⚫⚫🟡\n⚫🟡⚫🟡🟡⚫🟡⚫\n").exit(0)


@check50.check(compiles)
def bulbs_spaces():
    """bulbifies \" CS50 :) \" correctly"""
    check50.run("./bulbs").stdin(" CS50 :) ").stdout("⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡⚫⚫⚫⚫🟡🟡\n⚫🟡⚫🟡⚫⚫🟡🟡\n⚫⚫🟡🟡⚫🟡⚫🟡\n⚫⚫🟡🟡⚫⚫⚫⚫\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫⚫🟡🟡🟡⚫🟡⚫\n⚫⚫🟡⚫🟡⚫⚫🟡\n⚫⚫🟡⚫⚫⚫⚫⚫\n").exit(0)


@check50.check(compiles)
def bulbs_finale():
    """bulbifies The Great Gatsby's first sentence correctly"""
    check50.run("./bulbs").stdin("In my younger and more vulnerable years my father gave me some advice that I've been turning over in my mind ever since.").stdout("⚫🟡⚫⚫🟡⚫⚫🟡\n⚫🟡🟡⚫🟡🟡🟡⚫\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡⚫🟡🟡⚫🟡\n⚫🟡🟡🟡🟡⚫⚫🟡\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡🟡🟡⚫⚫🟡\n⚫🟡🟡⚫🟡🟡🟡🟡\n⚫🟡🟡🟡⚫🟡⚫🟡\n⚫🟡🟡⚫🟡🟡🟡⚫\n⚫🟡🟡⚫⚫🟡🟡🟡\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫🟡🟡🟡⚫⚫🟡⚫\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡⚫⚫⚫⚫🟡\n⚫🟡🟡⚫🟡🟡🟡⚫\n⚫🟡🟡⚫⚫🟡⚫⚫\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡⚫🟡🟡⚫🟡\n⚫🟡🟡⚫🟡🟡🟡🟡\n⚫🟡🟡🟡⚫⚫🟡⚫\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡🟡⚫🟡🟡⚫\n⚫🟡🟡🟡⚫🟡⚫🟡\n⚫🟡🟡⚫🟡🟡⚫⚫\n⚫🟡🟡⚫🟡🟡🟡⚫\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫🟡🟡🟡⚫⚫🟡⚫\n⚫🟡🟡⚫⚫⚫⚫🟡\n⚫🟡🟡⚫⚫⚫🟡⚫\n⚫🟡🟡⚫🟡🟡⚫⚫\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡🟡🟡⚫⚫🟡\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫🟡🟡⚫⚫⚫⚫🟡\n⚫🟡🟡🟡⚫⚫🟡⚫\n⚫🟡🟡🟡⚫⚫🟡🟡\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡⚫🟡🟡⚫🟡\n⚫🟡🟡🟡🟡⚫⚫🟡\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡⚫⚫🟡🟡⚫\n⚫🟡🟡⚫⚫⚫⚫🟡\n⚫🟡🟡🟡⚫🟡⚫⚫\n⚫🟡🟡⚫🟡⚫⚫⚫\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫🟡🟡🟡⚫⚫🟡⚫\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡⚫⚫🟡🟡🟡\n⚫🟡🟡⚫⚫⚫⚫🟡\n⚫🟡🟡🟡⚫🟡🟡⚫\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡⚫🟡🟡⚫🟡\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡🟡⚫⚫🟡🟡\n⚫🟡🟡⚫🟡🟡🟡🟡\n⚫🟡🟡⚫🟡🟡⚫🟡\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡⚫⚫⚫⚫🟡\n⚫🟡🟡⚫⚫🟡⚫⚫\n⚫🟡🟡🟡⚫🟡🟡⚫\n⚫🟡🟡⚫🟡⚫⚫🟡\n⚫🟡🟡⚫⚫⚫🟡🟡\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡🟡⚫🟡⚫⚫\n⚫🟡🟡⚫🟡⚫⚫⚫\n⚫🟡🟡⚫⚫⚫⚫🟡\n⚫🟡🟡🟡⚫🟡⚫⚫\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡⚫⚫🟡⚫⚫🟡\n⚫⚫🟡⚫⚫🟡🟡🟡\n⚫🟡🟡🟡⚫🟡🟡⚫\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡⚫⚫⚫🟡⚫\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫🟡🟡⚫🟡🟡🟡⚫\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡🟡⚫🟡⚫⚫\n⚫🟡🟡🟡⚫🟡⚫🟡\n⚫🟡🟡🟡⚫⚫🟡⚫\n⚫🟡🟡⚫🟡🟡🟡⚫\n⚫🟡🟡⚫🟡⚫⚫🟡\n⚫🟡🟡⚫🟡🟡🟡⚫\n⚫🟡🟡⚫⚫🟡🟡🟡\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡⚫🟡🟡🟡🟡\n⚫🟡🟡🟡⚫🟡🟡⚫\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫🟡🟡🟡⚫⚫🟡⚫\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡⚫🟡⚫⚫🟡\n⚫🟡🟡⚫🟡🟡🟡⚫\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡⚫🟡🟡⚫🟡\n⚫🟡🟡🟡🟡⚫⚫🟡\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡⚫🟡🟡⚫🟡\n⚫🟡🟡⚫🟡⚫⚫🟡\n⚫🟡🟡⚫🟡🟡🟡⚫\n⚫🟡🟡⚫⚫🟡⚫⚫\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫🟡🟡🟡⚫🟡🟡⚫\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫🟡🟡🟡⚫⚫🟡⚫\n⚫⚫🟡⚫⚫⚫⚫⚫\n⚫🟡🟡🟡⚫⚫🟡🟡\n⚫🟡🟡⚫🟡⚫⚫🟡\n⚫🟡🟡⚫🟡🟡🟡⚫\n⚫🟡🟡⚫⚫⚫🟡🟡\n⚫🟡🟡⚫⚫🟡⚫🟡\n⚫⚫🟡⚫🟡🟡🟡⚫\n").exit(0)