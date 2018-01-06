class Node
  attr_reader :val, :children

  def initialize(val, children: [])
    @val = val
    @children = children
  end

  def to_s
    @val.to_s
  end
end


x = Node.new("X")
w = Node.new("W")
y = Node.new("Y", children: [w])
z = Node.new("Z")
b = Node.new("B", children: [x, y, z])
m = Node.new("M")
c = Node.new("C", children: [m])
k = Node.new("K")
l = Node.new("L")
d = Node.new("D", children: [k, l])
a = Node.new("A", children: [b, c, d])


def recursive_traverse(root)
  root.children.each do |node|
    recursive_traverse(node)
  end
  print "#{root} "
end


def stack_traverse(root)
  stack = []
  current = root

  loop do
    while current do
      iter = current.children.each
      stack.push([current, iter])
      current = get_next(iter)
    end

    item, _ = stack.pop
    print "#{item} "

    pair = stack.pop
    if pair
      el, iter = pair
      current = get_next(iter)
      stack.push([el, iter])
    else
      return
    end
  end
end

def get_next(iter)
  iter.next
rescue StopIteration
  nil
end



def stack_traverse2(root)
  return if root.nil?

  stack = []
  current = root

  stack.push(current)

  while !stack.empty? do
    top = stack.pop()
    top.children.each do |child|
      stack.push(child)
    end
    print "#{top} "
  end
end

class Item
  attr_reader :node, :iter

  def initialize(node)
    @node = node
    @iter = node.children.each
  end

  def to_s
    node.to_s
  end

  def next
    Item.new(iter.next)
  rescue StopIteration
    nil
  end
end

def stack_traverse3(root)
  stack = []
  current = Item.new(root)

  loop do
    while current && new_cur = current.next do
      stack.push(current)
      current = new_cur
    end

    process(current)
    current = stack.pop

    return unless current
  end
end

def process(node)
  print "#{node} "
end


class Traversal
  def initialize(root)
    @current = Item.new(root)
    @stack = []
  end

  def next
    while @current && new_cur = @current.next do
      @stack.push(@current)
      @current = new_cur
    end

    result = @current
    @current = @stack.pop
    result
  end
end


puts "recursive_traverse"
recursive_traverse(a)
puts "\n\n"


puts "stack_traverse"
stack_traverse(a)
puts "\n\n"


puts "stack_traverse2"
stack_traverse2(a)
puts "\n\n"


puts "stack_traverse3"
stack_traverse3(a)
puts "\n\n"


puts "Traversal"
iter = Traversal.new(a)
while item = iter.next do
  process(item)
end
puts "\nDONE"
puts "\n\n"
