use crate::consistent_hash_ring::ConsitentHashRing;

struct CHRVecNode<ConsumerInfo>
where
    ConsumerInfo: Clone,
{
    hash: u64,
    key: String,

    /// Data stored about the cosumer like IP address, port, etc.
    data: ConsumerInfo,
}

struct CHRVec<ConsumerInfo>
where
    ConsumerInfo: Clone,
{
    consumers: Vec<CHRVecNode<ConsumerInfo>>,
    virtual_nodes: usize,
}

impl<ConsumerInfo> CHRVec<ConsumerInfo>
where
    ConsumerInfo: Clone,
{
    pub fn new(virtual_nodes_per_consumer: usize) -> Self {
        Self {
            consumers: Vec::new(),
            virtual_nodes: virtual_nodes_per_consumer,
        }
    }

    fn hash(key: &str) -> u64 {
        seahash::hash(key.as_bytes())
    }
}

impl<ConsumerInfo> ConsitentHashRing for CHRVec<ConsumerInfo>
where
    ConsumerInfo: Clone,
{
    type ConsumerInfo = ConsumerInfo;

    fn add_consumer(&mut self, key: &str, data: Self::ConsumerInfo) {
        self.consumers.reserve(self.virtual_nodes);

        let nodes_to_insert = (0..self.virtual_nodes).map(|i| {
            let key = format!("{}_{}", key, i);
            let hash = Self::hash(&key);
            CHRVecNode {
                key,
                hash,
                data: data.clone(),
            }
        });
        self.consumers.extend(nodes_to_insert);

        self.consumers.sort_by_key(|consumer| consumer.hash);
    }

    fn remove_consumer(&mut self, key: &str) {
        self.consumers
            .retain(|consumer| !consumer.key.starts_with(key));
    }

    fn get_consumer(&self, key: &str) -> Option<&Self::ConsumerInfo> {
        let hash = Self::hash(key);

        if self.consumers.is_empty() {
            return None;
        }

        let index_result = self
            .consumers
            .binary_search_by_key(&hash, |consumer| consumer.hash);

        let index = match index_result {
            Ok(index) => index,
            Err(index) => {
                if index >= self.consumers.len() {
                    0
                } else {
                    index
                }
            }
        };

        Some(&self.consumers[index].data)
    }
}
