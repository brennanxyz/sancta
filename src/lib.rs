#[allow(dead_code)] // haha
struct Void {}

struct Happening {}

struct World {}

struct Entity {}

struct Desire {}

struct Devotion {}

struct Conduit {}

struct Sanctum {}

struct Understanding {}

struct Teaching {}

struct Aufhebung {}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
