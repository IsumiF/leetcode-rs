/*
 * @lc app=leetcode id=721 lang=rust
 *
 * [721] Accounts Merge
 */

struct Solution {}

// @lc code=start
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn accounts_merge(account_vecs: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let accounts: Vec<Account> = account_vecs
            .into_iter()
            .map(|v| Account::new_from_vec(v))
            .collect();

        let mut email_to_account_ids: HashMap<&str, Vec<usize>> = HashMap::new();
        for (email, account_id) in accounts
            .iter()
            .enumerate()
            .flat_map(|(id, account)| account.get_emails().map(move |email| (email, id)))
        {
            let mut new_account_ids: Vec<usize> = vec![];
            if let Some(account_ids) = email_to_account_ids.remove(email) {
                new_account_ids = account_ids;
            }
            new_account_ids.push(account_id);
            email_to_account_ids.insert(email, new_account_ids);
        }

        let mut dsu = DSU::new(accounts.len());
        for (_, account_ids) in email_to_account_ids {
            if let Some((&first, remaining)) = account_ids.split_first() {
                for &other in remaining {
                    dsu.union(first, other);
                }
            }
        }

        let mut result: HashMap<usize, Account> = HashMap::new();
        for (id, account) in accounts.into_iter().enumerate() {
            let parent_id = dsu.find(id);
            match result.remove(&parent_id) {
                None => {
                    result.insert(parent_id, account);
                }
                Some(mut existing_account) => {
                    existing_account.merge(account);
                    result.insert(parent_id, existing_account);
                }
            }
        }
        result
            .into_iter()
            .map(|(_, account)| account.to_vec())
            .collect()
    }
}

struct Account {
    name: String,
    emails: HashSet<String>,
}

impl Account {
    pub fn new_from_vec(mut vec: Vec<String>) -> Self {
        let name = vec.remove(0);
        Self {
            name,
            emails: vec.into_iter().collect(),
        }
    }

    pub fn get_emails(&self) -> impl Iterator<Item = &str> {
        self.emails.iter().map(|email| email.as_str())
    }

    pub fn merge(&mut self, other: Self) {
        self.emails.extend(other.emails.into_iter());
    }

    pub fn to_vec(self) -> Vec<String> {
        let mut vec: Vec<String> = self.emails.into_iter().collect();
        vec.sort();
        vec.insert(0, self.name);
        vec
    }
}

struct DSU {
    // from set id to it's direct parent set's id
    par: Vec<usize>,
    // from set id to it's size
    size: Vec<usize>,
}

impl DSU {
    pub fn new(size: usize) -> Self {
        Self {
            par: (0..size).collect(),
            size: vec![1; size],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.par[x] != x {
            self.par[x] = self.find(self.par[x]);
        }
        self.par[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let xp = self.find(x);
        let yp = self.find(y);
        if self.size[xp] > self.size[yp] {
            self.par[yp] = self.par[xp];
            self.size[xp] += self.size[yp];
        } else {
            self.par[xp] = self.par[yp];
            self.size[yp] += self.size[xp];
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![
            vec!["John","johnsmith@mail.com","john_newyork@mail.com"],
            vec!["John","johnsmith@mail.com","john00@mail.com"],
            vec!["Mary","mary@mail.com"],
            vec!["John","johnnybravo@mail.com"]
        ] => vec![
            vec!["John","johnnybravo@mail.com"],
            vec!["John","john00@mail.com","john_newyork@mail.com","johnsmith@mail.com"],
            vec!["Mary","mary@mail.com"],
        ]; "one")]
    fn test(accounts: Vec<Vec<&'static str>>) -> Vec<Vec<String>> {
        let mut result = Solution::accounts_merge(
            accounts
                .into_iter()
                .map(|v| v.into_iter().map(|s| s.to_owned()).collect())
                .collect(),
        );
        result.sort_by(|a, b| a[0].cmp(&b[0]).then(a.len().cmp(&b.len())));
        result
        // TODO fix result order
    }
}
