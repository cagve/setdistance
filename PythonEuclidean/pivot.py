from operator import xor
import math
from networkx.algorithms import dijkstra_path, power, triangles,dijkstra_path_length
import random
from itertools import chain, combinations
from random import randint, randrange
from typing import NamedTuple
import matplotlib.pyplot as plt
import itertools as it
import networkx as nx
import numpy as np

def remove_duplicates(x):
  return list(dict.fromkeys(x))
   
def are_disjoint(list1, list2):
    result = False

    # traverse in the 1st list
    for x in list1:

        # traverse in the 2nd list
        for y in list2:

            # if one common
            if x == y:
                result = True
                return result 

    return result


def show(points, sets=[]):
    x_coords = [point.x for point in points]
    y_coords = [point.y for point in points]

    plt.scatter(x_coords, y_coords,color='black')
    plt.title('Plot of Points')
    i = 0
    markers = ['o','v','+','*']
    if len(sets)>0:
        for set1 in sets:
            marker = markers[i]
            x_coords = [point.x for point in set1]
            y_coords = [point.y for point in set1]
            plt.scatter(x_coords,y_coords,color='black', marker=marker)
            i = i+1;
    plt.show()

class Point(NamedTuple):
    x: float
    y: float


def discrete(a,b):
    if a==b:
        return 1
    else:
        return 0

def manhattan(a, b):
    d = abs(a.x-b.x)+abs(a.y-b.y)
    return d

def euclidean(a,b):
    cx = np.array((a.x, a.y))
    cy = np.array((b.x, b.y))
    return np.linalg.norm(cx - cy)

def get_subset(metricspace, size=0):
    max = len(metricspace)
    if size == 0 or size > max:
        size = random.randint(1,max)
    subset = list()
    while len(subset) != size:
        n = random.randint(0,max-1)
        subset.append(metricspace[n])
        remove_duplicates(subset)
    return subset

def get_disjoint_set(initial_list, num_lists=4):
    random_lists = []
    remaining_elements = initial_list.copy()
    list_length = random.randint(1,10)
    print("New set")
    for j in range(num_lists):
        n = len(remaining_elements)
        if j == 0: # MAX value in this case is total-3
            list_length = random.randint(1,n-num_lists)
            n = n-num_lists
            print("Quedan "+ str(n) + " y he cogido " + str(list_length))
        elif j == 1:
            list_length = random.randint(1,n-1)
            print("Quedan "+ str(n) + " y  he cogido " + str(list_length))
        elif j == 2:
            list_length = random.randint(1,math.floor(n/2))
            print("Quedan "+ str(n) + " y  he cogido " + str(list_length))
        else:
            print("Quedan "+ str(n) + " y  he cogido " + str(list_length))

        if len(remaining_elements) < list_length:
            # If there are not enough remaining elements, break out of the loop
            print("SE ACABARON")
            break
        random_list = random.sample(remaining_elements, list_length)
        random_lists.append(remove_duplicates(random_list))

        # Remove the elements of the generated list from the remaining elements
        remaining_elements = [elem for elem in remaining_elements if elem not in random_list]
    return random_lists


def create_metricspace(size):
    points = []
    for _ in range(size):
        a = Point(x=randrange(size), y=randrange(size))
        points.append(a)
    return points


def powerset(iterable):
    "powerset([1,2,3]) --> () (1,) (2,) (3,) (1,2) (1,3) (2,3) (1,2,3)"
    s = list(iterable)
    a = chain.from_iterable(combinations(s, r) for r in range(len(s)+1))
    return list(a)

def pivot_distance(set1, set2, pivot):
    d = 0
    for point in set1:
        d = d+euclidean(point,pivot)
    for point in set2:
        d = d+euclidean(point,pivot)
    return d

def weight(set1, metricspace):
    d = 0
    for i in set1:
        for j in metricspace:
            d = d + euclidean(i,j)
    return d/len(metricspace)

def full_pivot_distance(set1,set2, metricspace):
    if set1!=set2:
        return weight(set1, metricspace) + weight(set2,metricspace)
    else:
        return 0

def ax_decomposition(set1,set2,metricspace):
    union = set1 + list(set(set2) - set(set1))
    wx = weight(set1, metricspace)
    wy = weight(set2, metricspace)
    wxy= weight(set1, metricspace)+weight(set2,metricspace)
    wxuniony= weight(union, metricspace)
    print(str(wx)+"+"+str(wy)+"="+str(wxy)) #debug
    print(str(wxuniony)+"="+str(wxy)) #debug
    return wx+wy == wxy

def ax7(set1,set2,set3, metricspace):
    union = set3 + list(set(set2) - set(set3))
    dxy = full_pivot_distance(set1,set2, metricspace)
    dxyuz = full_pivot_distance(set1,union, metricspace)
    if are_disjoint(set1,set2) and are_disjoint(set2,set3) and are_disjoint(set1,set3):
        if(dxy >= dxyuz):
            print("FALSEEEEEEEEEEEEEEEEEE")
            return False
        else:
            print("Not counterexample found")
            return True
    return True

def ax7_1(set1,set2,set3, metricspace):
    union = set3 + list(set(set2) - set(set3))
    dxy = full_pivot_distance(set1,set2, metricspace)
    dxz = full_pivot_distance(set1,set3, metricspace)
    dxyuz = full_pivot_distance(set1,union, metricspace)
    if dxz < dxy:
        if(dxy >= dxyuz):
            print("Counterexample found")
            print("X"+str(set1))
            print("Y"+str(set2))
            print("Z"+str(set3))
            return False
        else:
            print("Not counterexample found")
            return True
    return True
    

def test_decom(metricspace):
    sets = get_disjoint_set(metricspace,2)
    r = True
    while r:
        sets = get_disjoint_set(metricspace,2)
        x = sets[0]
        y = sets[1]
        r = ax_decomposition(x,y,metricspace)

def test_ax(metricspace, n):
    r = True
    while r:
        x = get_subset(metricspace, randint(1,2))
        y = get_subset(metricspace, randint(1,2))
        z = get_subset(metricspace, randint(1,2))
        if n == 7:
            r = ax7(x,y,z,metricspace)
        elif n == 71:
            r = ax7_1(x,y,z,metricspace)
        else:
            print("Axiom not found")



def test(points):
    r = True
    while r:
            # r=False
            # x = get_subset(points)
            # y = get_subset(points)
            # m = random.randint(1,len(points))
            # z1 = get_subset(points, m)
            # z2 = get_subset(points, m)
            sets = get_disjoint_set(points)
            x = sets[0]
            y = sets[1]
            z1 =sets[2]
            z2 =sets[3]
            yz1 = y + list(set(z1) - set(y))
            yz2 = y + list(set(z2) - set(y))

            dxz1 = full_pivot_distance(x,z1,points)
            dxz2 = full_pivot_distance(x,z2,points)
            dxyz1 = full_pivot_distance(x,yz1,points) #Union XZ1
            dxyz2 = full_pivot_distance(x,yz2,points) #Union XZ2
            print("D(X,Z)="+str(dxz1))
            print("D(X,Z')="+str(dxz2))
            print("D(X,YUZ)="+str(dxyz1))
            print("D(X,YUZ')="+str(dxyz2))


            if(dxz1 == dxz2):
                print("No aplico por D(X,Z)==D(X,Z')")
                print(str(dxz1)+"="+str(dxz2))
            if len(z2) != len(z1):
                print("No aplico por |Z|!=|Z|")
            elif are_disjoint(x,yz1) or are_disjoint(x,yz2) or are_disjoint(y,z1) or are_disjoint(y,z2):
            # elif are_disjoint(x,z1) or are_disjoint(x,z2):
                print("No aplico por disjoint condition")
            elif dxz1 < dxz2: 
                print("Aplico Case (1) D(X,Z)<D(X,Z')")
                if dxyz1 >= dxyz2:
                    print("Counterexample "+str(dxz1)+"<"+str(dxz2)+" but " +str(dxyz1)+">="+str(dxyz2))
                    print("Metric space="+str(points))
                    print("X="+str(x))
                    print("Y="+str(y))
                    print("Z="+str(z1))
                    print("Z'="+str(z2))
                    print("D(X,Z)="+str(dxz1))
                    print("D(X,Z')="+str(dxz2))
                    print("D(X,YUZ)="+str(dxyz1))
                    print("D(X,YUZ')="+str(dxyz2))
                    r = False
            else:
                print("Aplico Case (2) D(X,Z)>D(X,Z')")
                if dxyz1 <= dxyz2:
                    print("Counter example: "+str(dxz1)+">"+str(dxz2)+" but " +str(dxyz1)+"<="+str(dxyz2))
                    print("Metric space="+str(points))
                    print("X="+str(x))
                    print("Y="+str(y))
                    print("Z="+str(z1))
                    print("Z'="+str(z2))
                    print("D(X,Z)="+str(dxz1))
                    print("D(X,Z')="+str(dxz2))
                    print("D(X,YUZ)="+str(dxyz1))
                    print("D(X,YUZ')="+str(dxyz2))
                    r = False

# Counterexample Disjoint(X,Z) Disjoint(X,Z2)
# metric_space=[Point(x=0, y=3), Point(x=5, y=7), Point(x=9, y=7), Point(x=9, y=5), Point(x=4, y=3), Point(x=6, y=2), Point(x=6, y=4), Point(x=6, y=7), Point(x=6, y=1), Point(x=4, y=9)]
#
# x=[Point(x=6, y=4), Point(x=9, y=7), Point(x=5, y=7), Point(x=4, y=9)]
# y=[Point(x=0, y=3)]
# z1=[Point(x=9, y=5)]
# z2=[Point(x=0, y=3)]

# Counterexample Disjoint(X,Z) Disjoint(X,Z2) Disjoint(Y,Z1) Dijsoint(Y,Z2) but X=Y
# Metric space=[Point(x=1, y=0), Point(x=0, y=2), Point(x=0, y=0)]
# X=[Point(x=0, y=0)]
# Y=[Point(x=0, y=0)]
# Z=[Point(x=1, y=0)]
# Z'=[Point(x=0, y=2)]

# Counterexample pairwise Disjoint but in oposite case, D(X,Z')<D(X,Z) but D(X,Z)=D(X,Z')
# Metric space=[Point(x=3, y=0), Point(x=0, y=2), Point(x=1, y=3), Point(x=2, y=1)]
# X=[Point(x=0, y=2)]
# Y=[Point(x=2, y=1)]
# Z=[Point(x=3, y=0)]
# Z'=[Point(x=1, y=3)]



def debug(x,y,metricspace):
    names = []
    size = len(metricspace)
    for i in metricspace:
        names.append("x_{"+str(i.x)+str(i.y)+"}")
    
    for pivot in names:  
        for p1 in x:
            print("d(x_{"+str(p1.x)+str(p1.y)+"}, "+pivot+")")
    for pivot in names:  
        for p2 in y:
            print("d(x_{"+str(p2.x)+str(p2.y)+"}, "+pivot+")")


metricspace = create_metricspace(80)
test_ax(metricspace, 71)


# space=[ Point(x=45, y=12),
#     Point(x=72, y=30),
#     Point(x=18, y=65),
#     Point(x=93, y=47),
#     Point(x=36, y=88),
#     Point(x=81, y=23)
#         ]
#
# x=[space[0], space[2], space[3]]
# y=[space[1], space[5]]
# show(space,[x])
# ax_decomposition(x,y, space)

# space=[Point(x=33, y=27), Point(x=4, y=23), Point(x=6, y=15), Point(x=36, y=18), Point(x=8, y=27), Point(x=16, y=21), Point(x=37, y=33), Point(x=20, y=22), Point(x=36, y=24), Point(x=39, y=6), Point(x=35, y=9), Point(x=21, y=4), Point(x=3, y=21), Point(x=7, y=23), Point(x=37, y=34), Point(x=0, y=31), Point(x=35, y=14), Point(x=19, y=1), Point(x=30, y=14), Point(x=25, y=37), Point(x=31, y=31), Point(x=9, y=36), Point(x=12, y=12), Point(x=18, y=35), Point(x=2, y=32), Point(x=24, y=17), Point(x=35, y=29), Point(x=7, y=26), Point(x=31, y=23), Point(x=0, y=7), Point(x=0, y=11), Point(x=34, y=34), Point(x=29, y=25), Point(x=18, y=35), Point(x=7, y=20), Point(x=37, y=23), Point(x=9, y=11), Point(x=14, y=0), Point(x=18, y=33), Point(x=28, y=25)]
# x=[Point(x=4, y=23), Point(x=24, y=17), Point(x=39, y=6), Point(x=36, y=18), Point(x=35, y=29), Point(x=28, y=25)]
# y=[Point(x=29, y=25), Point(x=7, y=20), Point(x=25, y=37), Point(x=34, y=34), Point(x=7, y=26), Point(x=2, y=32), Point(x=12, y=12), Point(x=19, y=1)]
# z1=[Point(x=8, y=27), Point(x=9, y=36), Point(x=18, y=33), Point(x=36, y=24), Point(x=33, y=27)]
# z2=[Point(x=37, y=33), Point(x=18, y=35), Point(x=0, y=11), Point(x=7, y=23), Point(x=18, y=35)]

# mspace = [l for l in space if l not in x]
# mspace = [l for l in mspace if l not in y]
# mspace = [l for l in mspace if l not in z1]
# mspace = [l for l in mspace if l not in z2]
#
# show(mspace,[x,y,z1,z2])

