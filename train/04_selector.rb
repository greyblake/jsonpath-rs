class BaseNode
  attr_reader :id

  def to_s
    @id
  end
end

class NodeStr < BaseNode
  def initialize(id)
    raise "Id must be a string" unless id.is_a?(String)
    @id = id
  end
end

class NodeArr < BaseNode
  attr_reader :children

  def initialize(id, children)
    raise "Id must be a string" unless id.is_a?(String)
    raise "Children of NodeArr must be a Array" unless children.is_a?(Array)
    @id = id
    @children = children
  end
end

class NodeObj < BaseNode
  attr_reader :children

  def initialize(id, children)
    raise "Id must be a string" unless id.is_a?(String)
    raise "Children of NodeObj must be a Hash" unless children.is_a?(Hash)
    @id = id
    @children = children
  end
end

type_cat = NodeStr.new("cat")
name_tom = NodeStr.new("Tom")
tom = NodeObj.new("Tom(obj)", {"type" => type_cat, "name" => name_tom })

type_dog = NodeStr.new("dog")
name_rex = NodeStr.new("Rex")
rex = NodeObj.new("Rex(obj)", {"type" => type_dog, "name" => name_rex })

pets = NodeArr.new("pets", [tom, rex])

user_age = NodeStr.new("27")
user_name = NodeStr.new("Sergey")
user = NodeObj.new("user", {"name" => user_name, "age" => user_age } )

root = NodeObj.new("root", {"pets" => pets, "user" => user})


class ItemBase
  attr_reader :node

  def to_s
    node.to_s
  end
end

class ItemStr < ItemBase
  def initialize(node)
    raise "Node for ItemStr must be NodeStr" unless node.is_a?(NodeStr)
    @node = node
  end

  def next
    nil
  end
end

class ItemArr < ItemBase
  def initialize(node)
    raise "Node for ItemArr must be NodeArr" unless node.is_a?(NodeArr)
    @node = node
    @iter = node.children.each
  end

  def next
    Item.new(@iter.next)
  rescue StopIteration
    nil
  end
end

class ItemObj < ItemBase
  def initialize(node)
    raise "Node for ItemObj must be NodeObj" unless node.is_a?(NodeObj)
    @node = node
    @iter = node.children.values.each
  end

  def next
    Item.new(@iter.next)
  rescue StopIteration
    nil
  end
end

module Item
  def self.new(node)
    case node
    when NodeStr then ItemStr.new(node)
    when NodeArr then ItemArr.new(node)
    when NodeObj then ItemObj.new(node)
    else raise("Unknown node: #{node}")
    end
  end
end

class Traversal
  def initialize(root, criteria)
    @current = Item.new(root)
    @stack = []
    @criteria = criteria
    @ci = 0
    @path = []
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

def process(node)
  puts node
end

puts "Traversal\n"
criteria = [:root]
iter = Traversal.new(root, criteria)
while item = iter.next do
  process(item)
end

puts
