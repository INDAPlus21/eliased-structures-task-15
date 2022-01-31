def insert(node, key):
    if node == None:
        # print("inserted", key)
        return {"key": key, "height": 0, "left": None, "right": None}
    elif key < node["key"]:
        # to_insert = insert(node["left"], key) # is it that it gets the left, not right
        # print("to_insert: ", to_insert)
        node["left"] = insert(node["left"], key)
    else:
        # to_insert = insert(node["right"], key)
        # print("to_insert: ", to_insert)
        node["right"] = insert(node["right"], key)
    # print("up loop :", node, "key: ", key) # remember than this can happen after
    node["height"] = new_height(node)

    print("root before balance: ", node)
    node = balance(node)
    print("will return: ", node)
    # print("node in insert:", node, "key: ", key) # remember than this can happen after

    return node


def new_height(node):
    left_height = -1
    right_height = -1
    if node["left"] != None:
        left_height = node["left"]["height"]
    if node["right"] != None:
        right_height = node["right"]["height"]
    return max(left_height, right_height) + 1


def right_rotation(node):
    print("node in right rotation: ", node)

    new_root = node["left"]
    node["left"] = new_root["right"]
    new_root["right"] = node

    # Update heights 
    new_root["right"]["height"] = new_height(new_root["right"])
    new_root["height"] = new_height(new_root)

    print("new_root: ", new_root)

    return new_root


def left_rotation(node):
    print("node in left rotation: ", node)

    new_root = node["right"]
    node["right"] = new_root["left"]
    new_root["left"] = node 

    # Update heights 
    new_root["left"]["height"] = new_height(new_root["left"])
    new_root["height"] = new_height(new_root)

    print("new_root: ", new_root)

    return new_root

def left_right_rotation(node):
    node["left"] = left_rotation(node["left"]) # left_rotation(node["left"])
    return left_rotation(node)


def right_left_rotation(node):
    node["right"] = right_rotation(node["right"])
    return right_rotation(node)


def balance(node):
    balance_value = balance_of_node(node)
    print("balance_value: ", balance_value)

    # Right heavy
    if balance_value == 2:
        if balance_of_node(node["left"]) >= 0:
            return left_rotation(node)
        else:
            return left_right_rotation(node)

    # Left heavy
    elif balance_value == -2:
        if balance_of_node(node["right"]) >= 0:
            return right_rotation(node)
        else:
            return right_left_rotation(node)

    return node


def balance_of_node(node):
    if node == None:
        return 0
    if node["right"] == None:
        right_height = 0
    else:
        right_height = node["right"]["height"]+1
    if node["left"] == None:
        left_height = 0
    else:
        left_height = node["left"]["height"]+1

    print("left height: ", left_height, "right_height: ", right_height, "of: ", node["key"])

    return right_height - left_height

def pretty_print(node, level=0):
    if node != None:
        pretty_print(node["right"], level+1)
        print(" "*4*level, "->", node["key"])
        pretty_print(node["left"], level+1)
    # for i in range(100):
        # print(root["key"])
        # pretty_print()

def insert_many(keys):
    root = insert(None, keys[0])
    for i in keys[1:]:
        root = insert(root, i)
    pretty_print(root) 

insert_many([18, 5, 11, 12, 17, 4, 8])
# insert_many([13, 10, 15, 5, 11, 16, 4, 6])
# insert_many([1, 2, 3])
# pretty_print(root) 
# root = insert(None, 3)
# print("root: ", root)
# pretty_print(root) 
# root = insert(root, 4)
# pretty_print(root) 
# print("root: ", root)
# root = insert(root, 5)
# root = insert(root, 6)
# root = insert(root, 10)
# root = insert(root, 2)
# print("root: ", root)
# pretty_print(root) 
# print("root: ", root)
# insert(root, 5)
# pretty_print(root) 
# print("root: ", root)
# insert(root, 6)
# insert(root, 6)
# insert(root, 7)
# insert(root, 3)
# print("final root: ", root)

# pretty_print(root)