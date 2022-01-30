print("hello world")

# node3 = {"key": 2, "height": 0, "left": None, "right": None}
# node2 = {"key": 3, "height": 0, "left": None, "right": None}
# node1 = {"key": 1, "height": 1, "left": node2, "right": node3}
# print(node1)
# node1["left"] = node1["right"]

def insert(node, key):
    print("node initial: ", node, "key: ", key)
    if node == None:
        print("inserted", key)
        return {"key": key, "height": 0, "left": None, "right": None}
    elif key < node["key"]:
        to_insert = insert(node["left"], key) # is it that it gets the left, not right 
        print("to_insert: ", to_insert)
        node["left"] = to_insert 
    else: 
        to_insert = insert(node["right"], key) 
        print("to_insert: ", to_insert)
        node["right"] = to_insert
    print("up loop :", node, "key: ", key) # remember than this can happen after 
    # it doesn't fucking return again here 
    return node 
    # node["height"] = new_height(node)
    # print("node in insert:", node, "key: ", key) # remember than this can happen after 

def new_height(node):
    left_height = -1 
    right_height = -1
    if node["left"] != None: 
        left_height = node["left"]["height"]
    if node["right"] != None:
        right_height  = node["right"]["height"]
    return max(left_height, right_height) + 1

def right_rotation(node):
    # this isn't right: https://www.wikiwand.com/en/Right_rotation#:~:text=In%20a%20binary%20search%20tree,becomes%20X's%20new%20left%20child.
    node_left = node["left"]
    node["left"] = node["right"]
    node["right"] = node_left
    return node

def left_rotation(node):
    node_left = node["left"]
    node["left"] = node["right"]
    node["right"] = node_left
    return node

def left_left_rotation(node):
    return right_rotation(node)

def left_right_rotation(node):
    node["left"] = left_rotation(node["left"])
    return left_left_rotation(node)

def right_right_rotation(node):
    return left_rotation(node)

def right_left_rotation(node):
    node["right"] = right_rotation(node["right"])
    return right_right_rotation(node)

def balance(node):
    if balance_of_node(node) == -2:
        if balance_of_node(node["left"]) >= 0:
            return left_rotation(node)
        else: 
            return left_right_rotation(node)
    elif balance_of_node(node) == 2:
        if balance_of_node(node) >= 0:
            return right_right_rotation(option_node)
        else:
            return right_left_rotation(option_node)

    return node        

def balance_of_node(node):
    return node["left"]["height"] - node["right"]["height"];  
# def height_of_node(left_height, right_height):
# return max(left_height, right_height) + 1; # it will return 0 if no nodes, right...

root = insert(None, 3)
print("root: ", root)
insert(root, 4)
print("root: ", root)
insert(root, 2)
print("root: ", root)
insert(root, 1)
insert(root, 5)
# root = right_rotation(root)
print("root: ", root)
# print(insert(None, 4))
# print(insert())