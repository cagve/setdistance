from networkx.algorithms import dijkstra_path, triangles,dijkstra_path_length
from random import randrange
from typing import NamedTuple
import matplotlib.pyplot as plt
import itertools as it
import networkx as nx
import numpy as np



def show(G):
    pos = nx.spring_layout(G, seed=9)  # positions for all nodes - seed for reproducibility
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

def euclidean(a,b):
    cx = np.array((a.x, a.y))
    cy = np.array((b.x, b.y))
    return np.linalg.norm(cx - cy)

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
        G.add_edge(x_name, y_name, weight=float(d))
        counter = counter +1    

    return G


# W is the weight of A. Note that b's weight is always 1
def new_metricspace(G,a,b, w, debug=False):
    if debug:
        print("==================Starting debug step "+str(w)+"==================")
        print("Starting nodes = "+str(a)+str(b))

    d_ab = G.edges[a, b]['weight']
    name = str(w)
    w=w+1 # 1/2 1/3 1/4
    n=w-1 # 1/2 2/3 3/4
    
    G.add_edge(name, a, weight=d_ab/w)
    G.add_edge(name, b, weight=(n*d_ab)/w)
    nodes = G.nodes
    # All nodes that are not link to new yet.
    filtered = filter(lambda node:node != a and node != b and node != name , nodes)
    if debug:
        print(nodes)
        print(str(1)+"/"+str(w))
        print(str(n)+"/"+str(w))

    for node in filtered:
        path = dijkstra_path(G, name, node)
        if debug:
            print("Min cost from "+ name +" to "+node+str(path))
        weight = dijkstra_path_length(G,name,node)
        if debug:
            print("PATH"+str(path))
            path = dijkstra_path(G, name, node)
            G.add_edge(node, name, weight=weight, label=path)
    print("")




def debug_graph(G, debug=False):
    print("==================Starting graph debug==================")
    print("Nodes of G:"+str(G.nodes))
    print("Edges of G:")
    for edge in G.edges:
        a = edge[0]
        b = edge[1]
        w = G.edges[a,b]['weight']
        if debug:
            try:
                l = G.edges[a,b]['label']
                print("     ("+str(a)+"-"+str(b)+")  w:"+str(w) +" l:"+str(l))
            except:
                print("     ("+str(a)+"-"+str(b)+")  w:"+str(w))

def triangle_inequality(G, debug=False):
    nodes = G.nodes()
    s = it.combinations(list(nodes),3)
    result = True
    episilon = 1
    for case in s:
        if debug:
            print("==================Starting triangle "+str(case)+" debug==================")
        permutations = it.permutations(case)
        for permutation in permutations:
            if debug:
                print("      Order: "+str(permutation))
            a=permutation[0]
            b=permutation[1]
            c=permutation[2]

            ab = G.edges[a,b]['weight']
            bc = G.edges[b,c]['weight']
            ac = G.edges[a,c]['weight']

            epsilon = 1e-100
            result_n = print(abs(ab + bc - ac))
            result =  abs(ab + bc - ac)> epsilon
            result = round(ab+bc) >= round(ac)
            if debug:
                print("AB="+str(a)+str(b)+"="+str(ab))
                print("BC="+str(b)+str(c)+"="+str(bc))
                print("AC="+str(a)+str(c)+"="+str(ac))
                print(str(ab)+"+"+str(bc)+">="+str(ac)+"....."+str(result))
                print("")
            if result == False:
                return result
    return result

# def main():
#     print("Searching for counterexample...")
#     result = True
#     while result:
#         a = Point(x=randrange(10), y=randrange(10), name='a')
#         b = Point(x=randrange(10), y=randrange(10), name='b')
#         c = Point(x=randrange(10), y=randrange(10), name='c')
#         d = Point(x=randrange(10), y=randrange(10), name='d')
#         e = Point(x=randrange(10), y=randrange(10), name='d')
#         f = Point(x=randrange(10), y=randrange(10), name='d')
#         points = [a,b,c, d, e, f]
#
#         G = create_graph(points, euclidean)
#
#         for i in range(1,len(points)):
#             if i == 1:
#                 n1 = a.name
#                 n2 = b.name
#             else:
#                 n1 = str(i-1)
#                 n2 = points[i].name
#             new_metricspace(G, n1, n2, i, True)
#         result = triangle_inequality(G, True)
#     # debug_graph(G, True)


def hamming():
    valuation_size = 3
    s = it.product(*[(0, 1)] * valuation_size)
    print(list(s))

hamming()

# def random():
#     a = Point(x=randrange(100), y=randrange(100), name='a')
#     b = Point(x=randrange(100), y=randrange(100), name='b')
#     c = Point(x=randrange(100), y=randrange(100), name='c')
#     d = Point(x=randrange(100), y=randrange(100), name='d')
#     e = Point(x=randrange(100), y=randrange(100), name='e')
#
#     points = [a,b,c,d]
#     G = create_graph(points,euclidean)
#     new_point(G,a,b,'new')
#
#     nodes = G.nodes
#     s = it.combinations(list(nodes),3)
#     for xy in list(s):
#         a = xy[0]
#         b = xy[1]
#         c = xy[2]
#         proof = triangle_inequality(G, a,b,c)
#         if proof != True:
#             return False;
#     return True
#
#
# result = True
# counter = 0
#
# while result:
#     print(str(counter)+" "+str(result))
#     result = random()
#     counter = counter+1
    
