class Node
  attr_reader :val, :lnode, :rnode

  def initialize(val, lnode: nil, rnode: nil)
    @val = val
    @lnode = lnode
    @rnode = rnode
  end

  def to_s
    @val.to_s
  end

  #def to_s
    #if lnode.nil? && rnode.nil?
      #val.to_s
    #else
      #out = ""
      #l = lnode.to_s
      #r = rnode.to_s
    #end
  #end
end

x = Node.new("X")
y = Node.new("Y")
g = Node.new("G", lnode: x, rnode: y)
f = Node.new("F", rnode: g)
e = Node.new("E")
c = Node.new("C", lnode: e, rnode: f)
d = Node.new("D")
b = Node.new("B", lnode: d)
a = Node.new("A", lnode: b, rnode: c)


class InorderTraversal
  def initialize(root)
    @root = root
    @stack = []
    @current = @root
  end

  def next
    loop do
      while @current do
        @stack.push(@current)
        @current = @current.lnode
      end

      if @current.nil? && !@stack.empty?
        item = @stack.pop
        @current = item.rnode
        return item
      elsif @current.nil? && @stack.empty?
        return nil
      else
        raise("Unreachable!")
      end
    end
  end
end

def traverse(root)
  stack = []
  current = root

  loop do
    while current do
      stack.push(current)
      current = current.lnode
    end

    if current.nil? && !stack.empty?
      # 4
      item = stack.pop
      puts item.val
      current = item.rnode
    elsif current.nil? && stack.empty?
      # 5
      puts "DONE"
      return
    else
      raise("Unreachable!")
    end
  end
end


t = InorderTraversal.new(a)

13.times do
  puts t.next
end
