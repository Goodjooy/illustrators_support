use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    hash::Hash,
    sync::{Arc, Mutex},
    time::Duration,
};

use chrono::Utc;
use dashmap::DashMap;

struct TimeOrd<T> {
    t: u64,
    data: Arc<T>,
}

impl<T> PartialEq for TimeOrd<T> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl<T> Eq for TimeOrd<T> {}

impl<T> Ord for TimeOrd<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.t.cmp(&other.t) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl<T> PartialOrd for TimeOrd<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Some(cmp_res) = self.t.partial_cmp(&other.t) {
            Some(match cmp_res {
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Less,
            })
        } else {
            None
        }
    }
}

fn get_now() -> u64 {
    let now = Utc::now();
    now.timestamp() as u64
}

pub struct LifeTimeMap<K, V> {
    pq: Mutex<BinaryHeap<TimeOrd<K>>>,
    data: DashMap<Arc<K>, V>,
}

impl<K: Eq + Hash, V> LifeTimeMap<K, V> {
    pub fn new() -> Self {
        LifeTimeMap {
            pq: Mutex::new(BinaryHeap::new()),
            data: DashMap::new(),
        }
    }
    fn updata_map(&self) -> Result<(), String> {
        let mut pq = self.pq.lock().or_else(|e| Err(e.to_string()))?;
        let map = &self.data;

        let now = get_now();
        loop {
            if let Some(t) = pq.peek() {
                if &t.t < &now {
                    map.remove(&t.data);
                } else {
                    break;
                }
            } else {
                break;
            }
            pq.pop();
        }

        Ok(())
    }

    pub fn get_pop(&self, key: &K) -> Option<V>
    where
        K: Hash + Eq,
    {
        self.updata_map().expect("Updata map Error");
        self.data.remove(key).and_then(|(_k, v)| Some(v))
    }

    pub fn insert(&self, key: K, value: V, lifetime: Duration) -> Option<V> {
        let key = Arc::new(key);
        let time_ord = TimeOrd {
            t: get_now() + lifetime.as_secs(),
            data: Arc::clone(&key),
        };

        let mut pq = self.pq.lock().ok()?;
        let map = &self.data;

        pq.push(time_ord);
        map.insert(Arc::clone(&key), value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_death_data() {
        let t = LifeTimeMap::new();
        t.insert(1, 2, Duration::from_secs(2));
        t.insert(12, 22, Duration::from_secs(2));
        t.insert(13, 23, Duration::from_secs(2));
        std::thread::sleep(Duration::from_secs(1));
        assert_eq!(Some(2), t.get_pop(&1));
        std::thread::sleep(Duration::from_secs(3));

        assert_eq!(None, t.get_pop(&12));
        assert_eq!(None, t.get_pop(&13));
    }
}
