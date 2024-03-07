from networkx.algorithms import shortest_path_length
from random import randrange
from typing import NamedTuple
import matplotlib.pyplot as plt
import itertools as it
import networkx as nx
import numpy as np


class Point(NamedTuple):
    x: float
    y: float
    name: str

def show(G):
    # pos = nx.spring_layout(G, seed=9)  # positions for all nodes - seed for reproducibility
    pos = nx.get_node_attributes(G, "pos")
    edge_labels = nx.get_edge_attributes(G, "weight")
    nx.draw_networkx_nodes(G, pos, node_size=700)
    nx.draw_networkx_edges(G, pos, width=6)

    nx.draw_networkx_labels(G, pos, font_size=20, font_family="sans-serif")
    
    nx.draw_networkx_edge_labels(G, pos, edge_labels)

    ax = plt.gca()
    ax.margins(0.08)
    plt.axis("off")
    plt.tight_layout()
    plt.show()


# POINTS: ARRAY OF POINTS
# DISTANCE: distance Function between those pointsc
def create_graph(points, distance):
    G = nx.Graph()
    s = it.combinations(list(points),2)
    counter = 0
    for xy in list(s):
        x = xy[0]
        y = xy[1]
        x_name = x.name
        y_name = y.name
        d = distance(x,y)
        G.add_node(x_name, pos=(x.x, x.y))
        G.add_node(y_name, pos=(y.x, y.y))
        G.add_edge(x_name, y_name, weight=str(d))
        counter = counter +1    
    return G


def shortest_path(G, x,y):
    n1 = x.name
    n2 = y.name
    return nx.shortest_path(G,source=n1,target=n2)

def euclidean(a,b):
    cx = np.array((a.x, a.y))
    cy = np.array((b.x, b.y))
    return np.linalg.norm(cx - cy)

def hamming(a,b):
    result = 0
    if a.x!=b.x:
        result = result+1;
    return result

def debug(a,b):
    print("D("+a.name+","+b.name+")="+str(euclidean(a,b)))

def show_2dplane(points):
    plt.figure(dpi=100)
    coords = list(map(lambda x: (x.x, x.y), points))
    for p in coords: 
        plt.plot(p)

    plt.title('Plot NumPy array')
    plt.xlabel('x-axis')
    plt.ylabel('y-axis')

    plt.show()


a = Point(x=1, y=1, name='a')
b = Point(x=8, y=8, name='b')
c = Point(x=7, y=8, name='c')
d = Point(x=9, y=8, name='d')


def find_counter(a,b,c,d):
    ac = hamming(a,c)
    bc = hamming(b,c)

    ad = hamming(a,d)
    bd = hamming(b,d)

    cd = hamming(c,d)

    close_AB_C = ac
    close_AB_D = ad
    if bc < ac:
        print("B-C+C-D >= ")
        close_AB_C = bc
    else:
        print("A-C+C-D >=")

    if bd < ad:
        print("B-D")
        close_AB_D = bd
    else:
        print("A-D")

    left = close_AB_C+cd
    right = close_AB_D
    print(str(close_AB_C)+"+"+str(cd)+">="+str(close_AB_D)+": "+str(left>=right))
    if left>=right:
        print(a,b,c,d)
    return left>=right

while True:
    a = Point(x=randrange(1), y=randrange(1), name='a')
    b = Point(x=randrange(1), y=randrange(1), name='b')
    c = Point(x=randrange(1), y=randrange(1), name='c')
    d = Point(x=randrange(1), y=randrange(1), name='d')

    find_counter(a,b,c,d)

    # a = Point(x=0, y=0, name='a')
    # b = Point(x=0, y=1, name='b')
    # c = Point(x=1, y=1, name='c')
    # d = Point(x=0, y=1, name='d')


# debug(a,b)
# debug(a,c)
# debug(a,d)
# debug(b,c)
# debug(b,d)
# debug(c,d)
# print("Closest to d")
# debug(a,b)
# debug(a,d)
# debug(b,d)
# print("Closest to c")
# debug(a,c)
# debug(b,c)
# print("_________________")
# if euclidean(b,d) < euclidean(a,d):
#     debug(b,d)
# else:
#     debug(a,d)
# debug(d,c)
# print(">=")
#
# r = euclidean(a,d)+euclidean(d,c)
# if euclidean(b,c) < euclidean(a,c):
#     debug(b,c)
#     print("Triangle inequality "+str(r>=euclidean(b,c)))
# else:
#     debug(a,c)
#     print("Triangle inequality "+str(r>=euclidean(a,c)))


# points = [a,b,c,d]
# graph = create_graph(points,euclidean)
# show(graph)





