use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

struct TimeOrd<T> {
    t: Instant,
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

pub struct LifeTimeHashMap<K, V>
where
    K: Eq + Hash,
{
    pq: Mutex<BinaryHeap<TimeOrd<K>>>,
    data: Mutex<HashMap<Arc<K>, V>>,
}

impl<K: Eq + Hash, V> LifeTimeHashMap<K, V> {
    pub fn new() -> Self {
        LifeTimeHashMap {
            pq: Mutex::new(BinaryHeap::new()),
            data: Mutex::new(HashMap::new()),
        }
    }
    fn updata_map(&self) -> Result<(), String> {
        let mut pq = self.pq.lock().or_else(|e| Err(e.to_string()))?;
        let mut map = self.data.lock().or_else(|e| Err(e.to_string()))?;

        let now = Instant::now();
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

    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        self.updata_map().expect("Updata map Error");
        self.data.lock().ok()?.get(key).map(|v| v.clone())
    }

    pub fn insert(&self, key: K, value: V, lifetime: Duration) -> Option<V> {
        let key = Arc::new(key);
        let time_ord = TimeOrd {
            t: Instant::now() + lifetime,
            data: Arc::clone(&key),
        };

        let mut pq = self.pq.lock().ok()?;
        let mut map = self.data.lock().ok()?;

        pq.push(time_ord);
        map.insert(Arc::clone(&key), value)
    }
}
