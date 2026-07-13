type MyTree:
  Node { ~left, ~right }
  Leaf { val }

def gen(d):
  switch d:
    case 0:
      return MyTree/Leaf { val: 1 }
    case _:
      return MyTree/Node { left: gen(d - 1), right: gen(d - 1) }

def sum(t):
  match t:
    case MyTree/Node:
      return sum(t.left) + sum(t.right)
    case MyTree/Leaf:
      return t.val

def main(depth):
  return sum(gen(depth))
