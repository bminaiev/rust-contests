use algo_lib::{collections::peek_random::PeekRandom, misc::rand::Random};

use crate::client::Client;

#[derive(Clone)]
pub struct Scorer {
    clients: Vec<Client>,
    num_likes: Vec<usize>,
    num_dislikes: Vec<usize>,
    use_ingredients: Vec<bool>,
    disliked_by: Vec<Vec<usize>>,
    liked_by: Vec<Vec<usize>>,
    num_ok_clients: usize,
}

impl Scorer {
    pub fn new(clients: Vec<Client>, num_ingredients: usize) -> Self {
        let num_clients = clients.len();
        let mut disliked_by = vec![vec![]; num_ingredients];
        let mut likes_by = vec![vec![]; num_ingredients];
        for (client_id, client) in clients.iter().enumerate() {
            for &dis in client.dislikes.iter() {
                disliked_by[dis].push(client_id);
            }
            for &like in client.likes.iter() {
                likes_by[like].push(client_id);
            }
        }
        Self {
            clients,
            num_likes: vec![0; num_clients],
            num_dislikes: vec![0; num_clients],
            use_ingredients: vec![false; num_ingredients],
            disliked_by,
            liked_by: likes_by,
            num_ok_clients: 0,
        }
    }

    fn client_state(&self, client_id: usize) -> usize {
        if self.num_likes[client_id] == self.clients[client_id].likes.len()
            && self.num_dislikes[client_id] == 0
        {
            1
        } else {
            0
        }
    }

    pub fn switch_ingredient(&mut self, id: usize) {
        self.use_ingredients[id] = !self.use_ingredients[id];
        let enable = self.use_ingredients[id];
        for &client_id in self.disliked_by[id].iter() {
            self.num_ok_clients -= self.client_state(client_id);
            if enable {
                self.num_dislikes[client_id] += 1;
            } else {
                self.num_dislikes[client_id] -= 1;
            }
            self.num_ok_clients += self.client_state(client_id);
        }

        for &client_id in self.liked_by[id].iter() {
            self.num_ok_clients -= self.client_state(client_id);
            if enable {
                self.num_likes[client_id] += 1;
            } else {
                self.num_likes[client_id] -= 1;
            }
            self.num_ok_clients += self.client_state(client_id);
        }
    }

    /// Get the scorer's num ok clients.
    pub fn num_ok_clients(&self) -> usize {
        self.num_ok_clients
    }

    pub fn use_ingredients(&self, idx: usize) -> bool {
        self.use_ingredients[idx]
    }

    pub fn peek_random_client_exn(&self, rnd: &mut Random) -> &Client {
        self.clients.peek_random_exn(rnd)
    }
}
