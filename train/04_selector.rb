require "pry"
require "pp"

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
    @iter = node.children.each_with_index
  end

  def next
    node, index = @iter.next
    item = Item.new(node)
    path_step = PathStep.new(:index, index)
    StackItem.new(item, path_step)
  rescue StopIteration
    nil
  end
end

class ItemObj < ItemBase
  def initialize(node)
    raise "Node for ItemObj must be NodeObj" unless node.is_a?(NodeObj)
    @node = node
    @iter = node.children.each
  end

  def next
    key, node = @iter.next
    item = Item.new(node)
    path_step = PathStep.new(:key, key)
    StackItem.new(item, path_step)
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

    # build root stack item
    item = Item.new(root)
    path_step = PathStep.new(:root)
    stack_item = StackItem.new(item, path_step)

    @current = stack_item
    @stack = []
    @criteria = criteria
    @ci = 0
    @path = []
  end

  def next
    while @current do
      log
      log "Current: #{@current}"
      log "Stack size: #{@stack.size}"
      criterion = @criteria[@ci]

      if criterion
        if match?(@current.path_step, criterion)
          log "MATCH  #{@current.path_step} -> #{criterion}"

          if @criteria[@ci + 1].nil?
            item = @current.item
            @ci -= 1
            @current = @stack.pop
            log "RETURN ITEM"
            return item
          else
            @ci += 1
            @stack.push(@current)
            @current = @current.next
            unless @current
              @ci -= 1
              @stack.pop
              @ci -= 1
              @current = @stack.pop
            end
          end
        else
          log "NO MATCH  #{@current.path_step} -> #{criterion}"
          if nxt = @stack.last.next
            @current = nxt
          else
            @ci -= 1
            @current = @stack.pop
          end
        end
      else
        raise "IMPOSSIBLE"
      end
    end

    log "@current is nil"
  end
end

def log(msg = nil)
  puts msg
end

class PathStep
  TYPES = [:root, :key, :index].freeze
  attr_reader :type, :val

  def initialize(type, val = nil)
    raise("Unknown PathStep type: #{type.inspect}") unless TYPES.include?(type)

    is_ok = (type == :key && val.is_a?(String)) || (type == :index && val.is_a?(Integer)) || type == :root

    unless is_ok
      raise("PathStep type and val do not match")
    end

    @type = type
    @val = val
  end

  def ==(other)
    self.type == other.type && self.val == other.val
  end

  def to_s
    "(#{type}: #{val})"
  end
end

class StackItem
  attr_reader :item, :path_step

  def initialize(item, path_step)
    raise("Unexpected item type") unless item.is_a?(ItemBase)
    raise("Unexpected path_step type") unless path_step.is_a?(PathStep)

    @item = item
    @path_step = path_step
  end

  def next
    item.next
  end

  def to_s
    "StackItem: item = #{item}, path_step = #{path_step}"
  end
end

class Criterion
  attr_reader :type, :val

  def initialize(type, val)
    @type = type
    @val = val
  end

  def to_s
    "(#{type}, #{val})"
  end
end

def process(node)
  puts node
end

def match?(path_step, criterion)
  raise "path_step must be PathStep" unless path_step.is_a?(PathStep)
  raise "criterion must be Criterion" unless criterion.is_a?(Criterion)

  case [path_step.type, criterion.type]
  when [:root, :root] then true
  when [:key, :key]
    path_step.val == criterion.val
  when [:key, :any_key] then true
  when [:index, :any_index] then true
  else
    false
  end
end


puts "Traversal\n\n"
criteria = [
  Criterion.new(:root, nil),
  Criterion.new(:key, "user"),
  Criterion.new(:any_key, nil),
  #Criterion.new(:any_index, nil),
  #Criterion.new(:key, "name"),
]
iter = Traversal.new(root, criteria)
while item = iter.next do
  print "  --->  "
  process(item)
  puts
end

