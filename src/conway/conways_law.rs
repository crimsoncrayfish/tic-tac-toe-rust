pub fn conways_law(is_alive: bool, live_siblings: u8) -> bool {
    assert!(live_siblings < 9);
    if is_alive {
        if live_siblings < 2 || live_siblings > 3 {
            return false;
        }
        return true;
    }
    if live_siblings == 3 {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        let scenarios = [
            (true, 0, false),
            (true, 1, false),
            (true, 2, true),
            (true, 3, true),
            (true, 4, false),
            (true, 5, false),
            (true, 6, false),
            (true, 7, false),
            (true, 8, false),
            (false, 0, false),
            (false, 1, false),
            (false, 2, false),
            (false, 3, true),
            (false, 4, false),
            (false, 5, false),
            (false, 6, false),
            (false, 7, false),
            (false, 8, false),
        ];

        for (alive, neighbors, expected) in scenarios {
            assert_eq!(
                conways_law(alive, neighbors),
                expected,
                "alive: {}, neighbors: {}",
                alive,
                neighbors
            );
        }
    }
}
