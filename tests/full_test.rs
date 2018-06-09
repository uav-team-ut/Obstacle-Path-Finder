extern crate pathfinder;
use std::collections::LinkedList;

use pathfinder::*;

#[test]
fn test0() {
    let mut waypoints = LinkedList::new();
    waypoints.push_back(
        Waypoint::from_degrees(0, 30.322280883789063, -97.60298156738281, 100f32, 10f32)
    );
    waypoints.push_back(
        Waypoint::from_degrees(1, 30.322280883789063, -97.60098266601564, 150f32, 10f32)
    );

    let flyzone = vec!(vec!(
        Point::from_degrees(30.32469, -97.60466, 0f32),
        Point::from_degrees(30.32437, -97.60367, 0f32),
        Point::from_degrees(30.32356, -97.60333, 0f32),
        Point::from_degrees(30.32276, -97.60398, 0f32),
        Point::from_degrees(30.32082, -97.60368, 0f32),
        Point::from_degrees(30.32173, -97.60008, 0f32),
        Point::from_degrees(30.32329, -97.59958, 0f32),
        Point::from_degrees(30.32545, -97.60066, 0f32),
        Point::from_degrees(30.32608, -97.60201, 0f32),
        Point::from_degrees(30.32613, -97.60339, 0f32),
        Point::from_degrees(30.32537, -97.60453, 0f32)
    ));
    let obstacles = vec!(
        Obstacle::from_degrees(30.32228, -97.60198, 50f32, 10f32)
    );

    let mut pathfinder = Pathfinder::new();
    pathfinder.init(5.0, flyzone, obstacles);
    let result = pathfinder.get_adjust_path(
        Plane::from_degrees(30.32298, -97.60310, 100.0),
        waypoints);
    println!("test 0 Result");
    for node in result {
        println!("{} {:.5}, {:.5}, {:.5}",
        node.index, node.location.lat_degree(), node.location.lon_degree(), node.location.alt());
    }
    println!();
}

#[test]
fn test1() {
    let flight_zone = vec!(
        Point::from_degrees(30.32521, -97.6023, 0f32),
        Point::from_degrees(30.32466, -97.59856, 0f32),
        Point::from_degrees(30.32107, -97.60032, 0f32),
        Point::from_degrees(30.32247, -97.60325, 0f32),
        Point::from_degrees(30.32473, -97.6041, 0f32)
    );

    println!("Flightzone:");
    for point in &flight_zone {
        println!("{:.5}, {:.5}", point.lat_degree(), point.lon_degree());
    }
    println!();
    let obstacles = vec!(
        Obstacle{coords: Point::from_degrees(30.32457, -97.60254, 0f32), radius: 50.0, height: 1.0},
        Obstacle{coords: Point::from_degrees(30.32429, -97.60166, 0f32), radius: 50.0, height: 1.0},
        Obstacle{coords: Point::from_degrees(30.32405, -97.60015, 0f32), radius: 50.0, height: 1.0},
        Obstacle{coords: Point::from_degrees(30.32344, -97.60077, 0f32), radius: 50.0, height: 1.0},
        Obstacle{coords: Point::from_degrees(30.32466, -97.60327, 0f32), radius: 50.0, height: 1.0}
    );

    let mut waypoints = LinkedList::new();
    waypoints.push_back(
        Waypoint::from_degrees(0, 30.32271, -97.60035, 100f32, 10f32)
    );
    waypoints.push_back(
        Waypoint::from_degrees(1, 30.32457, -97.59972, 150f32, 10f32)
    );
    // waypoints.push_back(
    //     Waypoint::new(2, Point::from_degrees(30.32271, -97.60035, 100f32), 10f32)
    // );
    let flyzone = vec!(flight_zone);
    let mut pathfinder = Pathfinder::new();
    pathfinder.init(5.0, flyzone, obstacles);
    let result = pathfinder.get_adjust_path(
        Plane::from_degrees(30.32491, -97.60159, 10.0),
        waypoints);
    eprintln!("test 1 Result");
    for node in result {
        eprintln!("{:.5}, {:.5}, {:.5}",
        node.location.lat_degree(), node.location.lon_degree(), node.location.alt());
    }
    eprintln!();

    // println!("A* Result w/o alt");
    // for node in result {
    //     println!("{:.5}, {:.5}",
    //     node.location.lat_degree(), node.location.lon_degree());
    // }
    // println!();
}