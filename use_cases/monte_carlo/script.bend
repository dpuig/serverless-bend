def lcg(seed):
  return (seed * 11035) + 12345

def map_to_range(rand):
  return (rand % 2000) - 1000

def is_in_circle(i):
  seed1 = lcg(i)
  seed2 = lcg(seed1)
  x = map_to_range(seed1)
  y = map_to_range(seed2)
  if (x * x) + (y * y) <= 1000000:
    return 1
  else:
    return 0

def monte_carlo(start, end):
  if start == end - 1:
    return is_in_circle(start)
  else:
    mid = (start + end) / 2
    return monte_carlo(start, mid) + monte_carlo(mid, end)

def main(n):
  return monte_carlo(0, n)
