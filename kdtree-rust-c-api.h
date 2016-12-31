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

void* kdtree3_create(Point3WithId* points, size_t len);
Point3WithId kdtree3_nearest_search(void* tree, Point3WithId* for_node);
bool kdtree3_has_neighbor_in_range(void* tree, Point3WithId* for_node, double range);
double kdtree3_distance_squared_to_nearest(void* tree, Point3WithId* for_node);
double kdtree3_insert_node(void* tree, Point3WithId* for_node);
void kdtree3_free(void* tree);

void* kdtree2_create(Point2WithId* points, size_t len);
Point2WithId kdtree2_nearest_search(void* tree, Point2WithId* for_node);
bool kdtree2_has_neighbor_in_range(void* tree, Point2WithId* for_node, double range);
double kdtree2_distance_squared_to_nearest(void* tree, Point2WithId* for_node);
double kdtree2_insert_node(void* tree, Point2WithId* for_node);
void kdtree2_free(void* tree);

}