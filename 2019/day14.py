import utils
import re
from copy import deepcopy

def parse_recipe(recipe):
    ingredients = re.findall(r'(\d+ [A-Z]+)', recipe)
    amount, output = ingredients[-1].split()
    amount = int(amount)
    ingredients = {ingredient:int(amount) for amount, ingredient in [i.split() for i in ingredients[:-1]]}
    ingredients.update({'amount':amount, 'required':1, 'owner':output})
    return {output:ingredients}

def get_recipe_book(recipes):
    book = {}
    for recipe in recipes:
        book.update(parse_recipe(recipe))
    return book

def is_raw(recipe):
    return 'ORE' in recipe


def amount_needed(needed, multiple):
    remainder = (needed % multiple)
    required = needed//multiple + (1 if remainder != 0 else 0)
    return required, (multiple - remainder) if remainder else 0

def first_star_answer(recipes):
    needed = [recipes['FUEL']]
    total_ore = 0
    raws = [recipes[r] for r in recipes if is_raw(recipes[r])]
    while needed:
        new_needed = {}
        temp_needed = {}
        node = needed.pop()
        amt = node['amount']
        req = node['required']
        for k, v in node.items():
            if k == 'amount':
                continue
            if k == 'required':
                continue
            if k == 'ORE':
                continue
            try:
                rec = recipes[k]
            except:
                continue
            required, remainder = amount_needed(v, rec['amount'] )#if not rec['required'] else rec['required'])
            rec['required'] += required * req
            if is_raw(rec):
                print(node,k, v, required, req)
                print(raws)
                pass
            else:
                needed.append(rec)
    return raws

def get_needed(mineral, amount, recipes):
    rec = recipes[mineral]
    children = [k for k in rec if k in recipes]
    req = {}
    for child in children:
        c = recipes[child]
        amount_required = amount_needed(amount*rec[c['owner']], c['amount'])
        req.update({c['owner']:amount_required + (amount_required[0]*c['amount'],)})
    return req
