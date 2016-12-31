#pragma once

extern "C" {

struct Point3WithId {
    void* pointer;
    double vals[3];
};

struct Point2WithId {
    void* pointer;
    double vals[2];
};

void kdtree3_create(Point3WithId* points, size_t len);
Point3WithId kdtree3_nearest_search(Point3WithId* for_node);
bool kdtree3_has_neighbor_in_range(Point3WithId* for_node, double range);
double kdtree3_distance_squared_to_nearest(Point3WithId* for_node);
double kdtree3_insert_node(Point3WithId* for_node);

void kdtree2_create(Point2WithId* points, size_t len);
Point2WithId kdtree2_nearest_search(Point2WithId* for_node);
bool kdtree2_has_neighbor_in_range(Point2WithId* for_node, double range);
double kdtree2_distance_squared_to_nearest(Point2WithId* for_node);
double kdtree2_insert_node(Point2WithId* for_node);
}
