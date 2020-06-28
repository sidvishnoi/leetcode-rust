struct Solution;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut itinerary = Vec::new();
        if tickets.is_empty() {
            return itinerary;
        }

        // source => destinations (as a min-heap)
        let mut flights: HashMap<&str, BinaryHeap<Reverse<&str>>> = HashMap::new();
        for ticket in &tickets {
            let (src, dst) = (&ticket[0], &ticket[1]);
            let flights_from_src = flights.entry(&src).or_default();
            flights_from_src.push(Reverse(&dst));
        }

        let mut stack = Vec::new();
        stack.push("JFK");
        while !stack.is_empty() {
            let src = stack.last().unwrap();
            if let Some(destinations) = flights.get_mut(src) {
                if let Some(Reverse(dst)) = destinations.pop() {
                    stack.push(dst);
                    continue;
                }
            }
            itinerary.push(src.to_string());
            stack.pop();
        }
        itinerary.reverse();
        itinerary
    }
}

fn to_tickets(tickets: Vec<[&str; 2]>) -> Vec<Vec<String>> {
    tickets
        .into_iter()
        .map(|s| s.iter().map(|l| l.to_string()).collect())
        .collect()
}

fn main() {
    let tickets = to_tickets(vec![
        ["MUC", "LHR"],
        ["JFK", "MUC"],
        ["SFO", "SJC"],
        ["LHR", "SFO"],
    ]);
    let itinerary = vec!["JFK", "MUC", "LHR", "SFO", "SJC"];
    assert_eq!(itinerary, Solution::find_itinerary(tickets));

    let tickets = to_tickets(vec![
        ["JFK", "SFO"],
        ["JFK", "ATL"],
        ["SFO", "ATL"],
        ["ATL", "JFK"],
        ["ATL", "SFO"],
    ]);
    let itinerary = vec!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"];
    assert_eq!(itinerary, Solution::find_itinerary(tickets));
}
