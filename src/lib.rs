#![allow(dead_code)]
#![allow(unused_variables)]

pub mod bot;
mod nft;
mod trader;
mod verusid;

#[cfg(test)]
mod tests {

    #[test]
    pub fn it_works() {
        assert_eq!(1 + 1, 2);
        assert_eq!(1 + 12, 13);
    }
}
