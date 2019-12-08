use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn get_center_orbiter_pair<R: BufRead>(read: R) -> Result<Vec<(String, String)>> {
    let mut map = Vec::new();
    for value in read.lines() {
        let value = value?;
        if value.is_empty() {
            continue;
        }
        let mut split = value.split(')');
        let center = split.next().ok_or("no center")?.to_string();
        let orbiter = split.next().ok_or("no orbiter")?.to_string();
        map.push((center, orbiter))
    }

    Ok(map)
}

struct OrbitMap {
    orbiter_center: HashMap<String, String>,
}

impl FromIterator<(String, String)> for OrbitMap {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        let mut reverse_map = HashMap::new();
        for (center, orbiter) in iter.into_iter() {
            reverse_map.insert(orbiter, center);
        }
        Self {
            orbiter_center: reverse_map,
        }
    }
}

impl OrbitMap {
    fn orbiters(&self) -> impl Iterator<Item = &str> {
        self.orbiter_center.keys().map(String::as_str)
    }

    fn direct_orbiting(&self, orbiter: &str) -> Option<&str> {
        self.orbiter_center.get(orbiter).map(String::as_str)
    }

    fn indirect_orbitings<'a>(&'a self, orbiter: &'a str) -> IndirectOrbitingsIterator<'a> {
        IndirectOrbitingsIterator {
            map: self,
            current: Some(orbiter),
        }
    }

    fn common_orbiting<'a>(&'a self, a: &'a str, b: &'a str) -> Option<&'a str> {
        let a_orbiting: HashSet<_> = self.indirect_orbitings(a).collect();
        self.indirect_orbitings(b)
            .find(|indirect| a_orbiting.contains(indirect))
    }

    fn jumps(&self, from: &str, to: &str) -> Option<usize> {
        self.indirect_orbitings(from).position(|x| x == to)
    }
}

struct IndirectOrbitingsIterator<'a> {
    map: &'a OrbitMap,
    current: Option<&'a str>,
}

impl<'a> Iterator for IndirectOrbitingsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        let result = self.map.direct_orbiting(current);
        self.current = result;
        result
    }
}

fn run<R: BufRead>(read: R) -> Result<usize> {
    let map = OrbitMap::from_iter(get_center_orbiter_pair(read)?);
    let common = map.common_orbiting("YOU", "SAN").ok_or("no common")?;
    let a = map.jumps("YOU", common).ok_or("no jump from YOU")?;
    let b = map.jumps("SAN", common).ok_or("no jump from SAN")?;
    Ok(a + b)
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let code = run(stdin.lock())?;
    println!("code: {}", code);
    Ok(())
}

#[test]
fn test() {
    let input = include_bytes!("../../input/06");
    assert_eq!(run(&input[..]).ok(), Some(409));
}
