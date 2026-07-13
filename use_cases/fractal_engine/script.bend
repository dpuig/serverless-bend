type MyList:
  Nil
  Cons { head, ~tail }

def bitwise_and_is_zero(x, y):
  switch x:
    case 0:
      return 1
    case _:
      switch y:
        case 0:
          return 1
        case _:
          if (x % 2 != 0):
            if (y % 2 != 0):
              return 0
            else:
              return bitwise_and_is_zero(x / 2, y / 2)
          else:
            return bitwise_and_is_zero(x / 2, y / 2)

def render_row(y, x, max_x):
  if x == max_x:
    return MyList/Nil
  else:
    return MyList/Cons { head: bitwise_and_is_zero(x, y), tail: render_row(y, x + 1, max_x) }

def render_grid(y, max_y, max_x):
  if y == max_y:
    return MyList/Nil
  else:
    return MyList/Cons { head: render_row(y, 0, max_x), tail: render_grid(y + 1, max_y, max_x) }

def main(size):
  return render_grid(0, size, size)
