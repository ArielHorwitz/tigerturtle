#[cfg(test)]
mod unit_tests {
    const EXAMPLE_TOML: &str = include_str!("../example/example.toml");

    fn keys_to_vec(keys: &[&str]) -> Vec<String> {
        keys.iter().map(std::string::ToString::to_string).collect()
    }

    #[test]
    fn simple() {
        let evaluation_string = tigerturtle::process_toml(
            EXAMPLE_TOML,
            keys_to_vec(&[
                "eggs",
                "bar",
                "numbers__sixty_nine",
                "numbers__four_twenty",
                "numbers__pi",
            ]),
            "toml__",
            "_",
            "__",
        )
        .expect("process");
        println!("output:\n{evaluation_string}");
        let evaluation_strings = evaluation_string.split('\n').collect::<Vec<&str>>();
        for s in [
            "toml__eggs=\"spam\"",
            "toml__bar=\"baz\"",
            "toml__numbers__sixty_nine=69",
            "toml__numbers__four_twenty=420",
            "toml__numbers__pi=3.1415",
        ] {
            println!("checking: {s}");
            assert!(evaluation_strings.contains(&s));
        }
    }

    #[test]
    fn nested() {
        let evaluation_string = tigerturtle::process_toml(
            EXAMPLE_TOML,
            keys_to_vec(&[
                "eggs",
                "bar",
                "numbersABCDEFGsixty_nine",
                "numbersABCDEFGfour_twenty",
                "numbersABCDEFGpi",
            ]),
            "toml__",
            "_",
            "ABCDEFG",
        )
        .expect("process");
        println!("output:\n{evaluation_string}");
        let evaluation_strings = evaluation_string.split('\n').collect::<Vec<&str>>();
        for s in [
            "toml__eggs=\"spam\"",
            "toml__bar=\"baz\"",
            "toml__numbersABCDEFGsixty_nine=69",
            "toml__numbersABCDEFGfour_twenty=420",
            "toml__numbersABCDEFGpi=3.1415",
        ] {
            println!("checking: {s}");
            assert!(evaluation_strings.contains(&s));
        }
    }

    #[test]
    fn required_fail() {
        let evaluation_string =
            tigerturtle::process_toml(EXAMPLE_TOML, keys_to_vec(&["_foo"]), "toml__", "_", "__");
        assert!(evaluation_string.is_err());
    }

    #[test]
    fn required_prefix_fail() {
        let evaluation_string =
            tigerturtle::process_toml(EXAMPLE_TOML, keys_to_vec(&["=foo"]), "toml__", "=", "__");
        assert!(evaluation_string.is_err());
    }

    #[test]
    fn required_prefix() {
        let evaluation_string =
            tigerturtle::process_toml(EXAMPLE_TOML, keys_to_vec(&["=eggs"]), "toml__", "=", "__")
                .expect("process");
        println!("output:\n{evaluation_string}");
        let evaluation_strings = evaluation_string.split('\n').collect::<Vec<&str>>();
        assert!(evaluation_strings.contains(&"toml__eggs=\"spam\""));
    }

    #[test]
    fn output_prefix() {
        let evaluation_string = tigerturtle::process_toml(
            EXAMPLE_TOML,
            keys_to_vec(&[
                "eggs",
                "bar",
                "numbers__sixty_nine",
                "numbers__four_twenty",
                "numbers__pi",
            ]),
            "output__",
            "_",
            "__",
        )
        .expect("process");
        println!("output:\n{evaluation_string}");
        let evaluation_strings = evaluation_string.split('\n').collect::<Vec<&str>>();
        for s in [
            "output__eggs=\"spam\"",
            "output__bar=\"baz\"",
            "output__numbers__sixty_nine=69",
            "output__numbers__four_twenty=420",
            "output__numbers__pi=3.1415",
        ] {
            println!("checking: {s}");
            assert!(evaluation_strings.contains(&s));
        }
    }

    #[test]
    fn output_prefix_empty() {
        let evaluation_string = tigerturtle::process_toml(
            EXAMPLE_TOML,
            keys_to_vec(&[
                "eggs",
                "bar",
                "numbers__sixty_nine",
                "numbers__four_twenty",
                "numbers__pi",
            ]),
            "",
            "_",
            "__",
        )
        .expect("process");
        println!("output:\n{evaluation_string}");
        let evaluation_strings = evaluation_string.split('\n').collect::<Vec<&str>>();
        for s in [
            "eggs=\"spam\"",
            "bar=\"baz\"",
            "numbers__sixty_nine=69",
            "numbers__four_twenty=420",
            "numbers__pi=3.1415",
        ] {
            println!("checking: {s}");
            assert!(evaluation_strings.contains(&s));
        }
    }
}
