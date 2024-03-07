from operator import xor
import math
from networkx.algorithms import dijkstra_path, power, triangles,dijkstra_path_length
import random
from itertools import chain, combinations
from random import randrange
from typing import NamedTuple
import matplotlib.pyplot as plt
import itertools as it
import networkx as nx
import numpy as np


def show(points, sets=[]):
    colors = ['red','green', 'blue', 'purple']

    x_coords = [point.x for point in points]
    y_coords = [point.y for point in points]


    plt.scatter(x_coords, y_coords, color='blue')
    plt.title('Plot of Points')
    i = 0
    if len(sets)>0:
        for set1 in sets:
            color = colors[i]
            x_coords = [point.x for point in set1]
            y_coords = [point.y for point in set1]
            plt.scatter(x_coords,y_coords,color=color)
            i = i+1;
    plt.show()

def euclidean(a,b):
    cx = np.array((a.x, a.y))
    cy = np.array((b.x, b.y))
    return np.linalg.norm(cx - cy)

class Point(NamedTuple):
    x: float
    y: float
    
    def beauty_print(self):
        return "("+str(self.x)+","+str(self.y)+")"

def beauty(setx):
    for point in setx:
        print(str(point.x)+"-"+str(point.y))

def w(x,metricspace):
    #Metricspace
    x_metric_coords = [point.x for point in metricspace]
    y_metric_coords = [point.y for point in metricspace]
    plt.scatter(x_metric_coords, y_metric_coords, color='black')

    #Set X
    x_coords = [point.x for point in x]
    y_coords = [point.y for point in x]

    plt.scatter(x_coords,y_coords,color='red')

    for point1 in x:
        for point2 in metricspace:
            plt.plot([point1[0], point2[0]], [point1[1], point2[1]], 'b-')

    plt.title('Plot of W(X)')
    plt.show()

def weight(x,metricspace, debug=False):
    if debug :
        print("=======================")
        print("Starting debuging set "+str(x))
    w = 0
    for point1 in x:
        if debug: print("Nuevo punto: "+str(point1))
        for point2 in metricspace:
            if debug: print("   D: "+str(point1)+ " - - "+str(point2) +"  = =  "+str(euclidean(point1,point2)))
            w = w + euclidean(point1,point2)
    return w/len(metricspace)



space=[Point(x=33, y=27), Point(x=4, y=23), Point(x=6, y=15), Point(x=36, y=18), Point(x=8, y=27), Point(x=16, y=21), Point(x=37, y=33), Point(x=20, y=22), Point(x=36, y=24), Point(x=39, y=6), Point(x=35, y=9), Point(x=21, y=4), Point(x=3, y=21), Point(x=7, y=23), Point(x=37, y=34), Point(x=0, y=31), Point(x=35, y=14), Point(x=19, y=1), Point(x=30, y=14), Point(x=25, y=37), Point(x=31, y=31), Point(x=9, y=36), Point(x=12, y=12), Point(x=18, y=35), Point(x=2, y=32), Point(x=24, y=17), Point(x=35, y=29), Point(x=7, y=26), Point(x=31, y=23), Point(x=0, y=7), Point(x=0, y=11), Point(x=34, y=34), Point(x=29, y=25), Point(x=18, y=35), Point(x=7, y=20), Point(x=37, y=23), Point(x=9, y=11), Point(x=14, y=0), Point(x=18, y=33), Point(x=28, y=25)]


x =[Point(x=4, y=23), Point(x=24, y=17), Point(x=39, y=6), Point(x=36, y=18), Point(x=35, y=29), Point(x=28, y=25)]

y =[Point(x=29, y=25), Point(x=7, y=20), Point(x=25, y=37), Point(x=34, y=34), Point(x=7, y=26), Point(x=2, y=32), Point(x=12, y=12), Point(x=19, y=1)]

z1=[Point(x=8, y=27), Point(x=9, y=36), Point(x=18, y=33), Point(x=36, y=24)]
z2=[Point(x=37, y=33), Point(x=0, y=11), Point(x=7, y=23), Point(x=18, y=35)]

common = list(set(x).union(y))
common = list(set(common).union(z1))
common = list(set(common).union(z2))

yz1 = y + list(set(z1) - set(y))
yz2 = y + list(set(z2) - set(y))
print(len(space))
print(len(common))
print(len(x))
print(len(y))
print(len(z1))
print(len(z2))
print(len(yz1))
print(len(yz2))

# wx = weight(x,nodup, True)
wz1 = weight(z1,common)
wz2 = weight(z2,common)
wyz1 = weight(yz1,common)
wyz2 = weight(yz2,common)
print("D(Z1)=" + str(wz1) + "   |   D(YUZ1)="+str(wyz1))
print("D(Z2)=" + str(wz2) + "   |   D(YUZ2)="+str(wyz2))
print(str(wz1 < wz2)+"                       |           "+str(wyz1 < wyz2))

# wy = weight(y,space)
# print("WX")
# print(wx)
# print("Wy")
# print(wy)
# print("Wz1")
# print(wz1)
# print("Wz2")
# print(wz2)
# print("Wyz1")
# print(wyz1)
# print("Wyz2")
# print(wyz2)
#
