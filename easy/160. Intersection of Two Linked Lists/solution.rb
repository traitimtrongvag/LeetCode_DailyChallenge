def get_intersection_node(head_a, head_b)
    a = head_a
    b = head_b
    
    while a != b
        a = a ? a.next : head_b
        b = b ? b.next : head_a
    end
    
    a
end