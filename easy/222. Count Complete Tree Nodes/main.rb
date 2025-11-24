# 222. Count Complete Tree Nodes
def count_nodes(root)
    return 0 if root.nil?
    
    left_height = 0
    current = root
    while current
        left_height += 1
        current = current.left
    end
    
    right_height = 0
    current = root
    while current
        right_height += 1
        current = current.right
    end
    
    if left_height == right_height
        return (1 << left_height) - 1
    end
    
    1 + count_nodes(root.left) + count_nodes(root.right)
end